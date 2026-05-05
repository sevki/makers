#!/usr/bin/env python3
"""
Mechanical cleanup of c2rust's verbose integer literal casts in a Rust source.

c2rust translates every C integer literal as `0 as ::core::ffi::c_int` to
preserve C semantics. Most of these casts are redundant in Rust because the
context already constrains the type. This pass strips the most common:

  - `N as ::core::ffi::c_int` → `N`               (where N is a small literal)
  - `-(N as ::core::ffi::c_int)` → `-N`
  - `N as size_t` → `N`
  - `'\\0' as i32` → `0`

Conservative: only rewrites unambiguous patterns. We rerun `cargo test` after
to catch any case where the cast was load-bearing for type inference.
"""
import pathlib
import re
import sys


def cleanup(src: str) -> str:
    # `'\0' as i32` → `0`. Same for c_int / c_char. Always safe.
    src = re.sub(
        r"'\\0' as (?:i32|::core::ffi::c_int|::core::ffi::c_char)",
        '0',
        src,
    )

    # Strip `as <int_type>` from small literals. We never touch the surrounding
    # parens — they may be call-arg parens (`f(1 as c_int)` is a 1-arg call,
    # not a grouped expression).
    #
    # Skipped contexts (where the cast is load-bearing):
    #   * preceded by unary `-`        (e.g. `-(1 as c_int)` — the cast forces
    #     signed semantics; bare `-1` in an unsigned context fails to compile)
    #   * followed by a method call   (e.g. `(2 as c_int).wrapping_mul(...)` —
    #     bare `2.wrapping_mul(...)` has ambiguous numeric type)
    #   * followed by another cast    (`as size_t as c_long` — chains pin types)
    #
    # `\b(?<![-(])\d{1,3}\s+as\s+...` catches the negative-cast form because
    # the `-` directly precedes the `(`. We use a positive form: only strip if
    # the digit is at a "value position" — preceded by `(` or non-`-(`.
    # The negative lookahead also rules out the case where parens wrap the cast
    # and a method call follows: `(2 as c_int).wrapping_mul(...)`. Allow
    # arbitrary whitespace/newlines between `)` and `.` since c2rust often
    # breaks long expressions across lines.
    int_cast = re.compile(
        r'\b(?P<n>\d{1,3}) as ::core::ffi::c_(?:int|uint|long|ulong|short|ushort|char|uchar)\b'
        r'(?!\.\w|\s+as|\)\s*\.\w)',
        re.DOTALL,
    )

    def replace_int(m: re.Match) -> str:
        # Look at the char just before the digit. If it's `(` and the char
        # before that is `-`, this is `-(N as ...)` — leave it.
        i = m.start()
        if i >= 2 and src[i - 1] == "(" and src[i - 2] == "-":
            return m.group(0)
        return m.group("n")

    src = int_cast.sub(replace_int, src)

    # `N as size_t` — same skip rules.
    src = re.sub(
        r'\b(\d{1,3}) as size_t\b(?!\.\w|\s+as|\)\s*\.\w)',
        r'\1',
        src,
        flags=re.DOTALL,
    )

    return src


def main(paths: list[str]) -> int:
    total = 0
    for p in paths:
        path = pathlib.Path(p)
        before = path.read_text()
        after = cleanup(before)
        if before != after:
            path.write_text(after)
            delta = before.count("\n") - after.count("\n")
            chars = len(before) - len(after)
            print(f"  {path.name}: -{chars} chars (-{delta} lines)")
            total += chars
    print(f"\ntotal: -{total} chars")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
