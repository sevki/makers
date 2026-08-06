# Agent Instructions: Translating C to Rust with c2rust

# C2Rust Porting Rules

## Issue tracking and project management

Github issues should be used to track followups todos and basic project 
maanagement. The [project](https://github.com/users/sevki/projects/7) 
is to trcack everything.

Do not track cleanup/refactor progress in a markdown checklist file
committed to the repo (e.g. a `*-checklist.md`). Open a GitHub issue (with
sub-issues for multi-slice campaigns) instead, and update issue bodies/
comments as work lands. Issues are the single source of truth for what's
done and what's left — a checklist file duplicates that and drifts out of
sync.

## Pull requests: always stack

Multi-slice campaigns here (Phase A/B, the module splits, the per-module
`fatal()` conversions) are naturally a *sequence* of dependent changes. Land
them as a **stack of pull requests** using GitHub's
[stacked pull requests](https://github.blog/changelog/2026-07-30-stacked-pull-requests-are-now-in-public-preview/)
(public preview, announced 2026-07-30) — not as a chain of branches each
retargeted by hand, and not as one omnibus PR.

**Rules:**
- Any campaign with more than one slice **must** be opened as a stack. Each
  slice keeps the properties the rest of this file already demands — one
  module per PR, behavior-preserving, coverage delta `>= 0`, differential
  tests green — and the stack makes the dependency order explicit instead of
  leaving reviewers to infer it from merge timing.
- Keep each PR in the stack independently reviewable. A stack is not a licence
  to make individual slices bigger; if a slice only makes sense read together
  with the one below it, the seam is in the wrong place.
- Do **not** collapse a stack into a single PR to dodge the review of an
  awkward middle slice, and do not stack unrelated work just because it is in
  flight at the same time — a stack encodes a real dependency.
- The base of the stack is `main`. Rebase the whole stack when `main` moves
  rather than merging `main` into individual entries, so each PR's diff stays
  the slice and nothing else.

Single, self-contained changes (a dependency bump, a one-file fix) stay
ordinary standalone PRs.

## North Star (the watermark)

Before writing or accepting any change, ask: **"Is this how
[hdonnay](https://github.com/hdonnay) or
[steveklabnik](https://github.com/steveklabnik) would write it as idiomatic
Rust from scratch?"** That is the watermark for every line in `src/`. If the
answer is no, redesign rather than transliterate — a c2rust shape that merely
compiles is not the goal.

Two non-negotiable rules flow from this:

1. **Safe conversions only.** Every cleanup must *remove* `unsafe`, never add
   it, and must be behavior-preserving (differential-tested against the C
   oracle). Eliminate a raw-pointer dereference by turning it into a safe
   borrow (`&T`/`&mut T`, `<*mut T>::as_ref()`/`as_mut()`, `NonNull`,
   `Option<&T>`, slices, iterators) — **do not** "fix" a raw deref by bolting
   on a null-check guard, `assert!`, or extra branch just to satisfy a linter
   or security scanner. If a pointer access can't be made safe without
   changing a stored struct field type or cascading signatures, leave it and
   say so; don't paper over it.
2. **No global singletons.** No `static mut`, no global mutable state, and no
   "mirror" statics that shadow owned data. Thread ownership explicitly:
   pass owned state by `&`/`&mut`, use `Cell`/`RefCell` for scoped interior
   mutability, or a scoped accessor channel — never a process-wide singleton.
   Readers reach state through the owner, not a global.
3. **Preserve the original as a test oracle.** When you replace an `unsafe`
   implementation with a safe one, do **not** delete the old code — move it
   verbatim into a `#[cfg(test)]` module (e.g. rename it `<fn>_unsafe_oracle`)
   and add a test that drives representative inputs through both the new safe
   version and the preserved unsafe one, asserting identical results. The old
   code stays out of the shipping build but proves the conversion is
   behavior-preserving; remove the oracle only once the safe version is
   covered by differential tests against the C oracle.

## Goals
1. Preserve behavior first
2. Reduce unsafe incrementally (safe conversions only — never widen `unsafe`)
3. Replace C patterns with Rust abstractions
4. Never preserve C architecture blindly
5. Hold every change to the watermark above

## Conversion Priorities (work order)

When choosing what to clean up next, prefer these, in order. Every change
must preserve behavior and be differential-tested against the in-tree C
oracle (`./make`).

**Every fixture runs against the C oracle and is verified.** Every fixture
listed in `scripts/fixtures-manifest.tsv` is executed against BOTH the Rust
port and the in-tree C oracle and their outputs are asserted
byte-identical — no fixture may skip the oracle comparison or assert
against a hand-written expectation instead. This now runs as three CI
jobs rather than in-process in `cargo test`: `fixtures-run-rust` and
`fixtures-run-c` each independently run every manifest fixture through one
binary (piping stdout/stderr to files and snapshotting the resulting
working tree), and `fixtures-diff` downloads both artifact sets and
diffoscopes them (ignoring mtimes) as the hard gate; a coverage or lint job
that tolerates failures does not count as verification. Most fixtures are
`kind=simple` (a fixture file run via `-f`/target/args, handled by
`scripts/run-fixtures.sh`); cases that need custom setup instead of that
shape (print-directory tracing, `-I` resolution, signal delivery, archive
pre-creation) get a different `kind` and are handled by
`scripts/run-bespoke-fixtures.sh` — every kind still lands in the same
output layout, so `fixtures-diff` compares them identically. `tests/rs_integration.rs`
itself only smoke-tests the Rust make (asserts it runs without crashing) —
it is fast local signal, not the enforcement point. A fixture may only be
exempted from the gate by quarantining it: the `skip` column in
`scripts/fixtures-manifest.tsv` (`fixtures-diff` reports it but does not
fail), mirrored by `#[ignore = "known divergence #NNN: ..."]` on the
corresponding `tests/rs_integration.rs` test, pointing at an open issue that
tracks the divergence. Keep the fixture's manifest row in sync with its test
(fixture file, target, args) when either changes. When the *oracle* is the
wrong side — the C code has a bug and matching it means shipping a known-wrong
result — do not quarantine: keep the default bug-compatible so the fixture
stays in the gate, and put the correct behaviour behind an opt-in
`MAKERS_*` environment variable documented in
[docs/divergences.md](docs/divergences.md), with the default pinned by the
fixture and the opt-in by its own test.

**Regression fixes land test-first.** When a C↔Rust output divergence is
found, the history must prove it: first a commit adding a differential
test that *fails* against the broken code (state that in the commit
message), then a separate commit with the fix that turns it green — as a
two-PR stack (red below, green above), so CI records the red run. Never land the fix and the test
in one commit — a test that has never been seen red proves nothing.
Fixing a quarantined divergence works the same way: un-ignore the test
(red), then fix (green).

**Always raise coverage.** Every pass must include tests that exercise the
code it touches — a `#[cfg(test)]` unit test for the converted function
and/or a fixture added to `scripts/fixtures-manifest.tsv` (plus its matching
`tests/rs_integration.rs` case) that differential-checks the relevant
`make` behavior against the C oracle in the `fixtures-diff` CI job. The
`cargo-llvm-cov` coverage delta for a pass must be `>= 0`; never merge a
change that lowers coverage. Prefer targets that are currently untested so
the conversion also closes a coverage gap. Measure the delta before pushing
with `./scripts/coverage-delta.sh --enforce` (see [coverage.md](coverage.md)).

1. **Remove raw pointer arithmetic** outside `ffi/`. Replace
   `.add()` / `.sub()` / `.offset()` / `.offset_from()` and walked `*p`
   cursors with slices, iterators, and indexing (`from_raw_parts`,
   `iter()`, `position()`/`rposition()`, `while let`). Computing a span as
   an address difference (`end as usize - start as usize`) is acceptable;
   producing a sub-pointer for a C call via `slice[i..].as_ptr()` is
   acceptable.
2. **Remove C ABI / FFI type leakage from internal (all-Rust-caller)
   APIs:**
   - `c_int` / `c_uint` -> `i32` / `u32` / `usize` / `bool`
     (semantic boolean flags become `bool`; indices and lengths become
     `usize`; choose unsigned when the value is never negative).
   - `*const c_char` (+ length) -> `&str`, `&[u8]`, or `&CStr`.
   - null-pointer sentinels -> `Option<T>`.
   - raw `*const` / `*mut` parameters -> references (`&` / `&mut`) when
     every caller is Rust.
   - drop needless `unsafe extern "C"` on functions that are not an FFI
     boundary.
3. Everything in **Required Refactors** below (RAII, `Result`, enums,
   newtypes, iterator/loop simplification).

### c2rust FFI artifacts are bad — remove them

`core::ffi::c_*` types, global singletons, and `#[repr(C)]` are c2rust
hold-overs, **not** requirements. The in-tree C sources are a differential-test
oracle, not a library this crate links against or shares structs/ABI with at
runtime — so none of these need to survive. Treat each as a defect to remove:

- **FFI scalar types are bad.** Replace `c_int` / `c_uint` / `c_long` /
  `c_ulong` / `c_short` / `c_char` / `size_t` / `ssize_t` and other
  `core::ffi`/libc scalars with the right native Rust type:
  `i8`/`i16`/`i32`/`i64`/`isize`, `u8`/`u16`/`u32`/`u64`/`usize`, `bool` for
  semantic flags, `char`/`u8` for bytes. Pick width and signedness from the
  actual value range, not the C type.
- **FFI string / pointer types are bad.** Replace `*const c_char` (+ length)
  with `String` / `&str` / `&[u8]` / `&CStr` / `Vec<u8>`; null-pointer
  sentinels with `Option<T>`; raw `*const` / `*mut` parameters with
  `&` / `&mut` references whenever every caller is Rust.
- **Never store a char pointer.** This applies even when the function body
  around it stays raw-pointer-heavy (unavoidable c2rust FFI-adjacent code):
  the type you *own* — a struct field, an `ExecContext` member, anything with
  a lifetime longer than one call — must be `String`, `Vec<u8>`, `CString`,
  or (borrowed) `CStr`, never `*mut c_char` / `*const c_char`. Cast to a raw
  char pointer only at the instant you hand it to an actual libc/FFI call
  (`buf.as_mut_ptr() as *mut c_char` right at the call site), then let the
  cast expire — don't let it leak back into the stored type. See the
  `PidString` (#475) and `file_seq_tmpbuf` (#476) fixes for the pattern: an
  owned `Vec<u8>`/fixed-size buffer on the context, cast to `*mut c_char`
  only at the single FFI boundary that needs it.
- **`#[repr(C)]` is bad.** Drop it; let structs use the default Rust layout.
  Keep it only for a type genuinely passed across a real FFI call you are
  keeping (e.g. a libc syscall struct).
- **`extern "C"` / `unsafe extern "C" fn` / `#[no_mangle]` are bad** on
  anything that is not a real FFI entry point. Remove them and give the
  function an idiomatic Rust signature.
- **Global singletons are bad** (see rule 2): no `static mut`, no global
  mutable state, no mirror statics. Thread ownership explicitly.

**Always work toward removing `unsafe`.** Every pass should *shrink* the unsafe
surface — turn raw derefs into borrows, FFI types into native types, and delete
the `unsafe` blocks / `unsafe fn` markers that are then no longer needed. Never
widen `unsafe`.

The only things that still gate a type change:
- **Behavior must stay identical** (differential-tested against the C oracle).
- **Integer width or signedness changes require overflow/range analysis** —
  runtime values must be unchanged.

## Forbidden Patterns
- storing `*mut c_char` / `*const c_char` in a struct field or other owned
  state — use `String` / `Vec<u8>` / `CString` / `CStr` and cast to a raw
  char pointer only at the instant of an actual libc/FFI call
- libc malloc/free ownership in Rust-facing APIs
- raw pointer arithmetic outside ffi/
- C-style out parameters
- integer error codes in internal APIs
- bool flags with semantic meaning
- pervasive unsafe blocks
- manual linked lists unless proven necessary
- global singletons: `static mut`, global mutable state, or "mirror" statics
  that shadow owned data
- null-check guards / `assert!` / extra branches added solely to silence a
  linter or security scanner around a raw deref (de-pointer it instead)
- any conversion that *adds* `unsafe` or changes observable behavior
- monolithic modules: single `.rs` files that have grown to many thousands of
  lines mixing several unrelated concerns. A 6,000-line module is a code smell,
  not a constraint to preserve — split it (see **File & Module Size** below)

## Required Refactors
- Convert char* + len -> &[u8] or &str
- Convert ownership pairs into RAII structs
- Replace errno-style APIs with Result<T, E>
- Replace tagged unions with enums
- Replace global mutable state with ownership transfer (pass owned state by
  `&`/`&mut`, `Cell`/`RefCell` for scoped interior mutability, or a scoped
  accessor) — not a new global singleton
- Replace manual allocators with Vec/Box/slab/arena crates
- Move functions into impl blocks
- Introduce newtypes for identifiers and units
- Split oversized modules (see below)

## File & Module Size

c2rust emits one giant `.rs` file per C translation unit, so several modules
have ballooned to thousands of lines (`main.rs`, `function.rs`, `read.rs`,
`job.rs`, `file.rs`, …). That is a c2rust artifact, **not** an architecture to
keep. Oversized files are hard to read, slow to compile, and bury the
boundaries between concerns — split them as you go.

**Guideline, not a hard gate:**
- Treat **~1,500 lines** as the point to start looking for a seam, and **~2,500
  lines** as "this should already have been split." Anything past **~5,000
  lines is unacceptable** and should be broken up proactively.
- Size is a smell, not the rule: a cohesive module that is naturally long is
  fine; a file mixing parsing, evaluation, and I/O is not, at any size. Split
  by **concern/responsibility**, never by an arbitrary line count.

**How to split (behavior-preserving):**
- Turn the file into a directory module: `foo.rs` → `foo/mod.rs` (or keep
  `foo.rs` plus a `foo/` dir on the 2018+ path style already used here) with
  focused submodules (`foo::parse`, `foo::eval`, `foo::print`, …). Move whole
  functions/types; do not rewrite them in the same pass.
- Keep the public API identical: re-export moved items with `pub use` so every
  caller and the `make` C-ABI surface compiles unchanged. A pure file split must
  not alter behavior, signatures, or the differential-test results.
- One module per pass. A split is its own change — do **not** combine it with a
  c2rust→idiomatic conversion in the same PR. Land the split as its own entry in
  the stack with the conversion stacked on top, so the diff stays reviewable and
  the "no behavior change" claim is easy to verify.
- Carry tests with the code they exercise, and keep the coverage delta `>= 0`.

When a conversion pass lands in a file that is already over the threshold,
prefer either splitting first (its own entry below the conversion in the stack)
or keeping the conversion small so the giant file at least stops growing.

## Migration Strategy
Phase 1:
- Compile translated code unchanged
- Add exhaustive tests

Phase 2:
- Encapsulate unsafe into modules
- Introduce safe wrappers
- Remove transmutes

Phase 3:
- Rewrite APIs idiomatically
- Introduce traits/enums/iterators
- Remove C naming/layout constraints

Phase 4:
- Performance recovery
- SIMD
- allocation reduction
- borrow-driven redesign

## Review Heuristics
- Ask: "Would a Rust developer write this from scratch?"
- If not, redesign rather than transliterate
- Unsafe must be justified with comments
- Prefer ownership over aliasing
- Prefer iterators over index loops
- Prefer slices over pointer+length

## Testing
- Differential test against original C
- Fuzz both implementations
- Use Miri on rewritten modules
- Run Clippy with -D warnings

The `src/` directory contains code generated with c2rust. 
   Here are some common patterns for cleaning up the code base to achive a more idiomatic codebase.
   
### Pattern 1: Loop Simplification

**Problem:** c2rust translates all C `for` loops to `while` loops for safety.

**Original c2rust Output:**
```rust
let mut array: [libc::c_int; 256] = [0; 256];
let mut i: libc::c_int = 0;
i = 0 as libc::c_int;
while i < 256 as libc::c_int {
    array[i as usize] = i;
    i += 1;
    i;  // pointless statement
}
```

**Cleanup Strategy:**
- Identify loops where the loop variable is not mutated inside the body
- Convert to `for` loops with ranges: `for i in 0..256`
- Use iterator methods when appropriate: `iter_mut().enumerate()`
- For simple array initialization, use `core::array::from_fn(|i| ...)`

**Keep `while` loops when:**
- The loop variable is mutated conditionally inside the loop
- The C code does something "sneaky" with the counter

### Pattern 2: Type and Casting Cleanup

**Problem:** c2rust adds explicit type annotations and integer casts everywhere to guarantee C semantics.

**Cleanup Strategy:**
1. Analyze the actual runtime values, not just the C types
2. Replace `libc::c_int` with more appropriate Rust types:
  - Use `u8` if values fit in 0-255
  - Use `usize` for loop indices and array lengths
  - Use unsigned types when values are never negative
3. Remove unnecessary casts after type changes
4. Be careful: This requires careful analysis to ensure no semantic changes

**Warning:** Time-consuming to verify correctness. Check for potential overflows.

### Pattern 3: Making Unsafe Code Safe

**Problem:** All pointer arithmetic becomes `unsafe` with `.offset()` calls.

**Original c2rust Output:**
```rust
pub unsafe extern "C" fn insertion_sort(n: libc::c_int, p: *mut libc::c_int) {
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < n {
        let tmp: libc::c_int = *p.offset(i as isize);
        let mut j: libc::c_int = i;
        while j > 0 && *p.offset((j - 1) as libc::c_int) as isize) > tmp {
            *p.offset(j as isize) = *p.offset((j - 1) as libc::c_int) as isize);
            j -= 1;
        }
        *p.offset(j as isize) = tmp;
        i += 1;
    }
}
```

**Cleanup Strategy:**
1. Remove unnecessary `extern "C"` declarations if not needed for FFI
2. Convert pointer + length pairs to slices: `&mut [T]`
3. Replace `.offset()` calls with safe indexing: `slice[i]`
4. Verify all indices are non-negative
5. Update call sites to provide slices instead of raw pointers

**After Cleanup:**
```rust
pub fn insertion_sort(p: &mut [libc::c_int]) {
    for i in 1..p.len() {
        let tmp = p[i];
        let mut j = i;
        while j > 0 && p[j - 1] > tmp {
            p[j] = p[j - 1];
            j -= 1;
        }
        p[j] = tmp;
    }
}
```

**Tools:** Use `miri` to verify memory safety correctness.

### Pattern 4: Complex Control Flow

**Problem:** `goto` statements and fall-through `switch` cases create irreducible control flow.

**c2rust Output Example:**
```rust
pub unsafe extern "C" fn adjustValue(
    mut value: *mut libc::c_int,
    mut operation: libc::c_int,
) {
    let mut current_block_1: u64;
    match operation {
        1 => {
            *value += 10;
            current_block_1 = 4407541767199398248;
        }
        2 => {
            current_block_1 = 4407541767199398248;
        }
        _ => {
            current_block_1 = 12675440807659640239;
        }
    }
    match current_block_1 {
        4407541767199398248 => {
            *value *= 2;
        }
        _ => {}
    }
}
```

**Cleanup Strategy:**
- Give meaningful names to block labels
- Consider if refactoring is worth potential code duplication
- Accept that some ugly patterns may be necessary for performance
- These constructs often appear in performance-critical code

**Reality Check:** This is difficult to improve without duplicating code blocks.

### Pattern 5: libc Function Cleanup

**Problem:** Direct libc calls are messy and lose context like:
- Conditional compilation directives
- Constant names (converted to raw numbers)
- Rust-idiomatic error handling

**Original c2rust Output:**
```rust
pub unsafe extern "C" fn fopen_output_safely(
    mut name: *mut libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    let mut fh: libc::c_int = 0;
    fh = open(
        name,
        0o1 | 0o100 | 0o200,  // Lost constant names
        0o200 | 0o400,
    );
    if fh == -1 {
        return 0 as *mut FILE;
    }
    // ... more unsafe code
}
```

**Cleanup Strategy:**
1. Replace libc calls with Rust standard library equivalents where possible
2. Use `std::fs::File` and `std::fs::OpenOptions`
3. Restore conditional compilation with `#[cfg(...)]` attributes
4. Keep some libc calls if necessary for compatibility (e.g., `fdopen` for `FILE*` handles)
5. Replace raw pointers with safe Rust types (`&Path` instead of `*mut c_char`)
6. Use `Option<T>` instead of null pointers

**After Cleanup:**
```rust
#[cfg(unix)]
fn open_output_safely(name: &Path) -> Option<*mut libc::FILE> {
    let mut opts = std::fs::File::options();
    opts.write(true).create_new(true);
    opts.mode((libc::S_IWUSR | libc::S_IRUSR) as u32);

    let file = opts.open(name).ok()?;
    let fd = file.into_raw_fd();

    let fp = unsafe { libc::fdopen(fd, WB_MODE) };
    if fp.is_null() {
        unsafe { libc::close(fd) };
        return None;
    }
    Some(fp)
}
```

**Reality Check:** This type of cleanup requires deep understanding of both C and Rust semantics. For stdio-heavy code, consider starting from scratch.

## Systematic Cleanup Process

1. **Quick Wins First**
  - Remove pointless statements like `i;` after `i += 1`
  - Clean up obvious type noise
  - Simplify simple loops

2. **Safety Improvements**
  - Convert pointers to slices where possible
  - Replace pointer arithmetic with indexing
  - Verify all indices are non-negative
  - Use `miri` to validate changes

3. **Type System Improvements**
  - Replace `libc::c_int` with appropriate Rust types
  - Use `usize` for indices
  - Remove unnecessary casts
  - Verify no overflows occur

4. **Idiomatic Rust**
  - Use iterator methods instead of manual loops
  - Replace `while` loops with `for` loops where safe
  - Use `Option<T>` instead of null pointers
  - Add proper error handling

5. **Testing After Each Change**
  - Run the full test suite
  - Run differential fuzzing
  - Check benchmarks for performance regressions
  - Use `miri` for memory safety validation

## Critical Guidelines

### DO:
- Set up testing infrastructure on day one
- Make incremental changes and test frequently
- Use `miri` to validate safety improvements
- Keep original test data for compatibility testing
- Document why certain patterns can't be cleaned up
- Accept that some ugly code may be necessary

### DON'T:
- Delay porting the test suite
- Make large refactorings without testing
- Assume type changes are safe without analysis
- Try to make everything pretty at the expense of correctness
- Forget about legacy format compatibility
- Over-optimize before establishing correctness

## Special Considerations

### Integer Type Precision
- Old C code is often imprecise about integer types
- Changing size or signedness requires careful overflow analysis
- Runtime behavior must remain identical

### Performance-Critical Code
- Complex control flow often appears in hot paths
- Sometimes ugly c2rust output must be kept for performance
- Benchmark before and after cleanup

## Semantic Patching with Coccinelle for Rust

Use Coccinelle for Rust for repository-wide, semantics-aware rewrites after the c2rust output compiles and tests exist. Treat it as a refactoring accelerator, not as a proof of correctness. Every semantic patch must be reviewed and followed by tests.

### Good Coccinelle for Rust Use Cases
- Removing repeated c2rust noise patterns across many files
- Rewriting mechanical API changes after a safe wrapper is introduced
- Finding forbidden patterns before review
- Enforcing that old c2rust shapes do not reappear
- Applying obvious local transformations where behavior is already covered by tests

### Do Not Use Coccinelle for Rust For
- Ownership redesigns that require whole-program reasoning
- Integer signedness or width changes without overflow analysis
- Pointer-to-slice conversion unless bounds and aliasing are already proven
- Control-flow rewrites involving `goto`-like `current_block_*` state machines
- Any transformation that changes public behavior without differential tests

### Suggested Semantic Patch Targets

Start with search-only patches, then promote them to rewrites once the matches are understood.

1. **Pointless statements**
   - Find generated no-op statements such as `i;` after `i += 1`.
   - Rewrite only when the expression has no side effects.

2. **Generated block labels**
   - Find `current_block_*` variables.
   - Do not automatically rewrite them. Use matches to prioritize manual control-flow cleanup.

3. **Raw pointer arithmetic**
   - Find `.offset(...)`, `.add(...)`, `.sub(...)`, and raw dereferences.
   - Prefer reporting and review over automatic rewriting.
   - Only rewrite to slice indexing after a safe wrapper exists.

4. **C ABI leftovers**
   - Find `pub unsafe extern "C" fn` outside explicit FFI modules.
   - Rewrite only internal functions whose call sites have already moved to Rust-native APIs.

5. **libc type leakage**
   - Find `libc::c_int`, `libc::c_uint`, `libc::c_char`, and pointer-heavy signatures in internal modules.
   - Do not bulk-rewrite integer types without range analysis.
   - Prefer using matches as a cleanup queue.

6. **Null pointer idioms**
   - Find `0 as *mut T`, `0 as *const T`, `ptr::null()`, and `ptr::null_mut()` in Rust-facing APIs.
   - Replace with `Option<T>` only after ownership and lifetime semantics are clear.

7. **Manual allocation idioms**
   - Find `malloc`, `calloc`, `realloc`, `free`, and libc allocation wrappers.
   - Rewrite to `Vec`, `Box`, arenas, or domain-specific allocators only module-by-module.

### Workflow

1. Write a search-only semantic patch.
2. Run it across `src/` and inspect all matches.
3. Classify matches as:
   - safe mechanical rewrite
   - needs local review
   - needs redesign
   - must remain C-shaped for now
4. Convert only the safe class into a rewrite patch.
5. Apply the patch in a small commit.
6. Run:
   - `cargo fmt`
   - `cargo clippy -- -D warnings`
   - unit tests
   - differential tests against the C implementation
   - fuzz targets, where available
   - `cargo miri test`, where applicable
7. Keep the semantic patch in `semantic-patches/` so the same cleanup can be re-run or enforced in CI.

### Repository Layout

Store semantic patches like this:

```text
semantic-patches/
  find-pointer-offsets.cocci
  find-current-blocks.cocci
  remove-pointless-statements.cocci
  find-libc-types.cocci
  find-null-pointer-idioms.cocci
```


### Project-Specific Coccinelle Semantic Patches

Keep project-specific semantic patches in `semantic-patches/`. These patches capture known c2rust cleanup cases that are safe only because the surrounding project types and modules are understood. Do not generalize them blindly.

Example patch set:

```diff
@@
@@

-#[derive(Copy, Clone)]
-#[repr(C)]
-pub struct timespec {
-    pub tv_sec: __time_t,
-    pub tv_nsec: __syscall_slong_t,
-}
+pub use crate::sys_stat::timespec;

```

```diff

@wrap_lit_add@
expression LEN;
@@

- (1).wrapping_add(LEN)
+ (1 as size_t).wrapping_add(LEN)

@wrap_lit_add_one_more@
expression LEN;
@@

- (1).wrapping_add(LEN).wrapping_add(1)
+ (1 as size_t).wrapping_add(LEN).wrapping_add(1)

@wrap_lit_add_4@
expression E;
@@

- (4).wrapping_add(E)
+ (4 as size_t).wrapping_add(E)

@wrap_lit_mul_53@
expression E;
@@

- (53).wrapping_mul(E)
+ (53 as size_t).wrapping_mul(E)
```

Notes for this patch set:
- The `timespec` rewrite is a type de-duplication rule. It should only be applied where the local generated `timespec` is layout-compatible with `crate::sys_stat::timespec`.
- The `wrapping_add` and `wrapping_mul` rewrites force integer literals to `size_t` before wrapping arithmetic. This is useful when c2rust inferred untyped integer literals but the intended C operation was `size_t` arithmetic.
- Keep rule names accurate. For example, use `wrap_lit_add_4` for `(4).wrapping_add(E)`, not `wrap_lit_mul_4`.
- After applying these patches, run `cargo fmt`, `cargo test`, and any differential tests that cover the affected arithmetic or ABI-facing structs.

### CI Usage

Use Coccinelle for Rust in CI in report-only mode for risky patterns. CI may fail on newly introduced instances of banned c2rust shapes, but it must not silently rewrite code.

Good CI checks:
- no new raw pointer arithmetic outside `ffi/` or explicitly unsafe modules
- no new `pub unsafe extern "C" fn` outside FFI boundaries
- no new `current_block_*` control-flow artifacts in rewritten modules
- no `malloc`/`free` ownership in Rust-facing APIs

### Review Rule

A Coccinelle rewrite is acceptable only when the reviewer can explain why the transformation is behavior-preserving without relying on the tool itself.


## Testing Strategy

1. **Unit Tests:** Port original test cases to `cargo test`
2. **Differential Fuzzing:** Compare Rust and C output byte-for-byte
3. **Legacy Compatibility:** Test with old file formats (10+ years old)
4. **Performance Benchmarking:** Track metrics continuously
5. **Memory Safety:** Regular `miri` runs during development

---

<!-- Commits in this repository follow Conventional Commits specification -->
