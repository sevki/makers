# Coccinelle-for-Rust semantic patches

Semantic patches (SmPL for Rust, run by [`cfr`](https://rust-for-linux.com/coccinelle-for-rust))
for the recurring `static mut` → atomic conversions in this c2rust port.

## Install `cfr`

```sh
git clone https://gitlab.inria.fr/coccinelle/coccinelleforrust && cd coccinelleforrust
cargo build --release
cp target/release/cfr ~/.local/bin/
```

## Patches

| File | Pattern | Example PRs |
|------|---------|-------------|
| `static_mut_bool_to_atomic.cocci` | one-shot 0/1 flag → `AtomicBool` + `flag() -> bool` accessor; `!= 0`→`()`, `== 0`→`!`, `= 1/0;`→`store(..)` | #102–#105, #108 |
| `static_mut_counter_to_atomic.cocci` | monotonic counter → `AtomicU32` + accessor; `wrapping_add(1)`→`fetch_add`; reads→`()` | #98, #100, #109 |

## Usage

Each patch is a **template written for one concrete symbol**. Coccinelle can't
invent the new `SCREAMING_CASE` storage name or the accessor, so you edit the
two/four tokens at the top of the `.cocci` for your flag, then run it over each
file that mentions the symbol:

```sh
cfr --rule-file cocci/static_mut_bool_to_atomic.cocci --rs-file src/output.rs --o-place .
```

`--o-place .` rewrites in place; drop it to print the patch to stdout for review.

## Caveats

- **Not validated in this repo's CI.** They were written against the documented
  SmPL-for-Rust subset; `cfr`'s upstream host was unreachable from the
  environment they were authored in, so **review every diff** and run
  `cargo build && cargo clippy --lib && cargo test` afterwards.
- `cfr` does not reliably rewrite doc-comments / attributes attached to items —
  add the accessor's `///` doc-comment by hand after the run.
- The bool read-site rules (`flag != 0` / `flag == 0`) use the **literal** flag
  identifier on purpose: a bare `identifier` metavariable would rewrite *every*
  `x != 0` in the file. Keep them concrete, one symbol per run.
- After a counter conversion, clippy may raise `needless_late_init` where a
  `let x; … x = counter();` snapshot became an initializer — inline it to
  `let x = counter();`.
- Add `use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};` (as needed) if
  the file doesn't already import them; `cfr` won't manage the `use` for you.
- These cover the mechanical bulk only. Genuinely manual: choosing bool vs.
  integer (tri-state sentinels like `SHELL_FUNCTION_COMPLETED` stay integer),
  the FFI boundary `as c_int` casts, and the differential/unit tests.
