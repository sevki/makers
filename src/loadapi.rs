use libc::{free};
use ::c2rust_bitfields;
extern "C" {
    pub type commands;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xstrdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut reading_file: *const floc;
    fn eval_buffer(buffer: *mut ::core::ffi::c_char, floc: *const floc);
    fn install_variable_buffer(bufp: *mut *mut ::core::ffi::c_char, lenp: *mut size_t);
    fn restore_variable_buffer(buf: *mut ::core::ffi::c_char, len: size_t);
    fn allocated_expand_string_for_file(
        line: *const ::core::ffi::c_char,
        file: *mut file,
    ) -> *mut ::core::ffi::c_char;
    fn define_new_function(
        flocp: *const floc,
        name: *const ::core::ffi::c_char,
        min: ::core::ffi::c_uint,
        max: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
        func: gmk_func_ptr,
    );
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gmk_floc {
    pub filenm: *const ::core::ffi::c_char,
    pub lineno: ::core::ffi::c_ulong,
}
pub type gmk_func_ptr = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        ::core::ffi::c_uint,
        *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const ::core::ffi::c_char,
    pub lineno: ::core::ffi::c_ulong,
    pub offset: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const ::core::ffi::c_char,
    pub hname: *const ::core::ffi::c_char,
    pub vpath: *const ::core::ffi::c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const ::core::ffi::c_char,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
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
pub type uintmax_t = ::libc::uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set_list {
    pub next: *mut variable_set_list,
    pub set: *mut variable_set,
    pub next_is_parent: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set {
    pub table: hash_table,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut ::core::ffi::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: ::core::ffi::c_ulong,
    pub ht_capacity: ::core::ffi::c_ulong,
    pub ht_fill: ::core::ffi::c_ulong,
    pub ht_empty_slots: ::core::ffi::c_ulong,
    pub ht_collisions: ::core::ffi::c_ulong,
    pub ht_lookups: ::core::ffi::c_ulong,
    pub ht_rehashes: ::core::ffi::c_uint,
    #[bitfield(name = "ht_in_map", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub ht_in_map: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type hash_cmp_func_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type hash_func_t =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct dep {
    pub next: *mut dep,
    pub name: *const ::core::ffi::c_char,
    pub file: *mut file,
    pub shuf: *mut dep,
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn gmk_alloc(mut len: ::core::ffi::c_uint) -> *mut ::core::ffi::c_char {
    xmalloc(len as size_t) as *mut ::core::ffi::c_char
}
#[no_mangle]
pub unsafe extern "C" fn gmk_free(mut s: *mut ::core::ffi::c_char) {
    free(s as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gmk_eval(
    mut buffer: *const ::core::ffi::c_char,
    mut gfloc: *const gmk_floc,
) {
    let mut pbuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut plen: size_t = 0;
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fl: floc = floc {
        filenm: ::core::ptr::null::<::core::ffi::c_char>(),
        lineno: 0,
        offset: 0,
    };
    let mut flp: *mut floc = ::core::ptr::null_mut::<floc>();
    if !gfloc.is_null() {
        fl.filenm = (*gfloc).filenm;
        fl.lineno = (*gfloc).lineno;
        fl.offset = 0 as ::core::ffi::c_ulong;
        flp = &raw mut fl;
    } else {
        flp = ::core::ptr::null_mut::<floc>();
    }
    install_variable_buffer(&raw mut pbuf, &raw mut plen);
    s = xstrdup(buffer);
    eval_buffer(s, flp);
    free(s as *mut ::core::ffi::c_void);
    restore_variable_buffer(pbuf, plen);
}
#[no_mangle]
pub unsafe extern "C" fn gmk_expand(
    mut ref_0: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    allocated_expand_string_for_file(ref_0, ::core::ptr::null_mut::<file>())
}
#[no_mangle]
pub unsafe extern "C" fn gmk_add_function(
    mut name: *const ::core::ffi::c_char,
    mut func: gmk_func_ptr,
    mut min: ::core::ffi::c_uint,
    mut max: ::core::ffi::c_uint,
    mut flags: ::core::ffi::c_uint,
) {
    define_new_function(reading_file, name, min, max, flags, func);
}
