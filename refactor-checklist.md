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
- [ ] temp files: `get_tmpfile`/`fdopen` (misc.rs), the `-f -` stdin spool
      `fread`/`fwrite` loop (main.rs), jobserver temp `fclose` (posixos.rs)
- [ ] makefile reading: `ebuf.fp` `fopen`/`fdopen`/`fclose` + `readline`
      (read.rs) → `BufReader`-style owned reader
- [ ] output.rs: the `fwrite` dump in `output_write`, `fputs`/`fflush` in
      the message writers
- [ ] the `-p` data-base printers: `printf`/`fputs`/`putchar` in
      variable.rs, file.rs, dir.rs, vpath.rs, strcache.rs, main.rs
      (`print_version`/`print_data_base`) → one shared safe byte-writer
      (rule.rs already is; its flush ordering is the pattern to follow)
- [ ] debug traces: `printf`+`fflush(stdout)` pairs in remake.rs, job.rs,
      implicit.rs, expand.rs, function.rs, commands.rs, read.rs, posixos.rs
- [ ] stdout plumbing: `setvbuf`/`fileno`/`check_io_state` (main.rs),
      `close_stdout` atexit handler — last, once no libc writers remain
- [ ] file ops with all-Rust callers: `unlink`/`chdir`/`getcwd` →
      `std::fs`/`std::env`
