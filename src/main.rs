use crate::default::{
    define_default_variables, install_default_implicit_rules, install_default_suffix_rules,
    set_default_suffixes, undefine_default_variables,
};
use crate::dir::{hash_init_directories, print_dir_data_base};
pub use crate::ffi_types::{
    __clock_t, __off64_t, __off_t, __pid_t, __sig_atomic_t, __uid_t, pid_t, sig_atomic_t, size_t,
    uintmax_t,
};
use crate::file::{Commands, Dep, File, VariableSet, VariableSetList};
use crate::floc::Floc;
use crate::load::unload_all;
use crate::misc::free_ns_chain;
use crate::misc::{get_tmpdir, get_tmpfile, spin};
use crate::misc::{make_toui, xcalloc, xmalloc, xstrdup};
use crate::read::construct_include_path;
use crate::remote_stub::{remote_cleanup, remote_setup};
use crate::stdio::FILE;
use crate::strcache::strcache_add;
use crate::strcache::{strcache_init, strcache_print_stats};
use crate::variable::print_variable_data_base;
use crate::vpath::{build_vpath_lists, print_vpath_data_base};
use c2rust_bitfields;
use libc;
use libc::{
    __errno_location, _exit, abort, atof, chdir, exit, free, isatty, printf, putchar, putenv,
    setlocale, sprintf, stpcpy, strchr, strcmp, strerror, strrchr, tolower, ttyname, unlink,
};
use std::sync::atomic::{AtomicBool, Ordering};
extern "C" {
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaddset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn sigaction(__sig: i32, __act: *const sigaction, __oact: *mut sigaction) -> i32;
    fn getcwd(__buf: *mut ::core::ffi::c_char, __size: size_t) -> *mut ::core::ffi::c_char;
    static mut environ: *mut *mut ::core::ffi::c_char;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: i32;
    static mut opterr: i32;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::core::ffi::c_char,
        __modes: i32,
        __n: size_t,
    ) -> i32;
    fn fprintf(__stream: *mut FILE, __format: *const ::core::ffi::c_char, ...) -> i32;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> i32;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn feof(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> i32;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> i32;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn getopt_long(
        argc: i32,
        argv: *const *mut ::core::ffi::c_char,
        shortopts: *const ::core::ffi::c_char,
        longopts: *const option,
        longind: *mut i32,
    ) -> i32;
}
pub type __uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [i32; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: i32,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_addr_lsb: ::core::ffi::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: i32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction:
        Option<unsafe extern "C" fn(i32, *mut siginfo_t, *mut ::core::ffi::c_void) -> ()>,
}
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_10 = 8;
pub const _ISpunct: C2RustUnnamed_10 = 4;
pub const _IScntrl: C2RustUnnamed_10 = 2;
pub const _ISblank: C2RustUnnamed_10 = 1;
pub const _ISgraph: C2RustUnnamed_10 = 32768;
pub const _ISprint: C2RustUnnamed_10 = 16384;
pub const _ISspace: C2RustUnnamed_10 = 8192;
pub const _ISxdigit: C2RustUnnamed_10 = 4096;
pub const _ISdigit: C2RustUnnamed_10 = 2048;
pub const _ISalpha: C2RustUnnamed_10 = 1024;
pub const _ISlower: C2RustUnnamed_10 = 512;
pub const _ISupper: C2RustUnnamed_10 = 256;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringlist {
    pub list: *mut *const ::core::ffi::c_char,
    pub idx: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct command_switch {
    pub c: i32,
    pub type_0: C2RustUnnamed_11,
    pub value_ptr: *mut ::core::ffi::c_void,
    #[bitfield(name = "env", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "toenv", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "no_makefile", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "specified", ty = "::core::ffi::c_uint", bits = "3..=3")]
    pub env_toenv_no_makefile_specified: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub noarg_value: *const ::core::ffi::c_void,
    pub default_value: *const ::core::ffi::c_void,
    pub long_name: *const ::core::ffi::c_char,
    pub origin: *mut variable_origin,
}
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const ignore: C2RustUnnamed_11 = 7;
pub const floating: C2RustUnnamed_11 = 6;
pub const positive_int: C2RustUnnamed_11 = 5;
pub const filename: C2RustUnnamed_11 = 4;
pub const strlist: C2RustUnnamed_11 = 3;
pub const string: C2RustUnnamed_11 = 2;
pub const flag_off: C2RustUnnamed_11 = 1;
pub const flag: C2RustUnnamed_11 = 0;
use crate::commands::{fatal_error_signal, handling_fatal_signal};
use crate::expand::{
    expand_string_buf, expand_variable_buf, initialize_variable_output, install_variable_buffer,
    restore_variable_buffer, variable_buffer, variable_buffer_output,
};
pub use crate::file::nameseq;
use crate::file::{
    enter_file, file_timestamp_now, file_timestamp_string, init_hash_files, lookup_file,
    print_file_data_base, print_targets, remove_intermediates, snap_deps, verify_file_data_base,
};
use crate::function::hash_init_function_table;
use crate::guile::guile_gmake_setup;
use crate::job::{
    child_handler, exec_command, job_slots_used, jobserver_tokens, reap_children, JOBSERVER_TOKENS,
};
use crate::load::load_file;
use crate::misc::concat;
pub use crate::output::output;
use crate::output::{
    error, fatal, output_context, perror_with_name, pfatal_with_name, set_stdio_traced,
    stdio_traced,
};
use crate::posixos::{
    check_io_state, jobserver_acquire_all, jobserver_clear, jobserver_enabled, jobserver_get_auth,
    jobserver_parse_auth, jobserver_post_child, jobserver_pre_child, jobserver_release,
    jobserver_setup, osync_clear, osync_get_mutex, osync_parse_mutex, osync_setup,
};
pub use crate::read::goaldep;
use crate::read::{eval_buffer, parse_file_seq, read_all_makefiles, tilde_expand};
use crate::remake::{f_mtime, update_goal_chain};
use crate::remote_stub::remote_description;
use crate::rule::{convert_to_pattern, print_rule_data_base, snap_implicit_rules, suffix_file};
use crate::variable::{
    current_variable_set_list, define_automatic_variables, define_variable_in_set,
    init_hash_global_variable_set, lookup_variable, reset_env_override, try_variable_definition,
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command_variable {
    pub next: *mut command_variable,
    pub variable: *mut variable,
}
pub type variable_scope = ::core::ffi::c_uint;
pub const s_pattern: variable_scope = 2;
pub const s_target: variable_scope = 1;
pub const s_global: variable_scope = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
pub type bsd_signal_ret_t = Option<unsafe extern "C" fn(i32) -> ()>;
pub const SIG_DFL: __sighandler_t = None;
pub const ENOENT: i32 = 2;
pub const EINTR: i32 = 4;
pub const SIGCHLD: i32 = 17;
pub const SIGUSR1: i32 = 10;
pub const SA_RESTART: i32 = 0x10000000_i32;
pub const SIG_SETMASK: i32 = 2;
pub const _IOLBF: i32 = 1;
pub const BUFSIZ: i32 = 8192_i32;
pub const EOF: i32 = -1_i32;
pub const UCHAR_MAX: i32 = __SCHAR_MAX__ * 2 + 1;
pub const CHAR_BIT: i32 = __CHAR_BIT__;
pub const CHAR_MAX: i32 = __SCHAR_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PATH_MAX: i32 = 4096_i32;
pub const GET_PATH_MAX: i32 = PATH_MAX;
pub const EXIT_SUCCESS: i32 = 0;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const __LC_ALL: i32 = 6;
pub const LC_ALL: i32 = __LC_ALL;
pub const DB_NONE: i32 = 0;
pub const DB_BASIC: i32 = 0x1_i32;
pub const DB_VERBOSE: i32 = 0x2_i32;
pub const DB_JOBS: i32 = 0x4_i32;
pub const DB_IMPLICIT: i32 = 0x8_i32;
pub const DB_PRINT: i32 = 0x10_i32;
pub const DB_WHY: i32 = 0x20_i32;
pub const DB_MAKEFILES: i32 = 0x100_i32;
pub const DB_ALL: i32 = 0xfff_i32;
pub const MAP_NUL: i32 = 0x1_i32;
pub const MAP_BLANK: i32 = 0x2_i32;
pub const MAP_NEWLINE: i32 = 0x4_i32;
pub const MAP_COMMENT: i32 = 0x8_i32;
pub const MAP_SEMI: i32 = 0x10_i32;
pub const MAP_EQUALS: i32 = 0x20_i32;
pub const MAP_COLON: i32 = 0x40_i32;
pub const MAP_VARSEP: i32 = 0x80_i32;
pub const MAP_PIPE: i32 = 0x100_i32;
pub const MAP_DOT: i32 = 0x200_i32;
pub const MAP_COMMA: i32 = 0x400_i32;
pub const MAP_USERFUNC: i32 = 0x2000_i32;
pub const MAP_VARIABLE: i32 = 0x4000_i32;
pub const MAP_DIRSEP: i32 = 0x8000_i32;
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const INTSTR_LENGTH: usize = 53_usize
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22_usize)
    .wrapping_add(3_usize);
pub const OUTPUT_SYNC_NONE: i32 = 0;
pub const OUTPUT_SYNC_LINE: i32 = 1;
pub const OUTPUT_SYNC_TARGET: i32 = 2;
pub const OUTPUT_SYNC_RECURSE: i32 = 3;
pub const MAKELEVEL_NAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"MAKELEVEL\0") };
pub const JOBSERVER_AUTH_OPT: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"jobserver-auth\0") };
pub const MAKE_SUCCESS: i32 = 0;
pub const MAKE_TROUBLE: i32 = 1;
pub const MAKE_FAILURE: i32 = 2;
pub const RM_INCLUDED: i32 = (1) << 1;
pub const RM_DONTCARE: i32 = (1) << 2;
pub const PARSEFS_NONE: i32 = 0;
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn alloc_goaldep() -> *mut goaldep {
    xcalloc(::core::mem::size_of::<goaldep>() as size_t) as *mut goaldep
}
#[inline]
unsafe extern "C" fn free_ns(n: *mut nameseq) {
    free(n as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn free_dep(d: *mut dep) {
    free_ns(d as *mut nameseq);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn free_goaldep(g: *mut goaldep) {
    free_dep(g as *mut dep);
}
#[inline]
unsafe extern "C" fn free_dep_chain(d: *mut dep) {
    free_ns_chain(d as *mut nameseq);
}
pub const UNKNOWN_MTIME: i32 = 0;
pub const NONEXISTENT_MTIME: i32 = 1;
pub const OLD_MTIME: i32 = 2;
pub const no_argument: i32 = 0;
pub const required_argument: i32 = 1;
pub const optional_argument: i32 = 2;
/// Read-only `--silent`/`-s` default: only referenced via `&raw const` as the
/// option table's `default_value`, never written. Immutable removes a mutable
/// global.
static default_silent_flag: i32 = 0;
pub static mut db_level: i32 = 0;
/// Read-only `--keep-going` default: only referenced via `&raw const` as the
/// option table's `default_value`, never written. Immutable removes a mutable
/// global.
static default_keep_going_flag: i32 = 0;
/// Read-only `--print-directory` default: only referenced via `&raw const` as
/// the option table's `default_value`, never written. Immutable removes a
/// mutable global.
static default_print_directory_flag: i32 = -1_i32;
pub const INVALID_JOB_SLOTS: i32 = -1_i32;
/// Jobserver master slot count (0 when this make is not the jobserver master):
/// the number of job slots handed to the jobserver when this make is the
/// master, set once during jobserver setup and read while draining tokens at
/// exit. The former `MASTER_JOB_SLOTS` global atomic, now read through the
/// `with_options`/`OPTIONS_PTR` channel off the owned per-run `Options`.
fn master_job_slots() -> ::core::ffi::c_uint {
    with_options(|o| o.master_job_slots.get())
}
/// Read-only default for the `-j`/`--jobs` option: only ever referenced via
/// `&raw const` as the option table's `default_value`, never written. Keeping
/// it an immutable `static` removes a needless mutable global.
static default_job_slots: i32 = INVALID_JOB_SLOTS;
/// Read-only sentinel for the `-j` no-argument case: only referenced via
/// `&raw const` as the option table's `noarg_value`, never written. Immutable
/// removes a mutable global.
static inf_jobs: i32 = 0;
/// Read-only `-l`/`--load-average` default and no-argument sentinel (`-1.0`,
/// "no load limit"): only referenced via `&raw const` as the option table's
/// `default_value`/`noarg_value`, never written. Immutable removes a mutable
/// global.
static default_load_average: ::core::ffi::c_double = -1.0f64;

/// Option/flag values collected into a single owned instance, threaded through
/// the call graph as `&Options`. Runtime-mutated fields use `Cell`/`RefCell`
/// interior mutability so a shared borrow suffices everywhere except where an
/// owned value is first created (`main_0`). The option-parser sets these fields
/// through char-keyed helpers (`opt_set_flag`/`opt_set_str`) rather than the
/// old raw `value_ptr` dispatch.
pub struct Options {
    pub silent: ::core::cell::Cell<bool>,
    pub silent_origin: ::core::cell::Cell<variable_origin>,
    pub touch: ::core::cell::Cell<bool>,
    pub just_print: ::core::cell::Cell<bool>,
    pub db_flags: ::core::cell::RefCell<Vec<::std::ffi::CString>>,
    pub debug_flag: ::core::cell::Cell<bool>,
    pub output_sync_option: ::core::cell::RefCell<Option<String>>,
    pub env_overrides: ::core::cell::Cell<bool>,
    pub ignore_errors: ::core::cell::Cell<bool>,
    pub print_data_base: ::core::cell::Cell<bool>,
    pub print_targets: ::core::cell::Cell<bool>,
    pub question: ::core::cell::Cell<bool>,
    pub no_builtin_rules: ::core::cell::Cell<bool>,
    pub no_builtin_variables: ::core::cell::Cell<bool>,
    /// Apply-once latches for [`disable_builtins`]. The original C kept these as
    /// function-local `static` flags so the built-in rule/variable cleanup runs
    /// exactly once per process even though `disable_builtins` is reached again
    /// on every `MAKEFLAGS` re-parse. They live on the run-owner `Options`
    /// (reached process-wide via `installed_options()`) rather than on a
    /// threaded `ExecContext`, because `MAKEFLAGS` can be assigned through the
    /// load-API `gmk_eval` path, which carries a throwaway context — latching
    /// there would leave the real run unlatched and re-run the cleanup.
    pub prev_no_builtin_rules: ::core::cell::Cell<bool>,
    pub prev_no_builtin_variables: ::core::cell::Cell<bool>,
    pub keep_going: ::core::cell::Cell<bool>,
    pub keep_going_origin: ::core::cell::Cell<variable_origin>,
    pub check_symlink: ::core::cell::Cell<bool>,
    /// Legacy tri-state: `None` == not specified (-1), `Some(true)` == -w,
    /// `Some(false)` == --no-print-directory.
    pub print_directory: ::core::cell::Cell<Option<bool>>,
    pub print_directory_origin: ::core::cell::Cell<variable_origin>,
    pub print_version: ::core::cell::Cell<bool>,
    pub makefiles: ::core::cell::RefCell<Vec<::std::ffi::CString>>,
    /// Legacy `INVALID_JOB_SLOTS` (-1) == `None`; infinite jobs == `Some(0)`.
    pub arg_job_slots: ::core::cell::Cell<Option<u32>>,
    pub jobserver_auth: ::core::cell::RefCell<Option<String>>,
    pub jobserver_style: ::core::cell::RefCell<Option<String>>,
    pub shuffle_mode: ::core::cell::RefCell<Option<String>>,
    pub sync_mutex: ::core::cell::RefCell<Option<String>>,
    pub max_load_average: ::core::cell::Cell<f64>,
    pub directories: ::core::cell::RefCell<Vec<::std::ffi::CString>>,
    pub include_dirs: ::core::cell::RefCell<Vec<::std::ffi::CString>>,
    pub old_files: ::core::cell::RefCell<Vec<::std::ffi::CString>>,
    pub new_files: ::core::cell::RefCell<Vec<::std::ffi::CString>>,
    pub eval_strings: ::core::cell::RefCell<Vec<::std::ffi::CString>>,
    pub print_usage: ::core::cell::Cell<bool>,
    pub warn_flags: ::core::cell::RefCell<Vec<::std::ffi::CString>>,
    pub warn_undefined_variables: ::core::cell::Cell<bool>,
    pub trace: ::core::cell::Cell<bool>,
    /// The `-B`/`--always-make` flag as set by option parsing.
    pub always_make: ::core::cell::Cell<bool>,
    /// The resolved include search path (validated `-I` dirs plus the default
    /// system directories), built once at startup by
    /// [`crate::read::construct_include_path`] and read by the makefile reader
    /// when resolving an `include` against the search path. Owned here in
    /// `main_0`'s `Options` and reached through the `with_options` borrow
    /// channel, replacing the former `static mut include_directories`.
    pub resolved_include_dirs: ::core::cell::RefCell<Vec<::std::path::PathBuf>>,
    /// Maintainer-mode file-database verification, the former
    /// `static mut verify_flag`: enables the `enter_file` strcache assertion and
    /// the end-of-run `verify_file_data_base` pass. Set when debugging is
    /// requested and unconditionally during startup in this maintainer build;
    /// owned here in `main_0`'s `Options` and read through the `with_options`
    /// borrow channel by `enter_file` and `die`, which carry no `&Options`.
    pub verify: ::core::cell::Cell<bool>,
    /// Effective recipe-echo suppression, the former `static mut run_silent`:
    /// `options.silent` (the `-s`/`--silent` switch) OR'd with a bare `.SILENT`
    /// target. Distinct from `silent`, which is the MAKEFLAGS-visible switch
    /// state — `.SILENT` silences this run without propagating to sub-makes.
    /// Set in `decode_switches` (from `silent`) and `snap_deps` (from
    /// `.SILENT`); read by the recipe-echo / `touch` / `rm` paths in
    /// `job`/`remake`/`file` through the `with_options` channel
    /// ([`opt_run_silent`]), which carry no `&Options`. Lives on `Options` (not
    /// `ExecContext`) because the `decode_switches` writer is reached on the
    /// `reset_makeflags` MAKEFLAGS-reparse path, which `gmk_eval` runs under a
    /// throwaway context.
    pub run_silent: ::core::cell::Cell<bool>,
    /// Export-everything latch, the former `static mut export_all_variables`:
    /// set by a bare `export` directive or the `.EXPORT_ALL_VARIABLES` special
    /// target, cleared by a bare `unexport`. When set, every exportable
    /// non-default/non-automatic variable is placed in each recipe's
    /// environment (see [`crate::variable::should_export`]). Written in
    /// `read::eval` (the `export`/`unexport` directive) and `file::snap_deps`
    /// (`.EXPORT_ALL_VARIABLES`); read by `should_export` through the
    /// `with_options` channel ([`opt_export_all_variables`]). Lives on
    /// `Options` (not `ExecContext`) because those makefile-time writers are
    /// reachable on the `gmk_eval` path, which runs under a throwaway context.
    pub export_all_variables: ::core::cell::Cell<bool>,
    /// The recipe-introducing prefix character, the former `static mut
    /// cmd_prefix`: a tab (`\t`) by default, changed by assigning the
    /// `.RECIPEPREFIX` special variable. The makefile parser uses it to tell a
    /// recipe line from an ordinary one. Written in `set_special_var`
    /// (`.RECIPEPREFIX`) and `print_file` (the `-p` database dump); read by the
    /// reader (`read::eval`) and printers through the `with_options` channel
    /// ([`opt_cmd_prefix`]). Lives on `Options` (not `ExecContext`) because the
    /// `set_special_var` writer is reached on the `gmk_eval` makefile-eval path,
    /// which runs under a throwaway context. Initialized to a tab, not the
    /// `Default` `\0`, so `Options::new()` sets it explicitly.
    pub cmd_prefix: ::core::cell::Cell<::core::ffi::c_char>,
    /// Resolved output-sync mode (`OUTPUT_SYNC_*`), the former `static mut
    /// output_sync`: derived from [`Self::output_sync_option`] (the raw
    /// `-O`/`--output-sync` argument) in `decode_output_sync_flags`, then gated
    /// down to `OUTPUT_SYNC_NONE` when the run turns out to be non-parallel.
    /// Read by the build loop's `syncing` computation and the `output`/`job`
    /// dump paths through the `with_options` channel ([`opt_output_sync`]).
    /// Lives on `Options` (not `ExecContext`) because the
    /// `decode_output_sync_flags` writer is reached on the `reset_makeflags`
    /// MAKEFLAGS-reparse path, which `gmk_eval` runs under a throwaway context.
    pub output_sync: ::core::cell::Cell<i32>,
    /// Resolved parallel job-slot count for this run, the former `static mut
    /// job_slots`: `0` means "unlimited / driven by an inherited jobserver", `1`
    /// is serial, `N>1` is the `-j N` width. Derived in `main_0` from
    /// [`Self::arg_job_slots`] (the raw `-j` argument) and the jobserver state,
    /// then zeroed once this make becomes the jobserver master. Read by the job
    /// scheduler (`start_waiting_jobs`, `load_too_high`) through the
    /// `with_options` channel ([`opt_job_slots`]). Lives on `Options` (not
    /// `ExecContext`) next to `arg_job_slots`; all writers are in `main_0`, so it
    /// is never touched on the `gmk_eval` throwaway path.
    pub job_slots: ::core::cell::Cell<::core::ffi::c_uint>,
    /// The master jobserver slot count. When this make is the jobserver master,
    /// `job_slots` is saved here and then zeroed (the master holds its slots in
    /// the jobserver rather than in `job_slots`); read once while draining
    /// tokens at exit. The former `MASTER_JOB_SLOTS` global atomic, reached
    /// through the `with_options`/`OPTIONS_PTR` channel via `master_job_slots()`.
    /// Both the write (jobserver setup) and the read (exit drain) are on
    /// `main_0`'s real path, so it lives on `Options` beside its `job_slots`
    /// companion rather than on the `gmk_eval`-throwaway `ExecContext`.
    pub master_job_slots: ::core::cell::Cell<::core::ffi::c_uint>,
    /// Monotonic command-generation counter for this run, the former `static
    /// mut command_count`. Bumped once per shell command run (`reap_children`,
    /// `$(shell)`, `$(file)`) via [`bump_command_count`], and read by the
    /// directory cache (`find_directory`) and the `update_goal_chain` loop
    /// through [`opt_command_count`] to invalidate stat/contents entries
    /// recorded before the latest command. Lives on `Options`, reached through
    /// the `with_options`/`OPTIONS_PTR` channel, rather than `ExecContext`: the
    /// `$(shell)`/`$(file)` writers run on the `gmk_eval` throwaway-context
    /// path, so they must reach `main_0`'s real run state, not the throwaway.
    pub command_count: ::core::cell::Cell<::core::ffi::c_ulong>,
    /// `snap_deps`-complete latch for this run, the former `file::SNAPPED_DEPS`
    /// global. Set once at the end of `snap_deps` via [`mark_snapped_deps`] and
    /// read by `record_files` through [`opt_snapped_deps`] to reject
    /// prerequisites defined from within a recipe (i.e. after the snapshot).
    /// Lives on `Options`, reached through the `with_options`/`OPTIONS_PTR`
    /// channel, rather than `ExecContext`: `record_files` is reached from the
    /// `gmk_eval` throwaway-context path, so the reader must see `main_0`'s real
    /// run state, not the throwaway.
    pub snapped_deps: ::core::cell::Cell<bool>,
    /// `true` only while `main_0` is remaking the makefiles themselves (the
    /// makefile-remaking `update_goal_chain` pass), so the remake logic can
    /// treat makefile targets specially. Toggled around that pass in `main_0`
    /// and read across the update walk (`update_goal_chain` / `update_file_1` /
    /// `remake_file`, via [`opt_rebuilding_makefiles`]) and by `reset_makeflags`.
    /// Lives on `Options`, reached through the `with_options`/`OPTIONS_PTR`
    /// channel, rather than `ExecContext`: `reset_makeflags` is reached from
    /// `set_special_var` on the `gmk_eval` throwaway-context path, so the reader
    /// must see `main_0`'s real run state, not the throwaway.
    pub rebuilding_makefiles: ::core::cell::Cell<bool>,
    /// The special-target feature latches, each set once when make sees the
    /// matching `.`-target and read widely thereafter — the former
    /// `POSIX_PEDANTIC` / `SECOND_EXPANSION` / `ONE_SHELL` / `NOT_PARALLEL`
    /// global atomics. Reached through the `with_options`/`OPTIONS_PTR` channel
    /// (via `posix_pedantic()` / `second_expansion()` / `one_shell()` /
    /// `not_parallel()` and their `set_*` setters), because the setters in
    /// `check_specials` / `snap_deps` are reachable from the `gmk_eval`
    /// throwaway-context path and must reach `main_0`'s real run state.
    pub posix_pedantic: ::core::cell::Cell<bool>,
    pub second_expansion: ::core::cell::Cell<bool>,
    pub one_shell: ::core::cell::Cell<bool>,
    pub not_parallel: ::core::cell::Cell<bool>,
    /// One-shot latch set once make has logged the working-directory "Entering
    /// directory" trace (so the matching "Leaving directory" is emitted and
    /// `MAKE_RESTARTS` is prefixed with `-`) — the former `STDIO_TRACED` global
    /// atomic. Reached through the `with_options`/`OPTIONS_PTR` channel (via
    /// `crate::output::stdio_traced()` / `set_stdio_traced()`): the writer in
    /// `output_start` runs on the shared output path, reachable from the
    /// `gmk_eval` throwaway-context path (a plugin-eval'd `$(shell)`/`$(info)`
    /// flushes output), so both ends must resolve to `main_0`'s real run state,
    /// not the throwaway.
    pub stdio_traced: ::core::cell::Cell<bool>,
}

impl Options {
    /// A fresh `Options` with every field at its zero/sentinel default,
    /// matching the original `static mut` initial values.
    pub fn new() -> Options {
        Options {
            silent: ::core::cell::Cell::new(false),
            silent_origin: ::core::cell::Cell::new(o_default),
            touch: ::core::cell::Cell::new(false),
            just_print: ::core::cell::Cell::new(false),
            db_flags: ::core::cell::RefCell::new(Vec::new()),
            debug_flag: ::core::cell::Cell::new(false),
            output_sync_option: ::core::cell::RefCell::new(None),
            env_overrides: ::core::cell::Cell::new(false),
            ignore_errors: ::core::cell::Cell::new(false),
            print_data_base: ::core::cell::Cell::new(false),
            print_targets: ::core::cell::Cell::new(false),
            question: ::core::cell::Cell::new(false),
            no_builtin_rules: ::core::cell::Cell::new(false),
            no_builtin_variables: ::core::cell::Cell::new(false),
            prev_no_builtin_rules: ::core::cell::Cell::new(false),
            prev_no_builtin_variables: ::core::cell::Cell::new(false),
            keep_going: ::core::cell::Cell::new(false),
            keep_going_origin: ::core::cell::Cell::new(o_default),
            check_symlink: ::core::cell::Cell::new(false),
            print_directory: ::core::cell::Cell::new(None),
            print_directory_origin: ::core::cell::Cell::new(o_default),
            print_version: ::core::cell::Cell::new(false),
            makefiles: ::core::cell::RefCell::new(Vec::new()),
            arg_job_slots: ::core::cell::Cell::new(None),
            jobserver_auth: ::core::cell::RefCell::new(None),
            jobserver_style: ::core::cell::RefCell::new(None),
            shuffle_mode: ::core::cell::RefCell::new(None),
            sync_mutex: ::core::cell::RefCell::new(None),
            max_load_average: ::core::cell::Cell::new(-1.0f64),
            directories: ::core::cell::RefCell::new(Vec::new()),
            include_dirs: ::core::cell::RefCell::new(Vec::new()),
            old_files: ::core::cell::RefCell::new(Vec::new()),
            new_files: ::core::cell::RefCell::new(Vec::new()),
            eval_strings: ::core::cell::RefCell::new(Vec::new()),
            print_usage: ::core::cell::Cell::new(false),
            warn_flags: ::core::cell::RefCell::new(Vec::new()),
            warn_undefined_variables: ::core::cell::Cell::new(false),
            trace: ::core::cell::Cell::new(false),
            always_make: ::core::cell::Cell::new(false),
            resolved_include_dirs: ::core::cell::RefCell::new(Vec::new()),
            verify: ::core::cell::Cell::new(false),
            run_silent: ::core::cell::Cell::new(false),
            export_all_variables: ::core::cell::Cell::new(false),
            cmd_prefix: ::core::cell::Cell::new('\t' as i32 as ::core::ffi::c_char),
            output_sync: ::core::cell::Cell::new(OUTPUT_SYNC_NONE),
            job_slots: ::core::cell::Cell::new(0),
            master_job_slots: ::core::cell::Cell::new(0),
            command_count: ::core::cell::Cell::new(1),
            snapped_deps: ::core::cell::Cell::new(false),
            rebuilding_makefiles: ::core::cell::Cell::new(false),
            posix_pedantic: ::core::cell::Cell::new(false),
            second_expansion: ::core::cell::Cell::new(false),
            one_shell: ::core::cell::Cell::new(false),
            not_parallel: ::core::cell::Cell::new(false),
            stdio_traced: ::core::cell::Cell::new(false),
        }
    }
}

impl Default for Options {
    fn default() -> Options {
        Options::new()
    }
}

/// Set a boolean / tri-state `flag`/`flag_off`-type option, keyed by switch
/// character, to `on`. Replaces the old raw `value_ptr` write for the `flag`
/// (type 0) and `flag_off` (type 1) table arms: the parser passes
/// `on = (type == flag)`, so `flag_off` clears the field. The `ignore`-type
/// switches ('b', 'm') and the terminating sentinel have no storage and are
/// silently ignored, matching the original table arms.
fn opt_set_flag(options: &Options, c: i32, on: bool) {
    if c == 'B' as i32 {
        options.always_make.set(on);
    } else if c == 'd' as i32 {
        options.debug_flag.set(on);
    } else if c == 'e' as i32 {
        options.env_overrides.set(on);
    } else if c == 'h' as i32 {
        options.print_usage.set(on);
    } else if c == 'i' as i32 {
        options.ignore_errors.set(on);
    } else if c == 'k' as i32 || c == 'S' as i32 {
        options.keep_going.set(on);
    } else if c == 'L' as i32 {
        options.check_symlink.set(on);
    } else if c == 'n' as i32 {
        options.just_print.set(on);
    } else if c == 'p' as i32 {
        options.print_data_base.set(on);
    } else if c == 'q' as i32 {
        options.question.set(on);
    } else if c == 'r' as i32 {
        options.no_builtin_rules.set(on);
    } else if c == 'R' as i32 {
        options.no_builtin_variables.set(on);
    } else if c == 's' as i32 || c == CHAR_MAX + 8 {
        options.silent.set(on);
    } else if c == 't' as i32 {
        options.touch.set(on);
    } else if c == 'v' as i32 {
        options.print_version.set(on);
    } else if c == 'w' as i32 || c == CHAR_MAX + 4 {
        // -w / --no-print-directory tri-state.
        options.print_directory.set(Some(on));
    } else if c == CHAR_MAX + 3 {
        options.trace.set(on);
    } else if c == CHAR_MAX + 5 {
        options.warn_undefined_variables.set(on);
    } else if c == CHAR_MAX + 14 {
        options.print_targets.set(on);
    }
    // 'b', 'm' (ignore) and the 0 sentinel have no storage.
}

/// Set a `string`-type option, keyed by switch character, to `s`. Replaces the
/// raw `value_ptr` write of the `string` (type 2) table arm.
fn opt_set_str(options: &Options, c: i32, s: String) {
    if c == 'O' as i32 {
        *options.output_sync_option.borrow_mut() = Some(s);
    } else if c == CHAR_MAX + 2 || c == CHAR_MAX + 9 {
        // --jobserver-auth / --jobserver-fds
        *options.jobserver_auth.borrow_mut() = Some(s);
    } else if c == CHAR_MAX + 7 {
        // --sync-mutex
        *options.sync_mutex.borrow_mut() = Some(s);
    } else if c == CHAR_MAX + 11 {
        // --shuffle
        *options.shuffle_mode.borrow_mut() = Some(s);
    } else if c == CHAR_MAX + 12 {
        // --jobserver-style
        *options.jobserver_style.borrow_mut() = Some(s);
    }
}

/// Read a `flag`/`flag_off`-type option's value as the legacy `i32` the
/// original `value_ptr` deref produced (0/1; tri-state `print_directory` maps
/// `None` -> -1). Used by `define_makeflags` to reproduce the MAKEFLAGS logic.
fn opt_flag_int(options: &Options, c: i32) -> i32 {
    let b = |v: bool| v as i32;
    if c == 'B' as i32 {
        b(options.always_make.get())
    } else if c == 'd' as i32 {
        b(options.debug_flag.get())
    } else if c == 'e' as i32 {
        b(options.env_overrides.get())
    } else if c == 'h' as i32 {
        b(options.print_usage.get())
    } else if c == 'i' as i32 {
        b(options.ignore_errors.get())
    } else if c == 'k' as i32 || c == 'S' as i32 {
        b(options.keep_going.get())
    } else if c == 'L' as i32 {
        b(options.check_symlink.get())
    } else if c == 'n' as i32 {
        b(options.just_print.get())
    } else if c == 'p' as i32 {
        b(options.print_data_base.get())
    } else if c == 'q' as i32 {
        b(options.question.get())
    } else if c == 'r' as i32 {
        b(options.no_builtin_rules.get())
    } else if c == 'R' as i32 {
        b(options.no_builtin_variables.get())
    } else if c == 's' as i32 || c == CHAR_MAX + 8 {
        b(options.silent.get())
    } else if c == 't' as i32 {
        b(options.touch.get())
    } else if c == 'v' as i32 {
        b(options.print_version.get())
    } else if c == 'w' as i32 || c == CHAR_MAX + 4 {
        match options.print_directory.get() {
            None => -1,
            Some(v) => b(v),
        }
    } else if c == CHAR_MAX + 3 {
        b(options.trace.get())
    } else if c == CHAR_MAX + 5 {
        b(options.warn_undefined_variables.get())
    } else if c == CHAR_MAX + 14 {
        b(options.print_targets.get())
    } else {
        0
    }
}

/// Whether the switch char `c` has an associated origin slot (the `-s`/`-k`/
/// `-w` family and their `--no-*` aliases), and which `Options` `Cell` backs it.
/// Returns `None` for switches with no origin (the old null `cs.origin`).
fn opt_origin_cell(options: &Options, c: i32) -> Option<&::core::cell::Cell<variable_origin>> {
    if c == 's' as i32 || c == CHAR_MAX + 8 {
        Some(&options.silent_origin)
    } else if c == 'k' as i32 || c == 'S' as i32 {
        Some(&options.keep_going_origin)
    } else if c == 'w' as i32 || c == CHAR_MAX + 4 {
        Some(&options.print_directory_origin)
    } else {
        None
    }
}

/// Read a `positive_int`-type option's value as the legacy `c_uint` the old
/// `value_ptr` deref produced. Only `-j` is a positive_int option; `None`
/// (unspecified) maps to the `INVALID_JOB_SLOTS` sentinel reinterpreted as
/// `c_uint`, matching the original `arg_job_slots` storage.
fn opt_uint(options: &Options, c: i32) -> ::core::ffi::c_uint {
    if c == 'j' as i32 {
        match options.arg_job_slots.get() {
            None => INVALID_JOB_SLOTS as ::core::ffi::c_uint,
            Some(n) => n,
        }
    } else {
        0
    }
}

/// Read a `floating`-type option's value. Only `-l` is a floating option.
fn opt_double(options: &Options, c: i32) -> ::core::ffi::c_double {
    if c == 'l' as i32 {
        options.max_load_average.get()
    } else {
        0.0
    }
}

/// Read a `string`-type option's current value as an owned `CString` (or
/// `None` when unset), keyed by switch character. The returned `CString` owns
/// its bytes, so it stays alive across `variable_buffer_output` in
/// `define_makeflags`.
fn opt_get_str(options: &Options, c: i32) -> Option<::std::ffi::CString> {
    let conv = |o: &Option<String>| {
        o.as_ref()
            .and_then(|s| ::std::ffi::CString::new(s.as_bytes()).ok())
    };
    if c == 'O' as i32 {
        conv(&options.output_sync_option.borrow())
    } else if c == CHAR_MAX + 2 || c == CHAR_MAX + 9 {
        conv(&options.jobserver_auth.borrow())
    } else if c == CHAR_MAX + 7 {
        conv(&options.sync_mutex.borrow())
    } else if c == CHAR_MAX + 11 {
        conv(&options.shuffle_mode.borrow())
    } else if c == CHAR_MAX + 12 {
        conv(&options.jobserver_style.borrow())
    } else {
        None
    }
}

thread_local! {
    /// Borrow channel to the `Options` owned as a local in `main_0`. This is a
    /// *pointer*, not the option data: the values still live in `main_0`'s
    /// `let options`. It exists solely so the one deep makefile-time callback
    /// that re-decodes `MAKEFLAGS` (`set_special_var` -> `reset_makeflags`,
    /// reached via the high-fan-in `do_variable_definition`) can reach the
    /// owned `Options` without threading a borrow through the entire
    /// read/eval engine. Set for the dynamic extent of `main_0`.
    static OPTIONS_PTR: ::core::cell::Cell<*const Options> =
        const { ::core::cell::Cell::new(::core::ptr::null()) };
}

/// Borrow the installed `main_0` `Options`. Only valid while `main_0` is on the
/// stack (its referent outlives every makefile-time callback).
unsafe fn installed_options<'a>() -> &'a Options {
    let p = OPTIONS_PTR.with(|c| c.get());
    debug_assert!(
        !p.is_null(),
        "installed_options called with no Options on the stack"
    );
    &*p
}

/// Run `f` with a borrow of `main_0`'s single owned `Options`, reached through
/// the `OPTIONS_PTR` borrow channel. This is the single source of truth for the
/// deep, high-fan-in option readers (`job`/`remake`/`file`/`output`/`variable`)
/// that sit behind hundreds of call sites and some C-ABI callbacks, so they
/// cannot take an `&Options` parameter. `OPTIONS_PTR` is installed at the very
/// start of `main_0`, before any code that could read options runs, and its
/// referent outlives every makefile-time/build-time callback.
pub fn with_options<R>(f: impl FnOnce(&Options) -> R) -> R {
    f(unsafe { installed_options() })
}

/// Test-only: install a default `Options` on the current thread's
/// `OPTIONS_PTR` borrow channel so option readers
/// (`opt_check_symlink`, etc.) can run inside `#[cfg(test)]` unit tests
/// that exercise code below `main_0`. The `Options` is leaked so the
/// installed pointer stays valid for the thread's lifetime; this only
/// affects test binaries and never changes shipping behavior.
#[cfg(test)]
pub fn install_default_options_for_test() {
    OPTIONS_PTR.with(|p| {
        if p.get().is_null() {
            let leaked: &'static Options = Box::leak(Box::new(Options::new()));
            p.set(leaked as *const Options);
        }
    });
}

/// Test-only: install a valid `program` name and reset `makelevel` to 0 so
/// the real `error()` / `message()` / `warning()` output paths can run inside
/// `#[cfg(test)]` unit tests without dereferencing the otherwise-null
/// `program` pointer (which segfaults outside full make init). The name is a
/// leaked `CString` so the installed pointer stays valid for the test
/// binary's lifetime. This only affects test builds and never changes
/// shipping behavior.
///
/// # Safety
/// Writes the `program` process global; callers must serialize against other
/// code touching that global (e.g. via the relevant test mutex).
#[cfg(test)]
pub unsafe fn install_program_name_for_test() {
    if program.is_null() {
        let leaked: &'static std::ffi::CStr =
            Box::leak(Box::new(std::ffi::CString::new("make").unwrap())).as_c_str();
        program = leaked.as_ptr();
    }
}

pub fn env_overrides() -> bool {
    with_options(|o| o.env_overrides.get())
}
pub fn opt_question() -> bool {
    with_options(|o| o.question.get())
}
pub fn opt_touch() -> bool {
    with_options(|o| o.touch.get())
}
pub fn opt_just_print() -> bool {
    with_options(|o| o.just_print.get())
}
/// Effective recipe-echo suppression (the former `run_silent` global), read
/// through the `with_options` borrow channel by the deep recipe-echo /
/// `touch` / `rm` paths in `job`/`remake`/`file` that carry no `&Options`.
pub fn opt_run_silent() -> bool {
    with_options(|o| o.run_silent.get())
}
/// Export-everything latch (the former `export_all_variables` global), read
/// through the `with_options` borrow channel by `should_export`, which carries
/// no `&Options`.
pub fn opt_export_all_variables() -> bool {
    with_options(|o| o.export_all_variables.get())
}
/// The recipe-introducing prefix character (the former `cmd_prefix` global),
/// read through the `with_options` borrow channel by the makefile reader and
/// the database printers, which carry no `&Options`.
pub fn opt_cmd_prefix() -> ::core::ffi::c_char {
    with_options(|o| o.cmd_prefix.get())
}
/// Resolved output-sync mode (the former `output_sync` global), read through
/// the `with_options` borrow channel by the `syncing` computation and the
/// `output`/`job` dump paths, which carry no `&Options`.
pub fn opt_output_sync() -> i32 {
    with_options(|o| o.output_sync.get())
}
/// Resolved parallel job-slot count (the former `job_slots` global), read
/// through the `with_options` borrow channel by the job scheduler, which
/// carries no `&Options`.
pub fn opt_job_slots() -> ::core::ffi::c_uint {
    with_options(|o| o.job_slots.get())
}
/// Monotonic command-generation counter (the former `command_count` global),
/// read through the `with_options` borrow channel by the directory cache
/// (`find_directory`) and the `update_goal_chain` loop, which carry no
/// `&Options`.
pub fn opt_command_count() -> ::core::ffi::c_ulong {
    with_options(|o| o.command_count.get())
}
/// Bump the command-generation counter, once per shell command run
/// (`reap_children`, `$(shell)`, `$(file)`). Goes through the `with_options`
/// channel so it always reaches `main_0`'s real `Options`, even on the
/// `gmk_eval` throwaway-context path the `$(shell)`/`$(file)` writers take.
pub fn bump_command_count() {
    with_options(|o| o.command_count.set(o.command_count.get().wrapping_add(1)));
}
/// Whether `snap_deps` has run for this make (the former `file::SNAPPED_DEPS`
/// global), read through the `with_options` channel by `record_files` — which
/// is reachable from the `gmk_eval` throwaway-context path and so cannot rely
/// on its `&ExecContext` being `main_0`'s real run context.
pub fn opt_snapped_deps() -> bool {
    with_options(|o| o.snapped_deps.get())
}
/// Mark the dependency snapshot complete, once, at the end of `snap_deps`. Goes
/// through the `with_options` channel so it always sets `main_0`'s real
/// `Options`.
pub fn mark_snapped_deps() {
    with_options(|o| o.snapped_deps.set(true));
}
/// Whether `main_0` is currently remaking the makefiles themselves (the former
/// `REBUILDING_MAKEFILES` global), read through the `with_options` channel by
/// the update walk and by `reset_makeflags` — the latter reached from
/// `set_special_var` on the `gmk_eval` throwaway path, so it must resolve to
/// `main_0`'s real run state rather than a throwaway context.
pub fn opt_rebuilding_makefiles() -> bool {
    with_options(|o| o.rebuilding_makefiles.get())
}
pub fn opt_ignore_errors() -> bool {
    with_options(|o| o.ignore_errors.get())
}
pub fn opt_keep_going() -> bool {
    with_options(|o| o.keep_going.get())
}
pub fn opt_check_symlink() -> bool {
    with_options(|o| o.check_symlink.get())
}
pub fn opt_no_builtin_rules() -> bool {
    with_options(|o| o.no_builtin_rules.get())
}
pub fn opt_print_data_base() -> bool {
    with_options(|o| o.print_data_base.get())
}
pub fn opt_print_version() -> bool {
    with_options(|o| o.print_version.get())
}
pub fn opt_jobserver_auth_present() -> bool {
    with_options(|o| o.jobserver_auth.borrow().is_some())
}
pub fn opt_max_load_average() -> f64 {
    with_options(|o| o.max_load_average.get())
}

/// `should_print_dir` for callers outside the `Options` borrow chain
/// (`output.rs`), reading the owned `Options` through the borrow channel.
pub fn should_print_dir_mirror(ctx: &crate::execctx::ExecContext) -> i32 {
    with_options(|o| match o.print_directory.get() {
        Some(v) => v as i32,
        None => {
            let ml = ctx.makelevel();
            (!o.silent.get() && ml > 0) as i32
        }
    })
}

pub fn set_touch_mirror(v: bool) {
    with_options(|o| o.touch.set(v));
}
pub fn set_question_mirror(v: bool) {
    with_options(|o| o.question.set(v));
}
pub fn set_just_print_mirror(v: bool) {
    with_options(|o| o.just_print.set(v));
}
pub fn set_ignore_errors_mirror(v: bool) {
    with_options(|o| o.ignore_errors.set(v));
}
pub static mut shell_var: variable = variable {
    name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    value: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    fileinfo: Floc {
        filenm: ::core::ptr::null::<::core::ffi::c_char>(),
        lineno: 0,
        offset: 0,
    },
    length: 0,
    recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
};
static mut stdin_offset: i32 = -1_i32;
/// Strcache'd name of the temporary file holding the makefile read from stdin
/// (or null). Mirrors `options.makefiles[stdin_offset]` so `temp_stdin_unlink`
/// can run from the deep `die` path without an `&Options` borrow. The pointer
/// is into the strcache, which lives for the whole run.
static mut temp_stdin_name: *const ::core::ffi::c_char = ::core::ptr::null();
static mut usage: [*const ::core::ffi::c_char; 36] = [
    b"Options:\n\0" as *const u8 as *const ::core::ffi::c_char,
    b"  -b, -m                      Ignored for compatibility.\n\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  -B, --always-make           Unconditionally make all targets.\n\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  -C DIRECTORY, --directory=DIRECTORY\n                              Change to DIRECTORY before doing anything.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -d                          Print lots of debugging information.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  --debug[=FLAGS]             Print various types of debugging information.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -e, --environment-overrides\n                              Environment variables override makefiles.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -E STRING, --eval=STRING    Evaluate STRING as a makefile statement.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -f FILE, --file=FILE, --makefile=FILE\n                              Read FILE as a makefile.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -h, --help                  Print this message and exit.\n\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  -i, --ignore-errors         Ignore errors from recipes.\n\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  -I DIRECTORY, --include-dir=DIRECTORY\n                              Search DIRECTORY for included makefiles.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -j [N], --jobs[=N]          Allow N jobs at once; infinite jobs with no arg.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  --jobserver-style=STYLE     Select the style of jobserver to use.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -k, --keep-going            Keep going when some targets can't be made.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -l [N], --load-average[=N], --max-load[=N]\n                              Don't start multiple jobs unless load is below N.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -L, --check-symlink-times   Use the latest mtime between symlinks and target.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -n, --just-print, --dry-run, --recon\n                              Don't actually run any recipe; just print them.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -o FILE, --old-file=FILE, --assume-old=FILE\n                              Consider FILE to be very old and don't remake it.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -O[TYPE], --output-sync[=TYPE]\n                              Synchronize output of parallel jobs by TYPE.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -p, --print-data-base       Print make's internal database.\n\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  -q, --question              Run no recipe; exit status says if up to date.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -r, --no-builtin-rules      Disable the built-in implicit rules.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -R, --no-builtin-variables  Disable the built-in variable settings.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  --shuffle[={SEED|random|reverse|none}]\n                              Perform shuffle of prerequisites and goals.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -s, --silent, --quiet       Don't echo recipes.\n\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  --no-silent                 Echo recipes (disable --silent mode).\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -S, --no-keep-going, --stop\n                              Turns off -k.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -t, --touch                 Touch targets instead of remaking them.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  --trace                     Print tracing information.\n\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  -v, --version               Print the version number of make and exit.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -w, --print-directory       Print the current directory.\n\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  --no-print-directory        Turn off -w, even if it was turned on implicitly.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  -W FILE, --what-if=FILE, --new-file=FILE, --assume-new=FILE\n                              Consider FILE to be infinitely new.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  --warn[=CONTROL]            Control warnings for makefile issues.\n\0"
        as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
pub const TEMP_STDIN_OPT: i32 = CHAR_MAX + 10;
pub const WARN_OPT: i32 = CHAR_MAX + 13;
static mut switches: [command_switch; 42] = [command_switch {
    c: 0,
    type_0: flag,
    value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    env_toenv_no_makefile_specified: [0; 1],
    c2rust_padding: [0; 7],
    noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
    default_value: ::core::ptr::null::<::core::ffi::c_void>(),
    long_name: ::core::ptr::null::<::core::ffi::c_char>(),
    origin: ::core::ptr::null_mut::<variable_origin>(),
}; 42];
static mut long_option_aliases: [option; 9] = [
    option {
        name: b"quiet\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null::<i32>() as *mut i32,
        val: 's' as i32,
    },
    option {
        name: b"stop\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null::<i32>() as *mut i32,
        val: 'S' as i32,
    },
    option {
        name: b"new-file\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<i32>() as *mut i32,
        val: 'W' as i32,
    },
    option {
        name: b"assume-new\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<i32>() as *mut i32,
        val: 'W' as i32,
    },
    option {
        name: b"assume-old\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<i32>() as *mut i32,
        val: 'o' as i32,
    },
    option {
        name: b"max-load\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: optional_argument,
        flag: ::core::ptr::null::<i32>() as *mut i32,
        val: 'l' as i32,
    },
    option {
        name: b"dry-run\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null::<i32>() as *mut i32,
        val: 'n' as i32,
    },
    option {
        name: b"recon\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null::<i32>() as *mut i32,
        val: 'n' as i32,
    },
    option {
        name: b"makefile\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<i32>() as *mut i32,
        val: 'f' as i32,
    },
];
static mut goals: *mut goaldep = ::core::ptr::null::<goaldep>() as *mut goaldep;
static mut lastgoal: *mut goaldep = ::core::ptr::null::<goaldep>() as *mut goaldep;
static mut command_variables: *mut command_variable =
    ::core::ptr::null::<command_variable>() as *mut command_variable;
pub static mut program: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
pub static mut directory_before_chdir: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
pub static mut starting_directory: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
pub static mut default_goal_var: *mut variable = ::core::ptr::null::<variable>() as *mut variable;
pub static mut default_file: *mut file = ::core::ptr::null::<file>() as *mut file;
// The four special-target feature latches — `.POSIX`, `.SECONDEXPANSION`,
// `.ONESHELL`, `.NOTPARALLEL` — each set once when make sees the corresponding
// special target and read widely thereafter. They live on the owned per-run
// `Options` (the former `POSIX_PEDANTIC` / `SECOND_EXPANSION` / `ONE_SHELL` /
// `NOT_PARALLEL` global atomics), reached through the `with_options`/`OPTIONS_PTR`
// channel: the setters run in `check_specials` / `snap_deps`, which are reachable
// from the `gmk_eval` throwaway-context path, so both ends resolve to `main_0`'s
// real run state, not a throwaway.

/// Whether `.POSIX` pedantic mode is in effect.
pub fn posix_pedantic() -> bool {
    with_options(|o| o.posix_pedantic.get())
}
/// Record that the `.POSIX` special target has been seen.
pub fn set_posix_pedantic() {
    with_options(|o| o.posix_pedantic.set(true));
}
/// Whether `.SECONDEXPANSION` is in effect.
pub fn second_expansion() -> bool {
    with_options(|o| o.second_expansion.get())
}
/// Record that the `.SECONDEXPANSION` special target has been seen.
pub fn set_second_expansion() {
    with_options(|o| o.second_expansion.set(true));
}
/// Whether `.ONESHELL` is in effect (each recipe runs in a single shell).
pub fn one_shell() -> bool {
    with_options(|o| o.one_shell.get())
}
/// Record that the `.ONESHELL` special target has been seen.
pub fn set_one_shell() {
    with_options(|o| o.one_shell.set(true));
}
/// Whether make is running non-parallel (one job at a time).
pub fn not_parallel() -> bool {
    with_options(|o| o.not_parallel.get())
}
/// Record that the `.NOTPARALLEL` special target has been seen.
pub fn set_not_parallel() {
    with_options(|o| o.not_parallel.set(true));
}
/// Per-byte classification bitmap (`MAP_*` flags), computed once at startup by
/// [`initialize_stopchar_map`]. Held behind a `OnceLock` so it is a safe
/// `static`; reads before initialization see a zeroed map, matching the C
/// `static`'s zero-initialized state.
static STOPCHAR_MAP: ::std::sync::OnceLock<[::core::ffi::c_ushort; 256]> =
    ::std::sync::OnceLock::new();
/// Borrow the classification map. Returns a zeroed map until
/// [`initialize_stopchar_map`] has run.
pub fn stopchar_map() -> &'static [::core::ffi::c_ushort; 256] {
    static ZERO: [::core::ffi::c_ushort; 256] = [0; 256];
    STOPCHAR_MAP.get().unwrap_or(&ZERO)
}
pub static mut make_sync: output = output {
    out: 0,
    err: 0,
    syncout: [0; 1],
    c2rust_padding: [0; 3],
};
unsafe fn make_sync_syncout() -> ::core::ffi::c_uint {
    ((*(&raw const make_sync)).syncout[0] & 1) as ::core::ffi::c_uint
}

unsafe fn set_make_sync_syncout(value: ::core::ffi::c_uint) {
    let make_sync_ptr = &raw mut make_sync;
    (*make_sync_ptr).syncout[0] = ((*make_sync_ptr).syncout[0] & !1) | (value as u8 & 1);
}
pub static mut fatal_signal_set: sigset_t = __sigset_t { __val: [0; 16] };
unsafe extern "C" fn bsd_signal(sig: i32, func: bsd_signal_ret_t) -> bsd_signal_ret_t {
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut oact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    act.__sigaction_handler.sa_handler = func as __sighandler_t;
    act.sa_flags = SA_RESTART;
    sigemptyset(&raw mut act.sa_mask);
    sigaddset(&raw mut act.sa_mask, sig);
    if sigaction(sig, &raw mut act, &raw mut oact) != 0 {
        return ::core::mem::transmute::<::libc::intptr_t, bsd_signal_ret_t>(
            -1_i32 as ::libc::intptr_t,
        );
    }
    oact.__sigaction_handler.sa_handler as bsd_signal_ret_t
}
fn signal_handler_addr(handler: bsd_signal_ret_t) -> usize {
    handler.map_or(0, |handler| handler as usize)
}

unsafe fn sig_ign_handler() -> bsd_signal_ret_t {
    ::core::mem::transmute::<::libc::intptr_t, bsd_signal_ret_t>(1_i32 as ::libc::intptr_t)
}

unsafe fn install_fatal_signal(sig: i32) {
    let old_handler = bsd_signal(
        sig,
        Some(fatal_error_signal as unsafe extern "C" fn(i32) -> ()),
    );
    if signal_handler_addr(old_handler) == 1 {
        bsd_signal(sig, sig_ign_handler());
    } else {
        sigaddset(&raw mut fatal_signal_set, sig);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn initialize_global_hash_tables() {
    init_hash_global_variable_set();
    strcache_init();
    init_hash_files();
    hash_init_directories();
    hash_init_function_table();
}
/// Build the global `stopchar_map` character-classification table the parser
/// consults to recognize separators, blanks, and special characters.
pub fn initialize_stopchar_map() {
    let mut map = [0 as ::core::ffi::c_ushort; 256];
    map[0] = MAP_NUL as ::core::ffi::c_ushort;
    map['#' as usize] = MAP_COMMENT as ::core::ffi::c_ushort;
    map[';' as usize] = MAP_SEMI as ::core::ffi::c_ushort;
    map['=' as usize] = MAP_EQUALS as ::core::ffi::c_ushort;
    map[':' as usize] = MAP_COLON as ::core::ffi::c_ushort;
    map['|' as usize] = MAP_PIPE as ::core::ffi::c_ushort;
    map['.' as usize] = (MAP_DOT | MAP_USERFUNC) as ::core::ffi::c_ushort;
    map[',' as usize] = MAP_COMMA as ::core::ffi::c_ushort;
    map['(' as usize] = MAP_VARSEP as ::core::ffi::c_ushort;
    map['{' as usize] = MAP_VARSEP as ::core::ffi::c_ushort;
    map['}' as usize] = MAP_VARSEP as ::core::ffi::c_ushort;
    map[')' as usize] = MAP_VARSEP as ::core::ffi::c_ushort;
    map['$' as usize] = MAP_VARIABLE as ::core::ffi::c_ushort;
    map['-' as usize] = MAP_USERFUNC as ::core::ffi::c_ushort;
    map['_' as usize] = MAP_USERFUNC as ::core::ffi::c_ushort;
    map[' ' as usize] = MAP_BLANK as ::core::ffi::c_ushort;
    map['\t' as usize] = MAP_BLANK as ::core::ffi::c_ushort;
    map['/' as usize] = MAP_DIRSEP as ::core::ffi::c_ushort;
    // Locale-dependent classes from the C ctype table (the only unsafe access).
    let ctype = unsafe { *__ctype_b_loc() };
    let mut i: i32 = 1;
    while i <= UCHAR_MAX {
        let cls = unsafe { *ctype.offset(i as isize) } as i32;
        if cls & _ISspace as i32 as ::core::ffi::c_ushort as i32 != 0
            && map[i as usize] as i32 & 0x2_i32 == 0
        {
            map[i as usize] = (map[i as usize] as i32 | MAP_NEWLINE) as ::core::ffi::c_ushort;
        } else if cls & _ISalnum as i32 as ::core::ffi::c_ushort as i32 != 0 {
            map[i as usize] = (map[i as usize] as i32 | MAP_USERFUNC) as ::core::ffi::c_ushort;
        }
        i += 1;
    }
    let _ = STOPCHAR_MAP.set(map);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe extern "C" fn close_stdout() {
    let prev_fail: i32 = ferror(stdout);
    let fclose_fail: i32 = fclose(stdout);
    if prev_fail != 0 || fclose_fail != 0 {
        // This is the `atexit`-registered handler: it cannot be passed the
        // owned `ExecContext` and there is deliberately no global to read it
        // from, so it must not route through a `ctx`-taking printer. Write the
        // bare diagnostic (no `make[N]:` prefix) straight to stderr.
        let msg = if fclose_fail != 0 {
            let err = libc::strerror(*__errno_location());
            format!(
                "write error: stdout: {}\n",
                std::ffi::CStr::from_ptr(err).to_string_lossy()
            )
        } else {
            "write error: stdout\n".to_string()
        };
        let mut bytes = msg.into_bytes();
        bytes.push(0);
        // Prefix-free path (atexit handler): a default ctx yields no `make[N]:`
        // prefix and reads no global.
        crate::output::outputs(
            &crate::execctx::ExecContext::default(),
            1,
            bytes.as_ptr() as *const ::core::ffi::c_char,
        );
        exit(MAKE_TROUBLE);
    }
}
unsafe fn expand_command_line_file(
    ctx: &crate::execctx::ExecContext,
    mut name: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let cp: *const ::core::ffi::c_char;
    let mut expanded: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *name.offset(0_i32 as isize) as i32 == 0 {
        fatal(
            ctx,
            ::core::ptr::null_mut::<Floc>(),
            0,
            b"empty string invalid as file name\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *name.offset(0_i32 as isize) as i32 == '~' as i32 {
        expanded = tilde_expand(ctx, name);
        if !expanded.is_null() && *expanded.offset(0_i32 as isize) as i32 != 0 {
            name = expanded;
        }
    }
    // Strip leading `./` prefixes via the shared safe parser core; an empty
    // result becomes `./`. `name.add(off)` stays inside the NUL-terminated
    // buffer, so the tail it points at is still a valid C string.
    let bytes = ::std::ffi::CStr::from_ptr(name).to_bytes();
    let off = crate::parser::strip_dot_slash_prefix(bytes);
    if off == bytes.len() {
        name = b"./\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        name = name.add(off);
    }
    cp = strcache_add(name);
    free(expanded as *mut ::core::ffi::c_void);
    cp
}
#[cfg(test)]
mod expand_command_line_file_tests {
    use crate::parser::strip_dot_slash_prefix;

    /// Verbatim pre-refactor `./`-stripping from `expand_command_line_file`,
    /// preserved as an oracle (AGENTS.md: keep the original logic as a test
    /// oracle when replacing it). Models the NUL-terminated C string — reads
    /// past the end yield 0 — and returns the resulting file-name bytes (`./`
    /// when the name collapses to empty).
    fn unsafe_oracle(name: &[u8]) -> Vec<u8> {
        let g = |i: usize| -> u8 { if i < name.len() { name[i] } else { 0 } };
        let mut i = 0usize;
        while g(i) == b'.' && g(i + 1) == b'/' {
            i += 2;
            while g(i) == b'/' {
                i += 1;
            }
        }
        if g(i) == 0 {
            b"./".to_vec()
        } else {
            name[i..].to_vec()
        }
    }

    /// The decision the refactored wrapper now makes before interning: the
    /// shared safe `strip_dot_slash_prefix` core plus the empty-result `./`
    /// substitution.
    fn refactored(name: &[u8]) -> Vec<u8> {
        let off = strip_dot_slash_prefix(name);
        if off == name.len() {
            b"./".to_vec()
        } else {
            name[off..].to_vec()
        }
    }

    #[test]
    fn matches_unsafe_oracle_exhaustively() {
        // Every byte string up to length 8 over {`.`, `/`, `x`} — the alphabet
        // that exercises the `./`-prefix runs, the empty collapse, and the
        // other-char stop.
        fn rec(alpha: &[u8], buf: &mut Vec<u8>, depth: usize) {
            assert_eq!(refactored(buf), unsafe_oracle(buf), "case {:?}", buf);
            if depth == 0 {
                return;
            }
            for &c in alpha {
                buf.push(c);
                rec(alpha, buf, depth - 1);
                buf.pop();
            }
        }
        rec(&[b'.', b'/', b'x'], &mut Vec::new(), 8);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe extern "C" fn debug_signal_handler(mut _sig: i32) {
    db_level = if db_level != 0 { DB_NONE } else { DB_BASIC };
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn decode_debug_flags(ctx: &crate::execctx::ExecContext, options: &Options) {
    if options.debug_flag.get() {
        db_level = DB_ALL;
    }
    if options.trace.get() {
        db_level |= DB_PRINT | DB_WHY;
    }
    {
        let db_flags = options.db_flags.borrow();
        for entry in db_flags.iter() {
            let mut p: *const ::core::ffi::c_char = entry.as_ptr();
            loop {
                match tolower(*p.offset(0_i32 as isize) as i32) {
                    97 => {
                        db_level |= DB_ALL;
                    }
                    98 => {
                        db_level |= DB_BASIC;
                    }
                    105 => {
                        db_level |= DB_BASIC | DB_IMPLICIT;
                    }
                    106 => {
                        db_level |= DB_JOBS;
                    }
                    109 => {
                        db_level |= DB_BASIC | DB_MAKEFILES;
                    }
                    110 => {
                        db_level = 0;
                    }
                    112 => {
                        db_level |= DB_PRINT;
                    }
                    118 => {
                        db_level |= DB_BASIC | DB_VERBOSE;
                    }
                    119 => {
                        db_level |= DB_WHY;
                    }
                    _ => {
                        fatal(
                            ctx,
                            ::core::ptr::null_mut::<Floc>(),
                            strlen(p) as size_t,
                            b"unknown debug level specification '%s'\0" as *const u8
                                as *const ::core::ffi::c_char,
                            p,
                        );
                    }
                }
                loop {
                    p = p.offset(1_i32 as isize);
                    if !(*p as i32 != 0) {
                        break;
                    }
                    if !(*p as i32 == ',' as i32 || *p as i32 == ' ' as i32) {
                        continue;
                    }
                    p = p.offset(1_i32 as isize);
                    break;
                }
                if *p as i32 == 0 {
                    break;
                }
            }
        }
    }
    if db_level != 0 {
        options.verify.set(true);
    }
    if db_level == 0 {
        options.debug_flag.set(false);
    }
}
/// Map an `--output-sync` argument value to its `OUTPUT_SYNC_*` mode, or
/// `None` if it names no known mode.
fn classify_output_sync(value: &[u8]) -> Option<i32> {
    match value {
        b"none" => Some(OUTPUT_SYNC_NONE),
        b"line" => Some(OUTPUT_SYNC_LINE),
        b"target" => Some(OUTPUT_SYNC_TARGET),
        b"recurse" => Some(OUTPUT_SYNC_RECURSE),
        _ => None,
    }
}
/// # Safety
///
/// Reads the global `FLAGS.output_sync_option` / `FLAGS.sync_mutex` C strings; both must be
/// null or valid NUL-terminated strings, and this must run single-threaded
/// during option decoding.
pub unsafe fn decode_output_sync_flags(ctx: &crate::execctx::ExecContext, options: &Options) {
    if let Some(opt) = options.output_sync_option.borrow().as_ref() {
        match classify_output_sync(opt.as_bytes()) {
            Some(mode) => options.output_sync.set(mode),
            None => {
                let c = ::std::ffi::CString::new(opt.as_bytes()).unwrap_or_default();
                fatal(
                    ctx,
                    ::core::ptr::null_mut::<Floc>(),
                    opt.len() as size_t,
                    b"unknown output-sync type '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                    c.as_ptr(),
                );
            }
        }
    }
    if let Some(mtx) = options.sync_mutex.borrow().as_ref() {
        let c = ::std::ffi::CString::new(mtx.as_bytes()).unwrap_or_default();
        osync_parse_mutex(ctx, c.as_ptr());
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_usage(ctx: &crate::execctx::ExecContext, options: &Options, bad: i32) -> ! {
    let mut cpp: *const *const ::core::ffi::c_char;
    let usageto: *mut FILE;
    if options.print_version.get() {
        print_version();
        fputs(b"\n\0" as *const u8 as *const ::core::ffi::c_char, stdout);
    }
    usageto = if bad != 0 { stderr } else { stdout };
    fprintf(
        usageto,
        b"Usage: %s [options] [target] ...\n\0" as *const u8 as *const ::core::ffi::c_char,
        program,
    );
    cpp = &raw const usage as *const *const ::core::ffi::c_char;
    while !(*cpp).is_null() {
        fputs(*cpp, usageto);
        cpp = cpp.offset(1_i32 as isize);
    }
    if remote_description.is_null() || *remote_description as i32 == 0 {
        fprintf(
            usageto,
            b"\nThis program built for %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            crate::version::make_host(),
        );
    } else {
        fprintf(
            usageto,
            b"\nThis program built for %s (%s)\n\0" as *const u8 as *const ::core::ffi::c_char,
            crate::version::make_host(),
            remote_description,
        );
    }
    fprintf(
        usageto,
        b"Report bugs to <bug-make@gnu.org>\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    die(ctx, if bad != 0 { MAKE_FAILURE } else { MAKE_SUCCESS });
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn reset_jobserver(options: &Options) {
    jobserver_clear();
    *options.jobserver_auth.borrow_mut() = None;
}

/// Jobserver reset for the end-of-run `clean_jobserver`/`die` path, which has
/// no `&Options` borrow. Reaches the owned `Options` through the borrow channel
/// (still installed for the dynamic extent of `main_0`).
pub unsafe fn reset_jobserver_mirror() {
    jobserver_clear();
    with_options(|o| *o.jobserver_auth.borrow_mut() = None);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn temp_stdin_unlink(ctx: &crate::execctx::ExecContext) {
    if stdin_offset >= 0 && !temp_stdin_name.is_null() {
        let nm: *const ::core::ffi::c_char = temp_stdin_name;
        let mut r: i32;
        stdin_offset = -1_i32;
        loop {
            r = unlink(nm);
            if !(r == -1_i32 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 && *__errno_location() != ENOENT && handling_fatal_signal == 0 {
            perror_with_name(
                ctx,
                b"unlink (temporary file): \0" as *const u8 as *const ::core::ffi::c_char,
                nm,
            );
        }
    }
}
unsafe fn main_0(
    argc: i32,
    argv: *mut *mut ::core::ffi::c_char,
    envp: *mut *mut ::core::ffi::c_char,
) -> i32 {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut makefile_status: i32 = MAKE_SUCCESS;
    let mut read_files: *mut goaldep;
    let mut current_directory: [::core::ffi::c_char; 4097] = [0; 4097];
    let mut restarts: ::core::ffi::c_uint = 0;
    let mut syncing: ::core::ffi::c_uint;
    let argv_slots: Option<u32>;
    // Owned option/flag state for this make invocation, borrowed (`&options`)
    // through the call graph. Replaces the former `static mut FLAGS` global.
    let options = Options::new();
    // Owned execution context for this make invocation, threaded (`&ctx`) down
    // the call graph in place of the former process-global makelevel. It
    // starts at level 0 (matching the old startup default) and is rebuilt from
    // the parsed `MAKELEVEL` env var below.
    let mut ctx = crate::execctx::ExecContext::new(crate::execctx::Config { makelevel: 0 });
    // Install a borrow channel to `options` for the single deep makefile-time
    // callback (`set_special_var` -> `reset_makeflags`) that cannot receive an
    // `&Options` parameter. `options` itself remains the owner.
    OPTIONS_PTR.with(|p| p.set(&options as *const Options));
    initialize_variable_output();
    spin(b"main-entry\0" as *const u8 as *const ::core::ffi::c_char);
    if check_io_state() & 0x8 as ::core::ffi::c_uint != 0 {
        atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    }
    crate::output::output_init(&raw mut make_sync);
    initialize_stopchar_map();
    crate::warning::init();
    options.verify.set(true);
    setlocale(LC_ALL, b"\0" as *const u8 as *const ::core::ffi::c_char);
    sigemptyset(&raw mut fatal_signal_set);
    install_fatal_signal(1);
    install_fatal_signal(3);
    install_fatal_signal(13);
    install_fatal_signal(2);
    install_fatal_signal(15);
    install_fatal_signal(24);
    install_fatal_signal(25);
    bsd_signal(SIGCHLD, SIG_DFL);
    crate::output::output_init(::core::ptr::null_mut::<output>());
    if (*argv.offset(0_i32 as isize)).is_null() {
        let fresh33 = &mut (*argv.offset(0_i32 as isize));
        *fresh33 = b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    if *(*argv.offset(0_i32 as isize)).offset(0_i32 as isize) as i32 == 0 {
        program = b"make\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        program = strrchr(*argv.offset(0_i32 as isize), '/' as i32);
        if program.is_null() {
            program = *argv.offset(0_i32 as isize);
        } else {
            program = program.offset(1_i32 as isize);
        }
    }
    initialize_global_hash_tables();
    get_tmpdir(&ctx);
    if getcwd(
        &raw mut current_directory as *mut ::core::ffi::c_char,
        GET_PATH_MAX as size_t,
    )
    .is_null()
    {
        perror_with_name(
            &ctx,
            b"getcwd\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
        current_directory[0_i32 as usize] = 0;
        directory_before_chdir = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        directory_before_chdir = xstrdup(&raw mut current_directory as *mut ::core::ffi::c_char);
    }
    let fresh34 = &mut (*define_variable_in_set(
        &ctx,
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    ));
    (*fresh34).set_special(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let fresh35 = &mut (*define_variable_in_set(
        &ctx,
        b".VARIABLES\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 11]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    ));
    (*fresh35).set_special(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let fresh36 = &mut (*define_variable_in_set(
        &ctx,
        b".RECIPEPREFIX\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    ));
    (*fresh36).set_special(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let fresh37 = &mut (*define_variable_in_set(
        &ctx,
        b".WARNINGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    ));
    (*fresh37).set_special(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    define_variable_in_set(
        &ctx,
        b".SHELLFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t).wrapping_sub(1),
        b"-c\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        &ctx,
        b".LOADED\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    let features: *const ::core::ffi::c_char = b"target-specific order-only second-expansion else-if shortest-stem undefine oneshell nocomment grouped-target extra-prereqs notintermediate shell-export archives jobserver jobserver-fifo output-sync check-symlink maintainer\0"
        as *const u8 as *const ::core::ffi::c_char;
    define_variable_in_set(
        &ctx,
        b".FEATURES\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        features,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    guile_gmake_setup(NILF);
    let mut i: ::core::ffi::c_uint;
    i = 0;
    while !(*envp.offset(i as isize)).is_null() {
        let v: *mut variable;
        let mut ep: *const ::core::ffi::c_char = *envp.offset(i as isize);
        let mut export: variable_export = v_export;
        let len: size_t;
        while !(stopchar_map()[*ep as ::core::ffi::c_uchar as usize] as i32 & (0x20_i32 | 0x1_i32)
            != 0)
        {
            ep = ep.offset(1_i32 as isize);
        }
        if !(*ep as i32 == 0) {
            let fresh38 = ep;
            ep = ep.offset(1_i32 as isize);
            len = fresh38.offset_from(*envp.offset(i as isize)) as ::core::ffi::c_long as size_t;
            if len == 13
                && memcmp(
                    *envp.offset(i as isize) as *const ::core::ffi::c_void,
                    b"MAKE_RESTARTS\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
                ) == 0
            {
                if *ep as i32 == '-' as i32 {
                    set_stdio_traced(true);
                    ep = ep.offset(1_i32 as isize);
                }
                restarts = make_toui(::core::ffi::CStr::from_ptr(ep)).unwrap_or(0);
                export = v_noexport;
            }
            v = define_variable_in_set(
                &ctx,
                *envp.offset(i as isize),
                len,
                ep,
                o_env,
                1,
                (*current_variable_set_list).set,
                NILF,
            );
            if *(*v).name as i32 == *(b"SHELL\0" as *const u8 as *const ::core::ffi::c_char) as i32
                && (*(*v).name as i32 == 0
                    || strcmp(
                        (*v).name.offset(1_i32 as isize),
                        (b"SHELL\0" as *const u8 as *const ::core::ffi::c_char)
                            .offset(1_i32 as isize),
                    ) == 0)
            {
                export = v_noexport;
                shell_var.name = xstrdup(b"SHELL\0" as *const u8 as *const ::core::ffi::c_char);
                shell_var.length = 5;
                shell_var.value = xstrdup(ep);
            }
            (*v).set_export(export as variable_export);
        }
        i = i.wrapping_add(1);
    }
    if !lookup_variable(
        &ctx,
        b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
    )
    .is_null()
    {
        decode_env_switches(
            &ctx,
            &options,
            b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
            o_command,
        );
        define_variable_in_set(
            &ctx,
            b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            o_env,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
    }
    decode_env_switches(
        &ctx,
        &options,
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        o_command,
    );
    set_make_sync_syncout(
        (opt_output_sync() == OUTPUT_SYNC_LINE || opt_output_sync() == OUTPUT_SYNC_TARGET) as i32
            as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
    output_context = if make_sync_syncout() as i32 != 0 {
        &raw mut make_sync
    } else {
        ::core::ptr::null_mut::<output>()
    };
    let env_slots: Option<u32> = options.arg_job_slots.get();
    options.arg_job_slots.set(None);
    decode_switches(
        &ctx,
        &options,
        argc,
        argv as *mut *const ::core::ffi::c_char,
        o_command,
    );
    argv_slots = options.arg_job_slots.get();
    if options.arg_job_slots.get().is_none() {
        options.arg_job_slots.set(env_slots);
    }
    if options.print_usage.get() {
        print_usage(&ctx, &options, 0);
    }
    if options.print_version.get() {
        print_version();
        die(&ctx, MAKE_SUCCESS);
    }
    setvbuf(
        stdout,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        _IOLBF,
        BUFSIZ as size_t,
    );
    {
        let shuffle = options.shuffle_mode.borrow().clone();
        if let Some(arg) = shuffle {
            crate::shuffle::set_mode(&ctx, &arg);
            *options.shuffle_mode.borrow_mut() = crate::shuffle::get_mode();
        }
    }
    if isatty(fileno(stdout)) != 0
        && lookup_variable(
            &ctx,
            b"MAKE_TERMOUT\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        )
        .is_null()
    {
        let tty: *const ::core::ffi::c_char = ttyname(fileno(stdout));
        let fresh39 = &mut (*define_variable_in_set(
            &ctx,
            b"MAKE_TERMOUT\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
            if !tty.is_null() {
                tty
            } else {
                b"true\0" as *const u8 as *const ::core::ffi::c_char
            },
            o_default,
            0,
            (*current_variable_set_list).set,
            NILF,
        ));
        (*fresh39).set_export(v_export as variable_export);
    }
    if isatty(fileno(stderr)) != 0
        && lookup_variable(
            &ctx,
            b"MAKE_TERMERR\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        )
        .is_null()
    {
        let tty_0: *const ::core::ffi::c_char = ttyname(fileno(stderr));
        let fresh40 = &mut (*define_variable_in_set(
            &ctx,
            b"MAKE_TERMERR\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
            if !tty_0.is_null() {
                tty_0
            } else {
                b"true\0" as *const u8 as *const ::core::ffi::c_char
            },
            o_default,
            0,
            (*current_variable_set_list).set,
            NILF,
        ));
        (*fresh40).set_export(v_export as variable_export);
    }
    syncing = (opt_output_sync() == OUTPUT_SYNC_LINE || opt_output_sync() == OUTPUT_SYNC_TARGET)
        as i32 as ::core::ffi::c_uint;
    if make_sync_syncout() as i32 != 0 && syncing == 0 {
        crate::output::output_close(&ctx, &raw mut make_sync);
    }
    set_make_sync_syncout(syncing as ::core::ffi::c_uint);
    output_context = if make_sync_syncout() as i32 != 0 {
        &raw mut make_sync
    } else {
        ::core::ptr::null_mut::<output>()
    };
    let v_0: *mut variable = lookup_variable(
        &ctx,
        b"MAKELEVEL\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
    );
    let parsed_makelevel: u32 = if !v_0.is_null()
        && *(*v_0).value.offset(0_i32 as isize) as i32 != 0
        && *(*v_0).value.offset(0_i32 as isize) as i32 != '-' as i32
    {
        make_toui(::core::ffi::CStr::from_ptr((*v_0).value)).unwrap_or(0)
    } else {
        0
    };
    ctx = crate::execctx::ExecContext::new(crate::execctx::Config {
        makelevel: parsed_makelevel,
    });
    ctx.always_make_flag.set(options.always_make.get() && restarts == 0);
    if options.no_builtin_variables.get() {
        options.no_builtin_rules.set(true);
    }
    if 0x1_i32 & db_level != 0 {
        print_version();
        fflush(stdout);
    }
    if current_directory[0_i32 as usize] as i32 != 0
        && !(*argv.offset(0_i32 as isize)).is_null()
        && *(*argv.offset(0_i32 as isize)).offset(0_i32 as isize) as i32 != '/' as i32
        && !strchr(*argv.offset(0_i32 as isize), '/' as i32).is_null()
    {
        let fresh41 = &mut (*argv.offset(0_i32 as isize));
        *fresh41 = xstrdup(concat(
            3,
            &raw mut current_directory as *mut ::core::ffi::c_char,
            b"/\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(0_i32 as isize),
        ));
    }
    starting_directory = &raw mut current_directory as *mut ::core::ffi::c_char;
    if !options.directories.borrow().is_empty() {
        for entry in options.directories.borrow().iter() {
            let dir: *const ::core::ffi::c_char = entry.as_ptr();
            if chdir(dir) < 0 {
                pfatal_with_name(&ctx, dir);
            }
        }
    }
    if !options.directories.borrow().is_empty() {
        if getcwd(
            &raw mut current_directory as *mut ::core::ffi::c_char,
            GET_PATH_MAX as size_t,
        )
        .is_null()
        {
            perror_with_name(
                &ctx,
                b"getcwd\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
            );
            starting_directory = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            starting_directory = &raw mut current_directory as *mut ::core::ffi::c_char;
        }
    }
    define_variable_in_set(
        &ctx,
        b"CURDIR\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1),
        &raw mut current_directory as *mut ::core::ffi::c_char,
        o_file,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    {
        let include_dirs = options.include_dirs.borrow();
        let inc_paths: Vec<std::path::PathBuf> = include_dirs
            .iter()
            .map(|s| {
                use std::os::unix::ffi::OsStrExt;
                std::path::PathBuf::from(std::ffi::OsStr::from_bytes(s.as_bytes()))
            })
            .collect();
        construct_include_path(&ctx, &inc_paths);
    }
    if options.jobserver_auth.borrow().is_some() {
        // Reset the jobserver unless we successfully inherited the parent's.
        let mut do_reset = true;
        if argv_slots.is_none() {
            let auth = options.jobserver_auth.borrow().clone().unwrap();
            let auth_c = ::std::ffi::CString::new(auth.as_bytes()).unwrap_or_default();
            if jobserver_parse_auth(&ctx, auth_c.as_ptr()) != 0 {
                do_reset = false;
            } else {
                error(
                    &ctx,
                    ::core::ptr::null_mut::<Floc>(),
                    0,
                    b"warning: jobserver unavailable: using -j1 (add '+' to parent make rule)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                options.arg_job_slots.set(Some(1));
            }
        } else if restarts == 0 && argv_slots != Some(1) {
            error(
                &ctx,
                ::core::ptr::null_mut::<Floc>(),
                INTSTR_LENGTH,
                b"warning: -j%d forced in submake: resetting jobserver mode\0" as *const u8
                    as *const ::core::ffi::c_char,
                argv_slots.unwrap_or(0),
            );
        }
        if do_reset {
            reset_jobserver(&options);
        }
    }
    define_variable_in_set(
        &ctx,
        b"MAKE_COMMAND\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        *argv.offset(0_i32 as isize),
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        &ctx,
        b"MAKE\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1),
        b"$(MAKE_COMMAND)\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    if !command_variables.is_null() {
        let mut cv: *mut command_variable;
        let mut v_1: *mut variable;
        let mut len_0: size_t = 0;
        let mut p: *mut ::core::ffi::c_char;
        cv = command_variables;
        while !cv.is_null() {
            v_1 = (*cv).variable;
            len_0 = len_0.wrapping_add((2 as size_t).wrapping_mul(strlen((*v_1).name)) as size_t);
            if (*v_1).recursive() == 0 {
                len_0 = len_0.wrapping_add(1);
            }
            len_0 = len_0.wrapping_add(1);
            len_0 = len_0.wrapping_add((2 as size_t).wrapping_mul(strlen((*v_1).value)) as size_t);
            len_0 = len_0.wrapping_add(1);
            cv = (*cv).next;
        }
        // Owned encoding scratch (was xmalloc + free); define_variable_in_set
        // copies the value, so the buffer is only needed locally.
        let mut value_buf: Vec<u8> = Vec::with_capacity(len_0 as usize);
        let value = value_buf.as_mut_ptr() as *mut ::core::ffi::c_char;
        p = value;
        cv = command_variables;
        while !cv.is_null() {
            v_1 = (*cv).variable;
            p = quote_for_env(p, (*v_1).name);
            if (*v_1).recursive() == 0 {
                let fresh42 = p;
                p = p.offset(1_i32 as isize);
                *fresh42 = ':' as i32 as ::core::ffi::c_char;
            }
            let fresh43 = p;
            p = p.offset(1_i32 as isize);
            *fresh43 = '=' as i32 as ::core::ffi::c_char;
            p = quote_for_env(p, (*v_1).value);
            let fresh44 = p;
            p = p.offset(1_i32 as isize);
            *fresh44 = ' ' as i32 as ::core::ffi::c_char;
            cv = (*cv).next;
        }
        *p.offset(-1_i32 as isize) = 0;
        define_variable_in_set(
            &ctx,
            b"-*-command-variables-*-\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 24]>() as size_t).wrapping_sub(1),
            value,
            o_automatic,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
        drop(value_buf);
        define_variable_in_set(
            &ctx,
            b"MAKEOVERRIDES\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
            b"${-*-command-variables-*-}\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            1,
            (*current_variable_set_list).set,
            NILF,
        );
    }
    if !options.makefiles.borrow().is_empty() {
        let mut i_1: usize;
        i_1 = 0;
        while i_1 < options.makefiles.borrow().len() {
            if options.makefiles.borrow()[i_1].as_bytes() == b"-" {
                let outfile: *mut FILE;
                let mut newnm: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                if stdin_offset >= 0 {
                    fatal(
                        &ctx,
                        ::core::ptr::null_mut::<Floc>(),
                        0,
                        b"Makefile from standard input specified twice\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                outfile = get_tmpfile(&ctx, &raw mut newnm);
                if outfile.is_null() {
                    fatal(
                        &ctx,
                        ::core::ptr::null_mut::<Floc>(),
                        0,
                        b"cannot store makefile from stdin to a temporary file\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                while feof(stdin) == 0 && ferror(stdin) == 0 {
                    let mut buf: [::core::ffi::c_char; 2048] = [0; 2048];
                    let n: size_t = fread(
                        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        1,
                        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
                        stdin,
                    ) as size_t;
                    if n > 0
                        && fwrite(
                            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                            1,
                            n as size_t,
                            outfile,
                        ) as size_t
                            != n
                    {
                        fatal(
                            &ctx,
                            ::core::ptr::null_mut::<Floc>(),
                            (strlen(newnm) as size_t)
                                .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                            b"fwrite: temporary file %s: %s\0" as *const u8
                                as *const ::core::ffi::c_char,
                            newnm,
                            strerror(*__errno_location()),
                        );
                    }
                }
                fclose(outfile);
                let cached = strcache_add(newnm);
                options.makefiles.borrow_mut()[i_1] =
                    ::core::ffi::CStr::from_ptr(cached).to_owned();
                stdin_offset = i_1 as i32;
                temp_stdin_name = cached;
                free(newnm as *mut ::core::ffi::c_void);
            }
            i_1 = i_1.wrapping_add(1);
        }
    }
    if stdin_offset >= 0 {
        let f: *mut file = enter_file(strcache_add(
            options.makefiles.borrow()[stdin_offset as usize].as_ptr(),
        ));
        (*f).set_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*f).set_update_status(us_success as update_status);
        (*f).set_command_state(cs_finished as cmd_state);
        (*f).set_intermediate(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*f).set_dontcare(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*f).mtime_before_update = f_mtime(&ctx, f, 0);
        (*f).last_mtime = (*f).mtime_before_update;
    }
    bsd_signal(
        SIGCHLD,
        Some(child_handler as unsafe extern "C" fn(i32) -> ()),
    );
    let mut block: sigset_t = __sigset_t { __val: [0; 16] };
    sigemptyset(&raw mut block);
    sigaddset(&raw mut block, SIGCHLD);
    if sigprocmask(
        SIG_SETMASK,
        &raw mut block,
        ::core::ptr::null_mut::<sigset_t>(),
    ) < 0
    {
        pfatal_with_name(
            &ctx,
            b"sigprocmask(SIG_SETMASK, SIGCHLD)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    bsd_signal(
        SIGUSR1,
        Some(debug_signal_handler as unsafe extern "C" fn(i32) -> ()),
    );
    set_default_suffixes(&ctx, &options);
    define_automatic_variables(&ctx);
    let fresh46 = &mut (*define_makeflags(&ctx, &options, 0));
    (*fresh46).set_export(v_export as variable_export);
    define_default_variables(&ctx, &options);
    default_file = enter_file(strcache_add(
        b".DEFAULT\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    default_goal_var = define_variable_in_set(
        &ctx,
        b".DEFAULT_GOAL\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_file,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    if !options.eval_strings.borrow().is_empty() {
        let eval_strings = options.eval_strings.borrow();
        let mut p_0: *mut ::core::ffi::c_char;
        let mut endp: *mut ::core::ffi::c_char;
        let mut len_1: size_t = (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t)
            .wrapping_sub(1)
            .wrapping_add(1)
            .wrapping_mul(eval_strings.len() as size_t);
        for es in eval_strings.iter() {
            // Own a mutable, NUL-terminated copy of the eval string instead of
            // xstrdup + free: `eval_buffer` parses it in place (only shrinking
            // it), and the `Vec`'s Drop reclaims the buffer at end of scope —
            // RAII in place of the manual malloc/free pair.
            let mut owned: Vec<u8> = es.as_bytes_with_nul().to_vec();
            len_1 = len_1.wrapping_add((2 as size_t).wrapping_mul((owned.len() - 1) as size_t));
            eval_buffer(
                &ctx,
                owned.as_mut_ptr() as *mut ::core::ffi::c_char,
                ::core::ptr::null::<Floc>(),
            );
        }
        let mut value_0_buf: Vec<u8> = Vec::with_capacity(len_1 as usize);
        let value_0 = value_0_buf.as_mut_ptr() as *mut ::core::ffi::c_char;
        endp = value_0;
        p_0 = endp;
        for es in eval_strings.iter() {
            p_0 = stpcpy(p_0, b"--eval=\0" as *const u8 as *const ::core::ffi::c_char);
            p_0 = quote_for_env(p_0, es.as_ptr());
            let fresh47 = p_0;
            p_0 = p_0.offset(1_i32 as isize);
            endp = fresh47;
            *endp = ' ' as i32 as ::core::ffi::c_char;
        }
        *endp = 0;
        define_variable_in_set(
            &ctx,
            b"-*-eval-flags-*-\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 17]>() as size_t).wrapping_sub(1),
            value_0,
            o_automatic,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
        drop(value_0_buf);
    }
    let old_arg_job_slots: Option<u32> = options.arg_job_slots.get();
    options
        .prev_no_builtin_rules
        .set(options.no_builtin_rules.get());
    options
        .prev_no_builtin_variables
        .set(options.no_builtin_variables.get());
    // Intern each makefile name in the strcache so the pointers handed to
    // read_all_makefiles (and stored as floc.filenm during eval) stay valid
    // for the whole run, matching the C code where makefiles->list holds
    // strcache'd pointers. Using the raw CString as_ptr() here would dangle
    // once the mirror-back below replaces the CString.
    let mut mf_ptrs: Vec<*const ::core::ffi::c_char> = options
        .makefiles
        .borrow()
        .iter()
        .map(|s| strcache_add(s.as_ptr()))
        .collect();
    let makefiles_empty = options.makefiles.borrow().is_empty();
    read_files = read_all_makefiles(
        &ctx,
        if makefiles_empty {
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>()
        } else {
            mf_ptrs.push(::core::ptr::null());
            mf_ptrs.as_mut_ptr()
        },
    );
    // `read_all_makefiles` rewrites each array entry in place to the actual
    // (strcache'd) makefile name it resolved/remade. Mirror those updates back
    // into `options.makefiles` so the restart path emits the resolved names.
    if !makefiles_empty {
        let mut makefiles = options.makefiles.borrow_mut();
        for (i, &ptr) in mf_ptrs.iter().enumerate() {
            if ptr.is_null() {
                break;
            }
            makefiles[i] = ::core::ffi::CStr::from_ptr(ptr).to_owned();
        }
    }
    options.arg_job_slots.set(None);
    decode_env_switches(
        &ctx,
        &options,
        b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        o_env,
    );
    define_variable_in_set(
        &ctx,
        b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_override,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    decode_env_switches(
        &ctx,
        &options,
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        o_env,
    );
    if options.arg_job_slots.get().is_none() || argv_slots.is_some() {
        options.arg_job_slots.set(old_arg_job_slots);
    } else if options.jobserver_auth.borrow().is_some()
        && options.arg_job_slots.get() != old_arg_job_slots
    {
        if restarts == 0 {
            error(
                &ctx,
                ::core::ptr::null_mut::<Floc>(),
                INTSTR_LENGTH,
                b"warning: -j%d forced in makefile: resetting jobserver mode\0" as *const u8
                    as *const ::core::ffi::c_char,
                options.arg_job_slots.get().unwrap_or(0),
            );
        }
        reset_jobserver(&options);
    }
    syncing = (opt_output_sync() == OUTPUT_SYNC_LINE || opt_output_sync() == OUTPUT_SYNC_TARGET)
        as i32 as ::core::ffi::c_uint;
    if make_sync_syncout() as i32 != 0 && syncing == 0 {
        crate::output::output_close(&ctx, &raw mut make_sync);
    }
    set_make_sync_syncout(syncing as ::core::ffi::c_uint);
    output_context = if make_sync_syncout() as i32 != 0 {
        &raw mut make_sync
    } else {
        ::core::ptr::null_mut::<output>()
    };
    disable_builtins(&ctx, &options);
    options.job_slots.set(if options.jobserver_auth.borrow().is_some() {
        0
    } else if options.arg_job_slots.get().is_none() {
        1
    } else {
        options.arg_job_slots.get().unwrap()
    });
    if options.job_slots.get() > 1 {
        let style_c = options
            .jobserver_style
            .borrow()
            .as_ref()
            .and_then(|s| ::std::ffi::CString::new(s.as_bytes()).ok());
        let style_ptr = style_c
            .as_ref()
            .map(|c| c.as_ptr())
            .unwrap_or(::core::ptr::null());
        if jobserver_setup(&ctx, options.job_slots.get().wrapping_sub(1) as i32, style_ptr) != 0 {
            let auth = jobserver_get_auth();
            if !auth.is_null() {
                *options.jobserver_auth.borrow_mut() = Some(
                    ::core::ffi::CStr::from_ptr(auth)
                        .to_string_lossy()
                        .into_owned(),
                );
                free(auth as *mut ::core::ffi::c_void);
                options.master_job_slots.set(options.job_slots.get());
                options.job_slots.set(0);
            }
        }
    }
    if syncing != 0 && options.job_slots.get() == 1 {
        output_context = ::core::ptr::null_mut::<output>();
        crate::output::output_close(&ctx, &raw mut make_sync);
        syncing = 0;
        options.output_sync.set(OUTPUT_SYNC_NONE);
    }
    if syncing != 0 {
        let has_mutex = options.sync_mutex.borrow().is_some();
        if !has_mutex {
            osync_setup(&ctx);
            let m = osync_get_mutex();
            if !m.is_null() {
                *options.sync_mutex.borrow_mut() = Some(
                    ::core::ffi::CStr::from_ptr(m)
                        .to_string_lossy()
                        .into_owned(),
                );
                free(m as *mut ::core::ffi::c_void);
            }
        } else {
            let mtx = options.sync_mutex.borrow().clone().unwrap();
            let mtx_c = ::std::ffi::CString::new(mtx.as_bytes()).unwrap_or_default();
            if osync_parse_mutex(&ctx, mtx_c.as_ptr()) == 0 {
                osync_clear();
                *options.sync_mutex.borrow_mut() = None;
            }
        }
    }
    if options.jobserver_auth.borrow().is_some() && (0x2_i32 | 0x4_i32) & db_level != 0 {
        let auth = options.jobserver_auth.borrow().clone().unwrap();
        let auth_c = ::std::ffi::CString::new(auth.as_bytes()).unwrap_or_default();
        printf(
            b"Using jobserver controller %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            auth_c.as_ptr(),
        );
        fflush(stdout);
    }
    if options.sync_mutex.borrow().is_some() && 0x2_i32 & db_level != 0 {
        let mtx = options.sync_mutex.borrow().clone().unwrap();
        let mtx_c = ::std::ffi::CString::new(mtx.as_bytes()).unwrap_or_default();
        printf(
            b"Using output-sync mutex %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            mtx_c.as_ptr(),
        );
        fflush(stdout);
    }
    define_makeflags(&ctx, &options, 0);
    snap_deps(&ctx);
    install_default_suffix_rules(&options);
    convert_to_pattern(&ctx);
    install_default_implicit_rules(&ctx, &options);
    snap_implicit_rules(&ctx);
    build_vpath_lists(&ctx);
    if !options.old_files.borrow().is_empty() {
        for of in options.old_files.borrow().iter() {
            let f_0: *mut file = enter_file(strcache_add(of.as_ptr()));
            (*f_0).mtime_before_update = OLD_MTIME as uintmax_t;
            (*f_0).last_mtime = (*f_0).mtime_before_update;
            (*f_0).set_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*f_0).set_update_status(us_success as update_status);
            (*f_0).set_command_state(cs_finished as cmd_state);
        }
    }
    if options.print_targets.get() {
        print_targets();
        die(&ctx, EXIT_SUCCESS);
    }
    if restarts == 0 && !options.new_files.borrow().is_empty() {
        for nf in options.new_files.borrow().iter() {
            let f_1: *mut file = enter_file(strcache_add(nf.as_ptr()));
            (*f_1).mtime_before_update =
                (!(0_i32 as uintmax_t)).wrapping_sub(if !(-1_i32 as uintmax_t <= 0 as uintmax_t) {
                    0_i32 as uintmax_t
                } else {
                    !(0_i32 as uintmax_t)
                        << (::core::mem::size_of::<uintmax_t>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(1_usize)
                });
            (*f_1).last_mtime = (*f_1).mtime_before_update;
        }
    }
    remote_setup();
    output_context = ::core::ptr::null_mut::<output>();
    crate::output::output_close(&ctx, &raw mut make_sync);
    if options.shuffle_mode.borrow().is_some() && 0x1_i32 & db_level != 0 {
        let sm = options.shuffle_mode.borrow().clone().unwrap();
        let sm_c = ::std::ffi::CString::new(sm.as_bytes()).unwrap_or_default();
        printf(
            b"Enabled shuffle mode: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            sm_c.as_ptr(),
        );
        fflush(stdout);
    }
    if !read_files.is_null() {
        let makefile_mtimes: *mut uintmax_t;
        let mut skipped_makefiles: *mut goaldep = ::core::ptr::null_mut::<goaldep>();
        let mut nargv: *mut *const ::core::ffi::c_char = argv as *mut *const ::core::ffi::c_char;
        let mut any_failed: i32 = 0;
        let mut status: update_status;
        if 0x1_i32 & db_level != 0 {
            printf(b"Updating makefiles....\n\0" as *const u8 as *const ::core::ffi::c_char);
            fflush(stdout);
        }
        let mut num_mkfiles: ::core::ffi::c_uint = 0;
        let mut d: *mut goaldep = read_files;
        read_files = ::core::ptr::null_mut::<goaldep>();
        while !d.is_null() {
            let t: *mut goaldep = d;
            d = (*d).next;
            (*t).next = read_files;
            read_files = t;
            num_mkfiles = num_mkfiles.wrapping_add(1);
        }
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (num_mkfiles as usize).wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
                as usize,
        ));
        makefile_mtimes = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut uintmax_t;
        let mut d_0: *mut goaldep = read_files;
        let mut last: *mut goaldep = ::core::ptr::null_mut::<goaldep>();
        let mut mm_idx: ::core::ffi::c_uint = 0;
        while let Some(d0r) = d_0.as_mut() {
            let mut skip: i32 = 0;
            let mut f_2: *mut file = d0r.file;
            let Some(f2_init) = f_2.as_ref() else { break };
            if f2_init.phony() != 0 {
                skip = 1;
            } else {
                f_2 = f2_init.double_colon;
                while let Some(f2r) = f_2.as_ref() {
                    if f2r.deps.is_null() && !f2r.cmds.is_null() {
                        skip = 1;
                        break;
                    } else {
                        f_2 = f2r.prev;
                    }
                }
            }
            if skip == 0 {
                let fresh48 = mm_idx;
                mm_idx = mm_idx.wrapping_add(1);
                *makefile_mtimes.offset(fresh48 as isize) =
                    if f2_init.last_mtime == UNKNOWN_MTIME as uintmax_t {
                        f_mtime(&ctx, d0r.file, 0)
                    } else {
                        f2_init.last_mtime
                    };
                last = d_0;
                d_0 = d0r.next;
            } else {
                if 0x2_i32 & db_level != 0 {
                    printf(
                        b"Makefile '%s' might loop; not remaking it.\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        f_2.as_ref().map_or(::core::ptr::null(), |f2r| f2r.name),
                    );
                    fflush(stdout);
                }
                if let Some(lastr) = last.as_mut() {
                    lastr.next = d0r.next;
                } else {
                    read_files = d0r.next;
                }
                if d0r.error != 0 && d0r.flags() as i32 & RM_DONTCARE == 0 {
                    d0r.next = skipped_makefiles;
                    skipped_makefiles = d_0;
                    any_failed = 1;
                } else {
                    free_goaldep(d_0);
                }
                d_0 = match last.as_ref() {
                    Some(lastr) => lastr.next,
                    None => read_files,
                };
            }
        }
        define_makeflags(&ctx, &options, 1);
        let orig_db_level: i32 = db_level;
        if 0x100_i32 & db_level == 0 {
            db_level = DB_NONE;
        }
        options.rebuilding_makefiles.set(true);
        status = update_goal_chain(&ctx, read_files) as update_status;
        options.rebuilding_makefiles.set(false);
        db_level = orig_db_level;
        while !skipped_makefiles.is_null() {
            let d_1: *mut goaldep = skipped_makefiles;
            let Some(d_1r) = d_1.as_mut() else { break };
            let err: *const ::core::ffi::c_char = strerror(d_1r.error);
            let d1_name: *const ::core::ffi::c_char = if !d_1r.name.is_null() {
                d_1r.name
            } else {
                d_1r.file.as_ref().map_or(::core::ptr::null(), |fr| fr.name)
            };
            error(
                &ctx,
                &raw mut d_1r.floc,
                (strlen(d1_name) as size_t).wrapping_add(strlen(err) as size_t),
                b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                d1_name,
                err,
            );
            skipped_makefiles = d_1r.next;
            free_goaldep(d_1);
        }
        if any_failed != 0
            && status as ::core::ffi::c_uint == us_success as i32 as ::core::ffi::c_uint
        {
            status = us_none;
        }
        let needs_restart = match status as ::core::ffi::c_uint {
            1 => {
                let mut d_2: *mut goaldep;
                d_2 = read_files;
                while let Some(d_2r) = d_2.as_mut() {
                    let f_3: *mut file = d_2r.file;
                    if let Some(f3r) = f_3.as_mut() {
                        if f3r.unloaded() != 0 {
                            if load_file(&ctx, &raw mut d_2r.floc, f_3, 0) == 0 {
                                fatal(
                                    &ctx,
                                    &raw mut d_2r.floc,
                                    strlen(f3r.name) as size_t,
                                    b"%s: failed to load\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    f3r.name,
                                );
                            }
                            f3r.set_unloaded(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            f3r.set_loaded(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        }
                    }
                    d_2 = d_2r.next;
                }
                false
            }
            3 => {
                let mut any_remade: i32 = 0;
                let mut i_3: ::core::ffi::c_uint;
                let mut d_4: *mut goaldep;
                i_3 = 0;
                d_4 = read_files;
                while let Some(d_4r) = d_4.as_mut() {
                    let f_4: *mut file = d_4r.file;
                    if f_4.as_ref().is_some_and(|f4r| f4r.updated() != 0) {
                        let f4r = f_4.as_ref().expect("f_4 checked non-null above");
                        if f4r.update_status() as i32 == us_success as i32 {
                            any_remade |= ((if f4r.last_mtime == UNKNOWN_MTIME as uintmax_t {
                                f_mtime(&ctx, f_4, 0)
                            } else {
                                f4r.last_mtime
                            }) != *makefile_mtimes.offset(i_3 as isize))
                                as i32;
                        } else if d_4r.flags() as i32 & RM_DONTCARE == 0 {
                            error(
                                &ctx,
                                &raw mut d_4r.floc,
                                strlen(f4r.name) as size_t,
                                b"failed to remake makefile '%s'\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                f4r.name,
                            );
                            let mtime: uintmax_t = if f4r.last_mtime == UNKNOWN_MTIME as uintmax_t {
                                f_mtime(&ctx, f_4, 0)
                            } else {
                                f4r.last_mtime
                            };
                            any_remade |= (mtime != NONEXISTENT_MTIME as uintmax_t
                                && mtime != *makefile_mtimes.offset(i_3 as isize))
                                as i32;
                            makefile_status = MAKE_FAILURE;
                            any_failed = 1;
                        }
                    } else if d_4r.flags() as i32 & RM_DONTCARE == 0 {
                        let dnm: *const ::core::ffi::c_char = if !d_4r.name.is_null() {
                            d_4r.name
                        } else {
                            f_4.as_ref().map_or(::core::ptr::null(), |f4r| f4r.name)
                        };
                        if d_4r.flags() as i32 & RM_INCLUDED != 0 {
                            error(
                                &ctx,
                                &raw mut d_4r.floc,
                                strlen(dnm) as size_t,
                                b"included makefile '%s' was not found\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                dnm,
                            );
                        } else {
                            error(
                                &ctx,
                                ::core::ptr::null_mut::<Floc>(),
                                strlen(dnm) as size_t,
                                b"makefile '%s' was not found\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                dnm,
                            );
                            any_failed = 1;
                        }
                    }
                    i_3 = i_3.wrapping_add(1);
                    d_4 = d_4r.next;
                }
                any_remade != 0
            }
            0 => true,
            2 | _ => false,
        };
        if needs_restart {
            remove_intermediates(&ctx, 0);
            if options.print_data_base.get() {
                print_data_base(&ctx);
            }
            clean_jobserver(&ctx, 0);
            if !options.makefiles.borrow().is_empty() {
                let mut mfidx: i32 = 0;
                let mut av: *mut *mut ::core::ffi::c_char = argv;
                let mut nv: *mut *const ::core::ffi::c_char;
                alloca_allocations.push(::std::vec::from_elem(
                    0,
                    (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        .wrapping_mul((argc + 1 + 1) as usize) as usize,
                ));
                nargv = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                    as *mut *const ::core::ffi::c_char;
                nv = nargv;
                let fresh49 = av;
                av = av.offset(1_i32 as isize);
                let fresh50 = nv;
                nv = nv.offset(1_i32 as isize);
                *fresh50 = *fresh49;
                while !(*av).is_null() {
                    let f_4: *mut ::core::ffi::c_char;
                    let a: *mut ::core::ffi::c_char = *av;
                    // mf is only consumed inside the -f/--file substitution
                    // branches (where mfidx is a valid index); for other argv
                    // elements the C code harmlessly read past the list, so
                    // fall back to null rather than panicking on bounds.
                    let mf: *const ::core::ffi::c_char = options
                        .makefiles
                        .borrow()
                        .get(mfidx as usize)
                        .map_or(::core::ptr::null(), |s| s.as_ptr());
                    if strlen(a) > 0 {
                    } else {
                        panic!("assertion failed: strlen (a) > 0");
                    };
                    *nv = a;
                    if !(*a.offset(0_i32 as isize) as i32 != '-' as i32) {
                        if *a.offset(1_i32 as isize) as i32 == '-' as i32 {
                            // Rewrite -f/--file long options so the restart
                            // reads the makefile we just remade.
                            let substitute = if strcmp(
                                a,
                                b"--file\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0
                                || strcmp(
                                    a,
                                    b"--makefile\0" as *const u8 as *const ::core::ffi::c_char,
                                ) == 0
                            {
                                av = av.offset(1_i32 as isize);
                                true
                            } else {
                                strncmp(
                                    a,
                                    b"--file=\0" as *const u8 as *const ::core::ffi::c_char,
                                    7,
                                ) == 0
                                    || strncmp(
                                        a,
                                        b"--makefile=\0" as *const u8 as *const ::core::ffi::c_char,
                                        11,
                                    ) == 0
                            };
                            if substitute {
                                if mfidx == stdin_offset {
                                    alloca_allocations.push(::std::vec::from_elem(
                                        0,
                                        ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                                            .wrapping_sub(1)
                                            .wrapping_add(strlen(mf))
                                            .wrapping_add(1),
                                    ));
                                    let na: *mut ::core::ffi::c_char =
                                        alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                            as *mut ::core::ffi::c_char;
                                    sprintf(
                                        na,
                                        b"--temp-stdin=%s\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        mf,
                                    );
                                    *nv = na;
                                } else {
                                    alloca_allocations.push(::std::vec::from_elem(
                                        0,
                                        strlen(mf).wrapping_add(3) as usize,
                                    ));
                                    let na_0: *mut ::core::ffi::c_char =
                                        alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                            as *mut ::core::ffi::c_char;
                                    sprintf(
                                        na_0,
                                        b"-f%s\0" as *const u8 as *const ::core::ffi::c_char,
                                        mf,
                                    );
                                    *nv = na_0;
                                }
                                mfidx += 1;
                            }
                        } else {
                            f_4 = strchr(a, 'f' as i32);
                            if !f_4.is_null() {
                                if *f_4.offset(1_i32 as isize) as i32 == 0 {
                                    av = av.offset(1_i32 as isize);
                                }
                                if mfidx == stdin_offset {
                                    let al: size_t =
                                        f_4.offset_from(a) as ::core::ffi::c_long as size_t;
                                    let mut na_1: *mut ::core::ffi::c_char;
                                    if al > 1 {
                                        alloca_allocations.push(::std::vec::from_elem(
                                            0,
                                            al.wrapping_add(1) as usize,
                                        ));
                                        na_1 = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                            as *mut ::core::ffi::c_char;
                                        memcpy(
                                            na_1 as *mut ::core::ffi::c_void,
                                            a as *const ::core::ffi::c_void,
                                            al as size_t,
                                        );
                                        *na_1.add(al) = 0;
                                        let fresh51 = nv;
                                        nv = nv.offset(1_i32 as isize);
                                        *fresh51 = na_1;
                                    }
                                    alloca_allocations.push(::std::vec::from_elem(
                                        0,
                                        ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                                            .wrapping_sub(1)
                                            .wrapping_add(strlen(mf))
                                            .wrapping_add(1),
                                    ));
                                    na_1 = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                        as *mut ::core::ffi::c_char;
                                    sprintf(
                                        na_1,
                                        b"--temp-stdin=%s\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        mf,
                                    );
                                    *nv = na_1;
                                } else if *f_4.offset(1_i32 as isize) as i32 == 0 {
                                    nv = nv.offset(1_i32 as isize);
                                    *nv = mf;
                                } else {
                                    let al_0: size_t =
                                        (f_4.offset_from(a) as ::core::ffi::c_long + 1) as size_t;
                                    let ml: size_t = (strlen(mf) as size_t).wrapping_add(1);
                                    alloca_allocations.push(::std::vec::from_elem(
                                        0,
                                        al_0.wrapping_add(ml) as usize,
                                    ));
                                    let na_2: *mut ::core::ffi::c_char =
                                        alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                            as *mut ::core::ffi::c_char;
                                    memcpy(
                                        na_2 as *mut ::core::ffi::c_void,
                                        a as *const ::core::ffi::c_void,
                                        al_0 as size_t,
                                    );
                                    memcpy(
                                        na_2.add(al_0) as *mut ::core::ffi::c_void,
                                        mf as *const ::core::ffi::c_void,
                                        ml as size_t,
                                    );
                                    *nv = na_2;
                                }
                                mfidx += 1;
                            }
                        }
                    }
                    av = av.offset(1_i32 as isize);
                    nv = nv.offset(1_i32 as isize);
                }
                *nv = ::core::ptr::null::<::core::ffi::c_char>();
            }
            if !options.directories.borrow().is_empty() {
                let mut bad: i32 = 1;
                if !directory_before_chdir.is_null() {
                    if chdir(directory_before_chdir) < 0 {
                        perror_with_name(
                            &ctx,
                            b"chdir\0" as *const u8 as *const ::core::ffi::c_char,
                            b"\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        bad = 0;
                    }
                }
                if bad != 0 {
                    fatal(
                        &ctx,
                        ::core::ptr::null_mut::<Floc>(),
                        0,
                        b"couldn't change back to original directory\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            }
            restarts = restarts.wrapping_add(1);
            if 0x1_i32 & db_level != 0 {
                let mut p_3: *mut *const ::core::ffi::c_char;
                printf(
                    b"Re-executing[%u]:\0" as *const u8 as *const ::core::ffi::c_char,
                    restarts,
                );
                p_3 = nargv;
                while !(*p_3).is_null() {
                    printf(b" %s\0" as *const u8 as *const ::core::ffi::c_char, *p_3);
                    p_3 = p_3.offset(1_i32 as isize);
                }
                putchar('\n' as i32);
                fflush(stdout);
            }
            let mut p_4: *mut *mut ::core::ffi::c_char;
            p_4 = environ;
            while !(*p_4).is_null() {
                if strncmp(
                    *p_4,
                    b"MAKELEVEL=\0" as *const u8 as *const ::core::ffi::c_char,
                    (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t)
                        .wrapping_sub(1)
                        .wrapping_add(1),
                ) == 0
                {
                    alloca_allocations.push(::std::vec::from_elem(
                        0,
                        40_i32 as ::core::ffi::c_ulong as usize,
                    ));
                    *p_4 = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                        as *mut ::core::ffi::c_char;
                    sprintf(
                        *p_4,
                        b"%s=%u\0" as *const u8 as *const ::core::ffi::c_char,
                        MAKELEVEL_NAME.as_ptr(),
                        ctx.makelevel(),
                    );
                } else if strncmp(
                    *p_4,
                    b"MAKE_RESTARTS=\0" as *const u8 as *const ::core::ffi::c_char,
                    (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t).wrapping_sub(1),
                ) == 0
                {
                    alloca_allocations.push(::std::vec::from_elem(
                        0,
                        40_i32 as ::core::ffi::c_ulong as usize,
                    ));
                    *p_4 = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                        as *mut ::core::ffi::c_char;
                    sprintf(
                        *p_4,
                        b"MAKE_RESTARTS=%s%u\0" as *const u8 as *const ::core::ffi::c_char,
                        if stdio_traced() {
                            b"-\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        restarts,
                    );
                    restarts = 0;
                }
                p_4 = p_4.offset(1_i32 as isize);
            }
            if restarts != 0 {
                alloca_allocations.push(::std::vec::from_elem(
                    0,
                    40_i32 as ::core::ffi::c_ulong as usize,
                ));
                let b: *mut ::core::ffi::c_char =
                    alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
                sprintf(
                    b,
                    b"MAKE_RESTARTS=%s%u\0" as *const u8 as *const ::core::ffi::c_char,
                    if stdio_traced() {
                        b"-\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    restarts,
                );
                putenv(b);
            }
            fflush(stdout);
            fflush(stderr);
            osync_clear();
            jobserver_pre_child(1);
            exec_command(&ctx, nargv as *mut *mut ::core::ffi::c_char, environ);
            jobserver_post_child(1);
            temp_stdin_unlink(&ctx);
            _exit(127);
        }
        if any_failed != 0 {
            die(&ctx, MAKE_FAILURE);
        }
    }
    define_makeflags(&ctx, &options, 0);
    ctx.always_make_flag.set(options.always_make.get());
    if restarts != 0 && !options.new_files.borrow().is_empty() {
        for nf in options.new_files.borrow().iter() {
            let f_5: *mut file = enter_file(strcache_add(nf.as_ptr()));
            (*f_5).mtime_before_update =
                (!(0_i32 as uintmax_t)).wrapping_sub(if !(-1_i32 as uintmax_t <= 0 as uintmax_t) {
                    0_i32 as uintmax_t
                } else {
                    !(0_i32 as uintmax_t)
                        << (::core::mem::size_of::<uintmax_t>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(1_usize)
                });
            (*f_5).last_mtime = (*f_5).mtime_before_update;
        }
    }
    temp_stdin_unlink(&ctx);
    if goals.is_null() {
        let mut p_6: *mut ::core::ffi::c_char;
        if (*default_goal_var).recursive() != 0 {
            p_6 = expand_string_buf(
                &ctx,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                (*default_goal_var).value,
                SIZE_MAX as size_t,
            );
        } else {
            p_6 = variable_buffer_output(
                variable_buffer,
                (*default_goal_var).value,
                strlen((*default_goal_var).value) as size_t,
            );
            *p_6 = 0;
            p_6 = variable_buffer;
        }
        if *p_6 as i32 != 0 {
            let mut f_6: *mut file = lookup_file(p_6);
            if f_6.is_null() {
                let ns: *mut nameseq;
                ns = parse_file_seq(
                    &ctx,
                    &raw mut p_6,
                    ::core::mem::size_of::<nameseq>() as size_t,
                    MAP_NUL,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    PARSEFS_NONE,
                ) as *mut nameseq;
                if !ns.is_null() {
                    if !(*ns).next.is_null() {
                        fatal(
                            &ctx,
                            ::core::ptr::null_mut::<Floc>(),
                            0,
                            b".DEFAULT_GOAL contains more than one target\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    f_6 = enter_file(strcache_add((*ns).name));
                    (*ns).name = ::core::ptr::null::<::core::ffi::c_char>();
                    free_ns_chain(ns);
                }
            }
            if !f_6.is_null() {
                goals = alloc_goaldep();
                (*goals).file = f_6;
            }
        }
    } else {
        (*lastgoal).next = ::core::ptr::null_mut::<goaldep>();
    }
    if goals.is_null() {
        let v_2: *mut variable = lookup_variable(
            &ctx,
            b"MAKEFILE_LIST\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
        );
        if !v_2.is_null()
            && !(*v_2).value.is_null()
            && *(*v_2).value.offset(0_i32 as isize) as i32 != 0
        {
            fatal(
                &ctx,
                ::core::ptr::null_mut::<Floc>(),
                0,
                b"No targets\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        fatal(
            &ctx,
            ::core::ptr::null_mut::<Floc>(),
            0,
            b"No targets specified and no makefile found\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    crate::shuffle::shuffle_deps_recursive(goals as *mut crate::file::Dep);
    if 0x1_i32 & db_level != 0 {
        printf(b"Updating goal targets....\n\0" as *const u8 as *const ::core::ffi::c_char);
        fflush(stdout);
    }
    match update_goal_chain(&ctx, goals) as ::core::ffi::c_uint {
        2 => {
            makefile_status = MAKE_TROUBLE;
        }
        3 => {
            makefile_status = MAKE_FAILURE;
        }
        1 | 0 | _ => {}
    }
    if ctx.clock_skew_detected.get() {
        error(
            &ctx,
            ::core::ptr::null_mut::<Floc>(),
            0,
            b"warning: clock skew detected: your build may be incomplete\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    die(&ctx, makefile_status);
}
static mut getopt_shorts: [::core::ffi::c_char; 127] = [0; 127];
static mut long_options: [option; 51] = [option {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    has_arg: 0,
    flag: ::core::ptr::null::<i32>() as *mut i32,
    val: 0,
}; 51];
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn init_switches() {
    let mut p: *mut ::core::ffi::c_char;
    let mut c: ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint;
    if getopt_shorts[0_i32 as usize] as i32 != 0 {
        return;
    }
    p = &raw mut getopt_shorts as *mut ::core::ffi::c_char;
    let fresh24 = p;
    p = p.offset(1_i32 as isize);
    *fresh24 = '-' as i32 as ::core::ffi::c_char;
    i = 0;
    while switches[i as usize].c != 0 {
        long_options[i as usize].name = (if switches[i as usize].long_name.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            switches[i as usize].long_name
        }) as *mut ::core::ffi::c_char;
        long_options[i as usize].flag = ::core::ptr::null_mut::<i32>();
        long_options[i as usize].val = switches[i as usize].c;
        if switches[i as usize].c <= CHAR_MAX {
            let fresh25 = p;
            p = p.offset(1_i32 as isize);
            *fresh25 = switches[i as usize].c as ::core::ffi::c_char;
        }
        match switches[i as usize].type_0 as ::core::ffi::c_uint {
            0 | 1 | 7 => {
                long_options[i as usize].has_arg = no_argument;
            }
            2 | 3 | 4 | 5 | 6 => {
                if switches[i as usize].c <= CHAR_MAX {
                    let fresh26 = p;
                    p = p.offset(1_i32 as isize);
                    *fresh26 = ':' as i32 as ::core::ffi::c_char;
                }
                if !switches[i as usize].noarg_value.is_null() {
                    if switches[i as usize].c <= CHAR_MAX {
                        let fresh27 = p;
                        p = p.offset(1_i32 as isize);
                        *fresh27 = ':' as i32 as ::core::ffi::c_char;
                    }
                    long_options[i as usize].has_arg = optional_argument;
                } else {
                    long_options[i as usize].has_arg = required_argument;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    *p = 0;
    c = 0;
    while (c as usize)
        < (::core::mem::size_of::<[option; 9]>() as usize)
            .wrapping_div(::core::mem::size_of::<option>() as usize)
    {
        let fresh28 = i;
        i = i.wrapping_add(1);
        long_options[fresh28 as usize] = long_option_aliases[c as usize];
        c = c.wrapping_add(1);
    }
    long_options[i as usize].name = ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe fn handle_non_switch_argument(
    ctx: &crate::execctx::ExecContext,
    arg: *const ::core::ffi::c_char,
    origin: variable_origin,
) -> ::core::ffi::c_uint {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let v: *mut variable;
    if *arg.offset(0_i32 as isize) as i32 == '-' as i32 && *arg.offset(1_i32 as isize) as i32 == 0 {
        return 0;
    }
    v = try_variable_definition(ctx, ::core::ptr::null::<Floc>(), arg, origin, s_global);
    if !v.is_null() {
        let mut cv: *mut command_variable;
        cv = command_variables;
        while !cv.is_null() {
            if (*cv).variable == v {
                break;
            }
            cv = (*cv).next;
        }
        if cv.is_null() {
            cv = xmalloc(::core::mem::size_of::<command_variable>() as size_t)
                as *mut command_variable;
            (*cv).variable = v;
            (*cv).next = command_variables;
            command_variables = cv;
        }
    } else if *arg.offset(0_i32 as isize) as i32 != 0
        && origin as ::core::ffi::c_uint == o_command as i32 as ::core::ffi::c_uint
    {
        let f: *mut file;
        if strcmp(arg, b".WAIT\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            return 1;
        }
        f = enter_file(strcache_add(expand_command_line_file(ctx, arg)));
        (*f).set_cmd_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if goals.is_null() {
            goals = alloc_goaldep();
            lastgoal = goals;
        } else {
            (*lastgoal).next = alloc_goaldep();
            lastgoal = (*lastgoal).next;
        }
        (*lastgoal).file = f;
        let gv: *mut variable;
        let value: *const ::core::ffi::c_char;
        gv = lookup_variable(
            ctx,
            b"MAKECMDGOALS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        );
        if gv.is_null() {
            value = (*f).name;
        } else {
            let oldlen: size_t;
            let newlen: size_t;
            let vp: *mut ::core::ffi::c_char;
            oldlen = strlen((*gv).value) as size_t;
            newlen = strlen((*f).name) as size_t;
            alloca_allocations.push(::std::vec::from_elem(
                0,
                oldlen.wrapping_add(1).wrapping_add(newlen).wrapping_add(1) as usize,
            ));
            vp = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                vp as *mut ::core::ffi::c_void,
                (*gv).value as *const ::core::ffi::c_void,
                oldlen as size_t,
            );
            *vp.offset(oldlen as isize) = ' ' as i32 as ::core::ffi::c_char;
            memcpy(
                vp.offset(oldlen.wrapping_add(1) as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                (*f).name as *const ::core::ffi::c_void,
                (newlen as size_t).wrapping_add(1),
            );
            value = vp;
        }
        define_variable_in_set(
            ctx,
            b"MAKECMDGOALS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
            value,
            o_default,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
    }
    0
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
/// `reset_makeflags` for the makefile-time `MAKEFLAGS` reassignment callback
/// (`set_special_var`), which is reached through `do_variable_definition` and
/// cannot thread an `&Options`. Uses the `main_0`-installed borrow.
pub unsafe fn reset_makeflags_special(ctx: &crate::execctx::ExecContext, origin: variable_origin) {
    reset_makeflags(ctx, installed_options(), origin);
}

pub unsafe fn reset_makeflags(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
    origin: variable_origin,
) {
    options.env_overrides.set(false);
    decode_env_switches(
        ctx,
        options,
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        origin,
    );
    {
        let include_dirs = options.include_dirs.borrow();
        let inc_paths: Vec<std::path::PathBuf> = include_dirs
            .iter()
            .map(|s| {
                use std::os::unix::ffi::OsStrExt;
                std::path::PathBuf::from(std::ffi::OsStr::from_bytes(s.as_bytes()))
            })
            .collect();
        construct_include_path(ctx, &inc_paths);
    }
    disable_builtins(ctx, options);
    define_makeflags(ctx, options, opt_rebuilding_makefiles() as i32);
}
unsafe fn decode_switches(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
    argc: i32,
    argv: *mut *const ::core::ffi::c_char,
    origin: variable_origin,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut bad: i32 = 0;
    let mut cs: *mut command_switch;
    let mut targets: stringlist = stringlist {
        list: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        idx: 0,
        max: 0,
    };
    let mut c: i32;
    let mut found_wait: ::core::ffi::c_uint = 0;
    let mut a: *mut *const ::core::ffi::c_char;
    // Re-entrancy guard: `decode_switches` must not be called recursively.
    // Atomic so the read/writes are plain safe ops; switch decoding runs
    // single-threaded, so `Relaxed` preserves the original program order.
    static USING_GETOPT: AtomicBool = AtomicBool::new(false);
    if !USING_GETOPT.load(Ordering::Relaxed) {
    } else {
        panic!("assertion failed: using_getopt == 0");
    };
    USING_GETOPT.store(true, Ordering::Relaxed);
    targets.max = (argc + 1) as ::core::ffi::c_uint;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (targets.max as usize)
            .wrapping_mul(::core::mem::size_of::<*mut *const ::core::ffi::c_char>() as usize)
            as usize,
    ));
    targets.list =
        alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *const ::core::ffi::c_char;
    targets.idx = 0;
    init_switches();
    opterr = (origin as ::core::ffi::c_uint == o_command as i32 as ::core::ffi::c_uint) as i32;
    optind = 0;
    while optind < argc {
        let mut coptarg: *const ::core::ffi::c_char;
        c = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            &raw mut getopt_shorts as *mut ::core::ffi::c_char,
            &raw mut long_options as *mut option,
            ::core::ptr::null_mut::<i32>(),
        );
        coptarg = optarg;
        if c == EOF {
            break;
        }
        if c == '?' as i32 {
            bad = 1;
        } else if c == 1 {
            let fresh8 = targets.idx;
            targets.idx = targets.idx.wrapping_add(1);
            let fresh9 = &mut (*targets.list.offset(fresh8 as isize));
            *fresh9 = coptarg;
        } else {
            cs = &raw mut switches as *mut command_switch;
            while (*cs).c != 0 {
                if (*cs).c == c {
                    let cs_origin = opt_origin_cell(options, (*cs).c);
                    let doit: i32 = (origin as ::core::ffi::c_uint
                        == o_command as i32 as ::core::ffi::c_uint
                        || (*cs).env() as i32 != 0
                            && (cs_origin.is_none()
                                || origin as ::core::ffi::c_uint
                                    >= cs_origin.unwrap().get() as ::core::ffi::c_uint))
                        as i32;
                    if doit != 0 {
                        (*cs).set_specified(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                    match (*cs).type_0 as ::core::ffi::c_uint {
                        7 => {}
                        0 | 1 => {
                            if doit != 0 {
                                let on = (*cs).type_0 as ::core::ffi::c_uint
                                    == flag as i32 as ::core::ffi::c_uint;
                                opt_set_flag(options, (*cs).c, on);
                                if let Some(oc) = cs_origin {
                                    oc.set(origin);
                                }
                            }
                        }
                        2 | 3 | 4 => {
                            if !(doit == 0) {
                                // Resolve the option argument; an empty value is an error
                                // and the option is skipped.
                                let arg_ok =
                                    if coptarg.is_null() {
                                        coptarg = (*cs).noarg_value as *const ::core::ffi::c_char;
                                        true
                                    } else if *coptarg as i32 == 0 {
                                        let mut opt: [::core::ffi::c_char; 2] =
                                            ::core::mem::transmute::<
                                                [u8; 2],
                                                [::core::ffi::c_char; 2],
                                            >(*b"c\0");
                                        let mut op: *const ::core::ffi::c_char =
                                            &raw mut opt as *mut ::core::ffi::c_char;
                                        if (*cs).c <= CHAR_MAX {
                                            opt[0_i32 as usize] = (*cs).c as ::core::ffi::c_char;
                                        } else {
                                            op = (*cs).long_name;
                                        }
                                        error(ctx,
                                        NILF,
                                        strlen(op) as size_t,
                                        b"the '%s%s' option requires a non-empty string argument\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                        if (*cs).c <= CHAR_MAX {
                                            b"-\0" as *const u8 as *const ::core::ffi::c_char
                                        } else {
                                            b"--\0" as *const u8 as *const ::core::ffi::c_char
                                        },
                                        op,
                                    );
                                        bad = 1;
                                        false
                                    } else {
                                        true
                                    };
                                if arg_ok {
                                    if (*cs).type_0 as ::core::ffi::c_uint
                                        == string as i32 as ::core::ffi::c_uint
                                    {
                                        let s = ::core::ffi::CStr::from_ptr(coptarg)
                                            .to_string_lossy()
                                            .into_owned();
                                        opt_set_str(options, (*cs).c, s);
                                        if let Some(oc) = cs_origin {
                                            oc.set(origin);
                                        }
                                    } else if (*cs).c == CHAR_MAX + 1 {
                                        // `--debug` accumulates its args into a
                                        // `Vec<CString>`, skipping an exact duplicate of an
                                        // already-stored value (matching the original
                                        // stringlist dedup logic).
                                        let mut db_flags = options.db_flags.borrow_mut();
                                        let want = ::core::ffi::CStr::from_ptr(coptarg);
                                        let duplicate =
                                            db_flags.iter().any(|e| e.as_c_str() == want);
                                        if !duplicate {
                                            db_flags.push(want.to_owned());
                                            if let Some(oc) = cs_origin {
                                                oc.set(origin);
                                            }
                                        }
                                    } else {
                                        // List options (`strlist`/`filename`) store owned
                                        // `CString`s in a `Vec` on `Options`. Dispatch on the
                                        // switch char to the relevant `Vec`.
                                        let mut list = match (*cs).c {
                                            c if c == 'C' as i32 => {
                                                options.directories.borrow_mut()
                                            }
                                            c if c == 'f' as i32 || c == TEMP_STDIN_OPT => {
                                                options.makefiles.borrow_mut()
                                            }
                                            c if c == 'I' as i32 => {
                                                options.include_dirs.borrow_mut()
                                            }
                                            c if c == 'o' as i32 => options.old_files.borrow_mut(),
                                            c if c == 'W' as i32 => options.new_files.borrow_mut(),
                                            c if c == 'E' as i32 => {
                                                options.eval_strings.borrow_mut()
                                            }
                                            c if c == WARN_OPT => options.warn_flags.borrow_mut(),
                                            _ => {
                                                unreachable!("non-list option in list arm")
                                            }
                                        };
                                        // Skip a value already present (but -f and --warn allow
                                        // duplicates). The comparison is against the raw
                                        // `coptarg` bytes, exactly as the original
                                        // stringlist code did.
                                        let duplicate =
                                            if (*cs).c != 'f' as i32 && (*cs).c != WARN_OPT {
                                                let want = ::core::ffi::CStr::from_ptr(coptarg);
                                                list.iter().any(|e| e.as_c_str() == want)
                                            } else {
                                                false
                                            };
                                        if !duplicate {
                                            // Build the owned `CString` to store.  `strlist`
                                            // stores the raw arg; `filename` stores the
                                            // expanded path (or the strcache entry for the
                                            // --temp-stdin placeholder).
                                            let stored: ::std::ffi::CString = if (*cs).type_0
                                                as ::core::ffi::c_uint
                                                == strlist as i32 as ::core::ffi::c_uint
                                            {
                                                ::core::ffi::CStr::from_ptr(coptarg).to_owned()
                                            } else if (*cs).c == TEMP_STDIN_OPT {
                                                if stdin_offset > 0 {
                                                    fatal(ctx,
                                                                NILF,
                                                                0,
                                                                b"INTERNAL: multiple --temp-stdin options provided!\0"
                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                            );
                                                }
                                                stdin_offset = list.len() as i32;
                                                let cached = strcache_add(coptarg);
                                                temp_stdin_name = cached;
                                                ::core::ffi::CStr::from_ptr(cached).to_owned()
                                            } else {
                                                ::core::ffi::CStr::from_ptr(
                                                    expand_command_line_file(ctx, coptarg),
                                                )
                                                .to_owned()
                                            };
                                            list.push(stored);
                                            if let Some(oc) = cs_origin {
                                                oc.set(origin);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        5 => {
                            if coptarg.is_null() && argc > optind {
                                let mut cp: *const ::core::ffi::c_char;
                                cp = *argv.offset(optind as isize);
                                while (*cp.offset(0_i32 as isize) as ::core::ffi::c_uint)
                                    .wrapping_sub('0' as i32 as ::core::ffi::c_uint)
                                    <= 9
                                {
                                    cp = cp.offset(1_i32 as isize);
                                }
                                if *cp.offset(0_i32 as isize) as i32 == 0 {
                                    let fresh18 = optind;
                                    optind += 1;
                                    coptarg = *argv.offset(fresh18 as isize);
                                }
                            }
                            if !(doit == 0) {
                                if !coptarg.is_null() {
                                    let i = make_toui(::core::ffi::CStr::from_ptr(coptarg))
                                        .unwrap_or(0);
                                    if i == 0 {
                                        error(ctx,
                                            NILF,
                                            0,
                                            b"the '-%c' option requires a positive integer argument\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                            (*cs).c,
                                        );
                                        bad = 1;
                                    } else {
                                        // Only `-j` is a positive_int option; it stores into
                                        // `arg_job_slots` (Some(n) for finite jobs).
                                        options.arg_job_slots.set(Some(i));
                                        if let Some(oc) = cs_origin {
                                            oc.set(origin);
                                        }
                                    }
                                } else {
                                    // No argument: the table's `noarg_value` constant
                                    // (`inf_jobs` == 0) marks infinite jobs => Some(0).
                                    let n = *((*cs).noarg_value as *const ::core::ffi::c_uint);
                                    options.arg_job_slots.set(Some(n));
                                    if let Some(oc) = cs_origin {
                                        oc.set(origin);
                                    }
                                }
                            }
                        }
                        6 => {
                            if coptarg.is_null()
                                && optind < argc
                                && ((*(*argv.offset(optind as isize)).offset(0_i32 as isize)
                                    as ::core::ffi::c_uint)
                                    .wrapping_sub('0' as i32 as ::core::ffi::c_uint)
                                    <= 9
                                    || *(*argv.offset(optind as isize)).offset(0_i32 as isize)
                                        as i32
                                        == '.' as i32)
                            {
                                let fresh19 = optind;
                                optind += 1;
                                coptarg = *argv.offset(fresh19 as isize);
                            }
                            if doit != 0 {
                                // Only `-l` is a floating option; it stores into
                                // `max_load_average`.
                                let v = if !coptarg.is_null() {
                                    atof(coptarg)
                                } else {
                                    *((*cs).noarg_value as *const ::core::ffi::c_double)
                                };
                                options.max_load_average.set(v);
                                if let Some(oc) = cs_origin {
                                    oc.set(origin);
                                }
                            }
                        }
                        _ => {
                            abort();
                        }
                    }
                    break;
                } else {
                    cs = cs.offset(1_i32 as isize);
                }
            }
        }
    }
    while optind < argc {
        let fresh20 = optind;
        optind += 1;
        let fresh21 = targets.idx;
        targets.idx = targets.idx.wrapping_add(1);
        let fresh22 = &mut (*targets.list.offset(fresh21 as isize));
        *fresh22 = *argv.offset(fresh20 as isize);
    }
    let fresh23 = &mut (*targets.list.offset(targets.idx as isize));
    *fresh23 = ::core::ptr::null::<::core::ffi::c_char>();
    USING_GETOPT.store(false, Ordering::Relaxed);
    a = targets.list;
    while !(*a).is_null() {
        let prior_found_wait: i32 = found_wait as i32;
        found_wait = handle_non_switch_argument(ctx, *a, origin);
        if prior_found_wait != 0 && !lastgoal.is_null() {
            (*lastgoal).set_wait_here(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        a = a.offset(1_i32 as isize);
    }
    if bad != 0 && origin as ::core::ffi::c_uint == o_command as i32 as ::core::ffi::c_uint {
        print_usage(ctx, options, bad);
    }
    decode_debug_flags(ctx, options);
    decode_output_sync_flags(ctx, options);
    if options.warn_undefined_variables.get() {
        crate::warning::decode_actions(ctx, "undefined-var", None);
        options.warn_undefined_variables.set(false);
    }
    {
        let warn_flags = options.warn_flags.borrow();
        for wf in warn_flags.iter() {
            let arg = wf.to_str().unwrap_or("");
            crate::warning::decode_actions(ctx, arg, None);
        }
    }
    options.run_silent.set(options.silent.get());
    reset_env_override();
}
unsafe fn decode_env_switches(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
    envar: *const ::core::ffi::c_char,
    mut len: size_t,
    origin: variable_origin,
) {
    let mut value: *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char;
    let buf: *mut ::core::ffi::c_char;
    let mut argc: i32;
    let argv: *mut *const ::core::ffi::c_char;
    value = expand_variable_buf(
        ctx,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        envar,
        len,
    );
    while stopchar_map()[*value as ::core::ffi::c_uchar as usize] as i32 & (0x2_i32 | 0x4_i32) != 0
    {
        value = value.offset(1_i32 as isize);
    }
    len = strlen(value) as size_t;
    if len == 0 {
        return;
    }
    argv = xmalloc(
        (1 as size_t)
            .wrapping_add(len)
            .wrapping_add(1)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    ) as *mut *const ::core::ffi::c_char;
    let fresh0 = &mut (*argv.offset(0_i32 as isize));
    *fresh0 = b"\0" as *const u8 as *const ::core::ffi::c_char;
    argc = 1;
    buf = xmalloc((1 as size_t).wrapping_add(len).wrapping_add(1)) as *mut ::core::ffi::c_char;
    *buf.offset(0_i32 as isize) = '-' as i32 as ::core::ffi::c_char;
    p = buf.offset(1_i32 as isize);
    let fresh1 = &mut (*argv.offset(argc as isize));
    *fresh1 = p;
    while *value as i32 != 0 {
        if *value as i32 == '\\' as i32 && *value.offset(1_i32 as isize) as i32 != 0 {
            value = value.offset(1_i32 as isize);
        } else if stopchar_map()[*value as ::core::ffi::c_uchar as usize] as i32 & 0x2_i32 != 0 {
            let fresh2 = p;
            p = p.offset(1_i32 as isize);
            *fresh2 = 0;
            argc += 1;
            let fresh3 = &mut (*argv.offset(argc as isize));
            *fresh3 = p;
            loop {
                value = value.offset(1_i32 as isize);
                if !(stopchar_map()[*value as ::core::ffi::c_uchar as usize] as i32 & 0x2_i32 != 0)
                {
                    break;
                }
            }
            continue;
        }
        let fresh4 = value;
        value = value.offset(1_i32 as isize);
        let fresh5 = p;
        p = p.offset(1_i32 as isize);
        *fresh5 = *fresh4;
    }
    *p = 0;
    argc += 1;
    let fresh6 = &mut (*argv.offset(argc as isize));
    *fresh6 = ::core::ptr::null::<::core::ffi::c_char>();
    if p < buf.offset(len as isize).offset(2_i32 as isize) {
    } else {
        panic!("assertion failed: p < buf + len + 2");
    };
    if *(*argv.offset(1_i32 as isize)).offset(0_i32 as isize) as i32 != '-' as i32
        && strchr(*argv.offset(1_i32 as isize), '=' as i32).is_null()
    {
        let fresh7 = &mut (*argv.offset(1_i32 as isize));
        *fresh7 = buf;
    }
    decode_switches(ctx, options, argc, argv, origin);
    free(buf as *mut ::core::ffi::c_void);
    free(argv as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn quote_for_env(
    mut out: *mut ::core::ffi::c_char,
    mut in_0: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    while *in_0 as i32 != 0 {
        if *in_0 as i32 == '$' as i32 {
            let fresh29 = out;
            out = out.offset(1_i32 as isize);
            *fresh29 = '$' as i32 as ::core::ffi::c_char;
        } else if stopchar_map()[*in_0 as ::core::ffi::c_uchar as usize] as i32 & 0x2_i32 != 0
            || *in_0 as i32 == '\\' as i32
        {
            let fresh30 = out;
            out = out.offset(1_i32 as isize);
            *fresh30 = '\\' as i32 as ::core::ffi::c_char;
        }
        let fresh31 = in_0;
        in_0 = in_0.offset(1_i32 as isize);
        let fresh32 = out;
        out = out.offset(1_i32 as isize);
        *fresh32 = *fresh31;
    }
    out
}
/// Drop the built-in suffix rules: free `.SUFFIXES`' builtin dependency chain
/// and reset the `SUFFIXES` variable to empty. Split out of `disable_builtins`
/// so that function stays a flat sequence of flag checks.
///
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; the global rule/variable tables must be initialized.
unsafe fn clear_builtin_rules(ctx: &crate::execctx::ExecContext) {
    if !suffix_file.is_null() && (*suffix_file).builtin() as i32 != 0 {
        free_dep_chain((*suffix_file).deps);
        (*suffix_file).deps = ::core::ptr::null_mut::<dep>();
    }
    define_variable_in_set(
        ctx,
        b"SUFFIXES\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn disable_builtins(ctx: &crate::execctx::ExecContext, options: &Options) {
    if options.no_builtin_variables.get() {
        options.no_builtin_rules.set(true);
    }
    if options.no_builtin_rules.get() && !options.prev_no_builtin_rules.get() {
        options.prev_no_builtin_rules.set(true);
        clear_builtin_rules(ctx);
    }
    if options.no_builtin_variables.get() && !options.prev_no_builtin_variables.get() {
        options.prev_no_builtin_variables.set(true);
        undefine_default_variables(ctx);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn define_makeflags(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
    makefile: i32,
) -> *mut variable {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let ref_0: [::core::ffi::c_char; 14] =
        ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"MAKEOVERRIDES\0");
    let posixref: [::core::ffi::c_char; 24] = ::core::mem::transmute::<
        [u8; 24],
        [::core::ffi::c_char; 24],
    >(*b"-*-command-variables-*-\0");
    let evalref: [::core::ffi::c_char; 21] =
        ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b" $(-*-eval-flags-*-)\0");
    let mut cs: *const command_switch;
    let mut v: *mut variable;
    let mut bufsave: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut lensave: size_t = 0;
    let mut fp: *mut ::core::ffi::c_char;
    let mut c: [::core::ffi::c_char; 3] = [0; 3];
    install_variable_buffer(&raw mut bufsave, &raw mut lensave);
    fp = variable_buffer_output(
        variable_buffer,
        b"-\0" as *const u8 as *const ::core::ffi::c_char,
        1,
    );
    cs = &raw mut switches as *mut command_switch;
    while (*cs).c != 0 {
        if (*cs).toenv() as i32 != 0
            && (*cs).c <= CHAR_MAX
            && (makefile == 0 || (*cs).no_makefile() == 0)
            && ((*cs).type_0 as ::core::ffi::c_uint == flag as i32 as ::core::ffi::c_uint
                || (*cs).type_0 as ::core::ffi::c_uint == flag_off as i32 as ::core::ffi::c_uint)
            && ((opt_flag_int(options, (*cs).c) == 0) as i32
                == ((*cs).type_0 as ::core::ffi::c_uint == flag_off as i32 as ::core::ffi::c_uint)
                    as i32
                && ((*cs).default_value.is_null()
                    || (*cs).specified() as i32 != 0
                    || opt_flag_int(options, (*cs).c) != *((*cs).default_value as *mut i32)))
        {
            c[0_i32 as usize] = (*cs).c as ::core::ffi::c_char;
            fp = variable_buffer_output(fp, &raw mut c as *mut ::core::ffi::c_char, 1);
        }
        cs = cs.offset(1_i32 as isize);
    }
    memcpy(
        &raw mut c as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        b" --\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        3,
    );
    cs = &raw mut switches as *mut command_switch;
    while (*cs).c != 0 {
        if (*cs).toenv() as i32 != 0 && (makefile == 0 || (*cs).no_makefile() == 0) {
            match (*cs).type_0 as ::core::ffi::c_uint {
                7 => {}
                0 | 1 => {
                    if !((*cs).c <= CHAR_MAX)
                        && ((opt_flag_int(options, (*cs).c) == 0) as i32
                            == ((*cs).type_0 as ::core::ffi::c_uint
                                == flag_off as i32 as ::core::ffi::c_uint)
                                as i32
                            && ((*cs).default_value.is_null()
                                || (*cs).specified() as i32 != 0
                                || opt_flag_int(options, (*cs).c)
                                    != *((*cs).default_value as *mut i32)))
                    {
                        if (*cs).c <= CHAR_MAX {
                            c[2_i32 as usize] = (*cs).c as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                        } else {
                            c[2_i32 as usize] = '-' as i32 as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                            fp = variable_buffer_output(
                                fp,
                                (*cs).long_name,
                                strlen((*cs).long_name) as size_t,
                            );
                        }
                    }
                }
                5 => {
                    if !(!(*cs).default_value.is_null()
                        && opt_uint(options, (*cs).c)
                            == *((*cs).default_value as *mut ::core::ffi::c_uint))
                    {
                        if (*cs).c <= CHAR_MAX {
                            c[2_i32 as usize] = (*cs).c as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                        } else {
                            c[2_i32 as usize] = '-' as i32 as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                            fp = variable_buffer_output(
                                fp,
                                (*cs).long_name,
                                strlen((*cs).long_name) as size_t,
                            );
                        }
                        if (*cs).noarg_value.is_null()
                            || opt_uint(options, (*cs).c)
                                != *((*cs).noarg_value as *mut ::core::ffi::c_uint)
                        {
                            alloca_allocations.push(::std::vec::from_elem(
                                0,
                                30_i32 as ::core::ffi::c_ulong as usize,
                            ));
                            let buf: *mut ::core::ffi::c_char =
                                alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                    as *mut ::core::ffi::c_char;
                            let buflen: i32 = sprintf(
                                buf,
                                b"%u\0" as *const u8 as *const ::core::ffi::c_char,
                                opt_uint(options, (*cs).c),
                            );
                            if !((*cs).c <= CHAR_MAX) {
                                fp = variable_buffer_output(
                                    fp,
                                    b"=\0" as *const u8 as *const ::core::ffi::c_char,
                                    1,
                                );
                            }
                            fp = variable_buffer_output(fp, buf, buflen as size_t);
                        }
                    }
                }
                6 => {
                    if !(!(*cs).default_value.is_null()
                        && opt_double(options, (*cs).c)
                            == *((*cs).default_value as *mut ::core::ffi::c_double))
                    {
                        if (*cs).c <= CHAR_MAX {
                            c[2_i32 as usize] = (*cs).c as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                        } else {
                            c[2_i32 as usize] = '-' as i32 as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                            fp = variable_buffer_output(
                                fp,
                                (*cs).long_name,
                                strlen((*cs).long_name) as size_t,
                            );
                        }
                        if (*cs).noarg_value.is_null()
                            || opt_double(options, (*cs).c)
                                != *((*cs).noarg_value as *mut ::core::ffi::c_double)
                        {
                            alloca_allocations.push(::std::vec::from_elem(
                                0,
                                100_i32 as ::core::ffi::c_ulong as usize,
                            ));
                            let buf_0: *mut ::core::ffi::c_char =
                                alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                    as *mut ::core::ffi::c_char;
                            let buflen_0: i32 = sprintf(
                                buf_0,
                                b"%g\0" as *const u8 as *const ::core::ffi::c_char,
                                opt_double(options, (*cs).c),
                            );
                            if !((*cs).c <= CHAR_MAX) {
                                fp = variable_buffer_output(
                                    fp,
                                    b"=\0" as *const u8 as *const ::core::ffi::c_char,
                                    1,
                                );
                            }
                            fp = variable_buffer_output(fp, buf_0, buflen_0 as size_t);
                        }
                    }
                }
                2 => {
                    // Keep the owned `CString` alive across the buffer writes.
                    let owned = opt_get_str(options, (*cs).c);
                    let p: *const ::core::ffi::c_char = match owned {
                        Some(ref s) => s.as_ptr(),
                        None => ::core::ptr::null(),
                    };
                    if !p.is_null() {
                        if (*cs).c <= CHAR_MAX {
                            c[2_i32 as usize] = (*cs).c as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                        } else {
                            c[2_i32 as usize] = '-' as i32 as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                            fp = variable_buffer_output(
                                fp,
                                (*cs).long_name,
                                strlen((*cs).long_name) as size_t,
                            );
                        }
                        if !((*cs).c <= CHAR_MAX) {
                            fp = variable_buffer_output(
                                fp,
                                b"=\0" as *const u8 as *const ::core::ffi::c_char,
                                1,
                            );
                        }
                        fp = variable_buffer_output(fp, p, strlen(p) as size_t);
                    }
                }
                4 | 3 => {
                    if (*cs).c == WARN_OPT {
                        fp = crate::warning::encode_flag(fp);
                    } else {
                        // Gather the item pointers to serialize. The migrated list
                        // options read their owned `Vec<CString>`; `--debug`
                        // (`db_flags`) still reads its C `stringlist`.
                        let mut items: Vec<*const ::core::ffi::c_char> = Vec::new();
                        // Borrow the relevant `Vec<CString>` on `options`; `--debug`
                        // reads `db_flags`, every other list option its own field.
                        // The borrow guard must outlive `items` (raw pointers into it).
                        let guard = match (*cs).c {
                            c if c == 'C' as i32 => options.directories.borrow(),
                            c if c == 'f' as i32 || c == TEMP_STDIN_OPT => {
                                options.makefiles.borrow()
                            }
                            c if c == 'I' as i32 => options.include_dirs.borrow(),
                            c if c == 'o' as i32 => options.old_files.borrow(),
                            c if c == 'W' as i32 => options.new_files.borrow(),
                            c if c == 'E' as i32 => options.eval_strings.borrow(),
                            c if c == CHAR_MAX + 1 => options.db_flags.borrow(),
                            _ => options.db_flags.borrow(),
                        };
                        items.extend(guard.iter().map(|s| s.as_ptr()));
                        {
                            for &item in items.iter() {
                                if (*cs).c <= CHAR_MAX {
                                    c[2_i32 as usize] = (*cs).c as ::core::ffi::c_char;
                                    fp = variable_buffer_output(
                                        fp,
                                        &raw mut c as *mut ::core::ffi::c_char,
                                        3,
                                    );
                                } else {
                                    c[2_i32 as usize] = '-' as i32 as ::core::ffi::c_char;
                                    fp = variable_buffer_output(
                                        fp,
                                        &raw mut c as *mut ::core::ffi::c_char,
                                        3,
                                    );
                                    fp = variable_buffer_output(
                                        fp,
                                        (*cs).long_name,
                                        strlen((*cs).long_name) as size_t,
                                    );
                                }
                                if !((*cs).c <= CHAR_MAX) {
                                    fp = variable_buffer_output(
                                        fp,
                                        b"=\0" as *const u8 as *const ::core::ffi::c_char,
                                        1,
                                    );
                                }
                                fp = variable_buffer_output(fp, item, strlen(item) as size_t);
                            }
                        }
                    }
                }
                _ => {
                    abort();
                }
            }
        }
        cs = cs.offset(1_i32 as isize);
    }
    if fp == variable_buffer.offset(1_i32 as isize) {
        fp = variable_buffer;
    }
    *fp = 0;
    define_variable_in_set(
        ctx,
        b"MFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1),
        variable_buffer.offset(
            (if *variable_buffer.offset(0_i32 as isize) as i32 == '-' as i32
                && *variable_buffer.offset(1_i32 as isize) as i32 == ' ' as i32
            {
                2
            } else {
                0
            }) as isize,
        ),
        o_env,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    if !options.eval_strings.borrow().is_empty() {
        fp = variable_buffer_output(
            fp,
            &raw const evalref as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 21]>() as size_t).wrapping_sub(1),
        );
    }
    let r: *const ::core::ffi::c_char = if posix_pedantic() {
        &raw const posixref as *const ::core::ffi::c_char
    } else {
        &raw const ref_0 as *const ::core::ffi::c_char
    };
    let l: size_t = strlen(r) as size_t;
    v = lookup_variable(ctx, r, l);
    if v.as_ref()
        .is_some_and(|vr| !vr.value.is_null() && *vr.value.offset(0) as i32 != 0)
    {
        fp = variable_buffer_output(
            fp,
            b" -- $(\0" as *const u8 as *const ::core::ffi::c_char,
            6,
        );
        fp = variable_buffer_output(fp, r, l);
        fp = variable_buffer_output(fp, b")\0" as *const u8 as *const ::core::ffi::c_char, 1);
    }
    *fp = 0;
    fp = variable_buffer;
    if *fp.offset(0_i32 as isize) as i32 == '-' as i32 {
        fp = fp.offset(1_i32 as isize);
    }
    v = define_variable_in_set(
        ctx,
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        fp,
        (if options.env_overrides.get() {
            o_env_override as i32
        } else {
            o_file as i32
        }) as variable_origin,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    (*v).set_special(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    restore_variable_buffer(bufsave, lensave);
    v
}
/// Decide whether `make` should announce directory changes.
///
/// Returns the explicit `--print-directory` / `--no-print-directory`
/// choice when one was given; otherwise it prints unless `--silent` is set
/// and both the make level is top-level and no `-C` directory was supplied.
///
/// The make level is read from the threaded [`ExecContext`] rather than a
/// process global.
pub fn should_print_dir(ctx: &crate::execctx::ExecContext, options: &Options) -> bool {
    if let Some(v) = options.print_directory.get() {
        return v;
    }
    let nested = ctx.makelevel() > 0;
    !options.silent.get() && (nested || !options.directories.borrow().is_empty())
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_version() {
    static PRINTED_VERSION: AtomicBool = AtomicBool::new(false);
    let precede: *const ::core::ffi::c_char = if opt_print_data_base() {
        b"# \0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"\0" as *const u8 as *const ::core::ffi::c_char
    };
    if PRINTED_VERSION.swap(true, Ordering::Relaxed) {
        return;
    }
    printf(
        b"%sGNU Make %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        precede,
        crate::version::version_string(),
    );
    if remote_description.is_null() || *remote_description as i32 == 0 {
        printf(
            b"%sBuilt for %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            precede,
            crate::version::make_host(),
        );
    } else {
        printf(
            b"%sBuilt for %s (%s)\n\0" as *const u8 as *const ::core::ffi::c_char,
            precede,
            crate::version::make_host(),
            remote_description,
        );
    }
    printf(
        b"%sCopyright (C) 1988-2025 Free Software Foundation, Inc.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        precede,
    );
    printf(
        b"%sLicense GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>\n%sThis is free software: you are free to change and redistribute it.\n%sThere is NO WARRANTY, to the extent permitted by law.\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        precede,
        precede,
        precede,
    );
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_data_base(ctx: &crate::execctx::ExecContext) {
    let stamp = ::std::ffi::CString::new(file_timestamp_string(file_timestamp_now(ctx).0))
        .expect("formatted timestamp never contains an interior NUL");
    print_version();
    printf(
        b"\n# Make data base, printed on %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        stamp.as_ptr(),
    );
    print_variable_data_base();
    print_dir_data_base();
    print_rule_data_base(ctx);
    print_file_data_base();
    print_vpath_data_base();
    strcache_print_stats(b"#\0" as *const u8 as *const ::core::ffi::c_char);
    let stamp = ::std::ffi::CString::new(file_timestamp_string(file_timestamp_now(ctx).0))
        .expect("formatted timestamp never contains an interior NUL");
    printf(
        b"\n# Finished Make data base on %s\n\n\0" as *const u8 as *const ::core::ffi::c_char,
        stamp.as_ptr(),
    );
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn clean_jobserver(ctx: &crate::execctx::ExecContext, status: i32) {
    if jobserver_enabled() != 0 && jobserver_tokens() != 0 {
        if status != 2 {
            error(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                INTSTR_LENGTH,
                b"INTERNAL: exiting with %u jobserver tokens (should be 0)!\0" as *const u8
                    as *const ::core::ffi::c_char,
                jobserver_tokens(),
            );
        } else {
            loop {
                JOBSERVER_TOKENS.fetch_sub(1, Ordering::Relaxed);
                if jobserver_tokens() == 0 {
                    break;
                }
                jobserver_release(ctx, 0);
            }
        }
    }
    let master_slots = master_job_slots();
    if master_slots != 0 {
        let tokens: ::core::ffi::c_uint =
            (1 as ::core::ffi::c_uint).wrapping_add(jobserver_acquire_all(ctx));
        if tokens != master_slots {
            error(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                INTSTR_LENGTH.wrapping_mul(2),
                b"INTERNAL: exiting with %u jobserver tokens available; should be %u!\0"
                    as *const u8 as *const ::core::ffi::c_char,
                tokens,
                master_slots,
            );
        }
    }
    reset_jobserver_mirror();
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn die(ctx: &crate::execctx::ExecContext, status: i32) -> ! {
    static DYING: AtomicBool = AtomicBool::new(false);
    if !DYING.swap(true, Ordering::Relaxed) {
        let err: i32;
        if opt_print_version() {
            print_version();
        }
        temp_stdin_unlink(ctx);
        err = (status != 0) as i32;
        while job_slots_used() > 0 {
            reap_children(ctx, 1, err);
        }
        remote_cleanup();
        remove_intermediates(ctx, 0);
        if opt_print_data_base() {
            print_data_base(ctx);
        }
        if with_options(|o| o.verify.get()) {
            verify_file_data_base(ctx);
        }
        unload_all();
        clean_jobserver(ctx, status);
        if !output_context.is_null() {
            crate::output::output_close(ctx, output_context);
            if output_context != &raw mut make_sync {
                crate::output::output_close(ctx, &raw mut make_sync);
            }
            output_context = ::core::ptr::null_mut::<output>();
        }
        crate::output::output_close(ctx, ::core::ptr::null_mut::<output>());
        osync_clear();
        if !directory_before_chdir.is_null() {
            let mut _x: i32 = 0;
            _x = chdir(directory_before_chdir);
        }
    }
    exit(status);
}
pub const __CHAR_BIT__: i32 = 8;
pub const __SCHAR_MAX__: i32 = 127;
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    let mut vars: Vec<*mut ::core::ffi::c_char> = Vec::new();
    for (var_name, var_value) in ::std::env::vars() {
        let var: String = format!("{}={}", var_name, var_value);
        vars.push(
            ::std::ffi::CString::new(var)
                .expect("Failed to convert environment variable into CString.")
                .into_raw(),
        );
    }
    vars.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as i32,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
            vars.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
unsafe extern "C" fn run_static_initializers() {
    switches = [
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'b' as i32,
                type_0: ignore,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: ::core::ptr::null::<::core::ffi::c_char>(),
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(0);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'B' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"always-make\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'd' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: ::core::ptr::null::<::core::ffi::c_char>(),
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'e' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"environment-overrides\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'E' as i32,
                type_0: strlist,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"eval\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'h' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"help\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(0);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'i' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"ignore-errors\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'k' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: &raw const default_keep_going_flag as *const ::core::ffi::c_void,
                long_name: b"keep-going\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'L' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"check-symlink-times\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'm' as i32,
                type_0: ignore,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: ::core::ptr::null::<::core::ffi::c_char>(),
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(0);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'n' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"just-print\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(1);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'p' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"print-data-base\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'q' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"question\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(1);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'r' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"no-builtin-rules\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'R' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"no-builtin-variables\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 's' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: &raw const default_silent_flag as *const ::core::ffi::c_void,
                long_name: b"silent\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'S' as i32,
                type_0: flag_off,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: &raw const default_keep_going_flag as *const ::core::ffi::c_void,
                long_name: b"no-keep-going\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 't' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"touch\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(1);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'v' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"version\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'w' as i32,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: &raw const default_print_directory_flag
                    as *const ::core::ffi::c_void,
                long_name: b"print-directory\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'C' as i32,
                type_0: filename,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"directory\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(0);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'f' as i32,
                type_0: filename,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"file\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(0);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'I' as i32,
                type_0: filename,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"include-dir\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'j' as i32,
                type_0: positive_int,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: &raw const inf_jobs as *const ::core::ffi::c_void,
                default_value: &raw const default_job_slots as *const ::core::ffi::c_void,
                long_name: b"jobs\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'l' as i32,
                type_0: floating,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: &raw const default_load_average as *const ::core::ffi::c_void,
                default_value: &raw const default_load_average as *const ::core::ffi::c_void,
                long_name: b"load-average\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'o' as i32,
                type_0: filename,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"old-file\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(0);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'O' as i32,
                type_0: string,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: b"target\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"output-sync\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'W' as i32,
                type_0: filename,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"what-if\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(0);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: CHAR_MAX + 1,
                type_0: strlist,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: b"basic\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: CHAR_MAX + 2,
                type_0: string,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: JOBSERVER_AUTH_OPT.as_ptr(),
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: CHAR_MAX + 3,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"trace\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: CHAR_MAX + 4,
                type_0: flag_off,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: &raw const default_print_directory_flag
                    as *const ::core::ffi::c_void,
                long_name: b"no-print-directory\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: CHAR_MAX + 5,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"warn-undefined-variables\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: CHAR_MAX + 7,
                type_0: string,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"sync-mutex\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: CHAR_MAX + 8,
                type_0: flag_off,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: &raw const default_silent_flag as *const ::core::ffi::c_void,
                long_name: b"no-silent\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: CHAR_MAX + 9,
                type_0: string,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"jobserver-fds\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: TEMP_STDIN_OPT,
                type_0: filename,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"temp-stdin\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(0);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: CHAR_MAX + 11,
                type_0: string,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: b"random\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"shuffle\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: CHAR_MAX + 12,
                type_0: string,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"jobserver-style\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: WARN_OPT,
                type_0: strlist,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: b"warn\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"warn\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: CHAR_MAX + 14,
                type_0: flag,
                value_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: b"print-targets\0" as *const u8 as *const ::core::ffi::c_char,
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(1);
            init.set_toenv(1);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 0,
                type_0: flag,
                value_ptr: NULL,
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: ::core::ptr::null::<::core::ffi::c_void>(),
                long_name: ::core::ptr::null::<::core::ffi::c_char>(),
                origin: ::core::ptr::null_mut::<variable_origin>(),
            };
            init.set_env(0);
            init.set_toenv(0);
            init.set_no_makefile(0);
            init.set_specified(0);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

#[cfg(test)]
mod default_load_average_tests {
    /// `default_load_average` is the read-only "no load limit" sentinel
    /// (`-1.0`) the option table hands the `-l`/`--load-average` parser as its
    /// `default_value` and no-argument `noarg_value`. It is now an immutable
    /// `static` (was a `static mut`), so this is a plain safe read.
    #[test]
    fn default_load_average_is_no_limit_sentinel() {
        assert_eq!(super::default_load_average, -1.0f64);
    }
}

#[cfg(test)]
mod job_slots_tests {
    use super::{install_default_options_for_test, opt_job_slots, with_options, Options};

    /// `Options::job_slots` (the resolved `-j` width, the former `job_slots`
    /// global) defaults to 0 ("driven by an inherited jobserver / unlimited")
    /// and round-trips.
    #[test]
    fn job_slots_defaults_to_zero_and_is_settable() {
        let options = Options::new();
        assert_eq!(options.job_slots.get(), 0);

        options.job_slots.set(4);
        assert_eq!(options.job_slots.get(), 4);
    }

    /// `opt_job_slots()` reflects the installed `Options::job_slots` through the
    /// `OPTIONS_PTR` borrow channel the job scheduler reads.
    #[test]
    fn opt_job_slots_reads_through_channel() {
        install_default_options_for_test();

        with_options(|o| o.job_slots.set(0));
        assert_eq!(opt_job_slots(), 0);

        with_options(|o| o.job_slots.set(8));
        assert_eq!(opt_job_slots(), 8);

        with_options(|o| o.job_slots.set(0));
    }
}

#[cfg(test)]
mod output_sync_tests {
    use super::{
        classify_output_sync, install_default_options_for_test, opt_output_sync, with_options,
        Options, OUTPUT_SYNC_LINE, OUTPUT_SYNC_NONE, OUTPUT_SYNC_RECURSE, OUTPUT_SYNC_TARGET,
    };

    #[test]
    fn known_modes() {
        assert_eq!(classify_output_sync(b"none"), Some(OUTPUT_SYNC_NONE));
        assert_eq!(classify_output_sync(b"line"), Some(OUTPUT_SYNC_LINE));
        assert_eq!(classify_output_sync(b"target"), Some(OUTPUT_SYNC_TARGET));
        assert_eq!(classify_output_sync(b"recurse"), Some(OUTPUT_SYNC_RECURSE));
    }

    #[test]
    fn unknown_modes() {
        assert_eq!(classify_output_sync(b""), None);
        assert_eq!(classify_output_sync(b"nonsense"), None);
        assert_eq!(classify_output_sync(b"NONE"), None); // case-sensitive, like make
        assert_eq!(classify_output_sync(b"none "), None); // exact match only
    }

    /// `Options::output_sync` (the resolved mode, the former `output_sync`
    /// global) defaults to `OUTPUT_SYNC_NONE` and holds whatever
    /// `decode_output_sync_flags` resolves into it.
    #[test]
    fn resolved_mode_defaults_to_none_and_is_settable() {
        let options = Options::new();
        assert_eq!(options.output_sync.get(), OUTPUT_SYNC_NONE);

        options.output_sync.set(OUTPUT_SYNC_TARGET);
        assert_eq!(options.output_sync.get(), OUTPUT_SYNC_TARGET);
    }

    /// `opt_output_sync()` reflects the installed `Options::output_sync` through
    /// the `OPTIONS_PTR` borrow channel the `syncing` / dump readers use.
    #[test]
    fn opt_output_sync_reads_through_channel() {
        install_default_options_for_test();

        with_options(|o| o.output_sync.set(OUTPUT_SYNC_NONE));
        assert_eq!(opt_output_sync(), OUTPUT_SYNC_NONE);

        with_options(|o| o.output_sync.set(OUTPUT_SYNC_LINE));
        assert_eq!(opt_output_sync(), OUTPUT_SYNC_LINE);

        with_options(|o| o.output_sync.set(OUTPUT_SYNC_NONE));
    }
}

#[cfg(test)]
mod special_target_latches_tests {
    use super::{
        install_default_options_for_test, not_parallel, one_shell, posix_pedantic,
        second_expansion, set_not_parallel, set_one_shell, set_posix_pedantic,
        set_second_expansion, with_options, Options,
    };

    /// The four special-target feature latches default to false on a fresh
    /// `Options` (the former `POSIX_PEDANTIC` / `SECOND_EXPANSION` / `ONE_SHELL`
    /// / `NOT_PARALLEL` globals).
    #[test]
    fn latches_default_to_false() {
        let options = Options::new();
        assert!(!options.posix_pedantic.get());
        assert!(!options.second_expansion.get());
        assert!(!options.one_shell.get());
        assert!(!options.not_parallel.get());
    }

    /// Each `set_*` latches its flag true and each reader observes it through the
    /// `OPTIONS_PTR` channel — the same channel `check_specials` / `snap_deps`
    /// write and the job/expand/rule/... readers read, including on the
    /// `gmk_eval` throwaway-context path. `OPTIONS_PTR` is thread-local, so this
    /// stays isolated under the parallel test harness.
    #[test]
    fn set_and_read_each_latch_through_channel() {
        install_default_options_for_test();
        with_options(|o| {
            o.posix_pedantic.set(false);
            o.second_expansion.set(false);
            o.one_shell.set(false);
            o.not_parallel.set(false);
        });
        assert!(!posix_pedantic() && !second_expansion() && !one_shell() && !not_parallel());

        set_posix_pedantic();
        set_second_expansion();
        set_one_shell();
        set_not_parallel();
        assert!(posix_pedantic(), "enabled by .POSIX");
        assert!(second_expansion(), "enabled by .SECONDEXPANSION");
        assert!(one_shell(), "enabled by .ONESHELL");
        assert!(not_parallel(), "enabled by .NOTPARALLEL");

        with_options(|o| {
            o.posix_pedantic.set(false);
            o.second_expansion.set(false);
            o.one_shell.set(false);
            o.not_parallel.set(false);
        });
    }
}

#[cfg(test)]
mod verify_flag_tests {
    use super::Options;

    /// `Options::verify` carries the former `verify_flag` global: false by
    /// default, true once set. `enter_file`/`die` read it through the
    /// `with_options` borrow channel during a run.
    #[test]
    fn verify_defaults_false_and_latches() {
        let options = Options::new();
        assert!(!options.verify.get(), "default is unset");

        options.verify.set(true);
        assert!(options.verify.get(), "set once enabled");
    }
}

#[cfg(test)]
mod run_silent_tests {
    use super::{install_default_options_for_test, opt_run_silent, with_options, Options};

    /// `Options::run_silent` carries the former `run_silent` global: false by
    /// default, true once `-s`/`.SILENT` sets it. Distinct storage from
    /// `silent` (the MAKEFLAGS-visible switch), so setting one never disturbs
    /// the other.
    #[test]
    fn run_silent_defaults_false_and_latches() {
        let options = Options::new();
        assert!(!options.run_silent.get(), "default is unset");
        assert!(!options.silent.get(), "silent default is unset");

        options.run_silent.set(true);
        assert!(options.run_silent.get(), "set once enabled");
        assert!(
            !options.silent.get(),
            "run_silent is independent of the MAKEFLAGS-visible silent switch"
        );
    }

    /// `opt_run_silent()` reflects the installed `Options::run_silent` through
    /// the `OPTIONS_PTR` borrow channel the recipe-echo readers use.
    #[test]
    fn opt_run_silent_reads_through_channel() {
        install_default_options_for_test();

        with_options(|o| o.run_silent.set(false));
        assert!(!opt_run_silent(), "channel reads the cleared flag");

        with_options(|o| o.run_silent.set(true));
        assert!(opt_run_silent(), "channel reads the set flag");

        with_options(|o| o.run_silent.set(false));
    }
}

#[cfg(test)]
mod export_all_variables_tests {
    use super::{install_default_options_for_test, opt_export_all_variables, with_options, Options};

    /// `Options::export_all_variables` carries the former
    /// `export_all_variables` global: false by default, toggled by
    /// `export`/`unexport`/`.EXPORT_ALL_VARIABLES`.
    #[test]
    fn export_all_variables_defaults_false_and_toggles() {
        let options = Options::new();
        assert!(!options.export_all_variables.get(), "default is unset");

        options.export_all_variables.set(true);
        assert!(options.export_all_variables.get(), "set by export-all");
        options.export_all_variables.set(false);
        assert!(!options.export_all_variables.get(), "cleared by unexport");
    }

    /// `opt_export_all_variables()` reflects the installed
    /// `Options::export_all_variables` through the `OPTIONS_PTR` borrow channel
    /// that `should_export` reads.
    #[test]
    fn opt_export_all_variables_reads_through_channel() {
        install_default_options_for_test();

        with_options(|o| o.export_all_variables.set(false));
        assert!(!opt_export_all_variables(), "channel reads the cleared flag");

        with_options(|o| o.export_all_variables.set(true));
        assert!(opt_export_all_variables(), "channel reads the set flag");

        with_options(|o| o.export_all_variables.set(false));
    }
}

#[cfg(test)]
mod cmd_prefix_tests {
    use super::{install_default_options_for_test, opt_cmd_prefix, with_options, Options};

    /// `Options::cmd_prefix` defaults to a tab (the recipe prefix), *not* the
    /// `Cell`/`Default` `\0`, and is changed by `.RECIPEPREFIX`.
    #[test]
    fn cmd_prefix_defaults_to_tab_and_is_settable() {
        let options = Options::new();
        assert_eq!(options.cmd_prefix.get(), b'\t' as ::core::ffi::c_char);

        options.cmd_prefix.set(b'>' as ::core::ffi::c_char);
        assert_eq!(options.cmd_prefix.get(), b'>' as ::core::ffi::c_char);
    }

    /// `opt_cmd_prefix()` reflects the installed `Options::cmd_prefix` through
    /// the `OPTIONS_PTR` borrow channel that the makefile reader uses.
    #[test]
    fn opt_cmd_prefix_reads_through_channel() {
        install_default_options_for_test();

        with_options(|o| o.cmd_prefix.set(b'\t' as ::core::ffi::c_char));
        assert_eq!(opt_cmd_prefix(), b'\t' as ::core::ffi::c_char);

        with_options(|o| o.cmd_prefix.set(b'>' as ::core::ffi::c_char));
        assert_eq!(opt_cmd_prefix(), b'>' as ::core::ffi::c_char);

        with_options(|o| o.cmd_prefix.set(b'\t' as ::core::ffi::c_char));
    }
}

#[cfg(test)]
mod stdio_traced_tests {
    use super::{install_default_options_for_test, with_options, Options};
    use crate::output::{set_stdio_traced, stdio_traced};

    /// The trace latch defaults to false on a fresh `Options` (the former
    /// `STDIO_TRACED` global).
    #[test]
    fn stdio_traced_defaults_to_false() {
        assert!(!Options::new().stdio_traced.get());
    }

    /// `set_stdio_traced` writes and `stdio_traced()` reads the one-shot latch
    /// through the `OPTIONS_PTR` channel — the same channel `output_start` uses
    /// when it logs the working-directory enter trace, including on the
    /// `gmk_eval` throwaway-context path. `OPTIONS_PTR` is thread-local, so this
    /// stays isolated under the parallel test harness.
    #[test]
    fn set_and_read_stdio_traced_through_channel() {
        install_default_options_for_test();
        with_options(|o| o.stdio_traced.set(false));
        assert!(!stdio_traced(), "not yet traced");

        set_stdio_traced(true);
        assert!(stdio_traced(), "trace emitted through the channel");

        set_stdio_traced(false);
        assert!(!stdio_traced(), "false through the channel");
    }
}

#[cfg(test)]
mod default_job_slots_tests {
    use super::{default_job_slots, INVALID_JOB_SLOTS};

    /// `default_job_slots` is an immutable `static` holding the option's
    /// read-only default and is accessible from safe code.
    #[test]
    fn default_job_slots_is_invalid_sentinel() {
        assert_eq!(default_job_slots, INVALID_JOB_SLOTS);
    }
}

#[cfg(test)]
mod master_job_slots_tests {
    use super::{install_default_options_for_test, master_job_slots, with_options, Options};

    /// The master jobserver slot count defaults to 0 on a fresh `Options` (the
    /// former `MASTER_JOB_SLOTS` global).
    #[test]
    fn master_job_slots_defaults_to_zero() {
        assert_eq!(Options::new().master_job_slots.get(), 0);
    }

    /// `master_job_slots()` reads the count through the `OPTIONS_PTR` channel —
    /// the same channel `main_0` writes at jobserver setup. `OPTIONS_PTR` is
    /// thread-local, so this stays isolated under the parallel test harness.
    #[test]
    fn master_job_slots_reads_through_channel() {
        install_default_options_for_test();
        with_options(|o| o.master_job_slots.set(0));
        assert_eq!(master_job_slots(), 0, "channel reads the installed value");

        with_options(|o| o.master_job_slots.set(4));
        assert_eq!(master_job_slots(), 4, "count through the channel");

        with_options(|o| o.master_job_slots.set(0));
    }
}

#[cfg(test)]
mod command_count_tests {
    use super::{
        bump_command_count, install_default_options_for_test, opt_command_count, with_options,
        Options,
    };

    /// `Options::command_count` carries the former `command_count` global: it
    /// starts at 1 and is bumped once per shell command run, which is what the
    /// directory cache compares against to invalidate entries recorded before
    /// the latest command.
    #[test]
    fn command_count_defaults_to_one_and_bumps() {
        let options = Options::new();
        assert_eq!(options.command_count.get(), 1, "default is 1");
        options
            .command_count
            .set(options.command_count.get().wrapping_add(1));
        assert_eq!(options.command_count.get(), 2, "one bump");
    }

    /// `opt_command_count()` reads, and `bump_command_count()` increments, the
    /// installed `Options::command_count` through the `OPTIONS_PTR` borrow
    /// channel the dir-cache / job / function paths use.
    #[test]
    fn bump_and_read_through_channel() {
        install_default_options_for_test();

        with_options(|o| o.command_count.set(1));
        assert_eq!(opt_command_count(), 1, "channel reads the installed value");

        bump_command_count();
        bump_command_count();
        assert_eq!(opt_command_count(), 3, "two bumps through the channel");

        with_options(|o| o.command_count.set(1));
    }
}

#[cfg(test)]
mod snapped_deps_tests {
    use super::{
        install_default_options_for_test, mark_snapped_deps, opt_snapped_deps, with_options,
        Options,
    };

    /// `Options::snapped_deps` carries the former `file::SNAPPED_DEPS` global: it
    /// starts false and latches true when `snap_deps` completes, which is what
    /// `record_files` checks to reject prerequisites defined inside a recipe.
    #[test]
    fn snapped_deps_defaults_to_false_and_latches() {
        let options = Options::new();
        assert!(!options.snapped_deps.get(), "default is false");
        options.snapped_deps.set(true);
        assert!(options.snapped_deps.get(), "latches true");
    }

    /// `opt_snapped_deps()` reads, and `mark_snapped_deps()` latches, the
    /// installed `Options::snapped_deps` through the `OPTIONS_PTR` borrow channel
    /// `record_files` uses — including on the `gmk_eval` throwaway-context path.
    #[test]
    fn mark_and_read_snapped_deps_through_channel() {
        install_default_options_for_test();

        with_options(|o| o.snapped_deps.set(false));
        assert!(!opt_snapped_deps(), "channel reads the installed value");

        mark_snapped_deps();
        assert!(opt_snapped_deps(), "marked through the channel");

        with_options(|o| o.snapped_deps.set(false));
    }
}

#[cfg(test)]
mod rebuilding_makefiles_tests {
    use super::{install_default_options_for_test, opt_rebuilding_makefiles, with_options, Options};

    /// `Options::rebuilding_makefiles` carries the former `REBUILDING_MAKEFILES`
    /// global: false outside the makefile-remake pass, true while `main_0`
    /// remakes the makefiles themselves.
    #[test]
    fn rebuilding_makefiles_defaults_to_false() {
        let options = Options::new();
        assert!(!options.rebuilding_makefiles.get(), "default is false");
        options.rebuilding_makefiles.set(true);
        assert!(options.rebuilding_makefiles.get(), "set true");
    }

    /// `opt_rebuilding_makefiles()` reads the installed
    /// `Options::rebuilding_makefiles` through the `OPTIONS_PTR` borrow channel
    /// the update walk and `reset_makeflags` use — including on the `gmk_eval`
    /// throwaway-context path.
    #[test]
    fn read_rebuilding_makefiles_through_channel() {
        install_default_options_for_test();

        with_options(|o| o.rebuilding_makefiles.set(false));
        assert!(!opt_rebuilding_makefiles(), "channel reads the installed value");

        with_options(|o| o.rebuilding_makefiles.set(true));
        assert!(opt_rebuilding_makefiles(), "true through the channel");

        with_options(|o| o.rebuilding_makefiles.set(false));
    }
}

#[cfg(test)]
mod option_default_statics_tests {
    use super::{default_keep_going_flag, default_print_directory_flag, inf_jobs};

    /// The option table's read-only `default_value`/`noarg_value` statics hold
    /// their initializers and are accessible from safe code (immutable, no
    /// `unsafe`).
    #[test]
    fn option_default_statics_hold_initializers() {
        assert_eq!(default_keep_going_flag, 0);
        assert_eq!(default_print_directory_flag, -1);
        assert_eq!(inf_jobs, 0);
    }
}

#[cfg(test)]
mod disable_builtins_latch_tests {
    use super::Options;

    /// The apply-once latches start cleared on a fresh `Options`, matching the
    /// original `static mut old_builtin_*_flag = 0` initial values.
    #[test]
    fn builtin_latches_start_cleared() {
        let options = Options::new();
        assert!(!options.prev_no_builtin_rules.get());
        assert!(!options.prev_no_builtin_variables.get());
        assert!(!Options::default().prev_no_builtin_variables.get());
    }

    /// `disable_builtins` clears the built-in rules/variables only on the
    /// `false -> true` transition of its latch, so the guarded work runs exactly
    /// once even though the function is reached repeatedly as `MAKEFLAGS` is
    /// re-parsed. This models that fold against the relocated `Options` latch
    /// (the run-owner reached process-wide via `installed_options()`), the field
    /// that replaced the `static mut old_builtin_rules_flag`.
    #[test]
    fn builtin_latch_fires_once_on_transition() {
        let options = Options::new();
        options.no_builtin_rules.set(true);
        let mut fired = 0;
        // The guard from disable_builtins, applied repeatedly: the flag stays
        // set, the latch transitions and the guard opens exactly once.
        for _ in 0..5 {
            if options.no_builtin_rules.get() && !options.prev_no_builtin_rules.get() {
                options.prev_no_builtin_rules.set(true);
                fired += 1;
            }
        }
        assert_eq!(fired, 1, "the cleanup runs exactly once across re-parses");

        // A pre-seeded latch (as main_0 seeds it from the startup options)
        // suppresses the work entirely.
        let seeded = Options::new();
        seeded.no_builtin_variables.set(true);
        seeded.prev_no_builtin_variables.set(true);
        let mut fired2 = 0;
        if seeded.no_builtin_variables.get() && !seeded.prev_no_builtin_variables.get() {
            seeded.prev_no_builtin_variables.set(true);
            fired2 += 1;
        }
        assert_eq!(fired2, 0, "a pre-seeded latch suppresses the work");
    }
}

#[cfg(test)]
mod option_helper_tests {
    use super::{opt_flag_int, opt_set_flag, opt_set_str, should_print_dir, Options, CHAR_MAX};

    /// Every `flag`-type switch round-trips through `opt_set_flag` ->
    /// `opt_flag_int`, covering both the letter and `CHAR_MAX`-offset codes.
    #[test]
    fn flag_options_round_trip() {
        let letters = [
            'B' as i32,
            'd' as i32,
            'e' as i32,
            'h' as i32,
            'i' as i32,
            'k' as i32,
            'S' as i32,
            'L' as i32,
            'n' as i32,
            'p' as i32,
            'q' as i32,
            'r' as i32,
            'R' as i32,
            's' as i32,
            't' as i32,
            'v' as i32,
            CHAR_MAX + 3,
            CHAR_MAX + 5,
            CHAR_MAX + 8,
            CHAR_MAX + 14,
        ];
        for &c in &letters {
            let o = Options::new();
            assert_eq!(opt_flag_int(&o, c), 0, "flag {c} should default to 0");
            opt_set_flag(&o, c, true);
            assert_eq!(
                opt_flag_int(&o, c),
                1,
                "flag {c} should read back 1 after set"
            );
        }
    }

    /// `-w` / `--no-print-directory` is the tri-state: unset reads -1, and
    /// set true/false read 1/0.
    #[test]
    fn print_directory_tristate() {
        for &c in &['w' as i32, CHAR_MAX + 4] {
            let o = Options::new();
            assert_eq!(opt_flag_int(&o, c), -1, "unset print_directory is -1");
            opt_set_flag(&o, c, true);
            assert_eq!(opt_flag_int(&o, c), 1);
            opt_set_flag(&o, c, false);
            assert_eq!(opt_flag_int(&o, c), 0);
        }
    }

    /// `string`-type switches store their value in the matching field.
    #[test]
    fn string_options_store() {
        let o = Options::new();
        opt_set_str(&o, 'O' as i32, "line".to_string());
        assert_eq!(o.output_sync_option.borrow().as_deref(), Some("line"));
        opt_set_str(&o, CHAR_MAX + 2, "fifo:/x".to_string());
        assert_eq!(o.jobserver_auth.borrow().as_deref(), Some("fifo:/x"));
        opt_set_str(&o, CHAR_MAX + 7, "mtx".to_string());
        assert_eq!(o.sync_mutex.borrow().as_deref(), Some("mtx"));
        opt_set_str(&o, CHAR_MAX + 11, "random".to_string());
        assert_eq!(o.shuffle_mode.borrow().as_deref(), Some("random"));
        opt_set_str(&o, CHAR_MAX + 12, "fifo".to_string());
        assert_eq!(o.jobserver_style.borrow().as_deref(), Some("fifo"));
    }

    /// `should_print_dir` honours the explicit tri-state when set, and falls
    /// back to the silent/dir-count heuristic when unset.
    #[test]
    fn should_print_dir_paths() {
        let ctx = crate::execctx::ExecContext::default();
        let o = Options::new();
        // Explicit -w wins.
        opt_set_flag(&o, 'w' as i32, true);
        assert!(should_print_dir(&ctx, &o));
        opt_set_flag(&o, 'w' as i32, false);
        assert!(!should_print_dir(&ctx, &o));
        // Unset: not silent, no -C dirs, makelevel 0 -> false.
        let o2 = Options::new();
        assert!(!should_print_dir(&ctx, &o2));
        // Silent suppresses it too.
        opt_set_flag(&o2, 's' as i32, true);
        assert!(!should_print_dir(&ctx, &o2));
    }
}

/// Verbatim copy of the pre-conversion `unsafe` implementation, kept as a
/// behavior oracle so the safe `should_print_dir` can be differential-tested
/// against the exact C-shaped logic it replaced.
#[cfg(test)]
mod should_print_dir_unsafe_oracle {
    use super::Options;
    use crate::execctx::ExecContext;

    pub unsafe fn should_print_dir(ctx: &ExecContext, options: &Options) -> i32 {
        if let Some(v) = options.print_directory.get() {
            return v as i32;
        }
        (!options.silent.get() && (ctx.makelevel() > 0 || !options.directories.borrow().is_empty()))
            as i32
    }
}

#[cfg(test)]
mod should_print_dir_diff_tests {
    use super::should_print_dir;
    use super::should_print_dir_unsafe_oracle::should_print_dir as oracle;
    use super::Options;

    /// Build an `Options` from a tri-state `--print-directory`, the silent
    /// flag, and a `-C` directory count, then assert the safe version and the
    /// preserved unsafe oracle agree on the boolean (the oracle's 0/1 maps to
    /// false/true).
    fn check(print_directory: Option<bool>, silent: bool, dir_count: usize) {
        let o = Options::new();
        o.print_directory.set(print_directory);
        o.silent.set(silent);
        {
            let mut dirs = o.directories.borrow_mut();
            for _ in 0..dir_count {
                dirs.push(c"d".to_owned());
            }
        }
        let ctx = crate::execctx::ExecContext::default();
        let safe = should_print_dir(&ctx, &o);
        // SAFETY: `o` is a fully owned, valid Options; the oracle only reads
        // its fields and the threaded `ctx` makelevel.
        let raw = unsafe { oracle(&ctx, &o) };
        assert_eq!(
            safe,
            raw != 0,
            "mismatch for {print_directory:?}/{silent}/{dir_count}"
        );
    }

    #[test]
    fn safe_matches_unsafe_oracle() {
        for print_directory in [None, Some(true), Some(false)] {
            for silent in [false, true] {
                for dir_count in [0usize, 1, 3] {
                    check(print_directory, silent, dir_count);
                }
            }
        }
    }
}

#[cfg(test)]
mod jobserver_and_stdin_cleanup_tests {
    use super::{reset_jobserver, stdin_offset, temp_stdin_name, temp_stdin_unlink, Options};

    /// `reset_jobserver` clears the auth field and tears down the (absent)
    /// jobserver. With no jobserver configured, `jobserver_clear` is a no-op,
    /// so this is safe to drive directly.
    #[test]
    fn reset_jobserver_clears_auth() {
        let o = Options::new();
        *o.jobserver_auth.borrow_mut() = Some("fifo:/tmp/x".to_string());
        unsafe { reset_jobserver(&o) };
        assert!(o.jobserver_auth.borrow().is_none());
    }

    /// `temp_stdin_unlink` removes the temp-stdin makefile and resets the
    /// offset. Drives the real unlink path with a throwaway temp file, then
    /// also confirms the no-op guard when no temp stdin is registered.
    #[test]
    fn temp_stdin_unlink_removes_file_and_noops() {
        // No temp stdin registered (defaults): must be a harmless no-op.
        let ctx = crate::execctx::ExecContext::default();
        unsafe {
            stdin_offset = -1;
            temp_stdin_name = ::core::ptr::null();
            temp_stdin_unlink(&ctx);
        }

        // Real unlink path: create a temp file and register it.
        let dir = std::env::temp_dir();
        let path = dir.join(format!("makers_tmpstdin_test_{}", std::process::id()));
        std::fs::write(&path, b"all:\n").unwrap();
        assert!(path.exists());
        let cpath = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
        unsafe {
            stdin_offset = 0;
            temp_stdin_name = cpath.as_ptr();
            temp_stdin_unlink(&ctx);
            // Restore globals before `cpath` is dropped to avoid a dangling ptr.
            temp_stdin_name = ::core::ptr::null();
            stdin_offset = -1;
        }
        assert!(!path.exists(), "temp stdin file should have been unlinked");
        drop(cpath);
    }
}
