use libc::{abort, free, printf, putchar, puts, sprintf, strchr, strcmp, strcpy, strstr};
use ::c2rust_bitfields;
use crate::stdio::{FILE};
use crate::file::{Commands, Dep, File, VariableSet, VariableSetList};
extern "C" {
    static mut stdout: *mut FILE;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn concat(_: ::core::ffi::c_uint, ...) -> *const ::core::ffi::c_char;
    fn error(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn fatal(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn format(
        prefix: *const ::core::ffi::c_char,
        length: size_t,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn reset_makeflags(origin: variable_origin);
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xcalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xrealloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn xstrdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn xstrndup(_: *const ::core::ffi::c_char, _: size_t) -> *mut ::core::ffi::c_char;
    fn next_token(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn skip_reference(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut reading_file: *const Floc;
    static mut expanding_var: *mut *const Floc;
    static mut stopchar_map: [::core::ffi::c_ushort; 0];
    static mut env_overrides: ::core::ffi::c_int;
    static mut export_all_variables: ::core::ffi::c_int;
    static mut default_shell: *const ::core::ffi::c_char;
    static mut cmd_prefix: ::core::ffi::c_char;
    static mut jobserver_auth: *mut ::core::ffi::c_char;
    static mut makelevel: ::core::ffi::c_uint;
    static mut remote_description: *mut ::core::ffi::c_char;
    static mut make_host: *mut ::core::ffi::c_char;
    static mut version_string: *mut ::core::ffi::c_char;
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
    fn hash_delete_at(
        ht: *mut hash_table,
        slot: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn hash_free(ht: *mut hash_table, free_items: ::core::ffi::c_int);
    fn hash_map(ht: *mut hash_table, map: hash_map_func_t);
    fn hash_map_arg(ht: *mut hash_table, map: hash_map_arg_func_t, arg: *mut ::core::ffi::c_void);
    fn hash_print_stats(ht: *mut hash_table, out_FILE: *mut FILE);
    fn jhash(key: *const ::core::ffi::c_uchar, n: ::core::ffi::c_int) -> ::core::ffi::c_uint;
    static mut hash_deleted_item: *const ::core::ffi::c_void;
    static mut variable_buffer: *mut ::core::ffi::c_char;
    static mut shell_var: variable;
    fn install_variable_buffer(bufp: *mut *mut ::core::ffi::c_char, lenp: *mut size_t);
    fn swap_variable_buffer(buf: *mut ::core::ffi::c_char, len: size_t)
        -> *mut ::core::ffi::c_char;
    fn allocated_expand_string_for_file(
        line: *const ::core::ffi::c_char,
        file: *mut file,
    ) -> *mut ::core::ffi::c_char;
    fn recursively_expand_for_file(v: *mut variable, file: *mut file) -> *mut ::core::ffi::c_char;
    fn allocated_expand_variable(
        name: *const ::core::ffi::c_char,
        length: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn func_shell_base(
        o: *mut ::core::ffi::c_char,
        argv: *mut *mut ::core::ffi::c_char,
        trim_newlines: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn jobserver_get_invalid_auth() -> *const ::core::ffi::c_char;
    static mut warnings: [warning_action; 4];
    fn decode_warn_actions(value: *const ::core::ffi::c_char, flocp: *const Floc);
}
pub type size_t = usize;
pub type uintmax_t = ::libc::uintmax_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern_var {
    pub next: *mut pattern_var,
    pub suffix: *const ::core::ffi::c_char,
    pub target: *const ::core::ffi::c_char,
    pub len: size_t,
    pub variable: variable,
}
pub type hash_map_arg_func_t = crate::hash::hash_map_arg_func_t;
pub type hash_map_func_t = crate::hash::hash_map_func_t;
pub type variable_scope = ::core::ffi::c_uint;
pub const s_pattern: variable_scope = 2;
pub const s_target: variable_scope = 1;
pub const s_global: variable_scope = 0;
pub const w_error: warning_action = 3;
pub type warning_action = ::core::ffi::c_uint;
pub const w_warn: warning_action = 2;
pub const w_ignore: warning_action = 1;
pub const w_unset: warning_action = 0;
pub const wt_invalid_var: warning_type = 2;
pub const wt_invalid_ref: warning_type = 1;
pub const wt_undefined_var: warning_type = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct defined_vars {
    pub name: *const ::core::ffi::c_char,
    pub len: size_t,
}
pub type warning_type = ::core::ffi::c_uint;
pub const wt_max: warning_type = 4;
pub const wt_circular_dep: warning_type = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const MAKELEVEL_NAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"MAKELEVEL\0") };
pub const RECIPEPREFIX_DEFAULT: ::core::ffi::c_int = '\t' as i32;
#[no_mangle]
pub static mut env_recursion: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
static mut variable_changenum: ::core::ffi::c_ulong = 0;
static mut pattern_vars: *mut pattern_var = ::core::ptr::null::<pattern_var>() as *mut pattern_var;
static mut last_pattern_vars: [*mut pattern_var; 256] =
    [::core::ptr::null::<pattern_var>() as *mut pattern_var; 256];
#[no_mangle]
pub unsafe extern "C" fn create_pattern_var(
    mut target: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
) -> *mut pattern_var {
    let mut len: size_t = strlen(target) as size_t;
    let mut p: *mut pattern_var =
        xcalloc(::core::mem::size_of::<pattern_var>() as size_t) as *mut pattern_var;
    if !pattern_vars.is_null() {
        if len < 256 && !last_pattern_vars[len as usize].is_null() {
            (*p).next = (*last_pattern_vars[len as usize]).next;
            (*last_pattern_vars[len as usize]).next = p;
        } else {
            let mut v: *mut *mut pattern_var = ::core::ptr::null_mut::<*mut pattern_var>();
            v = &raw mut pattern_vars;
            loop {
                if (*v).is_null() || (**v).len > len {
                    (*p).next = *v;
                    *v = p;
                    break;
                } else {
                    v = &raw mut (**v).next;
                }
            }
        }
    } else {
        pattern_vars = p;
        (*p).next = ::core::ptr::null_mut::<pattern_var>();
    }
    (*p).target = target;
    (*p).len = len;
    (*p).suffix = suffix.offset(1 as ::core::ffi::c_int as isize);
    if len < 256 {
        last_pattern_vars[len as usize] = p;
    }
    p
}
unsafe extern "C" fn lookup_pattern_var(
    mut start: *mut pattern_var,
    mut target: *const ::core::ffi::c_char,
    mut targlen: size_t,
) -> *mut pattern_var {
    let mut p: *mut pattern_var = ::core::ptr::null_mut::<pattern_var>();
    p = if !start.is_null() {
        (*start).next
    } else {
        pattern_vars
    };
    while !p.is_null() {
        let mut stem: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut stemlen: size_t = 0;
        if !((*p).len > targlen) {
            stem = target.offset(
                ((*p).suffix.offset_from((*p).target) as ::core::ffi::c_long
                    - 1) as isize,
            );
            stemlen = targlen.wrapping_sub((*p).len).wrapping_add(1);
            if !(stem > target
                && !(strncmp(
                    (*p).target,
                    target,
                    stem.offset_from(target) as ::core::ffi::c_long as size_t,
                ) == 0))
            {
                if *(*p).suffix as ::core::ffi::c_int
                    == *stem.offset(stemlen as isize) as ::core::ffi::c_int
                    && (*(*p).suffix as ::core::ffi::c_int == 0
                        || *(*p).suffix.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == *stem.offset(stemlen.wrapping_add(1) as isize)
                                as ::core::ffi::c_int
                            && (*(*p).suffix.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0
                                || strcmp(
                                    ((*p).suffix.offset(1 as ::core::ffi::c_int as isize)
                                        as *const ::core::ffi::c_char)
                                        .offset(1 as ::core::ffi::c_int as isize),
                                    (stem.offset(stemlen.wrapping_add(1) as isize)
                                        as *const ::core::ffi::c_char)
                                        .offset(1 as ::core::ffi::c_int as isize),
                                ) == 0))
                {
                    break;
                }
            }
        }
        p = (*p).next;
    }
    p
}
#[no_mangle]
pub unsafe extern "C" fn variable_hash_1(mut keyv: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut key: *const variable = keyv as *const variable;
    let mut _result_: ::core::ffi::c_ulong = 0;
    let mut _key_: *const ::core::ffi::c_uchar = (*key).name as *const ::core::ffi::c_uchar;
    _result_ = _result_
        .wrapping_add(jhash(_key_, (*key).length as ::core::ffi::c_int) as ::core::ffi::c_ulong);
    _result_
}
#[no_mangle]
pub unsafe extern "C" fn variable_hash_2(mut keyv: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _key: *const variable = keyv as *const variable;
    let mut _result_: ::core::ffi::c_ulong = 0;
    _result_
}
unsafe extern "C" fn variable_hash_cmp(
    mut xv: *const ::core::ffi::c_void,
    mut yv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut x: *const variable = xv as *const variable;
    let mut y: *const variable = yv as *const variable;
    let mut result: ::core::ffi::c_int =
        (*x).length.wrapping_sub((*y).length) as ::core::ffi::c_int;
    if result != 0 {
        return result;
    }
    if (*x).name == (*y).name {
        0
    } else {
        memcmp(
            (*x).name as *const ::core::ffi::c_void,
            (*y).name as *const ::core::ffi::c_void,
            (*x).length as size_t,
        )
    }
}
pub const VARIABLE_BUCKETS: ::core::ffi::c_int = 523;
pub const PERFILE_VARIABLE_BUCKETS: ::core::ffi::c_int = 23;
pub const SMALL_SCOPE_VARIABLE_BUCKETS: ::core::ffi::c_int = 13;
static mut global_variable_set: variable_set = variable_set {
    table: hash_table {
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
    },
};
static mut global_setlist: variable_set_list = unsafe {
    variable_set_list {
        next: ::core::ptr::null::<variable_set_list>() as *mut variable_set_list,
        set: &raw const global_variable_set as *mut variable_set,
        next_is_parent: 0,
    }
};
#[no_mangle]
pub static mut current_variable_set_list: *mut variable_set_list =
    unsafe { &raw const global_setlist as *mut variable_set_list };
unsafe extern "C" fn check_valid_name(
    mut flocp: *const Floc,
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
) {
    let mut cp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !(warnings[wt_invalid_var as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
        > w_ignore as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        return;
    }
    cp = name;
    end = name.offset(length as isize);
    while cp < end {
        if *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*cp as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
            != 0
        {
            break;
        }
        cp = cp.offset(1 as ::core::ffi::c_int as isize);
    }
    if cp == end {
        return;
    }
    if warnings[wt_invalid_var as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
        > w_ignore as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut _a: *mut ::core::ffi::c_char = xstrdup(format(
            ::core::ptr::null::<::core::ffi::c_char>(),
            (53 as size_t)
                .wrapping_mul(::core::mem::size_of::<uintmax_t>() as size_t)
                .wrapping_div(22)
                .wrapping_add(3)
                .wrapping_add(strlen(name) as size_t),
            b"invalid variable name '%.*s'\0" as *const u8 as *const ::core::ffi::c_char,
            length as ::core::ffi::c_int,
            name,
        ));
        if warnings[wt_invalid_var as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
            == w_error as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            fatal(
                flocp,
                strlen(_a) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                _a,
            );
        }
        error(
            flocp,
            strlen(_a) as size_t,
            b"warning: %s\0" as *const u8 as *const ::core::ffi::c_char,
            _a,
        );
        free(_a as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_hash_global_variable_set() {
    hash_init(
        &raw mut global_variable_set.table,
        VARIABLE_BUCKETS as ::core::ffi::c_ulong,
        Some(
            variable_hash_1
                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            variable_hash_2
                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            variable_hash_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn define_variable_in_set(
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
    mut value: *const ::core::ffi::c_char,
    mut origin: variable_origin,
    mut recursive: ::core::ffi::c_int,
    mut set: *mut variable_set,
    mut flocp: *const Floc,
) -> *mut variable {
    let mut v: *mut variable = ::core::ptr::null_mut::<variable>();
    let mut var_slot: *mut *mut variable = ::core::ptr::null_mut::<*mut variable>();
    let mut var_key: variable = variable {
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
    check_valid_name(flocp, name, length);
    if set.is_null() {
        set = &raw mut global_variable_set;
    }
    var_key.name = name as *mut ::core::ffi::c_char;
    var_key.length = length as ::core::ffi::c_uint;
    var_slot = hash_find_slot(
        &raw mut (*set).table,
        &raw mut var_key as *const ::core::ffi::c_void,
    ) as *mut *mut variable;
    v = *var_slot;
    if env_overrides != 0
        && origin as ::core::ffi::c_uint == o_env as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        origin = o_env_override;
    }
    if !(v.is_null()
        || v as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
    {
        if env_overrides != 0 && (*v).origin() as ::core::ffi::c_int == o_env as ::core::ffi::c_int
        {
            (*v).set_origin(o_env_override as variable_origin);
        }
        if origin as ::core::ffi::c_int >= (*v).origin() as ::core::ffi::c_int {
            free((*v).value as *mut ::core::ffi::c_void);
            (*v).value = xstrdup(value);
            if !flocp.is_null() {
                (*v).fileinfo = *flocp;
            } else {
                (*v).fileinfo.filenm = ::core::ptr::null::<::core::ffi::c_char>();
            }
            (*v).set_origin(origin as variable_origin);
            (*v).set_recursive(recursive as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        return v;
    }
    v = xcalloc(::core::mem::size_of::<variable>() as size_t) as *mut variable;
    (*v).name = xstrndup(name, length);
    (*v).length = length as ::core::ffi::c_uint;
    hash_insert_at(
        &raw mut (*set).table,
        v as *const ::core::ffi::c_void,
        var_slot as *const ::core::ffi::c_void,
    );
    if set == &raw mut global_variable_set {
        variable_changenum = variable_changenum.wrapping_add(1);
    }
    (*v).value = xstrdup(value);
    if !flocp.is_null() {
        (*v).fileinfo = *flocp;
    }
    (*v).set_origin(origin as variable_origin);
    (*v).set_recursive(recursive as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*v).set_export(v_default as variable_export);
    (*v).set_exportable(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    name = (*v).name;
    if *name as ::core::ffi::c_int != '_' as i32
        && ((*name as ::core::ffi::c_int) < 'A' as i32 || *name as ::core::ffi::c_int > 'Z' as i32)
        && ((*name as ::core::ffi::c_int) < 'a' as i32 || *name as ::core::ffi::c_int > 'z' as i32)
    {
        (*v).set_exportable(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        name = name.offset(1 as ::core::ffi::c_int as isize);
        while *name as ::core::ffi::c_int != 0 {
            if *name as ::core::ffi::c_int != '_' as i32
                && ((*name as ::core::ffi::c_int) < 'a' as i32
                    || *name as ::core::ffi::c_int > 'z' as i32)
                && ((*name as ::core::ffi::c_int) < 'A' as i32
                    || *name as ::core::ffi::c_int > 'Z' as i32)
                && !((*name as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint)
                    <= 9)
            {
                break;
            }
            name = name.offset(1 as ::core::ffi::c_int as isize);
        }
        if *name as ::core::ffi::c_int != 0 {
            (*v).set_exportable(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
    v
}
#[no_mangle]
pub unsafe extern "C" fn free_variable_name_and_value(mut item: *const ::core::ffi::c_void) {
    let mut v: *mut variable = item as *mut variable;
    free((*v).name as *mut ::core::ffi::c_void);
    free((*v).value as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn free_variable_set(mut list: *mut variable_set_list) {
    hash_map(
        &raw mut (*(*list).set).table,
        Some(
            free_variable_name_and_value as unsafe extern "C" fn(*const ::core::ffi::c_void) -> (),
        ),
    );
    hash_free(&raw mut (*(*list).set).table, 1);
    free((*list).set as *mut ::core::ffi::c_void);
    free(list as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn undefine_variable_in_set(
    mut flocp: *const Floc,
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
    mut origin: variable_origin,
    mut set: *mut variable_set,
) {
    let mut v: *mut variable = ::core::ptr::null_mut::<variable>();
    let mut var_slot: *mut *mut variable = ::core::ptr::null_mut::<*mut variable>();
    let mut var_key: variable = variable {
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
    check_valid_name(flocp, name, length);
    if set.is_null() {
        set = &raw mut global_variable_set;
    }
    var_key.name = name as *mut ::core::ffi::c_char;
    var_key.length = length as ::core::ffi::c_uint;
    var_slot = hash_find_slot(
        &raw mut (*set).table,
        &raw mut var_key as *const ::core::ffi::c_void,
    ) as *mut *mut variable;
    if env_overrides != 0
        && origin as ::core::ffi::c_uint == o_env as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        origin = o_env_override;
    }
    v = *var_slot;
    if !(v.is_null()
        || v as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
    {
        if env_overrides != 0 && (*v).origin() as ::core::ffi::c_int == o_env as ::core::ffi::c_int
        {
            (*v).set_origin(o_env_override as variable_origin);
        }
        if origin as ::core::ffi::c_int >= (*v).origin() as ::core::ffi::c_int {
            hash_delete_at(
                &raw mut (*set).table,
                var_slot as *const ::core::ffi::c_void,
            );
            free_variable_name_and_value(v as *const ::core::ffi::c_void);
            free(v as *mut ::core::ffi::c_void);
            if set == &raw mut global_variable_set {
                variable_changenum = variable_changenum.wrapping_add(1);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lookup_special_var(mut var: *mut variable) -> *mut variable {
    static mut last_changenum: ::core::ffi::c_ulong = 0;
    if variable_changenum != last_changenum
        && (*(*var).name as ::core::ffi::c_int
            == *(b".VARIABLES\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
            && (*(*var).name as ::core::ffi::c_int == 0
                || strcmp(
                    (*var).name.offset(1 as ::core::ffi::c_int as isize),
                    (b".VARIABLES\0" as *const u8 as *const ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize),
                ) == 0))
    {
        let mut max: size_t = (strlen((*var).value) as size_t)
            .wrapping_div(500)
            .wrapping_add(1)
            .wrapping_mul(500);
        let mut len: size_t = 0;
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut vp: *mut *mut variable = global_variable_set.table.ht_vec as *mut *mut variable;
        let mut end: *mut *mut variable =
            vp.offset(global_variable_set.table.ht_size as isize) as *mut *mut variable;
        (*var).value =
            xrealloc((*var).value as *mut ::core::ffi::c_void, max) as *mut ::core::ffi::c_char;
        p = (*var).value;
        len = 0;
        while vp < end {
            if !((*vp).is_null()
                || *vp as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
            {
                let mut v: *mut variable = *vp;
                let mut l: ::core::ffi::c_int = (*v).length as ::core::ffi::c_int;
                len = len.wrapping_add((l + 1) as size_t);
                if len > max {
                    let mut off: size_t =
                        p.offset_from((*var).value) as ::core::ffi::c_long as size_t;
                    max = max.wrapping_add(
                        (((l + 1) / 500
                            + 1)
                            * 500) as size_t,
                    );
                    (*var).value = xrealloc((*var).value as *mut ::core::ffi::c_void, max)
                        as *mut ::core::ffi::c_char;
                    p = (*var).value.offset(off as isize) as *mut ::core::ffi::c_char;
                }
                p = mempcpy(
                    p as *mut ::core::ffi::c_void,
                    (*v).name as *const ::core::ffi::c_void,
                    l as size_t,
                ) as *mut ::core::ffi::c_char;
                let fresh4 = p;
                p = p.offset(1 as ::core::ffi::c_int as isize);
                *fresh4 = ' ' as i32 as ::core::ffi::c_char;
            }
            vp = vp.offset(1 as ::core::ffi::c_int as isize);
        }
        *p.offset(-(1 as ::core::ffi::c_int as isize)) = 0;
        last_changenum = variable_changenum;
    }
    var
}
unsafe extern "C" fn check_variable_reference(
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
) {
    let mut cp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !(warnings[wt_invalid_ref as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
        > w_ignore as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        return;
    }
    cp = name;
    end = name.offset(length as isize);
    while cp < end {
        if *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*cp as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
            != 0
        {
            break;
        }
        cp = cp.offset(1 as ::core::ffi::c_int as isize);
    }
    if cp == end {
        return;
    }
    if warnings[wt_invalid_ref as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
        > w_ignore as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut _a: *mut ::core::ffi::c_char = xstrdup(format(
            ::core::ptr::null::<::core::ffi::c_char>(),
            (53 as size_t)
                .wrapping_mul(::core::mem::size_of::<uintmax_t>() as size_t)
                .wrapping_div(22)
                .wrapping_add(3)
                .wrapping_add(strlen(name) as size_t),
            b"invalid variable reference '%.*s'\0" as *const u8 as *const ::core::ffi::c_char,
            length as ::core::ffi::c_int,
            name,
        ));
        if warnings[wt_invalid_ref as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
            == w_error as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            fatal(
                *expanding_var,
                strlen(_a) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                _a,
            );
        }
        error(
            *expanding_var,
            strlen(_a) as size_t,
            b"warning: %s\0" as *const u8 as *const ::core::ffi::c_char,
            _a,
        );
        free(_a as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lookup_variable(
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
) -> *mut variable {
    let mut setlist: *const variable_set_list = ::core::ptr::null::<variable_set_list>();
    let mut var_key: variable = variable {
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
    let mut is_parent: ::core::ffi::c_int = 0;
    check_variable_reference(name, length);
    var_key.name = name as *mut ::core::ffi::c_char;
    var_key.length = length as ::core::ffi::c_uint;
    setlist = current_variable_set_list;
    while !setlist.is_null() {
        let mut set: *const variable_set = (*setlist).set;
        let mut v: *mut variable = ::core::ptr::null_mut::<variable>();
        v = hash_find_item(
            &raw const (*set).table as *mut hash_table,
            &raw mut var_key as *const ::core::ffi::c_void,
        ) as *mut variable;
        if !v.is_null() && (is_parent == 0 || (*v).private_var() == 0) {
            return if (*v).special() as ::core::ffi::c_int != 0 {
                lookup_special_var(v)
            } else {
                v
            };
        }
        is_parent |= (*setlist).next_is_parent;
        setlist = (*setlist).next;
    }
    ::core::ptr::null_mut::<variable>()
}
#[no_mangle]
pub unsafe extern "C" fn lookup_variable_for_file(
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
    mut file: *mut file,
) -> *mut variable {
    let mut var: *mut variable = ::core::ptr::null_mut::<variable>();
    let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    if file.is_null() {
        return lookup_variable(name, length);
    }
    install_file_context(file, &raw mut savev, ::core::ptr::null_mut::<*const Floc>());
    var = lookup_variable(name, length);
    restore_file_context(savev, ::core::ptr::null::<Floc>());
    var
}
#[no_mangle]
pub unsafe extern "C" fn lookup_variable_in_set(
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
    mut set: *const variable_set,
) -> *mut variable {
    let mut var_key: variable = variable {
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
    check_variable_reference(name, length);
    var_key.name = name as *mut ::core::ffi::c_char;
    var_key.length = length as ::core::ffi::c_uint;
    hash_find_item(
        &raw const (*set).table as *mut hash_table,
        &raw mut var_key as *const ::core::ffi::c_void,
    ) as *mut variable
}
#[no_mangle]
pub unsafe extern "C" fn initialize_file_variables(
    mut file: *mut file,
    mut reading: ::core::ffi::c_int,
) {
    let mut l: *mut variable_set_list = (*file).variables;
    if l.is_null() {
        l = xmalloc(::core::mem::size_of::<variable_set_list>() as size_t)
            as *mut variable_set_list;
        (*l).set = xmalloc(::core::mem::size_of::<variable_set>() as size_t) as *mut variable_set;
        hash_init(
            &raw mut (*(*l).set).table,
            PERFILE_VARIABLE_BUCKETS as ::core::ffi::c_ulong,
            Some(
                variable_hash_1
                    as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
            ),
            Some(
                variable_hash_2
                    as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
            ),
            Some(
                variable_hash_cmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        (*file).variables = l;
    }
    if !(*file).double_colon.is_null() && (*file).double_colon != file {
        initialize_file_variables((*file).double_colon, reading);
        (*l).next = (*(*file).double_colon).variables;
        (*l).next_is_parent = 0;
        return;
    }
    if (*file).parent.is_null() {
        (*l).next = &raw mut global_setlist;
    } else {
        initialize_file_variables((*file).parent, reading);
        (*l).next = (*(*file).parent).variables;
    }
    (*l).next_is_parent = 1;
    if reading == 0 && (*file).pat_searched() == 0 {
        let mut p: *mut pattern_var = ::core::ptr::null_mut::<pattern_var>();
        let targlen: size_t = strlen((*file).name) as size_t;
        p = lookup_pattern_var(
            ::core::ptr::null_mut::<pattern_var>(),
            (*file).name,
            targlen,
        );
        if !p.is_null() {
            let mut global: *mut variable_set_list = current_variable_set_list;
            (*file).pat_variables = create_new_variable_set();
            current_variable_set_list = (*file).pat_variables;
            loop {
                let mut v: *mut variable = ::core::ptr::null_mut::<variable>();
                if (*p).variable.flavor() as ::core::ffi::c_int == f_simple as ::core::ffi::c_int {
                    v = define_variable_in_set(
                        (*p).variable.name,
                        strlen((*p).variable.name) as size_t,
                        (*p).variable.value,
                        (*p).variable.origin(),
                        0,
                        (*current_variable_set_list).set,
                        &raw mut (*p).variable.fileinfo,
                    );
                    (*v).set_flavor(f_simple as variable_flavor);
                } else {
                    v = do_variable_definition(
                        &raw mut (*p).variable.fileinfo,
                        (*p).variable.name,
                        (*p).variable.value,
                        (*p).variable.origin(),
                        (*p).variable.flavor(),
                        (*p).variable.conditional() as ::core::ffi::c_int,
                        s_pattern,
                    );
                }
                (*v).set_per_target((*p).variable.per_target() as ::core::ffi::c_uint);
                (*v).set_export((*p).variable.export() as variable_export);
                (*v).set_private_var((*p).variable.private_var() as ::core::ffi::c_uint);
                p = lookup_pattern_var(p, (*file).name, targlen);
                if p.is_null() {
                    break;
                }
            }
            current_variable_set_list = global;
        }
        (*file).set_pat_searched(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if !(*file).pat_variables.is_null() {
        (*(*file).pat_variables).next = (*l).next;
        (*(*file).pat_variables).next_is_parent = (*l).next_is_parent;
        (*l).next = (*file).pat_variables;
        (*l).next_is_parent = 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn create_new_variable_set() -> *mut variable_set_list {
    let mut setlist: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut set: *mut variable_set = ::core::ptr::null_mut::<variable_set>();
    set = xmalloc(::core::mem::size_of::<variable_set>() as size_t) as *mut variable_set;
    hash_init(
        &raw mut (*set).table,
        SMALL_SCOPE_VARIABLE_BUCKETS as ::core::ffi::c_ulong,
        Some(
            variable_hash_1
                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            variable_hash_2
                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            variable_hash_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    setlist =
        xmalloc(::core::mem::size_of::<variable_set_list>() as size_t) as *mut variable_set_list;
    (*setlist).set = set;
    (*setlist).next = current_variable_set_list;
    (*setlist).next_is_parent = 0;
    setlist
}
#[no_mangle]
pub unsafe extern "C" fn push_new_variable_scope() -> *mut variable_set_list {
    current_variable_set_list = create_new_variable_set();
    if (*current_variable_set_list).next == &raw mut global_setlist {
        let mut set: *mut variable_set = (*current_variable_set_list).set;
        (*current_variable_set_list).set = global_setlist.set;
        global_setlist.set = set;
        (*current_variable_set_list).next = global_setlist.next;
        global_setlist.next = current_variable_set_list;
        current_variable_set_list = &raw mut global_setlist;
    }
    current_variable_set_list
}
#[no_mangle]
pub unsafe extern "C" fn pop_variable_scope() {
    let mut setlist: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut set: *mut variable_set = ::core::ptr::null_mut::<variable_set>();
    '_c2rust_label: {
        if !(*current_variable_set_list).next.is_null() {
        } else {
            make_assert_fail ! ( b"current_variable_set_list->next != NULL\0" as *const u8
                    as *const ::core::ffi::c_char ) ;
        }
    };
    if current_variable_set_list != &raw mut global_setlist {
        setlist = current_variable_set_list;
        set = (*setlist).set;
        current_variable_set_list = (*setlist).next;
    } else {
        setlist = global_setlist.next;
        set = global_setlist.set;
        global_setlist.set = (*setlist).set;
        global_setlist.next = (*setlist).next;
        global_setlist.next_is_parent = (*setlist).next_is_parent;
    }
    free(setlist as *mut ::core::ffi::c_void);
    hash_map(
        &raw mut (*set).table,
        Some(
            free_variable_name_and_value as unsafe extern "C" fn(*const ::core::ffi::c_void) -> (),
        ),
    );
    hash_free(&raw mut (*set).table, 1);
    free(set as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn install_file_context(
    mut file: *mut file,
    mut oldlist: *mut *mut variable_set_list,
    mut oldfloc: *mut *const Floc,
) {
    *oldlist = current_variable_set_list;
    current_variable_set_list = (*file).variables;
    if !oldfloc.is_null() {
        *oldfloc = reading_file;
        if !(*file).cmds.is_null() && !(*(*file).cmds).fileinfo.filenm.is_null() {
            reading_file = &raw mut (*(*file).cmds).fileinfo;
        } else {
            reading_file = ::core::ptr::null::<Floc>();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn restore_file_context(
    mut oldlist: *mut variable_set_list,
    mut oldfloc: *const Floc,
) {
    current_variable_set_list = oldlist;
    if !oldfloc.is_null() {
        reading_file = oldfloc;
    }
}
unsafe extern "C" fn merge_variable_sets(
    mut to_set: *mut variable_set,
    mut from_set: *mut variable_set,
) {
    let mut from_var_slot: *mut *mut variable = (*from_set).table.ht_vec as *mut *mut variable;
    let mut from_var_end: *mut *mut variable =
        from_var_slot.offset((*from_set).table.ht_size as isize);
    let mut inc: ::core::ffi::c_int = if to_set == &raw mut global_variable_set {
        1
    } else {
        0
    };
    while from_var_slot < from_var_end {
        if !((*from_var_slot).is_null()
            || *from_var_slot as *mut ::core::ffi::c_void
                == hash_deleted_item as *mut ::core::ffi::c_void)
        {
            let mut from_var: *mut variable = *from_var_slot;
            let mut to_var_slot: *mut *mut variable = hash_find_slot(
                &raw mut (*to_set).table,
                *from_var_slot as *const ::core::ffi::c_void,
            ) as *mut *mut variable;
            if (*to_var_slot).is_null()
                || *to_var_slot as *mut ::core::ffi::c_void
                    == hash_deleted_item as *mut ::core::ffi::c_void
            {
                hash_insert_at(
                    &raw mut (*to_set).table,
                    from_var as *const ::core::ffi::c_void,
                    to_var_slot as *const ::core::ffi::c_void,
                );
                variable_changenum = variable_changenum.wrapping_add(inc as ::core::ffi::c_ulong);
            } else {
                free((*from_var).value as *mut ::core::ffi::c_void);
                free(from_var as *mut ::core::ffi::c_void);
            }
        }
        from_var_slot = from_var_slot.offset(1 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn merge_variable_set_lists(
    mut setlist0: *mut *mut variable_set_list,
    mut setlist1: *mut variable_set_list,
) {
    let mut to: *mut variable_set_list = *setlist0;
    let mut last0: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    if setlist1.is_null() || setlist1 == &raw mut global_setlist {
        return;
    }
    if !to.is_null() {
        while to != &raw mut global_setlist {
            if to == setlist1 {
                return;
            }
            to = (*to).next;
        }
        to = *setlist0;
        while setlist1 != &raw mut global_setlist && to != &raw mut global_setlist {
            let mut from: *mut variable_set_list = setlist1;
            setlist1 = (*setlist1).next;
            merge_variable_sets((*to).set, (*from).set);
            last0 = to;
            to = (*to).next;
        }
    }
    if setlist1 != &raw mut global_setlist {
        if last0.is_null() {
            *setlist0 = setlist1;
        } else {
            (*last0).next = setlist1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn define_automatic_variables() {
    let mut v: *mut variable = ::core::ptr::null_mut::<variable>();
    let mut buf: [::core::ffi::c_char; 200] = [0; 200];
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%u\0" as *const u8 as *const ::core::ffi::c_char,
        makelevel,
    );
    define_variable_in_set(
        b"MAKELEVEL\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        &raw mut buf as *mut ::core::ffi::c_char,
        o_env,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        version_string,
        if remote_description.is_null()
            || *remote_description.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0
        {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"-\0" as *const u8 as *const ::core::ffi::c_char
        },
        if remote_description.is_null()
            || *remote_description.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0
        {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            remote_description as *const ::core::ffi::c_char
        },
    );
    define_variable_in_set(
        b"MAKE_VERSION\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        &raw mut buf as *mut ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"MAKE_HOST\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        make_host,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    v = define_variable_in_set(
        b"SHELL\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
        default_shell,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    if *(*v).value as ::core::ffi::c_int == 0
        || (*v).origin() as ::core::ffi::c_int == o_env as ::core::ffi::c_int
        || (*v).origin() as ::core::ffi::c_int == o_env_override as ::core::ffi::c_int
    {
        free((*v).value as *mut ::core::ffi::c_void);
        (*v).set_origin(o_file as variable_origin);
        (*v).value = xstrdup(default_shell);
    }
    v = define_variable_in_set(
        b"MAKEFILES\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    (*v).set_export(v_ifset as variable_export);
    define_variable_in_set(
        b"@D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $@))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"%D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $%))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"*D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $*))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"<D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $<))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"?D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $?))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"^D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $^))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"+D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $+))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"@F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $@)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"%F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $%)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"*F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $*)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"<F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $<)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"?F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $?)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"^F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $^)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        b"+F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $+)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
}
#[no_mangle]
pub unsafe extern "C" fn should_export(mut v: *const variable) -> ::core::ffi::c_int {
    match (*v).export() as ::core::ffi::c_int {
        2 => return 0,
        3 => {
            if (*v).origin() as ::core::ffi::c_int == o_default as ::core::ffi::c_int {
                return 0;
            }
        }
        0 => {
            if (*v).origin() as ::core::ffi::c_int == o_default as ::core::ffi::c_int
                || (*v).origin() as ::core::ffi::c_int == o_automatic as ::core::ffi::c_int
            {
                return 0;
            }
            if (*v).exportable() == 0 {
                return 0;
            }
            if export_all_variables == 0
                && (*v).origin() as ::core::ffi::c_int != o_command as ::core::ffi::c_int
                && (*v).origin() as ::core::ffi::c_int != o_env as ::core::ffi::c_int
                && (*v).origin() as ::core::ffi::c_int != o_env_override as ::core::ffi::c_int
            {
                return 0;
            }
        }
        1 | _ => {}
    }
    1
}
#[no_mangle]
pub unsafe extern "C" fn target_environment(
    mut file: *mut file,
    mut recursive: ::core::ffi::c_int,
) -> *mut *mut ::core::ffi::c_char {
    let mut set_list: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut s: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut table: hash_table = hash_table {
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
    let mut v_slot: *mut *mut variable = ::core::ptr::null_mut::<*mut variable>();
    let mut v_end: *mut *mut variable = ::core::ptr::null_mut::<*mut variable>();
    let mut result_0: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut result: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut invalid: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut added_SHELL: ::core::ffi::c_int =
        (shell_var.value == ::core::ptr::null_mut::<::core::ffi::c_char>()) as ::core::ffi::c_int;
    let mut found_makelevel: ::core::ffi::c_int = 0;
    let mut found_mflags: ::core::ffi::c_int = 0;
    let mut found_makeflags: ::core::ffi::c_int = 0;
    if file.is_null() {
        env_recursion = env_recursion.wrapping_add(1);
    }
    if recursive == 0 && !jobserver_auth.is_null() {
        invalid = jobserver_get_invalid_auth();
    }
    if !file.is_null() {
        set_list = (*file).variables;
    } else {
        set_list = current_variable_set_list;
    }
    hash_init(
        &raw mut table,
        VARIABLE_BUCKETS as ::core::ffi::c_ulong,
        Some(
            variable_hash_1
                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            variable_hash_2
                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            variable_hash_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    s = set_list;
    while !s.is_null() {
        let mut set: *mut variable_set = (*s).set;
        let islocal: ::core::ffi::c_int = (s == set_list) as ::core::ffi::c_int;
        let isglobal: ::core::ffi::c_int =
            (set == &raw mut global_variable_set) as ::core::ffi::c_int;
        v_slot = (*set).table.ht_vec as *mut *mut variable;
        v_end = v_slot.offset((*set).table.ht_size as isize);
        while v_slot < v_end {
            if !((*v_slot).is_null()
                || *v_slot as *mut ::core::ffi::c_void
                    == hash_deleted_item as *mut ::core::ffi::c_void)
            {
                let mut evslot: *mut *mut variable = ::core::ptr::null_mut::<*mut variable>();
                let mut v: *mut variable = *v_slot;
                if !(islocal == 0 && (*v).private_var() as ::core::ffi::c_int != 0) {
                    evslot = hash_find_slot(&raw mut table, v as *const ::core::ffi::c_void)
                        as *mut *mut variable;
                    if (*evslot).is_null()
                        || *evslot as *mut ::core::ffi::c_void
                            == hash_deleted_item as *mut ::core::ffi::c_void
                    {
                        if isglobal == 0 || should_export(v) != 0 {
                            hash_insert_at(
                                &raw mut table,
                                v as *const ::core::ffi::c_void,
                                evslot as *const ::core::ffi::c_void,
                            );
                        }
                    } else if (**evslot).export() as ::core::ffi::c_int
                        == v_default as ::core::ffi::c_int
                    {
                        (**evslot).set_export((*v).export() as variable_export);
                    }
                }
            }
            v_slot = v_slot.offset(1 as ::core::ffi::c_int as isize);
        }
        s = (*s).next;
    }
    result_0 = xmalloc(
        (table.ht_fill as size_t)
            .wrapping_add(3)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    result = result_0;
    v_slot = table.ht_vec as *mut *mut variable;
    v_end = v_slot.offset(table.ht_size as isize);
    while v_slot < v_end {
        if !((*v_slot).is_null()
            || *v_slot as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
        {
            let mut v_0: *mut variable = *v_slot;
            let mut value: *mut ::core::ffi::c_char = (*v_0).value;
            let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !(should_export(v_0) == 0) {
                if (*v_0).recursive() as ::core::ffi::c_int != 0
                    && ((*v_0).origin() as ::core::ffi::c_int != o_env as ::core::ffi::c_int
                        && (*v_0).origin() as ::core::ffi::c_int
                            != o_env_override as ::core::ffi::c_int
                        || *(*v_0).name as ::core::ffi::c_int
                            == *(b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                as ::core::ffi::c_int
                            && (*(*v_0).name as ::core::ffi::c_int == 0
                                || strcmp(
                                    (*v_0).name.offset(1 as ::core::ffi::c_int as isize),
                                    (b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                        .offset(1 as ::core::ffi::c_int as isize),
                                ) == 0))
                {
                    cp = recursively_expand_for_file(v_0, file);
                    value = cp;
                }
                if added_SHELL == 0
                    && (*(*v_0).name as ::core::ffi::c_int
                        == *(b"SHELL\0" as *const u8 as *const ::core::ffi::c_char)
                            as ::core::ffi::c_int
                        && (*(*v_0).name as ::core::ffi::c_int == 0
                            || strcmp(
                                (*v_0).name.offset(1 as ::core::ffi::c_int as isize),
                                (b"SHELL\0" as *const u8 as *const ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize),
                            ) == 0))
                {
                    added_SHELL = 1;
                } else if found_makelevel == 0
                    && (*(*v_0).name as ::core::ffi::c_int
                        == *(b"MAKELEVEL\0" as *const u8 as *const ::core::ffi::c_char)
                            as ::core::ffi::c_int
                        && (*(*v_0).name as ::core::ffi::c_int == 0
                            || strcmp(
                                (*v_0).name.offset(1 as ::core::ffi::c_int as isize),
                                (b"MAKELEVEL\0" as *const u8 as *const ::core::ffi::c_char)
                                    .offset(1 as ::core::ffi::c_int as isize),
                            ) == 0))
                {
                    let mut val: [::core::ffi::c_char; 23] = [0; 23];
                    sprintf(
                        &raw mut val as *mut ::core::ffi::c_char,
                        b"%u\0" as *const u8 as *const ::core::ffi::c_char,
                        makelevel.wrapping_add(1),
                    );
                    free(cp as *mut ::core::ffi::c_void);
                    cp = xstrdup(&raw mut val as *mut ::core::ffi::c_char);
                    value = cp;
                    found_makelevel = 1;
                } else if !invalid.is_null() {
                    if found_makeflags == 0
                        && (*(*v_0).name as ::core::ffi::c_int
                            == *(b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                as ::core::ffi::c_int
                            && (*(*v_0).name as ::core::ffi::c_int == 0
                                || strcmp(
                                    (*v_0).name.offset(1 as ::core::ffi::c_int as isize),
                                    (b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                        .offset(1 as ::core::ffi::c_int as isize),
                                ) == 0))
                    {
                        let mut mf: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let mut vars: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        found_makeflags = 1;
                        if !strstr(
                            value,
                            b" --jobserver-auth=\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        .is_null()
                        {
                            vars =
                                strstr(value, b" -- \0" as *const u8 as *const ::core::ffi::c_char);
                            if vars.is_null() {
                                mf = xstrdup(concat(2, value, invalid));
                            } else {
                                let mut lf: size_t =
                                    vars.offset_from(value) as ::core::ffi::c_long as size_t;
                                let mut li: size_t = strlen(invalid) as size_t;
                                mf = xmalloc(
                                    (strlen(value) as size_t)
                                        .wrapping_add(li)
                                        .wrapping_add(1),
                                ) as *mut ::core::ffi::c_char;
                                strcpy(
                                    mempcpy(
                                        mempcpy(
                                            mf as *mut ::core::ffi::c_void,
                                            value as *const ::core::ffi::c_void,
                                            lf as size_t,
                                        ),
                                        invalid as *const ::core::ffi::c_void,
                                        li as size_t,
                                    )
                                        as *mut ::core::ffi::c_char,
                                    vars,
                                );
                            }
                            free(cp as *mut ::core::ffi::c_void);
                            cp = mf;
                            value = cp;
                            if found_mflags != 0 {
                                invalid = ::core::ptr::null::<::core::ffi::c_char>();
                            }
                        }
                    } else if found_mflags == 0
                        && (*(*v_0).name as ::core::ffi::c_int
                            == *(b"MFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                as ::core::ffi::c_int
                            && (*(*v_0).name as ::core::ffi::c_int == 0
                                || strcmp(
                                    (*v_0).name.offset(1 as ::core::ffi::c_int as isize),
                                    (b"MFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                        .offset(1 as ::core::ffi::c_int as isize),
                                ) == 0))
                    {
                        let mut mf_0: *const ::core::ffi::c_char =
                            ::core::ptr::null::<::core::ffi::c_char>();
                        found_mflags = 1;
                        if !strstr(
                            value,
                            b" --jobserver-auth=\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        .is_null()
                        {
                            if !((*v_0).origin() as ::core::ffi::c_int
                                != o_env as ::core::ffi::c_int)
                            {
                                mf_0 = concat(2, value, invalid);
                                free(cp as *mut ::core::ffi::c_void);
                                cp = xstrdup(mf_0);
                                value = cp;
                                if found_makeflags != 0 {
                                    invalid = ::core::ptr::null::<::core::ffi::c_char>();
                                }
                            }
                        }
                    }
                }
                let fresh10 = result;
                result = result.offset(1 as ::core::ffi::c_int as isize);
                *fresh10 = xstrdup(concat(
                    3,
                    (*v_0).name,
                    b"=\0" as *const u8 as *const ::core::ffi::c_char,
                    value,
                ));
                free(cp as *mut ::core::ffi::c_void);
            }
        }
        v_slot = v_slot.offset(1 as ::core::ffi::c_int as isize);
    }
    if added_SHELL == 0 {
        let fresh11 = result;
        result = result.offset(1 as ::core::ffi::c_int as isize);
        *fresh11 = xstrdup(concat(
            3,
            shell_var.name,
            b"=\0" as *const u8 as *const ::core::ffi::c_char,
            shell_var.value,
        ));
    }
    if found_makelevel == 0 {
        let mut val_0: [::core::ffi::c_char; 33] = [0; 33];
        sprintf(
            &raw mut val_0 as *mut ::core::ffi::c_char,
            b"%s=%u\0" as *const u8 as *const ::core::ffi::c_char,
            MAKELEVEL_NAME.as_ptr(),
            makelevel.wrapping_add(1),
        );
        let fresh12 = result;
        result = result.offset(1 as ::core::ffi::c_int as isize);
        *fresh12 = xstrdup(&raw mut val_0 as *mut ::core::ffi::c_char);
    }
    *result = ::core::ptr::null_mut::<::core::ffi::c_char>();
    hash_free(&raw mut table, 0);
    if file.is_null() {
        env_recursion = env_recursion.wrapping_sub(1);
    }
    result_0
}
unsafe extern "C" fn set_special_var(
    mut var: *mut variable,
    mut origin: variable_origin,
) -> *mut variable {
    if *(*var).name as ::core::ffi::c_int
        == *(b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
        && (*(*var).name as ::core::ffi::c_int == 0
            || strcmp(
                (*var).name.offset(1 as ::core::ffi::c_int as isize),
                (b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize),
            ) == 0)
    {
        reset_makeflags(origin);
    } else if *(*var).name as ::core::ffi::c_int
        == *(b".RECIPEPREFIX\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
        && (*(*var).name as ::core::ffi::c_int == 0
            || strcmp(
                (*var).name.offset(1 as ::core::ffi::c_int as isize),
                (b".RECIPEPREFIX\0" as *const u8 as *const ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize),
            ) == 0)
    {
        cmd_prefix = (if *(*var).value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0
        {
            RECIPEPREFIX_DEFAULT
        } else {
            *(*var).value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
    } else if *(*var).name as ::core::ffi::c_int
        == *(b".WARNINGS\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
        && (*(*var).name as ::core::ffi::c_int == 0
            || strcmp(
                (*var).name.offset(1 as ::core::ffi::c_int as isize),
                (b".WARNINGS\0" as *const u8 as *const ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize),
            ) == 0)
    {
        let mut actions: *mut ::core::ffi::c_char = allocated_expand_variable(
            b".WARNINGS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t)
                .wrapping_sub(1),
        );
        decode_warn_actions(actions, &raw mut (*var).fileinfo);
        free(actions as *mut ::core::ffi::c_void);
    }
    var
}
#[no_mangle]
pub unsafe extern "C" fn shell_result(mut p: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    let mut args: [*mut ::core::ffi::c_char; 2] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 2];
    install_variable_buffer(&raw mut buf, &raw mut len);
    args[0 as ::core::ffi::c_int as usize] = p as *mut ::core::ffi::c_char;
    args[1 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
    func_shell_base(
        variable_buffer,
        &raw mut args as *mut *mut ::core::ffi::c_char,
        0,
    );
    swap_variable_buffer(buf, len)
}
#[no_mangle]
pub unsafe extern "C" fn do_variable_definition(
    mut flocp: *const Floc,
    mut varname: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
    mut origin: variable_origin,
    mut flavor: variable_flavor,
    mut conditional: ::core::ffi::c_int,
    mut scope: variable_scope,
) -> *mut variable {
    let mut current_block: u64;
    let mut newval: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut alloc_value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut v: *mut variable = ::core::ptr::null_mut::<variable>();
    let mut append: ::core::ffi::c_int = 0;
    if conditional != 0 {
        v = lookup_variable(varname, strlen(varname) as size_t);
        if !v.is_null() {
            return v;
        }
    }
    match flavor as ::core::ffi::c_uint {
        1 => {
            alloc_value = allocated_expand_string_for_file(value, ::core::ptr::null_mut::<file>());
            newval = alloc_value;
            current_block = 5159818223158340697;
        }
        3 => {
            let mut t: *mut ::core::ffi::c_char =
                allocated_expand_string_for_file(value, ::core::ptr::null_mut::<file>());
            alloc_value = xmalloc(
                (strlen(t) as size_t)
                    .wrapping_mul(2)
                    .wrapping_add(1),
            ) as *mut ::core::ffi::c_char;
            let mut np: *mut ::core::ffi::c_char = alloc_value;
            let mut op: *mut ::core::ffi::c_char = t;
            while *op.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0 {
                if *op.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '$' as i32 {
                    let fresh0 = np;
                    np = np.offset(1 as ::core::ffi::c_int as isize);
                    *fresh0 = '$' as i32 as ::core::ffi::c_char;
                }
                let fresh1 = op;
                op = op.offset(1 as ::core::ffi::c_int as isize);
                let fresh2 = np;
                np = np.offset(1 as ::core::ffi::c_int as isize);
                *fresh2 = *fresh1;
            }
            *np = 0;
            free(t as *mut ::core::ffi::c_void);
            newval = alloc_value;
            current_block = 5159818223158340697;
        }
        5 => {
            let mut q: *mut ::core::ffi::c_char =
                allocated_expand_string_for_file(value, ::core::ptr::null_mut::<file>());
            alloc_value = shell_result(q);
            free(q as *mut ::core::ffi::c_void);
            flavor = f_recursive;
            newval = alloc_value;
            current_block = 5159818223158340697;
        }
        2 => {
            newval = value;
            current_block = 5159818223158340697;
        }
        4 | 6 => {
            let mut override_0: ::core::ffi::c_int = 0;
            if scope as ::core::ffi::c_uint == s_global as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                v = lookup_variable(varname, strlen(varname) as size_t);
            } else {
                append = 1;
                v = lookup_variable_in_set(
                    varname,
                    strlen(varname) as size_t,
                    (*current_variable_set_list).set,
                );
                if !v.is_null() {
                    if (*v).append() == 0 {
                        append = 0;
                    }
                    if scope as ::core::ffi::c_uint
                        == s_pattern as ::core::ffi::c_int as ::core::ffi::c_uint
                        && ((*v).origin() as ::core::ffi::c_int
                            == o_env_override as ::core::ffi::c_int
                            || (*v).origin() as ::core::ffi::c_int
                                == o_command as ::core::ffi::c_int)
                    {
                        override_0 = 1;
                        append = 1;
                    }
                }
            }
            if v.is_null() {
                newval = value;
                flavor = f_recursive;
                current_block = 5159818223158340697;
            } else if override_0 != 0 {
                newval = value;
                flavor = f_recursive;
                current_block = 5159818223158340697;
            } else {
                let mut oldlen: size_t = 0;
                let mut vallen: size_t = 0;
                let mut alloclen: size_t = 0;
                let mut val: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                let mut cp: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut tp: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                val = value;
                if (*v).recursive() != 0 {
                    flavor = f_recursive;
                } else if flavor as ::core::ffi::c_uint
                    != f_append_value as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    tp = allocated_expand_string_for_file(val, ::core::ptr::null_mut::<file>());
                    val = tp;
                }
                vallen = strlen(val) as size_t;
                if vallen == 0 {
                    alloc_value = tp;
                    current_block = 3071571992406269834;
                } else {
                    oldlen = strlen((*v).value) as size_t;
                    alloclen = oldlen
                        .wrapping_add(1)
                        .wrapping_add(vallen)
                        .wrapping_add(1);
                    alloc_value = xmalloc(alloclen) as *mut ::core::ffi::c_char;
                    cp = alloc_value;
                    if oldlen != 0 {
                        let mut s: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        if *varname as ::core::ffi::c_int
                            == *(b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                as ::core::ffi::c_int
                            && (*varname as ::core::ffi::c_int == 0
                                || strcmp(
                                    varname.offset(1 as ::core::ffi::c_int as isize),
                                    (b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                        .offset(1 as ::core::ffi::c_int as isize),
                                ) == 0)
                            && {
                                s = strstr(
                                    (*v).value,
                                    b" -- \0" as *const u8 as *const ::core::ffi::c_char,
                                );
                                !s.is_null()
                            }
                        {
                            cp = mempcpy(
                                cp as *mut ::core::ffi::c_void,
                                (*v).value as *const ::core::ffi::c_void,
                                s.offset_from((*v).value) as ::core::ffi::c_long as size_t,
                            ) as *mut ::core::ffi::c_char;
                        } else {
                            cp = mempcpy(
                                cp as *mut ::core::ffi::c_void,
                                (*v).value as *const ::core::ffi::c_void,
                                oldlen as size_t,
                            ) as *mut ::core::ffi::c_char;
                        }
                        let fresh3 = cp;
                        cp = cp.offset(1 as ::core::ffi::c_int as isize);
                        *fresh3 = ' ' as i32 as ::core::ffi::c_char;
                    }
                    memcpy(
                        cp as *mut ::core::ffi::c_void,
                        val as *const ::core::ffi::c_void,
                        (vallen as size_t).wrapping_add(1),
                    );
                    free(tp as *mut ::core::ffi::c_void);
                    newval = alloc_value;
                    current_block = 5159818223158340697;
                }
            }
        }
        0 | _ => {
            abort();
        }
    }
    match current_block {
        5159818223158340697 => {
            '_c2rust_label: {
                if !newval.is_null() {
                } else {
                    make_assert_fail ! ( b"newval\0" as *const u8 as *const ::core::ffi::c_char ) ;
                }
            };
            v = define_variable_in_set(
                varname,
                strlen(varname) as size_t,
                newval,
                origin,
                (flavor as ::core::ffi::c_uint
                    == f_recursive as ::core::ffi::c_int as ::core::ffi::c_uint
                    || flavor as ::core::ffi::c_uint
                        == f_expand as ::core::ffi::c_int as ::core::ffi::c_uint)
                    as ::core::ffi::c_int,
                if scope as ::core::ffi::c_uint
                    == s_global as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    ::core::ptr::null_mut::<variable_set>()
                } else {
                    (*current_variable_set_list).set
                },
                flocp,
            );
            (*v).set_append(append as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*v).set_conditional(conditional as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        _ => {}
    }
    free(alloc_value as *mut ::core::ffi::c_void);
    if (*v).special() as ::core::ffi::c_int != 0 {
        set_special_var(v, origin)
    } else {
        v
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_variable_definition(
    mut str: *const ::core::ffi::c_char,
    mut var: *mut variable,
) -> *mut ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char = str;
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        != 0
    {
        p = p.offset(1 as ::core::ffi::c_int as isize);
    }
    (*var).name = p as *mut ::core::ffi::c_char;
    (*var).length = 0;
    (*var).set_conditional(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let mut current_block_37: u64;
    loop {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let fresh5 = p;
        p = p.offset(1 as ::core::ffi::c_int as isize);
        let mut c: ::core::ffi::c_int = *fresh5 as ::core::ffi::c_int;
        if *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(c as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x8 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
            != 0
        {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(c as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & 0x2 as ::core::ffi::c_int
            != 0
        {
            if !end.is_null() {
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            end = p.offset(-(1 as ::core::ffi::c_int as isize));
            while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                != 0
            {
                p = p.offset(1 as ::core::ffi::c_int as isize);
            }
        } else {
            start = p.offset(-(1 as ::core::ffi::c_int as isize));
            if c == '?' as i32 {
                (*var).set_conditional(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                let fresh6 = p;
                p = p.offset(1 as ::core::ffi::c_int as isize);
                c = *fresh6 as ::core::ffi::c_int;
            }
            if c == '=' as i32 {
                if end.is_null() {
                    end = start;
                }
                (*var).set_flavor(f_recursive as variable_flavor);
                break;
            } else if c == ':' as i32 {
                if end.is_null() {
                    end = start;
                }
                let fresh7 = p;
                p = p.offset(1 as ::core::ffi::c_int as isize);
                c = *fresh7 as ::core::ffi::c_int;
                if c == '=' as i32 {
                    (*var).set_flavor(f_simple as variable_flavor);
                    break;
                } else {
                    if c == ':' as i32 {
                        let fresh8 = p;
                        p = p.offset(1 as ::core::ffi::c_int as isize);
                        c = *fresh8 as ::core::ffi::c_int;
                        if c == '=' as i32 {
                            (*var).set_flavor(f_simple as variable_flavor);
                            break;
                        } else if c == ':' as i32 && {
                            let fresh9 = p;
                            p = p.offset(1 as ::core::ffi::c_int as isize);
                            *fresh9 as ::core::ffi::c_int == '=' as i32
                        } {
                            (*var).set_flavor(f_expand as variable_flavor);
                            break;
                        }
                    }
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            } else {
                if *p as ::core::ffi::c_int == '=' as i32 {
                    match c {
                        43 => {
                            current_block_37 = 11856292385005058703;
                            match current_block_37 {
                                5736403253062402380 => {
                                    (*var).set_flavor(f_shell as variable_flavor);
                                }
                                _ => {
                                    (*var).set_flavor(f_append as variable_flavor);
                                }
                            }
                            if end.is_null() {
                                end = start;
                            }
                            p = p.offset(1 as ::core::ffi::c_int as isize);
                            break;
                        }
                        33 => {
                            current_block_37 = 5736403253062402380;
                            match current_block_37 {
                                5736403253062402380 => {
                                    (*var).set_flavor(f_shell as variable_flavor);
                                }
                                _ => {
                                    (*var).set_flavor(f_append as variable_flavor);
                                }
                            }
                            if end.is_null() {
                                end = start;
                            }
                            p = p.offset(1 as ::core::ffi::c_int as isize);
                            break;
                        }
                        _ => {}
                    }
                }
                if !end.is_null() {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                if c == '$' as i32 {
                    p = skip_reference(p);
                }
                (*var).set_conditional(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
    }
    (*var).length = end.offset_from((*var).name) as ::core::ffi::c_long as ::core::ffi::c_uint;
    (*var).value = next_token(p);
    p as *mut ::core::ffi::c_char
}
#[no_mangle]
pub unsafe extern "C" fn assign_variable_definition(
    mut v: *mut variable,
    mut line: *const ::core::ffi::c_char,
) -> *mut variable {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if parse_variable_definition(line, v).is_null() {
        return ::core::ptr::null_mut::<variable>();
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (*v).length.wrapping_add(1) as ::core::ffi::c_ulong as usize,
    ));
    name = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    memcpy(
        name as *mut ::core::ffi::c_void,
        (*v).name as *const ::core::ffi::c_void,
        (*v).length as size_t,
    );
    *name.offset((*v).length as isize) = 0;
    (*v).name = allocated_expand_string_for_file(name, ::core::ptr::null_mut::<file>());
    if *(*v).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0 {
        fatal(
            &raw mut (*v).fileinfo,
            0,
            b"empty variable name\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    v
}
#[no_mangle]
pub unsafe extern "C" fn try_variable_definition(
    mut flocp: *const Floc,
    mut line: *const ::core::ffi::c_char,
    mut origin: variable_origin,
    mut scope: variable_scope,
) -> *mut variable {
    let mut v: variable = variable {
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
    let mut vp: *mut variable = ::core::ptr::null_mut::<variable>();
    if !flocp.is_null() {
        v.fileinfo = *flocp;
    } else {
        v.fileinfo.filenm = ::core::ptr::null::<::core::ffi::c_char>();
    }
    if assign_variable_definition(&raw mut v, line).is_null() {
        return ::core::ptr::null_mut::<variable>();
    }
    vp = do_variable_definition(
        flocp,
        v.name,
        v.value,
        origin,
        v.flavor(),
        v.conditional() as ::core::ffi::c_int,
        scope,
    );
    free(v.name as *mut ::core::ffi::c_void);
    vp
}
static mut defined_vars: [defined_vars; 13] = [defined_vars {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    len: 0,
}; 13];
#[no_mangle]
pub unsafe extern "C" fn warn_undefined(mut name: *const ::core::ffi::c_char, mut len: size_t) {
    if warnings[wt_undefined_var as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
        > w_ignore as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut dp: *const defined_vars = ::core::ptr::null::<defined_vars>();
        dp = &raw const defined_vars as *const defined_vars;
        while !(*dp).name.is_null() {
            if (*dp).len == len
                && memcmp(
                    (*dp).name as *const ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    len as size_t,
                ) == 0
            {
                return;
            }
            dp = dp.offset(1 as ::core::ffi::c_int as isize);
        }
        if warnings[wt_undefined_var as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
            > w_ignore as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut _a: *mut ::core::ffi::c_char = xstrdup(format(
                ::core::ptr::null::<::core::ffi::c_char>(),
                (53 as size_t)
                    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as size_t)
                    .wrapping_div(22)
                    .wrapping_add(3)
                    .wrapping_add(strlen(name) as size_t),
                b"reference to undefined variable '%.*s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                len as ::core::ffi::c_int,
                name,
            ));
            if warnings[wt_undefined_var as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
                == w_error as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                fatal(
                    reading_file,
                    strlen(_a) as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    _a,
                );
            }
            error(
                reading_file,
                strlen(_a) as size_t,
                b"warning: %s\0" as *const u8 as *const ::core::ffi::c_char,
                _a,
            );
            free(_a as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn set_env_override(
    mut item: *const ::core::ffi::c_void,
    mut _arg: *mut ::core::ffi::c_void,
) {
    let mut v: *mut variable = item as *mut variable;
    let mut old: variable_origin = (if env_overrides != 0 {
        o_env as ::core::ffi::c_int
    } else {
        o_env_override as ::core::ffi::c_int
    }) as variable_origin;
    let mut new: variable_origin = (if env_overrides != 0 {
        o_env_override as ::core::ffi::c_int
    } else {
        o_env as ::core::ffi::c_int
    }) as variable_origin;
    if (*v).origin() as ::core::ffi::c_uint == old as ::core::ffi::c_uint {
        (*v).set_origin(new as variable_origin);
    }
}
#[no_mangle]
pub unsafe extern "C" fn reset_env_override() {
    hash_map_arg(
        &raw mut global_variable_set.table,
        Some(
            set_env_override
                as unsafe extern "C" fn(*const ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        ),
        NULL,
    );
}
unsafe extern "C" fn print_variable(
    mut item: *const ::core::ffi::c_void,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut v: *const variable = item as *const variable;
    let mut prefix: *const ::core::ffi::c_char = arg as *const ::core::ffi::c_char;
    let mut origin: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    match (*v).origin() as ::core::ffi::c_int {
        6 => {
            origin = b"automatic\0" as *const u8 as *const ::core::ffi::c_char;
        }
        0 => {
            origin = b"default\0" as *const u8 as *const ::core::ffi::c_char;
        }
        1 => {
            origin = b"environment\0" as *const u8 as *const ::core::ffi::c_char;
        }
        2 => {
            origin = b"makefile\0" as *const u8 as *const ::core::ffi::c_char;
        }
        3 => {
            origin = b"environment under -e\0" as *const u8 as *const ::core::ffi::c_char;
        }
        4 => {
            origin = b"command line\0" as *const u8 as *const ::core::ffi::c_char;
        }
        5 => {
            origin = b"'override' directive\0" as *const u8 as *const ::core::ffi::c_char;
        }
        7 => {
            abort();
        }
        _ => {}
    }
    fputs(b"# \0" as *const u8 as *const ::core::ffi::c_char, stdout);
    fputs(origin, stdout);
    if (*v).private_var() != 0 {
        fputs(
            b" private\0" as *const u8 as *const ::core::ffi::c_char,
            stdout,
        );
    }
    if !(*v).fileinfo.filenm.is_null() {
        printf(
            b" (from '%s', line %lu)\0" as *const u8 as *const ::core::ffi::c_char,
            (*v).fileinfo.filenm,
            (*v).fileinfo.lineno.wrapping_add((*v).fileinfo.offset),
        );
    }
    putchar('\n' as i32);
    fputs(prefix, stdout);
    if (*v).recursive() as ::core::ffi::c_int != 0 && !strchr((*v).value, '\n' as i32).is_null() {
        printf(
            b"define %s\n%s\nendef\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*v).name,
            (*v).value,
        );
    } else {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        printf(
            b"%s %s= \0" as *const u8 as *const ::core::ffi::c_char,
            (*v).name,
            if (*v).recursive() as ::core::ffi::c_int != 0 {
                if (*v).append() as ::core::ffi::c_int != 0 {
                    b"+\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                }
            } else {
                b":\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        p = next_token((*v).value);
        if p != (*v).value && *p as ::core::ffi::c_int == 0 {
            printf(
                b"$(subst ,,%s)\0" as *const u8 as *const ::core::ffi::c_char,
                (*v).value,
            );
        } else if (*v).recursive() != 0 {
            fputs((*v).value, stdout);
        } else {
            p = (*v).value;
            while *p as ::core::ffi::c_int != 0 {
                if *p as ::core::ffi::c_int == '$' as i32 {
                    putchar('$' as i32);
                }
                putchar(*p as ::core::ffi::c_int);
                p = p.offset(1 as ::core::ffi::c_int as isize);
            }
        }
        putchar('\n' as i32);
    };
}
unsafe extern "C" fn print_auto_variable(
    mut item: *const ::core::ffi::c_void,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut v: *const variable = item as *const variable;
    if (*v).origin() as ::core::ffi::c_int == o_automatic as ::core::ffi::c_int {
        print_variable(item, arg);
    }
}
unsafe extern "C" fn print_noauto_variable(
    mut item: *const ::core::ffi::c_void,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut v: *const variable = item as *const variable;
    if (*v).origin() as ::core::ffi::c_int != o_automatic as ::core::ffi::c_int {
        print_variable(item, arg);
    }
}
unsafe extern "C" fn print_variable_set(
    mut set: *mut variable_set,
    mut prefix: *const ::core::ffi::c_char,
    mut pauto: ::core::ffi::c_int,
) {
    hash_map_arg(
        &raw mut (*set).table,
        if pauto != 0 {
            Some(
                print_auto_variable
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            )
        } else {
            Some(
                print_variable
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            )
        },
        prefix as *mut ::core::ffi::c_void,
    );
    fputs(
        b"# variable set hash-table stats:\n\0" as *const u8 as *const ::core::ffi::c_char,
        stdout,
    );
    fputs(b"# \0" as *const u8 as *const ::core::ffi::c_char, stdout);
    hash_print_stats(&raw mut (*set).table, stdout);
    putc('\n' as i32, stdout);
}
#[no_mangle]
pub unsafe fn print_variable_data_base() {
    puts(b"\n# Variables\n\0" as *const u8 as *const ::core::ffi::c_char);
    print_variable_set(
        &raw mut global_variable_set,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        0,
    );
    puts(b"\n# Pattern-specific Variable Values\0" as *const u8 as *const ::core::ffi::c_char);
    let mut p: *mut pattern_var = ::core::ptr::null_mut::<pattern_var>();
    let mut rules: ::core::ffi::c_uint = 0;
    p = pattern_vars;
    while !p.is_null() {
        rules = rules.wrapping_add(1);
        printf(
            b"\n%s :\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).target,
        );
        print_variable(
            &raw mut (*p).variable as *const ::core::ffi::c_void,
            b"# \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
        p = (*p).next;
    }
    if rules == 0 {
        puts(
            b"\n# No pattern-specific variable values.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        printf(
            b"\n# %u pattern-specific variable values\0" as *const u8 as *const ::core::ffi::c_char,
            rules,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_file_variables(mut file: *const file) {
    if !(*file).variables.is_null() {
        print_variable_set(
            (*(*file).variables).set,
            b"# \0" as *const u8 as *const ::core::ffi::c_char,
            1,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_target_variables(mut file: *const file) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    if !(*file).variables.is_null() {
        let mut l: size_t = strlen((*file).name) as size_t;
        alloca_allocations.push(::std::vec::from_elem(
            0,
            l.wrapping_add(3) as usize,
        ));
        let mut t: *mut ::core::ffi::c_char =
            alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        memcpy(
            t as *mut ::core::ffi::c_void,
            (*file).name as *const ::core::ffi::c_void,
            l as size_t,
        );
        *t.offset(l as isize) = ':' as i32 as ::core::ffi::c_char;
        *t.offset(l.wrapping_add(1) as isize) = ' ' as i32 as ::core::ffi::c_char;
        *t.offset(l.wrapping_add(2) as isize) = 0;
        hash_map_arg(
            &raw mut (*(*(*file).variables).set).table,
            Some(
                print_noauto_variable
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            t as *mut ::core::ffi::c_void,
        );
    }
}
unsafe extern "C" fn run_static_initializers() {
    defined_vars = [
        defined_vars {
            name: b"MAKECMDGOALS\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t)
                .wrapping_sub(1),
        },
        defined_vars {
            name: b"MAKE_RESTARTS\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t)
                .wrapping_sub(1),
        },
        defined_vars {
            name: b"MAKE_TERMOUT\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t)
                .wrapping_sub(1),
        },
        defined_vars {
            name: b"MAKE_TERMERR\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t)
                .wrapping_sub(1),
        },
        defined_vars {
            name: b"MAKEOVERRIDES\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t)
                .wrapping_sub(1),
        },
        defined_vars {
            name: b".DEFAULT\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t)
                .wrapping_sub(1),
        },
        defined_vars {
            name: b"-*-command-variables-*-\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 24]>() as size_t)
                .wrapping_sub(1),
        },
        defined_vars {
            name: b"-*-eval-flags-*-\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 17]>() as size_t)
                .wrapping_sub(1),
        },
        defined_vars {
            name: b"VPATH\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t)
                .wrapping_sub(1),
        },
        defined_vars {
            name: b"GPATH\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t)
                .wrapping_sub(1),
        },
        defined_vars {
            name: b".WARNINGS\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t)
                .wrapping_sub(1),
        },
        defined_vars {
            name: b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t)
                .wrapping_sub(1),
        },
        defined_vars {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            len: 0,
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
