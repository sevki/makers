# Refactor Checklist

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
