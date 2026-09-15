#[cfg(target_family = "wasm")]
use crate::compat::{getloadavg, stpcpy, strsignal};
pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __pid_t, __sig_atomic_t, __syscall_slong_t, __time_t, __uid_t, pid_t, sig_atomic_t, size_t,
    ssize_t, time_t, uintmax_t,
};
#[cfg(unix)]
use libc::{getloadavg, stpcpy, strsignal};
use {
    crate::{
        file::{
            cs_finished, cs_running, us_failed, us_question, us_success, CommandState, FileId,
            FileNode, UpdateStatus, VariableSet, VariableSetList,
        },
        misc::{xmalloc, xstrdup},
        recipe::RecipeLineFlags,
        stdio::FILE,
    },
    libc::{
        __errno_location, close, free, getenv, open, remove, sprintf, strchr, strcmp, strerror,
    },
    std::{
        sync::atomic::Ordering,
        time::{SystemTime, UNIX_EPOCH},
    },
};
extern "C" {
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> i32;
    fn read(__fd: i32, __buf: *mut ::core::ffi::c_void, __nbytes: size_t) -> ssize_t;
    static mut environ: *mut *mut ::core::ffi::c_char;
    fn confstr(__name: i32, __buf: *mut ::core::ffi::c_char, __len: size_t) -> size_t;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
// `sigemptyset`/`sigprocmask`/`execvp`/`wait`/`waitpid`/`lseek` are declared
// by hand here (not through `libc`, which does gate them per-target) so they
// link fine on unix but are genuinely absent from wasm32-wasip1's libc: WASI
// has no signals and no fork/exec, and there is no real job-control path on
// wasm to begin with (see `spawn_via_std` below). The wasm stand-ins report
// failure so the crate links and behaves sanely if ever reached.
#[cfg(unix)]
extern "C" {
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn execvp(__file: *const ::core::ffi::c_char, __argv: *const *mut ::core::ffi::c_char) -> i32;
    fn wait(__stat_loc: *mut i32) -> __pid_t;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut i32, __options: i32) -> __pid_t;
}
#[cfg(target_family = "wasm")]
unsafe fn sigemptyset(_set: *mut sigset_t) -> i32 {
    0
}
#[cfg(target_family = "wasm")]
unsafe fn sigprocmask(_how: i32, _set: *const sigset_t, _oset: *mut sigset_t) -> i32 {
    0
}
#[cfg(target_family = "wasm")]
unsafe fn lseek(_fd: i32, _offset: __off_t, _whence: i32) -> __off_t {
    -1
}
#[cfg(target_family = "wasm")]
unsafe fn execvp(
    _file: *const ::core::ffi::c_char,
    _argv: *const *mut ::core::ffi::c_char,
) -> i32 {
    *__errno_location() = libc::ENOSYS;
    -1
}
#[cfg(target_family = "wasm")]
unsafe fn wait(_stat_loc: *mut i32) -> __pid_t {
    -1
}
#[cfg(target_family = "wasm")]
unsafe fn waitpid(_pid: __pid_t, _stat_loc: *mut i32, _options: i32) -> __pid_t {
    -1
}
pub type sigset_t = crate::entry::SigsetT;
pub use crate::sys_stat::{stat, timespec};
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _CS_V7_ENV: C2RustUnnamed = 1149;
pub const _CS_V6_ENV: C2RustUnnamed = 1148;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: C2RustUnnamed = 1147;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: C2RustUnnamed = 1146;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: C2RustUnnamed = 1145;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: C2RustUnnamed = 1144;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: C2RustUnnamed = 1143;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: C2RustUnnamed = 1142;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: C2RustUnnamed = 1141;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: C2RustUnnamed = 1140;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: C2RustUnnamed = 1139;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: C2RustUnnamed = 1138;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: C2RustUnnamed = 1137;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: C2RustUnnamed = 1136;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: C2RustUnnamed = 1135;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: C2RustUnnamed = 1134;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: C2RustUnnamed = 1133;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: C2RustUnnamed = 1132;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: C2RustUnnamed = 1131;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: C2RustUnnamed = 1130;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: C2RustUnnamed = 1129;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: C2RustUnnamed = 1128;
pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: C2RustUnnamed = 1127;
pub const _CS_POSIX_V6_LP64_OFF64_LIBS: C2RustUnnamed = 1126;
pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: C2RustUnnamed = 1125;
pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: C2RustUnnamed = 1124;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: C2RustUnnamed = 1123;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: C2RustUnnamed = 1122;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: C2RustUnnamed = 1121;
pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: C2RustUnnamed = 1120;
pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: C2RustUnnamed = 1119;
pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: C2RustUnnamed = 1118;
pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: C2RustUnnamed = 1117;
pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: C2RustUnnamed = 1116;
pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS: C2RustUnnamed = 1115;
pub const _CS_XBS5_LPBIG_OFFBIG_LIBS: C2RustUnnamed = 1114;
pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS: C2RustUnnamed = 1113;
pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS: C2RustUnnamed = 1112;
pub const _CS_XBS5_LP64_OFF64_LINTFLAGS: C2RustUnnamed = 1111;
pub const _CS_XBS5_LP64_OFF64_LIBS: C2RustUnnamed = 1110;
pub const _CS_XBS5_LP64_OFF64_LDFLAGS: C2RustUnnamed = 1109;
pub const _CS_XBS5_LP64_OFF64_CFLAGS: C2RustUnnamed = 1108;
pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS: C2RustUnnamed = 1107;
pub const _CS_XBS5_ILP32_OFFBIG_LIBS: C2RustUnnamed = 1106;
pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS: C2RustUnnamed = 1105;
pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS: C2RustUnnamed = 1104;
pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS: C2RustUnnamed = 1103;
pub const _CS_XBS5_ILP32_OFF32_LIBS: C2RustUnnamed = 1102;
pub const _CS_XBS5_ILP32_OFF32_LDFLAGS: C2RustUnnamed = 1101;
pub const _CS_XBS5_ILP32_OFF32_CFLAGS: C2RustUnnamed = 1100;
pub const _CS_LFS64_LINTFLAGS: C2RustUnnamed = 1007;
pub const _CS_LFS64_LIBS: C2RustUnnamed = 1006;
pub const _CS_LFS64_LDFLAGS: C2RustUnnamed = 1005;
pub const _CS_LFS64_CFLAGS: C2RustUnnamed = 1004;
pub const _CS_LFS_LINTFLAGS: C2RustUnnamed = 1003;
pub const _CS_LFS_LIBS: C2RustUnnamed = 1002;
pub const _CS_LFS_LDFLAGS: C2RustUnnamed = 1001;
pub const _CS_LFS_CFLAGS: C2RustUnnamed = 1000;
pub const _CS_V7_WIDTH_RESTRICTED_ENVS: C2RustUnnamed = 5;
pub const _CS_V5_WIDTH_RESTRICTED_ENVS: C2RustUnnamed = 4;
pub const _CS_GNU_LIBPTHREAD_VERSION: C2RustUnnamed = 3;
pub const _CS_GNU_LIBC_VERSION: C2RustUnnamed = 2;
pub const _CS_V6_WIDTH_RESTRICTED_ENVS: C2RustUnnamed = 1;
pub const _CS_PATH: C2RustUnnamed = 0;
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
pub type HashTable = crate::hash::HashTable;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
use crate::floc::Floc;

pub const o_invalid: variable_origin = 7;
pub const o_automatic: variable_origin = 6;
pub const o_override: variable_origin = 5;
pub const o_command: variable_origin = 4;
pub const o_env_override: variable_origin = 3;
pub const o_file: variable_origin = 2;
pub const o_env: variable_origin = 1;
pub const o_default: variable_origin = 0;
pub use crate::variable::variable;
pub type variable_export = ::core::ffi::c_uint;
pub const v_ifset: variable_export = 3;
pub const v_noexport: variable_export = 2;
pub const v_export: variable_export = 1;
pub const v_default: variable_export = 0;
pub type variable_origin = ::core::ffi::c_uint;
pub type variable_flavor = ::core::ffi::c_uint;
pub const f_append_value: variable_flavor = 6;
pub const f_shell: variable_flavor = 5;
pub const f_append: variable_flavor = 4;
pub const f_expand: variable_flavor = 3;
pub const f_recursive: variable_flavor = 2;
pub const f_simple: variable_flavor = 1;
pub const f_bogus: variable_flavor = 0;
pub use crate::output::output;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChildBase {
    pub cmd_name: *mut ::core::ffi::c_char,
    pub environment: *mut *mut ::core::ffi::c_char,
    pub output: output,
}
#[repr(C)]
pub struct Child {
    // The first three fields mirror `ChildBase` (same order, `#[repr(C)]`) so
    // that `child as *mut ChildBase` stays a valid prefix cast for
    // `child_execute_job`/`free_childbase`.
    pub cmd_name: *mut ::core::ffi::c_char,
    pub environment: *mut *mut ::core::ffi::c_char,
    pub output: output,
    pub next: *mut Child,
    /// The target this child builds, by arena handle (the former `*mut File`).
    pub file: FileId,
    /// Which inline entry of a (possibly double-colon) target this child runs:
    /// `0` = the head node itself, `i >= 1` = `head.double_colon[i-1]`. For a
    /// single-colon target this is always `0`.
    pub entry: usize,
    pub sh_batch_file: *mut ::core::ffi::c_char,
    /// Owned, fully-expanded recipe lines (each NUL-free), the former
    /// `command_lines: *mut *mut c_char` + intrusive `ncommand_lines`.
    pub command_lines: Vec<Vec<u8>>,
    /// Per-line flags captured at chop time, parallel to `command_lines`.
    pub line_flags: Vec<RecipeLineFlags>,
    /// Cursor into `command_lines`: index of the next line to run.
    pub command_line: usize,
    /// Owned, NUL-terminated working copy of the line currently being
    /// consumed; `command_ptr` walks within it (the former in-place rewrite of
    /// the heap `command_lines[i]` buffer). Kept alive for the whole time
    /// `command_ptr` is non-null.
    pub command_buf: Vec<u8>,
    /// Cursor within `command_buf`; null when no line is loaded (the former
    /// `command_ptr`). Always points into `command_buf`, so it stays valid as
    /// long as `command_buf` is not reallocated.
    pub command_ptr: *mut ::core::ffi::c_char,
    pub pid: pid_t,
    pub(crate) remote: ::core::ffi::c_uint,
    pub(crate) noerror: ::core::ffi::c_uint,
    pub(crate) good_stdin: ::core::ffi::c_uint,
    pub(crate) deleted: ::core::ffi::c_uint,
    pub(crate) recursive: ::core::ffi::c_uint,
    pub(crate) jobslot: ::core::ffi::c_uint,
    pub(crate) dontcare: ::core::ffi::c_uint,
}

impl Child {
    pub fn remote(&self) -> ::core::ffi::c_uint {
        self.remote
    }
    pub fn set_remote(&mut self, val: ::core::ffi::c_uint) {
        self.remote = val;
    }
    pub fn noerror(&self) -> ::core::ffi::c_uint {
        self.noerror
    }
    pub fn set_noerror(&mut self, val: ::core::ffi::c_uint) {
        self.noerror = val;
    }
    pub fn good_stdin(&self) -> ::core::ffi::c_uint {
        self.good_stdin
    }
    pub fn set_good_stdin(&mut self, val: ::core::ffi::c_uint) {
        self.good_stdin = val;
    }
    pub fn deleted(&self) -> ::core::ffi::c_uint {
        self.deleted
    }
    pub fn set_deleted(&mut self, val: ::core::ffi::c_uint) {
        self.deleted = val;
    }
    pub fn recursive(&self) -> ::core::ffi::c_uint {
        self.recursive
    }
    pub fn set_recursive(&mut self, val: ::core::ffi::c_uint) {
        self.recursive = val;
    }
    pub fn jobslot(&self) -> ::core::ffi::c_uint {
        self.jobslot
    }
    pub fn set_jobslot(&mut self, val: ::core::ffi::c_uint) {
        self.jobslot = val;
    }
    pub fn dontcare(&self) -> ::core::ffi::c_uint {
        self.dontcare
    }
    pub fn set_dontcare(&mut self, val: ::core::ffi::c_uint) {
        self.dontcare = val;
    }
}
impl crate::file::NextLinked for Child {
    unsafe fn next(this: *const Self) -> *mut Self {
        if this.is_null() {
            return ::core::ptr::null_mut::<Self>();
        }
        (*this).next
    }
}
use crate::{
    commands::{chop_commands, delete_child_targets, handling_fatal_signal},
    entry::{db_level, die_cleanup, not_parallel, one_shell, posix_pedantic, stopchar_map},
    file::lookup_file,
    findprog::find_in_given_path,
    function::{shell_completed, shell_function_pid},
    output::{
        error, fatal_err, message, perror_with_name, pfatal_with_name_err, set_output_context,
        FmtArg,
    },
    posixos::{
        fd_noinherit, get_bad_stdin, jobserver_acquire, jobserver_enabled, jobserver_post_child,
        jobserver_pre_acquire, jobserver_pre_child, jobserver_release, jobserver_signal,
    },
    remake::{notice_finished_file, show_goal_error},
    variable::target_environment,
    warning::{self, Action, Type},
};
pub const __S_IFMT: i32 = 0o170000_i32;
pub const __S_IEXEC: i32 = 0o100_i32;
pub const SIG_BLOCK: i32 = 0;
pub const SIG_UNBLOCK: i32 = 1;
pub const SIG_SETMASK: i32 = 2;
pub const ENOENT: i32 = 2;
pub const EINTR: i32 = 4;
pub const ENOEXEC: i32 = 8;
pub const EACCES: i32 = 13;
pub const WNOHANG: i32 = 1;
pub const __WCOREFLAG: i32 = 0x80_i32;
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const INTSTR_LENGTH: usize = 53_usize
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22_usize)
    .wrapping_add(3_usize);
pub const OUTPUT_SYNC_LINE: i32 = 1;
pub const OUTPUT_SYNC_RECURSE: i32 = 3;
pub const MAKE_SUCCESS: i32 = 0;
pub const MAKE_TROUBLE: i32 = 1;
pub const MAKE_FAILURE: i32 = 2;
/// The only writers in the C original are W32/DOS-specific (this is a POSIX
/// port), so the value is fixed. `const` rather than `static`: a raw pointer
/// isn't `Sync`, so a `static` would need an `unsafe`-to-read wrapper for a
/// value nothing ever mutates — `const` just inlines the pointer at each use.
pub const default_shell: *const ::core::ffi::c_char =
    b"/bin/sh\0" as *const u8 as *const ::core::ffi::c_char;
pub const S_IXUSR: i32 = __S_IEXEC;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const COMMANDS_RECURSE: i32 = 1;
pub const COMMANDS_SILENT: i32 = 2;
pub const NONEXISTENT_MTIME: i32 = 1;
/// glibc `%p` bytes of a pointer, for `-d` trace lines.
fn ptr_bytes<T>(p: *const T) -> Vec<u8> {
    format!("{p:p}").into_bytes()
}

/// The `%lu` bytes `pid2str` formats, without the C buffer.
fn pid_bytes(pid: pid_t) -> Vec<u8> {
    (pid as ::core::ffi::c_ulong).to_string().into_bytes()
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn pid2str(ctx: &crate::execctx::ExecContext, pid: pid_t) -> *const ::core::ffi::c_char {
    let buf = ctx.pid_string.0.as_ptr() as *mut ::core::ffi::c_char;
    sprintf(
        buf,
        b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
        pid as ::core::ffi::c_ulong,
    );
    buf
}
// The live-children chain head (former `static mut children`) and the
// load-limited postponed-jobs chain head (former `static mut waiting_jobs`)
// now live on the owned per-run context: `ctx.children` / `ctx.waiting_jobs`
// (see `crate::execctx::ChildChain`). The fatal-signal handler reaches them
// through the `CTX_PTR` borrow channel.
/// Number of job slots currently in use. See
/// [`crate::execctx::ExecContext::job_slots_used`].
pub fn job_slots_used(ctx: &crate::execctx::ExecContext) -> u32 {
    ctx.job_slots_used.0.load(Ordering::Relaxed)
}
/// Number of jobs started since the load average was last sampled; used by
/// `load_too_high` to estimate the incremental load each new job adds. Atomic
/// so its reads/writes are plain safe ops; job bookkeeping is single-threaded,
/// so `Relaxed` preserves the original program order.
/// Number of jobserver tokens currently held. See
/// [`crate::execctx::ExecContext::jobserver_tokens`].
pub fn jobserver_tokens(ctx: &crate::execctx::ExecContext) -> u32 {
    ctx.jobserver_tokens.0.load(Ordering::Relaxed)
}
/// Safe port of make's `is_bourne_compatible_shell`: is the program named by
/// `path` one of the known Bourne-compatible shells, compared by its file stem
/// (the basename with any extension stripped)? On this target `ISDIRSEP` is just
/// `/` (only `/` carries `MAP_DIRSEP` in `stopchar_map`), which matches
/// `Path`'s component splitting.
pub fn is_bourne_compatible_shell(path: &::std::path::Path) -> bool {
    // List of known POSIX (or POSIX-ish) shells.
    const UNIX_SHELLS: [&str; 7] = ["sh", "bash", "dash", "ksh", "rksh", "zsh", "ash"];
    match path.file_stem().and_then(|s| s.to_str()) {
        Some(stem) => UNIX_SHELLS.contains(&stem),
        None => false,
    }
}
/// Borrow a NUL-terminated C string as a filesystem [`Path`](std::path::Path).
///
/// # Safety
///
/// `ptr` must point to a valid NUL-terminated C string that stays alive for
/// the lifetime `'a`.
unsafe fn path_from_cstr<'a>(ptr: *const ::core::ffi::c_char) -> &'a ::std::path::Path {
    #[cfg(unix)]
    use std::os::unix::ffi::OsStrExt;
    #[cfg(target_os = "wasi")]
    use std::os::wasi::ffi::OsStrExt;
    ::std::path::Path::new(::std::ffi::OsStr::from_bytes(
        ::core::ffi::CStr::from_ptr(ptr).to_bytes(),
    ))
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn block_sigs(ctx: &crate::execctx::ExecContext) {
    let set = ctx.fatal_signal_set.0.get();
    sigprocmask(
        SIG_BLOCK,
        &raw const set,
        ::core::ptr::null_mut::<sigset_t>(),
    );
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn unblock_sigs(ctx: &crate::execctx::ExecContext) {
    let set = ctx.fatal_signal_set.0.get();
    sigprocmask(
        SIG_UNBLOCK,
        &raw const set,
        ::core::ptr::null_mut::<sigset_t>(),
    );
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn unblock_all_sigs() {
    let mut empty: sigset_t = crate::entry::SigsetT { __val: [0; 16] };
    sigemptyset(&raw mut empty);
    sigprocmask(
        SIG_SETMASK,
        &raw mut empty,
        ::core::ptr::null_mut::<sigset_t>(),
    );
}
/// Build the `<file>:<line>` location label for a failed child's error message,
/// or the static `<builtin>` when the recipe has no source location. Any heap
/// buffer is pushed onto `allocations` so it outlives the `error()` call that
/// reads it. Split out of `child_error`; the only unsafety (formatting through
/// the recipe's `filenm` C string) is confined to the block below.
fn child_error_label(floc: &Floc, allocations: &mut Vec<Vec<u8>>) -> *const ::core::ffi::c_char {
    if floc.filenm.is_null() {
        return b"<builtin>\0" as *const u8 as *const ::core::ffi::c_char;
    }
    // SAFETY: `filenm` is a non-null, NUL-terminated C string taken from the
    // recipe's `fileinfo`; the buffer is sized to hold "<filenm>:<lineno>" and
    // is owned by `allocations`, so the returned pointer stays valid.
    unsafe {
        allocations.push(::std::vec::from_elem(
            0,
            strlen(floc.filenm)
                .wrapping_add(6)
                .wrapping_add(INTSTR_LENGTH)
                .wrapping_add(1) as usize,
        ));
        let a: *mut ::core::ffi::c_char =
            allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        sprintf(
            a,
            b"%s:%lu\0" as *const u8 as *const ::core::ffi::c_char,
            floc.filenm,
            floc.lineno.wrapping_add(floc.offset),
        );
        a
    }
}

/// The trailing shuffle-mode field for a child error: `smode` when shuffle mode
/// is active, otherwise the empty string (`smode` absent).
fn smode_or_empty(smode: Option<&::core::ffi::CStr>) -> &::core::ffi::CStr {
    smode.unwrap_or(c"")
}

/// Snapshot a target's NUL-terminated name from the arena into an owned buffer.
/// Returns an empty `"\0"` when the node is absent. The lock is dropped before
/// returning.
fn file_name_cstr(ctx: &crate::execctx::ExecContext, file: FileId) -> Vec<u8> {
    match ctx.filenodes.get(file) {
        Some(node) => {
            let g = node.lock().expect("file node poisoned");
            let mut nm = g.name.clone();
            nm.push(0);
            nm
        }
        None => vec![0],
    }
}

/// Snapshot a target's recipe definition location (`defined_in`/
/// `defined_lineno`) into an owned [`Floc`] plus the backing filename buffer.
/// The returned `Floc.filenm` points into `buf` (which the caller must keep
/// alive); a recipe with no source file yields a null `filenm`. The arena lock
/// is dropped before returning.
fn recipe_floc(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    offset: u64,
    buf: &mut Vec<u8>,
) -> Floc {
    let (defined_in, lineno) = match ctx.filenodes.get(file) {
        Some(node) => {
            let g = node.lock().expect("file node poisoned");
            match g.recipe.as_ref() {
                Some(r) => (r.defined_in.clone(), r.defined_lineno),
                None => (None, 0),
            }
        }
        None => (None, 0),
    };
    match defined_in {
        Some(mut name) => {
            name.push(0);
            *buf = name;
            Floc {
                filenm: buf.as_ptr() as *const ::core::ffi::c_char,
                lineno,
                // The error reports `lineno + offset`; the offset is the failing
                // command's 0-based index in the recipe (C: `fileinfo.offset =
                // child->command_line - 1`).
                offset,
            }
        }
        None => Floc {
            filenm: ::core::ptr::null(),
            lineno: 0,
            offset: 0,
        },
    }
}

/// Read a target's `command_state` through the arena, dropping the lock before
/// returning (per the job-state locking discipline: never hold a `FileNode`
/// guard across a job spawn or `notice_finished_file`).
fn file_command_state(ctx: &crate::execctx::ExecContext, file: FileId) -> CommandState {
    file_command_state_entry(ctx, file, 0)
}

/// Entry-aware form: `entry` 0 = the head, `i>=1` = `double_colon[i-1]`.
fn file_command_state_entry(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    entry: usize,
) -> CommandState {
    match ctx.filenodes.get(file) {
        Some(node) => {
            let mut guard = node.lock().expect("file node poisoned");
            entry_node(&mut guard, entry).command_state
        }
        None => CommandState::Finished,
    }
}

/// Read a target's `update_status` through the arena, dropping the lock first.
fn file_update_status_entry(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    entry: usize,
) -> UpdateStatus {
    match ctx.filenodes.get(file) {
        Some(node) => {
            let mut guard = node.lock().expect("file node poisoned");
            entry_node(&mut guard, entry).update_status
        }
        None => UpdateStatus::Success,
    }
}

/// Set a target's `update_status` through the arena, dropping the lock first.
fn set_file_update_status_entry(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    entry: usize,
    status: UpdateStatus,
) {
    if let Some(node) = ctx.filenodes.get(file) {
        let mut guard = node.lock().expect("file node poisoned");
        entry_node(&mut guard, entry).update_status = status;
    }
}

/// Set a target's `command_state` through the arena, dropping the lock first.
fn set_file_command_state_entry(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    entry: usize,
    state: CommandState,
) {
    // Port of C's `set_command_state`: set the state on this entry and, for the
    // head entry, propagate it to every `also_make` (grouped `&:`) peer. The
    // peers are snapshotted under the head guard, the guard is dropped, then
    // each peer is assigned on its own brief lock. Without this, starting the
    // recipe for one grouped target leaves its siblings at `cs_not_started`, so
    // a sibling considered before the running job is reaped starts the shared
    // recipe a second time.
    let peers: Vec<FileId> = {
        let Some(node) = ctx.filenodes.get(file) else {
            return;
        };
        let mut guard = node.lock().expect("file node poisoned");
        entry_node(&mut guard, entry).command_state = state;
        if entry == 0 {
            guard.also_make.iter().filter_map(|d| d.file).collect()
        } else {
            Vec::new()
        }
    };
    for pid in peers {
        if let Some(node) = ctx.filenodes.get(pid) {
            node.lock().expect("file node poisoned").command_state = state;
        }
    }
}

/// Resolve a (possibly double-colon) entry within a locked head node: `0` is
/// the head itself, `i>=1` is `double_colon[i-1]`.
fn entry_node(guard: &mut FileNode, entry: usize) -> &mut FileNode {
    if entry == 0 {
        guard
    } else {
        &mut guard.double_colon[entry - 1]
    }
}

unsafe fn child_error(
    ctx: &crate::execctx::ExecContext,
    child: *mut Child,
    exit_code: i32,
    exit_sig: i32,
    coredump: i32,
    ignored: i32,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut pre: *const ::core::ffi::c_char = b"*** \0" as *const u8 as *const ::core::ffi::c_char;
    let mut post: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let mut dump: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let mut floc_buf: Vec<u8> = Vec::new();
    let cmd_offset = ((*child).command_line as u64).saturating_sub(1);
    let floc = recipe_floc(ctx, (*child).file, cmd_offset, &mut floc_buf);
    let name_buf = file_name_cstr(ctx, (*child).file);
    let f_name: *const ::core::ffi::c_char = name_buf.as_ptr() as *const ::core::ffi::c_char;
    let mut smode: Option<&::core::ffi::CStr> = None;
    let l: size_t;
    if ignored != 0 && crate::entry::opt_run_silent(ctx) {
        return;
    }
    if exit_sig != 0 && coredump != 0 {
        dump = b" (core dumped)\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if ignored != 0 {
        pre = b"\0" as *const u8 as *const ::core::ffi::c_char;
        post = b" (ignored)\0" as *const u8 as *const ::core::ffi::c_char;
    }
    let nm = child_error_label(&floc, &mut alloca_allocations);
    l = strlen(pre)
        .wrapping_add(strlen(nm))
        .wrapping_add(strlen(f_name))
        .wrapping_add(strlen(post)) as size_t;
    if let Some(label) = crate::shuffle::get_mode(ctx) {
        let mut buf = format!(" shuffle={}", label).into_bytes();
        buf.push(0);
        alloca_allocations.push(buf);
        smode = Some(
            ::core::ffi::CStr::from_bytes_with_nul(alloca_allocations.last().unwrap())
                .expect("shuffle label is NUL-terminated with no interior NUL"),
        );
    }
    set_output_context(if (*child).output.syncout() as i32 != 0 {
        &raw mut (*child).output
    } else {
        ::core::ptr::null_mut::<output>()
    });
    show_goal_error(ctx);
    if exit_sig == 0 {
        error(
            ctx,
            NILF,
            l.wrapping_add(INTSTR_LENGTH),
            b"%s[%s: %s] Error %d%s%s\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                FmtArg::Str((pre) as *const ::core::ffi::c_char),
                FmtArg::Str((nm) as *const ::core::ffi::c_char),
                FmtArg::Str((f_name) as *const ::core::ffi::c_char),
                FmtArg::Int((exit_code) as i64),
                FmtArg::Str((post) as *const ::core::ffi::c_char),
                FmtArg::Str(smode_or_empty(smode).as_ptr()),
            ],
        );
    } else {
        let s: *const ::core::ffi::c_char = strsignal(exit_sig);
        error(
            ctx,
            NILF,
            l.wrapping_add(strlen(s) as size_t)
                .wrapping_add(strlen(dump) as size_t),
            b"%s[%s: %s] %s%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                FmtArg::Str((pre) as *const ::core::ffi::c_char),
                FmtArg::Str((nm) as *const ::core::ffi::c_char),
                FmtArg::Str((f_name) as *const ::core::ffi::c_char),
                FmtArg::Str((s) as *const ::core::ffi::c_char),
                FmtArg::Str((dump) as *const ::core::ffi::c_char),
                FmtArg::Str((post) as *const ::core::ffi::c_char),
                FmtArg::Str(smode_or_empty(smode).as_ptr()),
            ],
        );
    }
    set_output_context(::core::ptr::null_mut::<output>());
}
/// Number of children reaped by the `SIGCHLD` handler and not yet processed
/// by the reap loop. See [`crate::execctx::ExecContext::dead_children`].
fn dead_children(ctx: &crate::execctx::ExecContext) -> u32 {
    ctx.dead_children.0.load(Ordering::Relaxed)
}
/// `SIGCHLD` handler: record a reaped child and wake any blocked jobserver
/// acquire. Async-signal-safe (an atomic increment plus `jobserver_signal`'s
/// `close`). Reaches `ExecContext` through the `CTX_PTR` borrow channel since
/// a real signal handler cannot carry an extra parameter.
pub extern "C" fn child_handler(mut _sig: i32) {
    crate::entry::try_with_exec_context(|ctx| {
        ctx.dead_children.0.fetch_add(1, Ordering::Relaxed);
    });
    jobserver_signal();
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn reap_children(
    ctx: &crate::execctx::ExecContext,
    mut block: i32,
    err: i32,
) -> Result<(), crate::build_result::BuildError> {
    let mut status: i32 = 0;
    let mut reap_more: i32 = 1;
    while (!ctx.children.0.get().is_null() || shell_function_pid(ctx) != 0)
        && (block != 0 || reap_more != 0)
    {
        let mut remote: ::core::ffi::c_uint = 0;
        let mut pid: pid_t;
        let mut exit_code: i32 = 0;
        let mut exit_sig: i32 = 0;
        let mut coredump: i32 = 0;
        let mut lastc: *mut Child;
        let mut c: *mut Child;
        let mut child_failed: i32;
        let mut any_remote: i32;
        let mut any_local: i32;
        let dontcare: i32;
        if err != 0 && block != 0 {
            fflush(stdout);
            if !ctx.reap_children_printed.0.load(Ordering::Relaxed) {
                error(
                    ctx,
                    ::core::ptr::null_mut::<Floc>(),
                    0,
                    b"*** Waiting for unfinished jobs....\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            ctx.reap_children_printed.0.store(true, Ordering::Relaxed);
        }
        if dead_children(ctx) > 0 {
            ctx.dead_children.0.fetch_sub(1, Ordering::Relaxed);
        }
        any_remote = 0;
        any_local = (shell_function_pid(ctx) != 0) as i32;
        lastc = ::core::ptr::null_mut::<Child>();
        c = ctx.children.0.get();
        // Set when we find a child that already failed to launch (pid < 0);
        // otherwise we walk to the end of the list and must wait() for one.
        let mut found_bad: i32 = 0;
        loop {
            if c.is_null() {
                break;
            }
            any_remote |= (*c).remote() as i32;
            any_local |= ((*c).remote() == 0) as i32;
            if (*c).pid < 0 {
                exit_sig = 0;
                coredump = 0;
                exit_code = 127;
                found_bad = 1;
                break;
            } else {
                if 0x4_i32 & db_level(ctx) != 0 {
                    let fname = file_name_cstr(ctx, (*c).file);
                    crate::output::trace_parts(&[
                        b"Live child ",
                        &ptr_bytes(c),
                        b" (",
                        &fname[..fname.len() - 1],
                        b") PID ",
                        &pid_bytes((*c).pid),
                        b" ",
                        if (*c).remote() as i32 != 0 {
                            b" (remote)"
                        } else {
                            b""
                        },
                        b"\n",
                    ]);
                }
                lastc = c;
                c = (*c).next;
            }
        }
        if found_bad == 0 {
            if any_remote != 0 {
                pid = ctx.remote_backend.0.status(
                    &raw mut exit_code,
                    &raw mut exit_sig,
                    &raw mut coredump,
                    false,
                ) as pid_t;
            } else {
                pid = 0_i32 as pid_t;
            }
            if pid > 0 {
                remote = 1;
            } else if pid < 0 {
                return Err(pfatal_with_name_err(
                    ctx,
                    b"remote_status\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            } else {
                if any_local != 0 {
                    if block == 0 {
                        pid = waitpid(-(1 as __pid_t), &raw mut status, WNOHANG) as pid_t;
                    } else {
                        loop {
                            pid = wait(&raw mut status) as pid_t;
                            if !(pid == -1_i32 && *__errno_location() == EINTR) {
                                break;
                            }
                        }
                    }
                } else {
                    pid = 0_i32 as pid_t;
                }
                if pid < 0 {
                    return Err(pfatal_with_name_err(
                        ctx,
                        b"wait\0" as *const u8 as *const ::core::ffi::c_char,
                    ));
                } else if pid > 0 {
                    exit_code = (status & 0xff00_i32) >> 8;
                    exit_sig = if ((status & 0x7f_i32) + 1) as ::core::ffi::c_schar as i32 >> 1 > 0
                    {
                        status & 0x7f_i32
                    } else {
                        0
                    };
                    coredump = status & __WCOREFLAG;
                } else {
                    reap_more = 0;
                    if block == 0 || any_remote == 0 {
                        break;
                    }
                    pid = ctx.remote_backend.0.status(
                        &raw mut exit_code,
                        &raw mut exit_sig,
                        &raw mut coredump,
                        false,
                    ) as pid_t;
                    if pid < 0 {
                        return Err(pfatal_with_name_err(
                            ctx,
                            b"remote_status\0" as *const u8 as *const ::core::ffi::c_char,
                        ));
                    }
                    if pid == 0 {
                        break;
                    }
                    remote = 1;
                }
            }
            crate::entry::bump_command_count(ctx);
            if remote == 0 && pid == shell_function_pid(ctx) {
                shell_completed(ctx, exit_code, exit_sig)?;
                break;
            } else {
                lastc = ::core::ptr::null_mut::<Child>();
                c = ctx.children.0.get();
                while !c.is_null() {
                    if (*c).pid == pid && (*c).remote() == remote {
                        break;
                    }
                    lastc = c;
                    c = (*c).next;
                }
                if c.is_null() {
                    continue;
                }
                if 0x4_i32 & db_level(ctx) != 0 {
                    crate::output::trace_parts(&[
                        if exit_sig == 0 && exit_code == 0 {
                            b"Reaping winning child ".as_slice()
                        } else {
                            b"Reaping losing child ".as_slice()
                        },
                        &ptr_bytes(c),
                        b" PID ",
                        &pid_bytes((*c).pid),
                        b" ",
                        if (*c).remote() as i32 != 0 {
                            b" (remote)"
                        } else {
                            b""
                        },
                        b"\n",
                    ]);
                }
                if ctx.job_counter.0.load(Ordering::Relaxed) != 0 {
                    ctx.job_counter.0.fetch_sub(1, Ordering::Relaxed);
                }
            }
        }
        if exit_sig == 0 && exit_code == 127 && !(*c).cmd_name.is_null() {
            let mut e: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut st: stat = stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            };
            let mut r: i32;
            loop {
                r = stat((*c).cmd_name, &raw mut st);
                if !(r == -1_i32 && *__errno_location() == EINTR) {
                    break;
                }
            }
            if r < 0 {
                e = strerror(*__errno_location());
            } else if st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
                || st.st_mode & S_IXUSR as __mode_t == 0
            {
                e = strerror(EACCES);
            } else if st.st_size == 0 {
                e = strerror(ENOEXEC);
            }
            if !e.is_null() {
                error(
                    ctx,
                    ::core::ptr::null_mut::<Floc>(),
                    (strlen((*c).cmd_name) as size_t).wrapping_add(strlen(e) as size_t),
                    b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        FmtArg::Str(((*c).cmd_name) as *const ::core::ffi::c_char),
                        FmtArg::Str((e) as *const ::core::ffi::c_char),
                    ],
                );
            }
        }
        if exit_sig == 0 && exit_code == 0 {
            child_failed = MAKE_SUCCESS;
        } else if exit_sig == 0
            && exit_code == 1
            && crate::entry::opt_question(ctx)
            && (*c).recursive() as i32 != 0
        {
            child_failed = MAKE_TROUBLE;
        } else {
            child_failed = MAKE_FAILURE;
        }
        if !(*c).sh_batch_file.is_null() {
            let rm_status: i32;
            if 0x4_i32 & db_level(ctx) != 0 {
                crate::output::trace_parts(&[
                    b"Cleaning up temp batch file ",
                    ::core::ffi::CStr::from_ptr((*c).sh_batch_file).to_bytes(),
                    b"\n",
                ]);
            }
            *__errno_location() = 0;
            rm_status = remove((*c).sh_batch_file);
            if rm_status != 0 && 0x4_i32 & db_level(ctx) != 0 {
                crate::output::trace_parts(&[
                    b"Cleaning up temp batch file ",
                    ::core::ffi::CStr::from_ptr((*c).sh_batch_file).to_bytes(),
                    b" failed (",
                    (*__errno_location()).to_string().as_bytes(),
                    b")\n",
                ]);
            }
            free((*c).sh_batch_file as *mut ::core::ffi::c_void);
            (*c).sh_batch_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if (*c).good_stdin() != 0 {
            ctx.good_stdin_used.set(false);
        }
        dontcare = (*c).dontcare() as i32;
        if child_failed != 0 && (*c).noerror() == 0 && !crate::entry::opt_ignore_errors(ctx) {
            if dontcare == 0 && child_failed == MAKE_FAILURE {
                child_error(ctx, c, exit_code, exit_sig, coredump, 0);
            }
            set_file_update_status_entry(
                ctx,
                (*c).file,
                (*c).entry,
                if child_failed == MAKE_FAILURE {
                    us_failed
                } else {
                    us_question
                },
            );
            if ctx.delete_on_error.0.load(Ordering::Relaxed) == -1_i32 {
                let is_target = match lookup_file(ctx, b".DELETE_ON_ERROR") {
                    Some(fid) => match ctx.filenodes.get(fid) {
                        Some(node) => node.lock().expect("file node poisoned").is_target,
                        None => false,
                    },
                    None => false,
                };
                ctx.delete_on_error
                    .0
                    .store(is_target as i32, Ordering::Relaxed);
            }
            if exit_sig != 0 || ctx.delete_on_error.0.load(Ordering::Relaxed) != 0 {
                delete_child_targets(ctx, c)?;
            }
        } else {
            if child_failed != 0 {
                child_error(ctx, c, exit_code, exit_sig, coredump, 1);
                child_failed = 0;
            }
            if job_next_command(c) != 0 {
                if handling_fatal_signal(ctx) {
                    set_file_update_status_entry(ctx, (*c).file, (*c).entry, us_failed);
                } else {
                    if crate::entry::opt_output_sync(ctx) == OUTPUT_SYNC_LINE {
                        crate::output::output_dump(ctx, &raw mut (*c).output);
                    }
                    (*c).set_remote(
                        ctx.remote_backend.0.can_start_job(false) as ::core::ffi::c_uint
                    );
                    // The signal mask this loop blocked has to come back before
                    // a rejection leaves the reaper.
                    let started = start_job_command(ctx, c);
                    unblock_sigs(ctx);
                    started?;
                    if file_command_state_entry(ctx, (*c).file, (*c).entry) as i32
                        == cs_running as i32
                    {
                        continue;
                    }
                }
                if file_update_status_entry(ctx, (*c).file, (*c).entry) as i32 != us_success as i32
                {
                    delete_child_targets(ctx, c)?;
                }
            } else {
                set_file_update_status_entry(ctx, (*c).file, (*c).entry, us_success);
            }
        }
        crate::output::output_dump(ctx, &raw mut (*c).output);
        if !handling_fatal_signal(ctx) {
            notice_finished_file(ctx, (*c).file, (*c).entry)?;
        }
        block_sigs(ctx);
        if (*c).pid > 0 && 0x4_i32 & db_level(ctx) != 0 {
            crate::output::trace_parts(&[
                b"Removing child ",
                &ptr_bytes(c),
                b" PID ",
                &pid_bytes((*c).pid),
                if (*c).remote() as i32 != 0 {
                    b" (remote)"
                } else {
                    b""
                },
                b" from chain.\n",
            ]);
        }
        if job_slots_used(ctx) > 0 {
            ctx.job_slots_used.0.store(
                job_slots_used(ctx).wrapping_sub((*c).jobslot()),
                Ordering::Relaxed,
            );
        }
        if let Some(lastcr) = lastc.as_mut() {
            lastcr.next = (*c).next;
        } else {
            ctx.children.0.set((*c).next);
        }
        // Signals are blocked across the child-list splice, so unblock them
        // before handing a teardown failure back — the caller must not keep
        // running with the fatal-signal set still masked (#441).
        let freed = free_child(ctx, c);
        unblock_sigs(ctx);
        freed?;
        if err == 0
            && child_failed != 0
            && dontcare == 0
            && !crate::entry::opt_keep_going(ctx)
            && !handling_fatal_signal(ctx)
        {
            // `child_failed` is one of make's canonical statuses (set to
            // MAKE_SUCCESS/MAKE_TROUBLE/MAKE_FAILURE above) and the guard
            // rules out MAKE_SUCCESS, so this always takes the error arm. The
            // end-of-run cleanup still runs here rather than in the caller so
            // its output ordering is unchanged; only the exit itself moved out,
            // and the failure is now handed back as a value (#432 Phase B,
            // #441).
            if let Err(e) = crate::build_result::result_from_status(child_failed) {
                die_cleanup(ctx, child_failed);
                return Err(e);
            }
        }
        block = 0;
    }
    Ok(())
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn free_childbase(child: *mut ChildBase) {
    if !(*child).environment.is_null() {
        let mut ep: *mut *mut ::core::ffi::c_char = (*child).environment;
        while !(*ep).is_null() {
            let fresh9 = ep;
            ep = ep.offset(1_i32 as isize);
            free(*fresh9 as *mut ::core::ffi::c_void);
        }
        free((*child).environment as *mut ::core::ffi::c_void);
    }
    free((*child).cmd_name as *mut ::core::ffi::c_void);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn free_child(
    ctx: &crate::execctx::ExecContext,
    child: *mut Child,
) -> Result<(), crate::build_result::BuildError> {
    crate::output::output_close(ctx, &raw mut (*child).output);
    release_jobserver_token(ctx, child)?;
    if handling_fatal_signal(ctx) {
        return Ok(());
    }
    // Free the c2rust-allocated `ChildBase` members (cmd_name/environment) the
    // same way as before; the owned `command_lines`/`line_flags`/`command_buf`
    // Vecs are released when the `Box` is dropped below.
    free_childbase(child as *mut ChildBase);
    // The child was allocated with `Box::into_raw`; reclaim it so its owned
    // fields drop. Takes the place of the former `free(child)`.
    drop(Box::from_raw(child));
    Ok(())
}

/// Account for this child's jobserver token as it is freed: assert a token is
/// still outstanding, hand one back to the jobserver when more than one is held
/// (tracing the release under `-d`), and decrement the local token count. Split
/// out of `free_child` so that function stays a flat teardown sequence — these
/// jobserver paths only fire under specific token/debug conditions and are
/// exercised by the parallel-build integration tests rather than unit tests.
///
/// # Safety
///
/// `child` must be a valid `child` whose `file` is live; the jobserver globals
/// must be initialized.
unsafe fn release_jobserver_token(
    ctx: &crate::execctx::ExecContext,
    child: *mut Child,
) -> Result<(), crate::build_result::BuildError> {
    let name_buf = file_name_cstr(ctx, (*child).file);
    let name = name_buf.as_ptr() as *const ::core::ffi::c_char;
    if jobserver_tokens(ctx) == 0 {
        return Err(fatal_err(
            ctx,
            ::core::ptr::null_mut::<Floc>(),
            INTSTR_LENGTH.wrapping_add(strlen(name) as size_t),
            b"INTERNAL: freeing child %p (%s) but no tokens left\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                FmtArg::Ptr((child) as *const ::core::ffi::c_void),
                FmtArg::Str((name) as *const ::core::ffi::c_char),
            ],
        ));
    }
    if jobserver_enabled(ctx) != 0 && jobserver_tokens(ctx) > 1 {
        jobserver_release(ctx, 1);
        if 0x4_i32 & db_level(ctx) != 0 {
            crate::output::trace_parts(&[
                b"Released token for child ",
                &ptr_bytes(child),
                b" (",
                ::core::ffi::CStr::from_ptr(name).to_bytes(),
                b").\n",
            ]);
        }
    }
    ctx.jobserver_tokens.0.fetch_sub(1, Ordering::Relaxed);
    Ok(())
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn start_job_command(
    ctx: &crate::execctx::ExecContext,
    child: *mut Child,
) -> Result<(), crate::build_result::BuildError> {
    let mut flags: i32;
    let mut p: *mut ::core::ffi::c_char;
    let mut argv: *mut *mut ::core::ffi::c_char;
    // Snapshot the file's command-line flags and recipe prefix once, dropping
    // the arena lock before any job spawn or `notice_finished_file`.
    let (command_flags, prefix): (i32, ::core::ffi::c_char) = match ctx.filenodes.get((*child).file)
    {
        Some(node) => {
            let g = node.lock().expect("file node poisoned");
            let pfx = g.recipe.as_ref().map(|r| r.recipe_prefix).unwrap_or(b'\t');
            (g.command_flags, pfx as ::core::ffi::c_char)
        }
        None => (0, b'\t' as ::core::ffi::c_char),
    };
    // Index of the line currently loaded (`command_line` was advanced past it
    // by `job_next_command`).
    let line_idx = (*child).command_line.wrapping_sub(1);
    if !(*child).command_ptr.is_null() {
        let line_flags_ref: &[RecipeLineFlags] = &(*child).line_flags;
        let line_flag_bits = line_flags_ref
            .get(line_idx)
            .copied()
            .unwrap_or(RecipeLineFlags::empty())
            .bits() as i32;
        flags = command_flags | line_flag_bits;
        p = (*child).command_ptr;
        (*child).set_noerror((flags & 4 != 0) as i32 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        while *p as i32 != 0 {
            if *p as i32 == '@' as i32 {
                flags |= COMMANDS_SILENT;
            } else if *p as i32 == '+' as i32 {
                flags |= COMMANDS_RECURSE;
            } else if *p as i32 == '-' as i32 {
                (*child).set_noerror(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else if !(*(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
                .offset(*p as ::core::ffi::c_uchar as isize) as i32
                & 0x2_i32
                != 0)
            {
                break;
            }
            p = p.offset(1_i32 as isize);
        }
        (*child)
            .set_recursive((flags & 1 != 0) as i32 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        // Persist any newly-discovered RECURSE bit into the child's own copy of
        // the line flags (the former write-back into `cmds.lines_flags`).
        let line_flags_mut: &mut Vec<RecipeLineFlags> = &mut (*child).line_flags;
        if let Some(lf) = line_flags_mut.get_mut(line_idx) {
            if flags & COMMANDS_RECURSE != 0 {
                *lf |= RecipeLineFlags::RECURSE;
            }
        }
        let mut p1: *mut ::core::ffi::c_char;
        let mut p2: *mut ::core::ffi::c_char;
        p2 = p;
        p1 = p2;
        while *p1 as i32 != 0 {
            let fresh11 = p2;
            p2 = p2.offset(1_i32 as isize);
            *fresh11 = *p1;
            if *p1.offset(0_i32 as isize) as i32 == '\n' as i32
                && *p1.offset(1_i32 as isize) as i32 == prefix as i32
            {
                p1 = p1.offset(1_i32 as isize);
            }
            p1 = p1.offset(1_i32 as isize);
        }
        *p2 = *p1;
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        // Re-read the (possibly RECURSE-updated) line flags for the argv build.
        let argv_flags_ref: &[RecipeLineFlags] = &(*child).line_flags;
        let argv_line_flags = argv_flags_ref
            .get(line_idx)
            .copied()
            .unwrap_or(RecipeLineFlags::empty())
            .bits() as i32;
        argv = construct_command_argv(
            ctx,
            p,
            &raw mut end,
            Some((*child).file),
            argv_line_flags | command_flags,
            &raw mut (*child).sh_batch_file,
        )?;
        if end.is_null() {
            (*child).command_ptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            let end_ref = end
                .as_mut()
                .expect("construct_command_argv returned an invalid end pointer");
            *end_ref = 0;
            (*child).command_ptr = end.add(1);
        }
        if !argv.is_null() && crate::entry::opt_question(ctx) && !(flags & 1 != 0) {
            if !argv.is_null() {
                free(*argv.offset(0_i32 as isize) as *mut ::core::ffi::c_void);
                free(argv as *mut ::core::ffi::c_void);
            }
            set_file_update_status_entry(ctx, (*child).file, (*child).entry, us_question);
            notice_finished_file(ctx, (*child).file, (*child).entry)?;
            return Ok(());
        }
        if crate::entry::opt_touch(ctx) && !(flags & 1 != 0) {
            if !argv.is_null() {
                free(*argv.offset(0_i32 as isize) as *mut ::core::ffi::c_void);
                free(argv as *mut ::core::ffi::c_void);
            }
            argv = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        }
        if !argv.is_null() {
            let os = crate::entry::opt_output_sync(ctx);
            (*child).output.set_syncout(
                (os != 0 && (os == OUTPUT_SYNC_RECURSE || !(flags & 1 != 0))) as i32
                    as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
            set_output_context(if (*child).output.syncout() as i32 != 0 {
                &raw mut (*child).output
            } else {
                ::core::ptr::null_mut::<output>()
            });
            if (*child).output.syncout() == 0 {
                crate::output::output_dump(ctx, &raw mut (*child).output);
            }
            if crate::entry::opt_just_print(ctx)
                || 0x10_i32 & db_level(ctx) != 0
                || !(flags & 2 != 0) && !crate::entry::opt_run_silent(ctx)
            {
                message(
                    ctx,
                    0,
                    strlen(p) as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[FmtArg::Str((p) as *const ::core::ffi::c_char)],
                );
            }
            ctx.commands_started
                .set(ctx.commands_started.get().wrapping_add(1));
            if !(*argv.offset(0_i32 as isize)).is_null()
                && is_bourne_compatible_shell(path_from_cstr(*argv.offset(0_i32 as isize)))
                && (!(*argv.offset(1_i32 as isize)).is_null()
                    && *(*argv.offset(1_i32 as isize)).offset(0_i32 as isize) as i32 == '-' as i32
                    && (*(*argv.offset(1_i32 as isize)).offset(1_i32 as isize) as i32
                        == 'c' as i32
                        && *(*argv.offset(1_i32 as isize)).offset(2_i32 as isize) as i32 == 0
                        || *(*argv.offset(1_i32 as isize)).offset(1_i32 as isize) as i32
                            == 'e' as i32
                            && *(*argv.offset(1_i32 as isize)).offset(2_i32 as isize) as i32
                                == 'c' as i32
                            && *(*argv.offset(1_i32 as isize)).offset(3_i32 as isize) as i32 == 0))
                && (!(*argv.offset(2_i32 as isize)).is_null()
                    && *(*argv.offset(2_i32 as isize)).offset(0_i32 as isize) as i32 == ':' as i32
                    && *(*argv.offset(2_i32 as isize)).offset(1_i32 as isize) as i32 == 0)
                && (*argv.offset(3_i32 as isize)).is_null()
                || (crate::entry::opt_just_print(ctx) && !(flags & 1 != 0))
            {
                if !argv.is_null() {
                    free(*argv.offset(0_i32 as isize) as *mut ::core::ffi::c_void);
                    free(argv as *mut ::core::ffi::c_void);
                }
            } else {
                crate::output::output_start(ctx);
                fflush(stdout);
                fflush(stderr);
                (*child)
                    .set_good_stdin(!ctx.good_stdin_used.get() as i32 as ::core::ffi::c_uint
                        as ::core::ffi::c_uint);
                if (*child).good_stdin() != 0 {
                    ctx.good_stdin_used.set(true);
                }
                (*child).set_deleted(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                if (*child).environment.is_null() {
                    let any_recurse = match ctx.filenodes.get((*child).file) {
                        Some(node) => node
                            .lock()
                            .expect("file node poisoned")
                            .recipe
                            .as_ref()
                            .map(|r| r.any_recurse)
                            .unwrap_or(false),
                        None => false,
                    };
                    (*child).environment =
                        match target_environment(ctx, Some((*child).file), any_recurse as i32) {
                            Ok(env) => env,
                            Err(e) => {
                                // The recipe never launched, so the argv block is
                                // released and the sync-output context installed
                                // above is torn down before unwinding.
                                free(*argv.offset(0_i32 as isize) as *mut ::core::ffi::c_void);
                                free(argv as *mut ::core::ffi::c_void);
                                set_output_context(::core::ptr::null_mut::<output>());
                                return Err(e);
                            }
                        };
                }
                // Run the job locally unless it is successfully handed off to a
                // remote executor.
                let mut run_local = true;
                if (*child).remote() != 0 {
                    let mut is_remote: i32 = 0;
                    let mut used_stdin: i32 = 0;
                    let mut id: pid_t = 0;
                    if ctx.remote_backend.0.start_job(
                        argv,
                        (*child).environment,
                        if (*child).good_stdin() as i32 != 0 {
                            0
                        } else {
                            get_bad_stdin(ctx)
                        },
                        &raw mut is_remote,
                        &raw mut id,
                        &raw mut used_stdin,
                    ) == 0
                    {
                        if (*child).good_stdin() as i32 != 0 && used_stdin == 0 {
                            (*child)
                                .set_good_stdin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            ctx.good_stdin_used.set(false);
                        }
                        (*child)
                            .set_remote(is_remote as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*child).pid = id;
                        run_local = false;
                    }
                }
                if run_local {
                    block_sigs(ctx);
                    (*child).set_remote(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    jobserver_pre_child(ctx, (flags & 1 != 0) as i32);
                    (*child).pid = child_execute_job(
                        ctx,
                        child as *mut ChildBase,
                        (*child).good_stdin() as i32,
                        argv,
                    );
                    jobserver_post_child(ctx, (flags & 1 != 0) as i32);
                }
                if (*child).pid >= 0 {
                    ctx.job_counter.0.fetch_add(1, Ordering::Relaxed);
                }
                set_file_command_state_entry(
                    ctx,
                    (*child).file,
                    (*child).entry,
                    CommandState::Running,
                );
                if !argv.is_null() {
                    free(*argv.offset(0_i32 as isize) as *mut ::core::ffi::c_void);
                    free(argv as *mut ::core::ffi::c_void);
                }
                set_output_context(::core::ptr::null_mut::<output>());
                return Ok(());
            }
        }
    }
    // The tail always tears the output context down, so a rejection raised by
    // the next recipe line is held until that has run.
    let outcome = if job_next_command(child) != 0 {
        start_job_command(ctx, child)
    } else {
        set_file_command_state_entry(ctx, (*child).file, (*child).entry, cs_running);
        set_file_update_status_entry(ctx, (*child).file, (*child).entry, us_success);
        notice_finished_file(ctx, (*child).file, (*child).entry)?;
        Ok(())
    };
    set_output_context(::core::ptr::null_mut::<output>());
    outcome
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn start_waiting_job(
    ctx: &crate::execctx::ExecContext,
    c: *mut Child,
) -> Result<i32, crate::build_result::BuildError> {
    let f: FileId = (*c).file;
    let e: usize = (*c).entry;
    (*c).set_remote(ctx.remote_backend.0.can_start_job(true) as ::core::ffi::c_uint);
    if (*c).remote() == 0 && (job_slots_used(ctx) > 0 && load_too_high(ctx) != 0) {
        set_file_command_state_entry(ctx, f, e, cs_running);
        (*c).next = ctx.waiting_jobs.0.get();
        ctx.waiting_jobs.0.set(c);
        return Ok(0);
    }
    start_job_command(ctx, c)?;
    // Finished states (cs_not_started reset to success, cs_finished) need the
    // file noticed and the child freed; a still-running job does not.
    let mut finish = false;
    match file_command_state_entry(ctx, f, e) as i32 {
        2 => {
            (*c).next = ctx.children.0.get();
            if (*c).pid > 0 {
                if 0x4_i32 & db_level(ctx) != 0 {
                    let fname = file_name_cstr(ctx, (*c).file);
                    crate::output::trace_parts(&[
                        b"Putting child ",
                        &ptr_bytes(c),
                        b" (",
                        &fname[..fname.len() - 1],
                        b") PID ",
                        &pid_bytes((*c).pid),
                        if (*c).remote() as i32 != 0 {
                            b" (remote)"
                        } else {
                            b""
                        },
                        b" on the chain.\n",
                    ]);
                }
                ctx.job_slots_used.0.fetch_add(1, Ordering::Relaxed);
                if (*c).jobslot() as i32 == 0 {
                } else {
                    panic!("assertion failed: c->jobslot == 0");
                };
                (*c).set_jobslot(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            ctx.children.0.set(c);
            unblock_sigs(ctx);
        }
        0 => {
            set_file_update_status_entry(ctx, f, e, UpdateStatus::Success);
            finish = true;
        }
        3 => {
            finish = true;
        }
        _ => {
            if file_command_state_entry(ctx, f, e) as i32 == cs_finished as i32 {
            } else {
                panic!("assertion failed: f->command_state == cs_finished");
            };
        }
    }
    if finish {
        notice_finished_file(ctx, f, e)?;
        free_child(ctx, c)?;
    }
    Ok(1)
}
/// Fold one unescaped backslash-newline (and the whitespace around it) inside a
/// `$(...)`/`${...}` reference to a single space — or, when the backslash is
/// itself escaped, copy it through verbatim. `ref_0` marks the start of the
/// reference body and `outref` its first written byte (bounding the trailing-
/// whitespace rewind). Returns the advanced `(out, in_0)` write/read cursors.
///
/// # Safety
///
/// Cursors must point within one writable, NUL-terminated recipe-line buffer,
/// with `in_0` at a `\`-`\n` pair; `ref_0`/`outref` bound the current reference.
unsafe fn fold_ref_continuation(
    mut out: *mut ::core::ffi::c_char,
    mut in_0: *mut ::core::ffi::c_char,
    ref_0: *mut ::core::ffi::c_char,
    outref: *mut ::core::ffi::c_char,
) -> (*mut ::core::ffi::c_char, *mut ::core::ffi::c_char) {
    let mut quoted: i32 = 0;
    let mut p: *mut ::core::ffi::c_char = in_0.offset(-(1_i32 as isize));
    while p > ref_0 && *p as i32 == '\\' as i32 {
        quoted = (quoted == 0) as i32;
        p = p.offset(-1_i32 as isize);
    }
    if quoted != 0 {
        let fresh2 = in_0;
        in_0 = in_0.offset(1_i32 as isize);
        let fresh3 = out;
        out = out.offset(1_i32 as isize);
        *fresh3 = *fresh2;
    } else {
        in_0 = in_0.offset(2_i32 as isize);
        while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
            .offset(*in_0 as ::core::ffi::c_uchar as isize) as i32
            & (0x2_i32 | 0x4_i32)
            != 0
        {
            in_0 = in_0.offset(1_i32 as isize);
        }
        while out > outref
            && *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
                .offset(*out.offset(-1_i32 as isize) as ::core::ffi::c_uchar as isize)
                as i32
                & 0x2_i32
                != 0
        {
            out = out.offset(-1_i32 as isize);
        }
        let fresh4 = out;
        out = out.offset(1_i32 as isize);
        *fresh4 = ' ' as i32 as ::core::ffi::c_char;
    }
    (out, in_0)
}

/// Collapse one `$(...)`/`${...}` reference body in place. `ref_0` points just
/// past the `$`, at the opening paren; the body is copied to `out`, folding
/// continuations via [`fold_ref_continuation`], until the matching (nesting-
/// aware) close paren. Returns the advanced `(out, in_0)` cursors.
///
/// # Safety
///
/// Cursors must point within one writable, NUL-terminated recipe-line buffer,
/// with `ref_0` at the reference's opening paren.
unsafe fn collapse_one_ref(
    mut out: *mut ::core::ffi::c_char,
    mut in_0: *mut ::core::ffi::c_char,
    ref_0: *mut ::core::ffi::c_char,
) -> (*mut ::core::ffi::c_char, *mut ::core::ffi::c_char) {
    let openparen: ::core::ffi::c_char = *ref_0;
    let closeparen: ::core::ffi::c_char = (if openparen as i32 == '(' as i32 {
        ')' as i32
    } else {
        '}' as i32
    }) as ::core::ffi::c_char;
    let fresh0 = in_0;
    in_0 = in_0.offset(1_i32 as isize);
    let fresh1 = out;
    out = out.offset(1_i32 as isize);
    *fresh1 = *fresh0;
    let outref: *mut ::core::ffi::c_char = out;
    let mut count: i32 = 0;
    while *in_0 as i32 != 0 {
        if *in_0 as i32 == '\\' as i32 && *in_0.offset(1_i32 as isize) as i32 == '\n' as i32 {
            (out, in_0) = fold_ref_continuation(out, in_0, ref_0, outref);
        } else {
            if *in_0 as i32 == closeparen as i32 && {
                count -= 1;
                count < 0
            } {
                break;
            }
            if *in_0 as i32 == openparen as i32 {
                count += 1;
            }
            let fresh5 = in_0;
            in_0 = in_0.offset(1_i32 as isize);
            let fresh6 = out;
            out = out.offset(1_i32 as isize);
            *fresh6 = *fresh5;
        }
    }
    (out, in_0)
}

/// Collapse `$`-reference continuations in one already-NUL-terminated recipe
/// line, rewriting it in place (this was the inner per-line loop of
/// [`new_job`]). Outside a `$(...)`/`${...}` reference the bytes are copied
/// verbatim; inside one, an unescaped backslash-newline (and the whitespace
/// around it) is folded to a single space, exactly as GNU make does before a
/// recipe line is expanded.
///
/// # Safety
///
/// `line` must be a writable, NUL-terminated buffer; the rewrite only ever
/// shortens the line, so it stays within the original allocation.
unsafe fn collapse_dollar_refs(line: *mut ::core::ffi::c_char) {
    let mut out: *mut ::core::ffi::c_char = line;
    let mut in_0: *mut ::core::ffi::c_char = line;
    let mut ref_0: *mut ::core::ffi::c_char;
    loop {
        ref_0 = strchr(in_0, '$' as i32);
        if ref_0.is_null() {
            break;
        }
        ref_0 = ref_0.offset(1_i32 as isize);
        if out != in_0 {
            memmove(
                out as *mut ::core::ffi::c_void,
                in_0 as *const ::core::ffi::c_void,
                ref_0.offset_from(in_0) as ::core::ffi::c_long as size_t,
            );
        }
        out = out.offset(ref_0.offset_from(in_0) as ::core::ffi::c_long as isize);
        in_0 = ref_0;
        if *ref_0 as i32 == '(' as i32 || *ref_0 as i32 == '{' as i32 {
            (out, in_0) = collapse_one_ref(out, in_0, ref_0);
        }
    }
    if out != in_0 {
        memmove(
            out as *mut ::core::ffi::c_void,
            in_0 as *const ::core::ffi::c_void,
            strlen(in_0).wrapping_add(1),
        );
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn new_job(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    entry: usize,
) -> Result<(), crate::build_result::BuildError> {
    start_waiting_jobs(ctx)?;
    reap_children(ctx, 0, 0)?;

    // Chop the target's recipe into per-line text + flags. We clone the recipe,
    // chop the clone, then write the chopped lines back so the node's recipe is
    // populated too — all without holding the arena lock across a job spawn.
    // `entry` selects which inline entry of a double-colon target runs (0 =
    // head, i>=1 = double_colon[i-1]).
    let (mut chopped, dontcare) = {
        let node = ctx
            .filenodes
            .get(file)
            .expect("new_job requires an interned file");
        let mut guard = node.lock().expect("file node poisoned");
        let dontcare = guard.dontcare;
        let entry_node: &mut FileNode = if entry == 0 {
            &mut guard
        } else {
            &mut guard.double_colon[entry - 1]
        };
        let mut recipe = entry_node
            .recipe
            .clone()
            .expect("new_job requires a recipe");
        chop_commands(ctx, &mut recipe);
        // Persist the chopped view back onto the entry's recipe.
        if let Some(r) = entry_node.recipe.as_mut() {
            r.lines = recipe.lines.clone();
            r.any_recurse = recipe.any_recurse;
        }
        (recipe, dontcare)
    };

    // Allocate the child on the heap via Box (the former xcalloc); it owns its
    // expanded recipe lines.
    let mut boxed = Box::new(Child {
        cmd_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        environment: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        output: output {
            out: 0,
            err: 0,
            syncout: 0,
        },
        next: ::core::ptr::null_mut::<Child>(),
        file,
        entry,
        sh_batch_file: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        command_lines: Vec::new(),
        line_flags: Vec::new(),
        command_line: 0,
        command_buf: Vec::new(),
        command_ptr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pid: 0,
        remote: 0,
        noerror: 0,
        good_stdin: 0,
        deleted: 0,
        recursive: 0,
        jobslot: 0,
        dontcare: 0,
    });
    crate::output::output_init(ctx, &raw mut boxed.output);
    boxed.set_dontcare(dontcare as ::core::ffi::c_uint);
    set_output_context(if boxed.output.syncout() as i32 != 0 {
        &raw mut boxed.output
    } else {
        ::core::ptr::null_mut::<output>()
    });

    // Expand each chopped recipe line for this file, collapsing `$`-reference
    // continuations first (the former in-place `collapse_dollar_refs`). Each
    // expanded line is stored NUL-free in `command_lines`; its flags go in
    // `line_flags`.
    for line in chopped.lines.drain(..) {
        // collapse_dollar_refs rewrites in place over a NUL-terminated buffer.
        let mut buf = line.text.clone();
        buf.push(0);
        collapse_dollar_refs(buf.as_mut_ptr() as *mut ::core::ffi::c_char);
        let nul = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        buf.truncate(nul);
        buf.push(0);
        // BOUNDARY: `expand_string_for_file` is converging on the FileId form
        // (see implicit.rs); call it that way even though the expand layer is
        // still mid-flip. Returns the expanded bytes (NUL-terminated).
        let mut expanded: Vec<u8> = crate::expand::expand_string_for_file(ctx, &buf, file)?;
        // Drop the trailing NUL so each stored command line is NUL-free.
        if expanded.last() == Some(&0) {
            expanded.pop();
        }
        boxed.command_lines.push(expanded);
        boxed.line_flags.push(line.flags);
    }

    let c: *mut Child = Box::into_raw(boxed);
    job_next_command(c);
    // `job_slots` is fixed for the run (set only during `main_0` job setup), so
    // snapshot it once rather than reading the borrow channel each spin.
    let slots = crate::entry::opt_job_slots(ctx);
    if slots != 0 {
        while job_slots_used(ctx) == slots {
            reap_children(ctx, 1, 0)?;
        }
    } else if jobserver_enabled(ctx) != 0 {
        loop {
            if 0x4_i32 & db_level(ctx) != 0 {
                crate::output::trace_parts(&[
                    b"Need a job token; we ",
                    if !ctx.children.0.get().is_null() {
                        b""
                    } else {
                        b"don't ".as_slice()
                    },
                    b"have children\n",
                ]);
            }
            if jobserver_tokens(ctx) == 0 {
                break;
            }
            // `jobserver_pre_acquire`/`jobserver_acquire` are `Result`-returning
            // (#432 Phase B, #540: `std::process::exit` belongs only in
            // `bin/make.rs`'s `main()`), and `new_job` now returns one too, so a
            // fatal jobserver failure propagates instead of bridging (#441).
            jobserver_pre_acquire(ctx)?;
            reap_children(ctx, 0, 0)?;
            start_waiting_jobs(ctx)?;
            if jobserver_tokens(ctx) == 0 {
                break;
            }
            if ctx.children.0.get().is_null() {
                return Err(fatal_err(
                    ctx,
                    ::core::ptr::null_mut::<Floc>(),
                    0,
                    b"INTERNAL: no children as we go to sleep on read\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                ));
            }
            let got_token: i32 =
                jobserver_acquire(ctx, (ctx.waiting_jobs.0.get() != NULL as *mut Child) as i32)?
                    as i32;
            if !(got_token == 1) {
                continue;
            }
            if 0x4_i32 & db_level(ctx) != 0 {
                let fname = file_name_cstr(ctx, (*c).file);
                crate::output::trace_parts(&[
                    b"Obtained token for child ",
                    &ptr_bytes(c),
                    b" (",
                    &fname[..fname.len() - 1],
                    b").\n",
                ]);
            }
            break;
        }
    }
    ctx.jobserver_tokens.0.fetch_add(1, Ordering::Relaxed);
    if 0x20_i32 & db_level(ctx) != 0 {
        // Build the "update target '...' due to: ..." diagnostic from the arena
        // (the former pointer walk over `cmds.fileinfo`/`also_make`/`deps`).
        // Snapshot everything we need under the lock, then format and drop it.
        struct DbgInfo {
            defined_in: Option<Vec<u8>>,
            defined_lineno: u64,
            name: Vec<u8>,
            also_make_names: Vec<Vec<u8>>,
            phony: bool,
            last_mtime: u64,
            // Names of prerequisites that do not (yet) exist.
            nonexistent_deps: Vec<Vec<u8>>,
        }
        let info: Option<DbgInfo> = ctx.filenodes.get(file).map(|node| {
            let g = node.lock().expect("file node poisoned");
            let (defined_in, defined_lineno) = match g.recipe.as_ref() {
                Some(r) => (r.defined_in.clone(), r.defined_lineno),
                None => (None, 0),
            };
            let also_make_names = g
                .also_make
                .iter()
                .filter_map(|d| d.file)
                .filter_map(|fid| {
                    ctx.filenodes
                        .get(fid)
                        .map(|n| n.lock().expect("file node poisoned").name.clone())
                })
                .collect();
            let nonexistent_deps = g
                .deps
                .iter()
                .filter_map(|d| d.file)
                .filter_map(|fid| ctx.filenodes.get(fid))
                .filter_map(|n| {
                    let dg = n.lock().expect("file node poisoned");
                    if dg.last_mtime == NONEXISTENT_MTIME as u64 {
                        Some(dg.name.clone())
                    } else {
                        None
                    }
                })
                .collect();
            DbgInfo {
                defined_in,
                defined_lineno,
                name: g.name.clone(),
                also_make_names,
                phony: g.phony,
                last_mtime: g.last_mtime,
                nonexistent_deps,
            }
        });
        if let Some(info) = info {
            // Location label (`<builtin>` or `file:line`), NUL-terminated.
            let mut nm_buf: Vec<u8> = match &info.defined_in {
                None => b"<builtin>\0".to_vec(),
                Some(fnm) => {
                    let mut v = fnm.clone();
                    v.push(b':');
                    v.extend_from_slice(info.defined_lineno.to_string().as_bytes());
                    v.push(0);
                    v
                }
            };
            // Target list: the target name plus any also-make siblings, joined
            // by `', '`, NUL-terminated.
            let mut tp_buf: Vec<u8> = info.name.clone();
            for sib in &info.also_make_names {
                tp_buf.extend_from_slice(b"', '");
                tp_buf.extend_from_slice(sib);
            }
            tp_buf.push(0);
            let nm = nm_buf.as_mut_ptr() as *const ::core::ffi::c_char;
            let tp = tp_buf.as_mut_ptr() as *const ::core::ffi::c_char;
            if info.phony {
                message(
                    ctx,
                    0,
                    (strlen(nm) as size_t).wrapping_add(strlen(tp) as size_t),
                    b"%s: update target '%s' due to: target is .PHONY\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[FmtArg::Str(nm), FmtArg::Str(tp)],
                );
            } else if info.last_mtime == NONEXISTENT_MTIME as u64 {
                message(
                    ctx,
                    0,
                    (strlen(nm) as size_t).wrapping_add(strlen(tp) as size_t),
                    b"%s: update target '%s' due to: target does not exist\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[FmtArg::Str(nm), FmtArg::Str(tp)],
                );
            } else {
                // The set of newer prerequisites ($?), expanded for this file.
                // BOUNDARY: FileId-convention expand (see implicit.rs).
                let newer: Vec<u8> = crate::expand::expand_string_for_file(ctx, b"$?\0", file)?;
                if newer.first().is_some_and(|&b| b != 0) {
                    let mut newer_buf = newer.clone();
                    if newer_buf.last() != Some(&0) {
                        newer_buf.push(0);
                    }
                    let np = newer_buf.as_ptr() as *const ::core::ffi::c_char;
                    message(
                        ctx,
                        0,
                        (strlen(nm) as size_t)
                            .wrapping_add(strlen(tp) as size_t)
                            .wrapping_add(strlen(np) as size_t),
                        b"%s: update target '%s' due to: %s\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[FmtArg::Str(nm), FmtArg::Str(tp), FmtArg::Str(np)],
                    );
                } else if info.nonexistent_deps.is_empty() {
                    message(
                        ctx,
                        0,
                        (strlen(nm) as size_t).wrapping_add(strlen(tp) as size_t),
                        b"%s: update target '%s' due to: unknown reasons\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[FmtArg::Str(nm), FmtArg::Str(tp)],
                    );
                } else {
                    let mut newer_buf: Vec<u8> = Vec::new();
                    for dn in &info.nonexistent_deps {
                        if !newer_buf.is_empty() {
                            newer_buf.push(b' ');
                        }
                        newer_buf.extend_from_slice(dn);
                    }
                    newer_buf.push(0);
                    let np = newer_buf.as_ptr() as *const ::core::ffi::c_char;
                    message(
                        ctx,
                        0,
                        (strlen(nm) as size_t)
                            .wrapping_add(strlen(tp) as size_t)
                            .wrapping_add(strlen(np) as size_t),
                        b"%s: update target '%s' due to: %s\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[FmtArg::Str(nm), FmtArg::Str(tp), FmtArg::Str(np)],
                    );
                }
            }
        }
    }
    start_waiting_job(ctx, c)?;
    if crate::entry::opt_job_slots(ctx) == 1 || not_parallel(ctx) {
        while file_command_state(ctx, file) as i32 == cs_running as i32 {
            // Restore the output context before handing a reap failure back, so
            // the caller's diagnostics are not written into this job's captured
            // output block (#441).
            if let Err(e) = reap_children(ctx, 1, 0) {
                set_output_context(::core::ptr::null_mut::<output>());
                return Err(e);
            }
        }
    }
    set_output_context(::core::ptr::null_mut::<output>());
    Ok(())
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn job_next_command(child: *mut Child) -> i32 {
    // Advance to the next non-empty expanded line, loading it into the owned
    // `command_buf` and pointing `command_ptr` at its start. The former model
    // walked a `*mut *mut c_char` array and an in-place `command_ptr` cursor;
    // here each line is an owned `Vec<u8>` that we NUL-terminate into
    // `command_buf` so `command_ptr` (a raw cursor) stays valid while the line
    // is being consumed.
    while (*child).command_ptr.is_null() || *(*child).command_ptr as i32 == 0 {
        if (*child).command_line >= (*child).command_lines.len() {
            (*child).command_ptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return 0;
        }
        let idx = (*child).command_line;
        (*child).command_line = (*child).command_line.wrapping_add(1);
        // Own a NUL-terminated working copy of this line; `command_ptr` walks
        // within it (`start_job_command` may rewrite it in place).
        let command_lines_ref: &Vec<Vec<u8>> = &(*child).command_lines;
        let mut buf = command_lines_ref[idx].clone();
        buf.push(0);
        (*child).command_buf = buf;
        (*child).command_ptr = (*child).command_buf.as_mut_ptr() as *mut ::core::ffi::c_char;
    }
    1
}
pub const LOAD_WEIGHT_A: ::core::ffi::c_double = 0.25f64;
pub const LOAD_WEIGHT_B: ::core::ffi::c_double = 0.25f64;
/// Parse the number of currently-running jobs from `/proc/loadavg` contents:
/// the integer before `/` in the 4th whitespace-separated field (e.g. `1` in
/// `"0.00 0.01 0.05 1/234 5678"`). Returns `None` when that field is missing or
/// does not begin with a number — matching when C make's `load_too_high`
/// reports "Failed to parse /proc/loadavg".
fn loadavg_running_jobs(contents: &[u8]) -> Option<u32> {
    let field = contents.split(|&b| b == b' ').nth(3)?;
    let numerator = field.split(|&b| b == b'/').next()?;
    crate::misc::parse_uint_strtoul(numerator).ok()
}

/// The per-second load-sample fold from [`load_too_high`], extracted as a pure
/// function over its cache state so the smoothing arithmetic can be
/// differential-tested against the original `static mut last_now`/`last_sec`
/// implementation (see `load_sample_fold_oracle` in the tests).
///
/// Once per new wall-clock second it rolls the sample window forward: the
/// previous second's job count is carried at weight B only when the cached
/// second is exactly adjacent to `now` (otherwise the carry resets to zero),
/// the running job counter resets, and the cached second advances. It then
/// returns the updated cache triple plus the smoothed load `guess` (the actual
/// `load` plus weight A times the live job count and the carried weight). The
/// caller writes the returned `sample_second`/`prev_weight`/job counter back
/// into the `ExecContext` cells (`load_sample_second`/`load_prev_weight`/
/// `job_counter`); this matches the original, where the job counter was reset
/// inside the per-second branch and re-read when forming the guess.
fn load_sample_fold(
    sample_second: time_t,
    prev_weight: ::core::ffi::c_double,
    job_counter: u64,
    now: time_t,
    load: ::core::ffi::c_double,
) -> (time_t, ::core::ffi::c_double, u64, ::core::ffi::c_double) {
    let mut sample_second = sample_second;
    let mut prev_weight = prev_weight;
    let mut job_counter = job_counter;
    if sample_second < now {
        if sample_second == now - 1 as time_t {
            prev_weight = LOAD_WEIGHT_B * job_counter as ::core::ffi::c_double;
        } else {
            prev_weight = 0.0f64;
        }
        job_counter = 0;
        sample_second = now;
    }
    let guess: ::core::ffi::c_double =
        load + LOAD_WEIGHT_A * (job_counter as ::core::ffi::c_double + prev_weight);
    (sample_second, prev_weight, job_counter, guess)
}

/// Current wall-clock time in whole seconds since the Unix epoch, read through
/// `std::time` (CLOCK_REALTIME on Linux) rather than the C `time()` syscall.
/// `load_too_high` uses this only to bucket its per-second job-load samples, so
/// whole-second resolution — identical to what `time(NULL)` returned — is all
/// that is needed.
fn wall_clock_seconds() -> time_t {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs() as time_t,
        // Clock before the Unix epoch (pre-1970): not reachable in practice.
        // Mirror time()'s negative whole-second count rather than panicking.
        Err(e) => -(e.duration().as_secs() as time_t),
    }
}

pub unsafe fn load_too_high(ctx: &crate::execctx::ExecContext) -> i32 {
    // Lazily-probed `/proc/loadavg` descriptor cache, the former function-local
    // `static mut proc_fd`: `-2` before the first probe, `-1` once probing
    // failed, otherwise the open fd. Per-run state on the build-phase context.
    let proc_fd = &ctx.load_proc_fd.0;
    let mut load: ::core::ffi::c_double = 0.;
    if crate::entry::opt_max_load_average(ctx) < 0_i32 as ::core::ffi::c_double {
        return 0;
    }
    if proc_fd.get() == -2_i32 {
        loop {
            proc_fd.set(open(
                b"/proc/loadavg\0" as *const u8 as *const ::core::ffi::c_char,
                0,
            ));
            if !(proc_fd.get() == -1_i32 && *__errno_location() == EINTR) {
                break;
            }
        }
        if proc_fd.get() < 0 {
            if 0x4_i32 & db_level(ctx) != 0 {
                crate::output::trace_out(b"Using system load detection method.\n");
            }
        } else {
            if 0x4_i32 & db_level(ctx) != 0 {
                crate::output::trace_out(b"Using /proc/loadavg load detection method.\n");
            }
            fd_noinherit(proc_fd.get());
        }
    }
    if proc_fd.get() >= 0 {
        let mut r: i32;
        loop {
            r = lseek(proc_fd.get(), 0 as __off_t, 0) as i32;
            if !(r == -1_i32 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r >= 0 {
            let mut avg: [::core::ffi::c_char; 65] = [0; 65];
            loop {
                r = read(
                    proc_fd.get(),
                    &raw mut avg as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    64,
                ) as i32;
                if !(r == -1_i32 && *__errno_location() == EINTR) {
                    break;
                }
            }
            if r >= 0 {
                avg[r as usize] = 0;
                // SAFETY: avg[r] was just set to NUL, so this is a valid C string.
                let contents = ::core::ffi::CStr::from_ptr(avg.as_ptr()).to_bytes();
                if let Some(cnt) = loadavg_running_jobs(contents) {
                    if 0x4_i32 & db_level(ctx) != 0 {
                        crate::output::trace_parts(&[
                            b"Running: system = ",
                            cnt.to_string().as_bytes(),
                            b" / make = ",
                            job_slots_used(ctx).to_string().as_bytes(),
                            b" (max requested = ",
                            format!("{:.6}", crate::entry::opt_max_load_average(ctx)).as_bytes(),
                            b")\n",
                        ]);
                    }
                    return (cnt as ::core::ffi::c_double > crate::entry::opt_max_load_average(ctx))
                        as i32;
                }
                if 0x4_i32 & db_level(ctx) != 0 {
                    crate::output::trace_parts(&[
                        b"Failed to parse /proc/loadavg: ",
                        ::core::ffi::CStr::from_ptr(&raw const avg as *const ::core::ffi::c_char)
                            .to_bytes(),
                        b"\n",
                    ]);
                }
            }
        }
        if r < 0 && 0x4_i32 & db_level(ctx) != 0 {
            crate::output::trace_parts(&[
                b"Failed to read /proc/loadavg: ",
                ::core::ffi::CStr::from_ptr(strerror(*__errno_location())).to_bytes(),
                b"\n",
            ]);
        }
        close(proc_fd.get());
        proc_fd.set(-1_i32);
    }
    *__errno_location() = 0;
    if getloadavg(&raw mut load, 1) != 1 {
        // Last-reported `getloadavg` failure errno (the former function-local
        // `static mut lossage`), used to suppress repeating the same warning.
        let lossage = &ctx.load_lossage.0;
        if lossage.get() == -1_i32 || *__errno_location() != lossage.get() {
            if *__errno_location() == 0 {
                error(
                    ctx,
                    ::core::ptr::null_mut::<Floc>(),
                    0,
                    b"cannot enforce load limits on this operating system\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            } else {
                perror_with_name(
                    ctx,
                    b"cannot enforce load limit: \0" as *const u8 as *const ::core::ffi::c_char,
                    b"getloadavg\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        lossage.set(*__errno_location());
        load = 0_i32 as ::core::ffi::c_double;
    }
    let now: time_t = wall_clock_seconds();
    let (next_sample_second, next_prev_weight, next_job_counter, guess) = load_sample_fold(
        ctx.load_sample_second.get(),
        ctx.load_prev_weight.get(),
        ctx.job_counter.0.load(Ordering::Relaxed),
        now,
        load,
    );
    ctx.load_sample_second.set(next_sample_second);
    ctx.load_prev_weight.set(next_prev_weight);
    ctx.job_counter.0.store(next_job_counter, Ordering::Relaxed);
    if 0x4_i32 & db_level(ctx) != 0 {
        crate::output::trace_parts(&[
            b"Estimated system load = ",
            format!("{guess:.6}").as_bytes(),
            b" (actual = ",
            format!("{load:.6}").as_bytes(),
            b") (max requested = ",
            format!("{:.6}", crate::entry::opt_max_load_average(ctx)).as_bytes(),
            b")\n",
        ]);
    }
    (guess >= crate::entry::opt_max_load_average(ctx)) as i32
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn start_waiting_jobs(
    ctx: &crate::execctx::ExecContext,
) -> Result<(), crate::build_result::BuildError> {
    let mut job: *mut Child;
    if ctx.waiting_jobs.0.get().is_null() {
        return Ok(());
    }
    loop {
        reap_children(ctx, 0, 0)?;
        job = ctx.waiting_jobs.0.get();
        ctx.waiting_jobs.0.set((*job).next);
        if !(start_waiting_job(ctx, job)? != 0 && !ctx.waiting_jobs.0.get().is_null()) {
            break;
        }
    }
    Ok(())
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn child_execute_job(
    ctx: &crate::execctx::ExecContext,
    child: *mut ChildBase,
    good_stdin: i32,
    argv: *mut *mut ::core::ffi::c_char,
) -> pid_t {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let fdin: i32 = if good_stdin != 0 {
        fileno(stdin)
    } else {
        get_bad_stdin(ctx)
    };
    let mut fdout: i32 = fileno(stdout);
    let mut fderr: i32 = fileno(stderr);
    if (*child).output.syncout() != 0 {
        if (*child).output.out >= 0 {
            fdout = (*child).output.out;
        }
        if (*child).output.err >= 0 {
            fderr = (*child).output.err;
        }
    }
    let mut pid: pid_t = -(1 as pid_t);
    let r = spawn_child(
        child,
        argv,
        fdin,
        fdout,
        fderr,
        &raw mut pid,
        &mut alloca_allocations,
    );
    if r != 0 {
        pid = -1_i32 as pid_t;
    }
    if pid < 0 {
        error(
            ctx,
            ::core::ptr::null_mut::<Floc>(),
            (strlen(*argv.offset(0_i32 as isize)) as size_t)
                .wrapping_add(strlen(strerror(r)) as size_t),
            b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                FmtArg::Str((*argv.offset(0_i32 as isize)) as *const ::core::ffi::c_char),
                FmtArg::Str((strerror(r)) as *const ::core::ffi::c_char),
            ],
        );
    }
    pid
}
/// Launch `argv[0]` via [`std::process::Command`], looking it up on the
/// child's PATH. Returns the spawn `errno` (0 on success) and writes the new
/// pid into `*pid`. A command that exists but is not directly executable
/// (`ENOEXEC`) is retried as an argument to the default shell, exactly as
/// the former `posix_spawn` version did.
unsafe fn spawn_child(
    child: *mut ChildBase,
    argv: *mut *mut ::core::ffi::c_char,
    fdin: i32,
    fdout: i32,
    fderr: i32,
    pid: *mut pid_t,
    alloca_allocations: &mut Vec<Vec<u8>>,
) -> i32 {
    // Find PATH in the child's environment (falling back to confstr), then
    // resolve and spawn argv[0].
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pp: *mut *mut ::core::ffi::c_char = (*child).environment;
    while !(*pp).is_null() {
        if *(*pp).offset(0) as i32 == 'P' as i32
            && *(*pp).offset(1) as i32 == 'A' as i32
            && *(*pp).offset(2) as i32 == 'T' as i32
            && *(*pp).offset(3) as i32 == 'H' as i32
            && *(*pp).offset(4) as i32 == '=' as i32
        {
            p = (*pp).offset(5);
            break;
        }
        pp = pp.offset(1);
    }
    if p.is_null() {
        let l: size_t = confstr(
            _CS_PATH as i32,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0,
        ) as size_t;
        if l != 0 {
            alloca_allocations.push(::std::vec::from_elem(0, l as usize));
            let dp: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            confstr(_CS_PATH as i32, dp, l as size_t);
            p = dp;
        }
    }
    let cmd: *mut ::core::ffi::c_char = find_in_given_path(
        *argv.offset(0),
        p,
        ::core::ptr::null::<::core::ffi::c_char>(),
        false,
    ) as *mut ::core::ffi::c_char;
    if cmd.is_null() {
        return *__errno_location();
    }
    let mut r = spawn_via_std(cmd, argv, (*child).environment, fdin, fdout, fderr, pid);
    if r == ENOEXEC {
        // Not a directly executable file: retry it as an argument to the shell.
        let mut l_0: size_t = 0;
        let mut pp_0: *mut *mut ::core::ffi::c_char = argv;
        while pp_0.as_ref().is_some_and(|p| !p.is_null()) {
            l_0 = l_0.wrapping_add(1);
            pp_0 = pp_0.offset(1);
        }
        let nargv: *mut *mut ::core::ffi::c_char = xmalloc(
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul(l_0.wrapping_add(3)),
        ) as *mut *mut ::core::ffi::c_char;
        *nargv.offset(0) = default_shell as *mut ::core::ffi::c_char;
        *nargv.offset(1) = cmd;
        memcpy(
            nargv.offset(2) as *mut ::core::ffi::c_void,
            argv.offset(1) as *const ::core::ffi::c_void,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul(l_0 as size_t),
        );
        r = spawn_via_std(
            *nargv.offset(0),
            nargv,
            (*child).environment,
            fdin,
            fdout,
            fderr,
            pid,
        );
        free(nargv as *mut ::core::ffi::c_void);
    }
    if r == 0 {
        free((*child).cmd_name as *mut ::core::ffi::c_void);
        (*child).cmd_name = if cmd != *argv.offset(0) {
            cmd
        } else {
            xstrdup(cmd)
        };
    }
    r
}

/// Fork and exec `file` through [`std::process::Command`], which owns the
/// fork and the spawn-error reporting pipe. The exec itself is a verbatim
/// `execve` in a `pre_exec` hook so the child sees make's argv and `envp`
/// byte-identically — `Command`'s own env handling stores variables in a
/// sorted map, which would reorder (and dedupe) the child's `environ`
/// relative to the C oracle. The hook also clears the child's signal mask
/// (the former `POSIX_SPAWN_SETSIGMASK` setup) — the parent spawns with
/// `block_sigs`' mask held. Returns the spawn `errno` (0 on success) and
/// writes the new pid into `*pid`.
///
/// The returned [`std::process::Child`] handle is dropped without waiting:
/// reaping stays centralized in `reap_children`'s `wait`/`waitpid(-1)`,
/// which is shared with `$(shell)` and remote children.
#[cfg(unix)]
unsafe fn spawn_via_std(
    file: *const ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    envp: *mut *mut ::core::ffi::c_char,
    fdin: i32,
    fdout: i32,
    fderr: i32,
    pid: *mut pid_t,
) -> i32 {
    use std::os::unix::{ffi::OsStrExt, process::CommandExt};
    // Raw pointers are not `Send`/`Sync`, which `pre_exec`'s closure must be;
    // they cross the fork as plain addresses (valid in the child's copied
    // address space) and are only dereferenced there.
    let file_addr = file as usize;
    let argv_addr = argv as usize;
    let envp_addr = envp as usize;
    let stdin_fd = fileno(stdin);
    let stdout_fd = fileno(stdout);
    let stderr_fd = fileno(stderr);
    let program = ::std::ffi::OsStr::from_bytes(::core::ffi::CStr::from_ptr(file).to_bytes());
    let mut command = std::process::Command::new(program);
    command.pre_exec(move || {
        // Runs in the forked child. Route the job's stdio exactly as the
        // former posix_spawn file actions did, then exec. On any failure the
        // errno reaches the parent through Command's report pipe and
        // `spawn()` returns it.
        //
        // The parent holds `block_sigs`' fatal-signal mask across the spawn,
        // and recipes must start with an empty mask (the former
        // `POSIX_SPAWN_SETSIGMASK` contract) or they would ignore the
        // SIGTERM/SIGINT make passes on. std already clears the child's mask
        // before running these hooks, but that is an implementation detail,
        // not a documented `pre_exec` guarantee — clear it explicitly.
        let mut empty: sigset_t = crate::entry::SigsetT { __val: [0; 16] };
        sigemptyset(&raw mut empty);
        if sigprocmask(
            SIG_SETMASK,
            &raw const empty,
            ::core::ptr::null_mut::<sigset_t>(),
        ) < 0
        {
            return Err(::std::io::Error::last_os_error());
        }
        if fdin >= 0 && fdin != stdin_fd && libc::dup2(fdin, stdin_fd) < 0 {
            return Err(::std::io::Error::last_os_error());
        }
        if fdout != stdout_fd && libc::dup2(fdout, stdout_fd) < 0 {
            return Err(::std::io::Error::last_os_error());
        }
        if fderr != stderr_fd && libc::dup2(fderr, stderr_fd) < 0 {
            return Err(::std::io::Error::last_os_error());
        }
        libc::execve(
            file_addr as *const ::core::ffi::c_char,
            argv_addr as *const *const ::core::ffi::c_char,
            envp_addr as *const *const ::core::ffi::c_char,
        );
        Err(::std::io::Error::last_os_error())
    });
    loop {
        match command.spawn() {
            Ok(spawned) => {
                *pid = spawned.id() as pid_t;
                return 0;
            }
            Err(e) => {
                let errno = e.raw_os_error().unwrap_or(libc::EINVAL);
                if errno != EINTR {
                    return errno;
                }
            }
        }
    }
}

/// wasm has no `fork`/`exec`: recipe execution is an accepted, tracked
/// architectural gap on this target (the crate only needs to *compile* for
/// `wasm32-wasip1`, not run recipes there). Report the spawn as failed with
/// `ENOSYS` rather than panicking, so callers built for wasm still behave
/// sanely if this path is ever reached.
#[cfg(target_family = "wasm")]
unsafe fn spawn_via_std(
    _file: *const ::core::ffi::c_char,
    _argv: *mut *mut ::core::ffi::c_char,
    _envp: *mut *mut ::core::ffi::c_char,
    _fdin: i32,
    _fdout: i32,
    _fderr: i32,
    _pid: *mut pid_t,
) -> i32 {
    libc::ENOSYS
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn exec_command(
    ctx: &crate::execctx::ExecContext,
    argv: *mut *mut ::core::ffi::c_char,
    envp: *mut *mut ::core::ffi::c_char,
) -> pid_t {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let pid: pid_t = -(1 as pid_t);
    environ = envp;
    execvp(
        *argv.offset(0_i32 as isize),
        argv as *const *mut ::core::ffi::c_char,
    );
    match *__errno_location() {
        ENOENT => {
            error(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                (strlen(*argv.offset(0_i32 as isize)) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    FmtArg::Str((*argv.offset(0_i32 as isize)) as *const ::core::ffi::c_char),
                    FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char),
                ],
            );
        }
        ENOEXEC => {
            let mut shell: *const ::core::ffi::c_char;
            let new_argv: *mut *mut ::core::ffi::c_char;
            let mut argc: i32;
            let i: i32 = 1;
            shell = getenv(b"SHELL\0" as *const u8 as *const ::core::ffi::c_char);
            if shell.is_null() {
                shell = default_shell;
            }
            argc = 1;
            while !(*argv.offset(argc as isize)).is_null() {
                argc += 1;
            }
            alloca_allocations.push(::std::vec::from_elem(
                0,
                ((1 + argc + 1) as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                    as usize,
            ));
            new_argv = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                as *mut *mut ::core::ffi::c_char;
            let fresh49 = &mut (*new_argv.offset(0_i32 as isize));
            *fresh49 = shell as *mut ::core::ffi::c_char;
            let fresh50 = &mut (*new_argv.offset(i as isize));
            *fresh50 = *argv.offset(0_i32 as isize);
            while argc > 0 {
                let fresh51 = &mut (*new_argv.offset((i + argc) as isize));
                *fresh51 = *argv.offset(argc as isize);
                argc -= 1;
            }
            execvp(shell, new_argv as *const *mut ::core::ffi::c_char);
            error(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                (strlen(*new_argv.offset(0_i32 as isize)) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    FmtArg::Str((*new_argv.offset(0_i32 as isize)) as *const ::core::ffi::c_char),
                    FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char),
                ],
            );
        }
        _ => {
            error(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                (strlen(*argv.offset(0_i32 as isize)) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    FmtArg::Str((*argv.offset(0_i32 as isize)) as *const ::core::ffi::c_char),
                    FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char),
                ],
            );
        }
    }
    pid
}
#[allow(clippy::too_many_arguments)]
unsafe fn construct_command_argv_internal(
    ctx: &crate::execctx::ExecContext,
    mut line: *mut ::core::ffi::c_char,
    restp: *mut *mut ::core::ffi::c_char,
    mut shell: *const ::core::ffi::c_char,
    shellflags: *const ::core::ffi::c_char,
    ifs: *const ::core::ffi::c_char,
    flags: i32,
    mut _batch_filename: *mut *mut ::core::ffi::c_char,
) -> Result<*mut *mut ::core::ffi::c_char, crate::build_result::BuildError> {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    // Read-only tables (never reassigned): `const` avoids the `Sync` bound a
    // `static` would need for these raw-pointer elements (each use site gets
    // its own inlined copy — fine for tables this small that never mutate).
    const sh_chars: *const ::core::ffi::c_char =
        b"#;\"*?[]&|<>(){}$`^~!\0" as *const u8 as *const ::core::ffi::c_char;
    const sh_cmds: [*const ::core::ffi::c_char; 38] = [
        b".\0" as *const u8 as *const ::core::ffi::c_char,
        b":\0" as *const u8 as *const ::core::ffi::c_char,
        b"alias\0" as *const u8 as *const ::core::ffi::c_char,
        b"bg\0" as *const u8 as *const ::core::ffi::c_char,
        b"break\0" as *const u8 as *const ::core::ffi::c_char,
        b"case\0" as *const u8 as *const ::core::ffi::c_char,
        b"cd\0" as *const u8 as *const ::core::ffi::c_char,
        b"command\0" as *const u8 as *const ::core::ffi::c_char,
        b"continue\0" as *const u8 as *const ::core::ffi::c_char,
        b"eval\0" as *const u8 as *const ::core::ffi::c_char,
        b"exec\0" as *const u8 as *const ::core::ffi::c_char,
        b"exit\0" as *const u8 as *const ::core::ffi::c_char,
        b"export\0" as *const u8 as *const ::core::ffi::c_char,
        b"fc\0" as *const u8 as *const ::core::ffi::c_char,
        b"fg\0" as *const u8 as *const ::core::ffi::c_char,
        b"for\0" as *const u8 as *const ::core::ffi::c_char,
        b"getopts\0" as *const u8 as *const ::core::ffi::c_char,
        b"hash\0" as *const u8 as *const ::core::ffi::c_char,
        b"if\0" as *const u8 as *const ::core::ffi::c_char,
        b"jobs\0" as *const u8 as *const ::core::ffi::c_char,
        b"login\0" as *const u8 as *const ::core::ffi::c_char,
        b"logout\0" as *const u8 as *const ::core::ffi::c_char,
        b"read\0" as *const u8 as *const ::core::ffi::c_char,
        b"readonly\0" as *const u8 as *const ::core::ffi::c_char,
        b"return\0" as *const u8 as *const ::core::ffi::c_char,
        b"set\0" as *const u8 as *const ::core::ffi::c_char,
        b"shift\0" as *const u8 as *const ::core::ffi::c_char,
        b"test\0" as *const u8 as *const ::core::ffi::c_char,
        b"times\0" as *const u8 as *const ::core::ffi::c_char,
        b"trap\0" as *const u8 as *const ::core::ffi::c_char,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
        b"ulimit\0" as *const u8 as *const ::core::ffi::c_char,
        b"umask\0" as *const u8 as *const ::core::ffi::c_char,
        b"unalias\0" as *const u8 as *const ::core::ffi::c_char,
        b"unset\0" as *const u8 as *const ::core::ffi::c_char,
        b"wait\0" as *const u8 as *const ::core::ffi::c_char,
        b"while\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut i: size_t;
    let mut p: *mut ::core::ffi::c_char;
    let end: *mut ::core::ffi::c_char;
    let mut ap: *mut ::core::ffi::c_char;
    let mut cap: *const ::core::ffi::c_char;
    let mut cp: *const ::core::ffi::c_char;
    let mut instring: i32;
    let mut word_has_equals: i32;
    let mut seen_nonequals: i32;
    let mut last_argument_was_empty: i32;
    let mut new_argv: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut argstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !restp.is_null() {
        *restp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
        .offset(*line as ::core::ffi::c_uchar as isize) as i32
        & 0x2_i32
        != 0
    {
        line = line.offset(1_i32 as isize);
    }
    if *line as i32 == 0 {
        return Ok(::core::ptr::null_mut::<*mut ::core::ffi::c_char>());
    }
    if shell.is_null() {
        shell = default_shell;
    }
    'fast: {
        // Fast path: split the recipe line into argv and exec it directly,
        // skipping the shell. Valid only for the default shell, a whitespace
        // IFS, and -c/-ec flags; any shell metacharacter, builtin or oddity
        // found below bails to the slow path with `break 'fast`.
        if strcmp(shell, default_shell) != 0 {
            break 'fast;
        }
        if !ifs.is_null() {
            cap = ifs;
            loop {
                // SAFETY: `cap` walks the NUL-terminated IFS string starting at
                // the non-null `ifs`; read each byte through a checked reference.
                let c = *cap.as_ref().unwrap() as i32;
                if c == 0 {
                    break;
                }
                if c != ' ' as i32 && c != '\t' as i32 && c != '\n' as i32 {
                    break 'fast;
                }
                cap = cap.offset(1_i32 as isize);
            }
        }
        if !shellflags.is_null()
            && (*shellflags.offset(0_i32 as isize) as i32 != '-' as i32
                || (*shellflags.offset(1_i32 as isize) as i32 != 'c' as i32
                    || *shellflags.offset(2_i32 as isize) as i32 != 0)
                    && (*shellflags.offset(1_i32 as isize) as i32 != 'e' as i32
                        || *shellflags.offset(2_i32 as isize) as i32 != 'c' as i32
                        || *shellflags.offset(3_i32 as isize) as i32 != 0))
        {
            break 'fast;
        }
        i = strlen(line).wrapping_add(1) as size_t;
        new_argv =
            xmalloc(i.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t))
                as *mut *mut ::core::ffi::c_char;
        argstr = xmalloc(i) as *mut ::core::ffi::c_char;
        let fresh16 = &mut (*new_argv.offset(0_i32 as isize));
        *fresh16 = argstr;
        ap = *fresh16;
        end = ap.offset(i as isize);
        i = 0;
        last_argument_was_empty = 0;
        seen_nonequals = last_argument_was_empty;
        word_has_equals = seen_nonequals;
        instring = word_has_equals;
        p = line;
        loop {
            if !(*p as i32 != 0) {
                break;
            }
            if ap <= end {
            } else {
                panic!("assertion failed: ap <= end");
            };
            if instring != 0 {
                if *p as i32 == instring {
                    instring = 0;
                    if ap == *new_argv.offset(0_i32 as isize)
                        || *ap.offset(-(1_i32 as isize)) as i32 == 0
                    {
                        last_argument_was_empty = 1;
                    }
                } else if *p as i32 == '\\' as i32
                    && *p.offset(1_i32 as isize) as i32 == '\n' as i32
                {
                    if instring == '"' as i32 {
                        p = p.offset(1_i32 as isize);
                    } else {
                        let fresh17 = p;
                        p = p.offset(1_i32 as isize);
                        let fresh18 = ap;
                        ap = ap.offset(1_i32 as isize);
                        *fresh18 = *fresh17;
                        let fresh19 = ap;
                        ap = ap.offset(1_i32 as isize);
                        *fresh19 = *p;
                    }
                } else if *p as i32 == '\n' as i32 && !restp.is_null() {
                    *restp = p;
                    break;
                } else {
                    if instring == '"' as i32
                        && !strchr(
                            b"\\$`\0" as *const u8 as *const ::core::ffi::c_char,
                            *p as i32,
                        )
                        .is_null()
                        && ctx.shell_kind() == crate::execctx::ShellKind::Unixy
                    {
                        break 'fast;
                    }
                    let fresh20 = ap;
                    ap = ap.offset(1_i32 as isize);
                    *fresh20 = *p;
                }
            } else {
                if !strchr(sh_chars, *p as i32).is_null() {
                    break 'fast;
                }
                if one_shell(ctx) && *p as i32 == '\n' as i32 {
                    break 'fast;
                }
                match *p as i32 {
                    61 => {
                        if seen_nonequals == 0
                            && ctx.shell_kind() == crate::execctx::ShellKind::Unixy
                        {
                            break 'fast;
                        }
                        word_has_equals = 1;
                        let fresh21 = ap;
                        ap = ap.offset(1_i32 as isize);
                        *fresh21 = '=' as i32 as ::core::ffi::c_char;
                    }
                    92 => {
                        if *p.offset(1_i32 as isize) as i32 == '\n' as i32 {
                            p = p.offset(1_i32 as isize);
                            if ap == *new_argv.offset(i as isize) {
                                while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
                                    .offset(
                                        *p.offset(1_i32 as isize) as ::core::ffi::c_uchar as isize
                                    ) as i32
                                    & 0x2_i32
                                    != 0
                                {
                                    p = p.offset(1_i32 as isize);
                                }
                            }
                        } else if *p.offset(1_i32 as isize) as i32 != 0 {
                            p = p.offset(1_i32 as isize);
                            let fresh22 = ap;
                            ap = ap.offset(1_i32 as isize);
                            *fresh22 = *p;
                        }
                    }
                    39 | 34 => {
                        instring = *p as i32;
                    }
                    10 => {
                        if !restp.is_null() {
                            *restp = p;
                            break;
                        } else {
                            let fresh23 = ap;
                            ap = ap.offset(1_i32 as isize);
                            *fresh23 = '\n' as i32 as ::core::ffi::c_char;
                        }
                    }
                    32 | 9 => {
                        let fresh24 = ap;
                        ap = ap.offset(1_i32 as isize);
                        *fresh24 = 0;
                        i = i.wrapping_add(1);
                        let fresh25 = &mut (*new_argv.add(i));
                        *fresh25 = ap;
                        last_argument_was_empty = 0;
                        seen_nonequals |= (word_has_equals == 0) as i32;
                        if word_has_equals != 0 && seen_nonequals == 0 {
                            break 'fast;
                        }
                        word_has_equals = 0;
                        if i == 1 {
                            let mut j: i32;
                            j = 0;
                            while !sh_cmds[j as usize].is_null() {
                                if *sh_cmds[j as usize] as i32
                                    == **new_argv.offset(0_i32 as isize) as i32
                                    && (*sh_cmds[j as usize] as i32 == 0
                                        || strcmp(
                                            sh_cmds[j as usize].offset(1_i32 as isize),
                                            (*new_argv.offset(0_i32 as isize))
                                                .offset(1_i32 as isize),
                                        ) == 0)
                                {
                                    break 'fast;
                                }
                                j += 1;
                            }
                        }
                        while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
                            .offset(*p.offset(1_i32 as isize) as ::core::ffi::c_uchar as isize)
                            as i32
                            & 0x2_i32
                            != 0
                        {
                            p = p.offset(1_i32 as isize);
                        }
                    }
                    _ => {
                        let fresh26 = ap;
                        ap = ap.offset(1_i32 as isize);
                        *fresh26 = *p;
                    }
                }
            }
            p = p.offset(1_i32 as isize);
        }
        if instring != 0 {
            break 'fast;
        }
        *ap = 0;
        if *(*new_argv.add(i)).offset(0_i32 as isize) as i32 != 0 || last_argument_was_empty != 0 {
            i = i.wrapping_add(1);
        }
        *new_argv.offset(i as isize) = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if i == 1 {
            // a lone shell builtin must be run through the shell
            let mut j_0: i32 = 0;
            while !sh_cmds[j_0 as usize].is_null() {
                if *sh_cmds[j_0 as usize] as i32 == **new_argv.offset(0_i32 as isize) as i32
                    && (*sh_cmds[j_0 as usize] as i32 == 0
                        || strcmp(
                            sh_cmds[j_0 as usize].offset(1_i32 as isize),
                            (*new_argv.offset(0_i32 as isize)).offset(1_i32 as isize),
                        ) == 0)
                {
                    break 'fast;
                }
                j_0 += 1;
            }
        }
        if (*new_argv.offset(0_i32 as isize)).is_null() {
            free(argstr as *mut ::core::ffi::c_void);
            free(new_argv as *mut ::core::ffi::c_void);
            return Ok(::core::ptr::null_mut::<*mut ::core::ffi::c_char>());
        }
        return Ok(new_argv);
    }
    if !new_argv.is_null() {
        free(argstr as *mut ::core::ffi::c_void);
        free(new_argv as *mut ::core::ffi::c_void);
    }
    let new_line: *mut ::core::ffi::c_char;
    let shell_len: size_t = strlen(shell) as size_t;
    let line_len: size_t = strlen(line) as size_t;
    let sflags_len: size_t = if !shellflags.is_null() {
        strlen(shellflags) as size_t
    } else {
        0
    };
    if one_shell(ctx) {
        if is_bourne_compatible_shell(path_from_cstr(shell)) {
            let mut f: *const ::core::ffi::c_char = line;
            let mut t: *mut ::core::ffi::c_char = line;
            while *f.offset(0_i32 as isize) as i32 != 0 {
                let mut esc: i32 = 0;
                while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
                    .offset(*f as ::core::ffi::c_uchar as isize) as i32
                    & 0x2_i32
                    != 0
                    || *f as i32 == '-' as i32
                    || *f as i32 == '@' as i32
                    || *f as i32 == '+' as i32
                {
                    f = f.offset(1_i32 as isize);
                }
                while *f as i32 != 0 {
                    let fresh28 = f;
                    f = f.offset(1_i32 as isize);
                    let fresh29 = t;
                    t = t.offset(1_i32 as isize);
                    *fresh29 = *fresh28;
                    if *f.offset(-1_i32 as isize) as i32 == '\\' as i32 {
                        esc = (esc == 0) as i32;
                    } else {
                        if *f.offset(-1_i32 as isize) as i32 == '\n' as i32 && esc == 0 {
                            break;
                        }
                        esc = 0;
                    }
                }
            }
            *t = 0;
        }
        let mut n: i32 = 1;
        let mut nextp: *mut ::core::ffi::c_char;
        new_argv = xmalloc(
            (4 as size_t)
                .wrapping_add(sflags_len.wrapping_div(2))
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
        ) as *mut *mut ::core::ffi::c_char;
        let fresh30 = &mut (*new_argv.offset(0_i32 as isize));
        *fresh30 = xmalloc(
            shell_len
                .wrapping_add(sflags_len)
                .wrapping_add(line_len)
                .wrapping_add(3),
        ) as *mut ::core::ffi::c_char;
        nextp = *fresh30;
        nextp = mempcpy(
            nextp as *mut ::core::ffi::c_void,
            shell as *const ::core::ffi::c_void,
            (shell_len as size_t).wrapping_add(1),
        ) as *mut ::core::ffi::c_char;
        if shellflags.is_null() {
            let fresh31 = n;
            n += 1;
            let fresh32 = &mut (*new_argv.offset(fresh31 as isize));
            *fresh32 = nextp;
            let fresh33 = nextp;
            nextp = nextp.offset(1_i32 as isize);
            *fresh33 = 0;
        } else {
            let argv: *mut *mut ::core::ffi::c_char;
            alloca_allocations.push(::std::vec::from_elem(
                0,
                sflags_len.wrapping_add(1) as usize,
            ));
            let f_0: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                f_0 as *mut ::core::ffi::c_void,
                shellflags as *const ::core::ffi::c_void,
                (sflags_len as size_t).wrapping_add(1),
            );
            argv = construct_command_argv_internal(
                ctx,
                f_0,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                flags,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            )?;
            if !argv.is_null() {
                let mut a: *mut *mut ::core::ffi::c_char;
                a = argv;
                while !(*a).is_null() {
                    let fresh34 = n;
                    n += 1;
                    let fresh35 = &mut (*new_argv.offset(fresh34 as isize));
                    *fresh35 = nextp;
                    nextp = stpcpy(nextp, *a).offset(1_i32 as isize);
                    a = a.offset(1_i32 as isize);
                }
                free(*argv.offset(0_i32 as isize) as *mut ::core::ffi::c_void);
                free(argv as *mut ::core::ffi::c_void);
            }
        }
        let fresh36 = n;
        n += 1;
        let fresh37 = &mut (*new_argv.offset(fresh36 as isize));
        *fresh37 = nextp;
        memcpy(
            nextp as *mut ::core::ffi::c_void,
            line as *const ::core::ffi::c_void,
            (line_len as size_t).wrapping_add(1),
        );
        let fresh38 = n;
        let fresh39 = &mut (*new_argv.offset(fresh38 as isize));
        *fresh39 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return Ok(new_argv);
    }
    new_line = xmalloc(
        shell_len
            .wrapping_mul(2)
            .wrapping_add(1)
            .wrapping_add(sflags_len)
            .wrapping_add(1)
            .wrapping_add(line_len.wrapping_mul(2))
            .wrapping_add(1),
    ) as *mut ::core::ffi::c_char;
    ap = new_line;
    cp = shell;
    while let Some(&cc) = cp.as_ref().filter(|c| **c as i32 != 0) {
        if !strchr(sh_chars, cc as i32).is_null() {
            let fresh40 = ap;
            ap = ap.offset(1_i32 as isize);
            *fresh40 = '\\' as i32 as ::core::ffi::c_char;
        }
        let fresh41 = ap;
        ap = ap.offset(1_i32 as isize);
        *fresh41 = cc;
        cp = cp.offset(1_i32 as isize);
    }
    let fresh42 = ap;
    ap = ap.offset(1_i32 as isize);
    *fresh42 = ' ' as i32 as ::core::ffi::c_char;
    if !shellflags.is_null() {
        ap = mempcpy(
            ap as *mut ::core::ffi::c_void,
            shellflags as *const ::core::ffi::c_void,
            sflags_len as size_t,
        ) as *mut ::core::ffi::c_char;
        let fresh43 = ap;
        ap = ap.offset(1_i32 as isize);
        *fresh43 = ' ' as i32 as ::core::ffi::c_char;
    }
    p = line;
    while *p as i32 != 0 {
        if !restp.is_null() && *p as i32 == '\n' as i32 {
            *restp = p;
            break;
        } else {
            if *p as i32 == '\\' as i32 && *p.offset(1_i32 as isize) as i32 == '\n' as i32 {
                let fresh44 = ap;
                ap = ap.offset(1_i32 as isize);
                *fresh44 = '\\' as i32 as ::core::ffi::c_char;
                if ctx.shell_kind() != crate::execctx::ShellKind::Batch {
                    let fresh45 = ap;
                    ap = ap.offset(1_i32 as isize);
                    *fresh45 = '\\' as i32 as ::core::ffi::c_char;
                }
                let fresh46 = ap;
                ap = ap.offset(1_i32 as isize);
                *fresh46 = '\n' as i32 as ::core::ffi::c_char;
                p = p.offset(1_i32 as isize);
            } else {
                if ctx.shell_kind() == crate::execctx::ShellKind::Unixy
                    && (*p as i32 == '\\' as i32
                        || *p as i32 == '\'' as i32
                        || *p as i32 == '"' as i32
                        || *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
                            .offset(*p as ::core::ffi::c_uchar as isize)
                            as i32
                            & (0x2_i32 | 0x4_i32)
                            != 0
                        || !strchr(sh_chars, *p as i32).is_null())
                {
                    let fresh47 = ap;
                    ap = ap.offset(1_i32 as isize);
                    *fresh47 = '\\' as i32 as ::core::ffi::c_char;
                }
                let fresh48 = ap;
                ap = ap.offset(1_i32 as isize);
                *fresh48 = *p;
            }
            p = p.offset(1_i32 as isize);
        }
    }
    if ap
        == new_line
            .offset(shell_len as isize)
            .offset(sflags_len as isize)
            .offset(2_i32 as isize)
    {
        free(new_line as *mut ::core::ffi::c_void);
        return Ok(::core::ptr::null_mut::<*mut ::core::ffi::c_char>());
    }
    *ap = 0;
    if ctx.shell_kind() == crate::execctx::ShellKind::Unixy {
        // The nested call gets this same `ctx`, so it takes this same arm and
        // can only fail by way of its own nested call — inductively, never.
        // The rejection below is the function's only `Err`, so no cleanup is
        // owed here and the tail's `free(new_line)` always runs.
        new_argv = construct_command_argv_internal(
            ctx,
            new_line,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            flags,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        )?;
    } else {
        free(new_line as *mut ::core::ffi::c_void);
        return Err(fatal_err(
            ctx,
            NILF,
            (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t)
                .wrapping_sub(1)
                .wrapping_add(INTSTR_LENGTH),
            b"%s (line %d) Bad shell context (!unixy && !batch_mode_shell)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                FmtArg::Str(
                    (b"src/job.c\0" as *const u8 as *const ::core::ffi::c_char)
                        as *const ::core::ffi::c_char,
                ),
                FmtArg::Int((3621_i32) as i64),
            ],
        ));
    }
    free(new_line as *mut ::core::ffi::c_void);
    Ok(new_argv)
}
pub const PRESERVE_BSNL: i32 = 1;
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
/// Expand `string` (NUL-terminated) either in a target's variable context
/// (`Some(file)`) or in the global context (`None` — the former null
/// `*mut File`). Returns the expanded bytes, NUL-terminated.
unsafe fn expand_for_opt_file(
    ctx: &crate::execctx::ExecContext,
    string: &[u8],
    file: Option<FileId>,
) -> Result<Vec<u8>, crate::build_result::BuildError> {
    match file {
        Some(f) => crate::expand::expand_string_for_file(ctx, string, f),
        None => {
            let p = crate::expand::allocated_expand_string_for_file(
                ctx,
                string.as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<crate::file::File>(),
            )?;
            if p.is_null() {
                return Ok(vec![0]);
            }
            let len = libc::strlen(p) as usize;
            let mut v = ::core::slice::from_raw_parts(p as *const u8, len).to_vec();
            v.push(0);
            libc::free(p as *mut ::core::ffi::c_void);
            Ok(v)
        }
    }
}

pub unsafe fn construct_command_argv(
    ctx: &crate::execctx::ExecContext,
    line: *mut ::core::ffi::c_char,
    restp: *mut *mut ::core::ffi::c_char,
    file: Option<FileId>,
    cmd_flags: i32,
    batch_filename: *mut *mut ::core::ffi::c_char,
) -> Result<*mut *mut ::core::ffi::c_char, crate::build_result::BuildError> {
    // Look up SHELL/.SHELLFLAGS/IFS in the target's variable context (or the
    // global context when `file` is None — the former null `*mut File`). Each
    // returns an owned NUL-terminated buffer. Split out so the
    // undefined-variable suppression is restored on the rejection path as well
    // as the success one (the cleanup-paths contract from #561).
    let (shell_buf, shellflags_set, ifs_buf) = expand_shell_settings(ctx, file)?;
    let shellflags_owned: Vec<u8> = resolve_shellflags(ctx, &shellflags_set, cmd_flags);
    let shell = shell_buf.as_ptr() as *const ::core::ffi::c_char;
    let shellflags = shellflags_owned.as_ptr() as *const ::core::ffi::c_char;
    let ifs = ifs_buf.as_ptr() as *const ::core::ffi::c_char;
    // Returned directly rather than bound then `Ok`-wrapped, so the added
    // seam costs this frame no decision point.
    construct_command_argv_internal(
        ctx,
        line,
        restp,
        shell as *mut ::core::ffi::c_char,
        shellflags,
        ifs,
        cmd_flags,
        batch_filename,
    )
}

/// Pick the flags to hand the shell: a `.SHELLFLAGS` value that expanded to
/// something non-empty is used verbatim (NUL-terminated if it was not
/// already), otherwise the posix-pedantic `-ec` or the default `-c`.
fn resolve_shellflags(
    ctx: &crate::execctx::ExecContext,
    shellflags_set: &[u8],
    cmd_flags: i32,
) -> Vec<u8> {
    if shellflags_set.first().is_some_and(|&b| b != 0) {
        let mut v = shellflags_set.to_vec();
        if v.last() != Some(&0) {
            v.push(0);
        }
        v
    } else if posix_pedantic(ctx) && !crate::entry::opt_ignore_errors(ctx) && !(cmd_flags & 4 != 0)
    {
        b"-ec\0".to_vec()
    } else {
        b"-c\0".to_vec()
    }
}

/// Expand `$(SHELL)`, `$(.SHELLFLAGS)` and `$(IFS)` in `file`'s variable
/// context (or the global one when `file` is `None`), with the
/// undefined-variable warning suppressed for the duration.
///
/// The three expansions are held across the restore so that a rejected one
/// still puts the warning action back before the error leaves the frame.
///
/// # Safety
/// As [`construct_command_argv`].
type ShellSettings = (Vec<u8>, Vec<u8>, Vec<u8>);

unsafe fn expand_shell_settings(
    ctx: &crate::execctx::ExecContext,
    file: Option<FileId>,
) -> Result<ShellSettings, crate::build_result::BuildError> {
    let save: Action = warning::action(ctx, Type::UndefinedVar);
    warning::set_action(ctx, Type::UndefinedVar, Action::Ignore);
    let shell = expand_for_opt_file(ctx, b"$(SHELL)\0", file);
    let shellflags = expand_for_opt_file(ctx, b"$(.SHELLFLAGS)\0", file);
    let ifs = expand_for_opt_file(ctx, b"$(IFS)\0", file);
    warning::set_action(ctx, Type::UndefinedVar, save);
    Ok((shell?, shellflags?, ifs?))
}

#[cfg(test)]
mod loadavg_tests {
    use super::loadavg_running_jobs;

    #[test]
    fn parses_running_jobs_field() {
        // 4th field is "running/total"; we want the running count (numerator),
        // matching C strtoul stopping at '/'.
        assert_eq!(loadavg_running_jobs(b"0.00 0.01 0.05 1/234 5678"), Some(1));
        assert_eq!(loadavg_running_jobs(b"0.50 0.40 0.30 12/200 999"), Some(12));
        assert_eq!(loadavg_running_jobs(b"0.00 0.00 0.00 0/100 1"), Some(0));
    }

    #[test]
    fn rejects_malformed() {
        assert_eq!(loadavg_running_jobs(b"too short"), None); // fewer than 4 fields
        assert_eq!(loadavg_running_jobs(b"0.0 0.0 0.0 x/9 1"), None); // non-numeric
        assert_eq!(loadavg_running_jobs(b"0.0 0.0 0.0 /9 1"), None); // empty numerator
        assert_eq!(loadavg_running_jobs(b""), None);
    }
}

#[cfg(test)]
mod load_sample_fold_tests {
    use super::{load_sample_fold, time_t, LOAD_WEIGHT_A, LOAD_WEIGHT_B};

    /// Verbatim port of the original per-second fold from `load_too_high` as it
    /// stood before the cache was de-globalized — the `static mut last_now`
    /// (last sampled second) and `static mut last_sec` (carried weight) logic,
    /// with the global `JOB_COUNTER` reset modeled by the threaded
    /// `job_counter`. Kept as a `#[cfg(test)]` oracle per `AGENTS.md:30-37` so we
    /// can prove the extracted `load_sample_fold` is behavior-preserving.
    fn load_sample_fold_oracle(
        last_now: time_t,
        last_sec: ::core::ffi::c_double,
        job_counter: u64,
        now: time_t,
        load: ::core::ffi::c_double,
    ) -> (time_t, ::core::ffi::c_double, u64, ::core::ffi::c_double) {
        let mut last_now = last_now;
        let mut last_sec = last_sec;
        let mut job_counter = job_counter;
        if last_now < now {
            if last_now == now - 1 as time_t {
                last_sec = LOAD_WEIGHT_B * job_counter as ::core::ffi::c_double;
            } else {
                last_sec = 0.0f64;
            }
            job_counter = 0; // JOB_COUNTER.store(0, Relaxed)
            last_now = now;
        }
        let guess: ::core::ffi::c_double =
            load + LOAD_WEIGHT_A * (job_counter as ::core::ffi::c_double + last_sec);
        (last_now, last_sec, job_counter, guess)
    }

    /// Drive representative cache states through both the extracted
    /// `load_sample_fold` and the preserved oracle, asserting identical updated
    /// state and smoothed guess. Covers: first sample (gap from 0), an exactly
    /// adjacent second (carry of weight B), a multi-second gap (carry reset), a
    /// stale `now` that does not advance the window, and a zero-job second.
    #[test]
    fn matches_original_static_mut_fold() {
        let cases: &[(
            time_t,
            ::core::ffi::c_double,
            u64,
            time_t,
            ::core::ffi::c_double,
        )] = &[
            // (sample_second, prev_weight, job_counter, now, load)
            (0, 0.0, 8, 1000, 0.5),       // first sample: gap from 0 -> reset
            (1000, 0.0, 4, 1001, 0.5),    // adjacent second -> carry weight B
            (1000, 0.25, 12, 1005, 1.5),  // multi-second gap -> reset
            (1001, 1.0, 7, 1000, 2.0),    // stale now (< cached) -> no advance
            (1001, 1.0, 7, 1001, 2.0),    // equal now -> no advance
            (1000, 0.0, 0, 1001, 0.0),    // adjacent second, zero jobs
            (2000, 3.0, 99, 2001, 10.25), // adjacent second, large counter
        ];
        for &(sample_second, prev_weight, job_counter, now, load) in cases {
            let got = load_sample_fold(sample_second, prev_weight, job_counter, now, load);
            let want = load_sample_fold_oracle(sample_second, prev_weight, job_counter, now, load);
            assert_eq!(
                got, want,
                "fold diverged for (sample_second={sample_second}, prev_weight={prev_weight}, \
                 job_counter={job_counter}, now={now}, load={load})"
            );
        }
    }

    /// The carry/reset behavior is path-dependent across consecutive seconds, so
    /// also compare the two implementations when chained call-to-call: feed each
    /// one's output back as the next call's cache state and assert they stay in
    /// lockstep across a realistic timeline (steady ticks, a stall, a burst).
    #[test]
    fn matches_original_when_chained() {
        // (now, job_counter_at_call, load) for each successive sample.
        let timeline: &[(time_t, u64, ::core::ffi::c_double)] = &[
            (1000, 5, 0.10),
            (1001, 3, 0.20), // adjacent: carry from 1000
            (1002, 9, 0.30), // adjacent: carry from 1001
            (1010, 2, 0.40), // gap: reset
            (1011, 6, 0.50), // adjacent: carry from 1010
            (1011, 6, 0.55), // same second: no advance
            (1012, 0, 0.60), // adjacent, zero jobs
        ];
        let (mut s_new, mut w_new) = (0 as time_t, 0.0f64);
        let (mut s_ora, mut w_ora) = (0 as time_t, 0.0f64);
        for &(now, job_counter, load) in timeline {
            let (ns, nw, njc, ng) = load_sample_fold(s_new, w_new, job_counter, now, load);
            let (os, ow, ojc, og) = load_sample_fold_oracle(s_ora, w_ora, job_counter, now, load);
            assert_eq!(
                (ns, nw, njc, ng),
                (os, ow, ojc, og),
                "diverged at now={now}"
            );
            s_new = ns;
            w_new = nw;
            s_ora = os;
            w_ora = ow;
        }
    }
}

#[cfg(test)]
mod wall_clock_seconds_tests {
    use super::{time_t, wall_clock_seconds};

    extern "C" {
        fn time(__timer: *mut time_t) -> time_t;
    }

    /// Preserved original clock read: the C `time(NULL)` call that
    /// `load_too_high` used before the `std::time` conversion. Kept as a
    /// `#[cfg(test)]` oracle per `AGENTS.md:30-37` so the safe replacement can
    /// be differential-tested against it.
    unsafe fn wall_clock_seconds_oracle() -> time_t {
        time(::core::ptr::null_mut::<time_t>())
    }

    /// The `std::time` `wall_clock_seconds` must agree with the C `time()`
    /// oracle. Both read CLOCK_REALTIME and truncate to whole seconds, so they
    /// agree exactly except for the rare case where a second boundary falls
    /// between the two reads (microseconds apart) — a 1s tolerance covers that
    /// without being flaky, mirroring the `file_timestamp_now` oracle test.
    #[test]
    fn wall_clock_seconds_matches_time_oracle() {
        let oracle_before = unsafe { wall_clock_seconds_oracle() };
        let got = wall_clock_seconds();
        let oracle_after = unsafe { wall_clock_seconds_oracle() };

        assert!(got > 0, "wall clock is well past the Unix epoch");
        // `got` was sampled between the two oracle reads, so it must lie within
        // their (monotonic, <=1s wide) range.
        assert!(
            got >= oracle_before && got <= oracle_after,
            "got={got} not in [{oracle_before}, {oracle_after}]"
        );
        assert!(
            oracle_after - oracle_before <= 1,
            "oracle reads bracket at most a 1s boundary"
        );
    }
}

#[cfg(test)]
mod is_bourne_compatible_shell_tests {
    use {super::is_bourne_compatible_shell, std::path::Path};

    fn is_shell(s: &str) -> bool {
        is_bourne_compatible_shell(Path::new(s))
    }

    #[test]
    fn recognizes_known_shells_by_basename() {
        assert!(is_shell("sh"));
        assert!(is_shell("bash"));
        assert!(is_shell("/bin/sh"));
        assert!(is_shell("/usr/bin/bash"));
        assert!(is_shell("/usr/local/bin/dash"));
        assert!(is_shell("ksh"));
        assert!(is_shell("rksh"));
        assert!(is_shell("zsh"));
        assert!(is_shell("ash"));
    }

    #[test]
    fn rejects_non_bourne_shells() {
        assert!(!is_shell("/bin/csh"));
        assert!(!is_shell("tcsh"));
        assert!(!is_shell("/usr/bin/fish"));
        assert!(!is_shell("powershell"));
        assert!(!is_shell(""));
        // A name merely containing a shell isn't a match.
        assert!(!is_shell("/bin/bashful"));
        assert!(!is_shell("notsh"));
    }

    #[test]
    fn path_normalizes_trailing_separator() {
        // `Path` treats a trailing separator as part of the same final
        // component, so the stem of "/bin/sh/" is still "sh".
        assert!(is_shell("/bin/sh/"));
    }
}

#[cfg(test)]
mod good_stdin_used_tests {
    use crate::execctx::ExecContext;

    /// `good_stdin_used` defaults to false on a fresh `ExecContext` (stdin
    /// available) and round-trips through the owned `Cell` — the former
    /// `GOOD_STDIN_USED` global. The context is owned by the test, so no shared
    /// state leaks between tests.
    #[test]
    fn good_stdin_used_tracks_flag() {
        let ctx = ExecContext::default();
        assert!(
            !ctx.good_stdin_used.get(),
            "false means stdin still available"
        );

        ctx.good_stdin_used.set(true);
        assert!(
            ctx.good_stdin_used.get(),
            "true means stdin already claimed"
        );

        ctx.good_stdin_used.set(false);
        assert!(!ctx.good_stdin_used.get(), "cleared when the job is reaped");
    }
}

#[cfg(test)]
mod dead_children_tests {
    use {super::dead_children, crate::execctx::ExecContext, std::sync::atomic::Ordering};

    /// `dead_children(ctx)` reflects `ctx.dead_children`, and the atomic
    /// add/sub used by the signal handler and reap loop round-trip. Each test
    /// gets its own `ExecContext`, so unlike the former global counter there
    /// is nothing to restore for isolation from other tests.
    #[test]
    fn dead_children_counts_round_trip() {
        let ctx = ExecContext::default();
        assert_eq!(dead_children(&ctx), 0);

        ctx.dead_children.0.fetch_add(1, Ordering::Relaxed);
        ctx.dead_children.0.fetch_add(1, Ordering::Relaxed);
        assert_eq!(dead_children(&ctx), 2, "two reaped children pending");

        ctx.dead_children.0.fetch_sub(1, Ordering::Relaxed);
        assert_eq!(dead_children(&ctx), 1, "one processed");
    }
}

#[cfg(test)]
mod job_slots_used_tests {
    use {super::job_slots_used, crate::execctx::ExecContext, std::sync::atomic::Ordering};

    /// `job_slots_used(ctx)` reflects `ctx.job_slots_used`, and the add/sub
    /// used by the start/reap paths round-trip through it. Each test gets its
    /// own `ExecContext`, so unlike the former global counter there is
    /// nothing to restore for isolation from other tests.
    #[test]
    fn job_slots_used_counts_round_trip() {
        let ctx = ExecContext::default();
        assert_eq!(job_slots_used(&ctx), 0);

        ctx.job_slots_used.0.fetch_add(1, Ordering::Relaxed);
        ctx.job_slots_used.0.fetch_add(1, Ordering::Relaxed);
        assert_eq!(job_slots_used(&ctx), 2, "two slots in use");

        ctx.job_slots_used
            .0
            .store(job_slots_used(&ctx).wrapping_sub(1), Ordering::Relaxed);
        assert_eq!(job_slots_used(&ctx), 1, "one slot freed");
    }
}

#[cfg(test)]
mod jobserver_tokens_tests {
    use {super::jobserver_tokens, crate::execctx::ExecContext, std::sync::atomic::Ordering};

    /// `jobserver_tokens(ctx)` reflects `ctx.jobserver_tokens`, and the
    /// add/sub used by the acquire/free paths round-trip through it. Each
    /// test gets its own `ExecContext`, so unlike the former global counter
    /// there is nothing to restore for isolation from other tests.
    #[test]
    fn jobserver_tokens_counts_round_trip() {
        let ctx = ExecContext::default();
        assert_eq!(jobserver_tokens(&ctx), 0);

        ctx.jobserver_tokens.0.fetch_add(1, Ordering::Relaxed);
        ctx.jobserver_tokens.0.fetch_add(1, Ordering::Relaxed);
        assert_eq!(jobserver_tokens(&ctx), 2, "two tokens held");

        ctx.jobserver_tokens.0.fetch_sub(1, Ordering::Relaxed);
        assert_eq!(jobserver_tokens(&ctx), 1, "one released");
    }
}

#[cfg(test)]
mod shell_kind_tests {
    use crate::execctx::{ExecContext, ShellKind};

    /// `ctx.shell_kind()` is fixed at [`ShellKind::Unixy`] in this POSIX port
    /// and is readable from safe code (no `unsafe` needed).
    #[test]
    fn shell_kind_is_unixy() {
        assert_eq!(ExecContext::default().shell_kind(), ShellKind::Unixy);
    }
}

#[cfg(test)]
mod pid2str_tests {
    //! `pid2str`'s scratch buffer moved from a `static mut` to
    //! `ctx.pid_string`; these lock in the two contract points a caller
    //! relies on: correct digits, and a fresh call overwriting the same
    //! buffer (the address stability that made the former static safe to
    //! return a pointer into).

    use {
        super::pid2str,
        crate::execctx::{Config, ExecContext},
    };

    #[test]
    fn formats_the_pid_as_decimal() {
        let ctx = ExecContext::new(Config {
            makelevel: 0,
            ..Default::default()
        });
        // SAFETY: single-threaded test; ctx.pidstring is freshly owned.
        let s = unsafe { core::ffi::CStr::from_ptr(pid2str(&ctx, 12345)) };
        assert_eq!(s.to_bytes(), b"12345");
    }

    #[test]
    fn a_later_call_overwrites_the_same_buffer() {
        let ctx = ExecContext::new(Config {
            makelevel: 0,
            ..Default::default()
        });
        // SAFETY: single-threaded test; each pointer is read before the next
        // call, matching every real call site (arg to a printf-family call).
        unsafe {
            let first = pid2str(&ctx, 1);
            assert_eq!(core::ffi::CStr::from_ptr(first).to_bytes(), b"1");
            let second = pid2str(&ctx, 22);
            assert_eq!(
                first, second,
                "same backing buffer, per the former static's contract"
            );
            assert_eq!(core::ffi::CStr::from_ptr(second).to_bytes(), b"22");
        }
    }
}

#[cfg(test)]
mod collapse_dollar_refs_tests {
    use super::collapse_dollar_refs;

    /// Run `collapse_dollar_refs` over `input` (NUL-terminated in a writable
    /// buffer, as `new_job` would) and return the rewritten line as a `String`.
    fn collapse(input: &str) -> String {
        let mut buf: Vec<u8> = input.bytes().chain(std::iter::once(0)).collect();
        unsafe {
            collapse_dollar_refs(buf.as_mut_ptr() as *mut ::core::ffi::c_char);
        }
        let nul = buf.iter().position(|&b| b == 0).unwrap();
        String::from_utf8(buf[..nul].to_vec()).unwrap()
    }

    /// A line with no `$` is copied through unchanged.
    #[test]
    fn plain_line_unchanged() {
        assert_eq!(collapse("cc -c foo.c -o foo.o"), "cc -c foo.c -o foo.o");
    }

    /// A bare `$X` reference (no paren) is left exactly as-is.
    #[test]
    fn bare_dollar_unchanged() {
        assert_eq!(collapse("echo $X done"), "echo $X done");
        assert_eq!(collapse("$@: $<"), "$@: $<");
    }

    /// Inside a `$(...)`/`${...}` reference an unescaped backslash-newline and
    /// the whitespace around it fold to a single space; text outside the
    /// reference is untouched. The whitespace folding consults the global
    /// stopchar map, so initialize it first (as the real program does).
    #[test]
    fn folds_continuation_inside_reference() {
        crate::entry::initialize_stopchar_map();
        assert_eq!(collapse("$(foo \\\n   bar)"), "$(foo bar)");
        assert_eq!(collapse("${a \\\n b} z"), "${a b} z");
    }

    /// A reference with no continuation passes through verbatim.
    #[test]
    fn reference_without_continuation_unchanged() {
        assert_eq!(collapse("$(subst a,b,$(x))"), "$(subst a,b,$(x))");
    }
}

#[cfg(test)]
mod child_error_helper_tests {
    use {
        super::{child_error_label, smode_or_empty, Floc},
        std::ffi::{CStr, CString},
    };

    /// Original c2rust raw-pointer implementation, preserved verbatim as a
    /// differential oracle: passes a non-null `smode` through, maps null to a
    /// static empty C string.
    fn smode_or_empty_unsafe_oracle(
        smode: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char {
        if smode.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            smode
        }
    }

    /// `smode_or_empty` passes a present shuffle string through and maps the
    /// absent case to the empty string.
    #[test]
    fn smode_or_empty_maps_absent_to_empty() {
        let s = CString::new("shuffle=3").unwrap();
        assert_eq!(smode_or_empty(Some(s.as_c_str())), s.as_c_str());

        assert_eq!(smode_or_empty(None).to_bytes(), b"");
    }

    /// The safe `Option<&CStr>` form yields byte-for-byte the same string as the
    /// original raw-pointer implementation, for both the present and absent
    /// cases.
    #[test]
    fn smode_or_empty_matches_unsafe_oracle() {
        for label in ["shuffle=3", "shuffle=reverse", ""] {
            let s = CString::new(label).unwrap();
            let safe = smode_or_empty(Some(s.as_c_str()));
            let oracle = unsafe { CStr::from_ptr(smode_or_empty_unsafe_oracle(s.as_ptr())) };
            assert_eq!(safe.to_bytes(), oracle.to_bytes());
        }

        let safe_none = smode_or_empty(None);
        let oracle_null =
            unsafe { CStr::from_ptr(smode_or_empty_unsafe_oracle(::core::ptr::null())) };
        assert_eq!(safe_none.to_bytes(), oracle_null.to_bytes());
        assert_eq!(safe_none.to_bytes(), b"");
    }

    /// With no source file, the label is the static `<builtin>`.
    #[test]
    fn label_is_builtin_without_source() {
        let fl = Floc {
            filenm: ::core::ptr::null(),
            lineno: 0,
            offset: 0,
        };
        let mut allocations: Vec<Vec<u8>> = Vec::new();
        let label = child_error_label(&fl, &mut allocations);
        let label = unsafe { CStr::from_ptr(label) };
        assert_eq!(label.to_bytes(), b"<builtin>");
        assert!(allocations.is_empty(), "builtin path allocates nothing");
    }

    /// With a source file, the label is `<file>:<lineno+offset>` and lives in an
    /// owned buffer tracked by `allocations`.
    #[test]
    fn label_formats_file_and_line() {
        let name = CString::new("Makefile").unwrap();
        let fl = Floc {
            filenm: name.as_ptr(),
            lineno: 10,
            offset: 2,
        };
        let mut allocations: Vec<Vec<u8>> = Vec::new();
        let label = child_error_label(&fl, &mut allocations);
        let label = unsafe { CStr::from_ptr(label) };
        assert_eq!(label.to_bytes(), b"Makefile:12");
        assert_eq!(allocations.len(), 1, "formatted label owns one buffer");
    }
}

#[cfg(test)]
mod spawn_via_std_tests {
    //! The `std::process::Command` spawn path must hand the child make's argv
    //! and `envp` verbatim (order included) plus the requested stdio routing —
    //! that is what keeps recipe children byte-identical to the C oracle's
    //! `posix_spawn` behavior.

    use {super::spawn_via_std, crate::ffi_types::pid_t, std::ffi::CString};

    /// Build a NULL-terminated `char *[]` from `strings`, returning the owning
    /// `CString`s alongside the raw array.
    fn c_array(strings: &[&str]) -> (Vec<CString>, Vec<*mut ::core::ffi::c_char>) {
        let owned: Vec<CString> = strings.iter().map(|s| CString::new(*s).unwrap()).collect();
        let mut raw: Vec<*mut ::core::ffi::c_char> = owned
            .iter()
            .map(|c| c.as_ptr() as *mut ::core::ffi::c_char)
            .collect();
        raw.push(::core::ptr::null_mut());
        (owned, raw)
    }

    /// Reap `pid` and return its exit status, retrying on EINTR.
    fn wait_status(pid: pid_t) -> i32 {
        let mut status: i32 = 0;
        loop {
            // SAFETY: plain waitpid on a child this test spawned.
            let r = unsafe { libc::waitpid(pid, &mut status, 0) };
            if r == pid {
                return status;
            }
            assert_eq!(
                unsafe { *libc::__errno_location() },
                libc::EINTR,
                "waitpid failed"
            );
        }
    }

    /// Spawn `/usr/bin/env` with a deliberately non-sorted two-variable
    /// environment and stdout routed to a temp file: the child must see the
    /// variables in make's order (a sorted env map would flip them) and the
    /// `fdout` dup2 must route its output.
    #[test]
    fn passes_argv_env_verbatim_and_routes_stdout() {
        let dir = std::env::temp_dir().join(format!(
            "spawn-via-std-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let out_path = dir.join("out.txt");
        let out_c = CString::new(out_path.to_str().unwrap()).unwrap();
        // SAFETY: single-purpose libc/file plumbing on paths this test owns.
        unsafe {
            let fd = libc::open(
                out_c.as_ptr(),
                libc::O_RDWR | libc::O_CREAT | libc::O_TRUNC,
                0o600,
            );
            assert!(fd >= 0, "open temp stdout");
            let (_argv_own, mut argv) = c_array(&["env"]);
            let (_envp_own, mut envp) = c_array(&["Z_LATE=first", "A_EARLY=second"]);
            let file = CString::new("/usr/bin/env").unwrap();
            let mut pid: pid_t = -1;
            let r = spawn_via_std(
                file.as_ptr(),
                argv.as_mut_ptr(),
                envp.as_mut_ptr(),
                -1,
                fd,
                libc::STDERR_FILENO,
                &raw mut pid,
            );
            assert_eq!(r, 0, "spawn failed: errno {r}");
            assert!(pid > 0, "no pid recorded");
            let status = wait_status(pid);
            assert!(libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0);
            libc::close(fd);
        }
        let out = std::fs::read_to_string(&out_path).expect("read child stdout");
        assert_eq!(
            out, "Z_LATE=first\nA_EARLY=second\n",
            "child env must be make's envp verbatim, in order"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// `start_job_command` holds `block_sigs`' fatal-signal mask across the
    /// spawn, but the recipe child must start with an empty mask (the former
    /// `POSIX_SPAWN_SETSIGMASK` contract) — otherwise interrupted builds
    /// leave children ignoring the SIGTERM/SIGINT make passes on (#472
    /// review). Block the fatal signals on this thread, spawn a child that
    /// prints its own blocked-mask line, and require all zeros.
    #[test]
    fn clears_inherited_signal_mask() {
        let dir = std::env::temp_dir().join(format!(
            "spawn-mask-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let out_path = dir.join("mask.txt");
        let out_c = CString::new(out_path.to_str().unwrap()).unwrap();
        // SAFETY: signal-mask bookkeeping on this test thread (restored
        // below) and file plumbing on paths this test owns.
        unsafe {
            let mut blocked: libc::sigset_t = ::core::mem::zeroed();
            libc::sigemptyset(&mut blocked);
            libc::sigaddset(&mut blocked, libc::SIGTERM);
            libc::sigaddset(&mut blocked, libc::SIGINT);
            libc::sigaddset(&mut blocked, libc::SIGCHLD);
            let mut saved: libc::sigset_t = ::core::mem::zeroed();
            assert_eq!(
                libc::pthread_sigmask(libc::SIG_BLOCK, &blocked, &mut saved),
                0
            );

            let fd = libc::open(
                out_c.as_ptr(),
                libc::O_RDWR | libc::O_CREAT | libc::O_TRUNC,
                0o600,
            );
            assert!(fd >= 0, "open temp stdout");
            let (_argv_own, mut argv) = c_array(&["sh", "-c", "grep SigBlk /proc/self/status"]);
            let path_env = format!(
                "PATH={}",
                std::env::var("PATH").unwrap_or_else(|_| "/usr/bin:/bin".to_string())
            );
            let (_envp_own, mut envp) = c_array(&[path_env.as_str()]);
            let file = CString::new("/bin/sh").unwrap();
            let mut pid: pid_t = -1;
            let r = spawn_via_std(
                file.as_ptr(),
                argv.as_mut_ptr(),
                envp.as_mut_ptr(),
                -1,
                fd,
                libc::STDERR_FILENO,
                &raw mut pid,
            );
            let restored =
                libc::pthread_sigmask(libc::SIG_SETMASK, &saved, ::core::ptr::null_mut());
            assert_eq!(r, 0, "spawn failed: errno {r}");
            assert_eq!(restored, 0);
            let status = wait_status(pid);
            assert!(libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0);
            libc::close(fd);
        }
        let out = std::fs::read_to_string(&out_path).expect("read child mask");
        assert_eq!(
            out.trim(),
            "SigBlk:\t0000000000000000",
            "recipe child must start with an empty signal mask"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A missing executable reports its errno to the caller (the parent then
    /// prints `argv[0]: strerror(r)` and the reap path turns it into the
    /// exit-127 handling), and the pid stays unset.
    #[test]
    fn reports_spawn_errno_for_missing_file() {
        let (_argv_own, mut argv) = c_array(&["definitely-not-here"]);
        let (_envp_own, mut envp) = c_array(&[]);
        let file = CString::new("/nonexistent/definitely-not-here").unwrap();
        let mut pid: pid_t = -1;
        // SAFETY: all pointers live for the call; the spawn fails before any
        // child outlives it.
        let r = unsafe {
            spawn_via_std(
                file.as_ptr(),
                argv.as_mut_ptr(),
                envp.as_mut_ptr(),
                -1,
                libc::STDOUT_FILENO,
                libc::STDERR_FILENO,
                &raw mut pid,
            )
        };
        assert_eq!(r, libc::ENOENT, "expected ENOENT from execve");
        assert_eq!(pid, -1, "pid must stay unset on spawn failure");
    }
}

#[cfg(test)]
mod start_waiting_jobs_tests {
    //! `start_waiting_jobs` drains the postponed-job chain, and since #441 it
    //! hands a reap/start failure back as a `Result` instead of exiting. These
    //! cover both of its exits: the empty-chain short circuit, and one pass of
    //! the drain loop whose `start_waiting_job` re-queues the job (the
    //! load-limit path) and so stops the loop.

    use {
        super::{start_waiting_jobs, Child},
        crate::{
            execctx::{Config, ExecContext},
            file::{FileId, FileNode},
            output::output,
        },
        std::sync::atomic::Ordering,
    };

    fn test_ctx() -> ExecContext {
        ExecContext::new(Config {
            makelevel: 0,
            ..Default::default()
        })
    }

    /// Allocate a bare postponed child for `file`, matching the `Box::into_raw`
    /// ownership `new_job` gives every child.
    fn boxed_child(file: FileId) -> *mut Child {
        Box::into_raw(Box::new(Child {
            cmd_name: ::core::ptr::null_mut(),
            environment: ::core::ptr::null_mut(),
            output: output {
                out: 0,
                err: 0,
                syncout: 0,
            },
            next: ::core::ptr::null_mut(),
            file,
            entry: 0,
            sh_batch_file: ::core::ptr::null_mut(),
            command_lines: Vec::new(),
            line_flags: Vec::new(),
            command_line: 0,
            command_buf: Vec::new(),
            command_ptr: ::core::ptr::null_mut(),
            pid: 0,
            remote: 0,
            noerror: 0,
            good_stdin: 0,
            deleted: 0,
            recursive: 0,
            jobslot: 0,
            dontcare: 0,
        }))
    }

    /// With nothing postponed there is no work and no way to fail: the
    /// short circuit reports success rather than entering the drain loop.
    #[test]
    fn empty_chain_is_a_no_op() {
        let ctx = test_ctx();
        // SAFETY: the chain is empty, so no child pointer is dereferenced.
        unsafe { start_waiting_jobs(&ctx) }.expect("empty chain cannot fail");
        assert!(ctx.waiting_jobs.0.get().is_null(), "chain still empty");
    }

    /// One pass of the drain loop: with a job already running and the load
    /// limit pinned at zero, `start_waiting_job` puts the job straight back on
    /// the chain and returns 0, which ends the loop. The reap it does first
    /// finds no children, so the whole pass succeeds — the point being that
    /// success now travels back as `Ok(())` through the same `?` path a
    /// failure would take.
    #[test]
    fn load_limit_requeues_the_job_and_stops_the_loop() {
        let ctx = test_ctx();
        let file = ctx.filenodes.intern(FileNode::new(b"postponed\0".to_vec()));
        let c = boxed_child(file);
        ctx.waiting_jobs.0.set(c);
        // A slot in use plus a zero load ceiling is what makes `load_too_high`
        // fire; without a running job `start_waiting_job` would try to spawn.
        ctx.job_slots_used.0.fetch_add(1, Ordering::Relaxed);
        ctx.options.max_load_average.set(0.0);

        // SAFETY: `c` is a live, fully-initialized child owned by this test and
        // reachable only through the chain we just installed.
        unsafe { start_waiting_jobs(&ctx) }.expect("re-queueing a job is not a failure");

        assert_eq!(
            ctx.waiting_jobs.0.get(),
            c,
            "the job went back on the chain rather than starting"
        );
        // Reclaim it: the re-queue path deliberately does not free the child.
        // SAFETY: nothing else owns `c`, and the chain is dropped with the ctx.
        unsafe { drop(Box::from_raw(c)) };
    }
}

#[cfg(test)]
mod shell_settings_tests {
    //! Since #442 `construct_command_argv` returns `Result`: a `SHELL`,
    //! `.SHELLFLAGS` or `IFS` value that cannot be expanded comes back as a
    //! rejection instead of ending the process while a recipe is being built.
    //! The lookup suppresses the undefined-variable warning for its duration,
    //! so the rejection path has to put that action back too (the
    //! cleanup-paths contract from #561).

    use {
        super::{construct_command_argv, expand_shell_settings, resolve_shellflags},
        crate::{
            build_result::BuildError,
            expand::VARIABLE_BUFFER_TEST_LOCK,
            warning::{self, Action, Type},
        },
        std::ffi::CString,
    };

    /// `$(word 1)` is a builtin called with the wrong number of arguments, so
    /// expanding it is refused.
    const BAD: &str = "$(word 1)";

    /// Define `name` as a recursive global variable holding `value`.
    ///
    /// # Safety
    /// `ctx` must have its global variable set initialized.
    unsafe fn define_recursive(ctx: &crate::execctx::ExecContext, name: &str, value: &str) {
        let cname = CString::new(name).unwrap();
        let cvalue = CString::new(value).unwrap();
        crate::variable::define_variable_in_set(
            ctx,
            cname.as_ptr(),
            name.len() as crate::ffi_types::size_t,
            cvalue.as_ptr(),
            crate::variable::o_file,
            1,
            ctx.variable_globals.global_variable_set.as_ptr(),
            ::core::ptr::null::<crate::floc::Floc>(),
        )
        .expect("test fixture defines a well-formed name");
    }

    fn fresh_ctx() -> crate::execctx::ExecContext {
        crate::entry::initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();
        // SAFETY: fresh context; each table is initialized once.
        unsafe {
            crate::function::hash_init_function_table(&ctx);
            crate::variable::init_hash_global_variable_set(&ctx);
            crate::expand::initialize_variable_output(&ctx);
        }
        ctx
    }

    /// A `SHELL` that cannot be expanded is refused, and the suppression the
    /// lookup installed is lifted before the rejection leaves the frame.
    #[test]
    fn rejected_shell_restores_the_warning_action() {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = fresh_ctx();
        // SAFETY: NUL-terminated names; single-threaded fresh context.
        unsafe {
            define_recursive(&ctx, "SHELL", BAD);
            warning::set_action(&ctx, Type::UndefinedVar, Action::Error);

            assert!(matches!(
                expand_shell_settings(&ctx, None),
                Err(BuildError::Failure)
            ));
            assert_eq!(
                warning::action(&ctx, Type::UndefinedVar),
                Action::Error,
                "the undefined-variable action must be restored on the rejection path"
            );

            let mut line = *b"true\0";
            assert!(matches!(
                construct_command_argv(
                    &ctx,
                    line.as_mut_ptr() as *mut ::core::ffi::c_char,
                    ::core::ptr::null_mut(),
                    None,
                    0,
                    ::core::ptr::null_mut(),
                ),
                Err(BuildError::Failure)
            ));
        }
    }

    /// The same rejection reached through `.SHELLFLAGS` and through `IFS` —
    /// all three expansions are held to the same restore.
    #[test]
    fn rejected_shellflags_and_ifs_also_propagate() {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        for name in [".SHELLFLAGS", "IFS"] {
            let ctx = fresh_ctx();
            // SAFETY: as above.
            unsafe {
                define_recursive(&ctx, name, BAD);
                assert!(
                    matches!(expand_shell_settings(&ctx, None), Err(BuildError::Failure)),
                    "{name} must propagate its rejection"
                );
            }
        }
    }

    /// Well-formed values still expand, and the warning action is untouched.
    #[test]
    fn well_formed_settings_still_expand() {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = fresh_ctx();
        // SAFETY: as above.
        unsafe {
            define_recursive(&ctx, "SHELL", "/bin/sh");
            define_recursive(&ctx, "IFS", " ");
            warning::set_action(&ctx, Type::UndefinedVar, Action::Error);
            let (shell, _flags, ifs) = expand_shell_settings(&ctx, None).expect("well-formed");
            assert_eq!(shell, b"/bin/sh\0");
            assert_eq!(ifs, b" \0");
            assert_eq!(warning::action(&ctx, Type::UndefinedVar), Action::Error);
        }
    }

    /// `.SHELLFLAGS` chooses between the expanded value and the two defaults;
    /// a value that did not arrive NUL-terminated is terminated in place.
    #[test]
    fn shellflags_fall_back_when_unset() {
        let ctx = fresh_ctx();
        assert_eq!(resolve_shellflags(&ctx, b"-x\0", 0), b"-x\0");
        assert_eq!(resolve_shellflags(&ctx, b"-x", 0), b"-x\0");
        // Unset (empty, or expanded to the empty string) takes the default; the
        // posix-pedantic `-ec` arm needs `.POSIX:`, which this context lacks.
        assert_eq!(resolve_shellflags(&ctx, b"", 0), b"-c\0");
        assert_eq!(resolve_shellflags(&ctx, b"\0", 0), b"-c\0");
    }
}
