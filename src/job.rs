pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __pid_t, __sig_atomic_t, __syscall_slong_t, __time_t, __uid_t, pid_t, sig_atomic_t, size_t,
    ssize_t, time_t, uintmax_t,
};
use crate::file::{dep, file, Commands, Dep, File, VariableSet, VariableSetList};
use crate::file::{
    cs_finished, cs_not_started, cs_running, us_failed, us_question, us_success, CommandState,
    UpdateStatus,
};
use crate::misc::{xcalloc, xmalloc, xstrdup};
use crate::stdio::FILE;
use ::c2rust_bitfields;
use libc::{
    __errno_location, close, free, getenv, getloadavg, open, printf, remove, sprintf, stpcpy,
    strchr, strcmp, strerror, strsignal,
};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
extern "C" {
    pub type __spawn_action;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> i32;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn read(__fd: i32, __buf: *mut ::core::ffi::c_void, __nbytes: size_t) -> ssize_t;
    static mut environ: *mut *mut ::core::ffi::c_char;
    fn execvp(__file: *const ::core::ffi::c_char, __argv: *const *mut ::core::ffi::c_char) -> i32;
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
    fn wait(__stat_loc: *mut i32) -> __pid_t;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut i32, __options: i32) -> __pid_t;
    fn posix_spawn(
        __pid: *mut pid_t,
        __path: *const ::core::ffi::c_char,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut ::core::ffi::c_char,
        __envp: *const *mut ::core::ffi::c_char,
    ) -> i32;
    fn posix_spawnattr_init(__attr: *mut posix_spawnattr_t) -> i32;
    fn posix_spawnattr_destroy(__attr: *mut posix_spawnattr_t) -> i32;
    fn posix_spawnattr_setsigmask(
        __attr: *mut posix_spawnattr_t,
        __sigmask: *const sigset_t,
    ) -> i32;
    fn posix_spawnattr_setflags(
        _attr: *mut posix_spawnattr_t,
        __flags: ::core::ffi::c_short,
    ) -> i32;
    fn posix_spawn_file_actions_init(__file_actions: *mut posix_spawn_file_actions_t) -> i32;
    fn posix_spawn_file_actions_destroy(__file_actions: *mut posix_spawn_file_actions_t) -> i32;
    fn posix_spawn_file_actions_adddup2(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: i32,
        __newfd: i32,
    ) -> i32;
}
pub use crate::make_main::__sigset_t;
pub type sigset_t = __sigset_t;
pub use crate::sys_stat::stat;
pub use crate::sys_stat::timespec;
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
pub type hash_table = crate::hash::hash_table;
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
pub struct childbase {
    pub cmd_name: *mut ::core::ffi::c_char,
    pub environment: *mut *mut ::core::ffi::c_char,
    pub output: output,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct child {
    pub cmd_name: *mut ::core::ffi::c_char,
    pub environment: *mut *mut ::core::ffi::c_char,
    pub output: output,
    pub next: *mut child,
    pub file: *mut File,
    pub sh_batch_file: *mut ::core::ffi::c_char,
    pub command_lines: *mut *mut ::core::ffi::c_char,
    pub command_ptr: *mut ::core::ffi::c_char,
    pub command_line: ::core::ffi::c_uint,
    pub pid: pid_t,
    #[bitfield(name = "remote", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "noerror", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "good_stdin", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "deleted", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "recursive", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "jobslot", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "dontcare", ty = "::core::ffi::c_uint", bits = "6..=6")]
    pub remote_noerror_good_stdin_deleted_recursive_jobslot_dontcare: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
impl crate::file::NextLinked for child {
    unsafe fn next(this: *const Self) -> *mut Self {
        if this.is_null() {
            return ::core::ptr::null_mut::<Self>();
        }
        (*this).next
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawnattr_t {
    pub __flags: ::core::ffi::c_short,
    pub __pgrp: pid_t,
    pub __sd: sigset_t,
    pub __ss: sigset_t,
    pub __sp: sched_param,
    pub __policy: i32,
    pub __cgroup: i32,
    pub __pad: [i32; 15],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: i32,
}
#[allow(non_camel_case_types)]
pub type __spawn_action = ::core::ffi::c_void;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawn_file_actions_t {
    pub __allocated: i32,
    pub __used: i32,
    pub __actions: *mut __spawn_action,
    pub __pad: [i32; 16],
}
use crate::commands::{chop_commands, delete_child_targets, handling_fatal_signal};
use crate::expand::{allocated_expand_string_for_file, allocated_expand_variable_for_file};
use crate::file::{lookup_file, set_command_state};
use crate::findprog::find_in_given_path;
use crate::function::{shell_completed, shell_function_pid};
use crate::make_main::{
    db_level, die, fatal_signal_set, not_parallel, one_shell, posix_pedantic, stopchar_map,
};
use crate::output::{
    error, fatal, message, output_context, perror_with_name, pfatal_with_name, FmtArg,
};
use crate::posixos::{
    fd_noinherit, get_bad_stdin, jobserver_acquire, jobserver_enabled, jobserver_post_child,
    jobserver_pre_acquire, jobserver_pre_child, jobserver_release, jobserver_signal,
};
use crate::remake::{notice_finished_file, show_goal_error};
use crate::variable::{lookup_variable_for_file, target_environment};
use crate::warning::{self, Action, Type};
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
pub static mut default_shell: *const ::core::ffi::c_char =
    b"/bin/sh\0" as *const u8 as *const ::core::ffi::c_char;
/// Batch-mode shell is a W32/DOS feature: the only writers in the C original
/// are platform-specific, so the value is fixed at 0 in this POSIX port.
/// Keeping it an immutable `static` lets the read sites access it from safe
/// code.
pub static batch_mode_shell: i32 = 0;
pub const S_IXUSR: i32 = __S_IEXEC;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const POSIX_SPAWN_SETSIGMASK: i32 = 0x8_i32;
pub const POSIX_SPAWN_USEVFORK: i32 = 0x40_i32;
pub const COMMANDS_RECURSE: i32 = 1;
pub const COMMANDS_SILENT: i32 = 2;
pub const NONEXISTENT_MTIME: i32 = 1;
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn pid2str(pid: pid_t) -> *const ::core::ffi::c_char {
    static mut pidstring: [::core::ffi::c_char; 100] = [0; 100];
    sprintf(
        &raw mut pidstring as *mut ::core::ffi::c_char,
        b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
        pid as ::core::ffi::c_ulong,
    );
    &raw mut pidstring as *mut ::core::ffi::c_char
}
pub static mut children: *mut child = ::core::ptr::null::<child>() as *mut child;
/// Count of job slots currently in use. Stored in an atomic so its reads are
/// plain safe operations; all access is single-threaded, so `Relaxed`
/// preserves the original program order.
static JOB_SLOTS_USED: AtomicU32 = AtomicU32::new(0);

/// Number of job slots currently in use.
pub fn job_slots_used() -> ::core::ffi::c_uint {
    JOB_SLOTS_USED.load(Ordering::Relaxed)
}
static mut waiting_jobs: *mut child = ::core::ptr::null::<child>() as *mut child;
/// The shell is always "unixy" in this POSIX port: the only writers in the C
/// original are W32/DOS-specific, so the value is fixed at 1 here. Keeping it
/// an immutable `static` lets the read sites access it from safe code.
pub static unixy_shell: i32 = 1;
/// Number of jobs started since the load average was last sampled; used by
/// `load_too_high` to estimate the incremental load each new job adds. Atomic
/// so its reads/writes are plain safe ops; job bookkeeping is single-threaded,
/// so `Relaxed` preserves the original program order.
static JOB_COUNTER: AtomicU64 = AtomicU64::new(0);
/// Count of jobserver tokens this make instance currently holds (the implicit
/// token for its own slot plus one per running child). Stored in an atomic so
/// its reads are plain safe operations; all access is single-threaded, so
/// `Relaxed` preserves the original program order. `pub` because `main`'s
/// `clean_jobserver` drains it on exit.
pub static JOBSERVER_TOKENS: AtomicU32 = AtomicU32::new(0);

/// Number of jobserver tokens currently held.
pub fn jobserver_tokens() -> ::core::ffi::c_uint {
    JOBSERVER_TOKENS.load(Ordering::Relaxed)
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
    use ::std::os::unix::ffi::OsStrExt;
    ::std::path::Path::new(::std::ffi::OsStr::from_bytes(
        ::core::ffi::CStr::from_ptr(ptr).to_bytes(),
    ))
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn block_sigs() {
    sigprocmask(
        SIG_BLOCK,
        &raw mut fatal_signal_set,
        ::core::ptr::null_mut::<sigset_t>(),
    );
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn unblock_sigs() {
    sigprocmask(
        SIG_UNBLOCK,
        &raw mut fatal_signal_set,
        ::core::ptr::null_mut::<sigset_t>(),
    );
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn unblock_all_sigs() {
    let mut empty: sigset_t = __sigset_t { __val: [0; 16] };
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
/// is active, otherwise an empty string (`smode` null).
fn smode_or_empty(smode: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    if smode.is_null() {
        b"\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        smode
    }
}

unsafe fn child_error(
    ctx: &crate::execctx::ExecContext,
    child: *mut child,
    exit_code: i32,
    exit_sig: i32,
    coredump: i32,
    ignored: i32,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut pre: *const ::core::ffi::c_char = b"*** \0" as *const u8 as *const ::core::ffi::c_char;
    let mut post: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let mut dump: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let f: *const file = (*child).file;
    let flocp: *const Floc = &raw const f
        .as_ref()
        .expect("a child always has a file")
        .cmds
        .as_ref()
        .expect("a child being reported has a recipe")
        .fileinfo;
    let mut smode: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut l: size_t;
    if ignored != 0 && crate::make_main::opt_run_silent() {
        return;
    }
    if exit_sig != 0 && coredump != 0 {
        dump = b" (core dumped)\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if ignored != 0 {
        pre = b"\0" as *const u8 as *const ::core::ffi::c_char;
        post = b" (ignored)\0" as *const u8 as *const ::core::ffi::c_char;
    }
    let nm = child_error_label(&*flocp, &mut alloca_allocations);
    l = strlen(pre)
        .wrapping_add(strlen(nm))
        .wrapping_add(strlen((*f).name))
        .wrapping_add(strlen(post)) as size_t;
    if let Some(label) = crate::shuffle::get_mode() {
        let mut buf = format!(" shuffle={}", label).into_bytes();
        buf.push(0);
        alloca_allocations.push(buf);
        smode = alloca_allocations.last().unwrap().as_ptr() as *const ::core::ffi::c_char;
    }
    output_context = if (*child).output.syncout() as i32 != 0 {
        &raw mut (*child).output
    } else {
        ::core::ptr::null_mut::<output>()
    };
    show_goal_error(ctx);
    if exit_sig == 0 {
        error(
        ctx,
        NILF,
        l.wrapping_add(INTSTR_LENGTH),
        b"%s[%s: %s] Error %d%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((pre) as *const ::core::ffi::c_char),
            FmtArg::Str((nm) as *const ::core::ffi::c_char),
            FmtArg::Str(((*f).name) as *const ::core::ffi::c_char),
            FmtArg::Int((exit_code) as i32 as i64),
            FmtArg::Str((post) as *const ::core::ffi::c_char),
            FmtArg::Str((smode_or_empty(smode)) as *const ::core::ffi::c_char)],
    );
    } else {
        let s: *const ::core::ffi::c_char = strsignal(exit_sig);
        error(
        ctx,
        NILF,
        l.wrapping_add(strlen(s) as size_t)
                .wrapping_add(strlen(dump) as size_t),
        b"%s[%s: %s] %s%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((pre) as *const ::core::ffi::c_char),
            FmtArg::Str((nm) as *const ::core::ffi::c_char),
            FmtArg::Str(((*f).name) as *const ::core::ffi::c_char),
            FmtArg::Str((s) as *const ::core::ffi::c_char),
            FmtArg::Str((dump) as *const ::core::ffi::c_char),
            FmtArg::Str((post) as *const ::core::ffi::c_char),
            FmtArg::Str((smode_or_empty(smode)) as *const ::core::ffi::c_char)],
    );
    }
    output_context = ::core::ptr::null_mut::<output>();
}
/// Count of children reaped by the `SIGCHLD` handler and not yet processed
/// by the reap loop. Written from the signal handler and read on the main
/// path, so it must be an atomic rather than a `static mut` (a plain global
/// would be a data race); `Relaxed` suffices as it only gates a retry.
static DEAD_CHILDREN: AtomicU32 = AtomicU32::new(0);

fn dead_children() -> ::core::ffi::c_uint {
    DEAD_CHILDREN.load(Ordering::Relaxed)
}
/// `SIGCHLD` handler: record a reaped child and wake any blocked jobserver
/// acquire. Async-signal-safe (an atomic increment plus `jobserver_signal`'s
/// `close`).
pub extern "C" fn child_handler(mut _sig: i32) {
    DEAD_CHILDREN.fetch_add(1, Ordering::Relaxed);
    jobserver_signal();
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn reap_children(ctx: &crate::execctx::ExecContext, mut block: i32, err: i32) {
    let mut status: i32 = 0;
    let mut reap_more: i32 = 1;
    while (!children.is_null() || shell_function_pid() != 0) && (block != 0 || reap_more != 0) {
        let mut remote: ::core::ffi::c_uint = 0;
        let mut pid: pid_t;
        let mut exit_code: i32 = 0;
        let mut exit_sig: i32 = 0;
        let mut coredump: i32 = 0;
        let mut lastc: *mut child;
        let mut c: *mut child;
        let mut child_failed: i32;
        let mut any_remote: i32;
        let mut any_local: i32;
        let dontcare: i32;
        if err != 0 && block != 0 {
            // Guards the one-time "Waiting for unfinished jobs" notice. Atomic
            // so the read/write are plain safe ops; reaping is single-threaded,
            // so `Relaxed` preserves the original program order.
            static PRINTED: AtomicBool = AtomicBool::new(false);
            fflush(stdout);
            if !PRINTED.load(Ordering::Relaxed) {
                error(
                    ctx,
                    ::core::ptr::null_mut::<Floc>(),
                    0,
                    b"*** Waiting for unfinished jobs....\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            PRINTED.store(true, Ordering::Relaxed);
        }
        if dead_children() > 0 {
            DEAD_CHILDREN.fetch_sub(1, Ordering::Relaxed);
        }
        any_remote = 0;
        any_local = (shell_function_pid() != 0) as i32;
        lastc = ::core::ptr::null_mut::<child>();
        c = children;
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
                if 0x4_i32 & db_level != 0 {
                    printf(
                        b"Live child %p (%s) PID %s %s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        c,
                        (*c).file.as_ref().expect("a child always has a file").name,
                        pid2str((*c).pid),
                        if (*c).remote() as i32 != 0 {
                            b" (remote)\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                    );
                    fflush(stdout);
                }
                lastc = c;
                c = (*c).next;
            }
        }
        if found_bad == 0 {
            if any_remote != 0 {
                pid = crate::remote_stub::remote_status(
                    &raw mut exit_code,
                    &raw mut exit_sig,
                    &raw mut coredump,
                    0,
                ) as pid_t;
            } else {
                pid = 0_i32 as pid_t;
            }
            if pid > 0 {
                remote = 1;
            } else if pid < 0 {
                pfatal_with_name(
                    ctx,
                    b"remote_status\0" as *const u8 as *const ::core::ffi::c_char,
                );
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
                    pfatal_with_name(ctx, b"wait\0" as *const u8 as *const ::core::ffi::c_char);
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
                    pid = crate::remote_stub::remote_status(
                        &raw mut exit_code,
                        &raw mut exit_sig,
                        &raw mut coredump,
                        1,
                    ) as pid_t;
                    if pid < 0 {
                        pfatal_with_name(
                            ctx,
                            b"remote_status\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    if pid == 0 {
                        break;
                    }
                    remote = 1;
                }
            }
            crate::make_main::bump_command_count();
            if remote == 0 && pid == shell_function_pid() {
                shell_completed(ctx, exit_code, exit_sig);
                break;
            } else {
                lastc = ::core::ptr::null_mut::<child>();
                c = children;
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
                if 0x4_i32 & db_level != 0 {
                    printf(
                        if exit_sig == 0 && exit_code == 0 {
                            b"Reaping winning child %p PID %s %s\n\0" as *const u8
                                as *const ::core::ffi::c_char
                        } else {
                            b"Reaping losing child %p PID %s %s\n\0" as *const u8
                                as *const ::core::ffi::c_char
                        },
                        c,
                        pid2str((*c).pid),
                        if (*c).remote() as i32 != 0 {
                            b" (remote)\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                    );
                    fflush(stdout);
                }
                if JOB_COUNTER.load(Ordering::Relaxed) != 0 {
                    JOB_COUNTER.fetch_sub(1, Ordering::Relaxed);
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
            } else if st.st_size == 0 as __off_t {
                e = strerror(ENOEXEC);
            }
            if !e.is_null() {
                error(
        ctx,
        ::core::ptr::null_mut::<Floc>(),
        (strlen((*c).cmd_name) as size_t).wrapping_add(strlen(e) as size_t),
        b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str(((*c).cmd_name) as *const ::core::ffi::c_char),
            FmtArg::Str((e) as *const ::core::ffi::c_char)],
    );
            }
        }
        if exit_sig == 0 && exit_code == 0 {
            child_failed = MAKE_SUCCESS;
        } else if exit_sig == 0
            && exit_code == 1
            && crate::make_main::opt_question()
            && (*c).recursive() as i32 != 0
        {
            child_failed = MAKE_TROUBLE;
        } else {
            child_failed = MAKE_FAILURE;
        }
        if !(*c).sh_batch_file.is_null() {
            let rm_status: i32;
            if 0x4_i32 & db_level != 0 {
                printf(
                    b"Cleaning up temp batch file %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*c).sh_batch_file,
                );
                fflush(stdout);
            }
            *__errno_location() = 0;
            rm_status = remove((*c).sh_batch_file);
            if rm_status != 0 && 0x4_i32 & db_level != 0 {
                printf(
                    b"Cleaning up temp batch file %s failed (%d)\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*c).sh_batch_file,
                    *__errno_location(),
                );
                fflush(stdout);
            }
            free((*c).sh_batch_file as *mut ::core::ffi::c_void);
            (*c).sh_batch_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if (*c).good_stdin() != 0 {
            ctx.good_stdin_used.set(false);
        }
        dontcare = (*c).dontcare() as i32;
        if child_failed != 0 && (*c).noerror() == 0 && !crate::make_main::opt_ignore_errors() {
            // Caches whether `.DELETE_ON_ERROR` is a target: -1 = not yet
            // computed, 0/1 = the looked-up answer. Atomic so the read/write are
            // plain safe ops; access is single-threaded (children are reaped on
            // the main thread), so `Relaxed` preserves the original order.
            static DELETE_ON_ERROR: AtomicI32 = AtomicI32::new(-1);
            if dontcare == 0 && child_failed == MAKE_FAILURE {
                child_error(ctx, c, exit_code, exit_sig, coredump, 0);
            }
            (*c).file
                .as_mut()
                .expect("a child always has a file")
                .set_update_status(
                    (if child_failed == MAKE_FAILURE {
                        us_failed as i32
                    } else {
                        us_question as i32
                    }),
                );
            if DELETE_ON_ERROR.load(Ordering::Relaxed) == -1_i32 {
                let f: *mut file =
                    lookup_file(b".DELETE_ON_ERROR\0" as *const u8 as *const ::core::ffi::c_char);
                DELETE_ON_ERROR.store(
                    (!f.is_null() && (*f).is_target() as i32 != 0) as i32,
                    Ordering::Relaxed,
                );
            }
            if exit_sig != 0 || DELETE_ON_ERROR.load(Ordering::Relaxed) != 0 {
                delete_child_targets(ctx, c);
            }
        } else {
            if child_failed != 0 {
                child_error(ctx, c, exit_code, exit_sig, coredump, 1);
                child_failed = 0;
            }
            if job_next_command(c) != 0 {
                if handling_fatal_signal != 0 {
                    (*c).file
                        .as_mut()
                        .expect("a child always has a file")
                        .set_update_status(us_failed);
                } else {
                    if crate::make_main::opt_output_sync() == OUTPUT_SYNC_LINE {
                        crate::output::output_dump(ctx, &raw mut (*c).output);
                    }
                    (*c).set_remote(
                        crate::remote_stub::start_remote_job_p(0) as ::core::ffi::c_uint
                            as ::core::ffi::c_uint,
                    );
                    start_job_command(ctx, c);
                    unblock_sigs();
                    if (*c)
                        .file
                        .as_ref()
                        .expect("a child always has a file")
                        .command_state() as i32
                        == cs_running as i32
                    {
                        continue;
                    }
                }
                if (*c)
                    .file
                    .as_ref()
                    .expect("a child always has a file")
                    .update_status() as i32
                    != us_success as i32
                {
                    delete_child_targets(ctx, c);
                }
            } else {
                (*c).file
                    .as_mut()
                    .expect("a child always has a file")
                    .set_update_status(us_success);
            }
        }
        crate::output::output_dump(ctx, &raw mut (*c).output);
        if handling_fatal_signal == 0 {
            notice_finished_file(ctx, (*c).file);
        }
        block_sigs();
        if (*c).pid > 0 && 0x4_i32 & db_level != 0 {
            printf(
                b"Removing child %p PID %s%s from chain.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                c,
                pid2str((*c).pid),
                if (*c).remote() as i32 != 0 {
                    b" (remote)\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            fflush(stdout);
        }
        if job_slots_used() > 0 {
            JOB_SLOTS_USED.store(
                job_slots_used().wrapping_sub((*c).jobslot()),
                Ordering::Relaxed,
            );
        }
        if let Some(lastcr) = lastc.as_mut() {
            lastcr.next = (*c).next;
        } else {
            children = (*c).next;
        }
        free_child(ctx, c);
        unblock_sigs();
        if err == 0
            && child_failed != 0
            && dontcare == 0
            && !crate::make_main::opt_keep_going()
            && handling_fatal_signal == 0
        {
            die(ctx, child_failed);
        }
        block = 0;
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn free_childbase(child: *mut childbase) {
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
pub unsafe fn free_child(ctx: &crate::execctx::ExecContext, child: *mut child) {
    crate::output::output_close(ctx, &raw mut (*child).output);
    release_jobserver_token(ctx, child);
    if handling_fatal_signal != 0 {
        return;
    }
    free_command_lines(child);
    free_childbase(child as *mut childbase);
    free(child as *mut ::core::ffi::c_void);
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
unsafe fn release_jobserver_token(ctx: &crate::execctx::ExecContext, child: *mut child) {
    if jobserver_tokens() == 0 {
        fatal(
        ctx,
        ::core::ptr::null_mut::<Floc>(),
        INTSTR_LENGTH.wrapping_add(strlen(
                (*child)
                    .file
                    .as_ref()
                    .expect("a child always has a file")
                    .name,
            ) as size_t),
        b"INTERNAL: freeing child %p (%s) but no tokens left\0" as *const u8
                as *const ::core::ffi::c_char,
        &[FmtArg::Ptr((child) as *const ::core::ffi::c_void),
            FmtArg::Str(((*child)
                .file
                .as_ref()
                .expect("a child always has a file")
                .name) as *const ::core::ffi::c_char)],
    );
    }
    if jobserver_enabled() != 0 && jobserver_tokens() > 1 {
        jobserver_release(ctx, 1);
        if 0x4_i32 & db_level != 0 {
            printf(
                b"Released token for child %p (%s).\n\0" as *const u8 as *const ::core::ffi::c_char,
                child,
                (*child)
                    .file
                    .as_ref()
                    .expect("a child always has a file")
                    .name,
            );
            fflush(stdout);
        }
    }
    JOBSERVER_TOKENS.fetch_sub(1, Ordering::Relaxed);
}

/// Free a child's expanded per-line recipe argv — each line buffer and then the
/// array itself — if one was built. Split out of `free_child` to keep that
/// function's complexity flat.
///
/// # Safety
///
/// `child` must be a valid `child` whose `command_lines`, when non-null, holds
/// `ncommand_lines` heap-allocated line pointers.
unsafe fn free_command_lines(child: *mut child) {
    if (*child).command_lines.is_null() {
        return;
    }
    let mut i: ::core::ffi::c_uint = 0;
    while i
        < (*child)
            .file
            .as_ref()
            .expect("a child always has a file")
            .cmds
            .as_ref()
            .expect("a child being run has a recipe")
            .ncommand_lines as ::core::ffi::c_uint
    {
        free(*(*child).command_lines.offset(i as isize) as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    free((*child).command_lines as *mut ::core::ffi::c_void);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn start_job_command(ctx: &crate::execctx::ExecContext, child: *mut child) {
    let mut flags: i32;
    let mut p: *mut ::core::ffi::c_char;
    let mut argv: *mut *mut ::core::ffi::c_char;
    if !(*child).command_ptr.is_null() {
        flags = (*child)
            .file
            .as_ref()
            .expect("a child always has a file")
            .command_flags
            | *(*child)
                .file
                .as_ref()
                .expect("a child always has a file")
                .cmds
                .as_ref()
                .expect("a child being run has a recipe")
                .lines_flags
                .offset((*child).command_line.wrapping_sub(1) as isize) as i32;
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
        let fresh10 = &mut (*(*child)
            .file
            .as_ref()
            .expect("a child always has a file")
            .cmds
            .as_ref()
            .expect("a child being run has a recipe")
            .lines_flags
            .offset((*child).command_line.wrapping_sub(1) as isize));
        *fresh10 = (*fresh10 as i32 | flags & COMMANDS_RECURSE) as ::core::ffi::c_uchar;
        let prefix: ::core::ffi::c_char = (*child)
            .file
            .as_ref()
            .expect("a child always has a file")
            .cmds
            .as_ref()
            .expect("a child being run has a recipe")
            .recipe_prefix;
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
        argv = construct_command_argv(
            ctx,
            p,
            &raw mut end,
            (*child).file,
            *(*child)
                .file
                .as_ref()
                .expect("a child always has a file")
                .cmds
                .as_ref()
                .expect("a child being run has a recipe")
                .lines_flags
                .offset((*child).command_line.wrapping_sub(1) as isize) as i32
                | (*child)
                    .file
                    .as_ref()
                    .expect("a child always has a file")
                    .command_flags,
            &raw mut (*child).sh_batch_file,
        );
        if end.is_null() {
            (*child).command_ptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            let end_ref = end
                .as_mut()
                .expect("construct_command_argv returned an invalid end pointer");
            *end_ref = 0;
            (*child).command_ptr = end.add(1);
        }
        if !argv.is_null() && crate::make_main::opt_question() && !(flags & 1 != 0) {
            if !argv.is_null() {
                free(*argv.offset(0_i32 as isize) as *mut ::core::ffi::c_void);
                free(argv as *mut ::core::ffi::c_void);
            }
            (*child)
                .file
                .as_mut()
                .expect("a child always has a file")
                .set_update_status(us_question);
            notice_finished_file(ctx, (*child).file);
            return;
        }
        if crate::make_main::opt_touch() && !(flags & 1 != 0) {
            if !argv.is_null() {
                free(*argv.offset(0_i32 as isize) as *mut ::core::ffi::c_void);
                free(argv as *mut ::core::ffi::c_void);
            }
            argv = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        }
        if !argv.is_null() {
            let os = crate::make_main::opt_output_sync();
            (*child).output.set_syncout(
                (os != 0 && (os == OUTPUT_SYNC_RECURSE || !(flags & 1 != 0)))
                    as i32 as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
            output_context = if (*child).output.syncout() as i32 != 0 {
                &raw mut (*child).output
            } else {
                ::core::ptr::null_mut::<output>()
            };
            if (*child).output.syncout() == 0 {
                crate::output::output_dump(ctx, &raw mut (*child).output);
            }
            if crate::make_main::opt_just_print()
                || 0x10_i32 & db_level != 0
                || !(flags & 2 != 0) && !crate::make_main::opt_run_silent()
            {
                message(
        ctx,
        0,
        strlen(p) as size_t,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((p) as *const ::core::ffi::c_char)],
    );
            }
            ctx.commands_started.set(ctx.commands_started.get().wrapping_add(1));
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
                || (crate::make_main::opt_just_print() && !(flags & 1 != 0))
            {
                if !argv.is_null() {
                    free(*argv.offset(0_i32 as isize) as *mut ::core::ffi::c_void);
                    free(argv as *mut ::core::ffi::c_void);
                }
            } else {
                crate::output::output_start(ctx);
                fflush(stdout);
                fflush(stderr);
                (*child).set_good_stdin(
                    !ctx.good_stdin_used.get() as i32 as ::core::ffi::c_uint as ::core::ffi::c_uint
                );
                if (*child).good_stdin() != 0 {
                    ctx.good_stdin_used.set(true);
                }
                (*child).set_deleted(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                if (*child).environment.is_null() {
                    (*child).environment = target_environment(
                        ctx,
                        (*child).file,
                        (*child)
                            .file
                            .as_ref()
                            .expect("a child always has a file")
                            .cmds
                            .as_ref()
                            .expect("a child being run has a recipe")
                            .any_recurse() as i32,
                    );
                }
                // Run the job locally unless it is successfully handed off to a
                // remote executor.
                let mut run_local = true;
                if (*child).remote() != 0 {
                    let mut is_remote: i32 = 0;
                    let mut used_stdin: i32 = 0;
                    let mut id: pid_t = 0;
                    if crate::remote_stub::start_remote_job(
                        argv,
                        (*child).environment,
                        if (*child).good_stdin() as i32 != 0 {
                            0
                        } else {
                            get_bad_stdin()
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
                    block_sigs();
                    (*child).set_remote(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    jobserver_pre_child((flags & 1 != 0) as i32);
                    (*child).pid = child_execute_job(
                        ctx,
                        child as *mut childbase,
                        (*child).good_stdin() as i32,
                        argv,
                    );
                    jobserver_post_child((flags & 1 != 0) as i32);
                }
                if (*child).pid >= 0 {
                    JOB_COUNTER.fetch_add(1, Ordering::Relaxed);
                }
                set_command_state((*child).file, CommandState::Running);
                if !argv.is_null() {
                    free(*argv.offset(0_i32 as isize) as *mut ::core::ffi::c_void);
                    free(argv as *mut ::core::ffi::c_void);
                }
                output_context = ::core::ptr::null_mut::<output>();
                return;
            }
        }
    }
    if job_next_command(child) != 0 {
        start_job_command(ctx, child);
    } else {
        set_command_state((*child).file, cs_running);
        (*child)
            .file
            .as_mut()
            .expect("a child always has a file")
            .set_update_status(us_success);
        notice_finished_file(ctx, (*child).file);
    }
    output_context = ::core::ptr::null_mut::<output>();
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn start_waiting_job(ctx: &crate::execctx::ExecContext, c: *mut child) -> i32 {
    let f: *mut file = (*c).file;
    (*c).set_remote(
        crate::remote_stub::start_remote_job_p(1) as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
    if (*c).remote() == 0 && (job_slots_used() > 0 && load_too_high(ctx) != 0) {
        set_command_state(f, cs_running);
        (*c).next = waiting_jobs;
        waiting_jobs = c;
        return 0;
    }
    start_job_command(ctx, c);
    // Finished states (cs_not_started reset to success, cs_finished) need the
    // file noticed and the child freed; a still-running job does not.
    let mut finish = false;
    match (*f).command_state() as i32 {
        2 => {
            (*c).next = children;
            if (*c).pid > 0 {
                if 0x4_i32 & db_level != 0 {
                    printf(
                        b"Putting child %p (%s) PID %s%s on the chain.\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        c,
                        (*c).file.as_ref().expect("a child always has a file").name,
                        pid2str((*c).pid),
                        if (*c).remote() as i32 != 0 {
                            b" (remote)\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                    );
                    fflush(stdout);
                }
                JOB_SLOTS_USED.fetch_add(1, Ordering::Relaxed);
                if (*c).jobslot() as i32 == 0 {
                } else {
                    panic!("assertion failed: c->jobslot == 0");
                };
                (*c).set_jobslot(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            children = c;
            unblock_sigs();
        }
        0 => {
            (*f).update_status = UpdateStatus::Success;
            finish = true;
        }
        3 => {
            finish = true;
        }
        _ => {
            if (*f).command_state() as i32 == cs_finished as i32 {
            } else {
                panic!("assertion failed: f->command_state == cs_finished");
            };
        }
    }
    if finish {
        notice_finished_file(ctx, f);
        free_child(ctx, c);
    }
    1
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
pub unsafe fn new_job(ctx: &crate::execctx::ExecContext, file: *mut file) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let cmds: *mut Commands = (*file).cmds;
    let c: *mut child;
    let lines: *mut *mut ::core::ffi::c_char;
    let mut i: ::core::ffi::c_uint;
    start_waiting_jobs(ctx);
    reap_children(ctx, 0, 0);
    chop_commands(ctx, cmds);
    c = xcalloc(::core::mem::size_of::<child>() as size_t) as *mut child;
    crate::output::output_init(&raw mut (*c).output);
    (*c).file = file;
    (*c).sh_batch_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*c).set_dontcare((*file).dontcare() as ::core::ffi::c_uint);
    output_context = if (*c).output.syncout() as i32 != 0 {
        &raw mut (*c).output
    } else {
        ::core::ptr::null_mut::<output>()
    };
    lines = xmalloc(
        ((*cmds).ncommand_lines as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    i = 0;
    while i < (*cmds).ncommand_lines as ::core::ffi::c_uint {
        collapse_dollar_refs(*(*cmds).command_lines.offset(i as isize));
        (*cmds).fileinfo.offset = i as ::core::ffi::c_ulong;
        let fresh7 = &mut (*lines.offset(i as isize));
        *fresh7 =
            allocated_expand_string_for_file(ctx, *(*cmds).command_lines.offset(i as isize), file);
        i = i.wrapping_add(1);
    }
    (*cmds).fileinfo.offset = 0;
    (*c).command_lines = lines;
    job_next_command(c);
    // `job_slots` is fixed for the run (set only during `main_0` job setup), so
    // snapshot it once rather than reading the borrow channel each spin.
    let slots = crate::make_main::opt_job_slots();
    if slots != 0 {
        while job_slots_used() == slots {
            reap_children(ctx, 1, 0);
        }
    } else if jobserver_enabled() != 0 {
        loop {
            let got_token: i32;
            if 0x4_i32 & db_level != 0 {
                printf(
                    b"Need a job token; we %shave children\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    if !children.is_null() {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"don't \0" as *const u8 as *const ::core::ffi::c_char
                    },
                );
                fflush(stdout);
            }
            if jobserver_tokens() == 0 {
                break;
            }
            jobserver_pre_acquire(ctx);
            reap_children(ctx, 0, 0);
            start_waiting_jobs(ctx);
            if jobserver_tokens() == 0 {
                break;
            }
            if children.is_null() {
                fatal(
                    ctx,
                    ::core::ptr::null_mut::<Floc>(),
                    0,
                    b"INTERNAL: no children as we go to sleep on read\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            got_token = jobserver_acquire(ctx, (waiting_jobs != NULL as *mut child) as i32) as i32;
            if !(got_token == 1) {
                continue;
            }
            if 0x4_i32 & db_level != 0 {
                printf(
                    b"Obtained token for child %p (%s).\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    c,
                    (*c).file.as_ref().expect("a child always has a file").name,
                );
                fflush(stdout);
            }
            break;
        }
    }
    JOBSERVER_TOKENS.fetch_add(1, Ordering::Relaxed);
    if 0x20_i32 & db_level != 0 {
        // Owns the concatenated also-make name list when one is built below;
        // stays empty (no allocation) when the target has no also_make set.
        let mut nmbuf_buf: Vec<u8> = Vec::new();
        let nm: *const ::core::ffi::c_char;
        let tp: *const ::core::ffi::c_char;
        if (*cmds).fileinfo.filenm.is_null() {
            nm = b"<builtin>\0" as *const u8 as *const ::core::ffi::c_char;
        } else {
            alloca_allocations.push(::std::vec::from_elem(
                0,
                strlen((*cmds).fileinfo.filenm)
                    .wrapping_add(1)
                    .wrapping_add(11)
                    .wrapping_add(1) as usize,
            ));
            let n: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            sprintf(
                n,
                b"%s:%lu\0" as *const u8 as *const ::core::ffi::c_char,
                (*cmds).fileinfo.filenm,
                (*cmds).fileinfo.lineno,
            );
            nm = n;
        }
        if (*c)
            .file
            .as_ref()
            .expect("a child always has a file")
            .also_make
            .is_null()
        {
            tp = (*c).file.as_ref().expect("a child always has a file").name;
        } else {
            let mut dp: *const Dep;
            let mut cp: *mut ::core::ffi::c_char;
            let mut len: size_t =
                strlen((*c).file.as_ref().expect("a child always has a file").name) as size_t;
            dp = (*c)
                .file
                .as_ref()
                .expect("a child always has a file")
                .also_make;
            while !dp.is_null() {
                len = len.wrapping_add(
                    strlen((*dp).file.as_ref().expect("a dep always has a file").name)
                        .wrapping_add(4) as size_t,
                );
                dp = (*dp).next;
            }
            nmbuf_buf = Vec::with_capacity(len.wrapping_add(1) as usize);
            let nmbuf = nmbuf_buf.as_mut_ptr() as *mut ::core::ffi::c_char;
            tp = nmbuf;
            cp = stpcpy(
                nmbuf,
                (*c).file.as_ref().expect("a child always has a file").name,
            );
            dp = (*c)
                .file
                .as_ref()
                .expect("a child always has a file")
                .also_make;
            while !dp.is_null() {
                cp = stpcpy(
                    stpcpy(cp, b"', '\0" as *const u8 as *const ::core::ffi::c_char),
                    (*dp).file.as_ref().expect("a dep always has a file").name,
                );
                dp = (*dp).next;
            }
        }
        if (*c)
            .file
            .as_ref()
            .expect("a child always has a file")
            .phony()
            != 0
        {
            message(
        ctx,
        0,
        (strlen(nm) as size_t).wrapping_add(strlen(tp) as size_t),
        b"%s: update target '%s' due to: target is .PHONY\0" as *const u8
                    as *const ::core::ffi::c_char,
        &[FmtArg::Str((nm) as *const ::core::ffi::c_char),
            FmtArg::Str((tp) as *const ::core::ffi::c_char)],
    );
        } else if (*c)
            .file
            .as_ref()
            .expect("a child always has a file")
            .last_mtime
            == NONEXISTENT_MTIME as uintmax_t
        {
            message(
                ctx,
                0,
                (strlen(nm) as size_t).wrapping_add(strlen(tp) as size_t),
                b"%s: update target '%s' due to: target does not exist\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    FmtArg::Str((nm) as *const ::core::ffi::c_char),
                    FmtArg::Str((tp) as *const ::core::ffi::c_char),
                ],
            );
        } else {
            let mut newer: *mut ::core::ffi::c_char = allocated_expand_variable_for_file(
                ctx,
                b"?\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as size_t).wrapping_sub(1),
                (*c).file,
            );
            if *newer.offset(0_i32 as isize) as i32 != 0 {
                message(
                    ctx,
                    0,
                    (strlen(nm) as size_t)
                        .wrapping_add(strlen(tp) as size_t)
                        .wrapping_add(strlen(newer) as size_t),
                    b"%s: update target '%s' due to: %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        FmtArg::Str((nm) as *const ::core::ffi::c_char),
                        FmtArg::Str((tp) as *const ::core::ffi::c_char),
                        FmtArg::Str((newer) as *const ::core::ffi::c_char),
                    ],
                );
                free(newer as *mut ::core::ffi::c_void);
            } else {
                let mut len_0: size_t = 0;
                let mut d: *mut dep;
                d = (*c).file.as_ref().expect("a child always has a file").deps;
                while !d.is_null() {
                    if (*d)
                        .file
                        .as_ref()
                        .expect("a dep always has a file")
                        .last_mtime
                        == NONEXISTENT_MTIME as uintmax_t
                    {
                        len_0 = len_0.wrapping_add(
                            strlen((*d).file.as_ref().expect("a dep always has a file").name)
                                .wrapping_add(1) as size_t,
                        );
                    }
                    d = (*d).next;
                }
                if len_0 == 0 {
                    message(
                        ctx,
                        0,
                        (strlen(nm) as size_t).wrapping_add(strlen(tp) as size_t),
                        b"%s: update target '%s' due to: unknown reasons\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            FmtArg::Str((nm) as *const ::core::ffi::c_char),
                            FmtArg::Str((tp) as *const ::core::ffi::c_char),
                        ],
                    );
                } else {
                    alloca_allocations.push(::std::vec::from_elem(0, len_0 as usize));
                    newer = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                        as *mut ::core::ffi::c_char;
                    let mut cp_0: *mut ::core::ffi::c_char = newer;
                    d = (*c).file.as_ref().expect("a child always has a file").deps;
                    while !d.is_null() {
                        if (*d)
                            .file
                            .as_ref()
                            .expect("a dep always has a file")
                            .last_mtime
                            == NONEXISTENT_MTIME as uintmax_t
                        {
                            if cp_0 > newer {
                                let fresh8 = cp_0;
                                cp_0 = cp_0.offset(1_i32 as isize);
                                *fresh8 = ' ' as i32 as ::core::ffi::c_char;
                            }
                            cp_0 = stpcpy(
                                cp_0,
                                (*d).file.as_ref().expect("a dep always has a file").name,
                            );
                        }
                        d = (*d).next;
                    }
                    message(
                        ctx,
                        0,
                        (strlen(nm) as size_t)
                            .wrapping_add(strlen(tp) as size_t)
                            .wrapping_add(strlen(newer) as size_t),
                        b"%s: update target '%s' due to: %s\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            FmtArg::Str((nm) as *const ::core::ffi::c_char),
                            FmtArg::Str((tp) as *const ::core::ffi::c_char),
                            FmtArg::Str((newer) as *const ::core::ffi::c_char),
                        ],
                    );
                }
            }
        }
        drop(nmbuf_buf);
    }
    start_waiting_job(ctx, c);
    if crate::make_main::opt_job_slots() == 1 || not_parallel() {
        while (*file).command_state() as i32 == cs_running as i32 {
            reap_children(ctx, 1, 0);
        }
    }
    output_context = ::core::ptr::null_mut::<output>();
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn job_next_command(child: *mut child) -> i32 {
    while (*child).command_ptr.is_null() || *(*child).command_ptr as i32 == 0 {
        if (*child).command_line
            == (*child)
                .file
                .as_ref()
                .expect("a child always has a file")
                .cmds
                .as_ref()
                .expect("a child being run has a recipe")
                .ncommand_lines as ::core::ffi::c_uint
        {
            (*child).command_ptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*child)
                .file
                .as_ref()
                .expect("a child always has a file")
                .cmds
                .as_mut()
                .expect("a child being run has a recipe")
                .fileinfo
                .offset = 0;
            return 0;
        } else {
            let fresh15 = (*child).command_line;
            (*child).command_line = (*child).command_line.wrapping_add(1);
            (*child).command_ptr = *(*child).command_lines.offset(fresh15 as isize);
        }
    }
    (*child)
        .file
        .as_ref()
        .expect("a child always has a file")
        .cmds
        .as_mut()
        .expect("a child being run has a recipe")
        .fileinfo
        .offset = (*child).command_line.wrapping_sub(1) as ::core::ffi::c_ulong;
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
/// caller writes the returned `sample_second`/`prev_weight` back into the
/// `ExecContext` cells and the job counter back into `JOB_COUNTER`; this matches
/// the original, where `JOB_COUNTER` was reset inside the per-second branch and
/// re-read when forming the guess.
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
    static mut proc_fd: i32 = -2_i32;
    let mut load: ::core::ffi::c_double = 0.;
    if crate::make_main::opt_max_load_average() < 0_i32 as ::core::ffi::c_double {
        return 0;
    }
    if proc_fd == -2_i32 {
        loop {
            proc_fd = open(
                b"/proc/loadavg\0" as *const u8 as *const ::core::ffi::c_char,
                0,
            );
            if !(proc_fd == -1_i32 && *__errno_location() == EINTR) {
                break;
            }
        }
        if proc_fd < 0 {
            if 0x4_i32 & db_level != 0 {
                printf(
                    b"Using system load detection method.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fflush(stdout);
            }
        } else {
            if 0x4_i32 & db_level != 0 {
                printf(
                    b"Using /proc/loadavg load detection method.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fflush(stdout);
            }
            fd_noinherit(proc_fd);
        }
    }
    if proc_fd >= 0 {
        let mut r: i32;
        loop {
            r = lseek(proc_fd, 0 as __off_t, 0) as i32;
            if !(r == -1_i32 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r >= 0 {
            let mut avg: [::core::ffi::c_char; 65] = [0; 65];
            loop {
                r = read(
                    proc_fd,
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
                    if 0x4_i32 & db_level != 0 {
                        printf(
                            b"Running: system = %u / make = %u (max requested = %f)\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            cnt,
                            job_slots_used(),
                            crate::make_main::opt_max_load_average(),
                        );
                        fflush(stdout);
                    }
                    return (cnt as ::core::ffi::c_double > crate::make_main::opt_max_load_average())
                        as i32;
                }
                if 0x4_i32 & db_level != 0 {
                    printf(
                        b"Failed to parse /proc/loadavg: %s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &raw mut avg as *mut ::core::ffi::c_char,
                    );
                    fflush(stdout);
                }
            }
        }
        if r < 0 && 0x4_i32 & db_level != 0 {
            printf(
                b"Failed to read /proc/loadavg: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            fflush(stdout);
        }
        close(proc_fd);
        proc_fd = -1_i32;
    }
    *__errno_location() = 0;
    if getloadavg(&raw mut load, 1) != 1 {
        static mut lossage: i32 = -1_i32;
        if lossage == -1_i32 || *__errno_location() != lossage {
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
        lossage = *__errno_location();
        load = 0_i32 as ::core::ffi::c_double;
    }
    let now: time_t = wall_clock_seconds();
    let (next_sample_second, next_prev_weight, next_job_counter, guess) = load_sample_fold(
        ctx.load_sample_second.get(),
        ctx.load_prev_weight.get(),
        JOB_COUNTER.load(Ordering::Relaxed),
        now,
        load,
    );
    ctx.load_sample_second.set(next_sample_second);
    ctx.load_prev_weight.set(next_prev_weight);
    JOB_COUNTER.store(next_job_counter, Ordering::Relaxed);
    if 0x4_i32 & db_level != 0 {
        printf(
            b"Estimated system load = %f (actual = %f) (max requested = %f)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            guess,
            load,
            crate::make_main::opt_max_load_average(),
        );
        fflush(stdout);
    }
    (guess >= crate::make_main::opt_max_load_average()) as i32
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn start_waiting_jobs(ctx: &crate::execctx::ExecContext) {
    let mut job: *mut child;
    if waiting_jobs.is_null() {
        return;
    }
    loop {
        reap_children(ctx, 0, 0);
        job = waiting_jobs;
        waiting_jobs = (*job).next;
        if !(start_waiting_job(ctx, job) != 0 && !waiting_jobs.is_null()) {
            break;
        }
    }
}
/// RAII guard that runs `posix_spawnattr_destroy` on drop. Created only after a
/// successful `posix_spawnattr_init`, so cleanup happens automatically on every
/// exit path (replacing the C `goto`-to-cleanup dance).
struct SpawnAttr(*mut posix_spawnattr_t);
impl Drop for SpawnAttr {
    fn drop(&mut self) {
        unsafe {
            posix_spawnattr_destroy(self.0);
        }
    }
}
/// RAII guard that runs `posix_spawn_file_actions_destroy` on drop. Declared
/// after `SpawnAttr` so it drops first, matching the C cleanup order (file
/// actions before attributes).
struct SpawnFileActions(*mut posix_spawn_file_actions_t);
impl Drop for SpawnFileActions {
    fn drop(&mut self) {
        unsafe {
            posix_spawn_file_actions_destroy(self.0);
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn child_execute_job(
    ctx: &crate::execctx::ExecContext,
    child: *mut childbase,
    good_stdin: i32,
    argv: *mut *mut ::core::ffi::c_char,
) -> pid_t {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let fdin: i32 = if good_stdin != 0 {
        fileno(stdin)
    } else {
        get_bad_stdin()
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
        &[FmtArg::Str((*argv.offset(0_i32 as isize)) as *const ::core::ffi::c_char),
            FmtArg::Str((strerror(r)) as *const ::core::ffi::c_char)],
    );
    }
    pid
}
/// Configure a `posix_spawn` and launch `argv[0]`, looking it up on the child's
/// PATH. Returns the spawn `errno` (0 on success) and writes the new pid into
/// `*pid`. The attribute and file-action objects are released automatically by
/// their RAII guards on every return path.
unsafe fn spawn_child(
    child: *mut childbase,
    argv: *mut *mut ::core::ffi::c_char,
    fdin: i32,
    fdout: i32,
    fderr: i32,
    pid: *mut pid_t,
    alloca_allocations: &mut Vec<Vec<u8>>,
) -> i32 {
    let mut attr: posix_spawnattr_t = posix_spawnattr_t {
        __flags: 0,
        __pgrp: 0,
        __sd: __sigset_t { __val: [0; 16] },
        __ss: __sigset_t { __val: [0; 16] },
        __sp: sched_param { sched_priority: 0 },
        __policy: 0,
        __cgroup: 0,
        __pad: [0; 15],
    };
    let mut r = posix_spawnattr_init(&raw mut attr);
    if r != 0 {
        return r;
    }
    let _attr_guard = SpawnAttr(&raw mut attr);
    let mut fa: posix_spawn_file_actions_t = posix_spawn_file_actions_t {
        __allocated: 0,
        __used: 0,
        __actions: ::core::ptr::null_mut::<__spawn_action>(),
        __pad: [0; 16],
    };
    r = posix_spawn_file_actions_init(&raw mut fa);
    if r != 0 {
        return r;
    }
    let _fa_guard = SpawnFileActions(&raw mut fa);
    let mut mask: sigset_t = __sigset_t { __val: [0; 16] };
    sigemptyset(&raw mut mask);
    r = posix_spawnattr_setsigmask(&raw mut attr, &raw mut mask);
    if r != 0 {
        return r;
    }
    let flags: ::core::ffi::c_short =
        (POSIX_SPAWN_SETSIGMASK | POSIX_SPAWN_USEVFORK) as ::core::ffi::c_short;
    if fdin >= 0 && fdin != fileno(stdin) {
        r = posix_spawn_file_actions_adddup2(&raw mut fa, fdin, fileno(stdin));
        if r != 0 {
            return r;
        }
    }
    if fdout != fileno(stdout) {
        r = posix_spawn_file_actions_adddup2(&raw mut fa, fdout, fileno(stdout));
        if r != 0 {
            return r;
        }
    }
    if fderr != fileno(stderr) {
        r = posix_spawn_file_actions_adddup2(&raw mut fa, fderr, fileno(stderr));
        if r != 0 {
            return r;
        }
    }
    r = posix_spawnattr_setflags(&raw mut attr, flags);
    if r != 0 {
        return r;
    }
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
    loop {
        r = posix_spawn(
            pid,
            cmd,
            &raw mut fa,
            &raw mut attr,
            argv,
            (*child).environment,
        );
        if r != EINTR {
            break;
        }
    }
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
        loop {
            r = posix_spawn(
                pid,
                *nargv.offset(0),
                &raw mut fa,
                &raw mut attr,
                nargv,
                (*child).environment,
            );
            if r != EINTR {
                break;
            }
        }
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
        &[FmtArg::Str((*argv.offset(0_i32 as isize)) as *const ::core::ffi::c_char),
            FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char)],
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
        &[FmtArg::Str((*new_argv.offset(0_i32 as isize)) as *const ::core::ffi::c_char),
            FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char)],
    );
        }
        _ => {
            error(
        ctx,
        ::core::ptr::null_mut::<Floc>(),
        (strlen(*argv.offset(0_i32 as isize)) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
        b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((*argv.offset(0_i32 as isize)) as *const ::core::ffi::c_char),
            FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char)],
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
) -> *mut *mut ::core::ffi::c_char {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    static mut sh_chars: *const ::core::ffi::c_char =
        b"#;\"*?[]&|<>(){}$`^~!\0" as *const u8 as *const ::core::ffi::c_char;
    static mut sh_cmds: [*const ::core::ffi::c_char; 38] = [
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
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
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
            while *cap as i32 != 0 {
                if *cap as i32 != ' ' as i32
                    && *cap as i32 != '\t' as i32
                    && *cap as i32 != '\n' as i32
                {
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
                        && unixy_shell != 0
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
                if one_shell() && *p as i32 == '\n' as i32 {
                    break 'fast;
                }
                match *p as i32 {
                    61 => {
                        if seen_nonequals == 0 && unixy_shell != 0 {
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
            return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        }
        return new_argv;
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
    if one_shell() {
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
            );
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
        return new_argv;
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
                if batch_mode_shell == 0 {
                    let fresh45 = ap;
                    ap = ap.offset(1_i32 as isize);
                    *fresh45 = '\\' as i32 as ::core::ffi::c_char;
                }
                let fresh46 = ap;
                ap = ap.offset(1_i32 as isize);
                *fresh46 = '\n' as i32 as ::core::ffi::c_char;
                p = p.offset(1_i32 as isize);
            } else {
                if unixy_shell != 0
                    && batch_mode_shell == 0
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
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    *ap = 0;
    if unixy_shell != 0 {
        new_argv = construct_command_argv_internal(
            ctx,
            new_line,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            flags,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    } else {
        fatal(
        ctx,
        NILF,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t)
                .wrapping_sub(1)
                .wrapping_add(INTSTR_LENGTH),
        b"%s (line %d) Bad shell context (!unixy && !batch_mode_shell)\n\0" as *const u8
                as *const ::core::ffi::c_char,
        &[FmtArg::Str((b"src/job.c\0" as *const u8 as *const ::core::ffi::c_char) as *const ::core::ffi::c_char),
            FmtArg::Int((3621_i32) as i32 as i64)],
    );
    }
    free(new_line as *mut ::core::ffi::c_void);
    new_argv
}
pub const PRESERVE_BSNL: i32 = 1;
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn construct_command_argv(
    ctx: &crate::execctx::ExecContext,
    line: *mut ::core::ffi::c_char,
    restp: *mut *mut ::core::ffi::c_char,
    file: *mut file,
    cmd_flags: i32,
    batch_filename: *mut *mut ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let shell: *mut ::core::ffi::c_char;
    let ifs: *mut ::core::ffi::c_char;
    let mut allocflags: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let shellflags: *const ::core::ffi::c_char;
    let argv: *mut *mut ::core::ffi::c_char;
    let var: *mut variable;
    let save: Action = warning::action(Type::UndefinedVar);
    warning::set_action(Type::UndefinedVar, Action::Ignore);
    shell = allocated_expand_variable_for_file(
        ctx,
        b"SHELL\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
        file,
    );
    var = lookup_variable_for_file(
        ctx,
        b".SHELLFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t).wrapping_sub(1),
        file,
    ) as *mut variable;
    if var.is_null() {
        shellflags = b"\0" as *const u8 as *const ::core::ffi::c_char;
    } else if (*var).origin() as i32 != o_default as i32 {
        allocflags = allocated_expand_string_for_file(ctx, (*var).value, file);
        shellflags = allocflags;
    } else if posix_pedantic() && !crate::make_main::opt_ignore_errors() && !(cmd_flags & 4 != 0) {
        shellflags = b"-ec\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        shellflags = b"-c\0" as *const u8 as *const ::core::ffi::c_char;
    }
    ifs = allocated_expand_variable_for_file(
        ctx,
        b"IFS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 4]>() as size_t).wrapping_sub(1),
        file,
    );
    warning::set_action(Type::UndefinedVar, save);
    argv = construct_command_argv_internal(
        ctx,
        line,
        restp,
        shell,
        shellflags,
        ifs,
        cmd_flags,
        batch_filename,
    );
    free(shell as *mut ::core::ffi::c_void);
    free(allocflags as *mut ::core::ffi::c_void);
    free(ifs as *mut ::core::ffi::c_void);
    argv
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
    use super::is_bourne_compatible_shell;
    use std::path::Path;

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
        assert!(!ctx.good_stdin_used.get(), "false means stdin still available");

        ctx.good_stdin_used.set(true);
        assert!(ctx.good_stdin_used.get(), "true means stdin already claimed");

        ctx.good_stdin_used.set(false);
        assert!(!ctx.good_stdin_used.get(), "cleared when the job is reaped");
    }
}

#[cfg(test)]
mod dead_children_tests {
    use super::{dead_children, DEAD_CHILDREN};
    use std::sync::atomic::Ordering;

    /// `dead_children()` reflects the `DEAD_CHILDREN` counter and the atomic
    /// add/sub used by the signal handler and reap loop round-trip. Restores
    /// the prior value so it stays isolated from other tests.
    #[test]
    fn dead_children_counts_round_trip() {
        let saved = DEAD_CHILDREN.load(Ordering::Relaxed);

        DEAD_CHILDREN.store(0, Ordering::Relaxed);
        assert_eq!(dead_children(), 0);

        DEAD_CHILDREN.fetch_add(1, Ordering::Relaxed);
        DEAD_CHILDREN.fetch_add(1, Ordering::Relaxed);
        assert_eq!(dead_children(), 2, "two reaped children pending");

        DEAD_CHILDREN.fetch_sub(1, Ordering::Relaxed);
        assert_eq!(dead_children(), 1, "one processed");

        DEAD_CHILDREN.store(saved, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod job_slots_used_tests {
    use super::{job_slots_used, JOB_SLOTS_USED};
    use std::sync::atomic::Ordering;

    /// `job_slots_used()` reflects the `JOB_SLOTS_USED` counter, and the
    /// add/sub used by the start/reap paths round-trip through it. Restores the
    /// prior value so it stays isolated from other tests.
    #[test]
    fn job_slots_used_counts_round_trip() {
        let saved = JOB_SLOTS_USED.load(Ordering::Relaxed);

        JOB_SLOTS_USED.store(0, Ordering::Relaxed);
        assert_eq!(job_slots_used(), 0);

        JOB_SLOTS_USED.fetch_add(1, Ordering::Relaxed);
        JOB_SLOTS_USED.fetch_add(1, Ordering::Relaxed);
        assert_eq!(job_slots_used(), 2, "two slots in use");

        JOB_SLOTS_USED.store(job_slots_used().wrapping_sub(1), Ordering::Relaxed);
        assert_eq!(job_slots_used(), 1, "one slot freed");

        JOB_SLOTS_USED.store(saved, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod jobserver_tokens_tests {
    use super::{jobserver_tokens, JOBSERVER_TOKENS};
    use std::sync::atomic::Ordering;

    /// `jobserver_tokens()` reflects the `JOBSERVER_TOKENS` counter, and the
    /// add/sub used by the acquire/free paths round-trip through it. Restores
    /// the prior value so it stays isolated from other tests.
    #[test]
    fn jobserver_tokens_counts_round_trip() {
        let saved = JOBSERVER_TOKENS.load(Ordering::Relaxed);

        JOBSERVER_TOKENS.store(0, Ordering::Relaxed);
        assert_eq!(jobserver_tokens(), 0);

        JOBSERVER_TOKENS.fetch_add(1, Ordering::Relaxed);
        JOBSERVER_TOKENS.fetch_add(1, Ordering::Relaxed);
        assert_eq!(jobserver_tokens(), 2, "two tokens held");

        JOBSERVER_TOKENS.fetch_sub(1, Ordering::Relaxed);
        assert_eq!(jobserver_tokens(), 1, "one released");

        JOBSERVER_TOKENS.store(saved, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod unixy_shell_tests {
    use super::unixy_shell;

    /// `unixy_shell` is an immutable `static` fixed at 1 in this POSIX port and
    /// is readable from safe code (no `unsafe` needed).
    #[test]
    fn unixy_shell_is_one() {
        assert_eq!(unixy_shell, 1);
    }
}

#[cfg(test)]
mod batch_mode_shell_tests {
    use super::batch_mode_shell;

    /// `batch_mode_shell` is an immutable `static` fixed at 0 in this POSIX
    /// port and is readable from safe code (no `unsafe` needed).
    #[test]
    fn batch_mode_shell_is_zero() {
        assert_eq!(batch_mode_shell, 0);
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
        crate::make_main::initialize_stopchar_map();
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
    use super::{child_error_label, smode_or_empty, Floc};
    use std::ffi::{CStr, CString};

    /// `smode_or_empty` passes a non-null shuffle string through and maps null
    /// to the empty string.
    #[test]
    fn smode_or_empty_maps_null_to_empty() {
        let s = CString::new("shuffle=3").unwrap();
        assert_eq!(smode_or_empty(s.as_ptr()), s.as_ptr());

        let empty = smode_or_empty(::core::ptr::null());
        unsafe {
            assert_eq!(*empty, 0, "null shuffle mode -> empty string");
        }
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
