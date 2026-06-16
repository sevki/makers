# Coccinelle-for-Rust semantic patches

Semantic patches (SmPL for Rust, run by [`cfr`](https://rust-for-linux.com/coccinelle-for-rust))
for the recurring `static mut` → atomic conversions in this c2rust port.

## Install `cfr`

```sh
# upstream: https://gitlab.inria.fr/coccinelle/coccinelleforrust
git clone https://github.com/sevki/CoccinelleForRust && cd CoccinelleForRust
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
tokens at the top of the `.cocci` for your flag, then run it over each file
that mentions the symbol:

```sh
cfr -c cocci/static_mut_bool_to_atomic.cocci src/output.rs          # prints a diff
cfr -c cocci/static_mut_bool_to_atomic.cocci src/output.rs --apply  # rewrites in place
```

Default prints a unified diff for review; `--apply` edits the file. `cfr` also
accepts a directory as the target to sweep a whole tree.

## Caveats

- **Validated against `cfr`** (built from the mirror above): both patches apply
  cleanly to the worked-example symbols. Still **review the diff** and run
  `cargo build && cargo clippy --lib && cargo test` afterwards — the patches are
  the mechanical bulk, not a guarantee of correctness.
- `cfr` re-tokenizes the edited items, so the output loses original spacing and
  blank lines (e.g. the accessor collapses onto one line) — run **`cargo fmt`**
  after applying.
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
