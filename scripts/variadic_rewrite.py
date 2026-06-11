#!/usr/bin/env python3
"""Rewrite variadic call sites to the non-variadic &[FmtArg] API.

error(floc, len, fmt, a...)   -> error(floc, fmt, &[wrap(a)...])
fatal(floc, len, fmt, a...)   -> fatal(floc, fmt, &[wrap(a)...])
message(pfx, len, fmt, a...)  -> message(pfx, fmt, &[wrap(a)...])
format(pfx, len, fmt, a...)   -> format_message(pfx, fmt, &[wrap(a)...])
concat(n, a...)               -> concat(&[a...])

Wrapping is driven by the printf specifier list parsed from the (literal)
format string; non-literal formats are reported for manual conversion.
"""
import re
import sys
from pathlib import Path

SRC = Path(__file__).resolve().parent.parent / "src"

CALL_RE = re.compile(r"\b(error|fatal|message|format|concat)\s*\(")
FMT_LIT = re.compile(r'^b"(.*)\\0" as \*const u8 as \*const ::core::ffi::c_char$')
SPEC = re.compile(r"%([-#0 ]*)(\*|\d+)?(?:\.(\*|\d+))?(hh|h|ll|l|z)?([sducxXp%])")


def split_args(text, start):
    depth = 0
    i = start
    args = []
    cur = []
    in_str = None
    while i < len(text):
        ch = text[i]
        if in_str:
            if ch == "\\":
                cur.append(text[i : i + 2])
                i += 2
                continue
            if ch == in_str:
                in_str = None
            cur.append(ch)
        elif ch in "\"'":
            in_str = ch
            cur.append(ch)
        elif ch == "(":
            depth += 1
            if depth > 1:
                cur.append(ch)
        elif ch == ")":
            depth -= 1
            if depth == 0:
                if "".join(cur).strip():
                    args.append("".join(cur).strip())
                return args, i + 1
            cur.append(ch)
        elif ch == "," and depth == 1:
            args.append("".join(cur).strip())
            cur = []
        else:
            cur.append(ch)
        i += 1
    raise ValueError("unbalanced")


def wrap(spec, arg):
    """spec: (flags, width, prec, length, conv); may consume extra args for *"""
    _flags, width, prec, length, conv = spec
    wrapped = []
    if width == "*":
        a = arg.pop(0)
        wrapped.append(f"FmtArg::Int(({a}) as i64)")
    if prec == "*":
        a = arg.pop(0)
        wrapped.append(f"FmtArg::Int(({a}) as i64)")
    if conv == "%":
        return wrapped
    a = arg.pop(0)
    if conv == "s":
        wrapped.append(f"FmtArg::Str(({a}) as *const ::core::ffi::c_char)")
    elif conv in "di":
        if length in ("l", "ll", "z"):
            wrapped.append(f"FmtArg::Int(({a}) as i64)")
        elif length == "h":
            wrapped.append(f"FmtArg::Int(({a}) as i16 as i64)")
        else:
            wrapped.append(f"FmtArg::Int(({a}) as i32 as i64)")
    elif conv in "uxX":
        if length in ("l", "ll", "z"):
            wrapped.append(f"FmtArg::Uint(({a}) as u64)")
        elif length == "h":
            wrapped.append(f"FmtArg::Uint(({a}) as u16 as u64)")
        else:
            wrapped.append(f"FmtArg::Uint(({a}) as u32 as u64)")
    elif conv == "c":
        wrapped.append(f"FmtArg::Int(({a}) as i64)")
    elif conv == "p":
        wrapped.append(f"FmtArg::Ptr(({a}) as *const ::core::ffi::c_void)")
    return wrapped


def rewrite_file(p):
    t = p.read_text()
    out = []
    i = 0
    n_rewritten = 0
    manual = []
    while True:
        m = CALL_RE.search(t, i)
        if not m:
            out.append(t[i:])
            break
        name = m.group(1)
        line_start = t.rfind("\n", 0, m.start()) + 1
        prefix = t[line_start : m.start()]
        # skip defs, decls, macro use like format!, and string contents
        if ("fn " in prefix or "use " in prefix or prefix.rstrip().endswith("!")
                or prefix.count('"') % 2 == 1):
            out.append(t[i : m.end()])
            i = m.end()
            continue
        try:
            args, end = split_args(t, m.end() - 1)
        except ValueError:
            out.append(t[i : m.end()])
            i = m.end()
            continue

        if name == "concat":
            if len(args) < 1 or not args[0].rstrip().isdigit():
                out.append(t[i : m.end()])
                i = m.end()
                continue
            new = f"concat(&[{', '.join(a.strip() for a in args[1:])}])"
            out.append(t[i : m.start()] + new)
            i = end
            n_rewritten += 1
            continue

        fmt_idx = 2
        if len(args) <= fmt_idx:
            out.append(t[i : m.end()])
            i = m.end()
            continue
        fmt_norm = re.sub(r"\s+", " ", args[fmt_idx]).strip()
        lm = FMT_LIT.match(fmt_norm)
        if not lm:
            manual.append((p.name, name, fmt_norm[:60]))
            out.append(t[i : m.end()])
            i = m.end()
            continue
        specs = SPEC.findall(lm.group(1))
        rest = [a for a in args[fmt_idx + 1 :]]
        wrapped = []
        ok = True
        for s in specs:
            need = (1 if s[4] != "%" else 0) + (s[1] == "*") + (s[2] == "*")
            if len(rest) < need:
                ok = False
                break
            wrapped += wrap(s, rest)
        if not ok or rest:
            manual.append((p.name, name, f"arg-count-mismatch {fmt_norm[:50]}"))
            out.append(t[i : m.end()])
            i = m.end()
            continue
        fname = "format_message" if name == "format" else name
        arg0 = args[0]
        arglist = ",\n            ".join(wrapped)
        if wrapped:
            new = f"{fname}(\n        {arg0},\n        {args[fmt_idx]},\n        &[{arglist}],\n    )"
        else:
            new = f"{fname}(\n        {arg0},\n        {args[fmt_idx]},\n        &[],\n    )"
        out.append(t[i : m.start()] + new)
        i = end
        n_rewritten += 1
    new_t = "".join(out)
    if new_t != t:
        p.write_text(new_t)
    return n_rewritten, manual


def main():
    total = 0
    all_manual = []
    for p in sorted(SRC.glob("*.rs")):
        if p.name == "output.rs" and "--include-output" not in sys.argv:
            pass  # output.rs call sites (perror_with_name etc.) need rewriting too
        n, manual = rewrite_file(p)
        if n:
            print(f"{p.name}: {n} calls rewritten")
        total += n
        all_manual += manual
    print(f"total: {total}")
    print("--- manual sites:")
    for f, n, fmt in all_manual:
        print(f"  {f}: {n}( {fmt}")


if __name__ == "__main__":
    main()
