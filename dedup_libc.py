#!/usr/bin/env python3
"""
c2rust redeclares every libc function/static in every TU's `extern "C" { ... }`
block. Move those decls into `use libc::{ ... };` so the file imports from the
already-present `libc` crate instead of stating its own duplicate signatures.

Strategy:
  - Read libc's known function and static names (curated from the libc crate
    source under ~/.cargo/registry).
  - For each `.rs` file, scan top-level `extern "C" { ... }` blocks.
  - For each item inside, if it's a libc fn or static, drop it from the block
    and remember the name. Items not in libc (cross-TU references, glibc
    internals like stdin) stay put.
  - Prepend `use libc::{ name1, name2, ... };` to the file.
  - If the resulting extern block is empty, remove it entirely.

This is a lossless source rewrite when applied to c2rust output: the lib still
links the same symbols (libc declares them with the same C-ABI signatures).
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).parent / "src"

with open("/tmp/libc_fns.txt") as f:
    LIBC_FNS = {ln.strip() for ln in f if ln.strip()}
with open("/tmp/libc_statics.txt") as f:
    LIBC_STATICS = {ln.strip() for ln in f if ln.strip()}

# Items that c2rust emits but libc doesn't have under that name on this target.
# stdin/stdout/stderr: glibc inline accessors, not exported as `pub static`.
# putc/vsprintf: not in libc-rs (variadic / inline).
# environ/optarg/optind/opterr/optopt: declared in libc but behind cfg gates
# that are off here — leave the extern decl in place for now.
DENYLIST = {
    "stdin", "stdout", "stderr",
    "putc", "vsprintf",
    "environ", "optarg", "optind", "opterr", "optopt",
    # libc signatures differ subtly from c2rust's local decl:
    "execvp",  # libc: *const *const c_char ; c2rust: *const *mut c_char
    "atexit",  # libc: extern "C" fn() ; c2rust: Option<unsafe extern "C" fn() -> ()>
}


def find_local_types(src: str) -> set[str]:
    """Names declared as a struct/union/type/enum *anywhere* in the file —
    including extern-block opaque types. Two reasons we care:

      1. Direct collision: file declares `pub struct stat` and `libc::stat`
         (which is both struct and fn) would shadow each other.
      2. Signature drift: a libc fn whose signature mentions one of these
         types (e.g. `fstat(fd, *mut stat)`) won't match callers passing
         `*mut local::stat` — even if the field layouts are identical, Rust
         treats them as distinct types.

    Both cases are handled the same way: skip the import, leave the extern
    decl in the file.
    """
    names = set()
    for m in re.finditer(
        r'^\s*pub\s+(?:struct|union|type|enum)\s+([A-Za-z_]\w*)',
        src,
        flags=re.MULTILINE,
    ):
        names.add(m.group(1))
    # extern types: `pub type _IO_FILE;`, `pub type stat;`, etc.
    for m in re.finditer(
        r'^\s*pub\s+type\s+([A-Za-z_]\w*)\s*;',
        src,
        flags=re.MULTILINE,
    ):
        names.add(m.group(1))
    return names


def signature_references_local_type(item: str, local_types: set[str]) -> bool:
    """True if any identifier in `item` matches a locally-declared type name.
    Conservative — false positives are fine (we just keep the extern decl).
    """
    for ident in re.findall(r'\b([A-Za-z_]\w*)\b', item):
        if ident in local_types:
            return True
    return False


def parse_item_name(item: str) -> tuple[str, str] | None:
    """
    Return (kind, name) for an item inside an extern "C" {} block.
    kind in {'fn', 'static'}. None if unrecognized.
    """
    m = re.match(r'\s*(?:pub\s+)?fn\s+([A-Za-z_]\w*)\s*\(', item)
    if m:
        return ("fn", m.group(1))
    m = re.match(r'\s*(?:pub\s+)?static\s+(?:mut\s+)?([A-Za-z_]\w*)\s*:', item)
    if m:
        return ("static", m.group(1))
    return None


def split_items(block_body: str) -> list[str]:
    """Split the body of an extern "C" { ... } block into per-item strings.
    Each item ends with `;` at brace-depth 0 (inside the extern block).
    """
    items: list[str] = []
    depth = 0  # parens depth for multi-line fn signatures
    cur: list[str] = []
    for ch in block_body:
        cur.append(ch)
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        elif ch == ";" and depth == 0:
            items.append("".join(cur))
            cur = []
    tail = "".join(cur).strip()
    if tail:
        items.append(tail)
    return items


def process_file(path: pathlib.Path) -> tuple[int, int]:
    src = path.read_text()
    local_types = find_local_types(src)
    # Find every top-level `extern "C" { ... }` block. They are at column 0
    # in c2rust output. Match across newlines.
    pattern = re.compile(r'^extern "C" \{\n(.*?)^\}\n', re.DOTALL | re.MULTILINE)

    moved_fns: list[str] = []
    moved_statics: list[str] = []

    def rewrite_block(m: re.Match) -> str:
        body = m.group(1)
        items = split_items(body)
        kept: list[str] = []
        for it in items:
            ident = parse_item_name(it)
            if ident is None:
                kept.append(it)
                continue
            kind, name = ident
            if name in DENYLIST or name in local_types:
                kept.append(it)
                continue
            if signature_references_local_type(it, local_types):
                # libc fn whose signature mentions a locally-declared type;
                # importing would cause type-mismatch errors at call sites.
                kept.append(it)
                continue
            if kind == "fn" and name in LIBC_FNS:
                moved_fns.append(name)
                continue
            if kind == "static" and name in LIBC_STATICS:
                moved_statics.append(name)
                continue
            kept.append(it)
        if not kept:
            return ""
        return 'extern "C" {\n' + "".join(kept).rstrip() + "\n}\n"

    new_src = pattern.sub(rewrite_block, src)
    if not moved_fns and not moved_statics:
        return (0, 0)

    moved = sorted(set(moved_fns) | set(moved_statics))
    use_line = "use libc::{" + ", ".join(moved) + "};\n"

    # Find the existing `use ::libc;` line c2rust emits and insert after it,
    # or insert at the top after the initial `use ::c2rust_bitfields;` block.
    insertion_pat = re.compile(r'(^use ::libc;\n)', re.MULTILINE)
    if insertion_pat.search(new_src):
        new_src = insertion_pat.sub(r'\1' + use_line, new_src, count=1)
    else:
        new_src = use_line + new_src

    path.write_text(new_src)
    return (len(set(moved_fns)), len(set(moved_statics)))


def main() -> int:
    total_fns = 0
    total_statics = 0
    files_changed = 0
    for rs in sorted(ROOT.glob("*.rs")):
        fns, statics = process_file(rs)
        if fns or statics:
            files_changed += 1
            print(f"{rs.name}: -{fns} fns, -{statics} statics")
            total_fns += fns
            total_statics += statics
    print(f"\n{files_changed} files; {total_fns} fn decls and {total_statics} static decls moved to `use libc::`.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
