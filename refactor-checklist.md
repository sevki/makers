# Refactor Checklist

## Per-pass gate (every cleanup PR)
- [ ] Touched code has a `#[cfg(test)]` unit test and/or a fixture in
      `scripts/fixtures-manifest.tsv` (plus its matching
      `tests/rs_integration.rs` case) compared byte-for-byte against the C
      oracle in the `fixtures-diff` CI job.
- [ ] Coverage delta is `>= 0`: `./scripts/coverage-delta.sh --enforce` passes.
      See [coverage.md](coverage.md).

## Generic output sink on `ExecContext<W: Write>` (multi-tenant groundwork)
Follows directly from the libc-stdio campaign above: with every make writer
already on `std::io`, the next process global to retire is *which* stdout/
stderr a write goes to, so a future host can run several sessions in one
process with each session's recipe/trace/diagnostic output landing in its
own buffer instead of the real process stdout. `execctx.rs`'s existing
per-context state (`remote_backend`, `handling_fatal_signal`, ...) already
called this out as a "future multi-tenant host" concern; this is that slice
for output.

- [x] Core type: `ExecContext<Out: Write = StdoutSink, Err: Write = StderrSink>`
      — **two independent type parameters**, not one shared `W`: stdout and
      stderr are different channels with different content, and a caller may
      want (say) stdout captured into a buffer while stderr still goes to
      the real process stderr. `stdout`/`stderr` fields are `Rc<RefCell<_>>`
      (cheap-clone handle, matches `remote_backend`'s rationale).
      `StdoutSink`/`StderrSink` are the default sinks, reproducing today's
      exact behavior (re-fetch real `std::io::stdout()`/`stderr()` per
      write; `StdoutSink` feeds the sticky `output::record_stdout_error`,
      matching the C original's `ferror(stdout)`-only check). The default
      type parameters mean every existing `&ExecContext` site in the crate
      (399 signatures, 28 files) keeps compiling and behaving identically —
      **no call site needed to change** for this slice to land.
  - Two independent parameters, once each defaulted, let `#[derive(Default)]`
    work again (`Out::default()` and `Err::default()` resolve
    independently) — no hand-written field-by-field impl needed, unlike the
    single-`W` version this replaced (where one generic `W::default()`
    couldn't give stdout/stderr *different* starting values).
  - Bare `ExecContext::default()` call sites (mostly in tests) stopped
    resolving once there were two defaulted parameters: rustc's
    default-type-parameter substitution doesn't reliably fire through
    `Default` trait dispatch at an unannotated call site with more than one
    defaulted parameter. Fixed with a non-generic inherent
    `ExecContext::default()` shim (unqualified path calls prefer an
    inherent method over a trait method of the same name, and the inherent
    impl block is concrete, so no dispatch ambiguity) —
    `#[allow(clippy::should_implement_trait)]`'d with a comment explaining
    why. `ExecContext::new(...)` was unaffected (already a non-generic
    inherent method).
  - `ExecContext<Out, Err>::with_sinks(stdout: Out2, stderr: Err2) ->
    ExecContext<Out2, Err2>` is the escape hatch for non-default sinks
    without hand-listing every field at each call site: build a normal
    `ExecContext::new(...)`/`::default()` for config/startup, then convert
    — independently, so e.g. `with_sinks(Cursor::new(Vec::new()),
    Vec::new())` is fine.
- [x] Proof-of-concept consumer: `output::trace_out_ctx(&ExecContext<Out, Err>, &[u8])`
      writes through `ctx.stdout`. The existing ctx-less `output::trace_out`
      becomes a compatibility wrapper that reaches the live session's context
      through the `try_with_exec_context` borrow channel (same one
      `output_context`/`stdio_traced` already use) and falls back to real
      stdout only when no context is installed (startup, bare unit tests) —
      so it now honors a buffer-backed session's sink too, not just the
      process's real stdout.
  - Unit-tested with a real `std::io::Cursor<Vec<u8>>` sink (not just a bare
    `Vec<u8>`), including seeking back and reading what was written, and
    with stdout/stderr given genuinely different concrete types
    (`Cursor<Vec<u8>>` + plain `Vec<u8>`) to exercise the two-parameter
    split directly.
- [x] Converted the rest of the direct `std::io::stdout()`/`stderr()` call
      sites in output.rs/main.rs onto `ctx.stdout`/`ctx.stderr`: `_outputs`'s
      non-synced path, `pump_perror`/`pump_from_tmp`, `out_of_memory` (via
      the `try_with_exec_context` borrow channel plus a real-stdout
      fallback), `print_usage`'s stdout/stderr writes, and `main_0`'s
      pre-`exec` flush pair. `print_version`/`print_usage` also converted
      their bare `trace_out(...)` calls to `trace_out_ctx(ctx, ...)` since
      they already carry a `&ExecContext`.
  - Scoped down from the original "each needs a live `&ExecContext<Out,
    Err>`" plan: fully genericizing `_outputs`/`outputs`/`error`/`fatal`/
    `message` would transitively require genericizing everything *they*
    call too (jobserver/osync helpers with nothing to do with I/O) —
    scope creep well past this slice. These functions stay pinned to the
    default sink type (`ctx: &ExecContext`, meaning
    `ExecContext<StdoutSink, StderrSink>`); a buffer-backed
    `ExecContext<Out, Err>` still can't call `error()`/`fatal()`/
    `message()` directly. Full genericization of the printer family is its
    own future slice (see below).
  - `pump_copy`'s error-reporting is a caller-supplied `report_err`
    closure rather than a baked-in `ctx.stderr` access: `pump_from_tmp`'s
    to-stderr case has the pump destination and the error sink be the
    *same* `RefCell`, so reporting through a fresh `ctx.stderr.borrow_mut()`
    from inside `pump_copy` would double-borrow-panic. The closure lets
    each caller decide — reuse the already-borrowed `dst` when it's the
    same stream, or borrow the separate one when it isn't.
  - `_outputs` also dropped its `unsafe fn`/`extern "C"` markers (never
    used as a function pointer) in favor of safe `Option<&mut output>`/
    `&CStr` parameters, with only the sync-fd fast path's raw syscalls
    (lseek/writebuf — no safe std equivalent for append-without-a-`File`
    pattern) behind a narrow internal `unsafe` block; `outputs` (still
    `unsafe fn`, taking the raw pointers its own C-ABI callers pass)
    does the `CStr::from_ptr`/`.as_mut()` conversion at that one boundary.
- [ ] Genericize the printer family (`outputs`/`error`/`fatal`/`message`/
      `pfatal_with_name`/`perror_with_name`/`log_working_directory`/
      `output_start`/`output_dump`) over `<Out: Write, Err: Write>` so a
      buffer-backed `ExecContext<Out, Err>` can actually use them — the
      scope cut from the item above. Function-generic inference (not
      struct-default substitution) means existing call sites shouldn't need
      to change, the same way `trace_out_ctx` didn't force any; but it
      pulls in `osync_acquire`/`osync_release`/`setup_tmpfile`
      (posixos.rs) too, since `output_dump` calls them with a ctx of
      matching type — verify that chain doesn't spread further before
      committing to it.
- [x] Per-context sticky write-error tracking — investigated, not
      applicable as originally framed. `output::STDOUT_ERRNO` stays the
      process-global atomic it is: `close_stdout` is an `atexit` handler
      with no `&ExecContext` reachable at process-exit time (no live
      context to move the flag onto), so the global is what makes that
      check possible at all, not a stopgap to remove. Separately, a
      buffer-backed session's custom `Out` sink already "reports its own
      failures how it sees fit" (see `StdoutSink`'s doc comment) — it
      tracks its own write errors in its own `Write` impl and has no need
      for `ExecContext` to do it on its behalf. No code change needed.
- [x] hash.rs `hash_print_stats` — checked; it already routes through
      `output::trace_out` (ctx-less) rather than a bare `std::io::stdout()`
      call, and has no `&ExecContext` at its own call site to upgrade to
      `trace_out_ctx` (its callers in variable.rs/file.rs don't thread one
      through either) — leaving it on the borrow-channel fallback is
      correct as-is; no change needed.
- [x] Decide + document the concurrency story — investigated, no action
      needed yet. `Rc<RefCell<_>>` is intentionally not `Send`/`Sync`: fine
      as long as each session's `ExecContext` is created, used, and dropped
      on one thread, never handed off or read from another. Nothing in this
      codebase does that today — there's no async runtime and no threading
      at all, so there's no concrete concurrent-access requirement to design
      against. `Arc<Mutex<_>>` would only be needed if something outside a
      session's own thread ever needed to touch its `ExecContext` (e.g. a
      supervisor thread streaming output mid-build); revisit if/when that
      requirement actually exists rather than speculatively wrapping it now.

## Shared FILE / `_IO_FILE` cleanup
- [x] Replace local `_IO_FILE` clones / `FILE` aliases with `crate::ffi_types::{_IO_codecvt, _IO_marker, _IO_wide_data, FILE}` in:
  - [x] `src/vpath.rs`
  - [x] `src/variable.rs`
  - [x] `src/rule.rs`
  - [x] `src/remake.rs`
  - [x] `src/posixos.rs`
  - [x] `src/implicit.rs`
  - [x] `src/dir.rs`
  - [x] `src/commands.rs`
  - [x] `src/job.rs`
  - [x] `src/function.rs`
  - [x] `src/file.rs`
  - [x] `src/expand.rs`
  - [x] `src/read.rs`
  - [x] `src/output.rs`
  - [x] `src/strcache.rs`

## Shared hash-table type cleanup
- [x] Canonical `hash_table` lives in `src/hash.rs`
- [x] Move remaining `hash_*` aliases to `crate::hash::*` instead of `crate::file::*` in:
  - [x] `src/vpath.rs`
  - [x] `src/variable.rs`
  - [x] `src/rule.rs`
  - [x] `src/remake.rs`
  - [x] `src/loadapi.rs`
  - [x] `src/implicit.rs`
  - [x] `src/commands.rs`
  - [x] `src/job.rs`
  - [x] `src/function.rs`
  - [x] `src/expand.rs`
  - [x] `src/read.rs`
  - [x] `src/main.rs`
- [x] `src/file.rs` uses `crate::hash::*`
- [x] `src/default.rs` uses `crate::hash::*`
- [x] `src/dir.rs` uses `crate::hash::*`
- [x] `src/strcache.rs` uses `crate::hash::*`

## Shared `stat` cleanup
- [x] Canonical `stat` lives in `src/sys_stat.rs` (layout-equivalent to glibc `struct stat64` on x86_64 Linux)
- [x] Replace local `stat` structs with a shared canonical `stat` type in:
  - [x] `src/read.rs`
  - [x] `src/job.rs`
  - [x] `src/function.rs`
  - [x] `src/arscan.rs`
  - [x] `src/vpath.rs`
  - [x] `src/remake.rs`
  - [x] `src/dir.rs`
  - [x] `src/posixos.rs`
  - [x] `src/misc.rs`
  - [x] `src/commands.rs`
- [x] Replace local `extern fn stat/lstat(... *mut stat)` seams with shared `stat` type signatures in:
  - [x] `src/read.rs`
  - [x] `src/job.rs`
  - [x] `src/function.rs`
  - [x] `src/dir.rs`
  - [x] `src/commands.rs`
  - [x] `src/remake.rs`
  - [x] `src/misc.rs`
  - [x] `src/vpath.rs`
- [x] Local `timespec` clones unified to `crate::sys_stat::timespec` (re-export of `libc::timespec`) in: `arscan`, `commands`, `dir`, `file`, `function`, `job`, `misc`, `posixos`, `read`, `remake`, `vpath`

## Completed structural unifications
- [x] Shared `Floc`
- [x] Shared `File` / `Dep` / `Commands` / `VariableSetList`
- [x] Shared `hash_table` struct definition reduced to `src/hash.rs`
- [x] Duplicate `file` cluster structs eliminated from all modules

## Dependency graph: idiomatic Rust conversion (no C ABI)
- [x] `File`, `Dep`, `GoalDep` are plain Rust structs — no `#[repr(C)]`, no c2rust bitfields; flags are `bool`, status fields are real enums (`UpdateStatus`, `CommandState` with `PartialOrd`/`Default`)
- [x] `goaldep` deduplicated (was cloned in `remake`/`main`/`read`) into shared `GoalDep` in `src/file.rs`
- [x] `rule` struct deduplicated (implicit.rs clone removed); CamelCase names: `Rule`, `NameSeq`, `PatDeps`, `TryRule`
- [x] Per-module `us_*`/`cs_*` const blocks and `file`/`dep`/`commands` lowercase aliases deleted
- [x] Prefix-punning eliminated (was UB once `repr(C)` was dropped):
  - `parse_file_seq` and `ar_glob` are generic over the `SeqNode` trait instead of taking a node `size` and casting `NameSeq*`→`Dep*`
  - `update_goal_chain` works on a `GoalDep` chain (`copy_goal_chain`) instead of casting goals to `*mut Dep`
  - shuffle is generic over `ShuffleNode` (`Dep`/`GoalDep`) instead of shuffling goals through a `Dep` cast
  - `free_dep_chain`/`free_goaldep` walk their own node type (`free_seq_chain<T: NextLinked>`)
- [x] `child` struct deduplicated (commands.rs clone removed; job.rs is canonical)
- [x] Cross-module `extern "C"` declarations mentioning graph types replaced with direct `use crate::…` imports (~70 decls)
- [x] `SeqIter`/`seq_iter` chain iterator + `File::deps_iter`; ~30 manual `while !d.is_null()` traversals converted to iterators
- [ ] Remaining: duplicate `variable` struct clones per module (bridged with casts in `job.rs`/`rule.rs`/`variable.rs` for now)
- [ ] Remaining: `#[no_mangle] extern "C"` on definitions (kept while hash-table callbacks and remaining extern decls need C fn pointers)

### Semantic patches (Coccinelle for Rust)
The term-level rewrites of the dependency-graph conversion are captured as
`cfr` semantic patches (preferred over regex scripts for future passes):
- `semantic-patches/depgraph_bitfield_accessors.cocci` — bitfield accessors → bool fields (generated; regenerate, don't hand-edit)
- `semantic-patches/depgraph_status_enums.cocci` — `us_*`/`cs_*` consts → `UpdateStatus`/`CommandState` variants (delete the per-module const *definitions* first)

cfr gotchas found (cfr 2024-era binary): `--suppress-diff` silently reports
"Sites changed: 0" and skips transformation — never use it; occasional
per-file panics (`index out of bounds` in control-flow parsing) — rerun or
fall back to manual edit for that file. Not expressible in cocci: struct
redefinition + trait/generic introduction (SeqNode/ShuffleNode), extern-block
decl removal with import insertion (scripts/depgraph_extern_cleanup.py).

## libc stdio/file io → std::io / std::fs (campaign)
Each slice is its own PR, byte-identical against the C oracle (the split-
buffer hazard between libc `FILE` buffers and Rust's `std::io` buffers is
exactly why this goes clump by clump, never file-by-file mixing streams).
- [x] `$(file <…)`/`$(file >…)` (function.rs) and the `dbg` debug log
      (misc.rs) — `std::fs::File`/`OpenOptions`, EINTR retries preserved,
      fatal messages byte-identical; `file_func` fixture pins it
- [x] temp files: `get_tmpfile` returns `std::fs::File` (misc.rs), the
      `-f -` stdin spool reads `std::io::stdin` (main.rs); still libc:
      the `tmpfile()` fallback in posixos.rs `os_anontmp`
- [x] output.rs: `_outputs`' non-synced fallback writes through Rust
      stdout/stderr (every make message), and `pump_from_tmp` pumps the
      output-sync temp fd via `std::fs::File`/`Read`/`Write` (libc-stream
      flush kept at entry until the printf callers convert)
- [x] makefile reading: `ebuffer.fp` is a raw pointer to an owned
      `MakefileReader` (BufReader over std::fs::File; read/lines.rs) with an
      fgets-compatible method and errno-carrying error() for the ferror
      path; fopen/fdopen/fclose/fgets/ferror/fileno externs dropped from
      read.rs
- [x] the `-p` data-base printers + `--version`: every stdout printer in
      variable.rs, file.rs, dir.rs, vpath.rs, strcache.rs, commands.rs
      (recipe printers), and main.rs (`print_version`, data-base
      header/footer, usage banner) goes through `output::trace_out`/
      `trace_parts`; hash.rs `hash_print_stats` moved with the
      stdout-plumbing slice
- [x] posixos.rs: the `tmpfile()` fallback in `os_anontmp` is a std::fs
      create-and-unlink helper (`anon_unlinked_tmp`), and its two debug
      traces write through the new shared `output::trace_out`
- [x] raw-fd helpers: `writebuf`/`readbuf` (misc.rs) are `write_all` /
      an EINTR-retrying `Read` loop over a borrowed `std::fs::File`
- [x] debug traces, batch 1: remake.rs (all 23 sites incl. the printf+puts
      combo), job.rs (all 15), implicit.rs (`dbs`), and `print_spaces`
      (misc.rs) route through `output::trace_out`/`trace_parts`
- [x] debug traces, batch 2: expand.rs (shell-export recursion), function.rs
      (batch-file cleanup), read.rs (Reading makefiles/makefile, BOM), and
      main.rs (jobserver/mutex/shuffle/updating/loop/Re-executing) route
      through `output::trace_out`/`trace_parts`; commands.rs's two printf
      sites are `-p` recipe printers and move with that slice
- [x] stdout plumbing: hash.rs `hash_print_stats` (the last libc stdout
      writer) formats via `output::trace_out`; `close_stdout` flushes Rust
      stdout and reads a sticky write-errno (`output::record_stdout_error`,
      fed by every Rust stdout writer — the `ferror` equivalent); `setvbuf`
      dropped (Rust stdout is a `LineWriter`); `fileno(stdout/stderr)` →
      the constant fds in main.rs/output.rs/`check_io_state` (job.rs keeps
      its child-fd `fileno` plumbing, a separate concern)
- [x] file ops with all-Rust callers: every `unlink` goes through
      `misc::unlink_c` (std::fs::remove_file with the C EINTR retry and
      errno preserved for perror paths); main.rs `chdir`/`getcwd` go
      through `chdir_c`/`getcwd_into` (std::env, errno preserved, the
      fixed `current_directory` buffer semantics kept)
