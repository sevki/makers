use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type dep;
    static mut environ: *mut *mut ::core::ffi::c_char;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn fatal(flocp: *const floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xrealloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn xstrdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn xstrndup(_: *const ::core::ffi::c_char, _: size_t) -> *mut ::core::ffi::c_char;
    fn lindex(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn find_percent(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut reading_file: *const floc;
    static mut stopchar_map: [::core::ffi::c_ushort; 0];
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut db_level: ::core::ffi::c_int;
    static mut env_recursion: ::core::ffi::c_ulonglong;
    static mut current_variable_set_list: *mut variable_set_list;
    fn handle_function(
        op: *mut *mut ::core::ffi::c_char,
        stringp: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn patsubst_expand_pat(
        o: *mut ::core::ffi::c_char,
        text: *const ::core::ffi::c_char,
        pattern: *const ::core::ffi::c_char,
        replace: *const ::core::ffi::c_char,
        pattern_percent: *const ::core::ffi::c_char,
        replace_percent: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn install_file_context(
        file: *mut file,
        oldlist: *mut *mut variable_set_list,
        oldfloc: *mut *const floc,
    );
    fn restore_file_context(oldlist: *mut variable_set_list, oldfloc: *const floc);
    fn lookup_variable(name: *const ::core::ffi::c_char, length: size_t) -> *mut variable;
    fn lookup_variable_in_set(
        name: *const ::core::ffi::c_char,
        length: size_t,
        set: *const variable_set,
    ) -> *mut variable;
    fn warn_undefined(name: *const ::core::ffi::c_char, length: size_t);
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
pub const us_failed: update_status = 3;
pub const us_question: update_status = 2;
pub const us_none: update_status = 1;
pub const us_success: update_status = 0;
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
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut expanding_var: *mut *const floc =
    unsafe { &raw const reading_file as *mut *const floc };
pub const VARIABLE_BUFFER_ZONE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
static mut variable_buffer_length: size_t = 0;
#[no_mangle]
pub static mut variable_buffer: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn variable_buffer_output(
    mut ptr: *mut ::core::ffi::c_char,
    mut string: *const ::core::ffi::c_char,
    mut length: size_t,
) -> *mut ::core::ffi::c_char {
    let mut newlen: size_t =
        length.wrapping_add(ptr.offset_from(variable_buffer) as ::core::ffi::c_long as size_t);
    '_c2rust_label: {
        if ptr >= variable_buffer {
        } else {
            __assert_fail(
                b"ptr >= variable_buffer\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/expand.c\0" as *const u8 as *const ::core::ffi::c_char,
                61 as ::core::ffi::c_uint,
                b"char *variable_buffer_output(char *, const char *, size_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if ptr < variable_buffer.offset(variable_buffer_length as isize) {
        } else {
            __assert_fail(
                b"ptr < variable_buffer + variable_buffer_length\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"src/expand.c\0" as *const u8 as *const ::core::ffi::c_char,
                62 as ::core::ffi::c_uint,
                b"char *variable_buffer_output(char *, const char *, size_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if newlen
        .wrapping_add(VARIABLE_BUFFER_ZONE as size_t)
        .wrapping_add(1 as size_t)
        > variable_buffer_length
    {
        let mut offset: size_t = ptr.offset_from(variable_buffer) as ::core::ffi::c_long as size_t;
        variable_buffer_length = if newlen.wrapping_add(100 as size_t)
            > (2 as size_t).wrapping_mul(variable_buffer_length)
        {
            newlen.wrapping_add(100 as size_t)
        } else {
            (2 as size_t).wrapping_mul(variable_buffer_length)
        };
        variable_buffer = xrealloc(
            variable_buffer as *mut ::core::ffi::c_void,
            variable_buffer_length.wrapping_add(1 as size_t),
        ) as *mut ::core::ffi::c_char;
        ptr = variable_buffer.offset(offset as isize);
    }
    ptr = mempcpy(
        ptr as *mut ::core::ffi::c_void,
        string as *const ::core::ffi::c_void,
        length as size_t,
    ) as *mut ::core::ffi::c_char;
    *ptr = '\0' as i32 as ::core::ffi::c_char;
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn initialize_variable_output() -> *mut ::core::ffi::c_char {
    if variable_buffer.is_null() {
        variable_buffer_length = 200 as size_t;
        variable_buffer = xmalloc(variable_buffer_length) as *mut ::core::ffi::c_char;
    }
    *variable_buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    return variable_buffer;
}
#[no_mangle]
pub unsafe extern "C" fn install_variable_buffer(
    mut bufp: *mut *mut ::core::ffi::c_char,
    mut lenp: *mut size_t,
) {
    *bufp = variable_buffer;
    *lenp = variable_buffer_length;
    variable_buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    initialize_variable_output();
}
#[no_mangle]
pub unsafe extern "C" fn restore_variable_buffer(
    mut buf: *mut ::core::ffi::c_char,
    mut len: size_t,
) {
    free(variable_buffer as *mut ::core::ffi::c_void);
    variable_buffer = buf;
    variable_buffer_length = len;
}
#[no_mangle]
pub unsafe extern "C" fn swap_variable_buffer(
    mut buf: *mut ::core::ffi::c_char,
    mut len: size_t,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = variable_buffer;
    variable_buffer = buf;
    variable_buffer_length = len;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn recursively_expand_for_file(
    mut v: *mut variable,
    mut file: *mut file,
) -> *mut ::core::ffi::c_char {
    let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut this_var: *const floc = ::core::ptr::null::<floc>();
    let mut saved_varp: *mut *const floc = ::core::ptr::null_mut::<*const floc>();
    let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut set_reading: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nl: size_t = strlen((*v).name) as size_t;
    let mut parent: *mut variable = ::core::ptr::null_mut::<variable>();
    if (*v).expanding() as ::core::ffi::c_int != 0 && env_recursion != 0 {
        let mut ep: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        if 0x2 as ::core::ffi::c_int & db_level != 0 {
            printf(
                b"%s:%lu: not recursively expanding %s to export to shell function\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*v).fileinfo.filenm,
                (*v).fileinfo.lineno,
                (*v).name,
            );
            fflush(stdout);
        }
        ep = environ;
        while !(*ep).is_null() {
            if strncmp(*ep, (*v).name, nl as size_t) == 0 as ::core::ffi::c_int
                && *(*ep).offset(nl as isize) as ::core::ffi::c_int == '=' as i32
            {
                return xstrdup(
                    (*ep)
                        .offset(nl as isize)
                        .offset(1 as ::core::ffi::c_int as isize),
                );
            }
            ep = ep.offset(1);
        }
        return xstrdup(b"\0" as *const u8 as *const ::core::ffi::c_char);
    }
    saved_varp = expanding_var;
    if !(*v).fileinfo.filenm.is_null() {
        this_var = &raw mut (*v).fileinfo;
        expanding_var = &raw mut this_var;
    }
    if reading_file.is_null() {
        set_reading = 1 as ::core::ffi::c_int;
        reading_file = &raw mut (*v).fileinfo;
    }
    if (*v).expanding() != 0 {
        if (*v).exp_count() == 0 {
            fatal(
                *expanding_var,
                strlen((*v).name) as size_t,
                b"recursive variable '%s' references itself (eventually)\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*v).name,
            );
        }
        (*v).set_exp_count((*v).exp_count() - 1 as ::core::ffi::c_uint);
    }
    if !file.is_null() {
        install_file_context(file, &raw mut savev, ::core::ptr::null_mut::<*const floc>());
    }
    (*v).set_expanding(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if (*v).append() != 0 {
        let mut sl: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
        sl = current_variable_set_list;
        while !sl.is_null() && parent.is_null() {
            let mut vp: *mut variable = lookup_variable_in_set((*v).name, nl, (*sl).set);
            if !vp.is_null()
                && vp != v
                && (*vp).origin() as ::core::ffi::c_int == o_override as ::core::ffi::c_int
            {
                parent = vp;
            }
            sl = (*sl).next;
        }
    }
    if !parent.is_null() {
        value = if (*v).origin() as ::core::ffi::c_int == o_override as ::core::ffi::c_int {
            allocated_variable_append(v)
        } else {
            xstrdup((*parent).value)
        };
    } else if (*v).origin() as ::core::ffi::c_int == o_command as ::core::ffi::c_int
        || (*v).origin() as ::core::ffi::c_int == o_env_override as ::core::ffi::c_int
    {
        value = allocated_expand_string_for_file((*v).value, ::core::ptr::null_mut::<file>());
    } else if (*v).append() != 0 {
        value = allocated_variable_append(v);
    } else {
        value = allocated_expand_string_for_file((*v).value, ::core::ptr::null_mut::<file>());
    }
    (*v).set_expanding(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if set_reading != 0 {
        reading_file = ::core::ptr::null::<floc>();
    }
    if !file.is_null() {
        restore_file_context(savev, ::core::ptr::null::<floc>());
    }
    expanding_var = saved_varp;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn expand_variable_output(
    mut ptr: *mut ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
) -> *mut ::core::ffi::c_char {
    let mut v: *mut variable = ::core::ptr::null_mut::<variable>();
    let mut recursive: ::core::ffi::c_uint = 0;
    let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    v = lookup_variable(name, length);
    if v.is_null() {
        warn_undefined(name, length);
    }
    if v.is_null()
        || *(*v).value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
            && (*v).append() == 0
    {
        return ptr;
    }
    recursive = (*v).recursive();
    value = if recursive != 0 {
        recursively_expand_for_file(v, ::core::ptr::null_mut::<file>())
    } else {
        (*v).value
    };
    ptr = variable_buffer_output(ptr, value, strlen(value) as size_t);
    if recursive != 0 {
        free(value as *mut ::core::ffi::c_void);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn expand_variable_buf(
    mut buf: *mut ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
) -> *mut ::core::ffi::c_char {
    let mut offs: size_t = 0;
    if buf.is_null() {
        buf = initialize_variable_output();
    }
    '_c2rust_label: {
        if buf >= variable_buffer {
        } else {
            __assert_fail(
                b"buf >= variable_buffer\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/expand.c\0" as *const u8 as *const ::core::ffi::c_char,
                315 as ::core::ffi::c_uint,
                b"char *expand_variable_buf(char *, const char *, size_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if buf < variable_buffer.offset(variable_buffer_length as isize) {
        } else {
            __assert_fail(
                b"buf < variable_buffer + variable_buffer_length\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"src/expand.c\0" as *const u8 as *const ::core::ffi::c_char,
                316 as ::core::ffi::c_uint,
                b"char *expand_variable_buf(char *, const char *, size_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    offs = buf.offset_from(variable_buffer) as ::core::ffi::c_long as size_t;
    expand_variable_output(buf, name, length);
    return variable_buffer.offset(offs as isize);
}
#[no_mangle]
pub unsafe extern "C" fn allocated_expand_variable(
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
) -> *mut ::core::ffi::c_char {
    let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut olen: size_t = 0;
    install_variable_buffer(&raw mut obuf, &raw mut olen);
    expand_variable_output(variable_buffer, name, length);
    return swap_variable_buffer(obuf, olen);
}
#[no_mangle]
pub unsafe extern "C" fn allocated_expand_variable_for_file(
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
    mut file: *mut file,
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut savef: *const floc = ::core::ptr::null::<floc>();
    if file.is_null() {
        return allocated_expand_variable(name, length);
    }
    install_file_context(file, &raw mut savev, &raw mut savef);
    result = allocated_expand_variable(name, length);
    restore_file_context(savev, savef);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn expand_string_buf(
    mut buf: *mut ::core::ffi::c_char,
    mut string: *const ::core::ffi::c_char,
    mut length: size_t,
) -> *mut ::core::ffi::c_char {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut v: *mut variable = ::core::ptr::null_mut::<variable>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p1: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut save: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut o: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut line_offset: size_t = 0;
    if buf.is_null() {
        buf = initialize_variable_output();
    }
    o = buf;
    line_offset = buf.offset_from(variable_buffer) as ::core::ffi::c_long as size_t;
    if length == 0 as size_t {
        return variable_buffer;
    }
    save = if length == SIZE_MAX as size_t {
        xstrdup(string)
    } else {
        xstrndup(string, length)
    };
    p = save;
    loop {
        p1 = strchr(p, '$' as i32);
        o = variable_buffer_output(
            o,
            p,
            if !p1.is_null() {
                p1.offset_from(p) as ::core::ffi::c_long as size_t
            } else {
                (strlen(p) as size_t).wrapping_add(1 as size_t)
            },
        );
        if p1.is_null() {
            break;
        }
        p = p1.offset(1 as ::core::ffi::c_int as isize);
        match *p as ::core::ffi::c_int {
            36 | 0 => {
                o = variable_buffer_output(o, p1, 1 as size_t);
            }
            40 | 123 => {
                let mut openparen: ::core::ffi::c_char = *p;
                let mut closeparen: ::core::ffi::c_char =
                    (if openparen as ::core::ffi::c_int == '(' as i32 {
                        ')' as i32
                    } else {
                        '}' as i32
                    }) as ::core::ffi::c_char;
                let mut beg: *const ::core::ffi::c_char =
                    p.offset(1 as ::core::ffi::c_int as isize);
                let mut abeg: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut end: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                let mut colon: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                if !(handle_function(&raw mut o, &raw mut p) != 0) {
                    end = strchr(beg, closeparen as ::core::ffi::c_int);
                    if end.is_null() {
                        fatal(
                            *expanding_var,
                            0 as size_t,
                            b"unterminated variable reference\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    p1 = lindex(beg, end, '$' as i32);
                    if !p1.is_null() {
                        let mut count: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        p = beg;
                        while *p as ::core::ffi::c_int != '\0' as i32 {
                            if *p as ::core::ffi::c_int == openparen as ::core::ffi::c_int {
                                count += 1;
                            } else if *p as ::core::ffi::c_int == closeparen as ::core::ffi::c_int
                                && {
                                    count -= 1;
                                    count == 0 as ::core::ffi::c_int
                                }
                            {
                                break;
                            }
                            p = p.offset(1);
                        }
                        if count == 0 as ::core::ffi::c_int {
                            abeg = expand_argument(beg, p);
                            beg = abeg;
                            end = strchr(beg, '\0' as i32);
                        }
                    } else {
                        p = end;
                    }
                    colon = lindex(beg, end, ':' as i32);
                    if !colon.is_null() {
                        let mut subst_beg: *const ::core::ffi::c_char =
                            colon.offset(1 as ::core::ffi::c_int as isize);
                        let mut subst_end: *const ::core::ffi::c_char =
                            lindex(subst_beg, end, '=' as i32);
                        if subst_end.is_null() {
                            colon = ::core::ptr::null::<::core::ffi::c_char>();
                        } else {
                            let mut replace_beg: *const ::core::ffi::c_char =
                                subst_end.offset(1 as ::core::ffi::c_int as isize);
                            let mut replace_end: *const ::core::ffi::c_char = end;
                            v = lookup_variable(
                                beg,
                                colon.offset_from(beg) as ::core::ffi::c_long as size_t,
                            );
                            if v.is_null() {
                                warn_undefined(
                                    beg,
                                    colon.offset_from(beg) as ::core::ffi::c_long as size_t,
                                );
                            }
                            if !v.is_null() && *(*v).value as ::core::ffi::c_int != '\0' as i32 {
                                let mut pattern: *mut ::core::ffi::c_char =
                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                let mut replace: *mut ::core::ffi::c_char =
                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                let mut ppercent: *mut ::core::ffi::c_char =
                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                let mut rpercent: *mut ::core::ffi::c_char =
                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                let mut value: *mut ::core::ffi::c_char = if (*v).recursive()
                                    as ::core::ffi::c_int
                                    != 0
                                {
                                    recursively_expand_for_file(v, ::core::ptr::null_mut::<file>())
                                } else {
                                    (*v).value
                                };
                                alloca_allocations.push(::std::vec::from_elem(
                                    0,
                                    (subst_end.offset_from(subst_beg) as ::core::ffi::c_long
                                        + 2 as ::core::ffi::c_long)
                                        as ::core::ffi::c_ulong
                                        as usize,
                                ));
                                pattern = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                    as *mut ::core::ffi::c_char;
                                let fresh0 = pattern;
                                pattern = pattern.offset(1);
                                *fresh0 = '%' as i32 as ::core::ffi::c_char;
                                memcpy(
                                    pattern as *mut ::core::ffi::c_void,
                                    subst_beg as *const ::core::ffi::c_void,
                                    subst_end.offset_from(subst_beg) as ::core::ffi::c_long
                                        as size_t,
                                );
                                *pattern.offset(subst_end.offset_from(subst_beg)
                                    as ::core::ffi::c_long
                                    as isize) = '\0' as i32 as ::core::ffi::c_char;
                                alloca_allocations.push(::std::vec::from_elem(
                                    0,
                                    (replace_end.offset_from(replace_beg) as ::core::ffi::c_long
                                        + 2 as ::core::ffi::c_long)
                                        as ::core::ffi::c_ulong
                                        as usize,
                                ));
                                replace = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                    as *mut ::core::ffi::c_char;
                                let fresh1 = replace;
                                replace = replace.offset(1);
                                *fresh1 = '%' as i32 as ::core::ffi::c_char;
                                memcpy(
                                    replace as *mut ::core::ffi::c_void,
                                    replace_beg as *const ::core::ffi::c_void,
                                    replace_end.offset_from(replace_beg) as ::core::ffi::c_long
                                        as size_t,
                                );
                                *replace.offset(replace_end.offset_from(replace_beg)
                                    as ::core::ffi::c_long
                                    as isize) = '\0' as i32 as ::core::ffi::c_char;
                                ppercent = find_percent(pattern);
                                if !ppercent.is_null() {
                                    ppercent = ppercent.offset(1);
                                    rpercent = find_percent(replace);
                                    if !rpercent.is_null() {
                                        rpercent = rpercent.offset(1);
                                    }
                                } else {
                                    ppercent = pattern;
                                    rpercent = replace;
                                    pattern = pattern.offset(-1);
                                    replace = replace.offset(-1);
                                }
                                o = patsubst_expand_pat(
                                    o, value, pattern, replace, ppercent, rpercent,
                                );
                                if (*v).recursive() != 0 {
                                    free(value as *mut ::core::ffi::c_void);
                                }
                            }
                        }
                    }
                    if colon.is_null() {
                        o = expand_variable_output(
                            o,
                            beg,
                            end.offset_from(beg) as ::core::ffi::c_long as size_t,
                        );
                    }
                    free(abeg as *mut ::core::ffi::c_void);
                }
            }
            _ => {
                if !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort).offset(
                    *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar as isize,
                ) as ::core::ffi::c_int
                    & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                    != 0 as ::core::ffi::c_int)
                {
                    o = expand_variable_output(o, p, 1 as size_t);
                }
            }
        }
        if *p as ::core::ffi::c_int == '\0' as i32 {
            break;
        }
        p = p.offset(1);
    }
    free(save as *mut ::core::ffi::c_void);
    return variable_buffer.offset(line_offset as isize);
}
#[no_mangle]
pub unsafe extern "C" fn expand_argument(
    mut str: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut alloc: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if str == end {
        return xstrdup(b"\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if end.is_null() || *end as ::core::ffi::c_int == '\0' as i32 {
        return allocated_expand_string_for_file(str, ::core::ptr::null_mut::<file>());
    }
    if end.offset_from(str) as ::core::ffi::c_long + 1 as ::core::ffi::c_long
        > 1000 as ::core::ffi::c_long
    {
        alloc = xmalloc(
            (end.offset_from(str) as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as size_t,
        ) as *mut ::core::ffi::c_char;
        tmp = alloc;
    } else {
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (end.offset_from(str) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as ::core::ffi::c_ulong as usize,
        ));
        tmp = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    }
    memcpy(
        tmp as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        end.offset_from(str) as ::core::ffi::c_long as size_t,
    );
    *tmp.offset(end.offset_from(str) as ::core::ffi::c_long as isize) =
        '\0' as i32 as ::core::ffi::c_char;
    r = allocated_expand_string_for_file(tmp, ::core::ptr::null_mut::<file>());
    free(alloc as *mut ::core::ffi::c_void);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn expand_string_for_file(
    mut string: *const ::core::ffi::c_char,
    mut file: *mut file,
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut savef: *const floc = ::core::ptr::null::<floc>();
    if file.is_null() {
        return expand_string_buf(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            string,
            SIZE_MAX as size_t,
        );
    }
    install_file_context(file, &raw mut savev, &raw mut savef);
    result = expand_string_buf(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        string,
        SIZE_MAX as size_t,
    );
    restore_file_context(savev, savef);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn allocated_expand_string_for_file(
    mut string: *const ::core::ffi::c_char,
    mut file: *mut file,
) -> *mut ::core::ffi::c_char {
    let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut olen: size_t = 0;
    install_variable_buffer(&raw mut obuf, &raw mut olen);
    expand_string_for_file(string, file);
    return swap_variable_buffer(obuf, olen);
}
unsafe extern "C" fn variable_append(
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
    mut set: *const variable_set_list,
    mut local: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut v: *const variable = ::core::ptr::null::<variable>();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nextlocal: ::core::ffi::c_int = 0;
    if set.is_null() {
        return initialize_variable_output();
    }
    nextlocal =
        (local != 0 && (*set).next_is_parent == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    v = lookup_variable_in_set(name, length, (*set).set);
    if v.is_null() || local == 0 && (*v).private_var() as ::core::ffi::c_int != 0 {
        return variable_append(name, length, (*set).next, nextlocal);
    }
    if (*v).append() != 0 {
        buf = variable_append(name, length, (*set).next, nextlocal);
    } else {
        buf = initialize_variable_output();
    }
    if buf > variable_buffer {
        buf = variable_buffer_output(
            buf,
            b" \0" as *const u8 as *const ::core::ffi::c_char,
            1 as size_t,
        );
    }
    if (*v).recursive() == 0 {
        return variable_buffer_output(buf, (*v).value, strlen((*v).value) as size_t);
    }
    buf = expand_string_buf(buf, (*v).value, strlen((*v).value) as size_t);
    return buf.offset(strlen(buf) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn allocated_variable_append(mut v: *const variable) -> *mut ::core::ffi::c_char {
    let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut olen: size_t = 0;
    install_variable_buffer(&raw mut obuf, &raw mut olen);
    variable_append(
        (*v).name,
        strlen((*v).name) as size_t,
        current_variable_set_list,
        1 as ::core::ffi::c_int,
    );
    return swap_variable_buffer(obuf, olen);
}
