use libc::{__errno_location, abort, free, printf, putchar, puts, sprintf, strchr, strcmp, strcpy, unlink};
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
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
    fn error(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn fatal(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn perror_with_name(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char);
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xcalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xrealloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn xstrdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn end_of_token(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn find_percent(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strcache_iscached(str: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcache_add_len(str: *const ::core::ffi::c_char, len: size_t)
        -> *const ::core::ffi::c_char;
    static mut stopchar_map: [::core::ffi::c_ushort; 0];
    static mut just_print_flag: ::core::ffi::c_int;
    static mut run_silent: ::core::ffi::c_int;
    static mut ignore_errors_flag: ::core::ffi::c_int;
    static mut question_flag: ::core::ffi::c_int;
    static mut touch_flag: ::core::ffi::c_int;
    static mut no_builtin_rules_flag: ::core::ffi::c_int;
    static mut not_parallel: ::core::ffi::c_int;
    static mut second_expansion: ::core::ffi::c_int;
    static mut verify_flag: ::core::ffi::c_int;
    static mut export_all_variables: ::core::ffi::c_int;
    static mut cmd_prefix: ::core::ffi::c_char;
    static mut no_intermediates: ::core::ffi::c_uint;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn print_commands(cmds: *const commands);
    fn set_file_variables(file: *mut file, stem: *const ::core::ffi::c_char);
    static mut db_level: ::core::ffi::c_int;
    fn parse_file_seq(
        stringp: *mut *mut ::core::ffi::c_char,
        size: size_t,
        stopmap: ::core::ffi::c_int,
        prefix: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn free_ns_chain(n: *mut nameseq);
    fn copy_dep_chain(d: *const dep) -> *mut dep;
    fn hash_init(
        ht: *mut hash_table,
        size: ::core::ffi::c_ulong,
        hash_1: hash_func_t,
        hash_2: hash_func_t,
        hash_cmp: hash_cmp_func_t,
    );
    fn hash_find_slot(
        ht: *mut hash_table,
        key: *const ::core::ffi::c_void,
    ) -> *mut *mut ::core::ffi::c_void;
    fn hash_find_item(
        ht: *mut hash_table,
        key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn hash_insert_at(
        ht: *mut hash_table,
        item: *const ::core::ffi::c_void,
        slot: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn hash_delete(
        ht: *mut hash_table,
        item: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn hash_map(ht: *mut hash_table, map: hash_map_func_t);
    fn hash_print_stats(ht: *mut hash_table, out_FILE: *mut FILE);
    fn hash_dump(
        ht: *mut hash_table,
        vector_0: *mut *mut ::core::ffi::c_void,
        compare: qsort_cmp_t,
    ) -> *mut *mut ::core::ffi::c_void;
    fn jhash_string(key: *const ::core::ffi::c_uchar) -> ::core::ffi::c_uint;
    static mut hash_deleted_item: *const ::core::ffi::c_void;
    fn shuffle_deps_recursive(g: *mut dep);
    static mut variable_buffer: *mut ::core::ffi::c_char;
    fn variable_buffer_output(
        ptr: *mut ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
        length: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn expand_string_buf(
        buf: *mut ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
        length: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn expand_string_for_file(
        string: *const ::core::ffi::c_char,
        file: *mut file,
    ) -> *mut ::core::ffi::c_char;
    fn patsubst_expand_pat(
        o: *mut ::core::ffi::c_char,
        text: *const ::core::ffi::c_char,
        pattern: *const ::core::ffi::c_char,
        replace: *const ::core::ffi::c_char,
        pattern_percent: *const ::core::ffi::c_char,
        replace_percent: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn initialize_file_variables(file: *mut file, reading: ::core::ffi::c_int);
    fn print_file_variables(file: *const file);
    fn print_target_variables(file: *const file);
    fn merge_variable_set_lists(
        to_list: *mut *mut variable_set_list,
        from_list: *mut variable_set_list,
    );
    fn lookup_variable(name: *const ::core::ffi::c_char, length: size_t) -> *mut variable;
    fn lookup_variable_in_set(
        name: *const ::core::ffi::c_char,
        length: size_t,
        set: *const variable_set,
    ) -> *mut variable;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
pub type intmax_t = ::libc::intmax_t;
pub type uintmax_t = ::libc::uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct File {
    pub name: *const ::core::ffi::c_char,
    pub hname: *const ::core::ffi::c_char,
    pub vpath: *const ::core::ffi::c_char,
    pub deps: *mut Dep,
    pub cmds: *mut Commands,
    pub stem: *const ::core::ffi::c_char,
    pub also_make: *mut Dep,
    pub prev: *mut File,
    pub last: *mut File,
    pub renamed: *mut File,
    pub variables: *mut VariableSetList,
    pub pat_variables: *mut VariableSetList,
    pub parent: *mut File,
    pub double_colon: *mut File,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: ::core::ffi::c_uint,
    pub command_flags: ::core::ffi::c_int,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(
        name = "low_resolution_time",
        ty = "::core::ffi::c_uint",
        bits = "8..=8"
    )]
    #[bitfield(name = "tried_implicit", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "::core::ffi::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "::core::ffi::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "::core::ffi::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "::core::ffi::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "::core::ffi::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "::core::ffi::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "::core::ffi::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "::core::ffi::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "::core::ffi::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "::core::ffi::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "::core::ffi::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "::core::ffi::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "::core::ffi::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "::core::ffi::c_uint", bits = "24..=24")]
    #[bitfield(name = "suffix", ty = "::core::ffi::c_uint", bits = "25..=25")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped_suffix:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableSetList {
    pub next: *mut VariableSetList,
    pub set: *mut VariableSet,
    pub next_is_parent: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableSet {
    pub table: hash_table,
}
pub type hash_table = crate::hash::hash_table;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Dep {
    pub next: *mut Dep,
    pub name: *const ::core::ffi::c_char,
    pub file: *mut File,
    pub shuf: *mut Dep,
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
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Commands {
    pub fileinfo: Floc,
    pub commands: *mut ::core::ffi::c_char,
    pub command_lines: *mut *mut ::core::ffi::c_char,
    pub lines_flags: *mut ::core::ffi::c_uchar,
    pub ncommand_lines: ::core::ffi::c_ushort,
    pub recipe_prefix: ::core::ffi::c_char,
    #[bitfield(name = "any_recurse", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub any_recurse: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
use crate::floc::Floc;

pub type file = File;
pub type dep = Dep;
pub type commands = Commands;
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;

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
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const ::core::ffi::c_char,
}
pub type hash_map_func_t = Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> ()>;
pub type qsort_cmp_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub const ENOENT: ::core::ffi::c_int = 2;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const INTSTR_LENGTH: usize = (53 as usize)
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22 as usize)
    .wrapping_add(3 as usize);
pub const RECIPEPREFIX_DEFAULT: ::core::ffi::c_int = '\t' as i32;
pub const COMMANDS_SILENT: ::core::ffi::c_int = 2;
pub const COMMANDS_NOERROR: ::core::ffi::c_int = 4;
#[inline]
#[no_mangle]
pub unsafe extern "C" fn free_ns(mut n: *mut nameseq) {
    free(n as *mut ::core::ffi::c_void);
}
#[inline]
#[no_mangle]
pub unsafe extern "C" fn free_dep(mut d: *mut dep) {
    free_ns(d as *mut nameseq);
}
#[inline]
#[no_mangle]
pub unsafe extern "C" fn free_dep_chain(mut d: *mut dep) {
    free_ns_chain(d as *mut nameseq);
}
pub const UNKNOWN_MTIME: ::core::ffi::c_int = 0;
pub const NONEXISTENT_MTIME: ::core::ffi::c_int = 1;
pub const OLD_MTIME: ::core::ffi::c_int = 2;
pub const ORDINARY_MTIME_MIN: ::core::ffi::c_int = OLD_MTIME + 1;
#[no_mangle]
pub static mut snapped_deps: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn file_hash_1(mut key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0;
    let mut _key_: *const ::core::ffi::c_uchar =
        (*(key as *const file)).hname as *const ::core::ffi::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as ::core::ffi::c_ulong);
    _result_
}
#[no_mangle]
pub unsafe extern "C" fn file_hash_2(mut _key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0;
    _result_
}
unsafe extern "C" fn file_hash_cmp(
    mut x: *const ::core::ffi::c_void,
    mut y: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if (*(x as *const file)).hname == (*(y as *const file)).hname {
        0
    } else {
        strcmp((*(x as *const file)).hname, (*(y as *const file)).hname)
    }
}
static mut files: hash_table = hash_table {
    ht_vec: ::core::ptr::null::<*mut ::core::ffi::c_void>() as *mut *mut ::core::ffi::c_void,
    ht_hash_1: None,
    ht_hash_2: None,
    ht_compare: None,
    ht_size: 0,
    ht_capacity: 0,
    ht_fill: 0,
    ht_empty_slots: 0,
    ht_collisions: 0,
    ht_lookups: 0,
    ht_rehashes: 0,
    ht_in_map: [0; 1],
    c2rust_padding: [0; 3],
};
static mut rehashed_files: *mut *mut file = ::core::ptr::null::<*mut file>() as *mut *mut file;
static mut rehashed_files_len: size_t = 0;
pub const REHASHED_FILES_INCR: ::core::ffi::c_int = 5;
static mut all_secondary: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn lookup_file(mut name: *const ::core::ffi::c_char) -> *mut file {
    let mut f: *mut file = ::core::ptr::null_mut::<file>();
    let mut file_key: file = file {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        hname: ::core::ptr::null::<::core::ffi::c_char>(),
        vpath: ::core::ptr::null::<::core::ffi::c_char>(),
        deps: ::core::ptr::null_mut::<dep>(),
        cmds: ::core::ptr::null_mut::<commands>(),
        stem: ::core::ptr::null::<::core::ffi::c_char>(),
        also_make: ::core::ptr::null_mut::<dep>(),
        prev: ::core::ptr::null_mut::<file>(),
        last: ::core::ptr::null_mut::<file>(),
        renamed: ::core::ptr::null_mut::<file>(),
        variables: ::core::ptr::null_mut::<variable_set_list>(),
        pat_variables: ::core::ptr::null_mut::<variable_set_list>(),
        parent: ::core::ptr::null_mut::<file>(),
        double_colon: ::core::ptr::null_mut::<file>(),
        last_mtime: 0,
        mtime_before_update: 0,
        considered: 0,
        command_flags: 0,
        update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped_suffix: [0; 4],
        c2rust_padding: [0; 4],
    };
    '_c2rust_label: {
        if *name as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"*name != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/file.c\0" as *const u8 as *const ::core::ffi::c_char,
                92,
                b"struct file *lookup_file(const char *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    while *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32
        && *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as isize)
            as ::core::ffi::c_int
            & 0x8000 as ::core::ffi::c_int
            != 0
        && *name.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
    {
        name = name.offset(2 as ::core::ffi::c_int as isize);
        while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*name as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & 0x8000 as ::core::ffi::c_int
            != 0
        {
            name = name.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    if *name as ::core::ffi::c_int == 0 {
        name = b"./\0" as *const u8 as *const ::core::ffi::c_char;
    }
    file_key.hname = name;
    f = hash_find_item(
        &raw mut files,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) as *mut file;
    f
}
#[no_mangle]
pub unsafe extern "C" fn enter_file(mut name: *const ::core::ffi::c_char) -> *mut file {
    let mut f: *mut file = ::core::ptr::null_mut::<file>();
    let mut new: *mut file = ::core::ptr::null_mut::<file>();
    let mut file_slot: *mut *mut file = ::core::ptr::null_mut::<*mut file>();
    let mut file_key: file = file {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        hname: ::core::ptr::null::<::core::ffi::c_char>(),
        vpath: ::core::ptr::null::<::core::ffi::c_char>(),
        deps: ::core::ptr::null_mut::<dep>(),
        cmds: ::core::ptr::null_mut::<commands>(),
        stem: ::core::ptr::null::<::core::ffi::c_char>(),
        also_make: ::core::ptr::null_mut::<dep>(),
        prev: ::core::ptr::null_mut::<file>(),
        last: ::core::ptr::null_mut::<file>(),
        renamed: ::core::ptr::null_mut::<file>(),
        variables: ::core::ptr::null_mut::<variable_set_list>(),
        pat_variables: ::core::ptr::null_mut::<variable_set_list>(),
        parent: ::core::ptr::null_mut::<file>(),
        double_colon: ::core::ptr::null_mut::<file>(),
        last_mtime: 0,
        mtime_before_update: 0,
        considered: 0,
        command_flags: 0,
        update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped_suffix: [0; 4],
        c2rust_padding: [0; 4],
    };
    '_c2rust_label: {
        if *name as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"*name != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/file.c\0" as *const u8 as *const ::core::ffi::c_char,
                158,
                b"struct file *enter_file(const char *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if verify_flag == 0 || strcache_iscached(name) != 0 {
        } else {
            __assert_fail(
                b"! verify_flag || strcache_iscached (name)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"src/file.c\0" as *const u8 as *const ::core::ffi::c_char,
                159,
                b"struct file *enter_file(const char *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    file_key.hname = name;
    file_slot = hash_find_slot(
        &raw mut files,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) as *mut *mut file;
    f = *file_slot;
    if !(f.is_null()
        || f as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
        && (*f).double_colon.is_null()
    {
        (*f).set_builtin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        return f;
    }
    new = xcalloc(::core::mem::size_of::<file>() as size_t) as *mut file;
    (*new).hname = name;
    (*new).name = (*new).hname;
    (*new).set_update_status(us_none as update_status);
    if f.is_null() || f as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void
    {
        (*new).last = new;
        hash_insert_at(
            &raw mut files,
            new as *const ::core::ffi::c_void,
            file_slot as *const ::core::ffi::c_void,
        );
    } else {
        (*new).double_colon = f;
        (*(*f).last).prev = new;
        (*f).last = new;
    }
    new
}
#[no_mangle]
pub unsafe extern "C" fn rehash_file(
    mut from_file: *mut file,
    mut to_hname: *const ::core::ffi::c_char,
) {
    let mut file_key: file = file {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        hname: ::core::ptr::null::<::core::ffi::c_char>(),
        vpath: ::core::ptr::null::<::core::ffi::c_char>(),
        deps: ::core::ptr::null_mut::<dep>(),
        cmds: ::core::ptr::null_mut::<commands>(),
        stem: ::core::ptr::null::<::core::ffi::c_char>(),
        also_make: ::core::ptr::null_mut::<dep>(),
        prev: ::core::ptr::null_mut::<file>(),
        last: ::core::ptr::null_mut::<file>(),
        renamed: ::core::ptr::null_mut::<file>(),
        variables: ::core::ptr::null_mut::<variable_set_list>(),
        pat_variables: ::core::ptr::null_mut::<variable_set_list>(),
        parent: ::core::ptr::null_mut::<file>(),
        double_colon: ::core::ptr::null_mut::<file>(),
        last_mtime: 0,
        mtime_before_update: 0,
        considered: 0,
        command_flags: 0,
        update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped_suffix: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut file_slot: *mut *mut file = ::core::ptr::null_mut::<*mut file>();
    let mut to_file: *mut file = ::core::ptr::null_mut::<file>();
    let mut deleted_file: *mut file = ::core::ptr::null_mut::<file>();
    let mut f: *mut file = ::core::ptr::null_mut::<file>();
    (*from_file).set_builtin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    file_key.hname = to_hname;
    if file_hash_cmp(
        from_file as *const ::core::ffi::c_void,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) == 0
    {
        return;
    }
    file_key.hname = (*from_file).hname;
    while !(*from_file).renamed.is_null() {
        from_file = (*from_file).renamed;
    }
    if file_hash_cmp(
        from_file as *const ::core::ffi::c_void,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) != 0
    {
        abort();
    }
    deleted_file =
        hash_delete(&raw mut files, from_file as *const ::core::ffi::c_void) as *mut file;
    if deleted_file != from_file {
        abort();
    }
    file_key.hname = to_hname;
    file_slot = hash_find_slot(
        &raw mut files,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) as *mut *mut file;
    to_file = *file_slot;
    (*from_file).hname = to_hname;
    f = (*from_file).double_colon;
    while !f.is_null() {
        (*f).hname = to_hname;
        f = (*f).prev;
    }
    if to_file.is_null()
        || to_file as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void
    {
        hash_insert_at(
            &raw mut files,
            from_file as *const ::core::ffi::c_void,
            file_slot as *const ::core::ffi::c_void,
        );
        return;
    }
    if !(*from_file).cmds.is_null() {
        if (*to_file).cmds.is_null() {
            (*to_file).cmds = (*from_file).cmds;
        } else if (*from_file).cmds != (*to_file).cmds {
            let mut l: size_t = strlen((*from_file).name) as size_t;
            if !(*(*to_file).cmds).fileinfo.filenm.is_null() {
                error(
                    &raw mut (*(*from_file).cmds).fileinfo,
                    l.wrapping_add(strlen((*(*to_file).cmds).fileinfo.filenm) as size_t)
                        .wrapping_add(INTSTR_LENGTH),
                    b"recipe was specified for file '%s' at %s:%lu,\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*from_file).name,
                    (*(*from_file).cmds).fileinfo.filenm,
                    (*(*from_file).cmds).fileinfo.lineno,
                );
            } else {
                error(
                    &raw mut (*(*from_file).cmds).fileinfo,
                    l,
                    b"recipe for file '%s' was found by implicit rule search,\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*from_file).name,
                );
            }
            l = l.wrapping_add(strlen(to_hname) as size_t);
            error(
                &raw mut (*(*from_file).cmds).fileinfo,
                l,
                b"but '%s' is now considered the same file as '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*from_file).name,
                to_hname,
            );
            error(
                &raw mut (*(*from_file).cmds).fileinfo,
                l,
                b"recipe for '%s' will be ignored in favor of the one for '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*from_file).name,
                to_hname,
            );
        }
    }
    if (*to_file).deps.is_null() {
        (*to_file).deps = (*from_file).deps;
    } else {
        let mut deps: *mut dep = (*to_file).deps;
        while !(*deps).next.is_null() {
            deps = (*deps).next;
        }
        (*deps).next = (*from_file).deps;
    }
    merge_variable_set_lists(&raw mut (*to_file).variables, (*from_file).variables);
    if !(*to_file).double_colon.is_null()
        && (*from_file).is_target() as ::core::ffi::c_int != 0
        && (*from_file).double_colon.is_null()
    {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            (strlen((*from_file).name) as size_t).wrapping_add(strlen(to_hname) as size_t),
            b"can't rename single-colon '%s' to double-colon '%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            (*from_file).name,
            to_hname,
        );
    }
    if (*to_file).double_colon.is_null() && !(*from_file).double_colon.is_null() {
        if (*to_file).is_target() != 0 {
            fatal(
                ::core::ptr::null_mut::<Floc>(),
                (strlen((*from_file).name) as size_t).wrapping_add(strlen(to_hname) as size_t),
                b"can't rename double-colon '%s' to single-colon '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*from_file).name,
                to_hname,
            );
        } else {
            (*to_file).double_colon = (*from_file).double_colon;
        }
    }
    if (*from_file).last_mtime > (*to_file).last_mtime {
        (*to_file).last_mtime = (*from_file).last_mtime;
    }
    (*to_file).mtime_before_update = (*from_file).mtime_before_update;
    (*to_file).set_precious(
        (*to_file).precious()
            | (*from_file).precious() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_loaded(
        (*to_file).loaded() | (*from_file).loaded() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_tried_implicit(
        (*to_file).tried_implicit()
            | (*from_file).tried_implicit() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_updating(
        (*to_file).updating()
            | (*from_file).updating() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_updated(
        (*to_file).updated() | (*from_file).updated() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_is_target(
        (*to_file).is_target()
            | (*from_file).is_target() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_cmd_target(
        (*to_file).cmd_target()
            | (*from_file).cmd_target() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_phony(
        (*to_file).phony() | (*from_file).phony() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_is_explicit(
        (*to_file).is_explicit()
            | (*from_file).is_explicit() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_secondary(
        (*to_file).secondary()
            | (*from_file).secondary() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_notintermediate(
        (*to_file).notintermediate()
            | (*from_file).notintermediate() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_ignore_vpath(
        (*to_file).ignore_vpath()
            | (*from_file).ignore_vpath() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_snapped(
        (*to_file).snapped() | (*from_file).snapped() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_suffix(
        (*to_file).suffix() | (*from_file).suffix() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_builtin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*from_file).renamed = to_file;
    if rehashed_files_len.wrapping_rem(REHASHED_FILES_INCR as size_t) == 0 {
        rehashed_files = xrealloc(
            rehashed_files as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut file>() as size_t)
                .wrapping_mul(rehashed_files_len.wrapping_add(REHASHED_FILES_INCR as size_t)),
        ) as *mut *mut file;
    }
    let fresh2 = rehashed_files_len;
    rehashed_files_len = rehashed_files_len.wrapping_add(1);
    let ref mut fresh3 = *rehashed_files.offset(fresh2 as isize);
    *fresh3 = from_file;
}
#[no_mangle]
pub unsafe extern "C" fn rename_file(
    mut from_file: *mut file,
    mut to_hname: *const ::core::ffi::c_char,
) {
    rehash_file(from_file, to_hname);
    while !from_file.is_null() {
        (*from_file).name = (*from_file).hname;
        from_file = (*from_file).prev;
    }
}
#[no_mangle]
pub unsafe extern "C" fn remove_intermediates(mut sig: ::core::ffi::c_int) {
    let mut file_slot: *mut *mut file = ::core::ptr::null_mut::<*mut file>();
    let mut file_end: *mut *mut file = ::core::ptr::null_mut::<*mut file>();
    let mut doneany: ::core::ffi::c_int = 0;
    if question_flag != 0 || touch_flag != 0 || all_secondary != 0 || no_intermediates != 0 {
        return;
    }
    if sig != 0 && just_print_flag != 0 {
        return;
    }
    file_slot = files.ht_vec as *mut *mut file;
    file_end = file_slot.offset(files.ht_size as isize);
    let mut current_block_35: u64;
    while file_slot < file_end {
        if !((*file_slot).is_null()
            || *file_slot as *mut ::core::ffi::c_void
                == hash_deleted_item as *mut ::core::ffi::c_void)
        {
            let mut f: *mut file = *file_slot;
            if (*f).intermediate() as ::core::ffi::c_int != 0
                && ((*f).dontcare() as ::core::ffi::c_int != 0 || (*f).precious() == 0)
                && (*f).secondary() == 0
                && (*f).notintermediate() == 0
                && (*f).cmd_target() == 0
            {
                let mut status: ::core::ffi::c_int = 0;
                if !((*f).update_status() as ::core::ffi::c_int == us_none as ::core::ffi::c_int) {
                    if just_print_flag != 0 {
                        status = 0;
                        current_block_35 = 2979737022853876585;
                    } else {
                        status = unlink((*f).name);
                        if status < 0 && *__errno_location() == ENOENT {
                            current_block_35 = 6873731126896040597;
                        } else {
                            current_block_35 = 2979737022853876585;
                        }
                    }
                    match current_block_35 {
                        6873731126896040597 => {}
                        _ => {
                            if (*f).dontcare() == 0 {
                                if sig != 0 {
                                    error(
                                        ::core::ptr::null_mut::<Floc>(),
                                        strlen((*f).name) as size_t,
                                        b"*** deleting intermediate file '%s'\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        (*f).name,
                                    );
                                } else {
                                    if doneany == 0 {
                                        if 0x1 as ::core::ffi::c_int & db_level != 0 {
                                            printf(
                                                b"Removing intermediate files...\n\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                            fflush(stdout);
                                        }
                                    }
                                    if run_silent == 0 {
                                        if doneany == 0 {
                                            fputs(
                                                b"rm \0" as *const u8 as *const ::core::ffi::c_char,
                                                stdout,
                                            );
                                            doneany = 1;
                                        } else {
                                            putchar(' ' as i32);
                                        }
                                        fputs((*f).name, stdout);
                                        fflush(stdout);
                                    }
                                }
                                if status < 0 {
                                    if doneany != 0 {
                                        fputs(
                                            b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                                            stdout,
                                        );
                                    }
                                    fflush(stdout);
                                    perror_with_name(
                                        b"unlink: \0" as *const u8 as *const ::core::ffi::c_char,
                                        (*f).name,
                                    );
                                    doneany = 0;
                                }
                            }
                        }
                    }
                }
            }
        }
        file_slot = file_slot.offset(1 as ::core::ffi::c_int as isize);
    }
    if doneany != 0 && sig == 0 {
        putchar('\n' as i32);
        fflush(stdout);
    }
}
#[no_mangle]
pub unsafe extern "C" fn split_prereqs(mut p: *mut ::core::ffi::c_char) -> *mut dep {
    let mut new: *mut dep = parse_file_seq(
        &raw mut p,
        ::core::mem::size_of::<dep>() as size_t,
        0x100 as ::core::ffi::c_int,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0x40 as ::core::ffi::c_int,
    ) as *mut dep;
    if *p != 0 {
        let mut ood: *mut dep = ::core::ptr::null_mut::<dep>();
        p = p.offset(1 as ::core::ffi::c_int as isize);
        ood = parse_file_seq(
            &raw mut p,
            ::core::mem::size_of::<dep>() as size_t,
            0x1 as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0x40 as ::core::ffi::c_int,
        ) as *mut dep;
        if new.is_null() {
            new = ood;
        } else {
            let mut dp: *mut dep = ::core::ptr::null_mut::<dep>();
            dp = new;
            while !(*dp).next.is_null() {
                dp = (*dp).next;
            }
            (*dp).next = ood;
        }
        while !ood.is_null() {
            (*ood).set_ignore_mtime(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            ood = (*ood).next;
        }
    }
    new
}
#[no_mangle]
pub unsafe extern "C" fn enter_prereqs(
    mut deps: *mut dep,
    mut stem: *const ::core::ffi::c_char,
) -> *mut dep {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut d1: *mut dep = ::core::ptr::null_mut::<dep>();
    if deps.is_null() {
        return ::core::ptr::null_mut::<dep>();
    }
    if !stem.is_null() {
        let mut pattern: *const ::core::ffi::c_char =
            b"%\0" as *const u8 as *const ::core::ffi::c_char;
        let mut dp: *mut dep = deps;
        let mut dl: *mut dep = ::core::ptr::null_mut::<dep>();
        while !dp.is_null() {
            let mut percent: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut nl: size_t = (strlen((*dp).name) as size_t).wrapping_add(1);
            alloca_allocations.push(::std::vec::from_elem(0, nl as usize));
            let mut nm: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                nm as *mut ::core::ffi::c_void,
                (*dp).name as *const ::core::ffi::c_void,
                nl as size_t,
            );
            percent = find_percent(nm);
            if !percent.is_null() {
                let mut o: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                if *stem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0
                {
                    memmove(
                        percent as *mut ::core::ffi::c_void,
                        percent.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                        strlen(percent),
                    );
                    o = variable_buffer_output(
                        variable_buffer,
                        nm,
                        (strlen(nm) as size_t).wrapping_add(1),
                    );
                } else {
                    o = patsubst_expand_pat(
                        variable_buffer,
                        stem,
                        pattern,
                        nm,
                        pattern.offset(1 as ::core::ffi::c_int as isize),
                        percent.offset(1 as ::core::ffi::c_int as isize),
                    );
                }
                if *variable_buffer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0
                {
                    let mut df: *mut dep = dp;
                    if dp == deps {
                        deps = (*deps).next;
                        dp = deps;
                    } else {
                        (*dl).next = (*dp).next;
                        dp = (*dl).next;
                    }
                    free_dep(df);
                    continue;
                } else {
                    (*dp).name = strcache_add_len(
                        variable_buffer,
                        o.offset_from(variable_buffer) as ::core::ffi::c_long as size_t,
                    );
                }
            }
            (*dp).stem = stem;
            (*dp).set_staticpattern(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            dl = dp;
            dp = (*dp).next;
        }
    }
    d1 = deps;
    while !d1.is_null() {
        if !((*d1).need_2nd_expansion() != 0) {
            (*d1).file = lookup_file((*d1).name);
            if (*d1).file.is_null() {
                (*d1).file = enter_file((*d1).name);
            }
            (*d1).set_staticpattern(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*d1).name = ::core::ptr::null::<::core::ffi::c_char>();
            if stem.is_null() {
                (*(*d1).file).set_is_explicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        d1 = (*d1).next;
    }
    deps
}
#[no_mangle]
pub unsafe extern "C" fn expand_deps(mut f: *mut file) {
    let mut d: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut dp: *mut *mut dep = ::core::ptr::null_mut::<*mut dep>();
    let mut fstem: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut initialized: ::core::ffi::c_int = 0;
    let mut changed_dep: ::core::ffi::c_int = 0;
    if (*f).snapped() != 0 {
        return;
    }
    (*f).set_snapped(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    dp = &raw mut (*f).deps;
    d = (*f).deps;
    while !d.is_null() {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut new: *mut dep = ::core::ptr::null_mut::<dep>();
        let mut next: *mut dep = ::core::ptr::null_mut::<dep>();
        if (*d).name.is_null() || (*d).need_2nd_expansion() == 0 {
            dp = &raw mut (*d).next;
            d = (*d).next;
        } else {
            if (*d).staticpattern() != 0 {
                let mut cs: *const ::core::ffi::c_char = (*d).name;
                let mut nperc: size_t = 0;
                loop {
                    cs = strchr(cs, '%' as i32);
                    if cs.is_null() {
                        break;
                    }
                    nperc = nperc.wrapping_add(1);
                    cs = cs.offset(1 as ::core::ffi::c_int as isize);
                }
                if nperc != 0 {
                    let mut slen: size_t = (strlen((*d).name) as size_t)
                        .wrapping_add(nperc)
                        .wrapping_add(1);
                    let mut pcs: *const ::core::ffi::c_char = (*d).name;
                    let mut name: *mut ::core::ffi::c_char =
                        xmalloc(slen) as *mut ::core::ffi::c_char;
                    let mut s: *mut ::core::ffi::c_char = name;
                    cs = strchr(pcs, '%' as i32);
                    while !cs.is_null() {
                        s = mempcpy(
                            s as *mut ::core::ffi::c_void,
                            pcs as *const ::core::ffi::c_void,
                            cs.offset_from(pcs) as ::core::ffi::c_long as size_t,
                        ) as *mut ::core::ffi::c_char;
                        let fresh0 = s;
                        s = s.offset(1 as ::core::ffi::c_int as isize);
                        *fresh0 = '$' as i32 as ::core::ffi::c_char;
                        let fresh1 = s;
                        s = s.offset(1 as ::core::ffi::c_int as isize);
                        *fresh1 = '*' as i32 as ::core::ffi::c_char;
                        cs = cs.offset(1 as ::core::ffi::c_int as isize);
                        pcs = cs;
                        cs = strchr(end_of_token(cs), '%' as i32);
                    }
                    strcpy(s, pcs);
                    free((*d).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
                    (*d).name = name;
                }
            }
            if initialized == 0 {
                initialize_file_variables(f, 0);
                initialized = 1;
            }
            set_file_variables(
                f,
                if !(*d).stem.is_null() {
                    (*d).stem
                } else {
                    (*f).stem
                },
            );
            p = expand_string_for_file((*d).name, f);
            free((*d).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
            new = split_prereqs(p);
            if new.is_null() {
                *dp = (*d).next;
                changed_dep = 1;
                free_dep(d);
                d = *dp;
            } else {
                fstem = (*d).stem;
                next = (*d).next;
                changed_dep = 1;
                free_dep(d);
                *dp = new;
                dp = &raw mut new;
                d = new;
                while !d.is_null() {
                    (*d).file = lookup_file((*d).name);
                    if (*d).file.is_null() {
                        (*d).file = enter_file((*d).name);
                    }
                    (*d).name = ::core::ptr::null::<::core::ffi::c_char>();
                    (*d).stem = fstem;
                    if fstem.is_null() {
                        (*(*d).file)
                            .set_is_explicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                    dp = &raw mut (*d).next;
                    d = (*d).next;
                }
                *dp = next;
                d = *dp;
            }
        }
    }
    if changed_dep != 0 {
        shuffle_deps_recursive((*f).deps);
    }
}
#[no_mangle]
pub unsafe extern "C" fn expand_extra_prereqs(mut extra: *const variable) -> *mut dep {
    let mut d: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut prereqs: *mut dep = if !extra.is_null() {
        split_prereqs(expand_string_buf(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            (*extra).value,
            SIZE_MAX as size_t,
        ))
    } else {
        ::core::ptr::null_mut::<dep>()
    };
    d = prereqs;
    while !d.is_null() {
        (*d).file = lookup_file((*d).name);
        if (*d).file.is_null() {
            (*d).file = enter_file((*d).name);
        }
        (*d).name = ::core::ptr::null::<::core::ffi::c_char>();
        (*d).set_ignore_automatic_vars(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        d = (*d).next;
    }
    prereqs
}
#[no_mangle]
pub unsafe extern "C" fn snap_file(mut f: *mut file, mut deps: *const dep) {
    let mut prereqs: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut d: *mut dep = ::core::ptr::null_mut::<dep>();
    if second_expansion == 0 {
        (*f).set_updating(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if all_secondary != 0 && (*f).notintermediate() == 0 {
        (*f).set_intermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if no_intermediates != 0 && (*f).intermediate() == 0 && (*f).secondary() == 0 {
        (*f).set_notintermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if !(*f).variables.is_null() {
        prereqs = expand_extra_prereqs(lookup_variable_in_set(
            b".EXTRA_PREREQS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t)
                .wrapping_sub(1),
            (*(*f).variables).set,
        ));
        if second_expansion != 0 {
            d = prereqs;
            while !d.is_null() {
                if (*d).name.is_null() {
                    (*d).name = xstrdup((*(*d).file).name);
                }
                (*d).set_need_2nd_expansion(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                d = (*d).next;
            }
        }
    } else if (*f).is_target() != 0 {
        prereqs = copy_dep_chain(deps);
    }
    if !prereqs.is_null() {
        d = prereqs;
        while !d.is_null() {
            if *(*f).name as ::core::ffi::c_int
                == *(if !(*d).name.is_null() {
                    (*d).name
                } else {
                    (*(*d).file).name
                }) as ::core::ffi::c_int
                && (*(*f).name as ::core::ffi::c_int == 0
                    || strcmp(
                        (*f).name.offset(1 as ::core::ffi::c_int as isize),
                        (if !(*d).name.is_null() {
                            (*d).name
                        } else {
                            (*(*d).file).name
                        }) . offset ( 1 ) ,
                    ) == 0)
            {
                break;
            }
            d = (*d).next;
        }
        if !d.is_null() {
            free_dep_chain(prereqs);
        } else if (*f).deps.is_null() {
            (*f).deps = prereqs;
        } else {
            d = (*f).deps;
            while !(*d).next.is_null() {
                d = (*d).next;
            }
            (*d).next = prereqs;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn snap_deps() {
    let mut f: *mut file = ::core::ptr::null_mut::<file>();
    let mut f2: *mut file = ::core::ptr::null_mut::<file>();
    let mut d: *mut dep = ::core::ptr::null_mut::<dep>();
    snapped_deps = 1;
    f = lookup_file(b".PRECIOUS\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).set_precious(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".LOW_RESOLUTION_TIME\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).set_low_resolution_time(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".PHONY\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).set_phony(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*f2).set_is_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*f2).last_mtime = NONEXISTENT_MTIME as uintmax_t;
                (*f2).mtime_before_update = NONEXISTENT_MTIME as uintmax_t;
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".NOTINTERMEDIATE\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        if !(*f).deps.is_null() {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).set_notintermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        } else {
            no_intermediates = 1;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".INTERMEDIATE\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                if (*f2).notintermediate() != 0 {
                    fatal(
                        ::core::ptr::null_mut::<Floc>(),
                        strlen((*f2).name) as size_t,
                        b"%s cannot be both .NOTINTERMEDIATE and .INTERMEDIATE\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*f2).name,
                    );
                } else {
                    (*f2).set_intermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".SECONDARY\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        if !(*f).deps.is_null() {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    if (*f2).notintermediate() != 0 {
                        fatal(
                            ::core::ptr::null_mut::<Floc>(),
                            strlen((*f2).name) as size_t,
                            b"%s cannot be both .NOTINTERMEDIATE and .SECONDARY\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*f2).name,
                        );
                    } else {
                        let mut rhs = {
                            (*f2).set_secondary(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            (*f2).secondary()
                        } as ::core::ffi::c_uint;
                        (*f2).set_intermediate(rhs);
                    }
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        } else {
            all_secondary = 1;
        }
        f = (*f).prev;
    }
    if no_intermediates != 0 && all_secondary != 0 {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            0,
            b".NOTINTERMEDIATE and .SECONDARY are mutually exclusive\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    f = lookup_file(b".EXPORT_ALL_VARIABLES\0" as *const u8 as *const ::core::ffi::c_char);
    if !f.is_null() && (*f).is_target() as ::core::ffi::c_int != 0 {
        export_all_variables = 1;
    }
    f = lookup_file(b".IGNORE\0" as *const u8 as *const ::core::ffi::c_char);
    if !f.is_null() && (*f).is_target() as ::core::ffi::c_int != 0 {
        if (*f).deps.is_null() {
            ignore_errors_flag = 1;
        } else {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).command_flags |= COMMANDS_NOERROR;
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        }
    }
    f = lookup_file(b".SILENT\0" as *const u8 as *const ::core::ffi::c_char);
    if !f.is_null() && (*f).is_target() as ::core::ffi::c_int != 0 {
        if (*f).deps.is_null() {
            run_silent = 1;
        } else {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).command_flags |= COMMANDS_SILENT;
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        }
    }
    f = lookup_file(b".NOTPARALLEL\0" as *const u8 as *const ::core::ffi::c_char);
    if !f.is_null() && (*f).is_target() as ::core::ffi::c_int != 0 {
        let mut d2: *mut dep = ::core::ptr::null_mut::<dep>();
        if (*f).deps.is_null() {
            not_parallel = 1;
        } else {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    if !(*f2).deps.is_null() {
                        d2 = (*(*f2).deps).next;
                        while !d2.is_null() {
                            (*d2).set_wait_here(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            d2 = (*d2).next;
                        }
                    }
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        }
    }
    let mut prereqs: *mut dep = expand_extra_prereqs(lookup_variable(
        b".EXTRA_PREREQS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t).wrapping_sub(1),
    ));
    let mut filedump: *mut *mut ::core::ffi::c_void = hash_dump(
        &raw mut files,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        None,
    );
    let mut filep: *mut *mut ::core::ffi::c_void = filedump;
    while !(*filep).is_null() {
        snap_file(*filep as *mut file, prereqs);
        filep = filep.offset(1 as ::core::ffi::c_int as isize);
    }
    free(filedump as *mut ::core::ffi::c_void);
    free_dep_chain(prereqs);
}
#[no_mangle]
pub unsafe extern "C" fn set_command_state(mut file: *mut file, mut state: cmd_state) {
    let mut d: *mut dep = ::core::ptr::null_mut::<dep>();
    (*file).set_command_state(state as cmd_state as cmd_state);
    d = (*file).also_make;
    while !d.is_null() {
        if state as ::core::ffi::c_uint > (*(*d).file).command_state() as ::core::ffi::c_uint {
            (*(*d).file).set_command_state(state as cmd_state as cmd_state);
        }
        d = (*d).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn file_timestamp_cons(
    mut fname: *const ::core::ffi::c_char,
    mut stamp: time_t,
    mut ns: ::core::ffi::c_long,
) -> uintmax_t {
    let mut offset: ::core::ffi::c_int = (ORDINARY_MTIME_MIN as ::core::ffi::c_long
        + (if FILE_TIMESTAMP_HI_RES != 0 {
            ns
        } else {
            0
        })) as ::core::ffi::c_int;
    let mut s: uintmax_t = stamp as uintmax_t;
    let mut product: uintmax_t = s
        << (if FILE_TIMESTAMP_HI_RES != 0 {
            30
        } else {
            0
        });
    let mut ts: uintmax_t = product.wrapping_add(offset as uintmax_t);
    if !(s
        <= ((!(0 as ::core::ffi::c_int as uintmax_t))
            .wrapping_sub(
                if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                    0 as ::core::ffi::c_int as uintmax_t
                } else {
                    !(0 as ::core::ffi::c_int as uintmax_t)
                        << (::core::mem::size_of::<uintmax_t>() as usize)
                            .wrapping_mul(8 as usize)
                            .wrapping_sub(1 as usize)
                },
            )
            .wrapping_sub((2 + 1) as uintmax_t)
            >> (if 1 != 0 {
                30
            } else {
                0
            })
            << (if 1 != 0 {
                30
            } else {
                0
            }))
        .wrapping_add((2 + 1) as uintmax_t)
        .wrapping_add(
            (if 1 != 0 {
                1000000000 as ::core::ffi::c_int
            } else {
                1
            }) as uintmax_t,
        )
        .wrapping_sub(1 as uintmax_t)
        .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
            >> (if FILE_TIMESTAMP_HI_RES != 0 {
                30
            } else {
                0
            })
        && product <= ts
        && ts
            <= ((!(0 as ::core::ffi::c_int as uintmax_t))
                .wrapping_sub(
                    if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                        0 as ::core::ffi::c_int as uintmax_t
                    } else {
                        !(0 as ::core::ffi::c_int as uintmax_t)
                            << (::core::mem::size_of::<uintmax_t>() as usize)
                                .wrapping_mul(8 as usize)
                                .wrapping_sub(1 as usize)
                    },
                )
                .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                >> (if FILE_TIMESTAMP_HI_RES != 0 {
                    30
                } else {
                    0
                })
                << (if FILE_TIMESTAMP_HI_RES != 0 {
                    30
                } else {
                    0
                }))
            .wrapping_add(ORDINARY_MTIME_MIN as uintmax_t)
            .wrapping_add(
                (if FILE_TIMESTAMP_HI_RES != 0 {
                    1000000000 as ::core::ffi::c_int
                } else {
                    1
                }) as uintmax_t,
            )
            .wrapping_sub(1 as uintmax_t))
    {
        let mut buf: [::core::ffi::c_char; 43] = [0; 43];
        let mut f: *const ::core::ffi::c_char = if !fname.is_null() {
            fname
        } else {
            b"Current time\0" as *const u8 as *const ::core::ffi::c_char
        };
        ts = if s <= OLD_MTIME as uintmax_t {
            ORDINARY_MTIME_MIN as uintmax_t
        } else {
            ((!(0 as ::core::ffi::c_int as uintmax_t))
                .wrapping_sub(
                    if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                        0 as ::core::ffi::c_int as uintmax_t
                    } else {
                        !(0 as ::core::ffi::c_int as uintmax_t)
                            << (::core::mem::size_of::<uintmax_t>() as usize)
                                .wrapping_mul(8 as usize)
                                .wrapping_sub(1 as usize)
                    },
                )
                .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                >> (if FILE_TIMESTAMP_HI_RES != 0 {
                    30
                } else {
                    0
                })
                << (if FILE_TIMESTAMP_HI_RES != 0 {
                    30
                } else {
                    0
                }))
            .wrapping_add(ORDINARY_MTIME_MIN as uintmax_t)
            .wrapping_add(
                (if FILE_TIMESTAMP_HI_RES != 0 {
                    1000000000 as ::core::ffi::c_int
                } else {
                    1
                }) as uintmax_t,
            )
            .wrapping_sub(1 as uintmax_t)
        };
        file_timestamp_sprintf(&raw mut buf as *mut ::core::ffi::c_char, ts);
        error(
            ::core::ptr::null_mut::<Floc>(),
            (strlen(f) as size_t)
                .wrapping_add(strlen(&raw mut buf as *mut ::core::ffi::c_char) as size_t),
            b"%s: timestamp out of range: substituting %s\0" as *const u8
                as *const ::core::ffi::c_char,
            f,
            &raw mut buf as *mut ::core::ffi::c_char,
        );
    }
    ts
}
#[no_mangle]
pub unsafe extern "C" fn file_timestamp_now(mut resolution: *mut ::core::ffi::c_int) -> uintmax_t {
    let mut r: ::core::ffi::c_int = 0;
    let mut s: time_t = 0;
    let mut ns: ::core::ffi::c_int = 0;
    let mut timespec: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_REALTIME, &raw mut timespec) == 0 {
        r = 1;
        s = timespec.tv_sec as time_t;
        ns = timespec.tv_nsec as ::core::ffi::c_int;
    } else {
        let mut timeval: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        if gettimeofday(
            &raw mut timeval,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) == 0
        {
            r = 1000 as ::core::ffi::c_int;
            s = timeval.tv_sec as time_t;
            ns = (timeval.tv_usec * 1000 as __suseconds_t) as ::core::ffi::c_int;
        } else {
            r = 1000000000 as ::core::ffi::c_int;
            s = time(::core::ptr::null_mut::<time_t>());
            ns = 0;
        }
    }
    *resolution = r;
    file_timestamp_cons(
        ::core::ptr::null::<::core::ffi::c_char>(),
        s,
        ns as ::core::ffi::c_long,
    )
}
#[no_mangle]
pub unsafe extern "C" fn file_timestamp_sprintf(
    mut p: *mut ::core::ffi::c_char,
    mut ts: uintmax_t,
) {
    let mut t: time_t = (ts.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
        >> (if FILE_TIMESTAMP_HI_RES != 0 {
            30
        } else {
            0
        })) as time_t;
    let mut tm: *mut tm = localtime(&raw mut t);
    if !tm.is_null() {
        let mut year: intmax_t = (*tm).tm_year as intmax_t;
        p = p.offset(sprintf(
            p,
            b"%04ld-%02d-%02d %02d:%02d:%02d\0" as *const u8 as *const ::core::ffi::c_char,
            year + 1900 as intmax_t,
            (*tm).tm_mon + 1,
            (*tm).tm_mday,
            (*tm).tm_hour,
            (*tm).tm_min,
            (*tm).tm_sec,
        ) as isize);
    } else if t < 0 as time_t {
        p = p.offset(sprintf(
            p,
            b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
            t as intmax_t,
        ) as isize);
    } else {
        p = p.offset(sprintf(
            p,
            b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
            t as uintmax_t,
        ) as isize);
    }
    p = p.offset(
        (sprintf(
            p,
            b".%09d\0" as *const u8 as *const ::core::ffi::c_char,
            (ts.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                & (((1)
                    << (if FILE_TIMESTAMP_HI_RES != 0 {
                        30
                    } else {
                        0
                    }))
                    - 1) as uintmax_t) as ::core::ffi::c_int,
        ) - 1) as isize,
    );
    while *p as ::core::ffi::c_int == '0' as i32 {
        p = p.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    p = p.offset((*p as ::core::ffi::c_int != '.' as i32) as ::core::ffi::c_int as isize);
    *p = 0;
}
#[no_mangle]
pub unsafe extern "C" fn print_prereqs(mut deps: *const dep) {
    let mut ood: *const dep = ::core::ptr::null::<dep>();
    while !deps.is_null() {
        if (*deps).ignore_mtime() == 0 {
            printf(
                b" %s%s\0" as *const u8 as *const ::core::ffi::c_char,
                if (*deps).wait_here() as ::core::ffi::c_int != 0 {
                    b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                if !(*deps).name.is_null() {
                    (*deps).name
                } else {
                    (*(*deps).file).name
                },
            );
        } else if ood.is_null() {
            ood = deps;
        }
        deps = (*deps).next;
    }
    if !ood.is_null() {
        printf(
            b" | %s%s\0" as *const u8 as *const ::core::ffi::c_char,
            if (*ood).wait_here() as ::core::ffi::c_int != 0 {
                b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            if !(*ood).name.is_null() {
                (*ood).name
            } else {
                (*(*ood).file).name
            },
        );
        ood = (*ood).next;
        while !ood.is_null() {
            if (*ood).ignore_mtime() != 0 {
                printf(
                    b" %s%s\0" as *const u8 as *const ::core::ffi::c_char,
                    if (*ood).wait_here() as ::core::ffi::c_int != 0 {
                        b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    if !(*ood).name.is_null() {
                        (*ood).name
                    } else {
                        (*(*ood).file).name
                    },
                );
            }
            ood = (*ood).next;
        }
    }
    putchar('\n' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn print_file(mut item: *const ::core::ffi::c_void) {
    let mut f: *const file = item as *const file;
    if no_builtin_rules_flag != 0 && (*f).builtin() as ::core::ffi::c_int != 0 {
        return;
    }
    putchar('\n' as i32);
    if !(*f).cmds.is_null()
        && (*(*f).cmds).recipe_prefix as ::core::ffi::c_int != cmd_prefix as ::core::ffi::c_int
    {
        fputs(
            b".RECIPEPREFIX = \0" as *const u8 as *const ::core::ffi::c_char,
            stdout,
        );
        cmd_prefix = (*(*f).cmds).recipe_prefix;
        if cmd_prefix as ::core::ffi::c_int != RECIPEPREFIX_DEFAULT {
            putchar(cmd_prefix as ::core::ffi::c_int);
        }
        putchar('\n' as i32);
    }
    if !(*f).variables.is_null() {
        print_target_variables(f);
    }
    if (*f).is_target() == 0 {
        puts(b"# Not a target:\0" as *const u8 as *const ::core::ffi::c_char);
    }
    printf(
        b"%s:%s\0" as *const u8 as *const ::core::ffi::c_char,
        (*f).name,
        if !(*f).double_colon.is_null() {
            b":\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    print_prereqs((*f).deps);
    if (*f).precious() != 0 {
        puts(
            b"#  Precious file (prerequisite of .PRECIOUS).\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).phony() != 0 {
        puts(
            b"#  Phony target (prerequisite of .PHONY).\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).cmd_target() != 0 {
        puts(b"#  Command line target.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*f).dontcare() != 0 {
        puts(
            b"#  A default, MAKEFILES, or -include/sinclude makefile.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).builtin() != 0 {
        puts(b"#  Builtin rule\0" as *const u8 as *const ::core::ffi::c_char);
    }
    puts(if (*f).tried_implicit() as ::core::ffi::c_int != 0 {
        b"#  Implicit rule search has been done.\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"#  Implicit rule search has not been done.\0" as *const u8 as *const ::core::ffi::c_char
    });
    if !(*f).stem.is_null() {
        printf(
            b"#  Implicit/static pattern stem: '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).stem,
        );
    }
    if (*f).intermediate() != 0 {
        puts(
            b"#  File is an intermediate prerequisite.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).notintermediate() != 0 {
        puts(
            b"#  File is a prerequisite of .NOTINTERMEDIATE.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).secondary() != 0 {
        puts(
            b"#  File is secondary (prerequisite of .SECONDARY).\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).is_explicit() != 0 {
        puts(b"#  File is explicitly mentioned.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if !(*f).also_make.is_null() {
        let mut d: *const dep = ::core::ptr::null::<dep>();
        fputs(
            b"#  Also makes:\0" as *const u8 as *const ::core::ffi::c_char,
            stdout,
        );
        d = (*f).also_make;
        while !d.is_null() {
            printf(
                b" %s\0" as *const u8 as *const ::core::ffi::c_char,
                if !(*d).name.is_null() {
                    (*d).name
                } else {
                    (*(*d).file).name
                },
            );
            d = (*d).next;
        }
        putchar('\n' as i32);
    }
    if (*f).last_mtime == UNKNOWN_MTIME as uintmax_t {
        puts(b"#  Modification time never checked.\0" as *const u8 as *const ::core::ffi::c_char);
    } else if (*f).last_mtime == NONEXISTENT_MTIME as uintmax_t {
        puts(b"#  File does not exist.\0" as *const u8 as *const ::core::ffi::c_char);
    } else if (*f).last_mtime == OLD_MTIME as uintmax_t {
        puts(b"#  File is very old.\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        let mut buf: [::core::ffi::c_char; 43] = [0; 43];
        file_timestamp_sprintf(&raw mut buf as *mut ::core::ffi::c_char, (*f).last_mtime);
        printf(
            b"#  Last modified %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
        );
    }
    puts(if (*f).updated() as ::core::ffi::c_int != 0 {
        b"#  File has been updated.\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"#  File has not been updated.\0" as *const u8 as *const ::core::ffi::c_char
    });
    match (*f).command_state() as ::core::ffi::c_int {
        2 => {
            puts(
                b"#  Recipe currently running (THIS IS A BUG).\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        1 => {
            puts(
                b"#  Dependencies recipe running (THIS IS A BUG).\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        0 | 3 => match (*f).update_status() as ::core::ffi::c_int {
            0 => {
                puts(b"#  Successfully updated.\0" as *const u8 as *const ::core::ffi::c_char);
            }
            2 => {
                '_c2rust_label: {
                    if question_flag != 0 {
                    } else {
                        __assert_fail(
                            b"question_flag\0" as *const u8 as *const ::core::ffi::c_char,
                            b"src/file.c\0" as *const u8 as *const ::core::ffi::c_char,
                            1181 as ::core::ffi::c_uint,
                            b"void print_file(const void *)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                puts(
                    b"#  Needs to be updated (-q is set).\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            3 => {
                puts(b"#  Failed to be updated.\0" as *const u8 as *const ::core::ffi::c_char);
            }
            1 | _ => {}
        },
        _ => {
            puts(
                b"#  Invalid value in 'command_state' member!\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            fflush(stdout);
            fflush(stderr);
            abort();
        }
    }
    if !(*f).variables.is_null() {
        print_file_variables(f);
    }
    if !(*f).cmds.is_null() {
        print_commands((*f).cmds);
    }
    if !(*f).prev.is_null() {
        print_file((*f).prev as *const ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_file_data_base() {
    puts(b"\n# Files\0" as *const u8 as *const ::core::ffi::c_char);
    hash_map(
        &raw mut files,
        Some(print_file as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ()),
    );
    fputs(
        b"\n# files hash-table stats:\n# \0" as *const u8 as *const ::core::ffi::c_char,
        stdout,
    );
    hash_print_stats(&raw mut files, stdout);
}
#[no_mangle]
pub unsafe extern "C" fn print_target(mut item: *const ::core::ffi::c_void) {
    let mut f: *const file = item as *const file;
    if (*f).is_target() == 0 || (*f).suffix() as ::core::ffi::c_int != 0 {
        return;
    }
    if *(*f).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32
        && *(*__ctype_b_loc()).offset(*(*f).name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & _ISupper as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
    {
        let mut cp: *const ::core::ffi::c_char = (*f).name.offset(1 as ::core::ffi::c_int as isize);
        loop {
            cp = cp.offset(1 as ::core::ffi::c_int as isize);
            if !(*cp as ::core::ffi::c_int != 0) {
                break;
            }
            if *(*__ctype_b_loc())
                .offset(*cp as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISupper as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                == 0
            {
                break;
            }
        }
        if *cp as ::core::ffi::c_int == 0 {
            return;
        }
    }
    puts((*f).name);
}
#[no_mangle]
pub unsafe extern "C" fn print_targets() {
    hash_map(
        &raw mut files,
        Some(print_target as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn verify_file(mut item: *const ::core::ffi::c_void) {
    let mut f: *const file = item as *const file;
    let mut d: *const dep = ::core::ptr::null::<dep>();
    if !(*f).name.is_null()
        && *(*f).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && strcache_iscached((*f).name) == 0
    {
        error(
            ::core::ptr::null::<Floc>(),
            (strlen((*f).name) as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t)
                        .wrapping_sub(1),
                )
                .wrapping_add(strlen((*f).name) as size_t),
            b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).name,
            b"name\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).name,
        );
    }
    if !(*f).hname.is_null()
        && *(*f).hname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && strcache_iscached((*f).hname) == 0
    {
        error(
            ::core::ptr::null::<Floc>(),
            (strlen((*f).name) as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t)
                        .wrapping_sub(1),
                )
                .wrapping_add(strlen((*f).hname) as size_t),
            b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).name,
            b"hname\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).hname,
        );
    }
    if !(*f).vpath.is_null()
        && *(*f).vpath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && strcache_iscached((*f).vpath) == 0
    {
        error(
            ::core::ptr::null::<Floc>(),
            (strlen((*f).name) as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t)
                        .wrapping_sub(1),
                )
                .wrapping_add(strlen((*f).vpath) as size_t),
            b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).name,
            b"vpath\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).vpath,
        );
    }
    if !(*f).stem.is_null()
        && *(*f).stem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && strcache_iscached((*f).stem) == 0
    {
        error(
            ::core::ptr::null::<Floc>(),
            (strlen((*f).name) as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t)
                        .wrapping_sub(1),
                )
                .wrapping_add(strlen((*f).stem) as size_t),
            b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).name,
            b"stem\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).stem,
        );
    }
    d = (*f).deps;
    while !d.is_null() {
        if (*d).need_2nd_expansion() == 0 {
            if !(*d).name.is_null()
                && *(*d).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
                && strcache_iscached((*d).name) == 0
            {
                error(
                    ::core::ptr::null::<Floc>(),
                    (strlen((*d).name) as size_t)
                        .wrapping_add(
                            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t)
                                .wrapping_sub(1),
                        )
                        .wrapping_add(strlen((*d).name) as size_t),
                    b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*d).name,
                    b"name\0" as *const u8 as *const ::core::ffi::c_char,
                    (*d).name,
                );
            }
        }
        if !(*d).stem.is_null()
            && *(*d).stem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && strcache_iscached((*d).stem) == 0
        {
            error(
                ::core::ptr::null::<Floc>(),
                (strlen((*d).name) as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t)
                            .wrapping_sub(1),
                    )
                    .wrapping_add(strlen((*d).stem) as size_t),
                b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*d).name,
                b"stem\0" as *const u8 as *const ::core::ffi::c_char,
                (*d).stem,
            );
        }
        d = (*d).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn verify_file_data_base() {
    hash_map(
        &raw mut files,
        Some(verify_file as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn build_target_list(
    mut value: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    static mut last_targ_count: ::core::ffi::c_ulong = 0;
    if files.ht_fill != last_targ_count {
        let mut max: size_t = (strlen(value) as size_t)
            .wrapping_div(500)
            .wrapping_add(1)
            .wrapping_mul(500);
        let mut len: size_t = 0;
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut fp: *mut *mut file = files.ht_vec as *mut *mut file;
        let mut end: *mut *mut file = fp.offset(files.ht_size as isize) as *mut *mut file;
        value = xrealloc(value as *mut ::core::ffi::c_void, max) as *mut ::core::ffi::c_char;
        p = value;
        len = 0;
        while fp < end {
            if !((*fp).is_null()
                || *fp as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
                && (**fp).is_target() as ::core::ffi::c_int != 0
            {
                let mut f: *mut file = *fp;
                let mut l: size_t = strlen((*f).name) as size_t;
                len = len.wrapping_add(l.wrapping_add(1));
                if len > max {
                    let mut off: size_t = p.offset_from(value) as ::core::ffi::c_long as size_t;
                    max = max.wrapping_add(
                        l.wrapping_add(1)
                            .wrapping_div(500)
                            .wrapping_add(1)
                            .wrapping_mul(500),
                    );
                    value = xrealloc(value as *mut ::core::ffi::c_void, max)
                        as *mut ::core::ffi::c_char;
                    p = value.offset(off as isize) as *mut ::core::ffi::c_char;
                }
                p = mempcpy(
                    p as *mut ::core::ffi::c_void,
                    (*f).name as *const ::core::ffi::c_void,
                    l as size_t,
                ) as *mut ::core::ffi::c_char;
                let fresh4 = p;
                p = p.offset(1 as ::core::ffi::c_int as isize);
                *fresh4 = ' ' as i32 as ::core::ffi::c_char;
            }
            fp = fp.offset(1 as ::core::ffi::c_int as isize);
        }
        *p.offset(-(1 as ::core::ffi::c_int as isize)) = 0;
        last_targ_count = files.ht_fill;
    }
    value
}
#[no_mangle]
pub unsafe extern "C" fn init_hash_files() {
    hash_init(
        &raw mut files,
        1000 as ::core::ffi::c_ulong,
        Some(
            file_hash_1 as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            file_hash_2 as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            file_hash_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
}
pub const FILE_TIMESTAMP_HI_RES: ::core::ffi::c_int = 1;
