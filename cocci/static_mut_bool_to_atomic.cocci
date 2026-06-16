// Convert a c2rust `static mut <int> = 0/1` *boolean* flag into a safe
// `AtomicBool` behind an accessor, and rewrite its read/write sites.
//
// This is the transform applied by PRs #102-#105, #108 (rebuilding_makefiles,
// second_expansion, posix_pedantic, snapped_deps, good_stdin_used,
// all_secondary, stdio_traced, ...). It is a *template*: Coccinelle cannot infer
// the new SCREAMING_CASE storage name or accessor for you, so this file is
// written for ONE concrete flag and you copy/edit it per symbol.
//
// Worked example: `stdio_traced` (output.rs). To convert another flag, replace
// the tokens `stdio_traced` (snake_case ident + accessor) and `STDIO_TRACED`
// (new atomic storage), and adjust `pub` visibility.
//
// Run:  cfr -c cocci/static_mut_bool_to_atomic.cocci src/output.rs --apply
// Prints a diff by default; `--apply` rewrites in place. Run `cargo fmt`
// afterwards (cfr re-tokenizes, so spacing/blank lines need reformatting), and
// add `use std::sync::atomic::{AtomicBool, Ordering};` plus the accessor's
// `///` doc-comment by hand (cfr does not edit attributes/doc-comments).

// 1. Storage declaration + accessor.
@decl@
@@
-pub static mut stdio_traced: ::core::ffi::c_uint = 0;
+pub static STDIO_TRACED: AtomicBool = AtomicBool::new(false);
+pub fn stdio_traced() -> bool {
+    STDIO_TRACED.load(Ordering::Relaxed)
+}

// 2. Truthy read: `flag != 0`  =>  `flag()`
@read_true@
@@
-stdio_traced != 0
+stdio_traced()

// 3. Falsy read: `flag == 0`  =>  `!flag()`
@read_false@
@@
-stdio_traced == 0
+!stdio_traced()

// 4. Set-true write: `flag = 1;`  =>  `STDIO_TRACED.store(true, Relaxed);`
@write_true@
@@
-stdio_traced = 1;
+STDIO_TRACED.store(true, Ordering::Relaxed);

// 5. Set-false write: `flag = 0;`  =>  `STDIO_TRACED.store(false, Relaxed);`
@write_false@
@@
-stdio_traced = 0;
+STDIO_TRACED.store(false, Ordering::Relaxed);
