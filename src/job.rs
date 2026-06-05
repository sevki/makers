use libc::{__errno_location, close, free, getenv, getloadavg, open, printf, remove, sprintf, stpcpy, strchr, strcmp, strerror, strsignal};
use ::c2rust_bitfields;
use crate::stdio::{FILE};
use crate::file::{Commands, Dep, File, VariableSet, VariableSetList};
pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __pid_t, __sig_atomic_t, __syscall_slong_t, __time_t, __uid_t, pid_t, sig_atomic_t, size_t,
    ssize_t, time_t, uintmax_t,
};
extern "C" {
    pub type __spawn_action;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigprocmask(
        __how: ::core::ffi::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> ::core::ffi::c_int;
    fn lseek(__fd: ::core::ffi::c_int, __offset: __off_t, __whence: ::core::ffi::c_int) -> __off_t;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    static mut environ: *mut *mut ::core::ffi::c_char;
    fn execvp(
        __file: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn confstr(
        __name: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> size_t;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
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
    fn message(prefix: ::core::ffi::c_int, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn error(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn fatal(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn die(_: ::core::ffi::c_int) -> !;
    fn pfatal_with_name(_: *const ::core::ffi::c_char) -> !;
    fn perror_with_name(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char);
    fn make_toui(
        _: *const ::core::ffi::c_char,
        _: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_uint;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xcalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xstrdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn show_goal_error();
    static mut stopchar_map: [::core::ffi::c_ushort; 0];
    static mut just_print_flag: ::core::ffi::c_int;
    static mut run_silent: ::core::ffi::c_int;
    static mut keep_going_flag: ::core::ffi::c_int;
    static mut ignore_errors_flag: ::core::ffi::c_int;
    static mut touch_flag: ::core::ffi::c_int;
    static mut question_flag: ::core::ffi::c_int;
    static mut posix_pedantic: ::core::ffi::c_int;
    static mut not_parallel: ::core::ffi::c_int;
    static mut one_shell: ::core::ffi::c_int;
    static mut output_sync: ::core::ffi::c_int;
    static mut command_count: ::core::ffi::c_ulong;
    static mut job_slots: ::core::ffi::c_uint;
    static mut max_load_average: ::core::ffi::c_double;
    static mut commands_started: ::core::ffi::c_uint;
    static mut handling_fatal_signal: sig_atomic_t;
    static mut output_context: *mut output;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn wait(__stat_loc: *mut ::core::ffi::c_int) -> __pid_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    fn posix_spawn(
        __pid: *mut pid_t,
        __path: *const ::core::ffi::c_char,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut ::core::ffi::c_char,
        __envp: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn posix_spawnattr_init(__attr: *mut posix_spawnattr_t) -> ::core::ffi::c_int;
    fn posix_spawnattr_destroy(__attr: *mut posix_spawnattr_t) -> ::core::ffi::c_int;
    fn posix_spawnattr_setsigmask(
        __attr: *mut posix_spawnattr_t,
        __sigmask: *const sigset_t,
    ) -> ::core::ffi::c_int;
    fn posix_spawnattr_setflags(
        _attr: *mut posix_spawnattr_t,
        __flags: ::core::ffi::c_short,
    ) -> ::core::ffi::c_int;
    fn posix_spawn_file_actions_init(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> ::core::ffi::c_int;
    fn posix_spawn_file_actions_destroy(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> ::core::ffi::c_int;
    fn posix_spawn_file_actions_adddup2(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: ::core::ffi::c_int,
        __newfd: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn find_in_given_path(
        progname: *const ::core::ffi::c_char,
        path: *const ::core::ffi::c_char,
        directory: *const ::core::ffi::c_char,
        optimize_for_exec: bool,
    ) -> *const ::core::ffi::c_char;
    fn delete_child_targets(child: *mut child);
    fn chop_commands(cmds: *mut commands);
    static mut db_level: ::core::ffi::c_int;
    fn lookup_file(name: *const ::core::ffi::c_char) -> *mut file;
    fn set_command_state(file: *mut file, state: cmd_state);
    fn notice_finished_file(file: *mut file);
    fn fd_noinherit(fd: ::core::ffi::c_int);
    fn jobserver_enabled() -> ::core::ffi::c_uint;
    fn jobserver_release(is_fatal: ::core::ffi::c_int);
    fn jobserver_signal();
    fn jobserver_pre_child(_: ::core::ffi::c_int);
    fn jobserver_post_child(_: ::core::ffi::c_int);
    fn jobserver_pre_acquire();
    fn jobserver_acquire(timeout: ::core::ffi::c_int) -> ::core::ffi::c_uint;
    fn get_bad_stdin() -> ::core::ffi::c_int;
    fn allocated_expand_string_for_file(
        line: *const ::core::ffi::c_char,
        file: *mut file,
    ) -> *mut ::core::ffi::c_char;
    fn allocated_expand_variable_for_file(
        name: *const ::core::ffi::c_char,
        length: size_t,
        file: *mut file,
    ) -> *mut ::core::ffi::c_char;
    fn shell_completed(exit_code: ::core::ffi::c_int, exit_sig: ::core::ffi::c_int);
    fn lookup_variable_for_file(
        name: *const ::core::ffi::c_char,
        length: size_t,
        file: *mut file,
    ) -> *mut variable;
    fn target_environment(
        file: *mut file,
        recursive: ::core::ffi::c_int,
    ) -> *mut *mut ::core::ffi::c_char;
    static mut fatal_signal_set: sigset_t;
    static mut shell_function_pid: pid_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub use crate::sys_stat::timespec;
pub use crate::sys_stat::stat;
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
pub type file = File;
pub type cmd_state = ::core::ffi::c_uint;
pub const cs_finished: cmd_state = 3;
pub const cs_running: cmd_state = 2;
pub const cs_deps_running: cmd_state = 1;
pub const cs_not_started: cmd_state = 0;
pub type update_status = ::core::ffi::c_uint;
pub type update_status_0 = u32;
pub const us_failed: update_status_0 = 3;
pub const us_question: update_status_0 = 2;
pub const us_none: update_status_0 = 1;
pub const us_success: update_status_0 = 0;
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
pub type hash_table = crate::hash::hash_table;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
pub type dep = Dep;
pub type commands = Commands;
use crate::floc::Floc;

pub const o_invalid: variable_origin = 7;
pub const o_automatic: variable_origin = 6;
pub const o_override: variable_origin = 5;
pub const o_command: variable_origin = 4;
pub const o_env_override: variable_origin = 3;
pub const o_file: variable_origin = 2;
pub const o_env: variable_origin = 1;
pub const o_default: variable_origin = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct variable {
    pub name: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
    pub fileinfo: Floc,
    pub length: ::core::ffi::c_uint,
    #[bitfield(name = "recursive", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "append", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "conditional", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "per_target", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "special", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "exportable", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "expanding", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "private_var", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "exp_count", ty = "::core::ffi::c_uint", bits = "8..=22")]
    #[bitfield(name = "flavor", ty = "variable_flavor", bits = "23..=25")]
    #[bitfield(name = "origin", ty = "variable_origin", bits = "26..=28")]
    #[bitfield(name = "export", ty = "variable_export", bits = "29..=30")]
    pub recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export:
        [u8; 4],
}
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
    pub file: *mut file,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawnattr_t {
    pub __flags: ::core::ffi::c_short,
    pub __pgrp: pid_t,
    pub __sd: sigset_t,
    pub __ss: sigset_t,
    pub __sp: sched_param,
    pub __policy: ::core::ffi::c_int,
    pub __cgroup: ::core::ffi::c_int,
    pub __pad: [::core::ffi::c_int; 15],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawn_file_actions_t {
    pub __allocated: ::core::ffi::c_int,
    pub __used: ::core::ffi::c_int,
    pub __actions: *mut __spawn_action,
    pub __pad: [::core::ffi::c_int; 16],
}
use crate::warning::{self, Action, Type};
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IEXEC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const SIG_BLOCK: ::core::ffi::c_int = 0;
pub const SIG_UNBLOCK: ::core::ffi::c_int = 1;
pub const SIG_SETMASK: ::core::ffi::c_int = 2;
pub const ENOENT: ::core::ffi::c_int = 2;
pub const EINTR: ::core::ffi::c_int = 4;
pub const ENOEXEC: ::core::ffi::c_int = 8;
pub const EACCES: ::core::ffi::c_int = 13;
pub const WNOHANG: ::core::ffi::c_int = 1;
pub const __WCOREFLAG: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const INTSTR_LENGTH: usize = (53 as usize)
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22 as usize)
    .wrapping_add(3 as usize);
pub const OUTPUT_SYNC_LINE: ::core::ffi::c_int = 1;
pub const OUTPUT_SYNC_RECURSE: ::core::ffi::c_int = 3;
pub const MAKE_SUCCESS: ::core::ffi::c_int = 0;
pub const MAKE_TROUBLE: ::core::ffi::c_int = 1;
pub const MAKE_FAILURE: ::core::ffi::c_int = 2;
#[no_mangle]
pub static mut default_shell: *const ::core::ffi::c_char =
    b"/bin/sh\0" as *const u8 as *const ::core::ffi::c_char;
#[no_mangle]
pub static mut batch_mode_shell: ::core::ffi::c_int = 0;
pub const S_IXUSR: ::core::ffi::c_int = __S_IEXEC;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const POSIX_SPAWN_SETSIGMASK: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POSIX_SPAWN_USEVFORK: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const COMMANDS_RECURSE: ::core::ffi::c_int = 1;
pub const COMMANDS_SILENT: ::core::ffi::c_int = 2;
pub const NONEXISTENT_MTIME: ::core::ffi::c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn pid2str(pid: pid_t) -> *const ::core::ffi::c_char {
    static mut pidstring: [::core::ffi::c_char; 100] = [0; 100];
    sprintf(
        &raw mut pidstring as *mut ::core::ffi::c_char,
        b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
        pid as ::core::ffi::c_ulong,
    );
    &raw mut pidstring as *mut ::core::ffi::c_char
}
#[no_mangle]
pub static mut children: *mut child = ::core::ptr::null::<child>() as *mut child;
#[no_mangle]
pub static mut job_slots_used: ::core::ffi::c_uint = 0;
static mut good_stdin_used: ::core::ffi::c_int = 0;
static mut waiting_jobs: *mut child = ::core::ptr::null::<child>() as *mut child;
#[no_mangle]
pub static mut unixy_shell: ::core::ffi::c_int = 1;
#[no_mangle]
pub static mut job_counter: ::core::ffi::c_ulong = 0;
#[no_mangle]
pub static mut jobserver_tokens: ::core::ffi::c_uint = 0;
#[no_mangle]
pub unsafe extern "C" fn is_bourne_compatible_shell(
    path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut unix_shells: [*const ::core::ffi::c_char; 8] = [
        b"sh\0" as *const u8 as *const ::core::ffi::c_char,
        b"bash\0" as *const u8 as *const ::core::ffi::c_char,
        b"dash\0" as *const u8 as *const ::core::ffi::c_char,
        b"ksh\0" as *const u8 as *const ::core::ffi::c_char,
        b"rksh\0" as *const u8 as *const ::core::ffi::c_char,
        b"zsh\0" as *const u8 as *const ::core::ffi::c_char,
        b"ash\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut s: *mut *const ::core::ffi::c_char;
    let mut cp: *const ::core::ffi::c_char = path.offset(strlen(path) as isize);
    while cp > path
        && !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort).offset(*cp.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar as isize,
            ) as ::core::ffi::c_int
            & 0x8000 as ::core::ffi::c_int
            != 0)
    {
        cp = cp.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    s = &raw mut unix_shells as *mut *const ::core::ffi::c_char;
    while !(*s).is_null() {
        if strcmp(cp, *s) == 0 {
            return 1;
        }
        s = s.offset(1 as ::core::ffi::c_int as isize);
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn block_sigs() {
    sigprocmask(
        SIG_BLOCK,
        &raw mut fatal_signal_set,
        ::core::ptr::null_mut::<sigset_t>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn unblock_sigs() {
    sigprocmask(
        SIG_UNBLOCK,
        &raw mut fatal_signal_set,
        ::core::ptr::null_mut::<sigset_t>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn unblock_all_sigs() {
    let mut empty: sigset_t = __sigset_t { __val: [0; 16] };
    sigemptyset(&raw mut empty);
    sigprocmask(
        SIG_SETMASK,
        &raw mut empty,
        ::core::ptr::null_mut::<sigset_t>(),
    );
}
unsafe extern "C" fn child_error(
    child: *mut child,
    exit_code: ::core::ffi::c_int,
    exit_sig: ::core::ffi::c_int,
    coredump: ::core::ffi::c_int,
    ignored: ::core::ffi::c_int,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut pre: *const ::core::ffi::c_char = b"*** \0" as *const u8 as *const ::core::ffi::c_char;
    let mut post: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let mut dump: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let f: *const file = (*child).file;
    let flocp: *const Floc = &raw mut (*(*f).cmds).fileinfo;
    let nm: *const ::core::ffi::c_char;
    let mut smode: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut l: size_t;
    if ignored != 0 && run_silent != 0 {
        return;
    }
    if exit_sig != 0 && coredump != 0 {
        dump = b" (core dumped)\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if ignored != 0 {
        pre = b"\0" as *const u8 as *const ::core::ffi::c_char;
        post = b" (ignored)\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if (*flocp).filenm.is_null() {
        nm = b"<builtin>\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        alloca_allocations.push(::std::vec::from_elem(
            0,
            strlen((*flocp).filenm)
                .wrapping_add(6)
                .wrapping_add(INTSTR_LENGTH)
                .wrapping_add(1) as usize,
        ));
        let a: *mut ::core::ffi::c_char =
            alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        sprintf(
            a,
            b"%s:%lu\0" as *const u8 as *const ::core::ffi::c_char,
            (*flocp).filenm,
            (*flocp).lineno.wrapping_add((*flocp).offset),
        );
        nm = a;
    }
    l = strlen(pre)
        .wrapping_add(strlen(nm))
        .wrapping_add(strlen((*f).name))
        .wrapping_add(strlen(post)) as size_t;
    if let Some(label) = crate::shuffle::get_mode() {
        let mut buf = format!(" shuffle={}", label).into_bytes();
        let written = buf.len();
        buf.push(0);
        alloca_allocations.push(buf);
        smode = alloca_allocations.last().unwrap().as_ptr() as *const ::core::ffi::c_char;
        l = l.wrapping_add(written as size_t);
    }
    output_context = if (*child).output.syncout() as ::core::ffi::c_int != 0 {
        &raw mut (*child).output
    } else {
        ::core::ptr::null_mut::<output>()
    };
    show_goal_error();
    if exit_sig == 0 {
        error(
            NILF,
            l.wrapping_add(INTSTR_LENGTH),
            b"%s[%s: %s] Error %d%s%s\0" as *const u8 as *const ::core::ffi::c_char,
            pre,
            nm,
            (*f).name,
            exit_code,
            post,
            if !smode.is_null() {
                smode
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
    } else {
        let s: *const ::core::ffi::c_char = strsignal(exit_sig);
        error(
            NILF,
            l.wrapping_add(strlen(s) as size_t)
                .wrapping_add(strlen(dump) as size_t),
            b"%s[%s: %s] %s%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
            pre,
            nm,
            (*f).name,
            s,
            dump,
            post,
            if !smode.is_null() {
                smode
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
    }
    output_context = ::core::ptr::null_mut::<output>();
}
static mut dead_children: ::core::ffi::c_uint = 0;
#[no_mangle]
pub unsafe extern "C" fn child_handler(mut _sig: ::core::ffi::c_int) {
    dead_children = dead_children.wrapping_add(1);
    jobserver_signal();
}
#[no_mangle]
pub unsafe extern "C" fn reap_children(mut block: ::core::ffi::c_int, err: ::core::ffi::c_int) {
    let mut status: ::core::ffi::c_int = 0;
    let mut reap_more: ::core::ffi::c_int = 1;
    let mut current_block_143: u64;
    while (!children.is_null() || shell_function_pid != 0)
        && (block != 0 || reap_more != 0)
    {
        let mut remote: ::core::ffi::c_uint = 0;
        let mut pid: pid_t;
        let mut exit_code: ::core::ffi::c_int = 0;
        let mut exit_sig: ::core::ffi::c_int = 0;
        let mut coredump: ::core::ffi::c_int = 0;
        let mut lastc: *mut child;
        let mut c: *mut child;
        let mut child_failed: ::core::ffi::c_int;
        let mut any_remote: ::core::ffi::c_int;
        let mut any_local: ::core::ffi::c_int;
        let dontcare: ::core::ffi::c_int;
        if err != 0 && block != 0 {
            static mut printed: ::core::ffi::c_int = 0;
            fflush(stdout);
            if printed == 0 {
                error(
                    ::core::ptr::null_mut::<Floc>(),
                    0,
                    b"*** Waiting for unfinished jobs....\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            printed = 1;
        }
        if dead_children > 0 {
            dead_children = dead_children.wrapping_sub(1);
        }
        any_remote = 0;
        any_local = (shell_function_pid != 0) as ::core::ffi::c_int;
        lastc = ::core::ptr::null_mut::<child>();
        c = children;
        loop {
            if c.is_null() {
                current_block_143 = 17478428563724192186;
                break;
            }
            any_remote |= (*c).remote() as ::core::ffi::c_int;
            any_local |= ((*c).remote() == 0) as ::core::ffi::c_int;
            if (*c).pid < 0 {
                exit_sig = 0;
                coredump = 0;
                exit_code = 127;
                current_block_143 = 16201671960271928402;
                break;
            } else {
                if 0x4 as ::core::ffi::c_int & db_level != 0 {
                    printf(
                        b"Live child %p (%s) PID %s %s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        c,
                        (*(*c).file).name,
                        pid2str((*c).pid),
                        if (*c).remote() as ::core::ffi::c_int != 0 {
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
        match current_block_143 {
            17478428563724192186 => {
                if any_remote != 0 {
                    pid = crate::remote_stub::remote_status(
                        &raw mut exit_code,
                        &raw mut exit_sig,
                        &raw mut coredump,
                        0,
                    ) as pid_t;
                } else {
                    pid = 0 as ::core::ffi::c_int as pid_t;
                }
                if pid > 0 {
                    remote = 1;
                } else if pid < 0 {
                    pfatal_with_name(b"remote_status\0" as *const u8 as *const ::core::ffi::c_char);
                } else {
                    if any_local != 0 {
                        if block == 0 {
                            pid = waitpid(-(1 as __pid_t), &raw mut status, WNOHANG) as pid_t;
                        } else {
                            loop {
                                pid = wait(&raw mut status) as pid_t;
                                if !(pid == -(1 as ::core::ffi::c_int)
                                    && *__errno_location() == EINTR)
                                {
                                    break;
                                }
                            }
                        }
                    } else {
                        pid = 0 as ::core::ffi::c_int as pid_t;
                    }
                    if pid < 0 {
                        pfatal_with_name(b"wait\0" as *const u8 as *const ::core::ffi::c_char);
                    } else if pid > 0 {
                        exit_code =
                            (status & 0xff00 as ::core::ffi::c_int) >> 8;
                        exit_sig = if ((status & 0x7f as ::core::ffi::c_int)
                            + 1)
                            as ::core::ffi::c_schar
                            as ::core::ffi::c_int
                            >> 1
                            > 0
                        {
                            status & 0x7f as ::core::ffi::c_int
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
                                b"remote_status\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                        }
                        if pid == 0 {
                            break;
                        }
                        remote = 1;
                    }
                }
                command_count = command_count.wrapping_add(1);
                if remote == 0 && pid == shell_function_pid {
                    shell_completed(exit_code, exit_sig);
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
                    if 0x4 as ::core::ffi::c_int & db_level != 0 {
                        printf(
                            if exit_sig == 0
                                && exit_code == 0
                            {
                                b"Reaping winning child %p PID %s %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char
                            } else {
                                b"Reaping losing child %p PID %s %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char
                            },
                            c,
                            pid2str((*c).pid),
                            if (*c).remote() as ::core::ffi::c_int != 0 {
                                b" (remote)\0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"\0" as *const u8 as *const ::core::ffi::c_char
                            },
                        );
                        fflush(stdout);
                    }
                    if job_counter != 0 {
                        job_counter = job_counter.wrapping_sub(1);
                    }
                }
            }
            _ => {}
        }
        if exit_sig == 0
            && exit_code == 127
            && !(*c).cmd_name.is_null()
        {
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
            let mut r: ::core::ffi::c_int;
            loop {
                r = stat((*c).cmd_name, &raw mut st);
                if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
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
                    ::core::ptr::null_mut::<Floc>(),
                    (strlen((*c).cmd_name) as size_t).wrapping_add(strlen(e) as size_t),
                    b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*c).cmd_name,
                    e,
                );
            }
        }
        if exit_sig == 0 && exit_code == 0 {
            child_failed = MAKE_SUCCESS;
        } else if exit_sig == 0
            && exit_code == 1
            && question_flag != 0
            && (*c).recursive() as ::core::ffi::c_int != 0
        {
            child_failed = MAKE_TROUBLE;
        } else {
            child_failed = MAKE_FAILURE;
        }
        if !(*c).sh_batch_file.is_null() {
            let rm_status: ::core::ffi::c_int;
            if 0x4 as ::core::ffi::c_int & db_level != 0 {
                printf(
                    b"Cleaning up temp batch file %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*c).sh_batch_file,
                );
                fflush(stdout);
            }
            *__errno_location() = 0;
            rm_status = remove((*c).sh_batch_file);
            if rm_status != 0 && 0x4 as ::core::ffi::c_int & db_level != 0 {
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
            good_stdin_used = 0;
        }
        dontcare = (*c).dontcare() as ::core::ffi::c_int;
        if child_failed != 0 && (*c).noerror() == 0 && ignore_errors_flag == 0 {
            static mut delete_on_error: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            if dontcare == 0 && child_failed == MAKE_FAILURE {
                child_error(c, exit_code, exit_sig, coredump, 0);
            }
            (*(*c).file).set_update_status(
                (if child_failed == MAKE_FAILURE {
                    us_failed as ::core::ffi::c_int
                } else {
                    us_question as ::core::ffi::c_int
                }) as update_status as update_status,
            );
            if delete_on_error == -(1 as ::core::ffi::c_int) {
                let f: *mut file =
                    lookup_file(b".DELETE_ON_ERROR\0" as *const u8 as *const ::core::ffi::c_char);
                delete_on_error = (!f.is_null() && (*f).is_target() as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int;
            }
            if exit_sig != 0 || delete_on_error != 0 {
                delete_child_targets(c);
            }
        } else {
            if child_failed != 0 {
                child_error(c, exit_code, exit_sig, coredump, 1);
                child_failed = 0;
            }
            if job_next_command(c) != 0 {
                if handling_fatal_signal != 0 {
                    (*(*c).file).set_update_status(us_failed as update_status);
                } else {
                    if output_sync == OUTPUT_SYNC_LINE {
                        crate::output::output_dump(&raw mut (*c).output);
                    }
                    (*c).set_remote(crate::remote_stub::start_remote_job_p(0)
                        as ::core::ffi::c_uint
                        as ::core::ffi::c_uint);
                    start_job_command(c);
                    unblock_sigs();
                    if (*(*c).file).command_state() as ::core::ffi::c_int
                        == cs_running as ::core::ffi::c_int
                    {
                        continue;
                    }
                }
                if (*(*c).file).update_status() as ::core::ffi::c_int
                    != us_success as ::core::ffi::c_int
                {
                    delete_child_targets(c);
                }
            } else {
                (*(*c).file).set_update_status(us_success as update_status);
            }
        }
        crate::output::output_dump(&raw mut (*c).output);
        if handling_fatal_signal == 0 {
            notice_finished_file((*c).file);
        }
        block_sigs();
        if (*c).pid > 0 && 0x4 as ::core::ffi::c_int & db_level != 0 {
            printf(
                b"Removing child %p PID %s%s from chain.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                c,
                pid2str((*c).pid),
                if (*c).remote() as ::core::ffi::c_int != 0 {
                    b" (remote)\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            fflush(stdout);
        }
        if job_slots_used > 0 {
            job_slots_used = job_slots_used.wrapping_sub((*c).jobslot());
        }
        if lastc.is_null() {
            children = (*c).next;
        } else {
            (*lastc).next = (*c).next;
        }
        free_child(c);
        unblock_sigs();
        if err == 0
            && child_failed != 0
            && dontcare == 0
            && keep_going_flag == 0
            && handling_fatal_signal == 0
        {
            die(child_failed);
        }
        block = 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn free_childbase(child: *mut childbase) {
    if !(*child).environment.is_null() {
        let mut ep: *mut *mut ::core::ffi::c_char = (*child).environment;
        while !(*ep).is_null() {
            let fresh9 = ep;
            ep = ep.offset(1 as ::core::ffi::c_int as isize);
            free(*fresh9 as *mut ::core::ffi::c_void);
        }
        free((*child).environment as *mut ::core::ffi::c_void);
    }
    free((*child).cmd_name as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn free_child(child: *mut child) {
    crate::output::output_close(&raw mut (*child).output);
    if jobserver_tokens == 0 {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            INTSTR_LENGTH.wrapping_add(strlen((*(*child).file).name) as size_t),
            b"INTERNAL: freeing child %p (%s) but no tokens left\0" as *const u8
                as *const ::core::ffi::c_char,
            child,
            (*(*child).file).name,
        );
    }
    if jobserver_enabled() != 0 && jobserver_tokens > 1 {
        jobserver_release(1);
        if 0x4 as ::core::ffi::c_int & db_level != 0 {
            printf(
                b"Released token for child %p (%s).\n\0" as *const u8 as *const ::core::ffi::c_char,
                child,
                (*(*child).file).name,
            );
            fflush(stdout);
        }
    }
    jobserver_tokens = jobserver_tokens.wrapping_sub(1);
    if handling_fatal_signal != 0 {
        return;
    }
    if !(*child).command_lines.is_null() {
        let mut i: ::core::ffi::c_uint;
        i = 0;
        while i < (*(*(*child).file).cmds).ncommand_lines as ::core::ffi::c_uint {
            free(*(*child).command_lines.offset(i as isize) as *mut ::core::ffi::c_void);
            i = i.wrapping_add(1);
        }
        free((*child).command_lines as *mut ::core::ffi::c_void);
    }
    free_childbase(child as *mut childbase);
    free(child as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn start_job_command(mut child: *mut child) {
    let mut flags: ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_char;
    let mut argv: *mut *mut ::core::ffi::c_char;
    if !(*child).command_ptr.is_null() {
        flags = (*(*child).file).command_flags
            | *(*(*(*child).file).cmds)
                .lines_flags
                .offset((*child).command_line.wrapping_sub(1) as isize)
                as ::core::ffi::c_int;
        p = (*child).command_ptr;
        (*child).set_noerror(
            (flags & 4 != 0) as ::core::ffi::c_int
                as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        while *p as ::core::ffi::c_int != 0 {
            if *p as ::core::ffi::c_int == '@' as i32 {
                flags |= COMMANDS_SILENT;
            } else if *p as ::core::ffi::c_int == '+' as i32 {
                flags |= COMMANDS_RECURSE;
            } else if *p as ::core::ffi::c_int == '-' as i32 {
                (*child).set_noerror(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else if !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                .offset(*p as ::core::ffi::c_uchar as isize)
                as ::core::ffi::c_int
                & 0x2 as ::core::ffi::c_int
                != 0)
            {
                break;
            }
            p = p.offset(1 as ::core::ffi::c_int as isize);
        }
        (*child).set_recursive(
            (flags & 1 != 0) as ::core::ffi::c_int
                as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        let fresh10 = &mut (*(*(*(*child).file).cmds)
            .lines_flags
            .offset((*child).command_line.wrapping_sub(1) as isize));
        *fresh10 =
            (*fresh10 as ::core::ffi::c_int | flags & COMMANDS_RECURSE) as ::core::ffi::c_uchar;
        let prefix: ::core::ffi::c_char = (*(*(*child).file).cmds).recipe_prefix;
        let mut p1: *mut ::core::ffi::c_char;
        let mut p2: *mut ::core::ffi::c_char;
        p2 = p;
        p1 = p2;
        while *p1 as ::core::ffi::c_int != 0 {
            let fresh11 = p2;
            p2 = p2.offset(1 as ::core::ffi::c_int as isize);
            *fresh11 = *p1;
            if *p1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32
                && *p1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == prefix as ::core::ffi::c_int
            {
                p1 = p1.offset(1 as ::core::ffi::c_int as isize);
            }
            p1 = p1.offset(1 as ::core::ffi::c_int as isize);
        }
        *p2 = *p1;
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        argv = construct_command_argv(
            p,
            &raw mut end,
            (*child).file,
            *(*(*(*child).file).cmds)
                .lines_flags
                .offset((*child).command_line.wrapping_sub(1) as isize)
                as ::core::ffi::c_int
                | (*(*child).file).command_flags,
            &raw mut (*child).sh_batch_file,
        );
        if end.is_null() {
            (*child).command_ptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            let fresh12 = end;
            end = end.offset(1 as ::core::ffi::c_int as isize);
            *fresh12 = 0;
            (*child).command_ptr = end;
        }
        if !argv.is_null()
            && question_flag != 0
            && !(flags & 1 != 0)
        {
            if !argv.is_null() {
                free(*argv.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void);
                free(argv as *mut ::core::ffi::c_void);
            }
            (*(*child).file).set_update_status(us_question as update_status);
            notice_finished_file((*child).file);
            return;
        }
        if touch_flag != 0 && !(flags & 1 != 0) {
            if !argv.is_null() {
                free(*argv.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void);
                free(argv as *mut ::core::ffi::c_void);
            }
            argv = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        }
        if !argv.is_null() {
            (*child).output.set_syncout(
                (output_sync != 0
                    && (output_sync == OUTPUT_SYNC_RECURSE
                        || !(flags & 1 != 0)))
                    as ::core::ffi::c_int as ::core::ffi::c_uint
                    as ::core::ffi::c_uint,
            );
            output_context = if (*child).output.syncout() as ::core::ffi::c_int != 0 {
                &raw mut (*child).output
            } else {
                ::core::ptr::null_mut::<output>()
            };
            if (*child).output.syncout() == 0 {
                crate::output::output_dump(&raw mut (*child).output);
            }
            if just_print_flag != 0
                || 0x10 as ::core::ffi::c_int & db_level != 0
                || !(flags & 2 != 0) && run_silent == 0
            {
                message(
                    0,
                    strlen(p) as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    p,
                );
            }
            commands_started = commands_started.wrapping_add(1);
            if !(*argv.offset(0 as ::core::ffi::c_int as isize)).is_null()
                && is_bourne_compatible_shell(*argv.offset(0 as ::core::ffi::c_int as isize)) != 0
                && (!(*argv.offset(1 as ::core::ffi::c_int as isize)).is_null()
                    && *(*argv.offset(1 as ::core::ffi::c_int as isize)).offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
                    && (*(*argv.offset(1 as ::core::ffi::c_int as isize)).offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 'c' as i32
                        && *(*argv.offset(1 as ::core::ffi::c_int as isize)).offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0
                        || *(*argv.offset(1 as ::core::ffi::c_int as isize)).offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 'e' as i32
                            && *(*argv.offset(1 as ::core::ffi::c_int as isize)).offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 'c' as i32
                            && *(*argv.offset(1 as ::core::ffi::c_int as isize)).offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 0))
                && (!(*argv.offset(2 as ::core::ffi::c_int as isize)).is_null()
                    && *(*argv.offset(2 as ::core::ffi::c_int as isize)).offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == ':' as i32
                    && *(*argv.offset(2 as ::core::ffi::c_int as isize)).offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 0)
                && (*argv.offset(3 as ::core::ffi::c_int as isize)).is_null()
            {
                if !argv.is_null() {
                    free(*argv.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void);
                    free(argv as *mut ::core::ffi::c_void);
                }
            } else if just_print_flag != 0
                && !(flags & 1 != 0)
            {
                if !argv.is_null() {
                    free(*argv.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void);
                    free(argv as *mut ::core::ffi::c_void);
                }
            } else {
                crate::output::output_start();
                fflush(stdout);
                fflush(stderr);
                (*child).set_good_stdin(
                    (good_stdin_used == 0) as ::core::ffi::c_int as ::core::ffi::c_uint
                        as ::core::ffi::c_uint,
                );
                if (*child).good_stdin() != 0 {
                    good_stdin_used = 1;
                }
                (*child).set_deleted(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                if (*child).environment.is_null() {
                    (*child).environment = target_environment(
                        (*child).file,
                        (*(*(*child).file).cmds).any_recurse() as ::core::ffi::c_int,
                    );
                }
                // Run the job locally unless it is successfully handed off to a
                // remote executor.
                let mut run_local = true;
                if (*child).remote() != 0 {
                    let mut is_remote: ::core::ffi::c_int = 0;
                    let mut used_stdin: ::core::ffi::c_int = 0;
                    let mut id: pid_t = 0;
                    if crate::remote_stub::start_remote_job(
                        argv,
                        (*child).environment,
                        if (*child).good_stdin() as ::core::ffi::c_int != 0 {
                            0
                        } else {
                            get_bad_stdin()
                        },
                        &raw mut is_remote,
                        &raw mut id,
                        &raw mut used_stdin,
                    ) == 0
                    {
                        if (*child).good_stdin() as ::core::ffi::c_int != 0 && used_stdin == 0 {
                            (*child).set_good_stdin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            good_stdin_used = 0;
                        }
                        (*child).set_remote(is_remote as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*child).pid = id;
                        run_local = false;
                    }
                }
                if run_local {
                    block_sigs();
                    (*child).set_remote(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    jobserver_pre_child((flags & 1 != 0) as ::core::ffi::c_int);
                    (*child).pid = child_execute_job(
                        child as *mut childbase,
                        (*child).good_stdin() as ::core::ffi::c_int,
                        argv,
                    );
                    jobserver_post_child((flags & 1 != 0) as ::core::ffi::c_int);
                }
                if (*child).pid >= 0 {
                    job_counter = job_counter.wrapping_add(1);
                }
                set_command_state((*child).file, cs_running);
                if !argv.is_null() {
                    free(*argv.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void);
                    free(argv as *mut ::core::ffi::c_void);
                }
                output_context = ::core::ptr::null_mut::<output>();
                return;
            }
        }
    }
    if job_next_command(child) != 0 {
        start_job_command(child);
    } else {
        set_command_state((*child).file, cs_running);
        (*(*child).file).set_update_status(us_success as update_status);
        notice_finished_file((*child).file);
    }
    output_context = ::core::ptr::null_mut::<output>();
}
#[no_mangle]
pub unsafe extern "C" fn start_waiting_job(mut c: *mut child) -> ::core::ffi::c_int {
    let f: *mut file = (*c).file;
    (*c).set_remote(
        crate::remote_stub::start_remote_job_p(1) as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
    if (*c).remote() == 0 && (job_slots_used > 0 && load_too_high() != 0) {
        set_command_state(f, cs_running);
        (*c).next = waiting_jobs;
        waiting_jobs = c;
        return 0;
    }
    start_job_command(c);
    // Finished states (cs_not_started reset to success, cs_finished) need the
    // file noticed and the child freed; a still-running job does not.
    let mut finish = false;
    match (*f).command_state() as ::core::ffi::c_int {
        2 => {
            (*c).next = children;
            if (*c).pid > 0 {
                if 0x4 as ::core::ffi::c_int & db_level != 0 {
                    printf(
                        b"Putting child %p (%s) PID %s%s on the chain.\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        c,
                        (*(*c).file).name,
                        pid2str((*c).pid),
                        if (*c).remote() as ::core::ffi::c_int != 0 {
                            b" (remote)\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                    );
                    fflush(stdout);
                }
                job_slots_used = job_slots_used.wrapping_add(1);
                if (*c).jobslot() as ::core::ffi::c_int == 0 {
                    } else {
                        __assert_fail(
                            b"c->jobslot == 0\0" as *const u8 as *const ::core::ffi::c_char,
                            b"src/job.c\0" as *const u8 as *const ::core::ffi::c_char,
                            1625 as ::core::ffi::c_uint,
                            b"int start_waiting_job(struct child *)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    };
                (*c).set_jobslot(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            children = c;
            unblock_sigs();
        }
        0 => {
            (*f).set_update_status(us_success as update_status);
            finish = true;
        }
        3 => {
            finish = true;
        }
        _ => {
            if (*f).command_state() as ::core::ffi::c_int == cs_finished as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"f->command_state == cs_finished\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"src/job.c\0" as *const u8 as *const ::core::ffi::c_char,
                        1643 as ::core::ffi::c_uint,
                        b"int start_waiting_job(struct child *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                };
        }
    }
    if finish {
        notice_finished_file(f);
        free_child(c);
    }
    1
}
#[no_mangle]
pub unsafe extern "C" fn new_job(file: *mut file) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut cmds: *mut commands = (*file).cmds;
    let mut c: *mut child;
    let lines: *mut *mut ::core::ffi::c_char;
    let mut i: ::core::ffi::c_uint;
    start_waiting_jobs();
    reap_children(0, 0);
    chop_commands(cmds);
    c = xcalloc(::core::mem::size_of::<child>() as size_t) as *mut child;
    crate::output::output_init(&raw mut (*c).output);
    (*c).file = file;
    (*c).sh_batch_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*c).set_dontcare((*file).dontcare() as ::core::ffi::c_uint);
    output_context = if (*c).output.syncout() as ::core::ffi::c_int != 0 {
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
        let mut in_0: *mut ::core::ffi::c_char;
        let mut out: *mut ::core::ffi::c_char;
        let mut ref_0: *mut ::core::ffi::c_char;
        out = *(*cmds).command_lines.offset(i as isize);
        in_0 = out;
        loop {
            ref_0 = strchr(in_0, '$' as i32);
            if ref_0.is_null() {
                break;
            }
            ref_0 = ref_0.offset(1 as ::core::ffi::c_int as isize);
            if out != in_0 {
                memmove(
                    out as *mut ::core::ffi::c_void,
                    in_0 as *const ::core::ffi::c_void,
                    ref_0.offset_from(in_0) as ::core::ffi::c_long as size_t,
                );
            }
            out = out.offset(ref_0.offset_from(in_0) as ::core::ffi::c_long as isize);
            in_0 = ref_0;
            if *ref_0 as ::core::ffi::c_int == '(' as i32
                || *ref_0 as ::core::ffi::c_int == '{' as i32
            {
                let openparen: ::core::ffi::c_char = *ref_0;
                let closeparen: ::core::ffi::c_char =
                    (if openparen as ::core::ffi::c_int == '(' as i32 {
                        ')' as i32
                    } else {
                        '}' as i32
                    }) as ::core::ffi::c_char;
                let outref: *mut ::core::ffi::c_char;
                let mut count: ::core::ffi::c_int;
                let mut p: *mut ::core::ffi::c_char;
                let fresh0 = in_0;
                in_0 = in_0.offset(1 as ::core::ffi::c_int as isize);
                let fresh1 = out;
                out = out.offset(1 as ::core::ffi::c_int as isize);
                *fresh1 = *fresh0;
                outref = out;
                count = 0;
                while *in_0 as ::core::ffi::c_int != 0 {
                    if *in_0 as ::core::ffi::c_int == '\\' as i32
                        && *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32
                    {
                        let mut quoted: ::core::ffi::c_int = 0;
                        p = in_0.offset(-(1 as ::core::ffi::c_int as isize));
                        while p > ref_0 && *p as ::core::ffi::c_int == '\\' as i32 {
                            quoted = (quoted == 0) as ::core::ffi::c_int;
                            p = p.offset(-(1 as ::core::ffi::c_int) as isize);
                        }
                        if quoted != 0 {
                            let fresh2 = in_0;
                            in_0 = in_0.offset(1 as ::core::ffi::c_int as isize);
                            let fresh3 = out;
                            out = out.offset(1 as ::core::ffi::c_int as isize);
                            *fresh3 = *fresh2;
                        } else {
                            in_0 = in_0.offset(2 as ::core::ffi::c_int as isize);
                            while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                                .offset(*in_0 as ::core::ffi::c_uchar as isize)
                                as ::core::ffi::c_int
                                & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                                != 0
                            {
                                in_0 = in_0.offset(1 as ::core::ffi::c_int as isize);
                            }
                            while out > outref
                                && *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort).offset(
                                    *out.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar as isize,
                                ) as ::core::ffi::c_int
                                    & 0x2 as ::core::ffi::c_int
                                    != 0
                            {
                                out = out.offset(-(1 as ::core::ffi::c_int) as isize);
                            }
                            let fresh4 = out;
                            out = out.offset(1 as ::core::ffi::c_int as isize);
                            *fresh4 = ' ' as i32 as ::core::ffi::c_char;
                        }
                    } else {
                        if *in_0 as ::core::ffi::c_int == closeparen as ::core::ffi::c_int && {
                            count -= 1;
                            count < 0
                        } {
                            break;
                        }
                        if *in_0 as ::core::ffi::c_int == openparen as ::core::ffi::c_int {
                            count += 1;
                        }
                        let fresh5 = in_0;
                        in_0 = in_0.offset(1 as ::core::ffi::c_int as isize);
                        let fresh6 = out;
                        out = out.offset(1 as ::core::ffi::c_int as isize);
                        *fresh6 = *fresh5;
                    }
                }
            }
        }
        if out != in_0 {
            memmove(
                out as *mut ::core::ffi::c_void,
                in_0 as *const ::core::ffi::c_void,
                strlen(in_0).wrapping_add(1),
            );
        }
        (*cmds).fileinfo.offset = i as ::core::ffi::c_ulong;
        let fresh7 = &mut (*lines.offset(i as isize));
        *fresh7 = allocated_expand_string_for_file(*(*cmds).command_lines.offset(i as isize), file);
        i = i.wrapping_add(1);
    }
    (*cmds).fileinfo.offset = 0;
    (*c).command_lines = lines;
    job_next_command(c);
    if job_slots != 0 {
        while job_slots_used == job_slots {
            reap_children(1, 0);
        }
    } else if jobserver_enabled() != 0 {
        loop {
            let got_token: ::core::ffi::c_int;
            if 0x4 as ::core::ffi::c_int & db_level != 0 {
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
            if jobserver_tokens == 0 {
                break;
            }
            jobserver_pre_acquire();
            reap_children(0, 0);
            start_waiting_jobs();
            if jobserver_tokens == 0 {
                break;
            }
            if children.is_null() {
                fatal(
                    ::core::ptr::null_mut::<Floc>(),
                    0,
                    b"INTERNAL: no children as we go to sleep on read\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            got_token =
                jobserver_acquire((waiting_jobs != NULL as *mut child) as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            if !(got_token == 1) {
                continue;
            }
            if 0x4 as ::core::ffi::c_int & db_level != 0 {
                printf(
                    b"Obtained token for child %p (%s).\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    c,
                    (*(*c).file).name,
                );
                fflush(stdout);
            }
            break;
        }
    }
    jobserver_tokens = jobserver_tokens.wrapping_add(1);
    if 0x20 as ::core::ffi::c_int & db_level != 0 {
        let mut nmbuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
        if (*(*c).file).also_make.is_null() {
            tp = (*(*c).file).name;
        } else {
            let mut dp: *const dep;
            let mut cp: *mut ::core::ffi::c_char;
            let mut len: size_t = strlen((*(*c).file).name) as size_t;
            dp = (*(*c).file).also_make;
            while !dp.is_null() {
                len = len
                    .wrapping_add(strlen((*(*dp).file).name).wrapping_add(4) as size_t);
                dp = (*dp).next;
            }
            nmbuf = xmalloc(len.wrapping_add(1)) as *mut ::core::ffi::c_char;
            tp = nmbuf;
            cp = stpcpy(nmbuf, (*(*c).file).name);
            dp = (*(*c).file).also_make;
            while !dp.is_null() {
                cp = stpcpy(
                    stpcpy(cp, b"', '\0" as *const u8 as *const ::core::ffi::c_char),
                    (*(*dp).file).name,
                );
                dp = (*dp).next;
            }
        }
        if (*(*c).file).phony() != 0 {
            message(
                0,
                (strlen(nm) as size_t).wrapping_add(strlen(tp) as size_t),
                b"%s: update target '%s' due to: target is .PHONY\0" as *const u8
                    as *const ::core::ffi::c_char,
                nm,
                tp,
            );
        } else if (*(*c).file).last_mtime == NONEXISTENT_MTIME as uintmax_t {
            message(
                0,
                (strlen(nm) as size_t).wrapping_add(strlen(tp) as size_t),
                b"%s: update target '%s' due to: target does not exist\0" as *const u8
                    as *const ::core::ffi::c_char,
                nm,
                tp,
            );
        } else {
            let mut newer: *mut ::core::ffi::c_char = allocated_expand_variable_for_file(
                b"?\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 2]>() as size_t)
                    .wrapping_sub(1),
                (*c).file,
            );
            if *newer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0 {
                message(
                    0,
                    (strlen(nm) as size_t)
                        .wrapping_add(strlen(tp) as size_t)
                        .wrapping_add(strlen(newer) as size_t),
                    b"%s: update target '%s' due to: %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    nm,
                    tp,
                    newer,
                );
                free(newer as *mut ::core::ffi::c_void);
            } else {
                let mut len_0: size_t = 0;
                let mut d: *mut dep;
                d = (*(*c).file).deps;
                while !d.is_null() {
                    if (*(*d).file).last_mtime == NONEXISTENT_MTIME as uintmax_t {
                        len_0 = len_0.wrapping_add(
                            strlen((*(*d).file).name).wrapping_add(1) as size_t,
                        );
                    }
                    d = (*d).next;
                }
                if len_0 == 0 {
                    message(
                        0,
                        (strlen(nm) as size_t).wrapping_add(strlen(tp) as size_t),
                        b"%s: update target '%s' due to: unknown reasons\0" as *const u8
                            as *const ::core::ffi::c_char,
                        nm,
                        tp,
                    );
                } else {
                    alloca_allocations.push(::std::vec::from_elem(0, len_0 as usize));
                    newer = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                        as *mut ::core::ffi::c_char;
                    let mut cp_0: *mut ::core::ffi::c_char = newer;
                    d = (*(*c).file).deps;
                    while !d.is_null() {
                        if (*(*d).file).last_mtime == NONEXISTENT_MTIME as uintmax_t {
                            if cp_0 > newer {
                                let fresh8 = cp_0;
                                cp_0 = cp_0.offset(1 as ::core::ffi::c_int as isize);
                                *fresh8 = ' ' as i32 as ::core::ffi::c_char;
                            }
                            cp_0 = stpcpy(cp_0, (*(*d).file).name);
                        }
                        d = (*d).next;
                    }
                    message(
                        0,
                        (strlen(nm) as size_t)
                            .wrapping_add(strlen(tp) as size_t)
                            .wrapping_add(strlen(newer) as size_t),
                        b"%s: update target '%s' due to: %s\0" as *const u8
                            as *const ::core::ffi::c_char,
                        nm,
                        tp,
                        newer,
                    );
                }
            }
        }
        free(nmbuf as *mut ::core::ffi::c_void);
    }
    start_waiting_job(c);
    if job_slots == 1 || not_parallel != 0 {
        while (*file).command_state() as ::core::ffi::c_int == cs_running as ::core::ffi::c_int {
            reap_children(1, 0);
        }
    }
    output_context = ::core::ptr::null_mut::<output>();
}
#[no_mangle]
pub unsafe extern "C" fn job_next_command(mut child: *mut child) -> ::core::ffi::c_int {
    while (*child).command_ptr.is_null()
        || *(*child).command_ptr as ::core::ffi::c_int == 0
    {
        if (*child).command_line == (*(*(*child).file).cmds).ncommand_lines as ::core::ffi::c_uint {
            (*child).command_ptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*(*(*child).file).cmds).fileinfo.offset = 0;
            return 0;
        } else {
            let fresh15 = (*child).command_line;
            (*child).command_line = (*child).command_line.wrapping_add(1);
            (*child).command_ptr = *(*child).command_lines.offset(fresh15 as isize);
        }
    }
    (*(*(*child).file).cmds).fileinfo.offset =
        (*child).command_line.wrapping_sub(1) as ::core::ffi::c_ulong;
    1
}
pub const LOAD_WEIGHT_A: ::core::ffi::c_double = 0.25f64;
pub const LOAD_WEIGHT_B: ::core::ffi::c_double = 0.25f64;
#[no_mangle]
pub unsafe extern "C" fn load_too_high() -> ::core::ffi::c_int {
    static mut last_sec: ::core::ffi::c_double = 0.;
    static mut last_now: time_t = 0;
    static mut proc_fd: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
    let mut load: ::core::ffi::c_double = 0.;
    let guess: ::core::ffi::c_double;
    let now: time_t;
    if max_load_average < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return 0;
    }
    if proc_fd == -(2 as ::core::ffi::c_int) {
        loop {
            proc_fd = open(
                b"/proc/loadavg\0" as *const u8 as *const ::core::ffi::c_char,
                0,
            );
            if !(proc_fd == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                break;
            }
        }
        if proc_fd < 0 {
            if 0x4 as ::core::ffi::c_int & db_level != 0 {
                printf(
                    b"Using system load detection method.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fflush(stdout);
            }
        } else {
            if 0x4 as ::core::ffi::c_int & db_level != 0 {
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
        let mut r: ::core::ffi::c_int;
        loop {
            r = lseek(proc_fd, 0 as __off_t, 0) as ::core::ffi::c_int;
            if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
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
                ) as ::core::ffi::c_int;
                if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                    break;
                }
            }
            if r >= 0 {
                let mut p: *const ::core::ffi::c_char;
                avg[r as usize] = 0;
                p = strchr(&raw mut avg as *mut ::core::ffi::c_char, ' ' as i32);
                if !p.is_null() {
                    p = strchr(p.offset(1 as ::core::ffi::c_int as isize), ' ' as i32);
                }
                if !p.is_null() {
                    p = strchr(p.offset(1 as ::core::ffi::c_int as isize), ' ' as i32);
                }
                if !p.is_null()
                    && (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_sub('0' as i32 as ::core::ffi::c_uint)
                        <= 9
                {
                    let cnt: ::core::ffi::c_uint = make_toui(
                        p.offset(1 as ::core::ffi::c_int as isize),
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    );
                    if 0x4 as ::core::ffi::c_int & db_level != 0 {
                        printf(
                            b"Running: system = %u / make = %u (max requested = %f)\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            cnt,
                            job_slots_used,
                            max_load_average,
                        );
                        fflush(stdout);
                    }
                    return (cnt as ::core::ffi::c_double > max_load_average) as ::core::ffi::c_int;
                }
                if 0x4 as ::core::ffi::c_int & db_level != 0 {
                    printf(
                        b"Failed to parse /proc/loadavg: %s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &raw mut avg as *mut ::core::ffi::c_char,
                    );
                    fflush(stdout);
                }
            }
        }
        if r < 0 && 0x4 as ::core::ffi::c_int & db_level != 0 {
            printf(
                b"Failed to read /proc/loadavg: %s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            fflush(stdout);
        }
        close(proc_fd);
        proc_fd = -(1 as ::core::ffi::c_int);
    }
    *__errno_location() = 0;
    if getloadavg(&raw mut load, 1) != 1 {
        static mut lossage: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        if lossage == -(1 as ::core::ffi::c_int) || *__errno_location() != lossage {
            if *__errno_location() == 0 {
                error(
                    ::core::ptr::null_mut::<Floc>(),
                    0,
                    b"cannot enforce load limits on this operating system\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
                perror_with_name(
                    b"cannot enforce load limit: \0" as *const u8 as *const ::core::ffi::c_char,
                    b"getloadavg\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        lossage = *__errno_location();
        load = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    now = time(::core::ptr::null_mut::<time_t>());
    if last_now < now {
        if last_now == now - 1 as time_t {
            last_sec = LOAD_WEIGHT_B * job_counter as ::core::ffi::c_double;
        } else {
            last_sec = 0.0f64;
        }
        job_counter = 0;
        last_now = now;
    }
    guess = load + LOAD_WEIGHT_A * (job_counter as ::core::ffi::c_double + last_sec);
    if 0x4 as ::core::ffi::c_int & db_level != 0 {
        printf(
            b"Estimated system load = %f (actual = %f) (max requested = %f)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            guess,
            load,
            max_load_average,
        );
        fflush(stdout);
    }
    (guess >= max_load_average) as ::core::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn start_waiting_jobs() {
    let mut job: *mut child;
    if waiting_jobs.is_null() {
        return;
    }
    loop {
        reap_children(0, 0);
        job = waiting_jobs;
        waiting_jobs = (*job).next;
        if !(start_waiting_job(job) != 0 && !waiting_jobs.is_null()) {
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
#[no_mangle]
pub unsafe extern "C" fn child_execute_job(
    child: *mut childbase,
    good_stdin: ::core::ffi::c_int,
    argv: *mut *mut ::core::ffi::c_char,
) -> pid_t {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let fdin: ::core::ffi::c_int = if good_stdin != 0 {
        fileno(stdin) as ::core::ffi::c_int
    } else {
        get_bad_stdin() as ::core::ffi::c_int
    };
    let mut fdout: ::core::ffi::c_int = fileno(stdout);
    let mut fderr: ::core::ffi::c_int = fileno(stderr);
    if (*child).output.syncout() != 0 {
        if (*child).output.out >= 0 {
            fdout = (*child).output.out;
        }
        if (*child).output.err >= 0 {
            fderr = (*child).output.err;
        }
    }
    let mut pid: pid_t = -(1 as pid_t);
    let r = spawn_child(child, argv, fdin, fdout, fderr, &raw mut pid, &mut alloca_allocations);
    if r != 0 {
        pid = -(1 as ::core::ffi::c_int) as pid_t;
    }
    if pid < 0 {
        error(
            ::core::ptr::null_mut::<Floc>(),
            (strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as size_t)
                .wrapping_add(strlen(strerror(r)) as size_t),
            b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            strerror(r),
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
    fdin: ::core::ffi::c_int,
    fdout: ::core::ffi::c_int,
    fderr: ::core::ffi::c_int,
    pid: *mut pid_t,
    alloca_allocations: &mut Vec<Vec<u8>>,
) -> ::core::ffi::c_int {
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
        if *(*pp).offset(0) as ::core::ffi::c_int == 'P' as i32
            && *(*pp).offset(1) as ::core::ffi::c_int == 'A' as i32
            && *(*pp).offset(2) as ::core::ffi::c_int == 'T' as i32
            && *(*pp).offset(3) as ::core::ffi::c_int == 'H' as i32
            && *(*pp).offset(4) as ::core::ffi::c_int == '=' as i32
        {
            p = (*pp).offset(5);
            break;
        }
        pp = pp.offset(1);
    }
    if p.is_null() {
        let l: size_t = confstr(
            _CS_PATH as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0,
        ) as size_t;
        if l != 0 {
            alloca_allocations.push(::std::vec::from_elem(0, l as usize));
            let dp: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            confstr(_CS_PATH as ::core::ffi::c_int, dp, l as size_t);
            p = dp;
        }
    }
    let cmd: *mut ::core::ffi::c_char =
        find_in_given_path(*argv.offset(0), p, ::core::ptr::null::<::core::ffi::c_char>(), false)
            as *mut ::core::ffi::c_char;
    if cmd.is_null() {
        return *__errno_location();
    }
    loop {
        r = posix_spawn(pid, cmd, &raw mut fa, &raw mut attr, argv, (*child).environment);
        if r != EINTR {
            break;
        }
    }
    if r == ENOEXEC {
        // Not a directly executable file: retry it as an argument to the shell.
        let mut l_0: size_t = 0;
        let mut pp_0: *mut *mut ::core::ffi::c_char = argv;
        while !(*pp_0).is_null() {
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
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t).wrapping_mul(l_0 as size_t),
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
#[no_mangle]
pub unsafe extern "C" fn exec_command(
    argv: *mut *mut ::core::ffi::c_char,
    envp: *mut *mut ::core::ffi::c_char,
) -> pid_t {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let pid: pid_t = -(1 as pid_t);
    environ = envp;
    execvp(*argv.offset(0 as ::core::ffi::c_int as isize), argv as *const *mut ::core::ffi::c_char,
    );
    match *__errno_location() {
        ENOENT => {
            error(
                ::core::ptr::null_mut::<Floc>(),
                (strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                strerror(*__errno_location()),
            );
        }
        ENOEXEC => {
            let mut shell: *const ::core::ffi::c_char;
            let new_argv: *mut *mut ::core::ffi::c_char;
            let mut argc: ::core::ffi::c_int;
            let i: ::core::ffi::c_int = 1;
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
            let fresh49 = &mut (*new_argv.offset(0 as ::core::ffi::c_int as isize));
            *fresh49 = shell as *mut ::core::ffi::c_char;
            let fresh50 = &mut (*new_argv.offset(i as isize));
            *fresh50 = *argv.offset(0 as ::core::ffi::c_int as isize);
            while argc > 0 {
                let fresh51 = &mut (*new_argv.offset((i + argc) as isize));
                *fresh51 = *argv.offset(argc as isize);
                argc -= 1;
            }
            execvp(shell, new_argv as *const *mut ::core::ffi::c_char);
            error(
                ::core::ptr::null_mut::<Floc>(),
                (strlen(*new_argv.offset(0 as ::core::ffi::c_int as isize)) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                *new_argv.offset(0 as ::core::ffi::c_int as isize),
                strerror(*__errno_location()),
            );
        }
        _ => {
            error(
                ::core::ptr::null_mut::<Floc>(),
                (strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                strerror(*__errno_location()),
            );
        }
    }
    pid
}
unsafe extern "C" fn construct_command_argv_internal(
    mut line: *mut ::core::ffi::c_char,
    restp: *mut *mut ::core::ffi::c_char,
    mut shell: *const ::core::ffi::c_char,
    shellflags: *const ::core::ffi::c_char,
    ifs: *const ::core::ffi::c_char,
    flags: ::core::ffi::c_int,
    mut _batch_filename: *mut *mut ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut current_block: u64;
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
    let mut instring: ::core::ffi::c_int;
    let mut word_has_equals: ::core::ffi::c_int;
    let mut seen_nonequals: ::core::ffi::c_int;
    let mut last_argument_was_empty: ::core::ffi::c_int;
    let mut new_argv: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut argstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !restp.is_null() {
        *restp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*line as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & 0x2 as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1 as ::core::ffi::c_int as isize);
    }
    if *line as ::core::ffi::c_int == 0 {
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    if shell.is_null() {
        shell = default_shell;
        current_block = 2968425633554183086;
    } else if strcmp(shell, default_shell) != 0 {
        current_block = 16789764818708874114;
    } else {
        current_block = 2968425633554183086;
    }
    match current_block {
        2968425633554183086 => {
            if !ifs.is_null() {
                cap = ifs;
                loop {
                    if !(*cap as ::core::ffi::c_int != 0) {
                        current_block = 9606288038608642794;
                        break;
                    }
                    if *cap as ::core::ffi::c_int != ' ' as i32
                        && *cap as ::core::ffi::c_int != '\t' as i32
                        && *cap as ::core::ffi::c_int != '\n' as i32
                    {
                        current_block = 16789764818708874114;
                        break;
                    }
                    cap = cap.offset(1 as ::core::ffi::c_int as isize);
                }
            } else {
                current_block = 9606288038608642794;
            }
            match current_block {
                16789764818708874114 => {}
                _ => {
                    if !shellflags.is_null() {
                        if *shellflags.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32
                            || (*shellflags.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'c' as i32
                                || *shellflags.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    != 0)
                                && (*shellflags.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'e' as i32
                                    || *shellflags.offset(2 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        != 'c' as i32
                                    || *shellflags.offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        != 0)
                        {
                            current_block = 16789764818708874114;
                        } else {
                            current_block = 4956146061682418353;
                        }
                    } else {
                        current_block = 4956146061682418353;
                    }
                    match current_block {
                        16789764818708874114 => {}
                        _ => {
                            i = strlen(line).wrapping_add(1) as size_t;
                            new_argv =
                                xmalloc(i.wrapping_mul(::core::mem::size_of::<
                                    *mut ::core::ffi::c_char,
                                >()
                                    as size_t))
                                    as *mut *mut ::core::ffi::c_char;
                            argstr = xmalloc(i) as *mut ::core::ffi::c_char;
                            let fresh16 = &mut (*new_argv.offset(0 as ::core::ffi::c_int as isize));
                            *fresh16 = argstr;
                            ap = *fresh16;
                            end = ap.offset(i as isize);
                            i = 0;
                            last_argument_was_empty = 0;
                            seen_nonequals = last_argument_was_empty;
                            word_has_equals = seen_nonequals;
                            instring = word_has_equals;
                            p = line;
                            's_107: loop {
                                if !(*p as ::core::ffi::c_int != 0) {
                                    current_block = 16740858295659012994;
                                    break;
                                }
                                if ap <= end {
                                    } else {
                                        __assert_fail(
                                            b"ap <= end\0" as *const u8 as *const ::core::ffi::c_char,
                                            b"src/job.c\0" as *const u8 as *const ::core::ffi::c_char,
                                            2938 as ::core::ffi::c_uint,
                                            b"char **construct_command_argv_internal(char *, char **, const char *, const char *, const char *, int, char **)\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                        );
                                    };
                                if instring != 0 {
                                    if *p as ::core::ffi::c_int == instring {
                                        instring = 0;
                                        if ap == *new_argv.offset(0 as ::core::ffi::c_int as isize)
                                            || *ap.offset(-(1 as ::core::ffi::c_int as isize))
                                                as ::core::ffi::c_int
                                                == 0
                                        {
                                            last_argument_was_empty = 1;
                                        }
                                    } else if *p as ::core::ffi::c_int == '\\' as i32
                                        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32
                                    {
                                        if instring == '"' as i32 {
                                            p = p.offset(1 as ::core::ffi::c_int as isize);
                                        } else {
                                            let fresh17 = p;
                                            p = p.offset(1 as ::core::ffi::c_int as isize);
                                            let fresh18 = ap;
                                            ap = ap.offset(1 as ::core::ffi::c_int as isize);
                                            *fresh18 = *fresh17;
                                            let fresh19 = ap;
                                            ap = ap.offset(1 as ::core::ffi::c_int as isize);
                                            *fresh19 = *p;
                                        }
                                    } else if *p as ::core::ffi::c_int == '\n' as i32
                                        && !restp.is_null()
                                    {
                                        *restp = p;
                                        current_block = 16740858295659012994;
                                        break;
                                    } else {
                                        if instring == '"' as i32
                                            && !strchr(
                                                b"\\$`\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                *p as ::core::ffi::c_int,
                                            )
                                            .is_null()
                                            && unixy_shell != 0
                                        {
                                            current_block = 16789764818708874114;
                                            break;
                                        }
                                        let fresh20 = ap;
                                        ap = ap.offset(1 as ::core::ffi::c_int as isize);
                                        *fresh20 = *p;
                                    }
                                } else {
                                    if !strchr(sh_chars, *p as ::core::ffi::c_int).is_null() {
                                        current_block = 16789764818708874114;
                                        break;
                                    }
                                    if one_shell != 0 && *p as ::core::ffi::c_int == '\n' as i32 {
                                        current_block = 16789764818708874114;
                                        break;
                                    }
                                    match *p as ::core::ffi::c_int {
                                        61 => {
                                            if seen_nonequals == 0 && unixy_shell != 0 {
                                                current_block = 16789764818708874114;
                                                break;
                                            }
                                            word_has_equals = 1;
                                            let fresh21 = ap;
                                            ap = ap.offset(1 as ::core::ffi::c_int as isize);
                                            *fresh21 = '=' as i32 as ::core::ffi::c_char;
                                        }
                                        92 => {
                                            if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32
                                            {
                                                p = p.offset(1 as ::core::ffi::c_int as isize);
                                                if ap == *new_argv.offset(i as isize) {
                                                    while *(&raw mut stopchar_map
                                                        as *mut ::core::ffi::c_ushort)
                                                        .offset(*p.offset(
                                                            1 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_uchar
                                                            as isize)
                                                        as ::core::ffi::c_int
                                                        & 0x2 as ::core::ffi::c_int
                                                        != 0
                                                    {
                                                        p = p.offset(1 as ::core::ffi::c_int as isize);
                                                    }
                                                }
                                            } else if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
                                            {
                                                p = p.offset(1 as ::core::ffi::c_int as isize);
                                                let fresh22 = ap;
                                                ap = ap.offset(1 as ::core::ffi::c_int as isize);
                                                *fresh22 = *p;
                                            }
                                        }
                                        39 | 34 => {
                                            instring = *p as ::core::ffi::c_int;
                                        }
                                        10 => {
                                            if !restp.is_null() {
                                                *restp = p;
                                                current_block = 16740858295659012994;
                                                break;
                                            } else {
                                                let fresh23 = ap;
                                                ap = ap.offset(1 as ::core::ffi::c_int as isize);
                                                *fresh23 = '\n' as i32 as ::core::ffi::c_char;
                                            }
                                        }
                                        32 | 9 => {
                                            let fresh24 = ap;
                                            ap = ap.offset(1 as ::core::ffi::c_int as isize);
                                            *fresh24 = 0;
                                            i = i.wrapping_add(1);
                                            let fresh25 = &mut (*new_argv.offset(i as isize));
                                            *fresh25 = ap;
                                            last_argument_was_empty = 0;
                                            seen_nonequals |=
                                                (word_has_equals == 0) as ::core::ffi::c_int;
                                            if word_has_equals != 0 && seen_nonequals == 0 {
                                                current_block = 16789764818708874114;
                                                break;
                                            }
                                            word_has_equals = 0;
                                            if i == 1 {
                                                let mut j: ::core::ffi::c_int;
                                                j = 0;
                                                while !sh_cmds[j as usize].is_null() {
                                                    if *sh_cmds[j as usize] as ::core::ffi::c_int
                                                        == **new_argv.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                        && (*sh_cmds[j as usize]
                                                            as ::core::ffi::c_int
                                                            == 0
                                                            || strcmp(
                                                                sh_cmds[j as usize].offset(
                                                                    1 as ::core::ffi::c_int
                                                                        as isize,
                                                                ),
                                                                (*new_argv.offset(
                                                                    0 as ::core::ffi::c_int
                                                                        as isize,
                                                                ))
                                                                .offset(
                                                                    1 as ::core::ffi::c_int
                                                                        as isize,
                                                                ),
                                                            ) == 0)
                                                    {
                                                        current_block = 16789764818708874114;
                                                        break 's_107;
                                                    }
                                                    j += 1;
                                                }
                                            }
                                            while *(&raw mut stopchar_map
                                                as *mut ::core::ffi::c_ushort)
                                                .offset(*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as isize)
                                                as ::core::ffi::c_int
                                                & 0x2 as ::core::ffi::c_int
                                                != 0
                                            {
                                                p = p.offset(1 as ::core::ffi::c_int as isize);
                                            }
                                        }
                                        _ => {
                                            let fresh26 = ap;
                                            ap = ap.offset(1 as ::core::ffi::c_int as isize);
                                            *fresh26 = *p;
                                        }
                                    }
                                }
                                p = p.offset(1 as ::core::ffi::c_int as isize);
                            }
                            match current_block {
                                16789764818708874114 => {}
                                _ => {
                                    if !(instring != 0) {
                                        *ap = 0;
                                        if *(*new_argv.offset(i as isize)).offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            != 0
                                            || last_argument_was_empty != 0
                                        {
                                            i = i.wrapping_add(1);
                                        }
                                        let fresh27 = &mut (*new_argv.offset(i as isize));
                                        *fresh27 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                                        if i == 1 {
                                            let mut j_0: ::core::ffi::c_int;
                                            j_0 = 0;
                                            loop {
                                                if sh_cmds[j_0 as usize].is_null() {
                                                    current_block = 6002151390280567665;
                                                    break;
                                                }
                                                if *sh_cmds[j_0 as usize] as ::core::ffi::c_int
                                                    == **new_argv.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                                    && (*sh_cmds[j_0 as usize]
                                                        as ::core::ffi::c_int
                                                        == 0
                                                        || strcmp(
                                                            sh_cmds[j_0 as usize].offset(
                                                                1 as ::core::ffi::c_int as isize,
                                                            ),
                                                            (*new_argv.offset(
                                                                0 as ::core::ffi::c_int as isize,
                                                            ))
                                                            .offset(
                                                                1 as ::core::ffi::c_int as isize,
                                                            ),
                                                        ) == 0)
                                                {
                                                    current_block = 16789764818708874114;
                                                    break;
                                                }
                                                j_0 += 1;
                                            }
                                        } else {
                                            current_block = 6002151390280567665;
                                        }
                                        match current_block {
                                            16789764818708874114 => {}
                                            _ => {
                                                if (*new_argv.offset(0 as ::core::ffi::c_int as isize)).is_null()
                                                {
                                                    free(argstr as *mut ::core::ffi::c_void);
                                                    free(new_argv as *mut ::core::ffi::c_void);
                                                    return ::core::ptr::null_mut::<
                                                        *mut ::core::ffi::c_char,
                                                    >(
                                                    );
                                                }
                                                return new_argv;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
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
    if one_shell != 0 {
        if is_bourne_compatible_shell(shell) != 0 {
            let mut f: *const ::core::ffi::c_char = line;
            let mut t: *mut ::core::ffi::c_char = line;
            while *f.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0 {
                let mut esc: ::core::ffi::c_int = 0;
                while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                    .offset(*f as ::core::ffi::c_uchar as isize)
                    as ::core::ffi::c_int
                    & 0x2 as ::core::ffi::c_int
                    != 0
                    || *f as ::core::ffi::c_int == '-' as i32
                    || *f as ::core::ffi::c_int == '@' as i32
                    || *f as ::core::ffi::c_int == '+' as i32
                {
                    f = f.offset(1 as ::core::ffi::c_int as isize);
                }
                while *f as ::core::ffi::c_int != 0 {
                    let fresh28 = f;
                    f = f.offset(1 as ::core::ffi::c_int as isize);
                    let fresh29 = t;
                    t = t.offset(1 as ::core::ffi::c_int as isize);
                    *fresh29 = *fresh28;
                    if *f.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == '\\' as i32
                    {
                        esc = (esc == 0) as ::core::ffi::c_int;
                    } else {
                        if *f.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == '\n' as i32
                            && esc == 0
                        {
                            break;
                        }
                        esc = 0;
                    }
                }
            }
            *t = 0;
        }
        let mut n: ::core::ffi::c_int = 1;
        let mut nextp: *mut ::core::ffi::c_char;
        new_argv = xmalloc(
            (4 as size_t)
                .wrapping_add(sflags_len.wrapping_div(2))
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
        ) as *mut *mut ::core::ffi::c_char;
        let fresh30 = &mut (*new_argv.offset(0 as ::core::ffi::c_int as isize));
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
            nextp = nextp.offset(1 as ::core::ffi::c_int as isize);
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
                    nextp = stpcpy(nextp, *a).offset(1 as ::core::ffi::c_int as isize);
                    a = a.offset(1 as ::core::ffi::c_int as isize);
                }
                free(*argv.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void);
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
    while *cp as ::core::ffi::c_int != 0 {
        if !strchr(sh_chars, *cp as ::core::ffi::c_int).is_null() {
            let fresh40 = ap;
            ap = ap.offset(1 as ::core::ffi::c_int as isize);
            *fresh40 = '\\' as i32 as ::core::ffi::c_char;
        }
        let fresh41 = ap;
        ap = ap.offset(1 as ::core::ffi::c_int as isize);
        *fresh41 = *cp;
        cp = cp.offset(1 as ::core::ffi::c_int as isize);
    }
    let fresh42 = ap;
    ap = ap.offset(1 as ::core::ffi::c_int as isize);
    *fresh42 = ' ' as i32 as ::core::ffi::c_char;
    if !shellflags.is_null() {
        ap = mempcpy(
            ap as *mut ::core::ffi::c_void,
            shellflags as *const ::core::ffi::c_void,
            sflags_len as size_t,
        ) as *mut ::core::ffi::c_char;
        let fresh43 = ap;
        ap = ap.offset(1 as ::core::ffi::c_int as isize);
        *fresh43 = ' ' as i32 as ::core::ffi::c_char;
    }
    p = line;
    while *p as ::core::ffi::c_int != 0 {
        if !restp.is_null() && *p as ::core::ffi::c_int == '\n' as i32 {
            *restp = p;
            break;
        } else {
            if *p as ::core::ffi::c_int == '\\' as i32
                && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32
            {
                let fresh44 = ap;
                ap = ap.offset(1 as ::core::ffi::c_int as isize);
                *fresh44 = '\\' as i32 as ::core::ffi::c_char;
                if batch_mode_shell == 0 {
                    let fresh45 = ap;
                    ap = ap.offset(1 as ::core::ffi::c_int as isize);
                    *fresh45 = '\\' as i32 as ::core::ffi::c_char;
                }
                let fresh46 = ap;
                ap = ap.offset(1 as ::core::ffi::c_int as isize);
                *fresh46 = '\n' as i32 as ::core::ffi::c_char;
                p = p.offset(1 as ::core::ffi::c_int as isize);
            } else {
                if unixy_shell != 0
                    && batch_mode_shell == 0
                    && (*p as ::core::ffi::c_int == '\\' as i32
                        || *p as ::core::ffi::c_int == '\'' as i32
                        || *p as ::core::ffi::c_int == '"' as i32
                        || *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                            .offset(*p as ::core::ffi::c_uchar as isize)
                            as ::core::ffi::c_int
                            & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                            != 0
                        || !strchr(sh_chars, *p as ::core::ffi::c_int).is_null())
                {
                    let fresh47 = ap;
                    ap = ap.offset(1 as ::core::ffi::c_int as isize);
                    *fresh47 = '\\' as i32 as ::core::ffi::c_char;
                }
                let fresh48 = ap;
                ap = ap.offset(1 as ::core::ffi::c_int as isize);
                *fresh48 = *p;
            }
            p = p.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    if ap
        == new_line
            .offset(shell_len as isize)
            .offset(sflags_len as isize)
            .offset(2 as ::core::ffi::c_int as isize)
    {
        free(new_line as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    *ap = 0;
    if unixy_shell != 0 {
        new_argv = construct_command_argv_internal(
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
            NILF,
            (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t)
                .wrapping_sub(1)
                .wrapping_add(INTSTR_LENGTH),
            b"%s (line %d) Bad shell context (!unixy && !batch_mode_shell)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/job.c\0" as *const u8 as *const ::core::ffi::c_char,
            3621 as ::core::ffi::c_int,
        );
    }
    free(new_line as *mut ::core::ffi::c_void);
    new_argv
}
pub const PRESERVE_BSNL: ::core::ffi::c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn construct_command_argv(
    line: *mut ::core::ffi::c_char,
    restp: *mut *mut ::core::ffi::c_char,
    file: *mut file,
    cmd_flags: ::core::ffi::c_int,
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
        b"SHELL\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
        file,
    );
    var = lookup_variable_for_file(
        b".SHELLFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t).wrapping_sub(1),
        file,
    );
    if var.is_null() {
        shellflags = b"\0" as *const u8 as *const ::core::ffi::c_char;
    } else if (*var).origin() as ::core::ffi::c_int != o_default as ::core::ffi::c_int {
        allocflags = allocated_expand_string_for_file((*var).value, file);
        shellflags = allocflags;
    } else if posix_pedantic != 0
        && ignore_errors_flag == 0
        && !(cmd_flags & 4 != 0)
    {
        shellflags = b"-ec\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        shellflags = b"-c\0" as *const u8 as *const ::core::ffi::c_char;
    }
    ifs = allocated_expand_variable_for_file(
        b"IFS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 4]>() as size_t).wrapping_sub(1),
        file,
    );
    warning::set_action(Type::UndefinedVar, save);
    argv = construct_command_argv_internal(
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
