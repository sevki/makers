---
# Fill in the fields below to create a basic custom agent for your repository.
# The Copilot CLI can be used for local testing: https://gh.io/customagents/cli
# To make this agent available, merge this file into the default repository branch.
# For format details, see: https://gh.io/customagents/config

name: c2rust-translator
description: translates c2rust generated code to idiomatic rust
---

# c2rust idiomatic rust translator 

# Agent Instructions: Translating C to Rust with c2rust

# C2Rust Porting Rules

## Goals
1. Preserve behavior first
2. Reduce unsafe incrementally
3. Replace C patterns with Rust abstractions
4. Never preserve C architecture blindly

## Forbidden Patterns
- libc malloc/free ownership in Rust-facing APIs
- raw pointer arithmetic outside ffi/
- C-style out parameters
- integer error codes in internal APIs
- bool flags with semantic meaning
- pervasive unsafe blocks
- manual linked lists unless proven necessary

## Required Refactors
- Convert char* + len -> &[u8] or &str
- Convert ownership pairs into RAII structs
- Replace errno-style APIs with Result<T, E>
- Replace tagged unions with enums
- Replace global mutable state with Arc/Mutex/RwLock or ownership transfer
- Replace manual allocators with Vec/Box/slab/arena crates
- Move functions into impl blocks
- Introduce newtypes for identifiers and units

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
