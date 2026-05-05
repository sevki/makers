#!/usr/bin/env python3
"""
c2rust translates each C TU independently. Functions that are static-inline
in headers (or otherwise referenced cross-TU through `extern "C"` decls) end
up Rust-mangled in their defining .rs and unresolved at link time.

This pass injects `#[no_mangle]` and `pub` before every `extern "C" fn`
definition that lacks them, so cross-TU calls resolve.
"""
import pathlib
import re

EXCLUDE = {"build.rs", "c2rust-lib.rs", "fix_no_mangle.py"}
ROOT = pathlib.Path(__file__).parent / "src"

# Match function definitions, capturing the existing modifiers.
FN_DEF_RE = re.compile(
    r'^(?P<indent>\s*)(?P<pub>pub )?(?P<unsafe>unsafe )?extern "C" fn (?P<rest>.*)$'
)


def looks_like_def(line: str, next_line: str) -> bool:
    # Definitions end with `{` somewhere on this line or the next non-blank.
    return "{" in line or "{" in next_line


for rs in sorted(ROOT.glob("*.rs")):
    if rs.name in EXCLUDE:
        continue
    lines = rs.read_text().splitlines()
    out: list[str] = []
    changed = 0
    i = 0
    while i < len(lines):
        line = lines[i]
        m = FN_DEF_RE.match(line)
        if m and looks_like_def(line, lines[i + 1] if i + 1 < len(lines) else ""):
            # Look back for an existing #[no_mangle] (possibly through #[inline]).
            j = len(out) - 1
            already = False
            while j >= 0 and (out[j].strip().startswith("#[") or out[j].strip() == ""):
                if out[j].strip() == "#[no_mangle]":
                    already = True
                    break
                j -= 1
            if not already:
                indent = m.group("indent")
                pub = m.group("pub") or "pub "
                unsafe = m.group("unsafe") or ""
                rest = m.group("rest")
                out.append(f"{indent}#[no_mangle]")
                out.append(f'{indent}{pub}{unsafe}extern "C" fn {rest}')
                changed += 1
                i += 1
                continue
        out.append(line)
        i += 1
    if changed:
        rs.write_text("\n".join(out) + "\n")
        print(f"{rs.name}: +{changed} #[no_mangle]")
