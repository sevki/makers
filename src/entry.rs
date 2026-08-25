pub use crate::ffi_types::{
    __clock_t,
    __off64_t,
    __off_t,
    __pid_t,
    __sig_atomic_t,
    __uid_t,
    pid_t,
    sig_atomic_t,
    size_t,
    uintmax_t,
};
use {
    crate::{
        default::{
            define_default_variables,
            install_default_implicit_rules,
            install_default_suffix_rules,
            set_default_suffixes,
            undefine_default_variables,
        },
        dir::print_dir_data_base,
        file::{file, us_success, NameSeq, UpdateStatus, VariableSet, VariableSetList},
        floc::Floc,
        load::unload_all,
        misc::{get_tmpdir, get_tmpfile, make_toui, spin, xmalloc, xstrdup},
        read::construct_include_path,
        strcache::{strcache_add, strcache_init, strcache_print_stats},
        variable::print_variable_data_base,
        vpath::{build_vpath_lists, print_vpath_data_base},
    },
    libc::{
        self,
        __errno_location,
        _exit,
        atof,
        exit,
        free,
        isatty,
        putenv,
        setlocale,
        sprintf,
        stpcpy,
        strchr,
        strcmp,
        strerror,
        strrchr,
        tolower,
        ttyname,
    },
    std::sync::atomic::Ordering,
};

/// Differential-test oracle for the clap-based `decode_switches`: the
/// original `getopt_long`-based implementation, preserved verbatim per
/// AGENTS.md rule 3. Lives in its own file (not a nested `mod { .. }` block
/// here) so the `optarg`/`optind`/`opterr`/`getopt_long` externs it needs
/// never appear in this file's text at all -- this is the one piece of
/// process-wide libc state Phase A (#431/#439) couldn't move onto
/// `ExecContext`/`Options` outright, so instead it's fully retired from the
/// shipping binary and kept only as a `#[cfg(test)]`-gated correctness check.
#[cfg(test)]
#[path = "getopt_oracle_test.rs"]
mod getopt_oracle_test;

extern "C" {
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaddset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn sigaction(__sig: i32, __act: *const Sigaction, __oact: *mut Sigaction) -> i32;
    static mut environ: *mut *mut ::core::ffi::c_char;
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
}
pub type __uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigsetT {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = SigsetT;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
pub type __sigval_t = Sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SiginfoT {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: SiginfoFields,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SiginfoFields {
    pub _pad: [i32; 28],
    pub _kill: KillFields,
    pub _timer: TimerFields,
    pub _rt: RtFields,
    pub _sigchld: SigChldFields,
    pub _sigfault: SigFaultFields,
    pub _sigpoll: SigPollFields,
    pub _sigsys: SigSysFields,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigSysFields {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: i32,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigPollFields {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigFaultFields {
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_addr_lsb: ::core::ffi::c_short,
    pub _bounds: SigFaultBounds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SigFaultBounds {
    pub _addr_bnd: SigFaultAddrBounds,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigFaultAddrBounds {
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigChldFields {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: i32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RtFields {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TimerFields {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KillFields {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type SighandlerT = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sigaction {
    pub __sigaction_handler: SigactionHandler,
    pub sa_mask: SigsetT,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SigactionHandler {
    pub sa_handler: SighandlerT,
    pub sa_sigaction:
        Option<unsafe extern "C" fn(i32, *mut SiginfoT, *mut ::core::ffi::c_void) -> ()>,
}
pub type CTypeMask = ::core::ffi::c_uint;
pub const _ISalnum: CTypeMask = 8;
pub const _ISpunct: CTypeMask = 4;
pub const _IScntrl: CTypeMask = 2;
pub const _ISblank: CTypeMask = 1;
pub const _ISgraph: CTypeMask = 32768;
pub const _ISprint: CTypeMask = 16384;
pub const _ISspace: CTypeMask = 8192;
pub const _ISxdigit: CTypeMask = 4096;
pub const _ISdigit: CTypeMask = 2048;
pub const _ISalpha: CTypeMask = 1024;
pub const _ISlower: CTypeMask = 512;
pub const _ISupper: CTypeMask = 256;
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
pub type HashTable = crate::hash::HashTable;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
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
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct CommandSwitch {
    pub c: i32,
    pub type_0: OptionArgKind,
    pub value_ptr: *mut ::core::ffi::c_void,
    pub(crate) env: ::core::ffi::c_uint,
    pub(crate) toenv: ::core::ffi::c_uint,
    pub(crate) no_makefile: ::core::ffi::c_uint,
    pub(crate) specified: ::core::ffi::c_uint,
    pub noarg_value: *const ::core::ffi::c_void,
    pub default_value: *const ::core::ffi::c_void,
    pub long_name: *const ::core::ffi::c_char,
    pub origin: *mut variable_origin,
}

impl CommandSwitch {
    pub fn env(&self) -> ::core::ffi::c_uint {
        self.env
    }
    pub fn set_env(&mut self, val: ::core::ffi::c_uint) {
        self.env = val;
    }
    pub fn toenv(&self) -> ::core::ffi::c_uint {
        self.toenv
    }
    pub fn set_toenv(&mut self, val: ::core::ffi::c_uint) {
        self.toenv = val;
    }
    pub fn no_makefile(&self) -> ::core::ffi::c_uint {
        self.no_makefile
    }
    pub fn set_no_makefile(&mut self, val: ::core::ffi::c_uint) {
        self.no_makefile = val;
    }
    pub fn specified(&self) -> ::core::ffi::c_uint {
        self.specified
    }
    pub fn set_specified(&mut self, val: ::core::ffi::c_uint) {
        self.specified = val;
    }
}
pub type OptionArgKind = ::core::ffi::c_uint;
pub const ignore: OptionArgKind = 7;
pub const floating: OptionArgKind = 6;
pub const positive_int: OptionArgKind = 5;
pub const filename: OptionArgKind = 4;
pub const strlist: OptionArgKind = 3;
pub const string: OptionArgKind = 2;
pub const flag_off: OptionArgKind = 1;
pub const flag: OptionArgKind = 0;
use crate::{
    commands::{fatal_error_signal, handling_fatal_signal},
    expand::{
        expand_string_buf,
        expand_variable_buf,
        initialize_variable_output,
        install_variable_buffer,
        restore_variable_buffer,
        variable_buffer_output,
    },
    file::{
        enter_file,
        file_timestamp_now,
        file_timestamp_string,
        lookup_file,
        print_file_data_base,
        print_targets,
        remove_intermediates,
        snap_deps,
        verify_file_data_base,
    },
    function::hash_init_function_table,
    job::{child_handler, exec_command, job_slots_used, jobserver_tokens, reap_children},
    load::load_file,
    misc::{concat, cstr_bytes_or_empty},
    output::{
        error,
        fatal_err,
        output_context,
        perror_with_name,
        pfatal_with_name_err,
        set_output_context,
        set_stdio_traced,
        stdio_traced,
        FmtArg,
    },
    posixos::{
        check_io_state,
        jobserver_acquire_all,
        jobserver_clear,
        jobserver_enabled,
        jobserver_get_auth,
        jobserver_parse_auth,
        jobserver_post_child,
        jobserver_pre_child,
        jobserver_release,
        jobserver_setup,
        osync_clear,
        osync_get_mutex,
        osync_parse_mutex,
        osync_setup,
    },
    read::{eval_buffer, parse_file_seq, read_all_makefiles, tilde_expand},
    remake::{f_mtime, update_goal_chain},
    rule::{convert_to_pattern, print_rule_data_base, snap_implicit_rules},
    variable::{
        define_automatic_variables,
        define_variable_in_set,
        init_hash_global_variable_set,
        lookup_variable,
        reset_env_override,
        try_variable_definition,
    },
};
pub use crate::{file::nameseq, output::output, read::goaldep};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandVariable {
    pub next: *mut CommandVariable,
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
pub const SIG_DFL: SighandlerT = None;
pub const ENOENT: i32 = 2;
pub const EINTR: i32 = 4;
pub const SIGCHLD: i32 = 17;
pub const SIGUSR1: i32 = 10;
pub const SA_RESTART: i32 = 0x10000000_i32;
pub const SIG_SETMASK: i32 = 2;
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
/// Read the debug-level bitmask (`-d`/`--debug`, `DB_*` constants) off
/// [`crate::execctx::ExecContext::db_level`], the former `static mut`
/// (later a process-global atomic) this replaced.
#[inline]
pub fn db_level(ctx: &crate::execctx::ExecContext) -> i32 {
    ctx.db_level.get()
}

/// Overwrite the debug-level bitmask.
#[inline]
pub fn set_db_level(ctx: &crate::execctx::ExecContext, level: i32) {
    ctx.db_level.set(level);
}

#[cfg(test)]
mod db_level_tests {
    use {
        super::{
            db_level,
            set_db_level,
            DB_ALL,
            DB_BASIC,
            DB_IMPLICIT,
            DB_JOBS,
            DB_NONE,
            DB_PRINT,
            DB_WHY,
        },
        crate::execctx::ExecContext,
    };

    /// Exercises the accessors: a plain store/load round-trip plus the
    /// `db_level |= ...` read-modify-write that `decode_debug_flags`
    /// performs. `db_level` lives on `ExecContext` now (no longer a shared
    /// process-global), so this just builds its own isolated context.
    #[test]
    fn accessors_round_trip_and_accumulate() {
        let ctx = ExecContext::default();
        // store then load round-trips.
        set_db_level(&ctx, DB_NONE);
        assert_eq!(db_level(&ctx), DB_NONE);
        set_db_level(&ctx, DB_ALL);
        assert_eq!(db_level(&ctx), DB_ALL);

        // `|= ...` bit accumulation, as in the `-d` flag decoder.
        set_db_level(&ctx, DB_NONE);
        set_db_level(&ctx, db_level(&ctx) | DB_BASIC);
        set_db_level(&ctx, db_level(&ctx) | DB_JOBS);
        set_db_level(&ctx, db_level(&ctx) | DB_IMPLICIT);
        assert_eq!(db_level(&ctx), DB_BASIC | DB_JOBS | DB_IMPLICIT);

        // The `n` flag resets to zero mid-stream.
        set_db_level(&ctx, 0);
        set_db_level(&ctx, db_level(&ctx) | DB_PRINT | DB_WHY);
        assert_eq!(db_level(&ctx), DB_PRINT | DB_WHY);
        set_db_level(&ctx, DB_NONE);
    }
}
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
/// exit. The former `MASTER_JOB_SLOTS` global atomic, now read directly off `ctx.options`.
fn master_job_slots(ctx: &crate::execctx::ExecContext) -> ::core::ffi::c_uint {
    with_options(ctx, |o| o.master_job_slots.get())
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
#[derive(Debug, Clone)]
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
    /// Index into [`makefiles`](Self::makefiles) of the makefile read from
    /// standard input (`-f -`), or `-1` when none — the former
    /// `static mut stdin_offset`. It indexes the owned `makefiles` list, so it
    /// belongs here on the run-owner `Options` rather than shadowing that list
    /// in a global; readers without an `&Options` (e.g. `temp_stdin_unlink` on
    /// the deep cleanup path) reach it through the `with_options` channel
    /// ([`opt_stdin_offset`]). Kept as the C `i32` with its `-1` sentinel so
    /// every index/compare stays byte-identical.
    pub stdin_offset: ::core::cell::Cell<i32>,
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
    /// borrow channel by `enter_file` and `die_cleanup`, which carry no
    /// `&Options`.
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
    /// via `ctx.options` (through `master_job_slots()`), beside its
    /// `job_slots` companion field.
    pub master_job_slots: ::core::cell::Cell<::core::ffi::c_uint>,
    /// Monotonic command-generation counter for this run, the former `static
    /// mut command_count`. Bumped once per shell command run (`reap_children`,
    /// `$(shell)`, `$(file)`) via [`bump_command_count`], and read by the
    /// directory cache (`find_directory`) and the `update_goal_chain` loop
    /// through [`opt_command_count`] to invalidate stat/contents entries
    /// recorded before the latest command. Reached via `ctx.options`
    /// (through [`opt_command_count`]).
    pub command_count: ::core::cell::Cell<::core::ffi::c_ulong>,
    /// `snap_deps`-complete latch for this run, the former `file::SNAPPED_DEPS`
    /// global. Set once at the end of `snap_deps` via [`mark_snapped_deps`] and
    /// read by `record_files` through [`opt_snapped_deps`] to reject
    /// prerequisites defined from within a recipe (i.e. after the snapshot).
    /// Lives on `ctx.options`, reached via `with_options`.
    pub snapped_deps: ::core::cell::Cell<bool>,
    /// `true` only while `main_0` is remaking the makefiles themselves (the
    /// makefile-remaking `update_goal_chain` pass), so the remake logic can
    /// treat makefile targets specially. Toggled around that pass in `main_0`
    /// and read across the update walk (`update_goal_chain` / `update_file_1` /
    /// `remake_file`, via [`opt_rebuilding_makefiles`]) and by `reset_makeflags`.
    /// Lives on `ctx.options`, reached via `with_options`.
    pub rebuilding_makefiles: ::core::cell::Cell<bool>,
    /// The special-target feature latches, each set once when make sees the
    /// matching `.`-target and read widely thereafter — the former
    /// `POSIX_PEDANTIC` / `SECOND_EXPANSION` / `ONE_SHELL` / `NOT_PARALLEL`
    /// global atomics. Reached via `ctx.options` (through `posix_pedantic()` /
    /// `second_expansion()` / `one_shell()` / `not_parallel()` and their
    /// `set_*` setters).
    pub posix_pedantic: ::core::cell::Cell<bool>,
    pub second_expansion: ::core::cell::Cell<bool>,
    pub one_shell: ::core::cell::Cell<bool>,
    pub not_parallel: ::core::cell::Cell<bool>,
    /// One-shot latch set once make has logged the working-directory "Entering
    /// directory" trace (so the matching "Leaving directory" is emitted and
    /// `MAKE_RESTARTS` is prefixed with `-`) — the former `STDIO_TRACED` global
    /// atomic. Reached via `ctx.options` (through `crate::output::stdio_traced()` /
    /// `set_stdio_traced()`).
    pub stdio_traced: ::core::cell::Cell<bool>,
    /// The command-line goal targets, in order — the former `static mut goals`
    /// (itself the pointer-free replacement for the c2rust `*mut GoalDep
    /// goals`/`lastgoal` chain). Owned `GoalDepNode`s; the target file is
    /// `dep.file: Option<FileId>`. Lives on `Options` because goals *are*
    /// command-line state: `decode_switches`/`handle_non_switch_argument`
    /// populate it during argument decoding (before the build-phase
    /// `ExecContext` rebuild, which would otherwise wipe a context-owned
    /// list), and `main_0` consumes it around `update_goal_chain`.
    pub goals: ::core::cell::RefCell<Vec<crate::dep::GoalDepNode>>,
    /// The command-line switch table — the former process-global `switches`
    /// populated by the `.init_array` startup hook. Owned here because it is
    /// mutable per-run state: `decode_switches` sets each entry's `specified`
    /// bit as arguments are decoded, and `define_makeflags` reads those bits
    /// back when rebuilding MAKEFLAGS.
    pub switches: ::core::cell::RefCell<[CommandSwitch; 42]>,
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
            stdin_offset: ::core::cell::Cell::new(-1),
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
            goals: ::core::cell::RefCell::new(Vec::new()),
            switches: ::core::cell::RefCell::new(switches_template()),
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

/// Run `f` with a borrow of `ctx`'s option/flag state (`ctx.options`). Used
/// to keep `with_options(ctx, |o| ...)` calls reading the same shape they
/// did through the former `OPTIONS_PTR` thread-local, now that `Options`
/// lives directly on `ExecContext` instead of behind a global borrow
/// channel — every caller already had `ctx` in scope (see #532).
pub fn with_options<R>(ctx: &crate::execctx::ExecContext, f: impl FnOnce(&Options) -> R) -> R {
    f(&ctx.options)
}

thread_local! {
    /// Borrow channel to the `ExecContext` owned as a local in `main_0`. It
    /// exists solely so the glob `gl_opendir` callback
    /// (`dir::open_dirstream`), which is invoked through C-ABI glob machinery and
    /// cannot take an `&ExecContext`, can reach the per-run directory cache that
    /// now lives on the context. A *pointer*, not the data: the `ExecContext`
    /// still lives in `main_0`'s `let mut ctx` slot (a stable address even across
    /// the build-phase rebuild), and is installed for the dynamic extent of
    /// `main_0`.
    ///
    /// Phase A disposition, corrected: NOT an accepted permanent seam either.
    /// It currently exists because `open_dirstream` is handed to libc's
    /// `glob()` as a raw `extern "C" fn` callback pointer, which can't carry
    /// an `&ExecContext` — but the fix is to stop calling libc `glob()`, not
    /// to keep a thread-local around it. Project policy: no globals, thread-
    /// local or otherwise, as a permanent design choice. Tracked for removal
    /// by replacing the libc `glob()` dependency with a native Rust
    /// directory-walk + pattern-match implementation, at which point
    /// `open_dirstream` becomes a plain function taking `&ExecContext`
    /// directly and this channel disappears. See #431/#530, #533.
    static CTX_PTR: ::core::cell::Cell<*const crate::execctx::ExecContext> =
        const { ::core::cell::Cell::new(::core::ptr::null()) };
}

/// Borrow the installed `main_0` `ExecContext`. Only valid while `main_0` is on
/// the stack (its referent outlives every makefile-time / build-time callback).
unsafe fn installed_exec_context<'a>() -> &'a crate::execctx::ExecContext {
    let p = CTX_PTR.with(|c| c.get());
    debug_assert!(
        !p.is_null(),
        "installed_exec_context called with no ExecContext on the stack"
    );
    &*p
}

/// Run `f` with a borrow of `main_0`'s owned `ExecContext`, reached through the
/// `CTX_PTR` borrow channel. Used by the glob `gl_opendir` callback, which the C
/// glob machinery invokes without an `&ExecContext` parameter. `CTX_PTR` is
/// installed at the start of `main_0`, before any code that could glob runs.
pub fn with_exec_context<R>(f: impl FnOnce(&crate::execctx::ExecContext) -> R) -> R {
    f(unsafe { installed_exec_context() })
}

/// Like [`with_exec_context`], but returns `None` when no context is installed
/// instead of dereferencing a null channel. For callers that can legitimately
/// run outside `main_0`'s dynamic extent: allocation failure before startup
/// finishes, or bare unit tests.
pub fn try_with_exec_context<R>(f: impl FnOnce(&crate::execctx::ExecContext) -> R) -> Option<R> {
    let p = CTX_PTR.with(|c| c.get());
    if p.is_null() {
        None
    } else {
        // SAFETY: a non-null `CTX_PTR` always points at `main_0`'s live
        // context (installed for its dynamic extent).
        Some(f(unsafe { &*p }))
    }
}

/// Test-only: install a leaked default `ExecContext` on the current thread's
/// `CTX_PTR` borrow channel so the glob callback path can run inside
/// `#[cfg(test)]` unit tests below `main_0`, and return a reference to it for
/// tests that also need an `&ExecContext` to pass directly (e.g. to
/// `with_options`). The context is leaked so the pointer stays valid for the
/// thread's lifetime; test builds only. Idempotent: a second call on the same
/// thread returns the already-installed context rather than leaking another.
#[cfg(test)]
pub fn install_default_exec_context_for_test() -> &'static crate::execctx::ExecContext {
    CTX_PTR.with(|p| {
        if p.get().is_null() {
            let leaked: &'static crate::execctx::ExecContext =
                Box::leak(Box::new(crate::execctx::ExecContext::default()));
            p.set(leaked as *const crate::execctx::ExecContext);
            leaked
        } else {
            // SAFETY: a non-null `CTX_PTR` on this thread always points at a
            // leaked `'static` context installed by this same function (test
            // builds only ever install through here).
            unsafe { &*p.get() }
        }
    })
}

pub fn env_overrides(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.env_overrides.get())
}
pub fn opt_question(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.question.get())
}
pub fn opt_touch(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.touch.get())
}
pub fn opt_just_print(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.just_print.get())
}
/// Effective recipe-echo suppression (the former `run_silent` global), read
/// through the `with_options` borrow channel by the deep recipe-echo /
/// `touch` / `rm` paths in `job`/`remake`/`file` that carry no `&Options`.
pub fn opt_run_silent(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.run_silent.get())
}
/// `Options::stdin_offset` read through the `with_options` borrow channel by
/// `temp_stdin_unlink`, which runs from the deep cleanup path with no
/// `&Options`.
pub fn opt_stdin_offset(ctx: &crate::execctx::ExecContext) -> i32 {
    with_options(ctx, |o| o.stdin_offset.get())
}
/// Export-everything latch (the former `export_all_variables` global), read
/// through the `with_options` borrow channel by `should_export`, which carries
/// no `&Options`.
pub fn opt_export_all_variables(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.export_all_variables.get())
}
/// The recipe-introducing prefix character (the former `cmd_prefix` global),
/// read through the `with_options` borrow channel by the makefile reader and
/// the database printers, which carry no `&Options`.
pub fn opt_cmd_prefix(ctx: &crate::execctx::ExecContext) -> ::core::ffi::c_char {
    with_options(ctx, |o| o.cmd_prefix.get())
}
/// Resolved output-sync mode (the former `output_sync` global), read through
/// the `with_options` borrow channel by the `syncing` computation and the
/// `output`/`job` dump paths, which carry no `&Options`.
pub fn opt_output_sync(ctx: &crate::execctx::ExecContext) -> i32 {
    with_options(ctx, |o| o.output_sync.get())
}
/// Resolved parallel job-slot count (the former `job_slots` global), read
/// through the `with_options` borrow channel by the job scheduler, which
/// carries no `&Options`.
pub fn opt_job_slots(ctx: &crate::execctx::ExecContext) -> ::core::ffi::c_uint {
    with_options(ctx, |o| o.job_slots.get())
}
/// Monotonic command-generation counter (the former `command_count` global),
/// read through the `with_options` borrow channel by the directory cache
/// (`find_directory`) and the `update_goal_chain` loop, which carry no
/// `&Options`.
pub fn opt_command_count(ctx: &crate::execctx::ExecContext) -> ::core::ffi::c_ulong {
    with_options(ctx, |o| o.command_count.get())
}
/// Bump the command-generation counter, once per shell command run
/// (`reap_children`, `$(shell)`, `$(file)`). Goes through the `with_options`
/// channel so it always reaches `main_0`'s real `Options`, even on the
/// `gmk_eval` throwaway-context path the `$(shell)`/`$(file)` writers take.
pub fn bump_command_count(ctx: &crate::execctx::ExecContext) {
    with_options(ctx, |o| {
        o.command_count.set(o.command_count.get().wrapping_add(1))
    });
}
/// Whether `snap_deps` has run for this make (the former `file::SNAPPED_DEPS`
/// global), read through the `with_options` channel by `record_files` — which
/// is reachable from the `gmk_eval` throwaway-context path and so cannot rely
/// on its `&ExecContext` being `main_0`'s real run context.
pub fn opt_snapped_deps(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.snapped_deps.get())
}
/// Mark the dependency snapshot complete, once, at the end of `snap_deps`. Goes
/// through the `with_options` channel so it always sets `main_0`'s real
/// `Options`.
pub fn mark_snapped_deps(ctx: &crate::execctx::ExecContext) {
    with_options(ctx, |o| o.snapped_deps.set(true));
}
/// Whether `main_0` is currently remaking the makefiles themselves (the former
/// `REBUILDING_MAKEFILES` global), read through the `with_options` channel by
/// the update walk and by `reset_makeflags` — the latter reached from
/// `set_special_var` on the `gmk_eval` throwaway path, so it must resolve to
/// `main_0`'s real run state rather than a throwaway context.
pub fn opt_rebuilding_makefiles(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.rebuilding_makefiles.get())
}
pub fn opt_ignore_errors(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.ignore_errors.get())
}
pub fn opt_keep_going(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.keep_going.get())
}
pub fn opt_check_symlink(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.check_symlink.get())
}
pub fn opt_no_builtin_rules(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.no_builtin_rules.get())
}
pub fn opt_print_data_base(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.print_data_base.get())
}
pub fn opt_print_version(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.print_version.get())
}
pub fn opt_jobserver_auth_present(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.jobserver_auth.borrow().is_some())
}
pub fn opt_max_load_average(ctx: &crate::execctx::ExecContext) -> f64 {
    with_options(ctx, |o| o.max_load_average.get())
}

/// `should_print_dir` for callers outside the `Options` borrow chain
/// (`output.rs`), reading the owned `Options` through the borrow channel.
/// Delegates to [`should_print_dir`] — a hand-copied version of the logic
/// here once dropped the `-C` clause and silently lost the top-level
/// `Entering directory` lines (#456), so keep this a pure delegation.
pub fn should_print_dir_mirror(ctx: &crate::execctx::ExecContext) -> i32 {
    with_options(ctx, |o| should_print_dir(ctx, o) as i32)
}

pub fn set_touch_mirror(ctx: &crate::execctx::ExecContext, v: bool) {
    with_options(ctx, |o| o.touch.set(v));
}
pub fn set_question_mirror(ctx: &crate::execctx::ExecContext, v: bool) {
    with_options(ctx, |o| o.question.set(v));
}
pub fn set_just_print_mirror(ctx: &crate::execctx::ExecContext, v: bool) {
    with_options(ctx, |o| o.just_print.set(v));
}
pub fn set_ignore_errors_mirror(ctx: &crate::execctx::ExecContext, v: bool) {
    with_options(ctx, |o| o.ignore_errors.set(v));
}
/// Strcache'd name of the temporary file holding the makefile read from stdin
/// (or null), paired with `Options::stdin_offset`. Lets `temp_stdin_unlink`
/// run from the deep cleanup path without an `&Options` borrow; the pointer is
/// into the strcache, which lives for the whole run.
pub const TEMP_STDIN_OPT: i32 = CHAR_MAX + 10;
pub const WARN_OPT: i32 = CHAR_MAX + 13;
const LONG_OPTION_ALIASES: [option; 9] = [
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
/// The display name of a goal: its `dep.name` if set, else the name of its
/// target file (raw bytes).
fn goal_name_bytes(ctx: &crate::execctx::ExecContext, g: &crate::dep::GoalDepNode) -> Vec<u8> {
    if !g.dep.name.is_empty() {
        return g.dep.name.clone().into_bytes();
    }
    if let Some(fid) = g.dep.file {
        if let Some(node) = ctx.filenodes.get(fid) {
            return node.lock().expect("file node poisoned").name.clone();
        }
    }
    Vec::new()
}

/// Materialize a goal's source location as an owned `Floc` whose `filenm` lives
/// for the returned value's lifetime (the bytes are stored alongside it).
fn goal_floc(g: &crate::dep::GoalDepNode) -> Option<GoalFloc> {
    g.defined_in.as_ref().map(|f| {
        let mut bytes = f.clone();
        bytes.push(0);
        GoalFloc {
            floc: Floc {
                filenm: bytes.as_ptr() as *const ::core::ffi::c_char,
                lineno: g.lineno,
                offset: g.offset,
            },
            _bytes: bytes,
        }
    })
}

/// Owns the NUL-terminated `filenm` bytes referenced by `floc.filenm`. Deref
/// to `Floc` for the c2rust APIs that still take a `*const Floc`.
struct GoalFloc {
    floc: Floc,
    _bytes: Vec<u8>,
}

impl ::core::ops::Deref for GoalFloc {
    type Target = Floc;
    fn deref(&self) -> &Floc {
        &self.floc
    }
}

/// The synthetic "new file" mtime used for `-W`/`--what-if` targets: the
/// largest representable packed timestamp (`NEW_MTIME` in the C code).
fn new_file_mtime() -> uintmax_t {
    (!(0_i32 as uintmax_t)).wrapping_sub(if !(-1_i32 as uintmax_t <= 0 as uintmax_t) {
        0_i32 as uintmax_t
    } else {
        !(0_i32 as uintmax_t)
            << (::core::mem::size_of::<uintmax_t>() as usize)
                .wrapping_mul(CHAR_BIT as usize)
                .wrapping_sub(1_usize)
    })
}

/// Build a goal that targets `file` (no source location / flags).
fn goaldep_for_file(file: crate::file::FileId) -> crate::dep::GoalDepNode {
    crate::dep::GoalDepNode {
        dep: crate::dep::DepNode {
            name: String::new(),
            file: Some(file),
            shuf: None,
            stem: None,
            flags: crate::dep::DepFlags::empty(),
            changed: false,
            ignore_mtime: false,
            static_pattern: false,
            needs_second_expansion: false,
            ignore_automatic_vars: false,
            is_explicit: false,
            wait_here: false,
        },
        error: 0,
        defined_in: None,
        lineno: 0,
        offset: 0,
    }
}
// The four special-target feature latches — `.POSIX`, `.SECONDEXPANSION`,
// `.ONESHELL`, `.NOTPARALLEL` — each set once when make sees the corresponding
// special target and read widely thereafter. They live on `ctx.options` (the
// former `POSIX_PEDANTIC` / `SECOND_EXPANSION` / `ONE_SHELL` / `NOT_PARALLEL`
// global atomics), reached through `with_options`: the setters run in
// `check_specials` / `snap_deps`, both resolving to `main_0`'s
// real run state, not a throwaway.

/// Whether `.POSIX` pedantic mode is in effect.
pub fn posix_pedantic(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.posix_pedantic.get())
}
/// Record that the `.POSIX` special target has been seen.
pub fn set_posix_pedantic(ctx: &crate::execctx::ExecContext) {
    with_options(ctx, |o| o.posix_pedantic.set(true));
}
/// Whether `.SECONDEXPANSION` is in effect.
pub fn second_expansion(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.second_expansion.get())
}
/// Record that the `.SECONDEXPANSION` special target has been seen.
pub fn set_second_expansion(ctx: &crate::execctx::ExecContext) {
    with_options(ctx, |o| o.second_expansion.set(true));
}
/// Whether `.ONESHELL` is in effect (each recipe runs in a single shell).
pub fn one_shell(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.one_shell.get())
}
/// Record that the `.ONESHELL` special target has been seen.
pub fn set_one_shell(ctx: &crate::execctx::ExecContext) {
    with_options(ctx, |o| o.one_shell.set(true));
}
/// Whether make is running non-parallel (one job at a time).
pub fn not_parallel(ctx: &crate::execctx::ExecContext) -> bool {
    with_options(ctx, |o| o.not_parallel.get())
}
/// Record that the `.NOTPARALLEL` special target has been seen.
pub fn set_not_parallel(ctx: &crate::execctx::ExecContext) {
    with_options(ctx, |o| o.not_parallel.set(true));
}
/// Per-byte classification bitmap (`MAP_*` flags), computed once at startup by
/// [`initialize_stopchar_map`]. Held behind a `OnceLock` so it is a safe
/// `static`; reads before initialization see a zeroed map, matching the C
/// `static`'s zero-initialized state.
///
/// Deliberately stays a process-global rather than an `ExecContext` field
/// (an accepted Phase A exception, alongside `output::STDOUT_ERRNO`): its
/// contents are a pure, session-independent function of byte value — the
/// same classification table for every `make` run there could ever be, with
/// no per-session, per-thread, or per-build-phase variation possible. Moving
/// it onto `ExecContext` would force every one of its ~100+ call sites
/// (file.rs, read.rs, parser.rs, job.rs, function.rs, ...) to thread a `ctx`
/// reference through purely to reach an unchanging lookup table, for no
/// behavioral benefit — a multi-tenant host sharing one process across
/// sessions gets an identical table either way.
static STOPCHAR_MAP: ::std::sync::OnceLock<[::core::ffi::c_ushort; 256]> =
    ::std::sync::OnceLock::new();
/// Borrow the classification map. Returns a zeroed map until
/// [`initialize_stopchar_map`] has run.
pub fn stopchar_map() -> &'static [::core::ffi::c_ushort; 256] {
    static ZERO: [::core::ffi::c_ushort; 256] = [0; 256];
    STOPCHAR_MAP.get().unwrap_or(&ZERO)
}
// The run's own output-sync record (former `static mut make_sync`) now lives
// on the owned per-run context: `ctx.make_sync` (see
// `crate::execctx::MakeSync`), Boxed so its address survives the build-phase
// context rebuild for the `output_context` identity uses below.
fn make_sync_syncout(ctx: &crate::execctx::ExecContext) -> ::core::ffi::c_uint {
    ctx.make_sync.0.get().syncout()
}

fn set_make_sync_syncout(ctx: &crate::execctx::ExecContext, value: ::core::ffi::c_uint) {
    let mut ms = ctx.make_sync.0.get();
    ms.set_syncout(value & 1);
    ctx.make_sync.0.set(ms);
}
unsafe extern "C" fn bsd_signal(sig: i32, func: bsd_signal_ret_t) -> bsd_signal_ret_t {
    let mut act: Sigaction = Sigaction {
        __sigaction_handler: SigactionHandler { sa_handler: None },
        sa_mask: SigsetT { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut oact: Sigaction = Sigaction {
        __sigaction_handler: SigactionHandler { sa_handler: None },
        sa_mask: SigsetT { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    act.__sigaction_handler.sa_handler = func as SighandlerT;
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

unsafe fn install_fatal_signal(ctx: &crate::execctx::ExecContext, sig: i32) {
    let old_handler = bsd_signal(
        sig,
        Some(fatal_error_signal as unsafe extern "C" fn(i32) -> ()),
    );
    if signal_handler_addr(old_handler) == 1 {
        bsd_signal(sig, sig_ign_handler());
    } else {
        let mut set = ctx.fatal_signal_set.0.get();
        sigaddset(&raw mut set, sig);
        ctx.fatal_signal_set.0.set(set);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn initialize_global_hash_tables(ctx: &crate::execctx::ExecContext) {
    init_hash_global_variable_set(ctx);
    strcache_init();
    // The file table now lives on `ExecContext` (`ctx.files`); no global init.
    hash_init_function_table(ctx);
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
    // The libc pattern was sticky-`ferror` + `fclose`; every make writer now
    // goes through Rust stdout, whose write failures land in the sticky
    // errno (`output::record_stdout_error`). The final flush here plays
    // fclose's part: an error first discovered by it gets the strerror form,
    // one already recorded during the run the plain form.
    use std::io::Write;
    let prev_fail: i32 = crate::output::stdout_error();
    let flush_err = std::io::stdout().flush().err();
    if prev_fail != 0 || flush_err.is_some() {
        // This is the `atexit`-registered handler: it cannot be passed the
        // owned `ExecContext` and there is deliberately no global to read it
        // from, so it must not route through a `ctx`-taking printer. Write the
        // bare diagnostic (no `make[N]:` prefix) straight to stderr.
        let msg = if let Some(e) = flush_err {
            let err = libc::strerror(e.raw_os_error().unwrap_or(libc::EIO));
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
) -> Result<*const ::core::ffi::c_char, crate::build_result::BuildError> {
    let mut expanded: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *name.offset(0_i32 as isize) as i32 == 0 {
        return Err(fatal_err(
            ctx,
            ::core::ptr::null_mut::<Floc>(),
            0,
            b"empty string invalid as file name\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        ));
    }
    if *name.offset(0_i32 as isize) as i32 == '~' as i32 {
        expanded = tilde_expand(ctx, name)?;
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
    let cp = strcache_add(ctx, name);
    free(expanded as *mut ::core::ffi::c_void);
    Ok(cp)
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
        let g = |i: usize| -> u8 {
            if i < name.len() {
                name[i]
            } else {
                0
            }
        };
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
    // Real kernel-invoked signal handler: it cannot take an `&ExecContext`
    // parameter, so it reaches `main_0`'s live context through the `CTX_PTR`
    // borrow channel, like `fatal_error_signal`'s cleanup helpers.
    with_exec_context(|ctx| {
        set_db_level(
            ctx,
            if db_level(ctx) != 0 {
                DB_NONE
            } else {
                DB_BASIC
            },
        );
    });
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn decode_debug_flags(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
) -> Result<(), crate::build_result::BuildError> {
    if options.debug_flag.get() {
        set_db_level(ctx, DB_ALL);
    }
    if options.trace.get() {
        set_db_level(ctx, db_level(ctx) | DB_PRINT | DB_WHY);
    }
    {
        let db_flags = options.db_flags.borrow();
        for entry in db_flags.iter() {
            let mut p: *const ::core::ffi::c_char = entry.as_ptr();
            loop {
                match tolower(*p.offset(0_i32 as isize) as i32) {
                    97 => {
                        set_db_level(ctx, db_level(ctx) | DB_ALL);
                    }
                    98 => {
                        set_db_level(ctx, db_level(ctx) | DB_BASIC);
                    }
                    105 => {
                        set_db_level(ctx, db_level(ctx) | DB_BASIC | DB_IMPLICIT);
                    }
                    106 => {
                        set_db_level(ctx, db_level(ctx) | DB_JOBS);
                    }
                    109 => {
                        set_db_level(ctx, db_level(ctx) | DB_BASIC | DB_MAKEFILES);
                    }
                    110 => {
                        set_db_level(ctx, 0);
                    }
                    112 => {
                        set_db_level(ctx, db_level(ctx) | DB_PRINT);
                    }
                    118 => {
                        set_db_level(ctx, db_level(ctx) | DB_BASIC | DB_VERBOSE);
                    }
                    119 => {
                        set_db_level(ctx, db_level(ctx) | DB_WHY);
                    }
                    _ => {
                        return Err(fatal_err(
                            ctx,
                            ::core::ptr::null_mut::<Floc>(),
                            strlen(p) as size_t,
                            b"unknown debug level specification '%s'\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[FmtArg::Str((p) as *const ::core::ffi::c_char)],
                        ));
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
    if db_level(ctx) != 0 {
        options.verify.set(true);
    }
    if db_level(ctx) == 0 {
        options.debug_flag.set(false);
    }
    Ok(())
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
pub unsafe fn decode_output_sync_flags(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
) -> Result<(), crate::build_result::BuildError> {
    if let Some(opt) = options.output_sync_option.borrow().as_ref() {
        match classify_output_sync(opt.as_bytes()) {
            Some(mode) => options.output_sync.set(mode),
            None => {
                let c = ::std::ffi::CString::new(opt.as_bytes()).unwrap_or_default();
                return Err(fatal_err(
                    ctx,
                    ::core::ptr::null_mut::<Floc>(),
                    opt.len() as size_t,
                    b"unknown output-sync type '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                    &[FmtArg::Str((c.as_ptr()) as *const ::core::ffi::c_char)],
                ));
            }
        }
    }
    if let Some(mtx) = options.sync_mutex.borrow().as_ref() {
        let c = ::std::ffi::CString::new(mtx.as_bytes()).unwrap_or_default();
        osync_parse_mutex(ctx, c.as_ptr())?;
    }
    Ok(())
}
/// Print the usage table — to stdout for `-h`, to stderr for a bad switch —
/// byte-identical to the C oracle's hand-written table. Safe Rust throughout
/// the text assembly; the only libc touch left is the `-v`-with-`-h` version
/// banner, which prints through `print_version`'s printf path.
pub fn print_usage(ctx: &crate::execctx::ExecContext, options: &Options, bad: i32) {
    use std::io::Write;
    // The usage text, one option per line, in the C table's order.
    const USAGE: [&str; 35] = [
        "Options:\n",
        "  -b, -m                      Ignored for compatibility.\n",
        "  -B, --always-make           Unconditionally make all targets.\n",
        "  -C DIRECTORY, --directory=DIRECTORY\n                              Change to DIRECTORY before doing anything.\n",
        "  -d                          Print lots of debugging information.\n",
        "  --debug[=FLAGS]             Print various types of debugging information.\n",
        "  -e, --environment-overrides\n                              Environment variables override makefiles.\n",
        "  -E STRING, --eval=STRING    Evaluate STRING as a makefile statement.\n",
        "  -f FILE, --file=FILE, --makefile=FILE\n                              Read FILE as a makefile.\n",
        "  -h, --help                  Print this message and exit.\n",
        "  -i, --ignore-errors         Ignore errors from recipes.\n",
        "  -I DIRECTORY, --include-dir=DIRECTORY\n                              Search DIRECTORY for included makefiles.\n",
        "  -j [N], --jobs[=N]          Allow N jobs at once; infinite jobs with no arg.\n",
        "  --jobserver-style=STYLE     Select the style of jobserver to use.\n",
        "  -k, --keep-going            Keep going when some targets can't be made.\n",
        "  -l [N], --load-average[=N], --max-load[=N]\n                              Don't start multiple jobs unless load is below N.\n",
        "  -L, --check-symlink-times   Use the latest mtime between symlinks and target.\n",
        "  -n, --just-print, --dry-run, --recon\n                              Don't actually run any recipe; just print them.\n",
        "  -o FILE, --old-file=FILE, --assume-old=FILE\n                              Consider FILE to be very old and don't remake it.\n",
        "  -O[TYPE], --output-sync[=TYPE]\n                              Synchronize output of parallel jobs by TYPE.\n",
        "  -p, --print-data-base       Print make's internal database.\n",
        "  -q, --question              Run no recipe; exit status says if up to date.\n",
        "  -r, --no-builtin-rules      Disable the built-in implicit rules.\n",
        "  -R, --no-builtin-variables  Disable the built-in variable settings.\n",
        "  --shuffle[={SEED|random|reverse|none}]\n                              Perform shuffle of prerequisites and goals.\n",
        "  -s, --silent, --quiet       Don't echo recipes.\n",
        "  --no-silent                 Echo recipes (disable --silent mode).\n",
        "  -S, --no-keep-going, --stop\n                              Turns off -k.\n",
        "  -t, --touch                 Touch targets instead of remaking them.\n",
        "  --trace                     Print tracing information.\n",
        "  -v, --version               Print the version number of make and exit.\n",
        "  -w, --print-directory       Print the current directory.\n",
        "  --no-print-directory        Turn off -w, even if it was turned on implicitly.\n",
        "  -W FILE, --what-if=FILE, --new-file=FILE, --assume-new=FILE\n                              Consider FILE to be infinitely new.\n",
        "  --warn[=CONTROL]            Control warnings for makefile issues.\n",
    ];
    if options.print_version.get() {
        // SAFETY: `print_version` reads the NUL-terminated version/host
        // strings through the valid `ctx`; it writes and flushes through
        // Rust stdout, so the banner lands before the usage text below.
        unsafe {
            print_version(ctx);
        }
        crate::output::trace_out_ctx(ctx, b"\n");
    }
    let mut text = format!(
        "Usage: {} [options] [target] ...\n",
        crate::output::msg::program_name(ctx)
    );
    for line in USAGE {
        text.push_str(line);
    }
    let host = crate::version::MAKE_HOST.trim_end_matches('\0');
    match ctx.remote_backend.0.description() {
        None => {
            text.push_str(&format!("\nThis program built for {}\n", host));
        }
        Some(desc) => {
            text.push_str(&format!(
                "\nThis program built for {} ({})\n",
                host,
                String::from_utf8_lossy(desc.to_bytes())
            ));
        }
    }
    text.push_str("Report bugs to <bug-make@gnu.org>\n");
    // Flush explicitly: a fatal path ending in libc `exit()` would drop an
    // unflushed Rust buffer.
    if bad != 0 {
        let mut err = ctx.stderr.borrow_mut();
        let _ = err.write_all(text.as_bytes());
        let _ = err.flush();
    } else {
        let mut out = ctx.stdout.borrow_mut();
        if let Err(e) = out.write_all(text.as_bytes()).and_then(|()| out.flush()) {
            crate::output::record_stdout_error(&e);
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn reset_jobserver(options: &Options) {
    jobserver_clear();
    *options.jobserver_auth.borrow_mut() = None;
}

/// Jobserver reset for the end-of-run `clean_jobserver`/`die_cleanup` path,
/// which has
/// no `&Options` borrow. Reaches the owned `Options` through the borrow channel
/// (still installed for the dynamic extent of `main_0`).
///
/// # Safety
///
/// Calls `jobserver_clear`, which closes and resets the live jobserver fds;
/// must run single-threaded.
pub unsafe fn reset_jobserver_mirror(ctx: &crate::execctx::ExecContext) {
    jobserver_clear();
    with_options(ctx, |o| *o.jobserver_auth.borrow_mut() = None);
}
/// chdir(2) via `std::env::set_current_dir`: 0/-1 like the C call, errno set
/// on failure for the callers' perror/pfatal paths.
/// # Safety
/// `dir` must be a valid NUL-terminated path.
unsafe fn chdir_c(dir: *const ::core::ffi::c_char) -> i32 {
    use std::os::unix::ffi::OsStrExt;
    let os = ::std::ffi::OsStr::from_bytes(::core::ffi::CStr::from_ptr(dir).to_bytes());
    match ::std::env::set_current_dir(os) {
        Ok(()) => 0,
        Err(e) => {
            *__errno_location() = e.raw_os_error().unwrap_or(0);
            -1
        }
    }
}

/// getcwd(3) via `std::env::current_dir`, copied into the fixed buffer the
/// callers keep long-lived pointers into. Returns false with errno set on
/// failure — including ERANGE when the path cannot fit, as getcwd did.
/// # Safety
/// `buf` must be valid for writes of `size` bytes.
unsafe fn getcwd_into(buf: *mut ::core::ffi::c_char, size: usize) -> bool {
    use std::os::unix::ffi::OsStrExt;
    match ::std::env::current_dir() {
        Ok(p) => {
            let b = p.as_os_str().as_bytes();
            if b.len() + 1 > size {
                *__errno_location() = libc::ERANGE;
                return false;
            }
            ::core::ptr::copy_nonoverlapping(
                b.as_ptr() as *const ::core::ffi::c_char,
                buf,
                b.len(),
            );
            *buf.add(b.len()) = 0;
            true
        }
        Err(e) => {
            *__errno_location() = e.raw_os_error().unwrap_or(0);
            false
        }
    }
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn temp_stdin_unlink(ctx: &crate::execctx::ExecContext) {
    if opt_stdin_offset(ctx) >= 0 && !ctx.temp_stdin_name.0.get().is_null() {
        let nm: *const ::core::ffi::c_char = ctx.temp_stdin_name.0.get();
        with_options(ctx, |o| o.stdin_offset.set(-1));
        let r = crate::misc::unlink_c(nm);
        if r < 0 && *__errno_location() != ENOENT && !handling_fatal_signal(ctx) {
            perror_with_name(
                ctx,
                b"unlink (temporary file): \0" as *const u8 as *const ::core::ffi::c_char,
                nm,
            );
        }
    }
}

pub unsafe fn main_0(
    argc: i32,
    argv: *mut *mut ::core::ffi::c_char,
    envp: *mut *mut ::core::ffi::c_char,
) -> Result<crate::build_result::BuildReport, crate::build_result::BuildError> {
    use crate::build_result::{BuildError, BuildReport};
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut makefile_status: i32 = MAKE_SUCCESS;
    let mut read_files: Vec<crate::dep::GoalDepNode>;
    let mut current_directory: [::core::ffi::c_char; 4097] = [0; 4097];
    let mut restarts: ::core::ffi::c_uint = 0;
    let mut syncing: ::core::ffi::c_uint;
    let argv_slots: Option<u32>;
    // Owned execution context for this make invocation, threaded (`&ctx`) down
    // the call graph in place of the former process-global makelevel. It
    // starts at level 0 (matching the old startup default) and is rebuilt from
    // the parsed `MAKELEVEL` env var below.
    let mut ctx = crate::execctx::ExecContext::new(crate::execctx::Config {
        makelevel: 0,
        ..Default::default()
    });
    // Owned option/flag state for this make invocation now lives on `ctx`
    // directly (`ctx.options`) rather than a separate local reached through
    // the `OPTIONS_PTR` thread-local — every `&ExecContext` site already
    // carries it for free. `options` below is a plain reference alias so the
    // rest of `main_0`'s body (which predates this change) keeps reading/
    // writing through the familiar `options.field` shape unchanged; it is
    // re-derived after the build-phase context rebuild further down, since
    // that rebuild replaces `ctx` wholesale.
    let options = &ctx.options;
    // Borrow channel to `ctx` for the glob `gl_opendir` callback, which reaches
    // the per-run directory cache held on the context. `ctx`'s stack slot is
    // stable across the build-phase rebuild below, so this install stays valid.
    CTX_PTR.with(|p| p.set(&ctx as *const crate::execctx::ExecContext));
    initialize_variable_output(&ctx);
    spin(b"main-entry\0" as *const u8 as *const ::core::ffi::c_char);
    if check_io_state(&ctx) & 0x8 as ::core::ffi::c_uint != 0 {
        atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    }
    crate::output::output_init(&ctx, ctx.make_sync.as_ptr());
    initialize_stopchar_map();
    crate::warning::init(&ctx);
    options.verify.set(true);
    setlocale(LC_ALL, b"\0" as *const u8 as *const ::core::ffi::c_char);
    let mut fatal_sigs = ctx.fatal_signal_set.0.get();
    sigemptyset(&raw mut fatal_sigs);
    ctx.fatal_signal_set.0.set(fatal_sigs);
    install_fatal_signal(&ctx, 1);
    install_fatal_signal(&ctx, 3);
    install_fatal_signal(&ctx, 13);
    install_fatal_signal(&ctx, 2);
    install_fatal_signal(&ctx, 15);
    install_fatal_signal(&ctx, 24);
    install_fatal_signal(&ctx, 25);
    bsd_signal(SIGCHLD, SIG_DFL);
    crate::output::output_init(&ctx, ::core::ptr::null_mut::<output>());
    if (*argv.offset(0_i32 as isize)).is_null() {
        let fresh33 = &mut (*argv.offset(0_i32 as isize));
        *fresh33 = b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    if *(*argv.offset(0_i32 as isize)).offset(0_i32 as isize) as i32 == 0 {
        ctx.program
            .0
            .set(b"make\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        let mut prog: *const ::core::ffi::c_char =
            strrchr(*argv.offset(0_i32 as isize), '/' as i32);
        if prog.is_null() {
            prog = *argv.offset(0_i32 as isize);
        } else {
            prog = prog.offset(1_i32 as isize);
        }
        ctx.program.0.set(prog);
    }
    initialize_global_hash_tables(&ctx);
    get_tmpdir(&ctx);
    if !getcwd_into(
        &raw mut current_directory as *mut ::core::ffi::c_char,
        GET_PATH_MAX as usize,
    ) {
        perror_with_name(
            &ctx,
            b"getcwd\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
        current_directory[0_i32 as usize] = 0;
        ctx.directory_before_chdir
            .0
            .set(::core::ptr::null_mut::<::core::ffi::c_char>());
    } else {
        ctx.directory_before_chdir.0.set(xstrdup(
            &raw mut current_directory as *mut ::core::ffi::c_char,
        ));
    }
    define_special(&ctx, b"MAKEFLAGS\0")?;
    define_special(&ctx, b".VARIABLES\0")?;
    define_special(&ctx, b".RECIPEPREFIX\0")?;
    define_special(&ctx, b".WARNINGS\0")?;
    crate::variable::define_named(
        &ctx,
        b".SHELLFLAGS\0",
        b"-c\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
    )?;
    crate::variable::define_named(
        &ctx,
        b".LOADED\0",
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
    )?;
    let features: *const ::core::ffi::c_char = b"target-specific order-only second-expansion else-if shortest-stem undefine oneshell nocomment grouped-target extra-prereqs notintermediate shell-export archives jobserver jobserver-fifo output-sync check-symlink maintainer\0"
        as *const u8 as *const ::core::ffi::c_char;
    crate::variable::define_named(&ctx, b".FEATURES\0", features, o_default, 0)?;
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
                    set_stdio_traced(&ctx, true);
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
                (*ctx.variable_globals.current_variable_set_list.get()).set,
                NILF,
            )?;
            if *(*v).name as i32 == *(b"SHELL\0" as *const u8 as *const ::core::ffi::c_char) as i32
                && (*(*v).name as i32 == 0
                    || strcmp(
                        (*v).name.offset(1_i32 as isize),
                        (b"SHELL\0" as *const u8 as *const ::core::ffi::c_char)
                            .offset(1_i32 as isize),
                    ) == 0)
            {
                export = v_noexport;
                let mut sv = ctx.shell_var.0.get();
                sv.name = xstrdup(b"SHELL\0" as *const u8 as *const ::core::ffi::c_char);
                sv.length = 5;
                sv.value = xstrdup(ep);
                ctx.shell_var.0.set(sv);
            }
            (*v).set_export(export as variable_export);
        }
        i = i.wrapping_add(1);
    }
    if !lookup_named(&ctx, b"GNUMAKEFLAGS\0")?.is_null() {
        decode_env_switches(
            &ctx,
            options,
            b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
            o_command,
        )?;
        crate::variable::define_named(
            &ctx,
            b"GNUMAKEFLAGS\0",
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            o_env,
            0,
        )?;
    }
    decode_env_switches(
        &ctx,
        options,
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        o_command,
    )?;
    set_make_sync_syncout(
        &ctx,
        (opt_output_sync(&ctx) == OUTPUT_SYNC_LINE || opt_output_sync(&ctx) == OUTPUT_SYNC_TARGET)
            as i32 as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
    set_output_context(if make_sync_syncout(&ctx) as i32 != 0 {
        ctx.make_sync.as_ptr()
    } else {
        ::core::ptr::null_mut::<output>()
    });
    let env_slots: Option<u32> = options.arg_job_slots.get();
    options.arg_job_slots.set(None);
    let cli_tokens: Vec<::std::ffi::OsString> = {
        use std::os::unix::ffi::OsStrExt;
        (1..argc)
            .map(|i| {
                let cstr = ::core::ffi::CStr::from_ptr(*argv.offset(i as isize));
                ::std::ffi::OsStr::from_bytes(cstr.to_bytes()).to_os_string()
            })
            .collect()
    };
    decode_switches(&ctx, options, &cli_tokens, o_command)?;
    argv_slots = options.arg_job_slots.get();
    if options.arg_job_slots.get().is_none() {
        options.arg_job_slots.set(env_slots);
    }
    if options.print_usage.get() {
        print_usage(&ctx, options, 0);
        die_cleanup(&ctx, MAKE_SUCCESS);
        return Ok(BuildReport);
    }
    if options.print_version.get() {
        print_version(&ctx);
        die_cleanup(&ctx, MAKE_SUCCESS);
        return Ok(BuildReport);
    }
    // The C original forced stdout line-buffered here (`setvbuf(stdout, 0,
    // _IOLBF, BUFSIZ)`); Rust's stdout is always a `LineWriter`, and no libc
    // stdout writers remain, so there is no stream left to configure.
    {
        let shuffle = options.shuffle_mode.borrow().clone();
        if let Some(arg) = shuffle {
            crate::shuffle::set_mode(&ctx, &arg);
            *options.shuffle_mode.borrow_mut() = crate::shuffle::get_mode(&ctx);
        }
    }
    if isatty(libc::STDOUT_FILENO) != 0 && lookup_named(&ctx, b"MAKE_TERMOUT\0")?.is_null() {
        define_tty_var(&ctx, b"MAKE_TERMOUT\0", libc::STDOUT_FILENO)?;
    }
    if isatty(libc::STDERR_FILENO) != 0 && lookup_named(&ctx, b"MAKE_TERMERR\0")?.is_null() {
        define_tty_var(&ctx, b"MAKE_TERMERR\0", libc::STDERR_FILENO)?;
    }
    syncing = (opt_output_sync(&ctx) == OUTPUT_SYNC_LINE
        || opt_output_sync(&ctx) == OUTPUT_SYNC_TARGET) as i32 as ::core::ffi::c_uint;
    if make_sync_syncout(&ctx) as i32 != 0 && syncing == 0 {
        crate::output::output_close(&ctx, ctx.make_sync.as_ptr());
    }
    set_make_sync_syncout(&ctx, syncing as ::core::ffi::c_uint);
    set_output_context(if make_sync_syncout(&ctx) as i32 != 0 {
        ctx.make_sync.as_ptr()
    } else {
        ::core::ptr::null_mut::<output>()
    });
    let v_0: *mut variable = lookup_named(&ctx, b"MAKELEVEL\0")?;
    let parsed_makelevel: u32 = if !v_0.is_null()
        && *(*v_0).value.offset(0_i32 as isize) as i32 != 0
        && *(*v_0).value.offset(0_i32 as isize) as i32 != '-' as i32
    {
        make_toui(::core::ffi::CStr::from_ptr((*v_0).value)).unwrap_or(0)
    } else {
        0
    };
    // Rebuild the context now that `MAKELEVEL` is known, but hand the directory
    // cache across: it was populated during makefile parsing ($(wildcard),
    // vpath, includes) and must persist through the build, exactly as the former
    // process-global tables did. The glob `read_dirstream` scratch buffer rides
    // along for the same reason — a single heap block served the whole run as
    // the former `static mut buf` did (its contents are scratch, but carrying it
    // avoids re-allocating a second block mid-run). Everything else is per-build
    // state that resets. `CTX_PTR` keeps pointing at this same `ctx` slot, so the
    // glob callbacks see the carried state.
    let carried_directories = ::core::mem::take(&mut ctx.directories);
    let carried_directory_contents = ::core::mem::take(&mut ctx.directory_contents);
    let carried_read_dirstream_buf = ::core::mem::take(&mut ctx.read_dirstream_buf);
    let carried_read_dirstream_bufsz = ::core::mem::take(&mut ctx.read_dirstream_bufsz);
    // `parse_file_seq`'s scratch buffer is populated during parsing
    // ($(wildcard), prerequisite lists) and reused during the build
    // (function.rs, implicit.rs), so it rides along for the same reason.
    let carried_file_seq_tmpbuf = ::core::mem::take(&mut ctx.file_seq_tmpbuf);
    // The file table is populated during parsing and consulted throughout the
    // build, so carry it across the rebuild just like the directory cache;
    // otherwise every file entered while reading makefiles would be lost.
    let carried_files = ::core::mem::take(&mut ctx.filenodes);
    // Cleanup state recorded before the rebuild (`die_cleanup`/re-exec read it
    // after).
    let carried_temp_stdin = ::core::mem::take(&mut ctx.temp_stdin_name);
    let carried_dir_before_chdir = ::core::mem::take(&mut ctx.directory_before_chdir);
    // The program name is derived from argv[0] at startup and prefixes every
    // message for the rest of the run.
    let carried_program = ::core::mem::take(&mut ctx.program);
    // Computed once at startup by the unconditional `get_tmpdir` call above;
    // carrying it avoids re-probing the environment (and re-warning about an
    // invalid MAKE_TMPDIR/TMPDIR) for temp-file users that run after this
    // rebuild (get_tmpfile for `-f -`, jobserver_setup, output sync).
    let carried_tmpdir = ::core::mem::take(&mut ctx.tmpdir);
    // `--shuffle=` is decoded from argv/MAKEFLAGS before this rebuild (see the
    // `set_mode` call above); carrying it keeps the configured mode/seed (and
    // any PRNG advancement) alive for the shuffling that happens during and
    // after this rebuild.
    let carried_shuffle = ::core::mem::take(&mut ctx.shuffle);
    // SHELL was recorded from the environment scan above and is appended to
    // child environments during the build; the command-variable list is built
    // as argv/`MAKEFLAGS` switches are decoded (both before and after this
    // rebuild) and walked for `MAKEOVERRIDES` below.
    let carried_shell_var = ::core::mem::take(&mut ctx.shell_var);
    let carried_command_variables = ::core::mem::take(&mut ctx.command_variables);
    // The fatal-signal set was built by the `install_fatal_signal` calls above
    // and is what `block_sigs`/`unblock_sigs` mask around child bookkeeping.
    let carried_fatal_signal_set = ::core::mem::take(&mut ctx.fatal_signal_set);
    // The output-sync record was configured by `output_init` at startup and
    // `output_context` may already hold its address (set when `MAKEFLAGS`
    // enabled `-O` above); carrying the Box keeps that address valid, and the
    // pointer rides along with it.
    let carried_make_sync = ::core::mem::take(&mut ctx.make_sync);
    let carried_output_context = ::core::mem::take(&mut ctx.output_context);
    // `output_init(&ctx, null)` above (the stdio-append-mode branch) saved the
    // original stdout/stderr `O_APPEND` flags so `output_close`/`die_cleanup` can
    // restore them at exit; carrying them keeps that restoration working
    // after this rebuild instead of silently reverting to the "unset" -1
    // sentinel and making `fd_reset_append` a no-op.
    let carried_stdout_flags = ::core::mem::take(&mut ctx.stdout_flags);
    let carried_stderr_flags = ::core::mem::take(&mut ctx.stderr_flags);
    // `initialize_global_hash_tables` (via `hash_init_function_table`) filled
    // this in before the rebuild; carrying it keeps the function table's
    // `ht_vec` allocation (and every `gmk_add_function` registration since)
    // alive instead of leaking it and losing the lookups.
    let carried_function_table = ::core::mem::take(&mut ctx.function_table);
    // `warning::init` above configured the `--warn`/`.WARNINGS` defaults;
    // carrying it across keeps those defaults (and any `--warn`/`MAKEFLAGS`
    // overrides already decoded above) alive for the rest of the run.
    let carried_warning_state = ::core::mem::take(&mut ctx.warning_state);
    // `decode_switches` above already ran `decode_debug_flags` for any
    // `-d`/`--debug`/`MAKEFLAGS` bits given before this rebuild; carrying the
    // decoded level keeps the version banner and "Reading makefiles..." trace
    // below (both gated on `db_level`) from silently seeing a reset-to-0 value.
    let carried_db_level = ::core::mem::take(&mut ctx.db_level);
    // `initialize_global_hash_tables`/`define_variable_in_set` above already
    // populated the global variable set (`MAKEFLAGS`, `.VARIABLES`, every
    // inherited environment variable, `SHELL`, ...); carrying it keeps that
    // data (and the `global_setlist`/`current_variable_set_list` addresses
    // every pointer-identity check in `variable.rs` compares against) alive
    // across this rebuild instead of resetting to an empty table.
    let carried_variable_globals = ::core::mem::take(&mut ctx.variable_globals);
    // `initialize_variable_output()` above (at main_0's very start) already
    // allocated the shared `$(...)`/recipe expansion output buffer; carrying
    // it keeps that single allocation alive for the rest of the run instead
    // of silently discarding it and allocating a second one on first use
    // after this rebuild, matching the one-allocation-for-the-whole-run
    // invariant the former `static mut variable_buffer` had (and that
    // `read_dirstream_buf` preserves the same way).
    let carried_variable_buffer = ::core::mem::take(&mut ctx.variable_buffer);
    // `options` holds real accumulated run state (decoded command-line
    // flags, `goals`, `switches`) that must survive this rebuild rather than
    // reset to defaults; carry it forward like every other field above.
    let carried_options = ::core::mem::take(&mut ctx.options);
    ctx = crate::execctx::ExecContext {
        options: carried_options,
        directories: carried_directories,
        directory_contents: carried_directory_contents,
        read_dirstream_buf: carried_read_dirstream_buf,
        read_dirstream_bufsz: carried_read_dirstream_bufsz,
        file_seq_tmpbuf: carried_file_seq_tmpbuf,
        filenodes: carried_files,
        temp_stdin_name: carried_temp_stdin,
        directory_before_chdir: carried_dir_before_chdir,
        program: carried_program,
        tmpdir: carried_tmpdir,
        shuffle: carried_shuffle,
        shell_var: carried_shell_var,
        command_variables: carried_command_variables,
        fatal_signal_set: carried_fatal_signal_set,
        make_sync: carried_make_sync,
        output_context: carried_output_context,
        stdout_flags: carried_stdout_flags,
        stderr_flags: carried_stderr_flags,
        function_table: carried_function_table,
        warning_state: carried_warning_state,
        db_level: carried_db_level,
        variable_globals: carried_variable_globals,
        variable_buffer: carried_variable_buffer,
        ..crate::execctx::ExecContext::new(crate::execctx::Config {
            makelevel: parsed_makelevel,
            ..Default::default()
        })
    };
    // Re-derive the glob borrow channel from the rebuilt `ctx`. The `&mut ctx`
    // above (`mem::take` + reassignment) invalidates the pointer installed at
    // startup under Rust's aliasing model — even though the stack slot address
    // is unchanged — so post-rebuild glob callbacks must read a fresh
    // provenance pointing at the new context.
    CTX_PTR.with(|p| p.set(&ctx as *const crate::execctx::ExecContext));
    // Re-derive `options` too: the old `&ctx.options` borrow from before the
    // rebuild is stale (a fresh `ctx` was assigned above), but the data it
    // points at is the same `carried_options` value moved forward into it.
    let options = &ctx.options;
    ctx.always_make_flag
        .set(options.always_make.get() && restarts == 0);
    if options.no_builtin_variables.get() {
        options.no_builtin_rules.set(true);
    }
    if 0x1_i32 & db_level(&ctx) != 0 {
        // `print_version` writes and flushes through Rust stdout; the C
        // original's trailing `fflush(stdout)` has no buffer left to empty.
        print_version(&ctx);
    }
    if current_directory[0_i32 as usize] as i32 != 0
        && !(*argv.offset(0_i32 as isize)).is_null()
        && *(*argv.offset(0_i32 as isize)).offset(0_i32 as isize) as i32 != '/' as i32
        && !strchr(*argv.offset(0_i32 as isize), '/' as i32).is_null()
    {
        let fresh41 = &mut (*argv.offset(0_i32 as isize));
        *fresh41 = xstrdup(
            concat(&[
                cstr_bytes_or_empty(&raw mut current_directory as *const ::core::ffi::c_char),
                b"/",
                cstr_bytes_or_empty(*argv.offset(0_i32 as isize)),
            ])
            .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    ctx.starting_directory
        .0
        .set(&raw mut current_directory as *mut ::core::ffi::c_char);
    if !options.directories.borrow().is_empty() {
        for entry in options.directories.borrow().iter() {
            let dir: *const ::core::ffi::c_char = entry.as_ptr();
            if chdir_c(dir) < 0 {
                return Err(pfatal_with_name_err(&ctx, dir));
            }
        }
    }
    if !options.directories.borrow().is_empty() {
        if !getcwd_into(
            &raw mut current_directory as *mut ::core::ffi::c_char,
            GET_PATH_MAX as usize,
        ) {
            perror_with_name(
                &ctx,
                b"getcwd\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
            );
            ctx.starting_directory
                .0
                .set(::core::ptr::null_mut::<::core::ffi::c_char>());
        } else {
            ctx.starting_directory
                .0
                .set(&raw mut current_directory as *mut ::core::ffi::c_char);
        }
    }
    crate::variable::define_named(
        &ctx,
        b"CURDIR\0",
        &raw mut current_directory as *mut ::core::ffi::c_char,
        o_file,
        0,
    )?;
    {
        let include_dirs = options.include_dirs.borrow();
        let inc_paths: Vec<std::path::PathBuf> = include_dirs
            .iter()
            .map(|s| {
                use std::os::unix::ffi::OsStrExt;
                std::path::PathBuf::from(std::ffi::OsStr::from_bytes(s.as_bytes()))
            })
            .collect();
        construct_include_path(&ctx, &inc_paths)?;
    }
    if options.jobserver_auth.borrow().is_some() {
        // Reset the jobserver unless we successfully inherited the parent's.
        let mut do_reset = true;
        if argv_slots.is_none() {
            let auth = options.jobserver_auth.borrow().clone().unwrap();
            let auth_c = ::std::ffi::CString::new(auth.as_bytes()).unwrap_or_default();
            if jobserver_parse_auth(&ctx, auth_c.as_ptr())? != 0 {
                do_reset = false;
            } else {
                error(
                    &ctx,
                    ::core::ptr::null_mut::<Floc>(),
                    0,
                    b"warning: jobserver unavailable: using -j1 (add '+' to parent make rule)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    &[],
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
                &[FmtArg::Int((argv_slots.unwrap_or(0)) as i32 as i64)],
            );
        }
        if do_reset {
            reset_jobserver(options);
        }
    }
    crate::variable::define_named(
        &ctx,
        b"MAKE_COMMAND\0",
        *argv.offset(0_i32 as isize),
        o_default,
        0,
    )?;
    crate::variable::define_named(
        &ctx,
        b"MAKE\0",
        b"$(MAKE_COMMAND)\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        1,
    )?;
    if !ctx.command_variables.0.get().is_null() {
        let mut cv: *mut CommandVariable;
        let mut v_1: *mut variable;
        let mut len_0: size_t = 0;
        let mut p: *mut ::core::ffi::c_char;
        cv = ctx.command_variables.0.get();
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
        cv = ctx.command_variables.0.get();
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
        crate::variable::define_named(&ctx, b"-*-command-variables-*-\0", value, o_automatic, 0)?;
        drop(value_buf);
        crate::variable::define_named(
            &ctx,
            b"MAKEOVERRIDES\0",
            b"${-*-command-variables-*-}\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            1,
        )?;
    }
    if !options.makefiles.borrow().is_empty() {
        let mut i_1: usize;
        i_1 = 0;
        while i_1 < options.makefiles.borrow().len() {
            if options.makefiles.borrow()[i_1].as_bytes() == b"-" {
                let mut newnm: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                if options.stdin_offset.get() >= 0 {
                    return Err(fatal_err(
                        &ctx,
                        ::core::ptr::null_mut::<Floc>(),
                        0,
                        b"Makefile from standard input specified twice\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    ));
                }
                let Some(mut outfile) = get_tmpfile(&ctx, &raw mut newnm) else {
                    return Err(fatal_err(
                        &ctx,
                        ::core::ptr::null_mut::<Floc>(),
                        0,
                        b"cannot store makefile from stdin to a temporary file\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    ));
                };
                {
                    use ::std::io::{Read, Write};
                    let mut sin = ::std::io::stdin().lock();
                    let mut buf = [0u8; 2048];
                    loop {
                        // The C loop stopped on EOF *or* any read error
                        // (ferror, EINTR included) without reporting; mirror
                        // that by breaking on Err.
                        let n = match sin.read(&mut buf) {
                            Ok(0) | Err(_) => break,
                            Ok(n) => n,
                        };
                        if let Err(e) = outfile.write_all(&buf[..n]) {
                            let es: *const ::core::ffi::c_char =
                                strerror(e.raw_os_error().unwrap_or(0));
                            return Err(fatal_err(
                                &ctx,
                                ::core::ptr::null_mut::<Floc>(),
                                (strlen(newnm) as size_t).wrapping_add(strlen(es) as size_t),
                                b"fwrite: temporary file %s: %s\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[
                                    FmtArg::Str((newnm) as *const ::core::ffi::c_char),
                                    FmtArg::Str(es as *const ::core::ffi::c_char),
                                ],
                            ));
                        }
                    }
                }
                drop(outfile);
                let cached = strcache_add(&ctx, newnm);
                options.makefiles.borrow_mut()[i_1] =
                    ::core::ffi::CStr::from_ptr(cached).to_owned();
                options.stdin_offset.set(i_1 as i32);
                ctx.temp_stdin_name.0.set(cached);
                free(newnm as *mut ::core::ffi::c_void);
            }
            i_1 = i_1.wrapping_add(1);
        }
    }
    if options.stdin_offset.get() >= 0 {
        let name_bytes = {
            let mfs = options.makefiles.borrow();
            ::std::ffi::CStr::from_ptr(mfs[options.stdin_offset.get() as usize].as_ptr())
                .to_bytes()
                .to_vec()
        };
        let f = enter_file(&ctx, &name_bytes);
        let mtime = f_mtime(&ctx, f, false)?;
        if let Some(node) = ctx.filenodes.get(f) {
            let mut guard = node.lock().expect("file node poisoned");
            guard.updated = true;
            guard.update_status = crate::file::UpdateStatus::Success;
            guard.command_state = crate::file::CommandState::Finished;
            guard.intermediate = false;
            guard.dontcare = false;
            guard.mtime_before_update = mtime;
            guard.last_mtime = mtime;
        }
    }
    bsd_signal(
        SIGCHLD,
        Some(child_handler as unsafe extern "C" fn(i32) -> ()),
    );
    let mut block: sigset_t = SigsetT { __val: [0; 16] };
    sigemptyset(&raw mut block);
    sigaddset(&raw mut block, SIGCHLD);
    if sigprocmask(
        SIG_SETMASK,
        &raw mut block,
        ::core::ptr::null_mut::<sigset_t>(),
    ) < 0
    {
        return Err(pfatal_with_name_err(
            &ctx,
            b"sigprocmask(SIG_SETMASK, SIGCHLD)\0" as *const u8 as *const ::core::ffi::c_char,
        ));
    }
    bsd_signal(
        SIGUSR1,
        Some(debug_signal_handler as unsafe extern "C" fn(i32) -> ()),
    );
    set_default_suffixes(&ctx, options)?;
    define_automatic_variables(&ctx)?;
    // Bound before the dereference rather than `?`-ed inside it, and reached
    // through `as_mut` rather than a bare `*`: the pointer only exists once
    // the `Result` has been unwrapped, and `define_makeflags` returns whatever
    // `define_variable_in_set` produced, which the checked reference asserts
    // is non-null instead of assuming it.
    define_makeflags(&ctx, options, 0)?
        .as_mut()
        .expect("define_makeflags always defines MAKEFLAGS")
        .set_export(v_export as variable_export);
    define_default_variables(&ctx, options)?;
    enter_file(&ctx, b".DEFAULT");
    ctx.default_goal_var.0.set(crate::variable::define_named(
        &ctx,
        b".DEFAULT_GOAL\0",
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_file,
        0,
    )?);
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
            )?;
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
        crate::variable::define_named(&ctx, b"-*-eval-flags-*-\0", value_0, o_automatic, 0)?;
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
        .map(|s| strcache_add(&ctx, s.as_ptr()))
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
    )?;
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
        options,
        b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        o_env,
    )?;
    crate::variable::define_named(
        &ctx,
        b"GNUMAKEFLAGS\0",
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_override,
        0,
    )?;
    decode_env_switches(
        &ctx,
        options,
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        o_env,
    )?;
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
                &[FmtArg::Int(
                    (options.arg_job_slots.get().unwrap_or(0)) as i32 as i64,
                )],
            );
        }
        reset_jobserver(options);
    }
    syncing = (opt_output_sync(&ctx) == OUTPUT_SYNC_LINE
        || opt_output_sync(&ctx) == OUTPUT_SYNC_TARGET) as i32 as ::core::ffi::c_uint;
    if make_sync_syncout(&ctx) as i32 != 0 && syncing == 0 {
        crate::output::output_close(&ctx, ctx.make_sync.as_ptr());
    }
    set_make_sync_syncout(&ctx, syncing as ::core::ffi::c_uint);
    set_output_context(if make_sync_syncout(&ctx) as i32 != 0 {
        ctx.make_sync.as_ptr()
    } else {
        ::core::ptr::null_mut::<output>()
    });
    disable_builtins(&ctx, options)?;
    options
        .job_slots
        .set(if options.jobserver_auth.borrow().is_some() {
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
        if jobserver_setup(
            &ctx,
            options.job_slots.get().wrapping_sub(1) as i32,
            style_ptr,
        )? != 0
        {
            let auth = jobserver_get_auth(&ctx);
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
        set_output_context(::core::ptr::null_mut::<output>());
        crate::output::output_close(&ctx, ctx.make_sync.as_ptr());
        syncing = 0;
        options.output_sync.set(OUTPUT_SYNC_NONE);
    }
    if syncing != 0 {
        let has_mutex = options.sync_mutex.borrow().is_some();
        if !has_mutex {
            osync_setup(&ctx);
            let m = osync_get_mutex(&ctx);
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
            if osync_parse_mutex(&ctx, mtx_c.as_ptr())? == 0 {
                osync_clear();
                *options.sync_mutex.borrow_mut() = None;
            }
        }
    }
    if options.jobserver_auth.borrow().is_some() && (0x2_i32 | 0x4_i32) & db_level(&ctx) != 0 {
        let auth = options.jobserver_auth.borrow().clone().unwrap();
        let auth_c = ::std::ffi::CString::new(auth.as_bytes()).unwrap_or_default();
        crate::output::trace_parts(&[b"Using jobserver controller ", auth_c.to_bytes(), b"\n"]);
    }
    if options.sync_mutex.borrow().is_some() && 0x2_i32 & db_level(&ctx) != 0 {
        let mtx = options.sync_mutex.borrow().clone().unwrap();
        let mtx_c = ::std::ffi::CString::new(mtx.as_bytes()).unwrap_or_default();
        crate::output::trace_parts(&[b"Using output-sync mutex ", mtx_c.to_bytes(), b"\n"]);
    }
    define_makeflags(&ctx, options, 0)?;
    snap_deps(&ctx)?;
    install_default_suffix_rules(&ctx, options);
    convert_to_pattern(&ctx);
    install_default_implicit_rules(&ctx, options)?;
    snap_implicit_rules(&ctx)?;
    build_vpath_lists(&ctx)?;
    if !options.old_files.borrow().is_empty() {
        for of in options.old_files.borrow().iter() {
            let name_bytes = ::std::ffi::CStr::from_ptr(of.as_ptr()).to_bytes().to_vec();
            let f_0 = enter_file(&ctx, &name_bytes);
            if let Some(node) = ctx.filenodes.get(f_0) {
                let mut guard = node.lock().expect("file node poisoned");
                guard.mtime_before_update = OLD_MTIME as uintmax_t;
                guard.last_mtime = guard.mtime_before_update;
                guard.updated = true;
                guard.update_status = crate::file::UpdateStatus::Success;
                guard.command_state = crate::file::CommandState::Finished;
            }
        }
    }
    if options.print_targets.get() {
        print_targets(&ctx);
        die_cleanup(&ctx, EXIT_SUCCESS);
        return Ok(BuildReport);
    }
    if restarts == 0 && !options.new_files.borrow().is_empty() {
        for nf in options.new_files.borrow().iter() {
            let name_bytes = ::std::ffi::CStr::from_ptr(nf.as_ptr()).to_bytes().to_vec();
            let f_1 = enter_file(&ctx, &name_bytes);
            if let Some(node) = ctx.filenodes.get(f_1) {
                let mut guard = node.lock().expect("file node poisoned");
                guard.mtime_before_update = new_file_mtime();
                guard.last_mtime = guard.mtime_before_update;
            }
        }
    }
    ctx.remote_backend.0.setup();
    set_output_context(::core::ptr::null_mut::<output>());
    crate::output::output_close(&ctx, ctx.make_sync.as_ptr());
    if options.shuffle_mode.borrow().is_some() && 0x1_i32 & db_level(&ctx) != 0 {
        let sm = options.shuffle_mode.borrow().clone().unwrap();
        let sm_c = ::std::ffi::CString::new(sm.as_bytes()).unwrap_or_default();
        crate::output::trace_parts(&[b"Enabled shuffle mode: ", sm_c.to_bytes(), b"\n"]);
    }
    if !read_files.is_empty() {
        let mut skipped_makefiles: Vec<crate::dep::GoalDepNode> = Vec::new();
        let mut nargv: *mut *const ::core::ffi::c_char = argv as *mut *const ::core::ffi::c_char;
        let mut any_failed: i32 = 0;
        let mut status: UpdateStatus;
        if 0x1_i32 & db_level(&ctx) != 0 {
            crate::output::trace_out(b"Updating makefiles....\n");
        }
        // The c2rust list re-reversed `read_files` here (it had been built by
        // front-pushing); mirror that so makefiles are remade in source order.
        read_files.reverse();

        // For each makefile goal, decide whether it might loop (skip remaking)
        // and, for the rest, snapshot its current mtime. Drop skipped entries
        // from `read_files`, diverting errored ones to `skipped_makefiles`.
        let mut makefile_mtimes: Vec<uintmax_t> = Vec::with_capacity(read_files.len());
        let mut kept: Vec<crate::dep::GoalDepNode> = Vec::with_capacity(read_files.len());
        for g in ::core::mem::take(&mut read_files) {
            let Some(fid) = g.dep.file else { continue };
            // A makefile "might loop" if it is phony, or any of its double-colon
            // entries has a recipe but no prerequisites.
            let (skip, last_mtime, name) = {
                let node = ctx.filenodes.get(fid);
                match node {
                    None => (true, UNKNOWN_MTIME as uintmax_t, Vec::new()),
                    Some(node) => {
                        let guard = node.lock().expect("file node poisoned");
                        let mut skip = guard.phony;
                        // Match C: the "recipe but no deps ⇒ might loop" check
                        // walks only the `double_colon` chain (`f->double_colon`).
                        // For a single-colon target that chain is empty, so the
                        // check never applies — a plain `gen.mk:` rule with no
                        // prereqs must still be remade.
                        if !skip && guard.is_double_colon {
                            for entry in std::iter::once(&*guard).chain(guard.double_colon.iter()) {
                                if entry.deps.is_empty() && entry.recipe.is_some() {
                                    skip = true;
                                    break;
                                }
                            }
                        }
                        (skip, guard.last_mtime, guard.name.clone())
                    }
                }
            };
            if !skip {
                let mtime = if last_mtime == UNKNOWN_MTIME as uintmax_t {
                    f_mtime(&ctx, fid, false)?
                } else {
                    last_mtime
                };
                makefile_mtimes.push(mtime);
                kept.push(g);
            } else {
                if 0x2_i32 & db_level(&ctx) != 0 {
                    crate::output::trace_parts(&[
                        b"Makefile '",
                        &name,
                        b"' might loop; not remaking it.\n",
                    ]);
                }
                if g.error != 0 && !g.dep.flags.contains(crate::dep::DepFlags::DONTCARE) {
                    skipped_makefiles.push(g);
                    any_failed = 1;
                }
            }
        }
        read_files = kept;
        define_makeflags(&ctx, options, 1)?;
        let orig_db_level: i32 = db_level(&ctx);
        if 0x100_i32 & db_level(&ctx) == 0 {
            set_db_level(&ctx, DB_NONE);
        }
        options.rebuilding_makefiles.set(true);
        let goal_chain_result = update_goal_chain(&ctx, &mut read_files);
        // These are `ExecContext`/`Options` resets, not covered by
        // `die_cleanup`'s side effects — run them before propagating any
        // error so state stays correctly restored even on an early return.
        options.rebuilding_makefiles.set(false);
        set_db_level(&ctx, orig_db_level);
        status = goal_chain_result?;
        for d_1 in &skipped_makefiles {
            let err: *const ::core::ffi::c_char = strerror(d_1.error);
            let mut name_bytes = goal_name_bytes(&ctx, d_1);
            name_bytes.push(0);
            let d1_name = name_bytes.as_ptr() as *const ::core::ffi::c_char;
            let floc = goal_floc(d_1);
            error(
                &ctx,
                floc.as_ref()
                    .map_or(::core::ptr::null(), |f| &f.floc as *const Floc),
                (strlen(d1_name) as size_t).wrapping_add(strlen(err) as size_t),
                b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    FmtArg::Str((d1_name) as *const ::core::ffi::c_char),
                    FmtArg::Str((err) as *const ::core::ffi::c_char),
                ],
            );
        }
        if any_failed != 0
            && status as ::core::ffi::c_uint == us_success as i32 as ::core::ffi::c_uint
        {
            status = UpdateStatus::None;
        }
        let needs_restart = match status as ::core::ffi::c_uint {
            1 => {
                for d_2 in &read_files {
                    let Some(fid) = d_2.dep.file else { continue };
                    let unloaded = ctx
                        .filenodes
                        .get(fid)
                        .map(|n| n.lock().expect("file node poisoned").unloaded)
                        .unwrap_or(false);
                    if unloaded {
                        let floc = goal_floc(d_2);
                        if load_file(
                            &ctx,
                            floc.as_ref()
                                .map_or(::core::ptr::null(), |f| &f.floc as *const Floc),
                            ::core::ptr::null_mut::<file>(),
                            0,
                        ) == 0
                        {
                            let mut nm = goal_name_bytes(&ctx, d_2);
                            nm.push(0);
                            return Err(fatal_err(
                                &ctx,
                                floc.as_ref()
                                    .map_or(::core::ptr::null(), |f| &f.floc as *const Floc),
                                strlen(nm.as_ptr() as *const ::core::ffi::c_char) as size_t,
                                b"%s: failed to load\0" as *const u8 as *const ::core::ffi::c_char,
                                &[FmtArg::Str((nm.as_ptr()) as *const ::core::ffi::c_char)],
                            ));
                        }
                        if let Some(node) = ctx.filenodes.get(fid) {
                            let mut guard = node.lock().expect("file node poisoned");
                            guard.unloaded = false;
                            guard.loaded = true;
                        }
                    }
                }
                false
            }
            3 => {
                let mut any_remade: i32 = 0;
                for (i_3, d_4) in read_files.iter().enumerate() {
                    let saved_mtime = makefile_mtimes.get(i_3).copied().unwrap_or(0);
                    let fid = d_4.dep.file;
                    let (updated, upd_status, last_mtime, name) =
                        match fid.and_then(|f| ctx.filenodes.get(f).map(|n| (f, n))) {
                            Some((_f, node)) => {
                                let guard = node.lock().expect("file node poisoned");
                                (
                                    guard.updated,
                                    guard.update_status,
                                    guard.last_mtime,
                                    guard.name.clone(),
                                )
                            }
                            None => (false, crate::file::UpdateStatus::None, 0, Vec::new()),
                        };
                    if updated {
                        if upd_status == crate::file::UpdateStatus::Success {
                            let mtime = if last_mtime == UNKNOWN_MTIME as uintmax_t {
                                f_mtime(&ctx, fid.unwrap(), false)?
                            } else {
                                last_mtime
                            };
                            any_remade |= (mtime != saved_mtime) as i32;
                        } else if !d_4.dep.flags.contains(crate::dep::DepFlags::DONTCARE) {
                            let mut nm = name.clone();
                            nm.push(0);
                            let floc = goal_floc(d_4);
                            error(
                                &ctx,
                                floc.as_ref()
                                    .map_or(::core::ptr::null(), |f| &f.floc as *const Floc),
                                strlen(nm.as_ptr() as *const ::core::ffi::c_char) as size_t,
                                b"failed to remake makefile '%s'\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[FmtArg::Str((nm.as_ptr()) as *const ::core::ffi::c_char)],
                            );
                            let mtime: uintmax_t = if last_mtime == UNKNOWN_MTIME as uintmax_t {
                                f_mtime(&ctx, fid.unwrap(), false)?
                            } else {
                                last_mtime
                            };
                            any_remade |= (mtime != NONEXISTENT_MTIME as uintmax_t
                                && mtime != saved_mtime)
                                as i32;
                            makefile_status = MAKE_FAILURE;
                            any_failed = 1;
                        }
                    } else if !d_4.dep.flags.contains(crate::dep::DepFlags::DONTCARE) {
                        let mut nm = goal_name_bytes(&ctx, d_4);
                        nm.push(0);
                        let dnm = nm.as_ptr() as *const ::core::ffi::c_char;
                        if d_4.dep.flags.contains(crate::dep::DepFlags::INCLUDED) {
                            let floc = goal_floc(d_4);
                            error(
                                &ctx,
                                floc.as_ref()
                                    .map_or(::core::ptr::null(), |f| &f.floc as *const Floc),
                                strlen(dnm) as size_t,
                                b"included makefile '%s' was not found\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[FmtArg::Str((dnm) as *const ::core::ffi::c_char)],
                            );
                        } else {
                            error(
                                &ctx,
                                ::core::ptr::null_mut::<Floc>(),
                                strlen(dnm) as size_t,
                                b"makefile '%s' was not found\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[FmtArg::Str((dnm) as *const ::core::ffi::c_char)],
                            );
                            any_failed = 1;
                        }
                    }
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
                                if mfidx == options.stdin_offset.get() {
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
                                if mfidx == options.stdin_offset.get() {
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
                if !ctx.directory_before_chdir.0.get().is_null() {
                    if chdir_c(ctx.directory_before_chdir.0.get()) < 0 {
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
                    return Err(fatal_err(
                        &ctx,
                        ::core::ptr::null_mut::<Floc>(),
                        0,
                        b"couldn't change back to original directory\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    ));
                }
            }
            restarts = restarts.wrapping_add(1);
            if 0x1_i32 & db_level(&ctx) != 0 {
                let mut p_3: *mut *const ::core::ffi::c_char;
                let mut msg = format!("Re-executing[{restarts}]:").into_bytes();
                p_3 = nargv;
                while !(*p_3).is_null() {
                    msg.push(b' ');
                    msg.extend_from_slice(::core::ffi::CStr::from_ptr(*p_3).to_bytes());
                    p_3 = p_3.offset(1_i32 as isize);
                }
                msg.push(b'\n');
                crate::output::trace_out(&msg);
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
                        if stdio_traced(&ctx) {
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
                    if stdio_traced(&ctx) {
                        b"-\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    restarts,
                );
                putenv(b);
            }
            // Empty make's stream buffers before the exec replaces the
            // process image — the Rust counterpart of the C fflush pair.
            {
                use std::io::Write;
                let _ = ctx.stdout.borrow_mut().flush();
                let _ = ctx.stderr.borrow_mut().flush();
            }
            osync_clear();
            jobserver_pre_child(&ctx, 1);
            exec_command(&ctx, nargv as *mut *mut ::core::ffi::c_char, environ);
            jobserver_post_child(&ctx, 1);
            temp_stdin_unlink(&ctx);
            _exit(127);
        }
        if any_failed != 0 {
            die_cleanup(&ctx, MAKE_FAILURE);
            return Err(BuildError::Failure);
        }
    }
    define_makeflags(&ctx, options, 0)?;
    ctx.always_make_flag.set(options.always_make.get());
    if restarts != 0 && !options.new_files.borrow().is_empty() {
        for nf in options.new_files.borrow().iter() {
            let name_bytes = ::std::ffi::CStr::from_ptr(nf.as_ptr()).to_bytes().to_vec();
            let f_5 = enter_file(&ctx, &name_bytes);
            if let Some(node) = ctx.filenodes.get(f_5) {
                let mut guard = node.lock().expect("file node poisoned");
                guard.mtime_before_update = new_file_mtime();
                guard.last_mtime = guard.mtime_before_update;
            }
        }
    }
    temp_stdin_unlink(&ctx);
    if options.goals.borrow().is_empty() {
        let mut p_6: *mut ::core::ffi::c_char;
        let default_goal_var = ctx.default_goal_var.0.get();
        if (*default_goal_var).recursive() != 0 {
            p_6 = expand_string_buf(
                &ctx,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                (*default_goal_var).value,
                SIZE_MAX as size_t,
            )?;
        } else {
            p_6 = variable_buffer_output(
                &ctx,
                ctx.variable_buffer.ptr(),
                (*default_goal_var).value,
                strlen((*default_goal_var).value) as size_t,
            );
            *p_6 = 0;
            p_6 = ctx.variable_buffer.ptr();
        }
        assert!(
            !p_6.is_null(),
            "variable_buffer must be initialized by this point in main_0"
        );
        if *p_6 as i32 != 0 {
            let mut f_6: Option<crate::file::FileId> =
                lookup_file(&ctx, ::std::ffi::CStr::from_ptr(p_6).to_bytes());
            if f_6.is_none() {
                let names = parse_file_seq(
                    &ctx,
                    &raw mut p_6,
                    ::core::mem::size_of::<NameSeq>() as size_t,
                    MAP_NUL,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    PARSEFS_NONE,
                )?;
                if !names.is_empty() {
                    if names.len() > 1 {
                        return Err(fatal_err(
                            &ctx,
                            ::core::ptr::null_mut::<Floc>(),
                            0,
                            b".DEFAULT_GOAL contains more than one target\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        ));
                    }
                    f_6 = Some(enter_file(&ctx, &names[0].name));
                }
            }
            if let Some(fid) = f_6 {
                options.goals.borrow_mut().push(goaldep_for_file(fid));
            }
        }
    }
    if options.goals.borrow().is_empty() {
        let v_2: *mut variable = lookup_named(&ctx, b"MAKEFILE_LIST\0")?;
        if !v_2.is_null()
            && !(*v_2).value.is_null()
            && *(*v_2).value.offset(0_i32 as isize) as i32 != 0
        {
            return Err(fatal_err(
                &ctx,
                ::core::ptr::null_mut::<Floc>(),
                0,
                b"No targets\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            ));
        }
        return Err(fatal_err(
            &ctx,
            ::core::ptr::null_mut::<Floc>(),
            0,
            b"No targets specified and no makefile found\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        ));
    }
    // Diagnostics tap (MAKERS_DEPGRAPH): snapshot the fully-read graph before
    // shuffling touches goal order and before the update walk mutates state.
    crate::depgraph::dump_graph_if_requested(&ctx, &options.goals.borrow());
    crate::shuffle::shuffle_goals_recursive(&ctx, &mut options.goals.borrow_mut());
    if 0x1_i32 & db_level(&ctx) != 0 {
        crate::output::trace_out(b"Updating goal targets....\n");
    }
    match update_goal_chain(&ctx, &mut options.goals.borrow_mut())? as ::core::ffi::c_uint {
        2 => {
            makefile_status = MAKE_TROUBLE;
        }
        3 => {
            makefile_status = MAKE_FAILURE;
        }
        1 | 0 | _ => {}
    }
    // Diagnostics tap (MAKERS_DEPGRAPH_POST): snapshot the resolved graph —
    // implicit rules matched, provenance recorded — now that the walk is done.
    crate::depgraph::dump_graph_post_if_requested(&ctx, &options.goals.borrow());
    // Build plugins (#633/#636/#644): the `makers:plugin` analysis pass runs
    // at the same point as the depgraph tap above, over the same resolved
    // graph. Unlike that tap it can affect the exit status — but only for a
    // plugin whose manifest declares `failure-policy: fatal` *and* which was
    // granted `fail-build`; every other failure is reported and survived.
    #[cfg(feature = "wasmtime")]
    if crate::plugin::run_plugins_if_requested(&ctx, &options.goals.borrow())
        && makefile_status == MAKE_SUCCESS
    {
        makefile_status = MAKE_FAILURE;
    }
    if ctx.clock_skew_detected.get() {
        error(
            &ctx,
            ::core::ptr::null_mut::<Floc>(),
            0,
            b"warning: clock skew detected: your build may be incomplete\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    }
    die_cleanup(&ctx, makefile_status);
    crate::build_result::result_from_status(makefile_status)
}
unsafe fn handle_non_switch_argument(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
    arg: *const ::core::ffi::c_char,
    origin: variable_origin,
) -> Result<::core::ffi::c_uint, crate::build_result::BuildError> {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    if *arg.offset(0_i32 as isize) as i32 == '-' as i32 && *arg.offset(1_i32 as isize) as i32 == 0 {
        return Ok(0);
    }
    let v = try_variable_definition(ctx, ::core::ptr::null::<Floc>(), arg, origin, s_global)?;
    if !v.is_null() {
        let mut cv: *mut CommandVariable;
        cv = ctx.command_variables.0.get();
        while !cv.is_null() {
            if (*cv).variable == v {
                break;
            }
            cv = (*cv).next;
        }
        if cv.is_null() {
            cv = xmalloc(::core::mem::size_of::<CommandVariable>() as size_t)
                as *mut CommandVariable;
            (*cv).variable = v;
            (*cv).next = ctx.command_variables.0.get();
            ctx.command_variables.0.set(cv);
        }
    } else if *arg.offset(0_i32 as isize) as i32 != 0
        && origin as ::core::ffi::c_uint == o_command as i32 as ::core::ffi::c_uint
    {
        if strcmp(arg, b".WAIT\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            return Ok(1);
        }
        let fname_bytes = ::std::ffi::CStr::from_ptr(expand_command_line_file(ctx, arg)?)
            .to_bytes()
            .to_vec();
        let f = enter_file(ctx, &fname_bytes);
        if let Some(node) = ctx.filenodes.get(f) {
            node.lock().expect("file node poisoned").cmd_target = true;
        }
        options.goals.borrow_mut().push(goaldep_for_file(f));
        // NUL-terminated target name for the MAKECMDGOALS accumulation below.
        let mut fname_c = fname_bytes.clone();
        fname_c.push(0);
        let fname_ptr = fname_c.as_ptr() as *const ::core::ffi::c_char;
        let gv: *mut variable;
        let value: *const ::core::ffi::c_char;
        gv = lookup_named(ctx, b"MAKECMDGOALS\0")?;
        if gv.is_null() {
            value = fname_ptr;
        } else {
            let oldlen: size_t;
            let newlen: size_t;
            let vp: *mut ::core::ffi::c_char;
            oldlen = strlen((*gv).value) as size_t;
            newlen = strlen(fname_ptr) as size_t;
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
                fname_ptr as *const ::core::ffi::c_void,
                (newlen as size_t).wrapping_add(1),
            );
            value = vp;
        }
        crate::variable::define_named(ctx, b"MAKECMDGOALS\0", value, o_default, 0)?;
    }
    Ok(0)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn reset_makeflags(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
    origin: variable_origin,
) -> Result<(), crate::build_result::BuildError> {
    options.env_overrides.set(false);
    decode_env_switches(
        ctx,
        options,
        b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        origin,
    )?;
    {
        let include_dirs = options.include_dirs.borrow();
        let inc_paths: Vec<std::path::PathBuf> = include_dirs
            .iter()
            .map(|s| {
                use std::os::unix::ffi::OsStrExt;
                std::path::PathBuf::from(std::ffi::OsStr::from_bytes(s.as_bytes()))
            })
            .collect();
        construct_include_path(ctx, &inc_paths)?;
    }
    disable_builtins(ctx, options)?;
    define_makeflags(ctx, options, opt_rebuilding_makefiles(ctx) as i32)?;
    Ok(())
}
/// Switch chars whose `CommandSwitch.type_0` is `flag`/`flag_off` and which
/// share their underlying `Options` storage with a counterpart char (the
/// negation aliases): whichever of the pair appears *last* on the command
/// line wins, matching `opt_set_flag`'s "later assignment overwrites" getopt
/// loop semantics. Mirrors the char groupings already hardcoded in
/// `opt_set_flag`/`opt_origin_cell` (`'k'|'S'`, `'w'|CHAR_MAX+4`,
/// `'s'|CHAR_MAX+8`) -- there are only these three pairs in the whole table.
const FLAG_PAIRS: [(i32, i32); 3] = [
    ('k' as i32, 'S' as i32),
    ('w' as i32, CHAR_MAX + 4),
    ('s' as i32, CHAR_MAX + 8),
];

/// Every optional-argument switch (`CommandSwitch.noarg_value` non-null) and
/// its bare spellings (short + long name + any optional-argument alias). Used
/// by [`normalize_argv_for_clap`] to recognize a bare occurrence before clap
/// parses the token stream.
struct OptionalArgSwitch {
    c: i32,
    short: Option<String>,
    longs: Vec<String>,
    /// Only `-j`/`--jobs` and `-l`/`--load-average`/`--max-load` accept a
    /// *separate* following argv token as their value (the hand-rolled
    /// numeric lookahead in the original `decode_switches`); every other
    /// optional-arg switch never consumes a following token.
    numeric_lookahead: bool,
}

fn optional_arg_switches(switches: &[CommandSwitch]) -> Vec<OptionalArgSwitch> {
    let mut out = Vec::new();
    for cs in switches {
        if cs.c == 0 || cs.noarg_value.is_null() {
            continue;
        }
        let short = if cs.c <= CHAR_MAX {
            Some(format!("-{}", cs.c as u8 as char))
        } else {
            None
        };
        let mut longs = Vec::new();
        if !cs.long_name.is_null() {
            // SAFETY: `long_name` was just checked non-null; every table
            // entry's `long_name` is either null or a valid NUL-terminated
            // `&'static str` literal.
            if let Ok(s) = unsafe { ::core::ffi::CStr::from_ptr(cs.long_name) }.to_str() {
                if !s.is_empty() {
                    longs.push(format!("--{}", s));
                }
            }
        }
        for alias in LONG_OPTION_ALIASES.iter() {
            if alias.val == cs.c && alias.has_arg == optional_argument {
                // SAFETY: every `LONG_OPTION_ALIASES` entry's `name` is a
                // valid NUL-terminated `&'static str` literal.
                if let Ok(s) = unsafe { ::core::ffi::CStr::from_ptr(alias.name) }.to_str() {
                    longs.push(format!("--{}", s));
                }
            }
        }
        out.push(OptionalArgSwitch {
            c: cs.c,
            short,
            longs,
            numeric_lookahead: cs.c == 'j' as i32 || cs.c == 'l' as i32,
        });
    }
    out
}

/// Sentinel substituted for a *bare* optional-arg switch occurrence (see
/// [`normalize_argv_for_clap`]), so after clap parses it, "no value was
/// given" stays distinguishable from a genuine explicit empty value
/// (`--debug=`) -- the two must be treated differently (see
/// [`apply_value_switch`] and the `-j`/`-l` handling in [`decode_switches`]).
/// argv tokens can never contain a NUL byte (they're NUL-terminated C
/// strings at the OS level), so this can never collide with real input.
const NOARG_SENTINEL: &str = "\0";

/// Rewrites `tokens` so every *bare* occurrence of an optional-argument
/// switch carries an explicit value before clap ever sees it. clap's own
/// `num_args(0..=1)` handling (unlike `getopt_long`'s) always tries to
/// consume the *next* token as the value when one is bare, which is wrong
/// for every optional-arg switch except `-j`/`-l` -- and even for those two,
/// only when the next token is all-numeric (`-j`) or numeric/`.`-led (`-l`),
/// replicating the original hand-rolled lookahead exactly. Every other bare
/// occurrence is rewritten to [`NOARG_SENTINEL`] (`-j=\0`/`--debug=\0`) so
/// clap records "given, no value" instead of grabbing an unrelated following
/// token, while staying distinguishable from a real explicit empty value.
fn normalize_argv_for_clap(
    switches: &[CommandSwitch],
    tokens: &[::std::ffi::OsString],
) -> Vec<::std::ffi::OsString> {
    use std::os::unix::ffi::OsStrExt;
    // Byte-level `"{a}={b}"` concatenation, avoiding any UTF-8 assumption
    // about `a`/`b` (a switch value like `-l`'s can be arbitrary bytes past
    // its first char -- the original hand-rolled lookahead is byte-based,
    // not string-based).
    fn concat_eq(a: &::std::ffi::OsStr, b: &[u8]) -> ::std::ffi::OsString {
        let mut buf = a.as_bytes().to_vec();
        buf.push(b'=');
        buf.extend_from_slice(b);
        ::std::ffi::OsStr::from_bytes(&buf).to_os_string()
    }

    let opt_args = optional_arg_switches(switches);
    let mut out = Vec::with_capacity(tokens.len());
    let mut i = 0;
    while i < tokens.len() {
        let tok = &tokens[i];
        let matched = opt_args.iter().find(|o| {
            o.short
                .as_deref()
                .is_some_and(|s| tok.as_os_str() == ::std::ffi::OsStr::new(s))
                || o.longs
                    .iter()
                    .any(|l| tok.as_os_str() == ::std::ffi::OsStr::new(l.as_str()))
        });
        let Some(o) = matched else {
            out.push(tok.clone());
            i += 1;
            continue;
        };
        if o.numeric_lookahead {
            if let Some(next) = tokens.get(i + 1) {
                let bytes = next.as_bytes();
                let consume = if o.c == 'j' as i32 {
                    !bytes.is_empty() && bytes.iter().all(u8::is_ascii_digit)
                } else {
                    bytes
                        .first()
                        .is_some_and(|&b| b.is_ascii_digit() || b == b'.')
                };
                if consume {
                    out.push(concat_eq(tok.as_os_str(), bytes));
                    i += 2;
                    continue;
                }
            }
        }
        out.push(concat_eq(tok.as_os_str(), NOARG_SENTINEL.as_bytes()));
        i += 1;
    }
    out
}

/// Builds a [`clap::Command`] from the `switches` table: one `Arg` per
/// switch (short/long name + any long-option aliases from
/// `LONG_OPTION_ALIASES`), plus a trailing catch-all positional for
/// everything else (targets, `VAR=value` assignments). Replaces the former
/// `build_getopt_tables`/`getopt_long` pair -- rebuilt fresh on every
/// `decode_switches` call, same as the table it replaced.
fn build_clap_command(switches: &[CommandSwitch]) -> clap::Command {
    use clap::{builder::OsStringValueParser, Arg, ArgAction, Command};
    let mut cmd = Command::new("make")
        .no_binary_name(true)
        .disable_help_flag(true)
        .disable_version_flag(true);
    for cs in switches {
        if cs.c == 0 {
            continue;
        }
        let mut arg = Arg::new(format!("c{}", cs.c));
        if cs.c <= CHAR_MAX {
            arg = arg.short(cs.c as u8 as char);
        }
        if !cs.long_name.is_null() {
            // SAFETY: `long_name` was just checked non-null; every table
            // entry's `long_name` is either null or a valid NUL-terminated
            // `&'static str` literal.
            if let Ok(s) = unsafe { ::core::ffi::CStr::from_ptr(cs.long_name) }.to_str() {
                if !s.is_empty() {
                    arg = arg.long(s.to_string());
                }
            }
        }
        arg = match cs.type_0 {
            t if t == flag || t == flag_off || t == ignore => arg.action(ArgAction::Count),
            _ if cs.noarg_value.is_null() => {
                arg.action(ArgAction::Append)
                    .num_args(1)
                    .value_parser(OsStringValueParser::new())
            }
            _ => {
                arg.action(ArgAction::Append)
                    .num_args(0..=1)
                    .value_parser(OsStringValueParser::new())
            }
        };
        cmd = cmd.arg(arg);
    }
    for alias in LONG_OPTION_ALIASES.iter() {
        // SAFETY: every `LONG_OPTION_ALIASES` entry's `name` is a valid
        // NUL-terminated `&'static str` literal.
        if let Ok(name) = unsafe { ::core::ffi::CStr::from_ptr(alias.name) }.to_str() {
            let name = name.to_string();
            cmd = cmd.mut_arg(format!("c{}", alias.val), move |a| a.alias(name));
        }
    }
    cmd.arg(
        Arg::new("__rest")
            .action(ArgAction::Append)
            .num_args(0..)
            .allow_hyphen_values(false)
            .value_parser(OsStringValueParser::new()),
    )
}

/// Applies a single occurrence of a `string`/`strlist`/`filename`-type
/// switch, given its already-tokenized (possibly empty, meaning "no value
/// given") argument. Extracted from the getopt-loop body virtually
/// unchanged; only the argument's origin (an `ArgMatches` value instead of a
/// `getopt_long` `coptarg` pointer) differs.
#[allow(clippy::too_many_arguments)]
fn apply_value_switch(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
    cs: &CommandSwitch,
    raw_value: &::std::ffi::OsStr,
    doit: bool,
    cs_origin: Option<&::core::cell::Cell<variable_origin>>,
    origin: variable_origin,
    bad: &mut i32,
) -> Result<(), crate::build_result::BuildError> {
    use std::os::unix::ffi::OsStrExt;
    if !doit {
        return Ok(());
    }
    // Resolve the option argument. A bare occurrence (no value given at
    // all -- flagged by `NOARG_SENTINEL`, see `normalize_argv_for_clap`)
    // falls back to the switch's `noarg_value`; a genuine *explicit* empty
    // value (`--debug=`, `-f ''`) is always an error, matching
    // `getopt_long`'s `*coptarg == 0` check regardless of whether this
    // switch also has a `noarg_value` substitute. Built from raw bytes (not
    // through `str`) so a non-UTF-8 filename argument round-trips exactly,
    // matching the original getopt-based `CString` path.
    let resolved: ::std::ffi::CString = if raw_value == ::std::ffi::OsStr::new(NOARG_SENTINEL) {
        // SAFETY: `NOARG_SENTINEL` is only ever produced by
        // `normalize_argv_for_clap` for a switch listed in
        // `optional_arg_switches`, which filters on `noarg_value` being
        // non-null; every table entry's `noarg_value`, when non-null, is a
        // valid NUL-terminated `&'static str` literal.
        unsafe { ::core::ffi::CStr::from_ptr(cs.noarg_value as *const ::core::ffi::c_char) }
            .to_owned()
    } else if raw_value.is_empty() {
        let mut opt: [::core::ffi::c_char; 2] = [0; 2];
        let op: *const ::core::ffi::c_char = if cs.c <= CHAR_MAX {
            opt[0_i32 as usize] = cs.c as ::core::ffi::c_char;
            &raw mut opt as *mut ::core::ffi::c_char
        } else {
            // SAFETY: every table entry with `c > CHAR_MAX` (long-only) has
            // a non-null `long_name`, since it has no single-char spelling.
            cs.long_name
        };
        // SAFETY: `op` is either `opt` (a local, NUL-terminated 2-byte
        // buffer) or `cs.long_name` (non-null and NUL-terminated per above).
        unsafe {
            error(
                ctx,
                NILF,
                strlen(op) as size_t,
                b"the '%s%s' option requires a non-empty string argument\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    FmtArg::Str(if cs.c <= CHAR_MAX {
                        b"-\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"--\0" as *const u8 as *const ::core::ffi::c_char
                    }),
                    FmtArg::Str(op),
                ],
            );
        }
        *bad = 1;
        return Ok(());
    } else {
        ::std::ffi::CString::new(raw_value.as_bytes()).unwrap_or_default()
    };
    let coptarg = resolved.as_ptr();
    if cs.type_0 == string {
        let s = resolved.to_string_lossy().into_owned();
        opt_set_str(options, cs.c, s);
        if let Some(oc) = cs_origin {
            oc.set(origin);
        }
    } else if cs.c == CHAR_MAX + 1 {
        // `--debug` accumulates its args into a `Vec<CString>`, skipping an
        // exact duplicate of an already-stored value (matching the original
        // stringlist dedup logic).
        let mut db_flags = options.db_flags.borrow_mut();
        let duplicate = db_flags.iter().any(|e| e.as_c_str() == resolved.as_c_str());
        if !duplicate {
            db_flags.push(resolved.clone());
            if let Some(oc) = cs_origin {
                oc.set(origin);
            }
        }
    } else {
        // List options (`strlist`/`filename`) store owned `CString`s in a
        // `Vec` on `Options`. Dispatch on the switch char to the relevant
        // `Vec`.
        let mut list = match cs.c {
            c if c == 'C' as i32 => options.directories.borrow_mut(),
            c if c == 'f' as i32 || c == TEMP_STDIN_OPT => options.makefiles.borrow_mut(),
            c if c == 'I' as i32 => options.include_dirs.borrow_mut(),
            c if c == 'o' as i32 => options.old_files.borrow_mut(),
            c if c == 'W' as i32 => options.new_files.borrow_mut(),
            c if c == 'E' as i32 => options.eval_strings.borrow_mut(),
            c if c == WARN_OPT => options.warn_flags.borrow_mut(),
            _ => unreachable!("non-list option in list arm"),
        };
        // Skip a value already present (but -f and --warn allow duplicates).
        let duplicate = if cs.c != 'f' as i32 && cs.c != WARN_OPT {
            list.iter().any(|e| e.as_c_str() == resolved.as_c_str())
        } else {
            false
        };
        if !duplicate {
            // `strlist` stores the raw arg; `filename` stores the expanded
            // path (or the strcache entry for the --temp-stdin placeholder).
            let stored: ::std::ffi::CString = if cs.type_0 == strlist {
                resolved.clone()
            } else if cs.c == TEMP_STDIN_OPT {
                if options.stdin_offset.get() > 0 {
                    // SAFETY: `fatal_err` requires a valid NUL-terminated
                    // format string with no `%` conversions beyond the given
                    // args; this literal has none.
                    return Err(unsafe {
                        fatal_err(
                            ctx,
                            NILF,
                            0,
                            b"INTERNAL: multiple --temp-stdin options provided!\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        )
                    });
                }
                options.stdin_offset.set(list.len() as i32);
                // SAFETY: `strcache_add` requires a valid NUL-terminated C
                // string; `coptarg` is `resolved.as_ptr()`, a live `CString`.
                // Its return value is a strcache-owned, valid NUL-terminated
                // string for the process lifetime.
                let cached = unsafe { strcache_add(ctx, coptarg) };
                ctx.temp_stdin_name.0.set(cached);
                unsafe { ::core::ffi::CStr::from_ptr(cached) }.to_owned()
            } else {
                // SAFETY: `expand_command_line_file` requires a valid
                // NUL-terminated C string (`coptarg`, as above) and returns
                // one.
                unsafe { ::core::ffi::CStr::from_ptr(expand_command_line_file(ctx, coptarg)?) }
                    .to_owned()
            };
            list.push(stored);
            if let Some(oc) = cs_origin {
                oc.set(origin);
            }
        }
    }
    Ok(())
}

fn decode_switches(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
    tokens: &[::std::ffi::OsString],
    origin: variable_origin,
) -> Result<(), crate::build_result::BuildError> {
    let mut bad: i32 = 0;
    let switches_snapshot: Vec<CommandSwitch> = options.switches.borrow().to_vec();

    let normalized = normalize_argv_for_clap(&switches_snapshot, tokens);
    let mut retry_tokens = normalized;
    let matches = loop {
        match build_clap_command(&switches_snapshot).try_get_matches_from(retry_tokens.clone()) {
            Ok(m) => break Some(m),
            Err(e) => {
                bad = 1;
                // Best-effort recovery matching getopt_long's per-token
                // tolerance: drop the one token clap rejected and retry, so
                // an unrelated valid switch elsewhere on the same command
                // line (or MAKEFLAGS-derived token list) still applies.
                let culprit = e
                    .get(clap::error::ContextKind::InvalidArg)
                    .map(|v| v.to_string());
                if let Some(c) = culprit.and_then(|c| {
                    retry_tokens
                        .iter()
                        .position(|t| t.to_str() == Some(c.as_str()))
                }) {
                    retry_tokens.remove(c);
                    continue;
                }
                break None;
            }
        }
    };
    let Some(matches) = matches else {
        // Couldn't isolate the offending token (e.g. it was consumed as
        // another option's value) -- nothing more to recover from this
        // batch.
        if bad != 0 && origin == o_command {
            // Bad switch: print the usage table and run the same end-of-run
            // cleanup the retired `die` did, then hand the failure back to
            // `main_0` instead of exiting here (#432 Phase B, #537). The
            // cleanup still runs at this point rather than in the caller, so
            // the ordering of its output against the usage table is unchanged.
            print_usage(ctx, options, bad);
            die_cleanup(ctx, MAKE_FAILURE);
            return Err(crate::build_result::BuildError::Failure);
        }
        return Ok(());
    };

    // Flags (`flag`/`flag_off`/`ignore`): a bare switch applies once; one of
    // the three negation pairs (`FLAG_PAIRS`) applies whichever appeared
    // *last*, matching `opt_set_flag`'s overwrite-on-every-occurrence
    // getopt-loop semantics.
    let mut paired: std::collections::HashSet<i32> = std::collections::HashSet::new();
    for &(a, b) in FLAG_PAIRS.iter() {
        paired.insert(a);
        paired.insert(b);
        let a_last = matches
            .indices_of(format!("c{a}").as_str())
            .and_then(|mut ix| ix.next_back())
            .filter(|_| matches.get_count(format!("c{a}").as_str()) > 0);
        let b_last = matches
            .indices_of(format!("c{b}").as_str())
            .and_then(|mut ix| ix.next_back())
            .filter(|_| matches.get_count(format!("c{b}").as_str()) > 0);
        let winner = match (a_last, b_last) {
            (Some(ai), Some(bi)) => Some(if bi > ai { b } else { a }),
            (Some(_), None) => Some(a),
            (None, Some(_)) => Some(b),
            (None, None) => None,
        };
        // `k`/`S` (and each other pair) share the same origin cell, so
        // `doit` is identical for both -- but the original getopt loop marks
        // `specified` for *every* occurrence it dispatches, even one whose
        // value is later overwritten by the other half of the pair. Mark
        // both if both occurred; only the winner's value actually applies.
        for &c in &[a, b] {
            let occurred = if c == a {
                a_last.is_some()
            } else {
                b_last.is_some()
            };
            if !occurred {
                continue;
            }
            let Some(cs) = switches_snapshot.iter().find(|cs| cs.c == c) else {
                continue;
            };
            let cs_origin = opt_origin_cell(options, cs.c);
            let doit = origin == o_command
                || (cs.env() != 0 && (cs_origin.is_none() || origin >= cs_origin.unwrap().get()));
            if doit {
                options.switches.borrow_mut()
                    [switches_snapshot.iter().position(|s| s.c == cs.c).unwrap()]
                .set_specified(1);
            }
        }
        if let Some(c) = winner {
            if let Some(cs) = switches_snapshot.iter().find(|cs| cs.c == c) {
                let cs_origin = opt_origin_cell(options, cs.c);
                let doit = origin == o_command
                    || (cs.env() != 0
                        && (cs_origin.is_none() || origin >= cs_origin.unwrap().get()));
                if doit {
                    let on = cs.type_0 == flag;
                    opt_set_flag(options, cs.c, on);
                    if let Some(oc) = cs_origin {
                        oc.set(origin);
                    }
                }
            }
        }
    }
    for cs in switches_snapshot.iter() {
        if cs.c == 0 || paired.contains(&cs.c) {
            continue;
        }
        if !matches!(cs.type_0, t if t == flag || t == flag_off || t == ignore) {
            continue;
        }
        if matches.get_count(format!("c{}", cs.c).as_str()) == 0 {
            continue;
        }
        let cs_origin = opt_origin_cell(options, cs.c);
        let doit = origin == o_command
            || (cs.env() != 0 && (cs_origin.is_none() || origin >= cs_origin.unwrap().get()));
        if doit {
            options.switches.borrow_mut()
                [switches_snapshot.iter().position(|s| s.c == cs.c).unwrap()]
            .set_specified(1);
            if cs.type_0 != ignore {
                let on = cs.type_0 == flag;
                opt_set_flag(options, cs.c, on);
                if let Some(oc) = cs_origin {
                    oc.set(origin);
                }
            }
        }
    }

    // Value-taking switches: apply every occurrence, in the order clap
    // recorded them (matching each switch's own occurrence order in argv).
    for cs in switches_snapshot.iter() {
        if cs.c == 0 || !matches!(cs.type_0, string | strlist | filename) {
            continue;
        }
        let id = format!("c{}", cs.c);
        let Some(values) = matches.get_many::<::std::ffi::OsString>(&id) else {
            continue;
        };
        let cs_origin = opt_origin_cell(options, cs.c);
        for raw_value in values {
            let doit = origin == o_command
                || (cs.env() != 0 && (cs_origin.is_none() || origin >= cs_origin.unwrap().get()));
            if doit {
                options.switches.borrow_mut()
                    [switches_snapshot.iter().position(|s| s.c == cs.c).unwrap()]
                .set_specified(1);
            }
            apply_value_switch(
                ctx, options, cs, raw_value, doit, cs_origin, origin, &mut bad,
            )?;
        }
    }

    // `-j`/`--jobs` (positive_int) and `-l`/`--load-average` (floating): only
    // one occurrence is meaningful (repeats simply overwrite, same as the
    // original getopt loop re-dispatching each occurrence in turn).
    for cs in switches_snapshot.iter() {
        if cs.c != 'j' as i32 && cs.c != 'l' as i32 {
            continue;
        }
        let id = format!("c{}", cs.c);
        let Some(mut values) = matches.get_many::<::std::ffi::OsString>(&id) else {
            continue;
        };
        let cs_origin = opt_origin_cell(options, cs.c);
        for raw_value in values.by_ref() {
            let doit = origin == o_command
                || (cs.env() != 0 && (cs_origin.is_none() || origin >= cs_origin.unwrap().get()));
            if doit {
                options.switches.borrow_mut()
                    [switches_snapshot.iter().position(|s| s.c == cs.c).unwrap()]
                .set_specified(1);
            }
            if !doit {
                continue;
            }
            use std::os::unix::ffi::OsStrExt;
            let is_sentinel = raw_value.as_os_str() == ::std::ffi::OsStr::new(NOARG_SENTINEL);
            if cs.c == 'j' as i32 {
                if is_sentinel {
                    // SAFETY: `-j`'s table entry's `noarg_value` is always
                    // set (to `&inf_jobs`), a valid `c_uint`.
                    let n = unsafe { *(cs.noarg_value as *const ::core::ffi::c_uint) };
                    options.arg_job_slots.set(Some(n));
                } else {
                    let cstr = ::std::ffi::CString::new(raw_value.as_bytes()).unwrap_or_default();
                    let n = make_toui(&cstr).unwrap_or(0);
                    if n == 0 {
                        // SAFETY: `error` requires a valid NUL-terminated
                        // format string with args matching its `%`
                        // conversions; this literal has one `%c` matched by
                        // one `FmtArg::Int`.
                        unsafe {
                            error(
                                ctx,
                                NILF,
                                0,
                                b"the '-%c' option requires a positive integer argument\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                &[FmtArg::Int(cs.c as i64)],
                            );
                        }
                        bad = 1;
                    } else {
                        options.arg_job_slots.set(Some(n));
                    }
                }
            } else {
                let v = if is_sentinel {
                    // SAFETY: `-l`'s table entry's `noarg_value` is always
                    // set (to `&default_load_average`), a valid `c_double`.
                    unsafe { *(cs.noarg_value as *const ::core::ffi::c_double) }
                } else {
                    let cstr = ::std::ffi::CString::new(raw_value.as_bytes()).unwrap_or_default();
                    // SAFETY: `atof` requires a valid NUL-terminated C
                    // string; `cstr` is a live `CString`.
                    unsafe { atof(cstr.as_ptr()) }
                };
                options.max_load_average.set(v);
            }
            if let Some(oc) = cs_origin {
                oc.set(origin);
            }
        }
    }

    // Non-switch tokens (targets, `VAR=value`), in original relative order.
    if let Some(rest) = matches.get_many::<::std::ffi::OsString>("__rest") {
        use std::os::unix::ffi::OsStrExt;
        let mut found_wait: ::core::ffi::c_uint = 0;
        for tok in rest {
            let ctok = ::std::ffi::CString::new(tok.as_bytes()).unwrap_or_default();
            let prior_found_wait = found_wait;
            // SAFETY: `handle_non_switch_argument` requires a valid
            // NUL-terminated C string; `ctok` is a live `CString`.
            found_wait =
                unsafe { handle_non_switch_argument(ctx, options, ctok.as_ptr(), origin)? };
            if prior_found_wait != 0 {
                if let Some(last) = options.goals.borrow_mut().last_mut() {
                    last.dep.wait_here = true;
                }
            }
        }
    }

    // SAFETY: none of these take raw pointers or impose a precondition
    // beyond a valid `&ExecContext`/`&Options`, which we have. The
    // bad-switch path runs the same end-of-run cleanup the retired `die`
    // did and then returns the failure to `main_0` (#432 Phase B, #537).
    unsafe {
        if bad != 0 && origin == o_command {
            print_usage(ctx, options, bad);
            die_cleanup(ctx, MAKE_FAILURE);
            return Err(crate::build_result::BuildError::Failure);
        }
        decode_debug_flags(ctx, options)?;
        decode_output_sync_flags(ctx, options)?;
    }
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
    // SAFETY: `reset_env_override` takes no raw pointers and imposes no
    // precondition beyond a valid `&ExecContext`, which we have.
    unsafe {
        reset_env_override(ctx);
    }
    Ok(())
}
/// Tokenizes an already-expanded `MAKEFLAGS`/`GNUMAKEFLAGS` value on
/// whitespace (per [`stopchar_map`]'s `MAP_BLANK` bit), honoring
/// backslash-escapes exactly as the original buffer-based splitter did: a
/// backslash followed by any character copies that character literally
/// (never testing it against the stopchar map), and a run of consecutive
/// blank bytes ends the current word and is otherwise skipped. Always
/// produces at least one (possibly empty) trailing word, matching the
/// original's unconditional final `argc += 1`.
fn split_makeflags_value(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut words = Vec::new();
    let mut cur = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\\' && i + 1 < bytes.len() {
            cur.push(bytes[i + 1]);
            i += 2;
            continue;
        }
        if stopchar_map()[b as usize] & 0x2 != 0 {
            words.push(::core::mem::take(&mut cur));
            i += 1;
            while i < bytes.len() && stopchar_map()[bytes[i] as usize] & 0x2 != 0 {
                i += 1;
            }
            continue;
        }
        cur.push(b);
        i += 1;
    }
    words.push(cur);
    words
}

unsafe fn decode_env_switches(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
    envar: *const ::core::ffi::c_char,
    len: size_t,
    origin: variable_origin,
) -> Result<(), crate::build_result::BuildError> {
    let value = expand_variable_buf(
        ctx,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        envar,
        len,
    )?;
    let mut bytes = ::core::ffi::CStr::from_ptr(value).to_bytes();
    while !bytes.is_empty() && stopchar_map()[bytes[0] as usize] & (0x2 | 0x4) != 0 {
        bytes = &bytes[1..];
    }
    if bytes.is_empty() {
        return Ok(());
    }
    use std::os::unix::ffi::OsStrExt;
    let words = split_makeflags_value(bytes);
    let mut tokens: Vec<::std::ffi::OsString> = words
        .iter()
        .map(|w| ::std::ffi::OsStr::from_bytes(w).to_os_string())
        .collect();
    // Legacy dash-optional rewrite: a `MAKEFLAGS` value written without a
    // leading dash (e.g. `MAKEFLAGS=ik`) is bundled as short flags, but only
    // when its first word has no `=` (so `FOO=bar`-shaped content is left
    // alone).
    if let Some(first) = tokens.first_mut() {
        let raw = first.as_bytes();
        if !raw.starts_with(b"-") && !raw.contains(&b'=') {
            let mut rewritten = vec![b'-'];
            rewritten.extend_from_slice(raw);
            *first = ::std::ffi::OsStr::from_bytes(&rewritten).to_os_string();
        }
    }
    decode_switches(ctx, options, &tokens, origin)
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
unsafe fn clear_builtin_rules(
    ctx: &crate::execctx::ExecContext,
) -> Result<(), crate::build_result::BuildError> {
    if let Some(suffix_file) = crate::file::lookup_file(ctx, b".SUFFIXES") {
        if let Some(node) = ctx.filenodes.get(suffix_file) {
            let mut guard = node.lock().expect("file node poisoned");
            if guard.builtin {
                guard.deps.clear();
            }
        }
    }
    crate::variable::define_named(
        ctx,
        b"SUFFIXES\0",
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
    )?;
    Ok(())
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn disable_builtins(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
) -> Result<(), crate::build_result::BuildError> {
    if options.no_builtin_variables.get() {
        options.no_builtin_rules.set(true);
    }
    if options.no_builtin_rules.get() && !options.prev_no_builtin_rules.get() {
        options.prev_no_builtin_rules.set(true);
        clear_builtin_rules(ctx)?;
    }
    if options.no_builtin_variables.get() && !options.prev_no_builtin_variables.get() {
        options.prev_no_builtin_variables.set(true);
        undefine_default_variables(ctx)?;
    }
    Ok(())
}
/// Define `name` as the terminal path for `fd` — or the literal `true` when the
/// name is unavailable — and mark it exported. `MAKE_TERMOUT` and
/// `MAKE_TERMERR` are seeded with exactly this shape.
///
/// # Safety
///
/// Inherits `define_variable_in_set`'s contract; `name` must end in a NUL byte
/// and `fd` must be a valid descriptor.
unsafe fn define_tty_var(
    ctx: &crate::execctx::ExecContext,
    name: &'static [u8],
    fd: i32,
) -> Result<(), crate::build_result::BuildError> {
    let tty: *const ::core::ffi::c_char = ttyname(fd);
    define_variable_in_set(
        ctx,
        name.as_ptr() as *const ::core::ffi::c_char,
        (name.len() - 1) as size_t,
        if tty.is_null() {
            b"true\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            tty
        },
        o_default,
        0,
        (*ctx.variable_globals.current_variable_set_list.get()).set,
        NILF,
    )?
    .as_mut()
    .expect("define_variable_in_set always returns a variable")
    .set_export(v_export as variable_export);
    Ok(())
}
/// Define `name` as an empty, default-origin variable in the current scope and
/// mark it special — the shape every built-in special variable is seeded with.
/// The trailing NUL is not part of the name, so the length passed down is one
/// less than the literal's.
///
/// # Safety
///
/// Inherits `define_variable_in_set`'s contract; `name` must end in a NUL byte.
unsafe fn define_special(
    ctx: &crate::execctx::ExecContext,
    name: &'static [u8],
) -> Result<(), crate::build_result::BuildError> {
    define_variable_in_set(
        ctx,
        name.as_ptr() as *const ::core::ffi::c_char,
        (name.len() - 1) as size_t,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*ctx.variable_globals.current_variable_set_list.get()).set,
        NILF,
    )?
    .as_mut()
    .expect("define_variable_in_set always returns a variable")
    .set_special(1 as ::core::ffi::c_uint);
    Ok(())
}
/// Look up a variable whose name is a NUL-terminated byte literal, the shape
/// every lookup in this module uses. The trailing NUL is not part of the name,
/// so the length passed down is one less than the literal's.
///
/// # Safety
///
/// Inherits `lookup_variable`'s contract; `name` must end in a NUL byte.
unsafe fn lookup_named(
    ctx: &crate::execctx::ExecContext,
    name: &'static [u8],
) -> Result<*mut variable, crate::build_result::BuildError> {
    lookup_variable(
        ctx,
        name.as_ptr() as *const ::core::ffi::c_char,
        (name.len() - 1) as size_t,
    )
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn define_makeflags(
    ctx: &crate::execctx::ExecContext,
    options: &Options,
    makefile: i32,
) -> Result<*mut variable, crate::build_result::BuildError> {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let ref_0: [::core::ffi::c_char; 14] =
        ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"MAKEOVERRIDES\0");
    let posixref: [::core::ffi::c_char; 24] = ::core::mem::transmute::<
        [u8; 24],
        [::core::ffi::c_char; 24],
    >(*b"-*-command-variables-*-\0");
    let evalref: [::core::ffi::c_char; 21] =
        ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b" $(-*-eval-flags-*-)\0");
    let switches = options.switches.borrow();
    let mut cs: *const CommandSwitch;
    let mut v: *mut variable;
    let mut bufsave: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut lensave: size_t = 0;
    let mut fp: *mut ::core::ffi::c_char;
    let mut c: [::core::ffi::c_char; 3] = [0; 3];
    install_variable_buffer(ctx, &raw mut bufsave, &raw mut lensave);
    fp = variable_buffer_output(
        ctx,
        ctx.variable_buffer.ptr(),
        b"-\0" as *const u8 as *const ::core::ffi::c_char,
        1,
    );
    cs = switches.as_ptr();
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
            fp = variable_buffer_output(ctx, fp, &raw mut c as *mut ::core::ffi::c_char, 1);
        }
        cs = cs.offset(1_i32 as isize);
    }
    memcpy(
        &raw mut c as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        b" --\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        3,
    );
    cs = switches.as_ptr();
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
                                ctx,
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                        } else {
                            c[2_i32 as usize] = '-' as i32 as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                ctx,
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                            fp = variable_buffer_output(
                                ctx,
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
                                ctx,
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                        } else {
                            c[2_i32 as usize] = '-' as i32 as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                ctx,
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                            fp = variable_buffer_output(
                                ctx,
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
                                    ctx,
                                    fp,
                                    b"=\0" as *const u8 as *const ::core::ffi::c_char,
                                    1,
                                );
                            }
                            fp = variable_buffer_output(ctx, fp, buf, buflen as size_t);
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
                                ctx,
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                        } else {
                            c[2_i32 as usize] = '-' as i32 as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                ctx,
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                            fp = variable_buffer_output(
                                ctx,
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
                                    ctx,
                                    fp,
                                    b"=\0" as *const u8 as *const ::core::ffi::c_char,
                                    1,
                                );
                            }
                            fp = variable_buffer_output(ctx, fp, buf_0, buflen_0 as size_t);
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
                                ctx,
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                        } else {
                            c[2_i32 as usize] = '-' as i32 as ::core::ffi::c_char;
                            fp = variable_buffer_output(
                                ctx,
                                fp,
                                &raw mut c as *mut ::core::ffi::c_char,
                                3,
                            );
                            fp = variable_buffer_output(
                                ctx,
                                fp,
                                (*cs).long_name,
                                strlen((*cs).long_name) as size_t,
                            );
                        }
                        if !((*cs).c <= CHAR_MAX) {
                            fp = variable_buffer_output(
                                ctx,
                                fp,
                                b"=\0" as *const u8 as *const ::core::ffi::c_char,
                                1,
                            );
                        }
                        fp = variable_buffer_output(ctx, fp, p, strlen(p) as size_t);
                    }
                }
                4 | 3 => {
                    if (*cs).c == WARN_OPT {
                        fp = crate::warning::encode_flag(ctx, fp);
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
                                        ctx,
                                        fp,
                                        &raw mut c as *mut ::core::ffi::c_char,
                                        3,
                                    );
                                } else {
                                    c[2_i32 as usize] = '-' as i32 as ::core::ffi::c_char;
                                    fp = variable_buffer_output(
                                        ctx,
                                        fp,
                                        &raw mut c as *mut ::core::ffi::c_char,
                                        3,
                                    );
                                    fp = variable_buffer_output(
                                        ctx,
                                        fp,
                                        (*cs).long_name,
                                        strlen((*cs).long_name) as size_t,
                                    );
                                }
                                if !((*cs).c <= CHAR_MAX) {
                                    fp = variable_buffer_output(
                                        ctx,
                                        fp,
                                        b"=\0" as *const u8 as *const ::core::ffi::c_char,
                                        1,
                                    );
                                }
                                fp = variable_buffer_output(ctx, fp, item, strlen(item) as size_t);
                            }
                        }
                    }
                }
                // Every `command_switch` in the table carries one of the
                // handled types; a new one added without a case lands here.
                t => unreachable!("unhandled command_switch type {t} in MAKEFLAGS encoding"),
            }
        }
        cs = cs.offset(1_i32 as isize);
    }
    if fp == ctx.variable_buffer.ptr().offset(1_i32 as isize) {
        fp = ctx.variable_buffer.ptr();
    }
    assert!(
        !fp.is_null(),
        "variable_buffer must be initialized by this point in define_makeflags"
    );
    *fp = 0;
    define_variable_in_set(
        ctx,
        b"MFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1),
        ctx.variable_buffer.ptr().offset(
            (if *ctx.variable_buffer.ptr().offset(0_i32 as isize) as i32 == '-' as i32
                && *ctx.variable_buffer.ptr().offset(1_i32 as isize) as i32 == ' ' as i32
            {
                2
            } else {
                0
            }) as isize,
        ),
        o_env,
        1,
        (*ctx.variable_globals.current_variable_set_list.get()).set,
        NILF,
    )?;
    if !options.eval_strings.borrow().is_empty() {
        fp = variable_buffer_output(
            ctx,
            fp,
            &raw const evalref as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 21]>() as size_t).wrapping_sub(1),
        );
    }
    let r: *const ::core::ffi::c_char = if posix_pedantic(ctx) {
        &raw const posixref as *const ::core::ffi::c_char
    } else {
        &raw const ref_0 as *const ::core::ffi::c_char
    };
    let l: size_t = strlen(r) as size_t;
    // `inspect_err` rather than a bare `?`: the tail restores the variable
    // buffer this frame swapped out, and that has to run on the rejection path
    // too or the caller's expansion buffer is left displaced (#561).
    v = lookup_variable(ctx, r, l)
        .inspect_err(|_| restore_variable_buffer(ctx, bufsave, lensave))?;
    if v.as_ref()
        .is_some_and(|vr| !vr.value.is_null() && *vr.value.offset(0) as i32 != 0)
    {
        fp = variable_buffer_output(
            ctx,
            fp,
            b" -- $(\0" as *const u8 as *const ::core::ffi::c_char,
            6,
        );
        fp = variable_buffer_output(ctx, fp, r, l);
        fp = variable_buffer_output(
            ctx,
            fp,
            b")\0" as *const u8 as *const ::core::ffi::c_char,
            1,
        );
    }
    *fp = 0;
    fp = ctx.variable_buffer.ptr();
    if *fp.offset(0_i32 as isize) as i32 == '-' as i32 {
        fp = fp.offset(1_i32 as isize);
    }
    // Same hold as the lookup above: the tail restores the variable buffer this
    // frame swapped out, so the restore runs before a rejection escapes (#561).
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
        (*ctx.variable_globals.current_variable_set_list.get()).set,
        NILF,
    )
    .inspect_err(|_| restore_variable_buffer(ctx, bufsave, lensave))?;
    (*v).set_special(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    restore_variable_buffer(ctx, bufsave, lensave);
    Ok(v)
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
pub unsafe fn print_version(ctx: &crate::execctx::ExecContext) {
    let precede: &[u8] = if opt_print_data_base(ctx) { b"# " } else { b"" };
    if ctx.printed_version.0.swap(true, Ordering::Relaxed) {
        return;
    }
    let mut msg = Vec::with_capacity(512);
    msg.extend_from_slice(precede);
    msg.extend_from_slice(b"GNU Make ");
    msg.extend_from_slice(::core::ffi::CStr::from_ptr(crate::version::version_string()).to_bytes());
    msg.extend_from_slice(b"\n");
    msg.extend_from_slice(precede);
    msg.extend_from_slice(b"Built for ");
    msg.extend_from_slice(::core::ffi::CStr::from_ptr(crate::version::make_host()).to_bytes());
    if let Some(desc) = ctx.remote_backend.0.description() {
        msg.extend_from_slice(b" (");
        msg.extend_from_slice(desc.to_bytes());
        msg.extend_from_slice(b")");
    }
    msg.extend_from_slice(b"\n");
    msg.extend_from_slice(precede);
    msg.extend_from_slice(b"Copyright (C) 1988-2025 Free Software Foundation, Inc.\n");
    for line in [
        b"License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>\n"
            .as_slice(),
        b"This is free software: you are free to change and redistribute it.\n",
        b"There is NO WARRANTY, to the extent permitted by law.\n",
    ] {
        msg.extend_from_slice(precede);
        msg.extend_from_slice(line);
    }
    crate::output::trace_out_ctx(ctx, &msg);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_data_base(ctx: &crate::execctx::ExecContext) {
    let stamp = ::std::ffi::CString::new(file_timestamp_string(file_timestamp_now(ctx).0))
        .expect("formatted timestamp never contains an interior NUL");
    print_version(ctx);
    crate::output::trace_parts(&[b"\n# Make data base, printed on ", stamp.to_bytes(), b"\n"]);
    print_variable_data_base(ctx);
    print_dir_data_base(ctx);
    print_rule_data_base(ctx);
    print_file_data_base(ctx);
    print_vpath_data_base(ctx);
    strcache_print_stats(b"#\0" as *const u8 as *const ::core::ffi::c_char);
    let stamp = ::std::ffi::CString::new(file_timestamp_string(file_timestamp_now(ctx).0))
        .expect("formatted timestamp never contains an interior NUL");
    crate::output::trace_parts(&[
        b"\n# Finished Make data base on ",
        stamp.to_bytes(),
        b"\n\n",
    ]);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn clean_jobserver(ctx: &crate::execctx::ExecContext, status: i32) {
    if jobserver_enabled(ctx) != 0 && jobserver_tokens(ctx) != 0 {
        if status != 2 {
            error(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                INTSTR_LENGTH,
                b"INTERNAL: exiting with %u jobserver tokens (should be 0)!\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[FmtArg::Uint(jobserver_tokens(ctx) as u64)],
            );
        } else {
            loop {
                ctx.jobserver_tokens.0.fetch_sub(1, Ordering::Relaxed);
                if jobserver_tokens(ctx) == 0 {
                    break;
                }
                jobserver_release(ctx, 0);
            }
        }
    }
    let master_slots = master_job_slots(ctx);
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
                &[
                    FmtArg::Uint((tokens) as u32 as u64),
                    FmtArg::Uint((master_slots) as u32 as u64),
                ],
            );
        }
    }
    reset_jobserver_mirror(ctx);
}
/// The end-of-run cleanup the retired `die` performed before exiting —
/// reaping children, removing intermediates, closing output sync, releasing
/// jobserver tokens, chdir-ing back — split out so `main_0`'s own terminal
/// paths can run it and then *return* their status (Phase B, #432) instead of
/// exiting from inside the library. Guarded by `ctx.dying`: only the first
/// caller cleans up.
pub fn die_cleanup(ctx: &crate::execctx::ExecContext, status: i32) {
    if !ctx.dying.0.swap(true, Ordering::Relaxed) {
        // SAFETY: every call below takes the valid `ctx` this function was
        // handed (plus constants and pointers `ctx` itself owns: `make_sync`,
        // `directory_before_chdir`); none imposes a precondition the caller
        // must uphold beyond that. The block shrinks as the cleanup helpers
        // convert to safe fns.
        unsafe { die_cleanup_body(ctx, status) }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; `ctx` must be the live run's context.
unsafe fn die_cleanup_body(ctx: &crate::execctx::ExecContext, status: i32) {
    {
        let err: i32;
        if opt_print_version(ctx) {
            print_version(ctx);
        }
        temp_stdin_unlink(ctx);
        err = (status != 0) as i32;
        while job_slots_used(ctx) > 0 {
            // This is the end-of-run cleanup itself, reached once `ctx.dying` is
            // already set — there is no caller left to hand a `Result` to, and
            // `reap_children`'s own error arm finds `die_cleanup` guarded out,
            // so a reap failure here exits with that child's status exactly as
            // it did before the conversion (#432 Phase B, #441).
            reap_children(ctx, 1, err).unwrap_or_else(|e| crate::output::exit_on_err(e));
        }
        ctx.remote_backend.0.cleanup();
        remove_intermediates(ctx, 0);
        if opt_print_data_base(ctx) {
            print_data_base(ctx);
        }
        if with_options(ctx, |o| o.verify.get()) {
            verify_file_data_base(ctx);
        }
        unload_all();
        clean_jobserver(ctx, status);
        // Cleanup is always reached with the live run's context (the signal
        // handler routes here through the CTX_PTR channel), so `ctx.make_sync`
        // is the record `output_context` may be pointing at.
        let osync = output_context();
        if !osync.is_null() {
            crate::output::output_close(ctx, osync);
            if osync != ctx.make_sync.as_ptr() {
                crate::output::output_close(ctx, ctx.make_sync.as_ptr());
            }
            set_output_context(::core::ptr::null_mut::<output>());
        }
        crate::output::output_close(ctx, ::core::ptr::null_mut::<output>());
        osync_clear();
        if !ctx.directory_before_chdir.0.get().is_null() {
            let _ = chdir_c(ctx.directory_before_chdir.0.get());
        }
    }
}
pub const __CHAR_BIT__: i32 = 8;
pub const __SCHAR_MAX__: i32 = 127;

/// The command-line switch table, freshly built with its real contents — the
/// former `.init_array` `run_static_initializers` body that populated the
/// process-global `switches` at startup. Each `Options` owns its own
/// mutable copy (the `specified` bit is set during argument decoding), so two
/// sessions in one process no longer share switch state.
fn switches_template() -> [CommandSwitch; 42] {
    [
        {
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
            let mut init = CommandSwitch {
                env: 0,
                toenv: 0,
                no_makefile: 0,
                specified: 0,
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
    ]
}

#[cfg(test)]
mod default_load_average_tests {
    use crate::entry;

    /// `default_load_average` is the read-only "no load limit" sentinel
    /// (`-1.0`) the option table hands the `-l`/`--load-average` parser as its
    /// `default_value` and no-argument `noarg_value`. It is now an immutable
    /// `static` (was a `static mut`), so this is a plain safe read.
    #[test]
    fn default_load_average_is_no_limit_sentinel() {
        assert_eq!(entry::default_load_average, -1.0f64);
    }
}

#[cfg(test)]
mod job_slots_tests {
    use super::{install_default_exec_context_for_test, opt_job_slots, with_options, Options};

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
        let ctx = install_default_exec_context_for_test();

        with_options(ctx, |o| o.job_slots.set(0));
        assert_eq!(opt_job_slots(ctx), 0);

        with_options(ctx, |o| o.job_slots.set(8));
        assert_eq!(opt_job_slots(ctx), 8);

        with_options(ctx, |o| o.job_slots.set(0));
    }
}

#[cfg(test)]
mod output_sync_tests {
    use super::{
        classify_output_sync,
        install_default_exec_context_for_test,
        opt_output_sync,
        with_options,
        Options,
        OUTPUT_SYNC_LINE,
        OUTPUT_SYNC_NONE,
        OUTPUT_SYNC_RECURSE,
        OUTPUT_SYNC_TARGET,
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
        let ctx = install_default_exec_context_for_test();

        with_options(ctx, |o| o.output_sync.set(OUTPUT_SYNC_NONE));
        assert_eq!(opt_output_sync(ctx), OUTPUT_SYNC_NONE);

        with_options(ctx, |o| o.output_sync.set(OUTPUT_SYNC_LINE));
        assert_eq!(opt_output_sync(ctx), OUTPUT_SYNC_LINE);

        with_options(ctx, |o| o.output_sync.set(OUTPUT_SYNC_NONE));
    }
}

#[cfg(test)]
mod special_target_latches_tests {
    use super::{
        install_default_exec_context_for_test,
        not_parallel,
        one_shell,
        posix_pedantic,
        second_expansion,
        set_not_parallel,
        set_one_shell,
        set_posix_pedantic,
        set_second_expansion,
        with_options,
        Options,
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
        let ctx = install_default_exec_context_for_test();
        with_options(ctx, |o| {
            o.posix_pedantic.set(false);
            o.second_expansion.set(false);
            o.one_shell.set(false);
            o.not_parallel.set(false);
        });
        assert!(
            !posix_pedantic(ctx) && !second_expansion(ctx) && !one_shell(ctx) && !not_parallel(ctx)
        );

        set_posix_pedantic(ctx);
        set_second_expansion(ctx);
        set_one_shell(ctx);
        set_not_parallel(ctx);
        assert!(posix_pedantic(ctx), "enabled by .POSIX");
        assert!(second_expansion(ctx), "enabled by .SECONDEXPANSION");
        assert!(one_shell(ctx), "enabled by .ONESHELL");
        assert!(not_parallel(ctx), "enabled by .NOTPARALLEL");

        with_options(ctx, |o| {
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
    use super::{install_default_exec_context_for_test, opt_run_silent, with_options, Options};

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
        let ctx = install_default_exec_context_for_test();

        with_options(ctx, |o| o.run_silent.set(false));
        assert!(!opt_run_silent(ctx), "channel reads the cleared flag");

        with_options(ctx, |o| o.run_silent.set(true));
        assert!(opt_run_silent(ctx), "channel reads the set flag");

        with_options(ctx, |o| o.run_silent.set(false));
    }
}

#[cfg(test)]
mod export_all_variables_tests {
    use super::{
        install_default_exec_context_for_test,
        opt_export_all_variables,
        with_options,
        Options,
    };

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
        let ctx = install_default_exec_context_for_test();

        with_options(ctx, |o| o.export_all_variables.set(false));
        assert!(
            !opt_export_all_variables(ctx),
            "channel reads the cleared flag"
        );

        with_options(ctx, |o| o.export_all_variables.set(true));
        assert!(opt_export_all_variables(ctx), "channel reads the set flag");

        with_options(ctx, |o| o.export_all_variables.set(false));
    }
}

#[cfg(test)]
mod cmd_prefix_tests {
    use super::{install_default_exec_context_for_test, opt_cmd_prefix, with_options, Options};

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
        let ctx = install_default_exec_context_for_test();

        with_options(ctx, |o| o.cmd_prefix.set(b'\t' as ::core::ffi::c_char));
        assert_eq!(opt_cmd_prefix(ctx), b'\t' as ::core::ffi::c_char);

        with_options(ctx, |o| o.cmd_prefix.set(b'>' as ::core::ffi::c_char));
        assert_eq!(opt_cmd_prefix(ctx), b'>' as ::core::ffi::c_char);

        with_options(ctx, |o| o.cmd_prefix.set(b'\t' as ::core::ffi::c_char));
    }
}

#[cfg(test)]
mod stdio_traced_tests {
    use {
        super::{install_default_exec_context_for_test, with_options, Options},
        crate::output::{set_stdio_traced, stdio_traced},
    };

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
        let ctx = install_default_exec_context_for_test();
        with_options(ctx, |o| o.stdio_traced.set(false));
        assert!(!stdio_traced(ctx), "not yet traced");

        set_stdio_traced(ctx, true);
        assert!(stdio_traced(ctx), "trace emitted through the channel");

        set_stdio_traced(ctx, false);
        assert!(!stdio_traced(ctx), "false through the channel");
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
    use super::{install_default_exec_context_for_test, master_job_slots, with_options, Options};

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
        let ctx = install_default_exec_context_for_test();
        with_options(ctx, |o| o.master_job_slots.set(0));
        assert_eq!(
            master_job_slots(ctx),
            0,
            "channel reads the installed value"
        );

        with_options(ctx, |o| o.master_job_slots.set(4));
        assert_eq!(master_job_slots(ctx), 4, "count through the channel");

        with_options(ctx, |o| o.master_job_slots.set(0));
    }
}

#[cfg(test)]
mod command_count_tests {
    use super::{
        bump_command_count,
        install_default_exec_context_for_test,
        opt_command_count,
        with_options,
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
        let ctx = install_default_exec_context_for_test();

        with_options(ctx, |o| o.command_count.set(1));
        assert_eq!(
            opt_command_count(ctx),
            1,
            "channel reads the installed value"
        );

        bump_command_count(ctx);
        bump_command_count(ctx);
        assert_eq!(opt_command_count(ctx), 3, "two bumps through the channel");

        with_options(ctx, |o| o.command_count.set(1));
    }
}

#[cfg(test)]
mod snapped_deps_tests {
    use super::{
        install_default_exec_context_for_test,
        mark_snapped_deps,
        opt_snapped_deps,
        with_options,
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
        let ctx = install_default_exec_context_for_test();

        with_options(ctx, |o| o.snapped_deps.set(false));
        assert!(!opt_snapped_deps(ctx), "channel reads the installed value");

        mark_snapped_deps(ctx);
        assert!(opt_snapped_deps(ctx), "marked through the channel");

        with_options(ctx, |o| o.snapped_deps.set(false));
    }
}

#[cfg(test)]
mod rebuilding_makefiles_tests {
    use super::{
        install_default_exec_context_for_test,
        opt_rebuilding_makefiles,
        with_options,
        Options,
    };

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
        let ctx = install_default_exec_context_for_test();

        with_options(ctx, |o| o.rebuilding_makefiles.set(false));
        assert!(
            !opt_rebuilding_makefiles(ctx),
            "channel reads the installed value"
        );

        with_options(ctx, |o| o.rebuilding_makefiles.set(true));
        assert!(opt_rebuilding_makefiles(ctx), "true through the channel");

        with_options(ctx, |o| o.rebuilding_makefiles.set(false));
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
    use {super::Options, crate::execctx::ExecContext};

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
    use super::{
        should_print_dir,
        should_print_dir_unsafe_oracle::should_print_dir as oracle,
        Options,
    };

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
mod clean_jobserver_tests {
    use super::{clean_jobserver, with_options};

    /// The quiet end-of-run path a normal build takes: no live jobserver, no
    /// held tokens, no master slot count. The auth mirror is still cleared
    /// unconditionally (`reset_jobserver_mirror`).
    #[test]
    fn clears_auth_when_no_jobserver_and_no_master_slots() {
        // A fresh `ExecContext` starts with zero jobserver tokens, so unlike
        // the former global counter there is nothing to reset up front.
        let ctx = crate::execctx::ExecContext::default();
        with_options(&ctx, |o| {
            o.master_job_slots.set(0);
            *o.jobserver_auth.borrow_mut() = Some("fifo:/tmp/x".to_string());
        });

        unsafe { clean_jobserver(&ctx, 0) };

        with_options(&ctx, |o| {
            assert!(o.jobserver_auth.borrow().is_none(), "mirror reset")
        });
    }

    /// The master-make token accounting: with `master_job_slots` set and no
    /// live jobserver pipe (fds are closed sentinels), `jobserver_acquire_all`
    /// recovers 0 tokens, so `1 + 0` reconciles against a 1-slot master
    /// silently and mismatches a 2-slot master through the INTERNAL
    /// diagnostic — both paths must complete and still reset the mirror.
    #[test]
    fn master_slot_accounting_matches_and_mismatches() {
        let ctx = crate::execctx::ExecContext::default();

        // 1 + acquire_all() == master slots: counts reconcile, no diagnostic.
        with_options(&ctx, |o| o.master_job_slots.set(1));
        unsafe { clean_jobserver(&ctx, 0) };

        // 1 + 0 != 2: the mismatch diagnostic runs (prints INTERNAL to
        // stderr) and the cleanup still completes.
        with_options(&ctx, |o| {
            o.master_job_slots.set(2);
            *o.jobserver_auth.borrow_mut() = Some("fifo:/tmp/x".to_string());
        });
        unsafe { clean_jobserver(&ctx, 0) };

        with_options(&ctx, |o| {
            assert!(
                o.jobserver_auth.borrow().is_none(),
                "mirror reset after mismatch"
            );
            o.master_job_slots.set(0);
        });
    }
}

#[cfg(test)]
mod jobserver_and_stdin_cleanup_tests {
    use super::{reset_jobserver, temp_stdin_unlink, with_options, Options};

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
        // `temp_stdin_unlink` reads/clears `stdin_offset` through
        // `with_options`, which now reads `ctx.options` directly.
        // No temp stdin registered (defaults): must be a harmless no-op.
        let ctx = crate::execctx::ExecContext::default();
        unsafe {
            with_options(&ctx, |o| o.stdin_offset.set(-1));
            ctx.temp_stdin_name.0.set(::core::ptr::null());
            temp_stdin_unlink(&ctx);
        }

        // Real unlink path: create a temp file and register it.
        let dir = std::env::temp_dir();
        let path = dir.join(format!("makers_tmpstdin_test_{}", std::process::id()));
        std::fs::write(&path, b"all:\n").unwrap();
        assert!(path.exists());
        let cpath = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
        unsafe {
            with_options(&ctx, |o| o.stdin_offset.set(0));
            ctx.temp_stdin_name.0.set(cpath.as_ptr());
            temp_stdin_unlink(&ctx);
            // Restore globals before `cpath` is dropped to avoid a dangling ptr.
            ctx.temp_stdin_name.0.set(::core::ptr::null());
            with_options(&ctx, |o| o.stdin_offset.set(-1));
        }
        assert!(!path.exists(), "temp stdin file should have been unlinked");
        drop(cpath);
    }
}

#[cfg(test)]
mod decode_switches_clap_vs_getopt_tests {
    use {
        super::{
            decode_switches,
            getopt_oracle_test::decode_switches_oracle,
            install_default_exec_context_for_test,
            o_command,
            o_env,
            Options,
        },
        crate::{execctx::ExecContext, variable::init_hash_global_variable_set},
        std::ffi::CString,
    };

    /// Everything `decode_switches` writes, gathered into one comparable
    /// value. `max_load_average` is compared via its bit pattern so `NaN`
    /// (never actually produced here) wouldn't silently pass; every other
    /// field is a plain `Eq` type.
    #[derive(Debug, PartialEq)]
    struct Snapshot {
        silent: bool,
        touch: bool,
        just_print: bool,
        db_flags: Vec<String>,
        debug_flag: bool,
        output_sync_option: Option<String>,
        env_overrides: bool,
        ignore_errors: bool,
        print_data_base: bool,
        print_targets: bool,
        question: bool,
        no_builtin_rules: bool,
        no_builtin_variables: bool,
        keep_going: bool,
        check_symlink: bool,
        print_directory: Option<bool>,
        print_version: bool,
        makefiles: Vec<String>,
        stdin_offset: i32,
        arg_job_slots: Option<u32>,
        jobserver_auth: Option<String>,
        jobserver_style: Option<String>,
        shuffle_mode: Option<String>,
        sync_mutex: Option<String>,
        max_load_average_bits: u64,
        directories: Vec<String>,
        include_dirs: Vec<String>,
        old_files: Vec<String>,
        new_files: Vec<String>,
        eval_strings: Vec<String>,
        print_usage: bool,
        warn_flags: Vec<String>,
        trace: bool,
        always_make: bool,
        run_silent: bool,
        goal_count: usize,
        specified: Vec<u8>,
    }

    fn strs(v: &[CString]) -> Vec<String> {
        v.iter().map(|s| s.to_string_lossy().into_owned()).collect()
    }

    fn snapshot(options: &Options) -> Snapshot {
        Snapshot {
            silent: options.silent.get(),
            touch: options.touch.get(),
            just_print: options.just_print.get(),
            db_flags: strs(&options.db_flags.borrow()),
            debug_flag: options.debug_flag.get(),
            output_sync_option: options.output_sync_option.borrow().clone(),
            env_overrides: options.env_overrides.get(),
            ignore_errors: options.ignore_errors.get(),
            print_data_base: options.print_data_base.get(),
            print_targets: options.print_targets.get(),
            question: options.question.get(),
            no_builtin_rules: options.no_builtin_rules.get(),
            no_builtin_variables: options.no_builtin_variables.get(),
            keep_going: options.keep_going.get(),
            check_symlink: options.check_symlink.get(),
            print_directory: options.print_directory.get(),
            print_version: options.print_version.get(),
            makefiles: strs(&options.makefiles.borrow()),
            stdin_offset: options.stdin_offset.get(),
            arg_job_slots: options.arg_job_slots.get(),
            jobserver_auth: options.jobserver_auth.borrow().clone(),
            jobserver_style: options.jobserver_style.borrow().clone(),
            shuffle_mode: options.shuffle_mode.borrow().clone(),
            sync_mutex: options.sync_mutex.borrow().clone(),
            max_load_average_bits: options.max_load_average.get().to_bits(),
            directories: strs(&options.directories.borrow()),
            include_dirs: strs(&options.include_dirs.borrow()),
            old_files: strs(&options.old_files.borrow()),
            new_files: strs(&options.new_files.borrow()),
            eval_strings: strs(&options.eval_strings.borrow()),
            print_usage: options.print_usage.get(),
            warn_flags: strs(&options.warn_flags.borrow()),
            trace: options.trace.get(),
            always_make: options.always_make.get(),
            run_silent: options.run_silent.get(),
            goal_count: options.goals.borrow().len(),
            specified: options
                .switches
                .borrow()
                .iter()
                .map(|s| s.specified() as u8)
                .collect(),
        }
    }

    /// Serializes calls to the getopt-based oracle. This is test-only
    /// belt-and-suspenders: `cargo test`'s default parallel runner calls
    /// `check()` from many threads at once, and the oracle's `optarg`/
    /// `optind`/`opterr` are real libc globals shared by the whole process
    /// -- without this lock, concurrent oracle calls race on them (observed:
    /// garbage `Options` state, even a segfault), which is exactly the
    /// latent multi-tenant hazard this migration replaces `decode_switches`
    /// to eliminate from the shipping binary. The new, clap-based
    /// `decode_switches` has no such shared state and needs no lock.
    static ORACLE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Runs `tokens` through both the clap-based `decode_switches` and the
    /// preserved getopt-based oracle against fresh, independent `Options`,
    /// and asserts every field they write agrees.
    fn check(tokens: &[&str], origin: super::variable_origin) {
        // `handle_non_switch_argument`'s variable-assignment scan reads
        // `stopchar_map()`, which is a zeroed fallback until
        // `initialize_stopchar_map()` has run once (it's a `OnceLock`,
        // idempotent past the first call) -- without it, the scan for a
        // terminating stopchar never finds one and spins forever. Every
        // other test in this codebase that exercises parsing already calls
        // this first for the same reason.
        super::initialize_stopchar_map();
        let _ctx = install_default_exec_context_for_test();

        let ctx_new = ExecContext::default();
        unsafe { init_hash_global_variable_set(&ctx_new) };
        let options_new = Options::new();
        let owned_tokens: Vec<::std::ffi::OsString> =
            tokens.iter().map(::std::ffi::OsString::from).collect();
        let result_new = decode_switches(&ctx_new, &options_new, &owned_tokens, origin);

        let ctx_oracle = ExecContext::default();
        unsafe { init_hash_global_variable_set(&ctx_oracle) };
        let options_oracle = Options::new();
        // Build a C-style argv: argv[0] is the (skipped) program name, like
        // the real `main_0`/`decode_env_switches` callers always pass.
        let cstrings: Vec<CString> = ::core::iter::once(String::new())
            .chain(tokens.iter().map(|s| s.to_string()))
            .map(|s| CString::new(s).unwrap())
            .collect();
        let argv_ptrs: Vec<*const ::core::ffi::c_char> =
            cstrings.iter().map(|s| s.as_ptr()).collect();
        let result_oracle = {
            let _guard = ORACLE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            unsafe {
                decode_switches_oracle(
                    &ctx_oracle,
                    &options_oracle,
                    argv_ptrs.len() as i32,
                    argv_ptrs.as_ptr() as *mut *const ::core::ffi::c_char,
                    origin,
                )
            }
        };

        // Both sides now report a fatal switch error as `Err` instead of
        // exiting, so the outcome itself is part of the differential
        // comparison rather than something only one side could express.
        assert_eq!(
            result_new, result_oracle,
            "result mismatch for tokens {tokens:?} (origin {origin})"
        );
        let snap_new = snapshot(&options_new);
        let snap_oracle = snapshot(&options_oracle);
        assert_eq!(
            snap_new, snap_oracle,
            "mismatch for tokens {tokens:?} (origin {origin})"
        );
    }

    #[test]
    fn plain_boolean_flags() {
        check(&["-i", "-p", "-q", "-r", "-R", "-B", "-L", "-v"], o_command);
        check(&["--ignore-errors", "--print-data-base"], o_command);
    }

    #[test]
    fn flag_off_pairs_last_one_wins() {
        check(&["-k"], o_command);
        check(&["-S"], o_command);
        check(&["-k", "-S"], o_command);
        check(&["-S", "-k"], o_command);
        check(&["-w"], o_command);
        check(&["--no-print-directory"], o_command);
        check(&["-w", "--no-print-directory"], o_command);
        check(&["--no-print-directory", "-w"], o_command);
        check(&["-s"], o_command);
        check(&["--no-silent"], o_command);
        check(&["-s", "--no-silent"], o_command);
        check(&["--no-silent", "-s"], o_command);
        check(&["-kS"], o_command);
        check(&["-Sk"], o_command);
    }

    #[test]
    fn aliases() {
        check(&["--quiet"], o_command);
        check(&["--stop"], o_command);
        check(&["--dry-run", "t"], o_command);
        check(&["--recon", "t"], o_command);
        check(&["--makefile", "a.mk"], o_command);
        check(&["--assume-old", "a"], o_command);
        check(&["--max-load=2.5"], o_command);
        check(&["--assume-new", "a"], o_command);
    }

    #[test]
    fn jobs_optional_arg_variants() {
        check(&["-j"], o_command);
        check(&["-j4"], o_command);
        check(&["-j", "4"], o_command);
        check(&["--jobs"], o_command);
        check(&["--jobs=4"], o_command);
        check(&["--jobs", "4"], o_command);
        check(&["-j", "target"], o_command); // non-numeric next: -j stays bare, "target" is a target
        check(&["-j4", "-j8"], o_command); // repeats: last wins
                                           // o_env, not o_command: an explicit empty value is a real error
                                           // (bad=1), and bad=1 with origin==o_command triggers print_usage(),
                                           // which exits the whole test process.
        check(&["--jobs="], o_env); // explicit empty value (distinct from bare --jobs): error
    }

    #[test]
    fn load_average_optional_arg_variants() {
        check(&["-l"], o_command);
        check(&["-l3.5"], o_command);
        check(&["-l", "3.5"], o_command);
        check(&["-l", ".5"], o_command);
        check(&["--load-average"], o_command);
        check(&["--load-average=2"], o_command);
        check(&["--max-load", "2"], o_command);
        check(&["-l", "target"], o_command); // non-numeric, non-dot next: -l stays bare
        check(&["--load-average="], o_command); // explicit empty value
    }

    #[test]
    fn other_optional_arg_switches() {
        check(&["-O"], o_command);
        check(&["-Otarget"], o_command);
        check(&["-O", "target"], o_command); // -O never consumes a separate token
        check(&["--output-sync"], o_command);
        check(&["--output-sync=line"], o_command);
        check(&["--debug"], o_command);
        check(&["--debug=b"], o_command);
        check(&["--debug=b", "--debug=v"], o_command);
        check(&["--debug=b", "--debug=b"], o_command); // exact-dup skipped
        check(&["--shuffle"], o_command);
        check(&["--shuffle=reverse"], o_command);
        // o_env, not o_command: an explicit empty value is a real error
        // (bad=1), and bad=1 with origin==o_command triggers print_usage(),
        // which exits the whole test process.
        check(&["--debug="], o_env); // explicit empty value: error
        check(&["--output-sync="], o_env); // explicit empty value: error
        check(&["--shuffle="], o_env); // explicit empty value: error
    }

    #[test]
    fn required_arg_list_switches_dedup_rules() {
        check(&["-I", "a", "-I", "b"], o_command);
        check(&["-I", "a", "-I", "a"], o_command); // -I dedups
        check(&["-f", "a.mk", "-f", "a.mk"], o_command); // -f allows dups
        check(&["-C", "sub1", "-C", "sub2"], o_command);
        check(&["-o", "a"], o_command);
        check(&["-W", "a"], o_command);
        check(&["-E", "FOO=bar"], o_command);
        check(&["--warn=undefined-var", "--warn=undefined-var"], o_command); // --warn allows dups
    }

    #[test]
    fn required_arg_empty_value_is_an_error() {
        // o_env, not o_command: bad=1 with origin==o_command runs
        // print_usage() and the end-of-run cleanup before returning `Err`,
        // none of which the oracle mirrors, so the two sides would diverge.
        check(&["-f", ""], o_env);
        check(&["--file="], o_env);
    }

    /// A non-UTF-8 filename argument (e.g. from a byte-oriented POSIX
    /// filesystem) must round-trip through `decode_switches` exactly, not
    /// get replaced with U+FFFD -- unlike `check()`'s other cases, this
    /// bypasses the oracle comparison (its harness only accepts `&str`
    /// tokens) and asserts directly on the stored bytes.
    #[test]
    fn non_utf8_filename_argument_round_trips_exact_bytes() {
        use std::os::unix::ffi::OsStrExt;

        super::initialize_stopchar_map();
        let ctx = ExecContext::default();
        unsafe { init_hash_global_variable_set(&ctx) };
        let options = Options::new();

        let raw_name: &[u8] = b"weird-\xffname.mk";
        let tokens: Vec<::std::ffi::OsString> = vec![
            ::std::ffi::OsString::from("-f"),
            ::std::ffi::OsStr::from_bytes(raw_name).to_os_string(),
        ];
        decode_switches(&ctx, &options, &tokens, o_command).expect("valid -f switch");

        let makefiles = options.makefiles.borrow();
        assert_eq!(makefiles.len(), 1);
        assert_eq!(makefiles[0].as_bytes(), raw_name);
    }

    #[test]
    fn string_switches() {
        check(&["-Otarget"], o_command);
        check(&["--jobserver-auth=3,4"], o_command);
        check(&["--sync-mutex=foo"], o_command);
    }

    #[test]
    fn targets_and_assignments_interleaved() {
        check(&["-j4", "VAR=val", "-k", "target2"], o_command);
        check(&["target1", "-k", "VAR=1", "target2"], o_command);
    }

    #[test]
    fn unknown_option_mixed_with_valid_ones() {
        // o_env, not o_command: bad=1 with origin==o_command runs
        // print_usage() and the end-of-run cleanup before returning `Err`,
        // none of which the oracle mirrors, so the two sides would diverge.
        check(&["-j2", "-Q", "target"], o_env);
        check(&["-Q", "-j2", "target"], o_env);
        check(&["--bogus-long", "-k", "target"], o_env);
    }

    #[test]
    fn origin_env_does_not_override_command_origin() {
        // `-s`/`-k`/`-w` have an origin cell; a weaker `o_env`-origin re-parse
        // must not clobber a value already recorded from `o_command`.
        check(&["-k"], o_command);
        check(&["-k"], o_env);
        check(&["-w"], o_env);
    }

    #[test]
    fn makeflags_style_bundle_via_env_helper() {
        // `decode_env_switches`'s dash-optional legacy rewrite is exercised
        // separately (it runs before `decode_switches`); here we just check
        // that an already-dashed bundle behaves the same through both paths.
        check(&["-ik"], o_command);
    }
}

/// The switch-decoding fatals that used to end the process now come back as
/// [`BuildError`](crate::build_result::BuildError), so they are reachable from
/// a unit test for the first time (#432 Phase B, #442). Each case asserts both
/// that the error is reported *and* that it is reported as a value rather than
/// an exit — the whole point of the conversion.
#[cfg(test)]
mod decode_switches_error_paths {
    use {
        super::{
            decode_debug_flags,
            decode_output_sync_flags,
            decode_switches,
            expand_command_line_file,
            install_default_exec_context_for_test,
            o_env,
            Options,
        },
        crate::{build_result::BuildError, execctx::ExecContext},
    };

    #[test]
    fn unknown_debug_level_is_an_error_not_an_exit() {
        let _ctx = install_default_exec_context_for_test();
        let ctx = ExecContext::default();
        let options = Options::new();
        options
            .db_flags
            .borrow_mut()
            .push(::std::ffi::CString::new("zzz").unwrap());
        // SAFETY: `decode_debug_flags` reads `options.db_flags`, whose entries
        // are live NUL-terminated `CString`s, and needs a valid `&ExecContext`.
        let r = unsafe { decode_debug_flags(&ctx, &options) };
        assert_eq!(r, Err(BuildError::Failure));
    }

    #[test]
    fn every_known_debug_level_still_succeeds() {
        for level in ["a", "b", "i", "j", "m", "n", "p", "v", "w", "b,i", "j m"] {
            let _ctx = install_default_exec_context_for_test();
            let ctx = ExecContext::default();
            let options = Options::new();
            options
                .db_flags
                .borrow_mut()
                .push(::std::ffi::CString::new(level).unwrap());
            // SAFETY: as above.
            let r = unsafe { decode_debug_flags(&ctx, &options) };
            assert_eq!(r, Ok(()), "debug level {level:?} should be accepted");
        }
    }

    #[test]
    fn unknown_output_sync_type_is_an_error_not_an_exit() {
        let _ctx = install_default_exec_context_for_test();
        let ctx = ExecContext::default();
        let options = Options::new();
        *options.output_sync_option.borrow_mut() = Some("bogus".to_string());
        // SAFETY: `decode_output_sync_flags` reads the two option strings,
        // both of which are live `String`s here, and needs a valid context.
        let r = unsafe { decode_output_sync_flags(&ctx, &options) };
        assert_eq!(r, Err(BuildError::Failure));
    }

    #[test]
    fn known_output_sync_types_still_succeed() {
        for (name, mode) in [
            ("none", super::OUTPUT_SYNC_NONE),
            ("line", super::OUTPUT_SYNC_LINE),
            ("target", super::OUTPUT_SYNC_TARGET),
            ("recurse", super::OUTPUT_SYNC_RECURSE),
        ] {
            let _ctx = install_default_exec_context_for_test();
            let ctx = ExecContext::default();
            let options = Options::new();
            *options.output_sync_option.borrow_mut() = Some(name.to_string());
            // SAFETY: as above.
            let r = unsafe { decode_output_sync_flags(&ctx, &options) };
            assert_eq!(r, Ok(()), "output-sync {name:?} should be accepted");
            assert_eq!(options.output_sync.get(), mode);
        }
    }

    /// `reset_makeflags` re-reads `MAKEFLAGS` and rebuilds the include path.
    /// Since #442 both steps return `Result`, so the whole function propagates
    /// instead of exiting; on a clean context there is nothing to reject, so it
    /// succeeds and the `?` arms stay covered rather than sitting at 0%.
    #[test]
    fn reset_makeflags_succeeds_on_a_clean_context() {
        super::initialize_stopchar_map();
        let _ctx = install_default_exec_context_for_test();
        let ctx = ExecContext::default();
        // SAFETY: the global variable set and builtin function table must exist
        // before `decode_env_switches` and `define_makeflags` look anything up.
        unsafe {
            crate::variable::init_hash_global_variable_set(&ctx);
            crate::function::hash_init_function_table(&ctx);
        }
        let options = Options::new();
        // SAFETY: `reset_makeflags` is the c2rust raw-pointer API; `ctx` and
        // `options` are freshly built and valid for the call.
        let r = unsafe { super::reset_makeflags(&ctx, &options, super::o_env) };
        assert!(r.is_ok(), "a clean context has no MAKEFLAGS to reject");
    }

    #[test]
    fn empty_file_name_is_an_error_not_an_exit() {
        let _ctx = install_default_exec_context_for_test();
        let ctx = ExecContext::default();
        // SAFETY: `c""` is a valid NUL-terminated C string.
        let r = unsafe { expand_command_line_file(&ctx, c"".as_ptr()) };
        assert_eq!(r.err(), Some(BuildError::Failure));
    }

    #[test]
    fn empty_switch_argument_propagates_out_of_decode_switches() {
        // `-f ''` reaches `expand_command_line_file`'s empty-name fatal only
        // after clap accepts the (present but empty) value, so this exercises
        // the whole `decode_switches` -> `apply_value_switch` ->
        // `expand_command_line_file` chain as one `Result`. `o_env` keeps the
        // bad-switch branch (which runs `die_cleanup`) out of the picture.
        super::initialize_stopchar_map();
        let _ctx = install_default_exec_context_for_test();
        let ctx = ExecContext::default();
        // SAFETY: initializes the global variable set this context needs
        // before any variable lookup runs.
        unsafe { crate::variable::init_hash_global_variable_set(&ctx) };
        let options = Options::new();
        let tokens = vec![
            ::std::ffi::OsString::from("-f"),
            ::std::ffi::OsString::from(""),
        ];
        // An explicitly empty value is rejected by `apply_value_switch` before
        // it ever reaches the file expander, so this records `bad` and returns
        // `Ok` -- the error surface stays exactly where it was pre-conversion.
        assert_eq!(decode_switches(&ctx, &options, &tokens, o_env), Ok(()));
        assert!(options.makefiles.borrow().is_empty());
    }
}
