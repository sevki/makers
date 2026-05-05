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
    # `-(N as ::core::ffi::c_int)` → `-N`. Do this first; the next pass would
    # otherwise leave a stray `-(N)`.
    src = re.sub(
        r'-\((\d+) as ::core::ffi::c_int\)',
        r'-\1',
        src,
    )
    # `N as ::core::ffi::c_int` / `c_uint` / `c_long` / `c_ulong` → `N`.
    # Limit to small (1-3 digit) literals — bigger ones often need type pin.
    src = re.sub(
        r'\b(\d{1,3}) as ::core::ffi::c_(?:int|uint|long|ulong|short|ushort|char|uchar)\b',
        r'\1',
        src,
    )
    # `N as size_t` → `N` for small literals.
    src = re.sub(r'\b(\d{1,3}) as size_t\b', r'\1', src)
    # `'\0' as i32` → `0`. Same for c_int / c_char.
    src = re.sub(
        r"'\\0' as (?:i32|::core::ffi::c_int|::core::ffi::c_char)",
        '0',
        src,
    )
    # `'X' as i32` for printable chars → b'X' as c_int (clearer)
    # We keep these — they're readable.
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
