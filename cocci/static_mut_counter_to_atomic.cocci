// Convert a c2rust `static mut <uint>` *counter* into a safe `AtomicU32`
// behind an accessor, and rewrite its read/increment sites.
//
// This is the transform applied by PRs #98 (job_slots_used), #100
// (jobserver_tokens), #109 (commands_started). Counters keep an integer type
// (NOT bool) and use fetch_add for the c2rust `wrapping_add` increments.
//
// Worked example: `commands_started` (remake.rs). To convert another, replace
// `commands_started` (snake_case ident + accessor) and `COMMANDS_STARTED`
// (new atomic storage).
//
// Run:  cfr -c cocci/static_mut_counter_to_atomic.cocci src/remake.rs --apply
//       cfr -c cocci/static_mut_counter_to_atomic.cocci src/job.rs    --apply
// Prints a diff by default; `--apply` rewrites in place. Afterwards run
// `cargo fmt` (cfr re-tokenizes), add `use std::sync::atomic::{AtomicU32,
// Ordering};` if absent and the accessor `///` doc-comment by hand, and inline
// any `let x; ... x = counter();` that clippy's `needless_late_init` flags.

// 1. Storage declaration + accessor.
@decl@
@@
-pub static mut commands_started: ::core::ffi::c_uint = 0;
+pub static COMMANDS_STARTED: AtomicU32 = AtomicU32::new(0);
+pub fn commands_started() -> ::core::ffi::c_uint {
+    COMMANDS_STARTED.load(Ordering::Relaxed)
+}

// 2. Increment: `c = c.wrapping_add(1);`  =>  `C.fetch_add(1, Relaxed);`
//    (AtomicU32::fetch_add wraps on overflow, matching wrapping_add.)
@incr@
@@
-commands_started = commands_started.wrapping_add(1);
+COMMANDS_STARTED.fetch_add(1, Ordering::Relaxed);

// 3. Comparison read: `commands_started > e`  =>  `commands_started() > e`
@read_gt@
expression e;
@@
-commands_started > e
+commands_started() > e

// 4. Snapshot read: `x = commands_started;`  =>  `x = commands_started();`
@read_snapshot@
identifier x;
@@
-x = commands_started;
+x = commands_started();
