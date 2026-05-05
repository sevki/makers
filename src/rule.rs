use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn putchar(__c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn error(flocp: *const floc, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xcalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xrealloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn xstrdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn find_percent_cached(_: *mut *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn dir_file_exists_p(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcache_add_len(str: *const ::core::ffi::c_char, len: size_t)
        -> *const ::core::ffi::c_char;
    static mut posix_pedantic: ::core::ffi::c_int;
    static mut second_expansion: ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn print_commands(cmds: *const commands);
    fn parse_file_seq(
        stringp: *mut *mut ::core::ffi::c_char,
        size: size_t,
        stopmap: ::core::ffi::c_int,
        prefix: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn free_ns_chain(n: *mut nameseq);
    fn copy_dep_chain(d: *const dep) -> *mut dep;
    fn lookup_file(name: *const ::core::ffi::c_char) -> *mut file;
    fn expand_extra_prereqs(extra: *const variable) -> *mut dep;
    fn lookup_variable(name: *const ::core::ffi::c_char, length: size_t) -> *mut variable;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub type uintmax_t = ::libc::uintmax_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct commands {
    pub fileinfo: floc,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const ::core::ffi::c_char,
    pub lineno: ::core::ffi::c_ulong,
    pub offset: ::core::ffi::c_ulong,
}
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
    pub fileinfo: floc,
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
pub struct rule {
    pub next: *mut rule,
    pub targets: *mut *const ::core::ffi::c_char,
    pub lens: *mut ::core::ffi::c_uint,
    pub suffixes: *mut *const ::core::ffi::c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub _defn: *mut ::core::ffi::c_char,
    pub num: ::core::ffi::c_ushort,
    pub terminal: ::core::ffi::c_char,
    pub in_use: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pspec {
    pub target: *const ::core::ffi::c_char,
    pub dep: *const ::core::ffi::c_char,
    pub commands: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const ::core::ffi::c_char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAP_NUL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const INTSTR_LENGTH: usize = (53 as usize)
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22 as usize)
    .wrapping_add(3 as usize);
pub const RECIPEPREFIX_DEFAULT: ::core::ffi::c_int = '\t' as i32;
pub const PARSEFS_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]

unsafe extern "C" fn alloc_dep() -> *mut dep {
    return xcalloc(::core::mem::size_of::<dep>() as size_t) as *mut dep;
}
#[inline]

unsafe extern "C" fn free_dep_chain(mut d: *mut dep) {
    free_ns_chain(d as *mut nameseq);
}
#[no_mangle]
pub static mut pattern_rules: *mut rule = ::core::ptr::null::<rule>() as *mut rule;
#[no_mangle]
pub static mut last_pattern_rule: *mut rule = ::core::ptr::null::<rule>() as *mut rule;
#[no_mangle]
pub static mut num_pattern_rules: ::core::ffi::c_uint = 0;
#[no_mangle]
pub static mut max_pattern_targets: ::core::ffi::c_uint = 0;
#[no_mangle]
pub static mut max_pattern_deps: ::core::ffi::c_uint = 0;
#[no_mangle]
pub static mut max_pattern_dep_length: size_t = 0;
#[no_mangle]
pub static mut suffix_file: *mut file = ::core::ptr::null::<file>() as *mut file;
#[no_mangle]
pub unsafe extern "C" fn get_rule_defn(mut r: *mut rule) -> *const ::core::ffi::c_char {
    if (*r)._defn.is_null() {
        let mut len: size_t = 8 as size_t;
        let mut k: ::core::ffi::c_uint = 0;
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut sep: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
        let mut dep: *const dep = ::core::ptr::null::<dep>();
        let mut ood: *const dep = ::core::ptr::null::<dep>();
        k = 0 as ::core::ffi::c_uint;
        while k < (*r).num as ::core::ffi::c_uint {
            len = len.wrapping_add(
                (*(*r).lens.offset(k as isize)).wrapping_add(1 as ::core::ffi::c_uint) as size_t,
            );
            k = k.wrapping_add(1);
        }
        dep = (*r).deps;
        while !dep.is_null() {
            len = (len as ::core::ffi::c_ulong).wrapping_add(
                strlen(
                    (if !(*dep).name.is_null() {
                        (*dep).name
                    } else {
                        (*(*dep).file).name
                    }),
                )
                .wrapping_add(
                    (if (*dep).wait_here() as ::core::ffi::c_int != 0 {
                        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t)
                            .wrapping_sub(1 as size_t)
                    } else {
                        0 as size_t
                    }),
                )
                .wrapping_add(1 as size_t) as ::core::ffi::c_ulong,
            ) as size_t as size_t;
            dep = (*dep).next;
        }
        (*r)._defn = xmalloc(len) as *mut ::core::ffi::c_char;
        p = (*r)._defn;
        k = 0 as ::core::ffi::c_uint;
        while k < (*r).num as ::core::ffi::c_uint {
            p = mempcpy(
                mempcpy(
                    p as *mut ::core::ffi::c_void,
                    sep as *const ::core::ffi::c_void,
                    strlen(sep),
                ),
                *(*r).targets.offset(k as isize) as *const ::core::ffi::c_void,
                *(*r).lens.offset(k as isize) as size_t,
            ) as *mut ::core::ffi::c_char;
            k = k.wrapping_add(1);
            sep = b" \0" as *const u8 as *const ::core::ffi::c_char;
        }
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = ':' as i32 as ::core::ffi::c_char;
        if (*r).terminal != 0 {
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = ':' as i32 as ::core::ffi::c_char;
        }
        dep = (*r).deps;
        while !dep.is_null() {
            if (*dep).ignore_mtime() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if (*dep).wait_here() != 0 {
                    p = mempcpy(
                        p as *mut ::core::ffi::c_void,
                        b" .WAIT\0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t)
                            .wrapping_sub(1 as size_t),
                    ) as *mut ::core::ffi::c_char;
                }
                p = mempcpy(
                    mempcpy(
                        p as *mut ::core::ffi::c_void,
                        b" \0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        1 as size_t,
                    ),
                    (if !(*dep).name.is_null() {
                        (*dep).name
                    } else {
                        (*(*dep).file).name
                    }) as *const ::core::ffi::c_void,
                    strlen(if !(*dep).name.is_null() {
                        (*dep).name
                    } else {
                        (*(*dep).file).name
                    }),
                ) as *mut ::core::ffi::c_char;
            } else if ood.is_null() {
                ood = dep;
            }
            dep = (*dep).next;
        }
        sep = b" | \0" as *const u8 as *const ::core::ffi::c_char;
        while !ood.is_null() {
            if (*ood).ignore_mtime() != 0 {
                p = mempcpy(
                    p as *mut ::core::ffi::c_void,
                    sep as *const ::core::ffi::c_void,
                    strlen(sep),
                ) as *mut ::core::ffi::c_char;
                if (*ood).wait_here() != 0 {
                    p = mempcpy(
                        p as *mut ::core::ffi::c_void,
                        b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t)
                            .wrapping_sub(1 as size_t),
                    ) as *mut ::core::ffi::c_char;
                }
                p = mempcpy(
                    p as *mut ::core::ffi::c_void,
                    (if !(*ood).name.is_null() {
                        (*ood).name
                    } else {
                        (*(*ood).file).name
                    }) as *const ::core::ffi::c_void,
                    strlen(if !(*ood).name.is_null() {
                        (*ood).name
                    } else {
                        (*(*ood).file).name
                    }),
                ) as *mut ::core::ffi::c_char;
            }
            ood = (*ood).next;
            sep = b" \0" as *const u8 as *const ::core::ffi::c_char;
        }
        *p = '\0' as i32 as ::core::ffi::c_char;
    }
    return (*r)._defn;
}
#[no_mangle]
pub unsafe extern "C" fn snap_implicit_rules() {
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut namelen: size_t = 0 as size_t;
    let mut rule: *mut rule = ::core::ptr::null_mut::<rule>();
    let mut dep: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut prereqs: *mut dep = expand_extra_prereqs(lookup_variable(
        b".EXTRA_PREREQS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t).wrapping_sub(1 as size_t),
    ));
    let mut pre_deps: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    max_pattern_dep_length = 0 as size_t;
    dep = prereqs;
    while !dep.is_null() {
        let mut d: *const ::core::ffi::c_char = if !(*dep).name.is_null() {
            (*dep).name
        } else {
            (*(*dep).file).name
        };
        let mut l: size_t = strlen(d) as size_t;
        if second_expansion != 0 {
            if (*dep).name.is_null() {
                (*dep).name = xstrdup((*(*dep).file).name);
            }
            (*dep).set_need_2nd_expansion(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if (*dep).need_2nd_expansion() != 0 {
            loop {
                d = strchr(d, '%' as i32);
                if d.is_null() {
                    break;
                }
                l = l.wrapping_add(4 as size_t);
                d = d.offset(1);
            }
        }
        if l > max_pattern_dep_length {
            max_pattern_dep_length = l;
        }
        pre_deps = pre_deps.wrapping_add(1);
        dep = (*dep).next;
    }
    max_pattern_deps = 0 as ::core::ffi::c_uint;
    max_pattern_targets = max_pattern_deps;
    num_pattern_rules = max_pattern_targets;
    rule = pattern_rules;
    while !rule.is_null() {
        let mut ndeps: ::core::ffi::c_uint = pre_deps;
        let mut lastdep: *mut dep = ::core::ptr::null_mut::<dep>();
        num_pattern_rules = num_pattern_rules.wrapping_add(1);
        if (*rule).num as ::core::ffi::c_uint > max_pattern_targets {
            max_pattern_targets = (*rule).num as ::core::ffi::c_uint;
        }
        dep = (*rule).deps as *mut dep;
        while !dep.is_null() {
            let mut dname: *const ::core::ffi::c_char = if !(*dep).name.is_null() {
                (*dep).name
            } else {
                (*(*dep).file).name
            };
            let mut len: size_t = strlen(dname) as size_t;
            let mut p: *const ::core::ffi::c_char = strrchr(dname, '/' as i32);
            let mut p2: *const ::core::ffi::c_char = if !p.is_null() {
                strchr(p, '%' as i32)
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
            ndeps = ndeps.wrapping_add(1);
            if len > max_pattern_dep_length {
                max_pattern_dep_length = len;
            }
            if (*dep).next.is_null() {
                lastdep = dep;
            }
            if !p2.is_null() {
                if p == dname {
                    p = p.offset(1);
                }
                if p.offset_from(dname) as ::core::ffi::c_long as size_t > namelen {
                    namelen = p.offset_from(dname) as ::core::ffi::c_long as size_t;
                    name = xrealloc(
                        name as *mut ::core::ffi::c_void,
                        namelen.wrapping_add(1 as size_t),
                    ) as *mut ::core::ffi::c_char;
                }
                memcpy(
                    name as *mut ::core::ffi::c_void,
                    dname as *const ::core::ffi::c_void,
                    p.offset_from(dname) as ::core::ffi::c_long as size_t,
                );
                *name.offset(p.offset_from(dname) as ::core::ffi::c_long as isize) =
                    '\0' as i32 as ::core::ffi::c_char;
                (*dep).set_changed(
                    (dir_file_exists_p(name, b"\0" as *const u8 as *const ::core::ffi::c_char) == 0)
                        as ::core::ffi::c_int as ::core::ffi::c_uint
                        as ::core::ffi::c_uint,
                );
            } else {
                (*dep).set_changed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            dep = (*dep).next;
        }
        if !prereqs.is_null() {
            if !lastdep.is_null() {
                (*lastdep).next = copy_dep_chain(prereqs);
            } else {
                (*rule).deps = copy_dep_chain(prereqs) as *mut dep;
            }
        }
        if ndeps > max_pattern_deps {
            max_pattern_deps = ndeps;
        }
        rule = (*rule).next;
    }
    free(name as *mut ::core::ffi::c_void);
    free_dep_chain(prereqs);
}
unsafe extern "C" fn convert_suffix_rule(
    mut target: *const ::core::ffi::c_char,
    mut source: *const ::core::ffi::c_char,
    mut cmds: *mut commands,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut names: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut percents: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut deps: *mut dep = ::core::ptr::null_mut::<dep>();
    names = xmalloc(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t)
        as *mut *const ::core::ffi::c_char;
    percents = xmalloc(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t)
        as *mut *const ::core::ffi::c_char;
    if target.is_null() {
        *names = strcache_add_len(
            b"(%.o)\0" as *const u8 as *const ::core::ffi::c_char,
            5 as size_t,
        );
        *percents = (*names).offset(1 as ::core::ffi::c_int as isize);
    } else {
        let mut len: size_t = strlen(target) as size_t;
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (1 as size_t).wrapping_add(len).wrapping_add(1 as size_t) as usize,
        ));
        let mut p: *mut ::core::ffi::c_char =
            alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        *p.offset(0 as ::core::ffi::c_int as isize) = '%' as i32 as ::core::ffi::c_char;
        memcpy(
            p.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            target as *const ::core::ffi::c_void,
            (len as size_t).wrapping_add(1 as size_t),
        );
        *names = strcache_add_len(p, len.wrapping_add(1 as size_t));
        *percents = *names;
    }
    if source.is_null() {
        deps = ::core::ptr::null_mut::<dep>();
    } else {
        let mut len_0: size_t = strlen(source) as size_t;
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (1 as size_t).wrapping_add(len_0).wrapping_add(1 as size_t) as usize,
        ));
        let mut p_0: *mut ::core::ffi::c_char =
            alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        *p_0.offset(0 as ::core::ffi::c_int as isize) = '%' as i32 as ::core::ffi::c_char;
        memcpy(
            p_0.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            source as *const ::core::ffi::c_void,
            (len_0 as size_t).wrapping_add(1 as size_t),
        );
        deps = alloc_dep();
        (*deps).name = strcache_add_len(p_0, len_0.wrapping_add(1 as size_t));
    }
    create_pattern_rule(
        names,
        percents,
        1 as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_int,
        deps,
        cmds,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn convert_to_pattern() {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut d: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut d2: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut rulename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut maxsuffix: size_t = 0 as size_t;
    d = (*suffix_file).deps;
    while !d.is_null() {
        let mut l: size_t = strlen(if !(*d).name.is_null() {
            (*d).name
        } else {
            (*(*d).file).name
        }) as size_t;
        if l > maxsuffix {
            maxsuffix = l;
        }
        d = (*d).next;
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        maxsuffix
            .wrapping_mul(2 as size_t)
            .wrapping_add(1 as size_t) as usize,
    ));
    rulename = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    d = (*suffix_file).deps;
    while !d.is_null() {
        let mut f: *mut file = ::core::ptr::null_mut::<file>();
        let mut slen: size_t = 0;
        convert_suffix_rule(
            if !(*d).name.is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            },
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<commands>(),
        );
        if !(*(*d).file).cmds.is_null() {
            convert_suffix_rule(
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                if !(*d).name.is_null() {
                    (*d).name
                } else {
                    (*(*d).file).name
                },
                (*(*d).file).cmds,
            );
        }
        slen = strlen(if !(*d).name.is_null() {
            (*d).name
        } else {
            (*(*d).file).name
        }) as size_t;
        memcpy(
            rulename as *mut ::core::ffi::c_void,
            (if !(*d).name.is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            }) as *const ::core::ffi::c_void,
            (slen as size_t).wrapping_add(1 as size_t),
        );
        f = lookup_file(rulename);
        if !f.is_null() && !(*f).cmds.is_null() {
            if (*f).deps.is_null() {
                (*f).set_suffix(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else if posix_pedantic == 0 {
                error(
                    &raw mut (*(*f).cmds).fileinfo,
                    0 as size_t,
                    b"warning: ignoring prerequisites on suffix rule definition\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                (*f).set_suffix(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        let mut current_block_29: u64;
        d2 = (*suffix_file).deps;
        while !d2.is_null() {
            let mut s2len: size_t = 0;
            s2len = strlen(if !(*d2).name.is_null() {
                (*d2).name
            } else {
                (*(*d2).file).name
            }) as size_t;
            if !(slen == s2len
                && (*(if !(*d).name.is_null() {
                    (*d).name
                } else {
                    (*(*d).file).name
                }) as ::core::ffi::c_int
                    == *(if !(*d2).name.is_null() {
                        (*d2).name
                    } else {
                        (*(*d2).file).name
                    }) as ::core::ffi::c_int
                    && (*(if !(*d).name.is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    }) as ::core::ffi::c_int
                        == '\0' as i32
                        || strcmp(
                            (if !(*d).name.is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            })
                            .offset(1 as ::core::ffi::c_int as isize),
                            (if !(*d2).name.is_null() {
                                (*d2).name
                            } else {
                                (*(*d2).file).name
                            })
                            .offset(1 as ::core::ffi::c_int as isize),
                        ) == 0)))
            {
                memcpy(
                    rulename.offset(slen as isize) as *mut ::core::ffi::c_void,
                    (if !(*d2).name.is_null() {
                        (*d2).name
                    } else {
                        (*(*d2).file).name
                    }) as *const ::core::ffi::c_void,
                    (s2len as size_t).wrapping_add(1 as size_t),
                );
                f = lookup_file(rulename);
                if !(f.is_null() || (*f).cmds.is_null()) {
                    if !(*f).deps.is_null() {
                        if posix_pedantic != 0 {
                            current_block_29 = 11584701595673473500;
                        } else {
                            error(
                                &raw mut (*(*f).cmds).fileinfo,
                                0 as size_t,
                                b"warning: ignoring prerequisites on suffix rule definition\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block_29 = 14359455889292382949;
                        }
                    } else {
                        current_block_29 = 14359455889292382949;
                    }
                    match current_block_29 {
                        11584701595673473500 => {}
                        _ => {
                            (*f).set_suffix(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            if s2len == 2 as size_t
                                && *rulename.offset(slen as isize) as ::core::ffi::c_int
                                    == '.' as i32
                                && *rulename.offset(slen.wrapping_add(1 as size_t) as isize)
                                    as ::core::ffi::c_int
                                    == 'a' as i32
                            {
                                convert_suffix_rule(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    if !(*d).name.is_null() {
                                        (*d).name
                                    } else {
                                        (*(*d).file).name
                                    },
                                    (*f).cmds,
                                );
                            }
                            convert_suffix_rule(
                                if !(*d2).name.is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                },
                                if !(*d).name.is_null() {
                                    (*d).name
                                } else {
                                    (*(*d).file).name
                                },
                                (*f).cmds,
                            );
                        }
                    }
                }
            }
            d2 = (*d2).next;
        }
        d = (*d).next;
    }
}
unsafe extern "C" fn new_pattern_rule(
    mut rule: *mut rule,
    mut override_0: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut r: *mut rule = ::core::ptr::null_mut::<rule>();
    let mut lastrule: *mut rule = ::core::ptr::null_mut::<rule>();
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint = 0;
    (*rule).in_use = 0 as ::core::ffi::c_char;
    (*rule).terminal = 0 as ::core::ffi::c_char;
    (*rule).next = ::core::ptr::null_mut::<rule>();
    lastrule = ::core::ptr::null_mut::<rule>();
    r = pattern_rules;
    's_18: while !r.is_null() {
        i = 0 as ::core::ffi::c_uint;
        while i < (*rule).num as ::core::ffi::c_uint {
            j = 0 as ::core::ffi::c_uint;
            while j < (*r).num as ::core::ffi::c_uint {
                if !(**(*rule).targets.offset(i as isize) as ::core::ffi::c_int
                    == **(*r).targets.offset(j as isize) as ::core::ffi::c_int
                    && (**(*rule).targets.offset(i as isize) as ::core::ffi::c_int == '\0' as i32
                        || strcmp(
                            (*(*rule).targets.offset(i as isize))
                                .offset(1 as ::core::ffi::c_int as isize),
                            (*(*r).targets.offset(j as isize))
                                .offset(1 as ::core::ffi::c_int as isize),
                        ) == 0))
                {
                    break;
                }
                j = j.wrapping_add(1);
            }
            if j == (*r).num as ::core::ffi::c_uint {
                let mut d: *mut dep = ::core::ptr::null_mut::<dep>();
                let mut d2: *mut dep = ::core::ptr::null_mut::<dep>();
                d = (*rule).deps as *mut dep;
                d2 = (*r).deps as *mut dep;
                while !d.is_null() && !d2.is_null() {
                    if !(*(if !(*d).name.is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    }) as ::core::ffi::c_int
                        == *(if !(*d2).name.is_null() {
                            (*d2).name
                        } else {
                            (*(*d2).file).name
                        }) as ::core::ffi::c_int
                        && (*(if !(*d).name.is_null() {
                            (*d).name
                        } else {
                            (*(*d).file).name
                        }) as ::core::ffi::c_int
                            == '\0' as i32
                            || strcmp(
                                (if !(*d).name.is_null() {
                                    (*d).name
                                } else {
                                    (*(*d).file).name
                                })
                                .offset(1 as ::core::ffi::c_int as isize),
                                (if !(*d2).name.is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                })
                                .offset(1 as ::core::ffi::c_int as isize),
                            ) == 0))
                    {
                        break;
                    }
                    d = (*d).next;
                    d2 = (*d2).next;
                }
                if d.is_null() && d2.is_null() {
                    if override_0 != 0 {
                        freerule(r, lastrule);
                        if pattern_rules.is_null() {
                            pattern_rules = rule;
                        } else {
                            (*last_pattern_rule).next = rule;
                        }
                        last_pattern_rule = rule;
                        break 's_18;
                    } else {
                        freerule(rule, ::core::ptr::null_mut::<rule>());
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        lastrule = r;
        r = (*r).next;
    }
    if r.is_null() {
        if pattern_rules.is_null() {
            pattern_rules = rule;
        } else {
            (*last_pattern_rule).next = rule;
        }
        last_pattern_rule = rule;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn install_pattern_rule(
    mut p: *const pspec,
    mut terminal: ::core::ffi::c_int,
) {
    let mut r: *mut rule = ::core::ptr::null_mut::<rule>();
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    r = xmalloc(::core::mem::size_of::<rule>() as size_t) as *mut rule;
    (*r).num = 1 as ::core::ffi::c_ushort;
    (*r).targets = xmalloc(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t)
        as *mut *const ::core::ffi::c_char;
    (*r).suffixes = xmalloc(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t)
        as *mut *const ::core::ffi::c_char;
    (*r).lens = xmalloc(::core::mem::size_of::<::core::ffi::c_uint>() as size_t)
        as *mut ::core::ffi::c_uint;
    (*r)._defn = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *(*r).lens.offset(0 as ::core::ffi::c_int as isize) =
        strlen((*p).target) as ::core::ffi::c_uint;
    let ref mut fresh1 = *(*r).targets.offset(0 as ::core::ffi::c_int as isize);
    *fresh1 = (*p).target;
    let ref mut fresh2 = *(*r).suffixes.offset(0 as ::core::ffi::c_int as isize);
    *fresh2 =
        find_percent_cached((*r).targets.offset(0 as ::core::ffi::c_int as isize)
            as *mut *const ::core::ffi::c_char);
    '_c2rust_label: {
        if !(*(*r).suffixes.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        } else {
            __assert_fail(
                b"r->suffixes[0] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/rule.c\0" as *const u8 as *const ::core::ffi::c_char,
                492 as ::core::ffi::c_uint,
                b"void install_pattern_rule(const struct pspec *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let ref mut fresh3 = *(*r).suffixes.offset(0 as ::core::ffi::c_int as isize);
    *fresh3 = (*fresh3).offset(1);
    ptr = (*p).dep;
    (*r).deps = parse_file_seq(
        &raw mut ptr as *mut *mut ::core::ffi::c_char,
        ::core::mem::size_of::<dep>() as size_t,
        MAP_NUL,
        ::core::ptr::null::<::core::ffi::c_char>(),
        PARSEFS_NONE,
    ) as *mut dep as *mut dep;
    if new_pattern_rule(r, 0 as ::core::ffi::c_int) != 0 {
        (*r).terminal = (if terminal != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
        (*r).cmds = xmalloc(::core::mem::size_of::<commands>() as size_t) as *mut commands;
        (*(*r).cmds).fileinfo.filenm = ::core::ptr::null::<::core::ffi::c_char>();
        (*(*r).cmds).fileinfo.lineno = 0 as ::core::ffi::c_ulong;
        (*(*r).cmds).fileinfo.offset = 0 as ::core::ffi::c_ulong;
        (*(*r).cmds).commands = xstrdup((*p).commands);
        (*(*r).cmds).command_lines = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        (*(*r).cmds).recipe_prefix = RECIPEPREFIX_DEFAULT as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn freerule(mut rule: *mut rule, mut lastrule: *mut rule) {
    let mut next: *mut rule = (*rule).next;
    free_dep_chain((*rule).deps as *mut dep);
    free((*rule).targets as *mut ::core::ffi::c_void);
    free((*rule).suffixes as *mut ::core::ffi::c_void);
    free((*rule).lens as *mut ::core::ffi::c_void);
    free((*rule)._defn as *mut ::core::ffi::c_void);
    free(rule as *mut ::core::ffi::c_void);
    if pattern_rules == rule {
        if !lastrule.is_null() {
            abort();
        } else {
            pattern_rules = next;
        }
    } else if !lastrule.is_null() {
        (*lastrule).next = next;
    }
    if last_pattern_rule == rule {
        last_pattern_rule = lastrule;
    }
}
#[no_mangle]
pub unsafe extern "C" fn create_pattern_rule(
    mut targets: *mut *const ::core::ffi::c_char,
    mut target_percents: *mut *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_ushort,
    mut terminal: ::core::ffi::c_int,
    mut deps: *mut dep,
    mut commands: *mut commands,
    mut override_0: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_uint = 0;
    let mut r: *mut rule = xmalloc(::core::mem::size_of::<rule>() as size_t) as *mut rule;
    (*r).num = n;
    (*r).cmds = commands as *mut commands;
    (*r).deps = deps as *mut dep;
    (*r).targets = targets;
    (*r).suffixes = target_percents;
    (*r).lens = xmalloc(
        (n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    (*r)._defn = ::core::ptr::null_mut::<::core::ffi::c_char>();
    i = 0 as ::core::ffi::c_uint;
    while i < n as ::core::ffi::c_uint {
        *(*r).lens.offset(i as isize) = strlen(*targets.offset(i as isize)) as ::core::ffi::c_uint;
        '_c2rust_label: {
            if !(*(*r).suffixes.offset(i as isize)).is_null() {
            } else {
                __assert_fail(
                    b"r->suffixes[i] != NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"src/rule.c\0" as *const u8 as *const ::core::ffi::c_char,
                    584 as ::core::ffi::c_uint,
                    b"void create_pattern_rule(const char **, const char **, unsigned short, int, struct dep *, struct commands *, int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        let ref mut fresh0 = *(*r).suffixes.offset(i as isize);
        *fresh0 = (*fresh0).offset(1);
        i = i.wrapping_add(1);
    }
    if new_pattern_rule(r, override_0) != 0 {
        (*r).terminal = (if terminal != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_rule(mut r: *mut rule) {
    fputs(get_rule_defn(r), stdout);
    putchar('\n' as i32);
    if !(*r).cmds.is_null() {
        print_commands((*r).cmds);
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_rule_data_base() {
    let mut rules: ::core::ffi::c_uint = 0;
    let mut terminal: ::core::ffi::c_uint = 0;
    let mut r: *mut rule = ::core::ptr::null_mut::<rule>();
    puts(b"\n# Implicit Rules\0" as *const u8 as *const ::core::ffi::c_char);
    terminal = 0 as ::core::ffi::c_uint;
    rules = terminal;
    r = pattern_rules;
    while !r.is_null() {
        rules = rules.wrapping_add(1);
        putchar('\n' as i32);
        print_rule(r);
        if (*r).terminal != 0 {
            terminal = terminal.wrapping_add(1);
        }
        r = (*r).next;
    }
    if rules == 0 as ::core::ffi::c_uint {
        puts(b"\n# No implicit rules.\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        printf(
            b"\n# %u implicit rules, %u (%.1f%%) terminal.\0" as *const u8
                as *const ::core::ffi::c_char,
            rules,
            terminal,
            terminal as ::core::ffi::c_double / rules as ::core::ffi::c_double * 100.0f64,
        );
    }
    if num_pattern_rules != rules {
        if num_pattern_rules != 0 as ::core::ffi::c_uint {
            fatal(
                ::core::ptr::null_mut::<floc>(),
                INTSTR_LENGTH.wrapping_mul(2 as size_t),
                b"INTERNAL: num_pattern_rules is wrong!  %u != %u\0" as *const u8
                    as *const ::core::ffi::c_char,
                num_pattern_rules,
                rules,
            );
        }
    }
}
