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

- [x] Core type: `ExecContext<W: Write = StdioChannel>` — `stdout`/`stderr`
      fields are `Rc<RefCell<W>>` (cheap-clone handle, matches
      `remote_backend`'s rationale). `StdioChannel` is the default sink,
      reproducing today's exact behavior (re-fetches real
      `std::io::stdout()`/`stderr()` per write, feeds the sticky
      `output::record_stdout_error`). The default type parameter means every
      existing `&ExecContext` site in the crate (399 signatures, 28 files)
      keeps compiling and behaving identically — **no call site needed to
      change** for this slice to land.
  - `ExecContext<StdioChannel>` can't derive `Default` (stdout/stderr must
    start as *different* sink values — a single generic `W::default()`
    can't express that), so it's a hand-written concrete impl instead.
  - `ExecContext<W>::with_sinks(stdout: W2, stderr: W2) -> ExecContext<W2>`
    is the escape hatch for a non-default `W` without hand-listing every
    field at each call site: build a normal `ExecContext::new(...)`/
    `::default()` for config/startup, then convert.
- [x] Proof-of-concept consumer: `output::trace_out_ctx(&ExecContext<W>, &[u8])`
      writes through `ctx.stdout`. The existing ctx-less `output::trace_out`
      becomes a compatibility wrapper that reaches the live session's context
      through the `try_with_exec_context` borrow channel (same one
      `output_context`/`stdio_traced` already use) and falls back to real
      stdout only when no context is installed (startup, bare unit tests) —
      so it now honors a buffer-backed session's sink too, not just the
      process's real stdout.
- [ ] Convert the rest of the direct `std::io::stdout()`/`stderr()` call
      sites in output.rs onto `ctx.stdout`/`ctx.stderr` (`_outputs`'s
      non-synced path, `pump_from_tmp`, the usage/version printer in
      main.rs, `close_stdout`) — each needs a live `&ExecContext<W>` at the
      call site, which most already carry; the exceptions (the `atexit`
      handler, a couple of C-ABI callbacks) go through the borrow channel
      like `trace_out` now does.
- [ ] Per-context sticky write-error tracking: `output::STDOUT_ERRNO` is
      still one process-global atomic. Move it onto `ExecContext` (a `Cell`,
      following `clock_skew_detected`'s pattern) so two sessions' write
      failures don't clobber each other; `StdioChannel::write`/`flush` need
      a `ctx` reference to record into instead of the free function.
  - Actively wanted before real multi-tenant use, not just tidiness: today
        a second buffer-backed session's write failure has nowhere per-context
        to land.
- [ ] hash.rs `hash_print_stats` and any other bare `trace_out(...)` callers
      that have a `&ExecContext` available at the call site convert to
      `trace_out_ctx` directly instead of going through the borrow-channel
      fallback (cheaper, and correct even when called from a thread/session
      that isn't "the" installed context in a multi-tenant host).
- [ ] Decide + document the concurrency story before any real multi-tenant
      use: `Rc<RefCell<W>>` is intentionally not `Send`/`Sync` — fine for one
      session per `ExecContext` clone tree, wrong for sharing one sink
      across threads. A host running sessions on separate threads (not just
      separate `ExecContext`s in one thread) needs `Arc<Mutex<W>>` instead,
      which is a mechanical follow-up once the shape above is proven out.

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
