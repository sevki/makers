# Refactor Checklist

## Per-pass gate (every cleanup PR)
- [ ] Touched code has a `#[cfg(test)]` unit test and/or a fixture in
      `scripts/fixtures-manifest.tsv` (plus its matching
      `tests/rs_integration.rs` case) compared byte-for-byte against the C
      oracle in the `fixtures-diff` CI job.
- [ ] Coverage delta is `>= 0`: `./scripts/coverage-delta.sh --enforce` passes.
      See [coverage.md](coverage.md).

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
- [ ] makefile reading: `ebuf.fp` `fopen`/`fdopen`/`fclose` + `readline`
      (read.rs) → `BufReader`-style owned reader
- [ ] the `-p` data-base printers: `printf`/`fputs`/`putchar` in
      variable.rs, file.rs, dir.rs, vpath.rs, strcache.rs, main.rs
      (`print_version`/`print_data_base`) → one shared safe byte-writer
      (rule.rs already is; its flush ordering is the pattern to follow)
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
- [ ] stdout plumbing: `setvbuf`/`fileno`/`check_io_state` (main.rs),
      `close_stdout` atexit handler — last, once no libc writers remain
- [ ] file ops with all-Rust callers: `unlink`/`chdir`/`getcwd` →
      `std::fs`/`std::env`
