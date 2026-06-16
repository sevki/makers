// Convert a c2rust `static mut <uint>` *counter* into a safe `AtomicU32`
// behind an accessor, and rewrite its read/increment sites.
//
// This is the transform applied by PRs #98 (job_slots_used), #100
// (jobserver_tokens), #109 (commands_started). Counters keep an integer type
// (NOT bool) and use fetch_add for the c2rust `wrapping_add` increments.
//
// Template written for ONE concrete counter, `commands_started` (remake.rs).
// To convert another, replace:
//   commands_started   -> existing snake_case identifier (also the accessor)
//   COMMANDS_STARTED   -> new atomic storage name
//
// Run (per file that mentions the counter):
//   cfr --rule-file cocci/static_mut_counter_to_atomic.cocci --rs-file src/remake.rs --o-place .
//   cfr --rule-file cocci/static_mut_counter_to_atomic.cocci --rs-file src/job.rs    --o-place .
//
// NOTE: unvalidated here (cfr host unreachable); review the diff. The accessor
// doc-comment is added by hand. After running, also inline any binding that
// clippy's `needless_late_init` flags (e.g. `let x; ...; x = counter();`), and
// add `use std::sync::atomic::{AtomicU32, Ordering};` if absent.

// 1. The storage declaration + accessor.
@decl@
@@
- pub static mut commands_started: ::core::ffi::c_uint = 0;
+ pub static COMMANDS_STARTED: AtomicU32 = AtomicU32::new(0);
+
+ pub fn commands_started() -> ::core::ffi::c_uint {
+     COMMANDS_STARTED.load(Ordering::Relaxed)
+ }

// 2. Increment: `c = c.wrapping_add(1);`  =>  `C.fetch_add(1, Relaxed);`
//    (AtomicU32::fetch_add wraps on overflow, matching wrapping_add.)
@incr@
@@
- commands_started = commands_started.wrapping_add(1);
+ COMMANDS_STARTED.fetch_add(1, Ordering::Relaxed);

// 3. Plain read used in an expression: `commands_started`  =>  `commands_started()`
//    Constrained to comparison contexts so we do not touch the decl/incr rules.
@read_gt@
expression e;
@@
- commands_started > e
+ commands_started() > e

@read_snapshot@
identifier x;
@@
- x = commands_started;
+ x = commands_started();
