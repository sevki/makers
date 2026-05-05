# Agent Instructions: Translating C to Rust with c2rust

MyCrush directory contains code generated with c2rust. 
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
   - Use unsigned types when values are never negative# Agent Instructions: Translating C to Rust with c2rust
   
   MyCrush directory contains code generated with c2rust. 
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
   
   
   ## Testing Strategy
   
   1. **Unit Tests:** Port original test cases to `cargo test`
   2. **Differential Fuzzing:** Compare Rust and C output byte-for-byte
   3. **Legacy Compatibility:** Test with old file formats (10+ years old)
   4. **Performance Benchmarking:** Track metrics continuously
   5. **Memory Safety:** Regular `miri` runs during development
   
   ---
   
   <!-- Commits in this repository follow Conventional Commits specification -->
   
    
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


## Testing Strategy

1. **Unit Tests:** Port original test cases to `cargo test`
2. **Differential Fuzzing:** Compare Rust and C output byte-for-byte
3. **Legacy Compatibility:** Test with old file formats (10+ years old)
4. **Performance Benchmarking:** Track metrics continuously
5. **Memory Safety:** Regular `miri` runs during development

---

<!-- Commits in this repository follow Conventional Commits specification -->
