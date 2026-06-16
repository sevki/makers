// Convert a c2rust `static mut <int> = 0/1` *boolean* flag into a safe
// `AtomicBool` behind an accessor, and rewrite its read/write sites.
//
// This is the transform applied by PRs #102–#105, #108 (rebuilding_makefiles,
// second_expansion, posix_pedantic, snapped_deps, good_stdin_used,
// all_secondary, stdio_traced, ...). It is a *template*: Coccinelle cannot infer
// the new SCREAMING_CASE storage name or accessor for you, so this file is
// written for ONE concrete flag and you copy/edit it per symbol.
//
// Here the worked example is `stdio_traced` (output.rs). To convert another
// flag, replace the four tokens:
//   stdio_traced   -> the existing snake_case identifier (also the accessor)
//   STDIO_TRACED   -> the new atomic storage name
// and adjust `pub`/visibility as needed.
//
// Run (per file that mentions the flag):
//   cfr --rule-file cocci/static_mut_bool_to_atomic.cocci --rs-file src/output.rs --o-place .
//
// NOTE: built against the documented coccinelle-for-rust SmPL subset; it was not
// executed in CI here (the cfr host was unreachable), so review the diff before
// committing. `cfr` does not reliably edit doc-comments/attributes attached to
// items, so the accessor doc-comment is added by hand after the run.

// 1. The storage declaration + accessor.
@decl@
@@
- pub static mut stdio_traced: ::core::ffi::c_uint = 0;
+ pub static STDIO_TRACED: AtomicBool = AtomicBool::new(false);
+
+ pub fn stdio_traced() -> bool {
+     STDIO_TRACED.load(Ordering::Relaxed)
+ }

// 2. Truthy read site: `flag != 0`  =>  `flag()`
@read_true@
@@
- stdio_traced != 0
+ stdio_traced()

// 3. Falsy read site: `flag == 0`  =>  `!flag()`
@read_false@
@@
- stdio_traced == 0
+ !stdio_traced()

// 4. Set-true write: `flag = 1;`  =>  `STDIO_TRACED.store(true, Relaxed);`
@write_true@
@@
- stdio_traced = 1;
+ STDIO_TRACED.store(true, Ordering::Relaxed);

// 5. Set-false write: `flag = 0;`  =>  `STDIO_TRACED.store(false, Relaxed);`
@write_false@
@@
- stdio_traced = 0;
+ STDIO_TRACED.store(false, Ordering::Relaxed);
