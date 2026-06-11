#!/usr/bin/env python3
"""Parse error/fatal/message/dbg/concat call sites (balanced-paren), report or
rewrite them for the non-variadic conversion."""
import re
import sys
from pathlib import Path

SRC = Path(__file__).resolve().parent.parent / "src"
FNS = ("error", "fatal", "message", "dbg")

CALL_RE = re.compile(r"\b(error|fatal|message|dbg|concat)\s*\(")


def split_args(text, start):
    """text[start] == '(' -> (args list, end index after ')')."""
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


FMT_LIT = re.compile(r'^b"(.*)\\0" as \*const u8 as \*const ::core::ffi::c_char$', re.S)
SPEC = re.compile(r"%(%|\.\*s|[-#0-9.]*(?:hh|h|ll|l|z)?[sducxXofgeFGp])")


def main():
    report = {}
    nonliteral = []
    for p in sorted(SRC.glob("*.rs")):
        t = p.read_text()
        for m in CALL_RE.finditer(t):
            # skip declarations/definitions
            line_start = t.rfind("\n", 0, m.start()) + 1
            prefix = t[line_start : m.start()]
            if "fn " in prefix or "use " in prefix:
                continue
            try:
                args, _ = split_args(t, m.end() - 1)
            except ValueError:
                continue
            name = m.group(1)
            if name == "concat":
                continue
            fmt_idx = {"error": 2, "fatal": 2, "message": 2, "dbg": 0}[name]
            if len(args) <= fmt_idx:
                continue
            fmt = args[fmt_idx]
            lm = FMT_LIT.match(fmt)
            if not lm:
                nonliteral.append((p.name, name, fmt[:60]))
                continue
            for s in SPEC.findall(lm.group(1)):
                report[s] = report.get(s, 0) + 1
    for s, c in sorted(report.items(), key=lambda kv: -kv[1]):
        print(f"{c:5} %{s}")
    print("--- non-literal fmts:")
    for f, n, fmt in nonliteral:
        print(f"  {f}: {n}( {fmt}")


if __name__ == "__main__":
    main()
