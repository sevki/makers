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
use crate::misc::{get_tmpdir, get_tmpfile, spin};
use crate::read::construct_include_path;
use crate::remote_stub::{remote_cleanup, remote_setup};
use crate::stdio::FILE;
use crate::strcache::{strcache_init, strcache_print_stats};
use crate::variable::print_variable_data_base;
use crate::vpath::{build_vpath_lists, print_vpath_data_base};
use ::c2rust_bitfields;
use ::libc;
use libc::{
    __errno_location, _exit, abort, atof, chdir, exit, free, isatty, printf, putchar, putenv,
    setlocale, sprintf, stpcpy, strchr, strcmp, strerror, strrchr, tolower, ttyname, unlink,
};
extern "C" {
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sigprocmask(
        __how: ::core::ffi::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> ::core::ffi::c_int;
    fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    fn getcwd(__buf: *mut ::core::ffi::c_char, __size: size_t) -> *mut ::core::ffi::c_char;
    static mut environ: *mut *mut ::core::ffi::c_char;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    static mut opterr: ::core::ffi::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::core::ffi::c_char,
        __modes: ::core::ffi::c_int,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn concat(_: ::core::ffi::c_uint, ...) -> *const ::core::ffi::c_char;
    fn error(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn fatal(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn pfatal_with_name(_: *const ::core::ffi::c_char) -> !;
    fn perror_with_name(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char);
    fn make_toui(
        _: *const ::core::ffi::c_char,
        _: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_uint;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xcalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xrealloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn xstrdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strcache_add(str: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn guile_gmake_setup(flocp: *const Floc) -> ::core::ffi::c_int;
    fn load_file(
        flocp: *const Floc,
        file: *mut file,
        noerror: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut remote_description: *mut ::core::ffi::c_char;
    static mut make_host: *mut ::core::ffi::c_char;
    static mut version_string: *mut ::core::ffi::c_char;
    static mut handling_fatal_signal: sig_atomic_t;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn fatal_error_signal(sig: ::core::ffi::c_int);
    fn parse_file_seq(
        stringp: *mut *mut ::core::ffi::c_char,
        size: size_t,
        stopmap: ::core::ffi::c_int,
        prefix: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn tilde_expand(name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn free_ns_chain(n: *mut nameseq);
    fn read_all_makefiles(makefiles_0: *mut *const ::core::ffi::c_char) -> *mut goaldep;
    fn eval_buffer(buffer: *mut ::core::ffi::c_char, floc: *const Floc);
    fn update_goal_chain(goals_0: *mut goaldep) -> update_status;
    fn lookup_file(name: *const ::core::ffi::c_char) -> *mut file;
    fn enter_file(name: *const ::core::ffi::c_char) -> *mut file;
    fn remove_intermediates(sig: ::core::ffi::c_int);
    fn snap_deps();
    fn init_hash_files();
    fn verify_file_data_base();
    fn print_file_data_base();
    fn print_targets();
    fn file_timestamp_now(_: *mut ::core::ffi::c_int) -> uintmax_t;
    fn file_timestamp_sprintf(p: *mut ::core::ffi::c_char, ts: uintmax_t);
    fn f_mtime(file: *mut file, search: ::core::ffi::c_int) -> uintmax_t;
    fn getopt_long(
        argc: ::core::ffi::c_int,
        argv: *const *mut ::core::ffi::c_char,
        shortopts: *const ::core::ffi::c_char,
        longopts: *const option,
        longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut output_context: *mut output;
    static mut stdio_traced: ::core::ffi::c_uint;
    fn output_init(out: *mut output);
    fn output_close(out: *mut output);
    fn child_handler(sig: ::core::ffi::c_int);
    fn reap_children(block: ::core::ffi::c_int, err: ::core::ffi::c_int);
    fn exec_command(
        argv: *mut *mut ::core::ffi::c_char,
        envp: *mut *mut ::core::ffi::c_char,
    ) -> pid_t;
    static mut job_slots_used: ::core::ffi::c_uint;
    static mut jobserver_tokens: ::core::ffi::c_uint;
    fn check_io_state() -> ::core::ffi::c_uint;
    fn jobserver_enabled() -> ::core::ffi::c_uint;
    fn jobserver_setup(
        job_slots_0: ::core::ffi::c_int,
        style: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_uint;
    fn jobserver_parse_auth(auth: *const ::core::ffi::c_char) -> ::core::ffi::c_uint;
    fn jobserver_get_auth() -> *mut ::core::ffi::c_char;
    fn jobserver_clear();
    fn jobserver_acquire_all() -> ::core::ffi::c_uint;
    fn jobserver_release(is_fatal: ::core::ffi::c_int);
    fn jobserver_pre_child(_: ::core::ffi::c_int);
    fn jobserver_post_child(_: ::core::ffi::c_int);
    fn osync_setup();
    fn osync_get_mutex() -> *mut ::core::ffi::c_char;
    fn osync_parse_mutex(mutex: *const ::core::ffi::c_char) -> ::core::ffi::c_uint;
    fn osync_clear();
    static mut suffix_file: *mut file;
    fn snap_implicit_rules();
    fn convert_to_pattern();
    fn print_rule_data_base();
    static mut variable_buffer: *mut ::core::ffi::c_char;
    static mut current_variable_set_list: *mut variable_set_list;
    fn initialize_variable_output() -> *mut ::core::ffi::c_char;
    fn variable_buffer_output(
        ptr: *mut ::core::ffi::c_char,
        string_0: *const ::core::ffi::c_char,
        length: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn install_variable_buffer(bufp: *mut *mut ::core::ffi::c_char, lenp: *mut size_t);
    fn restore_variable_buffer(buf: *mut ::core::ffi::c_char, len: size_t);
    fn expand_string_buf(
        buf: *mut ::core::ffi::c_char,
        string_0: *const ::core::ffi::c_char,
        length: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn expand_variable_buf(
        buf: *mut ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
        length: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn define_automatic_variables();
    fn try_variable_definition(
        flocp: *const Floc,
        line: *const ::core::ffi::c_char,
        origin: variable_origin,
        scope: variable_scope,
    ) -> *mut variable;
    fn init_hash_global_variable_set();
    fn hash_init_function_table();
    fn lookup_variable(name: *const ::core::ffi::c_char, length: size_t) -> *mut variable;
    fn define_variable_in_set(
        name: *const ::core::ffi::c_char,
        length: size_t,
        value: *const ::core::ffi::c_char,
        origin: variable_origin,
        recursive: ::core::ffi::c_int,
        set: *mut variable_set,
        flocp: *const Floc,
    ) -> *mut variable;
    fn reset_env_override();
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
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub __pad0: ::core::ffi::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [::core::ffi::c_int; 28],
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
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: ::core::ffi::c_int,
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
    pub si_status: ::core::ffi::c_int,
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
    pub si_tid: ::core::ffi::c_int,
    pub si_overrun: ::core::ffi::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::core::ffi::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut siginfo_t, *mut ::core::ffi::c_void) -> (),
    >,
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
    pub c: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct output {
    pub out: ::core::ffi::c_int,
    pub err: ::core::ffi::c_int,
    #[bitfield(name = "syncout", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub syncout: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct goaldep {
    pub next: *mut goaldep,
    pub name: *const ::core::ffi::c_char,
    pub file: *mut file,
    pub shuf: *mut goaldep,
    pub stem: *const ::core::ffi::c_char,
    #[bitfield(name = "flags", ty = "::core::ffi::c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(
        name = "need_2nd_expansion",
        ty = "::core::ffi::c_uint",
        bits = "11..=11"
    )]
    #[bitfield(
        name = "ignore_automatic_vars",
        ty = "::core::ffi::c_uint",
        bits = "12..=12"
    )]
    #[bitfield(name = "is_explicit", ty = "::core::ffi::c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "::core::ffi::c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here:
        [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub error: ::core::ffi::c_int,
    pub floc: Floc,
}
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
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type bsd_signal_ret_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub const SIG_DFL: __sighandler_t = None;
pub const ENOENT: ::core::ffi::c_int = 2;
pub const EINTR: ::core::ffi::c_int = 4;
pub const SIGCHLD: ::core::ffi::c_int = 17;
pub const SIGUSR1: ::core::ffi::c_int = 10;
pub const SA_RESTART: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SIG_SETMASK: ::core::ffi::c_int = 2;
pub const _IOLBF: ::core::ffi::c_int = 1;
pub const BUFSIZ: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const UCHAR_MAX: ::core::ffi::c_int = __SCHAR_MAX__ * 2 + 1;
pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
pub const CHAR_MAX: ::core::ffi::c_int = __SCHAR_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PATH_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const GET_PATH_MAX: ::core::ffi::c_int = PATH_MAX;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const __LC_ALL: ::core::ffi::c_int = 6;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const DB_NONE: ::core::ffi::c_int = 0;
pub const DB_BASIC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const DB_VERBOSE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const DB_JOBS: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const DB_IMPLICIT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const DB_PRINT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const DB_WHY: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const DB_MAKEFILES: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const DB_ALL: ::core::ffi::c_int = 0xfff as ::core::ffi::c_int;
pub const MAP_NUL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAP_BLANK: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_NEWLINE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MAP_COMMENT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MAP_SEMI: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MAP_EQUALS: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MAP_COLON: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const MAP_VARSEP: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const MAP_PIPE: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const MAP_DOT: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const MAP_COMMA: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const MAP_USERFUNC: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const MAP_VARIABLE: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const MAP_DIRSEP: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const INTSTR_LENGTH: usize = (53 as usize)
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22 as usize)
    .wrapping_add(3 as usize);
pub const OUTPUT_SYNC_NONE: ::core::ffi::c_int = 0;
pub const OUTPUT_SYNC_LINE: ::core::ffi::c_int = 1;
pub const OUTPUT_SYNC_TARGET: ::core::ffi::c_int = 2;
pub const OUTPUT_SYNC_RECURSE: ::core::ffi::c_int = 3;
pub const MAKELEVEL_NAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"MAKELEVEL\0") };
pub const JOBSERVER_AUTH_OPT: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"jobserver-auth\0") };
pub const MAKE_SUCCESS: ::core::ffi::c_int = 0;
pub const MAKE_TROUBLE: ::core::ffi::c_int = 1;
pub const MAKE_FAILURE: ::core::ffi::c_int = 2;
pub const RM_INCLUDED: ::core::ffi::c_int = (1) << 1;
pub const RM_DONTCARE: ::core::ffi::c_int = (1) << 2;
pub const PARSEFS_NONE: ::core::ffi::c_int = 0;
#[inline]
#[no_mangle]
pub unsafe extern "C" fn alloc_goaldep() -> *mut goaldep {
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
#[inline]
#[no_mangle]
pub unsafe extern "C" fn free_goaldep(g: *mut goaldep) {
    free_dep(g as *mut dep);
}
#[inline]

unsafe extern "C" fn free_dep_chain(d: *mut dep) {
    free_ns_chain(d as *mut nameseq);
}
pub const UNKNOWN_MTIME: ::core::ffi::c_int = 0;
pub const NONEXISTENT_MTIME: ::core::ffi::c_int = 1;
pub const OLD_MTIME: ::core::ffi::c_int = 2;
pub const no_argument: ::core::ffi::c_int = 0;
pub const required_argument: ::core::ffi::c_int = 1;
pub const optional_argument: ::core::ffi::c_int = 2;
#[no_mangle]
pub static mut verify_flag: ::core::ffi::c_int = 0;
static mut silent_flag: ::core::ffi::c_int = 0;
static mut default_silent_flag: ::core::ffi::c_int = 0;
static mut silent_origin: variable_origin = o_default;
#[no_mangle]
pub static mut run_silent: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut touch_flag: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut just_print_flag: ::core::ffi::c_int = 0;
static mut db_flags: *mut stringlist = ::core::ptr::null::<stringlist>() as *mut stringlist;
static mut debug_flag: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut db_level: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut output_sync_option: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut env_overrides: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ignore_errors_flag: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut print_data_base_flag: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut print_targets_flag: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut question_flag: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut no_builtin_rules_flag: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut no_builtin_variables_flag: ::core::ffi::c_int = 0;
static mut old_builtin_rules_flag: ::core::ffi::c_int = 0;
static mut old_builtin_variables_flag: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut export_all_variables: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut keep_going_flag: ::core::ffi::c_int = 0;
static mut default_keep_going_flag: ::core::ffi::c_int = 0;
static mut keep_going_origin: variable_origin = o_default;
#[no_mangle]
pub static mut check_symlink_flag: ::core::ffi::c_int = 0;
static mut print_directory_flag: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut default_print_directory_flag: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut print_directory_origin: variable_origin = o_default;
#[no_mangle]
pub static mut print_version_flag: ::core::ffi::c_int = 0;
static mut makefiles: *mut stringlist = ::core::ptr::null::<stringlist>() as *mut stringlist;
#[no_mangle]
pub static mut job_slots: ::core::ffi::c_uint = 0;
pub const INVALID_JOB_SLOTS: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut master_job_slots: ::core::ffi::c_uint = 0;
static mut arg_job_slots: ::core::ffi::c_int = INVALID_JOB_SLOTS;
static mut default_job_slots: ::core::ffi::c_int = INVALID_JOB_SLOTS;
static mut inf_jobs: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut jobserver_auth: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut jobserver_style: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut shuffle_mode: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut sync_mutex: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut max_load_average: ::core::ffi::c_double = -1.0f64;
#[no_mangle]
pub static mut default_load_average: ::core::ffi::c_double = -1.0f64;
static mut directories: *mut stringlist = ::core::ptr::null::<stringlist>() as *mut stringlist;
static mut include_dirs: *mut stringlist = ::core::ptr::null::<stringlist>() as *mut stringlist;
static mut old_files: *mut stringlist = ::core::ptr::null::<stringlist>() as *mut stringlist;
static mut new_files: *mut stringlist = ::core::ptr::null::<stringlist>() as *mut stringlist;
static mut eval_strings: *mut stringlist = ::core::ptr::null::<stringlist>() as *mut stringlist;
static mut print_usage_flag: ::core::ffi::c_int = 0;
static mut warn_flags: *mut stringlist = ::core::ptr::null::<stringlist>() as *mut stringlist;
static mut warn_undefined_variables_flag: ::core::ffi::c_int = 0;
static mut always_make_set: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut always_make_flag: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut rebuilding_makefiles: ::core::ffi::c_int = 0;
#[no_mangle]
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
#[no_mangle]
pub static mut cmd_prefix: ::core::ffi::c_char = '\t' as i32 as ::core::ffi::c_char;
#[no_mangle]
pub static mut no_intermediates: ::core::ffi::c_uint = 0;
#[no_mangle]
pub static mut command_count: ::core::ffi::c_ulong = 1;
static mut stdin_offset: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
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
static mut trace_flag: ::core::ffi::c_int = 0;
pub const TEMP_STDIN_OPT: ::core::ffi::c_int = CHAR_MAX + 10;
pub const WARN_OPT: ::core::ffi::c_int = CHAR_MAX + 13;
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
        flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
        val: 's' as i32,
    },
    option {
        name: b"stop\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
        val: 'S' as i32,
    },
    option {
        name: b"new-file\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
        val: 'W' as i32,
    },
    option {
        name: b"assume-new\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
        val: 'W' as i32,
    },
    option {
        name: b"assume-old\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
        val: 'o' as i32,
    },
    option {
        name: b"max-load\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: optional_argument,
        flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
        val: 'l' as i32,
    },
    option {
        name: b"dry-run\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
        val: 'n' as i32,
    },
    option {
        name: b"recon\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
        val: 'n' as i32,
    },
    option {
        name: b"makefile\0" as *const u8 as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
        val: 'f' as i32,
    },
];
static mut goals: *mut goaldep = ::core::ptr::null::<goaldep>() as *mut goaldep;
static mut lastgoal: *mut goaldep = ::core::ptr::null::<goaldep>() as *mut goaldep;
static mut command_variables: *mut command_variable =
    ::core::ptr::null::<command_variable>() as *mut command_variable;
#[no_mangle]
pub static mut program: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
pub static mut directory_before_chdir: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut starting_directory: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut makelevel: ::core::ffi::c_uint = 0;
#[no_mangle]
pub static mut default_goal_var: *mut variable = ::core::ptr::null::<variable>() as *mut variable;
#[no_mangle]
pub static mut default_file: *mut file = ::core::ptr::null::<file>() as *mut file;
#[no_mangle]
pub static mut posix_pedantic: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut second_expansion: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut one_shell: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut output_sync: ::core::ffi::c_int = OUTPUT_SYNC_NONE;
#[no_mangle]
pub static mut not_parallel: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut clock_skew_detected: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut stopchar_map: [::core::ffi::c_ushort; 256] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub static mut make_sync: output = output {
    out: 0,
    err: 0,
    syncout: [0; 1],
    c2rust_padding: [0; 3],
};
#[no_mangle]
pub static mut fatal_signal_set: sigset_t = __sigset_t { __val: [0; 16] };
unsafe extern "C" fn bsd_signal(
    sig: ::core::ffi::c_int,
    func: bsd_signal_ret_t,
) -> bsd_signal_ret_t {
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
            -(1 as ::core::ffi::c_int) as ::libc::intptr_t,
        );
    }
    oact.__sigaction_handler.sa_handler as bsd_signal_ret_t
}
#[no_mangle]
pub unsafe extern "C" fn initialize_global_hash_tables() {
    init_hash_global_variable_set();
    strcache_init();
    init_hash_files();
    hash_init_directories();
    hash_init_function_table();
}
#[no_mangle]
pub unsafe extern "C" fn initialize_stopchar_map() {
    let mut i: ::core::ffi::c_int;
    stopchar_map[0 as usize] = MAP_NUL as ::core::ffi::c_ushort;
    stopchar_map['#' as i32 as usize] = MAP_COMMENT as ::core::ffi::c_ushort;
    stopchar_map[';' as i32 as usize] = MAP_SEMI as ::core::ffi::c_ushort;
    stopchar_map['=' as i32 as usize] = MAP_EQUALS as ::core::ffi::c_ushort;
    stopchar_map[':' as i32 as usize] = MAP_COLON as ::core::ffi::c_ushort;
    stopchar_map['|' as i32 as usize] = MAP_PIPE as ::core::ffi::c_ushort;
    stopchar_map['.' as i32 as usize] = (MAP_DOT | MAP_USERFUNC) as ::core::ffi::c_ushort;
    stopchar_map[',' as i32 as usize] = MAP_COMMA as ::core::ffi::c_ushort;
    stopchar_map['(' as i32 as usize] = MAP_VARSEP as ::core::ffi::c_ushort;
    stopchar_map['{' as i32 as usize] = MAP_VARSEP as ::core::ffi::c_ushort;
    stopchar_map['}' as i32 as usize] = MAP_VARSEP as ::core::ffi::c_ushort;
    stopchar_map[')' as i32 as usize] = MAP_VARSEP as ::core::ffi::c_ushort;
    stopchar_map['$' as i32 as usize] = MAP_VARIABLE as ::core::ffi::c_ushort;
    stopchar_map['-' as i32 as usize] = MAP_USERFUNC as ::core::ffi::c_ushort;
    stopchar_map['_' as i32 as usize] = MAP_USERFUNC as ::core::ffi::c_ushort;
    stopchar_map[' ' as i32 as usize] = MAP_BLANK as ::core::ffi::c_ushort;
    stopchar_map['\t' as i32 as usize] = MAP_BLANK as ::core::ffi::c_ushort;
    stopchar_map['/' as i32 as usize] = MAP_DIRSEP as ::core::ffi::c_ushort;
    i = 1;
    while i <= UCHAR_MAX {
        if *(*__ctype_b_loc()).offset(i as isize) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
            && !(stopchar_map[i as usize] as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0)
        {
            stopchar_map[i as usize] = (stopchar_map[i as usize] as ::core::ffi::c_int
                | MAP_NEWLINE) as ::core::ffi::c_ushort;
        } else if *(*__ctype_b_loc()).offset(i as isize) as ::core::ffi::c_int
            & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
        {
            stopchar_map[i as usize] = (stopchar_map[i as usize] as ::core::ffi::c_int
                | MAP_USERFUNC) as ::core::ffi::c_ushort;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn close_stdout() {
    let prev_fail: ::core::ffi::c_int = ferror(stdout);
    let fclose_fail: ::core::ffi::c_int = fclose(stdout);
    if prev_fail != 0 || fclose_fail != 0 {
        if fclose_fail != 0 {
            perror_with_name(
                b"write error: stdout\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            error(
                ::core::ptr::null_mut::<Floc>(),
                0,
                b"write error: stdout\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        exit(MAKE_TROUBLE);
    }
}
unsafe extern "C" fn expand_command_line_file(
    mut name: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let cp: *const ::core::ffi::c_char;
    let mut expanded: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0 {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            0,
            b"empty string invalid as file name\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '~' as i32 {
        expanded = tilde_expand(name);
        if !expanded.is_null()
            && *expanded.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        {
            name = expanded;
        }
    }
    while *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32
        && *name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32
    {
        name = name.offset(2 as ::core::ffi::c_int as isize);
        while *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32 {
            name = name.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0 {
        name = b"./\0" as *const u8 as *const ::core::ffi::c_char;
    }
    cp = strcache_add(name);
    free(expanded as *mut ::core::ffi::c_void);
    cp
}
#[no_mangle]
pub unsafe extern "C" fn debug_signal_handler(mut _sig: ::core::ffi::c_int) {
    db_level = if db_level != 0 { DB_NONE } else { DB_BASIC };
}
#[no_mangle]
pub unsafe extern "C" fn decode_debug_flags() {
    let mut pp: *mut *const ::core::ffi::c_char;
    if debug_flag != 0 {
        db_level = DB_ALL;
    }
    if trace_flag != 0 {
        db_level |= DB_PRINT | DB_WHY;
    }
    if !db_flags.is_null() {
        pp = (*db_flags).list;
        while !(*pp).is_null() {
            let mut p: *const ::core::ffi::c_char = *pp;
            loop {
                match tolower(*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int) {
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
                            ::core::ptr::null_mut::<Floc>(),
                            strlen(p) as size_t,
                            b"unknown debug level specification '%s'\0" as *const u8
                                as *const ::core::ffi::c_char,
                            p,
                        );
                    }
                }
                loop {
                    p = p.offset(1 as ::core::ffi::c_int as isize);
                    if !(*p as ::core::ffi::c_int != 0) {
                        break;
                    }
                    if !(*p as ::core::ffi::c_int == ',' as i32
                        || *p as ::core::ffi::c_int == ' ' as i32)
                    {
                        continue;
                    }
                    p = p.offset(1 as ::core::ffi::c_int as isize);
                    break;
                }
                if *p as ::core::ffi::c_int == 0 {
                    break;
                }
            }
            pp = pp.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    if db_level != 0 {
        verify_flag = 1;
    }
    if db_level == 0 {
        debug_flag = 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn decode_output_sync_flags() {
    if !output_sync_option.is_null() {
        if *output_sync_option as ::core::ffi::c_int
            == *(b"none\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
            && (*output_sync_option as ::core::ffi::c_int == 0
                || strcmp(
                    output_sync_option.offset(1 as ::core::ffi::c_int as isize),
                    (b"none\0" as *const u8 as *const ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                ) == 0)
        {
            output_sync = OUTPUT_SYNC_NONE;
        } else if *output_sync_option as ::core::ffi::c_int
            == *(b"line\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
            && (*output_sync_option as ::core::ffi::c_int == 0
                || strcmp(
                    output_sync_option.offset(1 as ::core::ffi::c_int as isize),
                    (b"line\0" as *const u8 as *const ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                ) == 0)
        {
            output_sync = OUTPUT_SYNC_LINE;
        } else if *output_sync_option as ::core::ffi::c_int
            == *(b"target\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
            && (*output_sync_option as ::core::ffi::c_int == 0
                || strcmp(
                    output_sync_option.offset(1 as ::core::ffi::c_int as isize),
                    (b"target\0" as *const u8 as *const ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                ) == 0)
        {
            output_sync = OUTPUT_SYNC_TARGET;
        } else if *output_sync_option as ::core::ffi::c_int
            == *(b"recurse\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
            && (*output_sync_option as ::core::ffi::c_int == 0
                || strcmp(
                    output_sync_option.offset(1 as ::core::ffi::c_int as isize),
                    (b"recurse\0" as *const u8 as *const ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                ) == 0)
        {
            output_sync = OUTPUT_SYNC_RECURSE;
        } else {
            fatal(
                ::core::ptr::null_mut::<Floc>(),
                strlen(output_sync_option) as size_t,
                b"unknown output-sync type '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                output_sync_option,
            );
        }
    }
    if !sync_mutex.is_null() {
        osync_parse_mutex(sync_mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_usage(bad: ::core::ffi::c_int) -> ! {
    let mut cpp: *const *const ::core::ffi::c_char;
    let usageto: *mut FILE;
    if print_version_flag != 0 {
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
        cpp = cpp.offset(1 as ::core::ffi::c_int as isize);
    }
    if remote_description.is_null() || *remote_description as ::core::ffi::c_int == 0 {
        fprintf(
            usageto,
            b"\nThis program built for %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            make_host,
        );
    } else {
        fprintf(
            usageto,
            b"\nThis program built for %s (%s)\n\0" as *const u8 as *const ::core::ffi::c_char,
            make_host,
            remote_description,
        );
    }
    fprintf(
        usageto,
        b"Report bugs to <bug-make@gnu.org>\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    die(if bad != 0 { MAKE_FAILURE } else { MAKE_SUCCESS });
}
#[no_mangle]
pub unsafe extern "C" fn reset_jobserver() {
    jobserver_clear();
    free(jobserver_auth as *mut ::core::ffi::c_void);
    jobserver_auth = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn temp_stdin_unlink() {
    if stdin_offset >= 0 {
        let nm: *const ::core::ffi::c_char = *(*makefiles).list.offset(stdin_offset as isize);
        let mut r: ::core::ffi::c_int;
        stdin_offset = -(1 as ::core::ffi::c_int);
        loop {
            r = unlink(nm);
            if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 && *__errno_location() != ENOENT && handling_fatal_signal == 0 {
            perror_with_name(
                b"unlink (temporary file): \0" as *const u8 as *const ::core::ffi::c_char,
                nm,
            );
        }
    }
}
unsafe fn main_0(
    argc: ::core::ffi::c_int,
    argv: *mut *mut ::core::ffi::c_char,
    envp: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let current_block: u64;
    let mut makefile_status: ::core::ffi::c_int = MAKE_SUCCESS;
    let mut read_files: *mut goaldep;
    let mut current_directory: [::core::ffi::c_char; 4097] = [0; 4097];
    let mut restarts: ::core::ffi::c_uint = 0;
    let mut syncing: ::core::ffi::c_uint;
    let argv_slots: ::core::ffi::c_int;
    initialize_variable_output();
    spin(b"main-entry\0" as *const u8 as *const ::core::ffi::c_char);
    if check_io_state() & 0x8 as ::core::ffi::c_uint != 0 {
        atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    }
    output_init(&raw mut make_sync);
    initialize_stopchar_map();
    crate::warning::init();
    verify_flag = 1;
    setlocale(LC_ALL, b"\0" as *const u8 as *const ::core::ffi::c_char);
    sigemptyset(&raw mut fatal_signal_set);
    if bsd_signal(
        1,
        Some(fatal_error_signal as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    ) == ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
        1 as ::core::ffi::c_int as ::libc::intptr_t,
    ) {
        bsd_signal(
            1,
            ::core::mem::transmute::<::libc::intptr_t, bsd_signal_ret_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            ),
        );
    } else {
        sigaddset(&raw mut fatal_signal_set, 1);
    }
    if bsd_signal(
        3,
        Some(fatal_error_signal as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    ) == ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
        1 as ::core::ffi::c_int as ::libc::intptr_t,
    ) {
        bsd_signal(
            3,
            ::core::mem::transmute::<::libc::intptr_t, bsd_signal_ret_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            ),
        );
    } else {
        sigaddset(&raw mut fatal_signal_set, 3);
    }
    if bsd_signal(
        13,
        Some(fatal_error_signal as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    ) == ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
        1 as ::core::ffi::c_int as ::libc::intptr_t,
    ) {
        bsd_signal(
            13,
            ::core::mem::transmute::<::libc::intptr_t, bsd_signal_ret_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            ),
        );
    } else {
        sigaddset(&raw mut fatal_signal_set, 13);
    }
    if bsd_signal(
        2,
        Some(fatal_error_signal as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    ) == ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
        1 as ::core::ffi::c_int as ::libc::intptr_t,
    ) {
        bsd_signal(
            2,
            ::core::mem::transmute::<::libc::intptr_t, bsd_signal_ret_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            ),
        );
    } else {
        sigaddset(&raw mut fatal_signal_set, 2);
    }
    if bsd_signal(
        15,
        Some(fatal_error_signal as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    ) == ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
        1 as ::core::ffi::c_int as ::libc::intptr_t,
    ) {
        bsd_signal(
            15,
            ::core::mem::transmute::<::libc::intptr_t, bsd_signal_ret_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            ),
        );
    } else {
        sigaddset(&raw mut fatal_signal_set, 15);
    }
    if bsd_signal(
        24,
        Some(fatal_error_signal as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    ) == ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
        1 as ::core::ffi::c_int as ::libc::intptr_t,
    ) {
        bsd_signal(
            24,
            ::core::mem::transmute::<::libc::intptr_t, bsd_signal_ret_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            ),
        );
    } else {
        sigaddset(&raw mut fatal_signal_set, 24);
    }
    if bsd_signal(
        25,
        Some(fatal_error_signal as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    ) == ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
        1 as ::core::ffi::c_int as ::libc::intptr_t,
    ) {
        bsd_signal(
            25,
            ::core::mem::transmute::<::libc::intptr_t, bsd_signal_ret_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            ),
        );
    } else {
        sigaddset(&raw mut fatal_signal_set, 25);
    }
    bsd_signal(SIGCHLD, SIG_DFL);
    output_init(::core::ptr::null_mut::<output>());
    if (*argv.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        let ref mut fresh33 = *argv.offset(0 as ::core::ffi::c_int as isize);
        *fresh33 = b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    if *(*argv.offset(0 as ::core::ffi::c_int as isize)).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 0
    {
        program = b"make\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        program = strrchr(*argv.offset(0 as ::core::ffi::c_int as isize), '/' as i32);
        if program.is_null() {
            program = *argv.offset(0 as ::core::ffi::c_int as isize);
        } else {
            program = program.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    initialize_global_hash_tables();
    get_tmpdir();
    if getcwd(
        &raw mut current_directory as *mut ::core::ffi::c_char,
        GET_PATH_MAX as size_t,
    )
    .is_null()
    {
        perror_with_name(
            b"getcwd\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
        current_directory[0 as ::core::ffi::c_int as usize] = 0;
        directory_before_chdir = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        directory_before_chdir = xstrdup(&raw mut current_directory as *mut ::core::ffi::c_char);
    }
    let ref mut fresh34 = *define_variable_in_set(
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    (*fresh34).set_special(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let ref mut fresh35 = *define_variable_in_set(
        b".VARIABLES\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 11]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    (*fresh35).set_special(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let ref mut fresh36 = *define_variable_in_set(
        b".RECIPEPREFIX\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    (*fresh36).set_special(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let ref mut fresh37 = *define_variable_in_set(
        b".WARNINGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    (*fresh37).set_special(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    define_variable_in_set(
        b".SHELLFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t).wrapping_sub(1),
        b"-c\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
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
        while !(stopchar_map[*ep as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
            & (0x20 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
            != 0)
        {
            ep = ep.offset(1 as ::core::ffi::c_int as isize);
        }
        if !(*ep as ::core::ffi::c_int == 0) {
            let fresh38 = ep;
            ep = ep.offset(1 as ::core::ffi::c_int as isize);
            len = fresh38.offset_from(*envp.offset(i as isize)) as ::core::ffi::c_long as size_t;
            if len == 13
                && memcmp(
                    *envp.offset(i as isize) as *const ::core::ffi::c_void,
                    b"MAKE_RESTARTS\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
                ) == 0
            {
                if *ep as ::core::ffi::c_int == '-' as i32 {
                    stdio_traced = 1;
                    ep = ep.offset(1 as ::core::ffi::c_int as isize);
                }
                restarts = make_toui(ep, ::core::ptr::null_mut::<*const ::core::ffi::c_char>());
                export = v_noexport;
            }
            v = define_variable_in_set(
                *envp.offset(i as isize),
                len,
                ep,
                o_env,
                1,
                (*current_variable_set_list).set,
                NILF,
            );
            if *(*v).name as ::core::ffi::c_int
                == *(b"SHELL\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
                && (*(*v).name as ::core::ffi::c_int == 0
                    || strcmp(
                        (*v).name.offset(1 as ::core::ffi::c_int as isize),
                        (b"SHELL\0" as *const u8 as *const ::core::ffi::c_char)
                            .offset(1 as ::core::ffi::c_int as isize),
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
        b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
    )
    .is_null()
    {
        decode_env_switches(
            b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
            o_command,
        );
        define_variable_in_set(
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
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        o_command,
    );
    make_sync.set_syncout(
        (output_sync == OUTPUT_SYNC_LINE || output_sync == OUTPUT_SYNC_TARGET) as ::core::ffi::c_int
            as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
    output_context = if make_sync.syncout() as ::core::ffi::c_int != 0 {
        &raw mut make_sync
    } else {
        ::core::ptr::null_mut::<output>()
    };
    let env_slots: ::core::ffi::c_int = arg_job_slots;
    arg_job_slots = INVALID_JOB_SLOTS;
    decode_switches(argc, argv as *mut *const ::core::ffi::c_char, o_command);
    argv_slots = arg_job_slots;
    if arg_job_slots == INVALID_JOB_SLOTS {
        arg_job_slots = env_slots;
    }
    if print_usage_flag != 0 {
        print_usage(0);
    }
    if print_version_flag != 0 {
        print_version();
        die(MAKE_SUCCESS);
    }
    setvbuf(
        stdout,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        _IOLBF,
        BUFSIZ as size_t,
    );
    if !shuffle_mode.is_null() {
        let arg = ::core::ffi::CStr::from_ptr(shuffle_mode)
            .to_str()
            .unwrap_or("");
        crate::shuffle::set_mode(arg);
        free(shuffle_mode as *mut ::core::ffi::c_void);
        shuffle_mode = match crate::shuffle::get_mode() {
            Some(s) => {
                let cs = ::std::ffi::CString::new(s).unwrap();
                xstrdup(cs.as_ptr())
            }
            None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
    }
    if isatty(fileno(stdout)) != 0 {
        if lookup_variable(
            b"MAKE_TERMOUT\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        )
        .is_null()
        {
            let tty: *const ::core::ffi::c_char = ttyname(fileno(stdout));
            let ref mut fresh39 = *define_variable_in_set(
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
            );
            (*fresh39).set_export(v_export as variable_export);
        }
    }
    if isatty(fileno(stderr)) != 0 {
        if lookup_variable(
            b"MAKE_TERMERR\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        )
        .is_null()
        {
            let tty_0: *const ::core::ffi::c_char = ttyname(fileno(stderr));
            let ref mut fresh40 = *define_variable_in_set(
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
            );
            (*fresh40).set_export(v_export as variable_export);
        }
    }
    syncing = (output_sync == OUTPUT_SYNC_LINE || output_sync == OUTPUT_SYNC_TARGET)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
    if make_sync.syncout() as ::core::ffi::c_int != 0 && syncing == 0 {
        output_close(&raw mut make_sync);
    }
    make_sync.set_syncout(syncing as ::core::ffi::c_uint);
    output_context = if make_sync.syncout() as ::core::ffi::c_int != 0 {
        &raw mut make_sync
    } else {
        ::core::ptr::null_mut::<output>()
    };
    let v_0: *mut variable = lookup_variable(
        b"MAKELEVEL\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
    );
    if !v_0.is_null()
        && *(*v_0).value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && *(*v_0).value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '-' as i32
    {
        makelevel = make_toui(
            (*v_0).value,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
    } else {
        makelevel = 0;
    }
    always_make_flag = (always_make_set != 0 && restarts == 0) as ::core::ffi::c_int;
    if no_builtin_variables_flag != 0 {
        no_builtin_rules_flag = 1;
    }
    if 0x1 as ::core::ffi::c_int & db_level != 0 {
        print_version();
        fflush(stdout);
    }
    if current_directory[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        && !(*argv.offset(0 as ::core::ffi::c_int as isize)).is_null()
        && *(*argv.offset(0 as ::core::ffi::c_int as isize))
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '/' as i32
        && !strchr(*argv.offset(0 as ::core::ffi::c_int as isize), '/' as i32).is_null()
    {
        let ref mut fresh41 = *argv.offset(0 as ::core::ffi::c_int as isize);
        *fresh41 = xstrdup(concat(
            3,
            &raw mut current_directory as *mut ::core::ffi::c_char,
            b"/\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ));
    }
    starting_directory = &raw mut current_directory as *mut ::core::ffi::c_char;
    if !directories.is_null() {
        let mut i_0: ::core::ffi::c_uint;
        i_0 = 0;
        while !(*(*directories).list.offset(i_0 as isize)).is_null() {
            let dir: *const ::core::ffi::c_char = *(*directories).list.offset(i_0 as isize);
            if chdir(dir) < 0 {
                pfatal_with_name(dir);
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    if !directories.is_null() {
        if getcwd(
            &raw mut current_directory as *mut ::core::ffi::c_char,
            GET_PATH_MAX as size_t,
        )
        .is_null()
        {
            perror_with_name(
                b"getcwd\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
            );
            starting_directory = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            starting_directory = &raw mut current_directory as *mut ::core::ffi::c_char;
        }
    }
    define_variable_in_set(
        b"CURDIR\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1),
        &raw mut current_directory as *mut ::core::ffi::c_char,
        o_file,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    construct_include_path(if !include_dirs.is_null() {
        (*include_dirs).list
    } else {
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>()
    });
    if !jobserver_auth.is_null() {
        if argv_slots == INVALID_JOB_SLOTS {
            if jobserver_parse_auth(jobserver_auth) != 0 {
                current_block = 6702893977082974455;
            } else {
                error(
                    ::core::ptr::null_mut::<Floc>(),
                    0,
                    b"warning: jobserver unavailable: using -j1 (add '+' to parent make rule)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                arg_job_slots = 1;
                current_block = 2473505634946569239;
            }
        } else {
            if restarts == 0 && argv_slots != 1 {
                error(
                    ::core::ptr::null_mut::<Floc>(),
                    INTSTR_LENGTH,
                    b"warning: -j%d forced in submake: resetting jobserver mode\0" as *const u8
                        as *const ::core::ffi::c_char,
                    argv_slots,
                );
            }
            current_block = 2473505634946569239;
        }
        match current_block {
            6702893977082974455 => {}
            _ => {
                reset_jobserver();
            }
        }
    }
    define_variable_in_set(
        b"MAKE_COMMAND\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        *argv.offset(0 as ::core::ffi::c_int as isize),
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
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
        let value: *mut ::core::ffi::c_char;
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
        value = xmalloc(len_0) as *mut ::core::ffi::c_char;
        p = value;
        cv = command_variables;
        while !cv.is_null() {
            v_1 = (*cv).variable;
            p = quote_for_env(p, (*v_1).name);
            if (*v_1).recursive() == 0 {
                let fresh42 = p;
                p = p.offset(1 as ::core::ffi::c_int as isize);
                *fresh42 = ':' as i32 as ::core::ffi::c_char;
            }
            let fresh43 = p;
            p = p.offset(1 as ::core::ffi::c_int as isize);
            *fresh43 = '=' as i32 as ::core::ffi::c_char;
            p = quote_for_env(p, (*v_1).value);
            let fresh44 = p;
            p = p.offset(1 as ::core::ffi::c_int as isize);
            *fresh44 = ' ' as i32 as ::core::ffi::c_char;
            cv = (*cv).next;
        }
        *p.offset(-(1 as ::core::ffi::c_int) as isize) = 0;
        define_variable_in_set(
            b"-*-command-variables-*-\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 24]>() as size_t).wrapping_sub(1),
            value,
            o_automatic,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
        free(value as *mut ::core::ffi::c_void);
        define_variable_in_set(
            b"MAKEOVERRIDES\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
            b"${-*-command-variables-*-}\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            1,
            (*current_variable_set_list).set,
            NILF,
        );
    }
    if !makefiles.is_null() {
        let mut i_1: ::core::ffi::c_uint;
        i_1 = 0;
        while i_1 < (*makefiles).idx {
            if *(*(*makefiles).list.offset(i_1 as isize)).offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                == '-' as i32
                && *(*(*makefiles).list.offset(i_1 as isize))
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 0
            {
                let outfile: *mut FILE;
                let mut newnm: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                if stdin_offset >= 0 {
                    fatal(
                        ::core::ptr::null_mut::<Floc>(),
                        0,
                        b"Makefile from standard input specified twice\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                outfile = get_tmpfile(&raw mut newnm);
                if outfile.is_null() {
                    fatal(
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
                let ref mut fresh45 = *(*makefiles).list.offset(i_1 as isize);
                *fresh45 = strcache_add(newnm);
                stdin_offset = i_1 as ::core::ffi::c_int;
                free(newnm as *mut ::core::ffi::c_void);
            }
            i_1 = i_1.wrapping_add(1);
        }
    }
    if stdin_offset >= 0 {
        let mut f: *mut file = enter_file(*(*makefiles).list.offset(stdin_offset as isize));
        (*f).set_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*f).set_update_status(us_success as update_status);
        (*f).set_command_state(cs_finished as cmd_state);
        (*f).set_intermediate(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*f).set_dontcare(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*f).mtime_before_update = f_mtime(f, 0);
        (*f).last_mtime = (*f).mtime_before_update;
    }
    bsd_signal(
        SIGCHLD,
        Some(child_handler as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
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
            b"sigprocmask(SIG_SETMASK, SIGCHLD)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    bsd_signal(
        SIGUSR1,
        Some(debug_signal_handler as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    set_default_suffixes();
    define_automatic_variables();
    let ref mut fresh46 = *define_makeflags(0);
    (*fresh46).set_export(v_export as variable_export);
    define_default_variables();
    default_file = enter_file(strcache_add(
        b".DEFAULT\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    default_goal_var = define_variable_in_set(
        b".DEFAULT_GOAL\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_file,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    if !eval_strings.is_null() {
        let mut p_0: *mut ::core::ffi::c_char;
        let mut endp: *mut ::core::ffi::c_char;
        let value_0: *mut ::core::ffi::c_char;
        let mut i_2: ::core::ffi::c_uint;
        let mut len_1: size_t = (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t)
            .wrapping_sub(1)
            .wrapping_add(1)
            .wrapping_mul((*eval_strings).idx as size_t);
        i_2 = 0;
        while i_2 < (*eval_strings).idx {
            p_0 = xstrdup(*(*eval_strings).list.offset(i_2 as isize));
            len_1 = len_1.wrapping_add((2 as size_t).wrapping_mul(strlen(p_0)) as size_t);
            eval_buffer(p_0, ::core::ptr::null::<Floc>());
            free(p_0 as *mut ::core::ffi::c_void);
            i_2 = i_2.wrapping_add(1);
        }
        value_0 = xmalloc(len_1) as *mut ::core::ffi::c_char;
        endp = value_0;
        p_0 = endp;
        i_2 = 0;
        while i_2 < (*eval_strings).idx {
            p_0 = stpcpy(p_0, b"--eval=\0" as *const u8 as *const ::core::ffi::c_char);
            p_0 = quote_for_env(p_0, *(*eval_strings).list.offset(i_2 as isize));
            let fresh47 = p_0;
            p_0 = p_0.offset(1 as ::core::ffi::c_int as isize);
            endp = fresh47;
            *endp = ' ' as i32 as ::core::ffi::c_char;
            i_2 = i_2.wrapping_add(1);
        }
        *endp = 0;
        define_variable_in_set(
            b"-*-eval-flags-*-\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 17]>() as size_t).wrapping_sub(1),
            value_0,
            o_automatic,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
        free(value_0 as *mut ::core::ffi::c_void);
    }
    let old_arg_job_slots: ::core::ffi::c_int = arg_job_slots;
    old_builtin_rules_flag = no_builtin_rules_flag;
    old_builtin_variables_flag = no_builtin_variables_flag;
    read_files = read_all_makefiles(if makefiles.is_null() {
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>()
    } else {
        (*makefiles).list
    });
    arg_job_slots = INVALID_JOB_SLOTS;
    decode_env_switches(
        b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        o_env,
    );
    define_variable_in_set(
        b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_override,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    decode_env_switches(
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        o_env,
    );
    if arg_job_slots == INVALID_JOB_SLOTS || argv_slots != INVALID_JOB_SLOTS {
        arg_job_slots = old_arg_job_slots;
    } else if !jobserver_auth.is_null() && arg_job_slots != old_arg_job_slots {
        if restarts == 0 {
            error(
                ::core::ptr::null_mut::<Floc>(),
                INTSTR_LENGTH,
                b"warning: -j%d forced in makefile: resetting jobserver mode\0" as *const u8
                    as *const ::core::ffi::c_char,
                arg_job_slots,
            );
        }
        reset_jobserver();
    }
    syncing = (output_sync == OUTPUT_SYNC_LINE || output_sync == OUTPUT_SYNC_TARGET)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
    if make_sync.syncout() as ::core::ffi::c_int != 0 && syncing == 0 {
        output_close(&raw mut make_sync);
    }
    make_sync.set_syncout(syncing as ::core::ffi::c_uint);
    output_context = if make_sync.syncout() as ::core::ffi::c_int != 0 {
        &raw mut make_sync
    } else {
        ::core::ptr::null_mut::<output>()
    };
    disable_builtins();
    if !jobserver_auth.is_null() {
        job_slots = 0;
    } else if arg_job_slots == INVALID_JOB_SLOTS {
        job_slots = 1;
    } else {
        job_slots = arg_job_slots as ::core::ffi::c_uint;
    }
    if job_slots > 1
        && jobserver_setup(
            job_slots.wrapping_sub(1) as ::core::ffi::c_int,
            jobserver_style,
        ) != 0
    {
        jobserver_auth = jobserver_get_auth();
        if !jobserver_auth.is_null() {
            master_job_slots = job_slots;
            job_slots = 0;
        }
    }
    if syncing != 0 && job_slots == 1 {
        output_context = ::core::ptr::null_mut::<output>();
        output_close(&raw mut make_sync);
        syncing = 0;
        output_sync = OUTPUT_SYNC_NONE;
    }
    if syncing != 0 {
        if sync_mutex.is_null() {
            osync_setup();
            sync_mutex = osync_get_mutex();
        } else if osync_parse_mutex(sync_mutex) == 0 {
            osync_clear();
            free(sync_mutex as *mut ::core::ffi::c_void);
            sync_mutex = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    if !jobserver_auth.is_null() {
        if (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int) & db_level != 0 {
            printf(
                b"Using jobserver controller %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                jobserver_auth,
            );
            fflush(stdout);
        }
    }
    if !sync_mutex.is_null() {
        if 0x2 as ::core::ffi::c_int & db_level != 0 {
            printf(
                b"Using output-sync mutex %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                sync_mutex,
            );
            fflush(stdout);
        }
    }
    define_makeflags(0);
    snap_deps();
    install_default_suffix_rules();
    convert_to_pattern();
    install_default_implicit_rules();
    snap_implicit_rules();
    build_vpath_lists();
    if !old_files.is_null() {
        let mut p_1: *mut *const ::core::ffi::c_char;
        p_1 = (*old_files).list;
        while !(*p_1).is_null() {
            let mut f_0: *mut file = enter_file(*p_1);
            (*f_0).mtime_before_update = OLD_MTIME as uintmax_t;
            (*f_0).last_mtime = (*f_0).mtime_before_update;
            (*f_0).set_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*f_0).set_update_status(us_success as update_status);
            (*f_0).set_command_state(cs_finished as cmd_state);
            p_1 = p_1.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    if print_targets_flag != 0 {
        print_targets();
        die(EXIT_SUCCESS);
    }
    if restarts == 0 && !new_files.is_null() {
        let mut p_2: *mut *const ::core::ffi::c_char;
        p_2 = (*new_files).list;
        while !(*p_2).is_null() {
            let mut f_1: *mut file = enter_file(*p_2);
            (*f_1).mtime_before_update = (!(0 as ::core::ffi::c_int as uintmax_t)).wrapping_sub(
                if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                    0 as ::core::ffi::c_int as uintmax_t
                } else {
                    !(0 as ::core::ffi::c_int as uintmax_t)
                        << (::core::mem::size_of::<uintmax_t>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(1 as usize)
                },
            );
            (*f_1).last_mtime = (*f_1).mtime_before_update;
            p_2 = p_2.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    remote_setup();
    output_context = ::core::ptr::null_mut::<output>();
    output_close(&raw mut make_sync);
    if !shuffle_mode.is_null() {
        if 0x1 as ::core::ffi::c_int & db_level != 0 {
            printf(
                b"Enabled shuffle mode: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                shuffle_mode,
            );
            fflush(stdout);
        }
    }
    if !read_files.is_null() {
        let makefile_mtimes: *mut uintmax_t;
        let mut skipped_makefiles: *mut goaldep = ::core::ptr::null_mut::<goaldep>();
        let mut nargv: *mut *const ::core::ffi::c_char = argv as *mut *const ::core::ffi::c_char;
        let mut any_failed: ::core::ffi::c_int = 0;
        let mut status: update_status;
        if 0x1 as ::core::ffi::c_int & db_level != 0 {
            printf(b"Updating makefiles....\n\0" as *const u8 as *const ::core::ffi::c_char);
            fflush(stdout);
        }
        let mut num_mkfiles: ::core::ffi::c_uint = 0;
        let mut d: *mut goaldep = read_files;
        read_files = ::core::ptr::null_mut::<goaldep>();
        while !d.is_null() {
            let mut t: *mut goaldep = d;
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
        while !d_0.is_null() {
            let mut skip: ::core::ffi::c_int = 0;
            let mut f_2: *mut file = (*d_0).file;
            if (*f_2).phony() != 0 {
                skip = 1;
            } else {
                f_2 = (*f_2).double_colon;
                while !f_2.is_null() {
                    if (*f_2).deps.is_null() && !(*f_2).cmds.is_null() {
                        skip = 1;
                        break;
                    } else {
                        f_2 = (*f_2).prev;
                    }
                }
            }
            if skip == 0 {
                let fresh48 = mm_idx;
                mm_idx = mm_idx.wrapping_add(1);
                *makefile_mtimes.offset(fresh48 as isize) =
                    if (*(*d_0).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                        f_mtime((*d_0).file, 0)
                    } else {
                        (*(*d_0).file).last_mtime
                    };
                last = d_0;
                d_0 = (*d_0).next;
            } else {
                if 0x2 as ::core::ffi::c_int & db_level != 0 {
                    printf(
                        b"Makefile '%s' might loop; not remaking it.\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*f_2).name,
                    );
                    fflush(stdout);
                }
                if !last.is_null() {
                    (*last).next = (*d_0).next;
                } else {
                    read_files = (*d_0).next;
                }
                if (*d_0).error != 0 && (*d_0).flags() as ::core::ffi::c_int & RM_DONTCARE == 0 {
                    (*d_0).next = skipped_makefiles;
                    skipped_makefiles = d_0;
                    any_failed = 1;
                } else {
                    free_goaldep(d_0);
                }
                d_0 = if !last.is_null() {
                    (*last).next
                } else {
                    read_files
                };
            }
        }
        define_makeflags(1);
        let orig_db_level: ::core::ffi::c_int = db_level;
        if 0x100 as ::core::ffi::c_int & db_level == 0 {
            db_level = DB_NONE;
        }
        rebuilding_makefiles = 1;
        status = update_goal_chain(read_files) as update_status;
        rebuilding_makefiles = 0;
        db_level = orig_db_level;
        while !skipped_makefiles.is_null() {
            let d_1: *mut goaldep = skipped_makefiles;
            let err: *const ::core::ffi::c_char = strerror((*d_1).error);
            error(
                &raw mut (*d_1).floc,
                (strlen(if !(*d_1).name.is_null() {
                    (*d_1).name
                } else {
                    (*(*d_1).file).name
                }) as size_t)
                    .wrapping_add(strlen(err) as size_t),
                b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                if !(*d_1).name.is_null() {
                    (*d_1).name
                } else {
                    (*(*d_1).file).name
                },
                err,
            );
            skipped_makefiles = (*skipped_makefiles).next;
            free_goaldep(d_1);
        }
        if any_failed != 0
            && status as ::core::ffi::c_uint
                == us_success as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            status = us_none;
        }
        let current_block_552: u64;
        match status as ::core::ffi::c_uint {
            1 => {
                let mut d_2: *mut goaldep;
                d_2 = read_files;
                while !d_2.is_null() {
                    if (*(*d_2).file).unloaded() != 0 {
                        let f_3: *mut file = (*d_2).file;
                        if load_file(&raw mut (*d_2).floc, f_3, 0) == 0 {
                            fatal(
                                &raw mut (*d_2).floc,
                                strlen((*f_3).name) as size_t,
                                b"%s: failed to load\0" as *const u8 as *const ::core::ffi::c_char,
                                (*f_3).name,
                            );
                        }
                        (*f_3).set_unloaded(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*f_3).set_loaded(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                    d_2 = (*d_2).next;
                }
                current_block_552 = 6015864261243718670;
            }
            3 => {
                let mut any_remade: ::core::ffi::c_int = 0;
                let mut i_3: ::core::ffi::c_uint;
                let mut d_4: *mut goaldep;
                i_3 = 0;
                d_4 = read_files;
                while !d_4.is_null() {
                    if (*(*d_4).file).updated() != 0 {
                        if (*(*d_4).file).update_status() as ::core::ffi::c_int
                            == us_success as ::core::ffi::c_int
                        {
                            any_remade |=
                                ((if (*(*d_4).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                                    f_mtime((*d_4).file, 0)
                                } else {
                                    (*(*d_4).file).last_mtime
                                }) != *makefile_mtimes.offset(i_3 as isize))
                                    as ::core::ffi::c_int;
                        } else if (*d_4).flags() as ::core::ffi::c_int & RM_DONTCARE == 0 {
                            let mtime: uintmax_t;
                            error(
                                &raw mut (*d_4).floc,
                                strlen((*(*d_4).file).name) as size_t,
                                b"failed to remake makefile '%s'\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*(*d_4).file).name,
                            );
                            mtime = if (*(*d_4).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                                f_mtime((*d_4).file, 0)
                            } else {
                                (*(*d_4).file).last_mtime
                            };
                            any_remade |= (mtime != NONEXISTENT_MTIME as uintmax_t
                                && mtime != *makefile_mtimes.offset(i_3 as isize))
                                as ::core::ffi::c_int;
                            makefile_status = MAKE_FAILURE;
                            any_failed = 1;
                        }
                    } else if (*d_4).flags() as ::core::ffi::c_int & RM_DONTCARE == 0 {
                        let dnm: *const ::core::ffi::c_char = if !(*d_4).name.is_null() {
                            (*d_4).name
                        } else {
                            (*(*d_4).file).name
                        };
                        if (*d_4).flags() as ::core::ffi::c_int & RM_INCLUDED != 0 {
                            error(
                                &raw mut (*d_4).floc,
                                strlen(dnm) as size_t,
                                b"included makefile '%s' was not found\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                dnm,
                            );
                        } else {
                            error(
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
                    d_4 = (*d_4).next;
                }
                if any_remade != 0 {
                    current_block_552 = 5019185999593908445;
                } else {
                    current_block_552 = 6015864261243718670;
                }
            }
            0 => {
                current_block_552 = 5019185999593908445;
            }
            2 | _ => {
                current_block_552 = 6015864261243718670;
            }
        }
        match current_block_552 {
            6015864261243718670 => {}
            _ => {
                remove_intermediates(0);
                if print_data_base_flag != 0 {
                    print_data_base();
                }
                clean_jobserver(0);
                if !makefiles.is_null() {
                    let mut mfidx: ::core::ffi::c_int = 0;
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
                    av = av.offset(1 as ::core::ffi::c_int as isize);
                    let fresh50 = nv;
                    nv = nv.offset(1 as ::core::ffi::c_int as isize);
                    *fresh50 = *fresh49;
                    let mut current_block_505: u64;
                    while !(*av).is_null() {
                        let f_4: *mut ::core::ffi::c_char;
                        let a: *mut ::core::ffi::c_char = *av;
                        let mf: *const ::core::ffi::c_char =
                            *(*makefiles).list.offset(mfidx as isize);
                        '_c2rust_label: {
                            if strlen(a) > 0 {
                            } else {
                                __assert_fail(
                                    b"strlen (a) > 0\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
                                    2602 as ::core::ffi::c_uint,
                                    b"int main(int, char **, char **)\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                        };
                        *nv = a;
                        if !(*a.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            != '-' as i32)
                        {
                            if *a.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == '-' as i32
                            {
                                if strcmp(a, b"--file\0" as *const u8 as *const ::core::ffi::c_char)
                                    == 0
                                    || strcmp(
                                        a,
                                        b"--makefile\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) == 0
                                {
                                    av = av.offset(1 as ::core::ffi::c_int as isize);
                                    current_block_505 = 11348189074185403949;
                                } else if !(strncmp(
                                    a,
                                    b"--file=\0" as *const u8 as *const ::core::ffi::c_char,
                                    7,
                                ) == 0)
                                    && !(strncmp(
                                        a,
                                        b"--makefile=\0" as *const u8 as *const ::core::ffi::c_char,
                                        11,
                                    ) == 0)
                                {
                                    current_block_505 = 7920992607899116551;
                                } else {
                                    current_block_505 = 11348189074185403949;
                                }
                                match current_block_505 {
                                    7920992607899116551 => {}
                                    _ => {
                                        if mfidx == stdin_offset {
                                            alloca_allocations.push(::std::vec::from_elem(
                                                0,
                                                (::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                                                    as usize)
                                                    .wrapping_sub(1 as usize)
                                                    .wrapping_add(strlen(mf) as usize)
                                                    .wrapping_add(1 as usize)
                                                    as usize,
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
                                                b"-f%s\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                mf,
                                            );
                                            *nv = na_0;
                                        }
                                        mfidx += 1;
                                    }
                                }
                            } else {
                                f_4 = strchr(a, 'f' as i32);
                                if !f_4.is_null() {
                                    if *f_4.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == 0
                                    {
                                        av = av.offset(1 as ::core::ffi::c_int as isize);
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
                                            na_1 =
                                                alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                                    as *mut ::core::ffi::c_char;
                                            memcpy(
                                                na_1 as *mut ::core::ffi::c_void,
                                                a as *const ::core::ffi::c_void,
                                                al as size_t,
                                            );
                                            *na_1.offset(al as isize) = 0;
                                            let fresh51 = nv;
                                            nv = nv.offset(1 as ::core::ffi::c_int as isize);
                                            *fresh51 = na_1;
                                        }
                                        alloca_allocations.push(::std::vec::from_elem(
                                            0,
                                            (::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                                                as usize)
                                                .wrapping_sub(1 as usize)
                                                .wrapping_add(strlen(mf) as usize)
                                                .wrapping_add(1 as usize)
                                                as usize,
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
                                    } else if *f_4.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == 0
                                    {
                                        nv = nv.offset(1 as ::core::ffi::c_int as isize);
                                        *nv = mf;
                                    } else {
                                        let al_0: size_t =
                                            (f_4.offset_from(a) as ::core::ffi::c_long + 1)
                                                as size_t;
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
                                            na_2.offset(al_0 as isize) as *mut ::core::ffi::c_void,
                                            mf as *const ::core::ffi::c_void,
                                            ml as size_t,
                                        );
                                        *nv = na_2;
                                    }
                                    mfidx += 1;
                                }
                            }
                        }
                        av = av.offset(1 as ::core::ffi::c_int as isize);
                        nv = nv.offset(1 as ::core::ffi::c_int as isize);
                    }
                    *nv = ::core::ptr::null::<::core::ffi::c_char>();
                }
                if !directories.is_null() && (*directories).idx > 0 {
                    let mut bad: ::core::ffi::c_int = 1;
                    if !directory_before_chdir.is_null() {
                        if chdir(directory_before_chdir) < 0 {
                            perror_with_name(
                                b"chdir\0" as *const u8 as *const ::core::ffi::c_char,
                                b"\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                        } else {
                            bad = 0;
                        }
                    }
                    if bad != 0 {
                        fatal(
                            ::core::ptr::null_mut::<Floc>(),
                            0,
                            b"couldn't change back to original directory\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
                restarts = restarts.wrapping_add(1);
                if 0x1 as ::core::ffi::c_int & db_level != 0 {
                    let mut p_3: *mut *const ::core::ffi::c_char;
                    printf(
                        b"Re-executing[%u]:\0" as *const u8 as *const ::core::ffi::c_char,
                        restarts,
                    );
                    p_3 = nargv;
                    while !(*p_3).is_null() {
                        printf(b" %s\0" as *const u8 as *const ::core::ffi::c_char, *p_3);
                        p_3 = p_3.offset(1 as ::core::ffi::c_int as isize);
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
                            40 as ::core::ffi::c_int as ::core::ffi::c_ulong as usize,
                        ));
                        *p_4 = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                            as *mut ::core::ffi::c_char;
                        sprintf(
                            *p_4,
                            b"%s=%u\0" as *const u8 as *const ::core::ffi::c_char,
                            MAKELEVEL_NAME.as_ptr(),
                            makelevel,
                        );
                    } else if strncmp(
                        *p_4,
                        b"MAKE_RESTARTS=\0" as *const u8 as *const ::core::ffi::c_char,
                        (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t)
                            .wrapping_sub(1),
                    ) == 0
                    {
                        alloca_allocations.push(::std::vec::from_elem(
                            0,
                            40 as ::core::ffi::c_int as ::core::ffi::c_ulong as usize,
                        ));
                        *p_4 = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                            as *mut ::core::ffi::c_char;
                        sprintf(
                            *p_4,
                            b"MAKE_RESTARTS=%s%u\0" as *const u8 as *const ::core::ffi::c_char,
                            if stdio_traced != 0 {
                                b"-\0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"\0" as *const u8 as *const ::core::ffi::c_char
                            },
                            restarts,
                        );
                        restarts = 0;
                    }
                    p_4 = p_4.offset(1 as ::core::ffi::c_int as isize);
                }
                if restarts != 0 {
                    alloca_allocations.push(::std::vec::from_elem(
                        0,
                        40 as ::core::ffi::c_int as ::core::ffi::c_ulong as usize,
                    ));
                    let b: *mut ::core::ffi::c_char =
                        alloca_allocations.last_mut().unwrap().as_mut_ptr()
                            as *mut ::core::ffi::c_char;
                    sprintf(
                        b,
                        b"MAKE_RESTARTS=%s%u\0" as *const u8 as *const ::core::ffi::c_char,
                        if stdio_traced != 0 {
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
                exec_command(nargv as *mut *mut ::core::ffi::c_char, environ);
                jobserver_post_child(1);
                temp_stdin_unlink();
                _exit(127);
            }
        }
        if any_failed != 0 {
            die(MAKE_FAILURE);
        }
    }
    define_makeflags(0);
    always_make_flag = always_make_set;
    if restarts != 0 && !new_files.is_null() {
        let mut p_5: *mut *const ::core::ffi::c_char;
        p_5 = (*new_files).list;
        while !(*p_5).is_null() {
            let mut f_5: *mut file = enter_file(*p_5);
            (*f_5).mtime_before_update = (!(0 as ::core::ffi::c_int as uintmax_t)).wrapping_sub(
                if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                    0 as ::core::ffi::c_int as uintmax_t
                } else {
                    !(0 as ::core::ffi::c_int as uintmax_t)
                        << (::core::mem::size_of::<uintmax_t>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(1 as usize)
                },
            );
            (*f_5).last_mtime = (*f_5).mtime_before_update;
            p_5 = p_5.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    temp_stdin_unlink();
    if goals.is_null() {
        let mut p_6: *mut ::core::ffi::c_char;
        if (*default_goal_var).recursive() != 0 {
            p_6 = expand_string_buf(
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
        if *p_6 as ::core::ffi::c_int != 0 {
            let mut f_6: *mut file = lookup_file(p_6);
            if f_6.is_null() {
                let mut ns: *mut nameseq;
                ns = parse_file_seq(
                    &raw mut p_6,
                    ::core::mem::size_of::<nameseq>() as size_t,
                    MAP_NUL,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    PARSEFS_NONE,
                ) as *mut nameseq;
                if !ns.is_null() {
                    if !(*ns).next.is_null() {
                        fatal(
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
            b"MAKEFILE_LIST\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
        );
        if !v_2.is_null()
            && !(*v_2).value.is_null()
            && *(*v_2).value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        {
            fatal(
                ::core::ptr::null_mut::<Floc>(),
                0,
                b"No targets\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            0,
            b"No targets specified and no makefile found\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    crate::shuffle::shuffle_deps_recursive(goals as *mut crate::file::Dep);
    if 0x1 as ::core::ffi::c_int & db_level != 0 {
        printf(b"Updating goal targets....\n\0" as *const u8 as *const ::core::ffi::c_char);
        fflush(stdout);
    }
    match update_goal_chain(goals) as ::core::ffi::c_uint {
        2 => {
            makefile_status = MAKE_TROUBLE;
        }
        3 => {
            makefile_status = MAKE_FAILURE;
        }
        1 | 0 | _ => {}
    }
    if clock_skew_detected != 0 {
        error(
            ::core::ptr::null_mut::<Floc>(),
            0,
            b"warning: clock skew detected: your build may be incomplete\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    die(makefile_status);
}
static mut options: [::core::ffi::c_char; 127] = [0; 127];
static mut long_options: [option; 51] = [option {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    has_arg: 0,
    flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    val: 0,
}; 51];
#[no_mangle]
pub unsafe extern "C" fn init_switches() {
    let mut p: *mut ::core::ffi::c_char;
    let mut c: ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint;
    if options[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0 {
        return;
    }
    p = &raw mut options as *mut ::core::ffi::c_char;
    let fresh24 = p;
    p = p.offset(1 as ::core::ffi::c_int as isize);
    *fresh24 = '-' as i32 as ::core::ffi::c_char;
    i = 0;
    while switches[i as usize].c != 0 {
        long_options[i as usize].name = (if switches[i as usize].long_name.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            switches[i as usize].long_name
        }) as *mut ::core::ffi::c_char;
        long_options[i as usize].flag = ::core::ptr::null_mut::<::core::ffi::c_int>();
        long_options[i as usize].val = switches[i as usize].c;
        if switches[i as usize].c <= CHAR_MAX {
            let fresh25 = p;
            p = p.offset(1 as ::core::ffi::c_int as isize);
            *fresh25 = switches[i as usize].c as ::core::ffi::c_char;
        }
        match switches[i as usize].type_0 as ::core::ffi::c_uint {
            0 | 1 | 7 => {
                long_options[i as usize].has_arg = no_argument;
            }
            2 | 3 | 4 | 5 | 6 => {
                if switches[i as usize].c <= CHAR_MAX {
                    let fresh26 = p;
                    p = p.offset(1 as ::core::ffi::c_int as isize);
                    *fresh26 = ':' as i32 as ::core::ffi::c_char;
                }
                if !switches[i as usize].noarg_value.is_null() {
                    if switches[i as usize].c <= CHAR_MAX {
                        let fresh27 = p;
                        p = p.offset(1 as ::core::ffi::c_int as isize);
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
unsafe extern "C" fn handle_non_switch_argument(
    arg: *const ::core::ffi::c_char,
    origin: variable_origin,
) -> ::core::ffi::c_uint {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let v: *mut variable;
    if *arg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
        && *arg.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0
    {
        return 0;
    }
    v = try_variable_definition(::core::ptr::null::<Floc>(), arg, origin, s_global);
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
    } else if *arg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && origin as ::core::ffi::c_uint == o_command as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let f: *mut file;
        if strcmp(arg, b".WAIT\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            return 1;
        }
        f = enter_file(strcache_add(expand_command_line_file(arg)));
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
#[no_mangle]
pub unsafe extern "C" fn reset_makeflags(origin: variable_origin) {
    env_overrides = 0;
    decode_env_switches(
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        origin,
    );
    construct_include_path(if !include_dirs.is_null() {
        (*include_dirs).list
    } else {
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>()
    });
    disable_builtins();
    define_makeflags(rebuilding_makefiles);
}
unsafe extern "C" fn decode_switches(
    argc: ::core::ffi::c_int,
    argv: *mut *const ::core::ffi::c_char,
    origin: variable_origin,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut bad: ::core::ffi::c_int = 0;
    let mut cs: *mut command_switch;
    let mut sl: *mut stringlist;
    let mut targets: stringlist = stringlist {
        list: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        idx: 0,
        max: 0,
    };
    let mut c: ::core::ffi::c_int;
    let mut found_wait: ::core::ffi::c_uint = 0;
    let mut a: *mut *const ::core::ffi::c_char;
    static mut using_getopt: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if using_getopt == 0 {
        } else {
            __assert_fail(
                b"using_getopt == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
                3125 as ::core::ffi::c_uint,
                b"void decode_switches(int, const char **, enum variable_origin)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    using_getopt = 1;
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
    opterr = (origin as ::core::ffi::c_uint
        == o_command as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    optind = 0;
    while optind < argc {
        let mut coptarg: *const ::core::ffi::c_char;
        c = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            &raw mut options as *mut ::core::ffi::c_char,
            &raw mut long_options as *mut option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
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
            let ref mut fresh9 = *targets.list.offset(fresh8 as isize);
            *fresh9 = coptarg;
        } else {
            cs = &raw mut switches as *mut command_switch;
            while (*cs).c != 0 {
                if (*cs).c == c {
                    let doit: ::core::ffi::c_int = (origin as ::core::ffi::c_uint
                        == o_command as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*cs).env() as ::core::ffi::c_int != 0
                            && ((*cs).origin.is_null()
                                || origin as ::core::ffi::c_uint
                                    >= *(*cs).origin as ::core::ffi::c_uint))
                        as ::core::ffi::c_int;
                    if doit != 0 {
                        (*cs).set_specified(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                    let mut current_block_90: u64;
                    match (*cs).type_0 as ::core::ffi::c_uint {
                        7 => {}
                        0 | 1 => {
                            if doit != 0 {
                                *((*cs).value_ptr as *mut ::core::ffi::c_int) = ((*cs).type_0
                                    as ::core::ffi::c_uint
                                    == flag as ::core::ffi::c_int as ::core::ffi::c_uint)
                                    as ::core::ffi::c_int;
                                if !(*cs).origin.is_null() {
                                    *(*cs).origin = origin;
                                }
                            }
                        }
                        2 | 3 | 4 => {
                            if !(doit == 0) {
                                if coptarg.is_null() {
                                    coptarg = (*cs).noarg_value as *const ::core::ffi::c_char;
                                    current_block_90 = 9853141518545631134;
                                } else if *coptarg as ::core::ffi::c_int == 0 {
                                    let mut opt: [::core::ffi::c_char; 2] = ::core::mem::transmute::<
                                        [u8; 2],
                                        [::core::ffi::c_char; 2],
                                    >(
                                        *b"c\0"
                                    );
                                    let mut op: *const ::core::ffi::c_char =
                                        &raw mut opt as *mut ::core::ffi::c_char;
                                    if (*cs).c <= CHAR_MAX {
                                        opt[0 as ::core::ffi::c_int as usize] =
                                            (*cs).c as ::core::ffi::c_char;
                                    } else {
                                        op = (*cs).long_name;
                                    }
                                    error(
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
                                    current_block_90 = 6540614962658479183;
                                } else {
                                    current_block_90 = 9853141518545631134;
                                }
                                match current_block_90 {
                                    6540614962658479183 => {}
                                    _ => {
                                        if (*cs).type_0 as ::core::ffi::c_uint
                                            == string as ::core::ffi::c_int as ::core::ffi::c_uint
                                        {
                                            let val: *mut *mut ::core::ffi::c_char =
                                                (*cs).value_ptr as *mut *mut ::core::ffi::c_char;
                                            free(*val as *mut ::core::ffi::c_void);
                                            *val = xstrdup(coptarg);
                                            if !(*cs).origin.is_null() {
                                                *(*cs).origin = origin;
                                            }
                                        } else {
                                            sl = *((*cs).value_ptr as *mut *mut stringlist);
                                            if sl.is_null() {
                                                sl =
                                                    xmalloc(::core::mem::size_of::<stringlist>()
                                                        as size_t)
                                                        as *mut stringlist;
                                                (*sl).max = 5;
                                                (*sl).idx = 0;
                                                (*sl).list =
                                                    xmalloc((5 as size_t).wrapping_mul(
                                                        ::core::mem::size_of::<
                                                            *mut ::core::ffi::c_char,
                                                        >(
                                                        )
                                                            as size_t,
                                                    ))
                                                        as *mut *const ::core::ffi::c_char;
                                                let ref mut fresh10 =
                                                    *((*cs).value_ptr as *mut *mut stringlist);
                                                *fresh10 = sl;
                                            } else if (*sl).idx == (*sl).max.wrapping_sub(1) {
                                                (*sl).max = (*sl).max.wrapping_add(5);
                                                (*sl).list = xrealloc(
                                                    (*sl).list as *mut ::core::ffi::c_void,
                                                    ((*sl).max as size_t).wrapping_mul(
                                                        ::core::mem::size_of::<
                                                            *mut ::core::ffi::c_char,
                                                        >(
                                                        )
                                                            as size_t,
                                                    ),
                                                )
                                                    as *mut *const ::core::ffi::c_char;
                                            }
                                            if (*cs).c != 'f' as i32 && (*cs).c != WARN_OPT {
                                                let mut k: ::core::ffi::c_uint;
                                                k = 0;
                                                while k < (*sl).idx {
                                                    if **(*sl).list.offset(k as isize)
                                                        as ::core::ffi::c_int
                                                        == *coptarg as ::core::ffi::c_int
                                                        && (**(*sl).list.offset(k as isize)
                                                            as ::core::ffi::c_int
                                                            == 0
                                                            || strcmp(
                                                                (*(*sl).list.offset(k as isize))
                                                                    .offset(
                                                                        1 as ::core::ffi::c_int
                                                                            as isize,
                                                                    ),
                                                                coptarg.offset(
                                                                    1 as ::core::ffi::c_int
                                                                        as isize,
                                                                ),
                                                            ) == 0)
                                                    {
                                                        break;
                                                    }
                                                    k = k.wrapping_add(1);
                                                }
                                                if k < (*sl).idx {
                                                    current_block_90 = 6540614962658479183;
                                                } else {
                                                    current_block_90 = 2290177392965769716;
                                                }
                                            } else {
                                                current_block_90 = 2290177392965769716;
                                            }
                                            match current_block_90 {
                                                6540614962658479183 => {}
                                                _ => {
                                                    if (*cs).type_0 as ::core::ffi::c_uint
                                                        == strlist as ::core::ffi::c_int
                                                            as ::core::ffi::c_uint
                                                    {
                                                        let fresh11 = (*sl).idx;
                                                        (*sl).idx = (*sl).idx.wrapping_add(1);
                                                        let ref mut fresh12 =
                                                            *(*sl).list.offset(fresh11 as isize);
                                                        *fresh12 = xstrdup(coptarg);
                                                        if !(*cs).origin.is_null() {
                                                            *(*cs).origin = origin;
                                                        }
                                                    } else if (*cs).c == TEMP_STDIN_OPT {
                                                        if stdin_offset > 0 {
                                                            fatal(
                                                                NILF,
                                                                0,
                                                                b"INTERNAL: multiple --temp-stdin options provided!\0"
                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                            );
                                                        }
                                                        stdin_offset =
                                                            (*sl).idx as ::core::ffi::c_int;
                                                        let fresh13 = (*sl).idx;
                                                        (*sl).idx = (*sl).idx.wrapping_add(1);
                                                        let ref mut fresh14 =
                                                            *(*sl).list.offset(fresh13 as isize);
                                                        *fresh14 = strcache_add(coptarg);
                                                        if !(*cs).origin.is_null() {
                                                            *(*cs).origin = origin;
                                                        }
                                                    } else {
                                                        let fresh15 = (*sl).idx;
                                                        (*sl).idx = (*sl).idx.wrapping_add(1);
                                                        let ref mut fresh16 =
                                                            *(*sl).list.offset(fresh15 as isize);
                                                        *fresh16 =
                                                            expand_command_line_file(coptarg);
                                                        if !(*cs).origin.is_null() {
                                                            *(*cs).origin = origin;
                                                        }
                                                    }
                                                    let ref mut fresh17 =
                                                        *(*sl).list.offset((*sl).idx as isize);
                                                    *fresh17 =
                                                        ::core::ptr::null::<::core::ffi::c_char>();
                                                }
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
                                while (*cp.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    .wrapping_sub('0' as i32 as ::core::ffi::c_uint)
                                    <= 9
                                {
                                    cp = cp.offset(1 as ::core::ffi::c_int as isize);
                                }
                                if *cp.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 0
                                {
                                    let fresh18 = optind;
                                    optind = optind + 1;
                                    coptarg = *argv.offset(fresh18 as isize);
                                }
                            }
                            if !(doit == 0) {
                                if !coptarg.is_null() {
                                    let mut err: *const ::core::ffi::c_char =
                                        ::core::ptr::null::<::core::ffi::c_char>();
                                    let i: ::core::ffi::c_uint =
                                        make_toui(coptarg, &raw mut err);
                                    if !err.is_null() || i == 0 {
                                        error(
                                            NILF,
                                            0,
                                            b"the '-%c' option requires a positive integer argument\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                            (*cs).c,
                                        );
                                        bad = 1;
                                    } else {
                                        *((*cs).value_ptr as *mut ::core::ffi::c_uint) = i;
                                        if !(*cs).origin.is_null() {
                                            *(*cs).origin = origin;
                                        }
                                    }
                                } else {
                                    *((*cs).value_ptr as *mut ::core::ffi::c_uint) =
                                        *((*cs).noarg_value as *mut ::core::ffi::c_uint);
                                    if !(*cs).origin.is_null() {
                                        *(*cs).origin = origin;
                                    }
                                }
                            }
                        }
                        6 => {
                            if coptarg.is_null()
                                && optind < argc
                                && ((*(*argv.offset(optind as isize))
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    .wrapping_sub('0' as i32 as ::core::ffi::c_uint)
                                    <= 9
                                    || *(*argv.offset(optind as isize))
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '.' as i32)
                            {
                                let fresh19 = optind;
                                optind = optind + 1;
                                coptarg = *argv.offset(fresh19 as isize);
                            }
                            if doit != 0 {
                                *((*cs).value_ptr as *mut ::core::ffi::c_double) =
                                    if !coptarg.is_null() {
                                        atof(coptarg)
                                    } else {
                                        *((*cs).noarg_value as *mut ::core::ffi::c_double)
                                    };
                                if !(*cs).origin.is_null() {
                                    *(*cs).origin = origin;
                                }
                            }
                        }
                        _ => {
                            abort();
                        }
                    }
                    break;
                } else {
                    cs = cs.offset(1 as ::core::ffi::c_int as isize);
                }
            }
        }
    }
    while optind < argc {
        let fresh20 = optind;
        optind = optind + 1;
        let fresh21 = targets.idx;
        targets.idx = targets.idx.wrapping_add(1);
        let ref mut fresh22 = *targets.list.offset(fresh21 as isize);
        *fresh22 = *argv.offset(fresh20 as isize);
    }
    let ref mut fresh23 = *targets.list.offset(targets.idx as isize);
    *fresh23 = ::core::ptr::null::<::core::ffi::c_char>();
    using_getopt = 0;
    a = targets.list;
    while !(*a).is_null() {
        let prior_found_wait: ::core::ffi::c_int = found_wait as ::core::ffi::c_int;
        found_wait = handle_non_switch_argument(*a, origin);
        if prior_found_wait != 0 && !lastgoal.is_null() {
            (*lastgoal).set_wait_here(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        a = a.offset(1 as ::core::ffi::c_int as isize);
    }
    if bad != 0
        && origin as ::core::ffi::c_uint == o_command as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        print_usage(bad);
    }
    decode_debug_flags();
    decode_output_sync_flags();
    if warn_undefined_variables_flag != 0 {
        crate::warning::decode_actions("undefined-var", None);
        warn_undefined_variables_flag = 0;
    }
    if !warn_flags.is_null() {
        let mut pp: *mut *const ::core::ffi::c_char = (*warn_flags).list;
        while !(*pp).is_null() {
            let arg = ::core::ffi::CStr::from_ptr(*pp).to_str().unwrap_or("");
            crate::warning::decode_actions(arg, None);
            pp = pp.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    run_silent = silent_flag;
    reset_env_override();
}
unsafe extern "C" fn decode_env_switches(
    envar: *const ::core::ffi::c_char,
    mut len: size_t,
    origin: variable_origin,
) {
    let mut value: *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char;
    let buf: *mut ::core::ffi::c_char;
    let mut argc: ::core::ffi::c_int;
    let argv: *mut *const ::core::ffi::c_char;
    value = expand_variable_buf(::core::ptr::null_mut::<::core::ffi::c_char>(), envar, len);
    while stopchar_map[*value as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        != 0
    {
        value = value.offset(1 as ::core::ffi::c_int as isize);
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
    let ref mut fresh0 = *argv.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = b"\0" as *const u8 as *const ::core::ffi::c_char;
    argc = 1;
    buf = xmalloc((1 as size_t).wrapping_add(len).wrapping_add(1)) as *mut ::core::ffi::c_char;
    *buf.offset(0 as ::core::ffi::c_int as isize) = '-' as i32 as ::core::ffi::c_char;
    p = buf.offset(1 as ::core::ffi::c_int as isize);
    let ref mut fresh1 = *argv.offset(argc as isize);
    *fresh1 = p;
    while *value as ::core::ffi::c_int != 0 {
        if *value as ::core::ffi::c_int == '\\' as i32
            && *value.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        {
            value = value.offset(1 as ::core::ffi::c_int as isize);
        } else if stopchar_map[*value as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
            & 0x2 as ::core::ffi::c_int
            != 0
        {
            let fresh2 = p;
            p = p.offset(1 as ::core::ffi::c_int as isize);
            *fresh2 = 0;
            argc += 1;
            let ref mut fresh3 = *argv.offset(argc as isize);
            *fresh3 = p;
            loop {
                value = value.offset(1 as ::core::ffi::c_int as isize);
                if !(stopchar_map[*value as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
                    & 0x2 as ::core::ffi::c_int
                    != 0)
                {
                    break;
                }
            }
            continue;
        }
        let fresh4 = value;
        value = value.offset(1 as ::core::ffi::c_int as isize);
        let fresh5 = p;
        p = p.offset(1 as ::core::ffi::c_int as isize);
        *fresh5 = *fresh4;
    }
    *p = 0;
    argc += 1;
    let ref mut fresh6 = *argv.offset(argc as isize);
    *fresh6 = ::core::ptr::null::<::core::ffi::c_char>();
    '_c2rust_label: {
        if p < buf
            .offset(len as isize)
            .offset(2 as ::core::ffi::c_int as isize)
        {
        } else {
            __assert_fail(
                b"p < buf + len + 2\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
                3451 as ::core::ffi::c_uint,
                b"void decode_env_switches(const char *, size_t, enum variable_origin)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if *(*argv.offset(1 as ::core::ffi::c_int as isize)).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        != '-' as i32
        && strchr(*argv.offset(1 as ::core::ffi::c_int as isize), '=' as i32).is_null()
    {
        let ref mut fresh7 = *argv.offset(1 as ::core::ffi::c_int as isize);
        *fresh7 = buf;
    }
    decode_switches(argc, argv, origin);
    free(buf as *mut ::core::ffi::c_void);
    free(argv as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn quote_for_env(
    mut out: *mut ::core::ffi::c_char,
    mut in_0: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    while *in_0 as ::core::ffi::c_int != 0 {
        if *in_0 as ::core::ffi::c_int == '$' as i32 {
            let fresh29 = out;
            out = out.offset(1 as ::core::ffi::c_int as isize);
            *fresh29 = '$' as i32 as ::core::ffi::c_char;
        } else if stopchar_map[*in_0 as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
            & 0x2 as ::core::ffi::c_int
            != 0
            || *in_0 as ::core::ffi::c_int == '\\' as i32
        {
            let fresh30 = out;
            out = out.offset(1 as ::core::ffi::c_int as isize);
            *fresh30 = '\\' as i32 as ::core::ffi::c_char;
        }
        let fresh31 = in_0;
        in_0 = in_0.offset(1 as ::core::ffi::c_int as isize);
        let fresh32 = out;
        out = out.offset(1 as ::core::ffi::c_int as isize);
        *fresh32 = *fresh31;
    }
    out
}
#[no_mangle]
pub unsafe extern "C" fn disable_builtins() {
    if no_builtin_variables_flag != 0 {
        no_builtin_rules_flag = 1;
    }
    if no_builtin_rules_flag != 0 && old_builtin_rules_flag == 0 {
        old_builtin_rules_flag = 1;
        if !suffix_file.is_null() && (*suffix_file).builtin() as ::core::ffi::c_int != 0 {
            free_dep_chain((*suffix_file).deps);
            (*suffix_file).deps = ::core::ptr::null_mut::<dep>();
        }
        define_variable_in_set(
            b"SUFFIXES\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t).wrapping_sub(1),
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
    }
    if no_builtin_variables_flag != 0 && old_builtin_variables_flag == 0 {
        old_builtin_variables_flag = 1;
        undefine_default_variables();
    }
}
#[no_mangle]
pub unsafe extern "C" fn define_makeflags(makefile: ::core::ffi::c_int) -> *mut variable {
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
        if (*cs).toenv() as ::core::ffi::c_int != 0
            && (*cs).c <= CHAR_MAX
            && (makefile == 0 || (*cs).no_makefile() == 0)
            && ((*cs).type_0 as ::core::ffi::c_uint
                == flag as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cs).type_0 as ::core::ffi::c_uint
                    == flag_off as ::core::ffi::c_int as ::core::ffi::c_uint)
            && ((*((*cs).value_ptr as *mut ::core::ffi::c_int) == 0) as ::core::ffi::c_int
                == ((*cs).type_0 as ::core::ffi::c_uint
                    == flag_off as ::core::ffi::c_int as ::core::ffi::c_uint)
                    as ::core::ffi::c_int
                && ((*cs).default_value.is_null()
                    || (*cs).specified() as ::core::ffi::c_int != 0
                    || *((*cs).value_ptr as *mut ::core::ffi::c_int)
                        != *((*cs).default_value as *mut ::core::ffi::c_int)))
        {
            c[0 as ::core::ffi::c_int as usize] = (*cs).c as ::core::ffi::c_char;
            fp = variable_buffer_output(fp, &raw mut c as *mut ::core::ffi::c_char, 1);
        }
        cs = cs.offset(1 as ::core::ffi::c_int as isize);
    }
    memcpy(
        &raw mut c as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        b" --\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        3,
    );
    cs = &raw mut switches as *mut command_switch;
    while (*cs).c != 0 {
        if (*cs).toenv() as ::core::ffi::c_int != 0 && (makefile == 0 || (*cs).no_makefile() == 0) {
            match (*cs).type_0 as ::core::ffi::c_uint {
                7 => {}
                0 | 1 => {
                    if !((*cs).c <= CHAR_MAX)
                        && ((*((*cs).value_ptr as *mut ::core::ffi::c_int) == 0)
                            as ::core::ffi::c_int
                            == ((*cs).type_0 as ::core::ffi::c_uint
                                == flag_off as ::core::ffi::c_int as ::core::ffi::c_uint)
                                as ::core::ffi::c_int
                            && ((*cs).default_value.is_null()
                                || (*cs).specified() as ::core::ffi::c_int != 0
                                || *((*cs).value_ptr as *mut ::core::ffi::c_int)
                                    != *((*cs).default_value as *mut ::core::ffi::c_int)))
                    {
                        if (*cs).c <= CHAR_MAX {
                            c[2 as ::core::ffi::c_int as usize] = (*cs).c as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                        } else {
                            c[2 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
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
                        && *((*cs).value_ptr as *mut ::core::ffi::c_uint)
                            == *((*cs).default_value as *mut ::core::ffi::c_uint))
                    {
                        if (*cs).c <= CHAR_MAX {
                            c[2 as ::core::ffi::c_int as usize] = (*cs).c as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                        } else {
                            c[2 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
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
                            || *((*cs).value_ptr as *mut ::core::ffi::c_uint)
                                != *((*cs).noarg_value as *mut ::core::ffi::c_uint)
                        {
                            alloca_allocations.push(::std::vec::from_elem(
                                0,
                                30 as ::core::ffi::c_int as ::core::ffi::c_ulong as usize,
                            ));
                            let buf: *mut ::core::ffi::c_char =
                                alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                    as *mut ::core::ffi::c_char;
                            let buflen: ::core::ffi::c_int = sprintf(
                                buf,
                                b"%u\0" as *const u8 as *const ::core::ffi::c_char,
                                *((*cs).value_ptr as *mut ::core::ffi::c_uint),
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
                        && *((*cs).value_ptr as *mut ::core::ffi::c_double)
                            == *((*cs).default_value as *mut ::core::ffi::c_double))
                    {
                        if (*cs).c <= CHAR_MAX {
                            c[2 as ::core::ffi::c_int as usize] = (*cs).c as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                        } else {
                            c[2 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
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
                            || *((*cs).value_ptr as *mut ::core::ffi::c_double)
                                != *((*cs).noarg_value as *mut ::core::ffi::c_double)
                        {
                            alloca_allocations.push(::std::vec::from_elem(
                                0,
                                100 as ::core::ffi::c_int as ::core::ffi::c_ulong as usize,
                            ));
                            let buf_0: *mut ::core::ffi::c_char =
                                alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                    as *mut ::core::ffi::c_char;
                            let buflen_0: ::core::ffi::c_int = sprintf(
                                buf_0,
                                b"%g\0" as *const u8 as *const ::core::ffi::c_char,
                                *((*cs).value_ptr as *mut ::core::ffi::c_double),
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
                    let p: *mut ::core::ffi::c_char =
                        *((*cs).value_ptr as *mut *mut ::core::ffi::c_char);
                    if !p.is_null() {
                        if (*cs).c <= CHAR_MAX {
                            c[2 as ::core::ffi::c_int as usize] = (*cs).c as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                        } else {
                            c[2 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
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
                        let sl: *mut stringlist = *((*cs).value_ptr as *mut *mut stringlist);
                        if !sl.is_null() {
                            let mut i: ::core::ffi::c_uint;
                            i = 0;
                            while i < (*sl).idx {
                                if (*cs).c <= CHAR_MAX {
                                    c[2 as ::core::ffi::c_int as usize] =
                                        (*cs).c as ::core::ffi::c_char;
                                    fp = variable_buffer_output(
                                        fp,
                                        &raw mut c as *mut ::core::ffi::c_char,
                                        3,
                                    );
                                } else {
                                    c[2 as ::core::ffi::c_int as usize] =
                                        '-' as i32 as ::core::ffi::c_char;
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
                                fp = variable_buffer_output(
                                    fp,
                                    *(*sl).list.offset(i as isize),
                                    strlen(*(*sl).list.offset(i as isize)) as size_t,
                                );
                                i = i.wrapping_add(1);
                            }
                        }
                    }
                }
                _ => {
                    abort();
                }
            }
        }
        cs = cs.offset(1 as ::core::ffi::c_int as isize);
    }
    if fp == variable_buffer.offset(1 as ::core::ffi::c_int as isize) {
        fp = variable_buffer;
    }
    *fp = 0;
    define_variable_in_set(
        b"MFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1),
        variable_buffer.offset(
            (if *variable_buffer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '-' as i32
                && *variable_buffer.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ' ' as i32
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
    if !eval_strings.is_null() {
        fp = variable_buffer_output(
            fp,
            &raw const evalref as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 21]>() as size_t).wrapping_sub(1),
        );
    }
    let r: *const ::core::ffi::c_char = if posix_pedantic != 0 {
        &raw const posixref as *const ::core::ffi::c_char
    } else {
        &raw const ref_0 as *const ::core::ffi::c_char
    };
    let l: size_t = strlen(r) as size_t;
    v = lookup_variable(r, l);
    if !v.is_null()
        && !(*v).value.is_null()
        && *(*v).value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
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
    if *fp.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32 {
        fp = fp.offset(1 as ::core::ffi::c_int as isize);
    }
    v = define_variable_in_set(
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        fp,
        (if env_overrides != 0 {
            o_env_override as ::core::ffi::c_int
        } else {
            o_file as ::core::ffi::c_int
        }) as variable_origin,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    (*v).set_special(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    restore_variable_buffer(bufsave, lensave);
    v
}
#[no_mangle]
pub unsafe extern "C" fn should_print_dir() -> ::core::ffi::c_int {
    if print_directory_flag >= 0 {
        return print_directory_flag;
    }
    (silent_flag == 0 && (makelevel > 0 || !directories.is_null())) as ::core::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn print_version() {
    static mut printed_version: ::core::ffi::c_int = 0;
    let precede: *const ::core::ffi::c_char = if print_data_base_flag != 0 {
        b"# \0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"\0" as *const u8 as *const ::core::ffi::c_char
    };
    if printed_version != 0 {
        return;
    }
    printf(
        b"%sGNU Make %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        precede,
        version_string,
    );
    if remote_description.is_null() || *remote_description as ::core::ffi::c_int == 0 {
        printf(
            b"%sBuilt for %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            precede,
            make_host,
        );
    } else {
        printf(
            b"%sBuilt for %s (%s)\n\0" as *const u8 as *const ::core::ffi::c_char,
            precede,
            make_host,
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
    printed_version = 1;
}
#[no_mangle]
pub unsafe extern "C" fn print_data_base() {
    let mut resolution: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 43] = [0; 43];
    file_timestamp_sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        file_timestamp_now(&raw mut resolution),
    );
    print_version();
    printf(
        b"\n# Make data base, printed on %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut buf as *mut ::core::ffi::c_char,
    );
    print_variable_data_base();
    print_dir_data_base();
    print_rule_data_base();
    print_file_data_base();
    print_vpath_data_base();
    strcache_print_stats(b"#\0" as *const u8 as *const ::core::ffi::c_char);
    file_timestamp_sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        file_timestamp_now(&raw mut resolution),
    );
    printf(
        b"\n# Finished Make data base on %s\n\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut buf as *mut ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn clean_jobserver(status: ::core::ffi::c_int) {
    if jobserver_enabled() != 0 && jobserver_tokens != 0 {
        if status != 2 {
            error(
                ::core::ptr::null_mut::<Floc>(),
                INTSTR_LENGTH,
                b"INTERNAL: exiting with %u jobserver tokens (should be 0)!\0" as *const u8
                    as *const ::core::ffi::c_char,
                jobserver_tokens,
            );
        } else {
            loop {
                jobserver_tokens = jobserver_tokens.wrapping_sub(1);
                if !(jobserver_tokens != 0) {
                    break;
                }
                jobserver_release(0);
            }
        }
    }
    if master_job_slots != 0 {
        let tokens: ::core::ffi::c_uint =
            (1 as ::core::ffi::c_uint).wrapping_add(jobserver_acquire_all());
        if tokens != master_job_slots {
            error(
                ::core::ptr::null_mut::<Floc>(),
                INTSTR_LENGTH.wrapping_mul(2),
                b"INTERNAL: exiting with %u jobserver tokens available; should be %u!\0"
                    as *const u8 as *const ::core::ffi::c_char,
                tokens,
                master_job_slots,
            );
        }
    }
    reset_jobserver();
}
#[no_mangle]
pub unsafe extern "C" fn die(status: ::core::ffi::c_int) -> ! {
    static mut dying: ::core::ffi::c_char = 0;
    if dying == 0 {
        let err: ::core::ffi::c_int;
        dying = 1;
        if print_version_flag != 0 {
            print_version();
        }
        temp_stdin_unlink();
        err = (status != 0) as ::core::ffi::c_int;
        while job_slots_used > 0 {
            reap_children(1, err);
        }
        remote_cleanup();
        remove_intermediates(0);
        if print_data_base_flag != 0 {
            print_data_base();
        }
        if verify_flag != 0 {
            verify_file_data_base();
        }
        unload_all();
        clean_jobserver(status);
        if !output_context.is_null() {
            output_close(output_context);
            if output_context != &raw mut make_sync {
                output_close(&raw mut make_sync);
            }
            output_context = ::core::ptr::null_mut::<output>();
        }
        output_close(::core::ptr::null_mut::<output>());
        osync_clear();
        if !directory_before_chdir.is_null() {
            let mut _x: ::core::ffi::c_int = 0;
            _x = chdir(directory_before_chdir);
        }
    }
    exit(status);
}
pub const __CHAR_BIT__: ::core::ffi::c_int = 8;
pub const __SCHAR_MAX__: ::core::ffi::c_int = 127;
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
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
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
                value_ptr: &raw mut always_make_set as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut debug_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut env_overrides as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut eval_strings as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut print_usage_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut ignore_errors_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut keep_going_flag as *mut ::core::ffi::c_void,
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: &raw const default_keep_going_flag as *const ::core::ffi::c_void,
                long_name: b"keep-going\0" as *const u8 as *const ::core::ffi::c_char,
                origin: &raw mut keep_going_origin,
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
                value_ptr: &raw mut check_symlink_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut just_print_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut print_data_base_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut question_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut no_builtin_rules_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut no_builtin_variables_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut silent_flag as *mut ::core::ffi::c_void,
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: &raw const default_silent_flag as *const ::core::ffi::c_void,
                long_name: b"silent\0" as *const u8 as *const ::core::ffi::c_char,
                origin: &raw mut silent_origin,
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
                value_ptr: &raw mut keep_going_flag as *mut ::core::ffi::c_void,
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: &raw const default_keep_going_flag as *const ::core::ffi::c_void,
                long_name: b"no-keep-going\0" as *const u8 as *const ::core::ffi::c_char,
                origin: &raw mut keep_going_origin,
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
                value_ptr: &raw mut touch_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut print_version_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut print_directory_flag as *mut ::core::ffi::c_void,
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: &raw const default_print_directory_flag
                    as *const ::core::ffi::c_void,
                long_name: b"print-directory\0" as *const u8 as *const ::core::ffi::c_char,
                origin: &raw mut print_directory_origin,
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
                value_ptr: &raw mut directories as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut makefiles as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut include_dirs as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut arg_job_slots as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut max_load_average as *mut ::core::ffi::c_void,
                noarg_value: &raw mut default_load_average as *const ::core::ffi::c_void,
                default_value: &raw mut default_load_average as *const ::core::ffi::c_void,
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
                value_ptr: &raw mut old_files as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut output_sync_option as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut new_files as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut db_flags as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut jobserver_auth as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut trace_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut print_directory_flag as *mut ::core::ffi::c_void,
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: &raw const default_print_directory_flag
                    as *const ::core::ffi::c_void,
                long_name: b"no-print-directory\0" as *const u8 as *const ::core::ffi::c_char,
                origin: &raw mut print_directory_origin,
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
                value_ptr: &raw mut warn_undefined_variables_flag as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut sync_mutex as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut silent_flag as *mut ::core::ffi::c_void,
                noarg_value: ::core::ptr::null::<::core::ffi::c_void>(),
                default_value: &raw const default_silent_flag as *const ::core::ffi::c_void,
                long_name: b"no-silent\0" as *const u8 as *const ::core::ffi::c_char,
                origin: &raw mut silent_origin,
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
                value_ptr: &raw mut jobserver_auth as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut makefiles as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut shuffle_mode as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut jobserver_style as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut warn_flags as *mut ::core::ffi::c_void,
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
                value_ptr: &raw mut print_targets_flag as *mut ::core::ffi::c_void,
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
