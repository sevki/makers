pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __pid_t, __syscall_slong_t, __time_t, __uid_t, pid_t, ptrdiff_t, size_t, ssize_t, uintmax_t,
};
use crate::file::{Commands, Dep, File, VariableSet, VariableSetList};
use crate::misc::{
    end_of_token, find_next_token, make_lltoa, next_token, xmalloc, xrealloc, xstrndup,
};
use crate::stdio::FILE;
use crate::strcache::strcache_add;
use c2rust_bitfields;
use libc::{
    __errno_location, abort, close, free, pipe, printf, realpath, remove, sprintf, strchr, strcmp,
    strcpy, strerror, strstr, strtoll,
};
extern "C" {
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
}

/// RAII owner for a string produced by `expand_argument`.
///
/// `expand_argument` hands back a freshly `malloc`ed, NUL-terminated buffer
/// that the caller must release. Wrapping it here lets `Drop` free the buffer
/// on every exit path, replacing the hand-written `expand_argument(...) ...
/// free(...)` ownership pairs the `$(if)`/`$(or)`/`$(and)` builtins used.
struct ExpandedArg(*mut ::core::ffi::c_char);

impl ExpandedArg {
    /// Expand `arg`, stopping at `end` (or at the NUL when `end` is null).
    unsafe fn new(arg: *const ::core::ffi::c_char, end: *const ::core::ffi::c_char) -> Self {
        ExpandedArg(expand_argument(arg, end))
    }

    /// Take ownership of an already-expanded, `malloc`ed buffer (e.g. from
    /// `allocated_expand_string_for_file`).
    unsafe fn from_raw(ptr: *mut ::core::ffi::c_char) -> Self {
        ExpandedArg(ptr)
    }

    /// Borrow the underlying NUL-terminated buffer.
    fn as_ptr(&self) -> *mut ::core::ffi::c_char {
        self.0
    }
}

impl Drop for ExpandedArg {
    fn drop(&mut self) {
        unsafe { free(self.0 as *mut ::core::ffi::c_void) }
    }
}

pub type gmk_func_ptr = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        ::core::ffi::c_uint,
        *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char,
>;
pub use crate::sys_stat::stat;
pub use crate::sys_stat::timespec;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
use crate::expand::{
    allocated_expand_string_for_file, expand_argument, expand_string_buf, expand_variable_output,
    expanding_var, install_variable_buffer, restore_variable_buffer, variable_buffer_output,
};
pub use crate::file::nameseq;
use crate::hash::{
    hash_find_item, hash_free, hash_init, hash_insert, hash_load, jhash, jhash_string,
};
pub use crate::job::childbase;
use crate::job::{child_execute_job, construct_command_argv, free_childbase, reap_children};
use crate::make_main::{command_count, db_level, starting_directory, stopchar_map};
use crate::misc::alpha_compare;
pub use crate::output::output;
use crate::output::{error, fatal, output_context, outputs};
use crate::posixos::fd_noinherit;
use crate::read::{eval_buffer, find_percent, parse_file_seq, reading_file};
use crate::variable::{
    current_variable_set_list, define_variable_in_set, lookup_variable, pop_variable_scope,
    push_new_variable_scope, target_environment, warn_undefined,
};
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct function_table_entry {
    pub fptr: C2RustUnnamed,
    pub name: *const ::core::ffi::c_char,
    pub len: ::core::ffi::c_uchar,
    pub minimum_args: ::core::ffi::c_uchar,
    pub maximum_args: ::core::ffi::c_uchar,
    #[bitfield(name = "expand_args", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "alloc_fn", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "adds_command", ty = "::core::ffi::c_uint", bits = "2..=2")]
    pub expand_args_alloc_fn_adds_command: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub func_ptr: Option<
        unsafe fn(
            *mut ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub alloc_func_ptr: gmk_func_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct a_word {
    pub chain: *mut a_word,
    pub str_0: *mut ::core::ffi::c_char,
    pub length: size_t,
    pub matched: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct a_pattern {
    pub str_0: *mut ::core::ffi::c_char,
    pub percent: *mut ::core::ffi::c_char,
    pub length: size_t,
}
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const ENOENT: ::core::ffi::c_int = 2;
pub const EINTR: ::core::ffi::c_int = 4;
pub const ERANGE: ::core::ffi::c_int = 34;
pub const PATH_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const GET_PATH_MAX: ::core::ffi::c_int = PATH_MAX;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAP_NUL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAP_DOT: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const MAP_DIRSEP: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const INTSTR_LENGTH: usize = (53 as usize)
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22 as usize)
    .wrapping_add(3 as usize);
pub const EXP_COUNT_BITS: ::core::ffi::c_int = 15;
pub const EXP_COUNT_MAX: ::core::ffi::c_int = ((1) << EXP_COUNT_BITS) - 1;
unsafe fn function_table_entry_hash_1(keyv: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let key: *const function_table_entry = keyv as *const function_table_entry;
    let mut _result_: ::core::ffi::c_ulong = 0;
    let mut _key_: *const ::core::ffi::c_uchar = (*key).name as *const ::core::ffi::c_uchar;
    _result_ = _result_
        .wrapping_add(jhash(_key_, (*key).len as ::core::ffi::c_int) as ::core::ffi::c_ulong);
    _result_
}
unsafe fn function_table_entry_hash_2(keyv: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _key: *const function_table_entry = keyv as *const function_table_entry;
    let mut _result_: ::core::ffi::c_ulong = 0;
    _result_
}
unsafe fn function_table_entry_hash_cmp(
    xv: *const ::core::ffi::c_void,
    yv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let x: *const function_table_entry = xv as *const function_table_entry;
    let y: *const function_table_entry = yv as *const function_table_entry;
    let result: ::core::ffi::c_int =
        (*x).len as ::core::ffi::c_int - (*y).len as ::core::ffi::c_int;
    if result != 0 {
        return result;
    }
    if (*x).name == (*y).name {
        0
    } else {
        memcmp(
            (*x).name as *const ::core::ffi::c_void,
            (*y).name as *const ::core::ffi::c_void,
            (*x).len as size_t,
        )
    }
}
static mut function_table: hash_table = hash_table {
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
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn subst_expand(
    mut o: *mut ::core::ffi::c_char,
    text: *const ::core::ffi::c_char,
    subst: *const ::core::ffi::c_char,
    replace: *const ::core::ffi::c_char,
    slen: size_t,
    rlen: size_t,
    by_word: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut t: *const ::core::ffi::c_char = text;
    let mut p: *const ::core::ffi::c_char;
    if slen == 0 && by_word == 0 {
        o = variable_buffer_output(o, t, strlen(t) as size_t);
        if rlen > 0 {
            o = variable_buffer_output(o, replace, rlen);
        }
        return o;
    }
    loop {
        if by_word != 0 && slen == 0 {
            p = end_of_token(next_token(t));
        } else {
            p = strstr(t, subst);
            if p.is_null() {
                o = variable_buffer_output(o, t, strlen(t) as size_t);
                return o;
            }
        }
        if p > t {
            o = variable_buffer_output(o, t, p.offset_from(t) as ::core::ffi::c_long as size_t);
        }
        if by_word != 0
            && (p > text
                && !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort).offset(
                    *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar as isize,
                ) as ::core::ffi::c_int
                    & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                    != 0)
                || !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                    .offset(*p.offset(slen as isize) as ::core::ffi::c_uchar as isize)
                    as ::core::ffi::c_int
                    & (0x2 as ::core::ffi::c_int
                        | 0x4 as ::core::ffi::c_int
                        | 0x1 as ::core::ffi::c_int)
                    != 0))
        {
            o = variable_buffer_output(o, subst, slen);
        } else if rlen > 0 {
            o = variable_buffer_output(o, replace, rlen);
        }
        t = p.offset(slen as isize);
        if !(*t as ::core::ffi::c_int != 0) {
            break;
        }
    }
    o
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn patsubst_expand_pat(
    mut o: *mut ::core::ffi::c_char,
    mut text: *const ::core::ffi::c_char,
    pattern: *const ::core::ffi::c_char,
    replace: *const ::core::ffi::c_char,
    pattern_percent: *const ::core::ffi::c_char,
    replace_percent: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let pattern_prepercent_len: size_t;
    let pattern_postpercent_len: size_t;
    let replace_prepercent_len: size_t;
    let replace_postpercent_len: size_t;
    let mut t: *const ::core::ffi::c_char;
    let mut len: size_t = 0;
    let mut doneany: ::core::ffi::c_int = 0;
    if !replace_percent.is_null() {
        replace_prepercent_len =
            (replace_percent.offset_from(replace) as ::core::ffi::c_long - 1) as size_t;
        replace_postpercent_len = strlen(replace_percent) as size_t;
    } else {
        replace_prepercent_len = strlen(replace) as size_t;
        replace_postpercent_len = 0;
    }
    if pattern_percent.is_null() {
        return subst_expand(
            o,
            text,
            pattern,
            replace,
            strlen(pattern) as size_t,
            strlen(replace) as size_t,
            1,
        );
    }
    pattern_prepercent_len =
        (pattern_percent.offset_from(pattern) as ::core::ffi::c_long - 1) as size_t;
    pattern_postpercent_len = strlen(pattern_percent) as size_t;
    loop {
        t = find_next_token(&raw mut text, &raw mut len);
        if t.is_null() {
            break;
        }
        let t0 = match t.as_ref() {
            Some(&b) => b,
            None => break,
        };
        let mut fail: ::core::ffi::c_int = 0;
        if len < pattern_prepercent_len.wrapping_add(pattern_postpercent_len) {
            fail = 1;
        }
        if fail == 0
            && pattern_prepercent_len > 0
            && (t0 as ::core::ffi::c_int != *pattern as ::core::ffi::c_int
                || *t.offset(pattern_prepercent_len.wrapping_sub(1) as isize) as ::core::ffi::c_int
                    != *pattern_percent.offset(-(2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                || !(strncmp(
                    t.offset(1 as ::core::ffi::c_int as isize),
                    pattern.offset(1 as ::core::ffi::c_int as isize),
                    (pattern_prepercent_len as size_t).wrapping_sub(1),
                ) == 0))
        {
            fail = 1;
        }
        if fail == 0
            && pattern_postpercent_len > 0
            && (*t.offset(len.wrapping_sub(1) as isize) as ::core::ffi::c_int
                != *pattern_percent.offset(pattern_postpercent_len.wrapping_sub(1) as isize)
                    as ::core::ffi::c_int
                || *t.offset(len.wrapping_sub(pattern_postpercent_len) as isize)
                    as ::core::ffi::c_int
                    != *pattern_percent as ::core::ffi::c_int
                || !(strncmp(
                    t.offset(len.wrapping_sub(pattern_postpercent_len) as isize)
                        as *const ::core::ffi::c_char,
                    pattern_percent,
                    (pattern_postpercent_len as size_t).wrapping_sub(1),
                ) == 0))
        {
            fail = 1;
        }
        if fail != 0 {
            o = variable_buffer_output(o, t, len);
        } else {
            o = variable_buffer_output(o, replace, replace_prepercent_len);
            if !replace_percent.is_null() {
                o = variable_buffer_output(
                    o,
                    t.offset(pattern_prepercent_len as isize),
                    len.wrapping_sub(pattern_prepercent_len.wrapping_add(pattern_postpercent_len)),
                );
                o = variable_buffer_output(o, replace_percent, replace_postpercent_len);
            }
        }
        if fail != 0
            || replace_prepercent_len > 0
            || !replace_percent.is_null() && len.wrapping_add(replace_postpercent_len) > 0
        {
            o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
            doneany = 1;
        }
    }
    if doneany != 0 {
        o = o.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    o
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn patsubst_expand(
    o: *mut ::core::ffi::c_char,
    text: *const ::core::ffi::c_char,
    pattern: *mut ::core::ffi::c_char,
    replace: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut pattern_percent: *const ::core::ffi::c_char = find_percent(pattern);
    let mut replace_percent: *const ::core::ffi::c_char = find_percent(replace);
    if !replace_percent.is_null() {
        replace_percent = replace_percent.offset(1 as ::core::ffi::c_int as isize);
    }
    if !pattern_percent.is_null() {
        pattern_percent = pattern_percent.offset(1 as ::core::ffi::c_int as isize);
    }
    patsubst_expand_pat(o, text, pattern, replace, pattern_percent, replace_percent)
}
unsafe extern "C" fn lookup_function(s: *const ::core::ffi::c_char) -> *const function_table_entry {
    let mut function_table_entry_key: function_table_entry = function_table_entry {
        fptr: C2RustUnnamed { func_ptr: None },
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        len: 0,
        minimum_args: 0,
        maximum_args: 0,
        expand_args_alloc_fn_adds_command: [0; 1],
        c2rust_padding: [0; 4],
    };
    let mut e: *const ::core::ffi::c_char = s;
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*e as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & 0x2000 as ::core::ffi::c_int
        != 0
    {
        e = e.offset(1 as ::core::ffi::c_int as isize);
    }
    if e == s
        || !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*e as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x1 as ::core::ffi::c_int | (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int))
            != 0)
    {
        return ::core::ptr::null::<function_table_entry>();
    }
    function_table_entry_key.name = s;
    function_table_entry_key.len = e.offset_from(s) as ::core::ffi::c_long as ::core::ffi::c_uchar;
    hash_find_item(
        &raw mut function_table,
        &raw mut function_table_entry_key as *const ::core::ffi::c_void,
    ) as *const function_table_entry
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn pattern_matches(
    mut pattern: *const ::core::ffi::c_char,
    mut percent: *const ::core::ffi::c_char,
    str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let sfxlen: size_t;
    let strlength: size_t;
    if percent.is_null() {
        let len: size_t = (strlen(pattern) as size_t).wrapping_add(1);
        alloca_allocations.push(::std::vec::from_elem(0, len as usize));
        let new_chars: *mut ::core::ffi::c_char =
            alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        memcpy(
            new_chars as *mut ::core::ffi::c_void,
            pattern as *const ::core::ffi::c_void,
            len as size_t,
        );
        percent = find_percent(new_chars);
        if percent.is_null() {
            return (*new_chars as ::core::ffi::c_int == *str as ::core::ffi::c_int
                && (*new_chars as ::core::ffi::c_int == 0
                    || strcmp(
                        new_chars.offset(1 as ::core::ffi::c_int as isize),
                        str.offset(1 as ::core::ffi::c_int as isize),
                    ) == 0)) as ::core::ffi::c_int;
        }
        pattern = new_chars;
    }
    sfxlen = strlen(percent.offset(1 as ::core::ffi::c_int as isize)) as size_t;
    strlength = strlen(str) as size_t;
    if strlength
        < (percent.offset_from(pattern) as ::core::ffi::c_long as size_t).wrapping_add(sfxlen)
        || !(strncmp(
            pattern,
            str,
            percent.offset_from(pattern) as ::core::ffi::c_long as size_t,
        ) == 0)
    {
        return 0;
    }
    (strcmp(
        percent.offset(1 as ::core::ffi::c_int as isize),
        str.offset(strlength.wrapping_sub(sfxlen) as isize),
    ) == 0) as ::core::ffi::c_int
}
unsafe extern "C" fn find_next_argument(
    startparen: ::core::ffi::c_char,
    endparen: ::core::ffi::c_char,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut count: ::core::ffi::c_int = 0;
    while ptr < end {
        if *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*ptr as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x80 as ::core::ffi::c_int | 0x400 as ::core::ffi::c_int)
            != 0
        {
            if *ptr as ::core::ffi::c_int == startparen as ::core::ffi::c_int {
                count += 1;
            } else if *ptr as ::core::ffi::c_int == endparen as ::core::ffi::c_int {
                count -= 1;
                if count < 0 {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            } else if *ptr as ::core::ffi::c_int == ',' as i32 && count == 0 {
                return ptr as *mut ::core::ffi::c_char;
            }
        }
        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
    }
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn string_glob(mut line: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    static mut result: *mut ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    static mut length: size_t = 0;
    let mut chain: *mut nameseq;
    let mut idx: size_t;
    chain = parse_file_seq(
        &raw mut line,
        ::core::mem::size_of::<nameseq>() as size_t,
        0x1 as ::core::ffi::c_int,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0x1 as ::core::ffi::c_int | 0x10 as ::core::ffi::c_int | 0x8 as ::core::ffi::c_int,
    ) as *mut nameseq;
    if result.is_null() {
        length = 100;
        result = xmalloc(100) as *mut ::core::ffi::c_char;
    }
    idx = 0;
    while !chain.is_null() {
        let next: *mut nameseq = (*chain).next;
        let len: size_t = strlen((*chain).name) as size_t;
        if idx.wrapping_add(len).wrapping_add(1) > length {
            length = length.wrapping_add(len.wrapping_add(1 as size_t).wrapping_mul(2));
            result =
                xrealloc(result as *mut ::core::ffi::c_void, length) as *mut ::core::ffi::c_char;
        }
        memcpy(
            result.offset(idx as isize) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            (*chain).name as *const ::core::ffi::c_void,
            len as size_t,
        );
        idx = idx.wrapping_add(len);
        let fresh2 = idx;
        idx = idx.wrapping_add(1);
        *result.offset(fresh2 as isize) = ' ' as i32 as ::core::ffi::c_char;
        free((*chain).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        free(chain as *mut ::core::ffi::c_void);
        chain = next;
    }
    if idx == 0 {
        *result.offset(0 as ::core::ffi::c_int as isize) = 0;
    } else {
        *result.offset(idx.wrapping_sub(1) as isize) = 0;
    }
    result
}
unsafe fn func_patsubst(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    o = patsubst_expand(
        o,
        *argv.offset(2 as ::core::ffi::c_int as isize),
        *argv.offset(0 as ::core::ffi::c_int as isize),
        *argv.offset(1 as ::core::ffi::c_int as isize),
    );
    o
}
unsafe fn func_join(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut doneany: ::core::ffi::c_int = 0;
    let mut tp: *const ::core::ffi::c_char;
    let mut pp: *const ::core::ffi::c_char;
    let mut list1_iterator: *const ::core::ffi::c_char =
        *argv.offset(0 as ::core::ffi::c_int as isize);
    let mut list2_iterator: *const ::core::ffi::c_char =
        *argv.offset(1 as ::core::ffi::c_int as isize);
    loop {
        let mut len1: size_t = 0;
        let mut len2: size_t = 0;
        tp = find_next_token(&raw mut list1_iterator, &raw mut len1);
        if !tp.is_null() {
            o = variable_buffer_output(o, tp, len1);
        }
        pp = find_next_token(&raw mut list2_iterator, &raw mut len2);
        if !pp.is_null() {
            o = variable_buffer_output(o, pp, len2);
        }
        if !tp.is_null() || !pp.is_null() {
            o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
            doneany = 1;
        }
        if !(!tp.is_null() || !pp.is_null()) {
            break;
        }
    }
    if doneany != 0 {
        o = o.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    o
}
unsafe fn func_origin(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let v: *mut variable = lookup_variable(
        *argv.offset(0 as ::core::ffi::c_int as isize),
        strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as size_t,
    );
    if v.is_null() {
        o = variable_buffer_output(
            o,
            b"undefined\0" as *const u8 as *const ::core::ffi::c_char,
            9,
        );
    } else {
        match (*v).origin() as ::core::ffi::c_int {
            7 => {
                abort();
            }
            0 => {
                o = variable_buffer_output(
                    o,
                    b"default\0" as *const u8 as *const ::core::ffi::c_char,
                    7,
                );
            }
            1 => {
                o = variable_buffer_output(
                    o,
                    b"environment\0" as *const u8 as *const ::core::ffi::c_char,
                    11,
                );
            }
            2 => {
                o = variable_buffer_output(
                    o,
                    b"file\0" as *const u8 as *const ::core::ffi::c_char,
                    4,
                );
            }
            3 => {
                o = variable_buffer_output(
                    o,
                    b"environment override\0" as *const u8 as *const ::core::ffi::c_char,
                    20,
                );
            }
            4 => {
                o = variable_buffer_output(
                    o,
                    b"command line\0" as *const u8 as *const ::core::ffi::c_char,
                    12,
                );
            }
            5 => {
                o = variable_buffer_output(
                    o,
                    b"override\0" as *const u8 as *const ::core::ffi::c_char,
                    8,
                );
            }
            6 => {
                o = variable_buffer_output(
                    o,
                    b"automatic\0" as *const u8 as *const ::core::ffi::c_char,
                    9,
                );
            }
            _ => {}
        }
    }
    o
}
unsafe fn func_flavor(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let v: *mut variable = lookup_variable(
        *argv.offset(0 as ::core::ffi::c_int as isize),
        strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as size_t,
    );
    if v.is_null() {
        o = variable_buffer_output(
            o,
            b"undefined\0" as *const u8 as *const ::core::ffi::c_char,
            9,
        );
    } else if (*v).recursive() != 0 {
        o = variable_buffer_output(
            o,
            b"recursive\0" as *const u8 as *const ::core::ffi::c_char,
            9,
        );
    } else {
        o = variable_buffer_output(o, b"simple\0" as *const u8 as *const ::core::ffi::c_char, 6);
    }
    o
}
unsafe fn func_notdir_suffix(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut list_iterator: *const ::core::ffi::c_char =
        *argv.offset(0 as ::core::ffi::c_int as isize);
    let mut p2: *const ::core::ffi::c_char;
    let mut doneany: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    let is_suffix: ::core::ffi::c_int = (*funcname.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 's' as i32) as ::core::ffi::c_int;
    let is_notdir: ::core::ffi::c_int = (is_suffix == 0) as ::core::ffi::c_int;
    let stop: ::core::ffi::c_int = MAP_DIRSEP | (if is_suffix != 0 { MAP_DOT } else { 0 });
    loop {
        p2 = find_next_token(&raw mut list_iterator, &raw mut len);
        if p2.is_null() {
            break;
        }
        let mut p: *const ::core::ffi::c_char = p2
            .offset(len as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        while p >= p2
            && !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                & stop
                != 0)
        {
            p = p.offset(-(1 as ::core::ffi::c_int) as isize);
        }
        if p >= p2 {
            if is_notdir != 0 {
                p = p.offset(1 as ::core::ffi::c_int as isize);
            } else if *p as ::core::ffi::c_int != '.' as i32 {
                continue;
            }
            o = variable_buffer_output(
                o,
                p,
                len.wrapping_sub(p.offset_from(p2) as ::core::ffi::c_long as size_t),
            );
        } else if is_notdir != 0 {
            o = variable_buffer_output(o, p2, len);
        }
        if is_notdir != 0 || p >= p2 {
            o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
            doneany = 1;
        }
    }
    if doneany != 0 {
        o = o.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    o
}
unsafe fn func_basename_dir(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut p3: *const ::core::ffi::c_char = *argv.offset(0 as ::core::ffi::c_int as isize);
    let mut p2: *const ::core::ffi::c_char;
    let mut doneany: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    let is_basename: ::core::ffi::c_int = (*funcname.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'b' as i32) as ::core::ffi::c_int;
    let is_dir: ::core::ffi::c_int = (is_basename == 0) as ::core::ffi::c_int;
    let stop: ::core::ffi::c_int =
        MAP_DIRSEP | (if is_basename != 0 { MAP_DOT } else { 0 }) | MAP_NUL;
    loop {
        p2 = find_next_token(&raw mut p3, &raw mut len);
        if p2.is_null() {
            break;
        }
        let mut p: *const ::core::ffi::c_char = p2
            .offset(len as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        while p >= p2
            && !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                & stop
                != 0)
        {
            p = p.offset(-(1 as ::core::ffi::c_int) as isize);
        }
        if p >= p2 && is_dir != 0 {
            p = p.offset(1 as ::core::ffi::c_int as isize);
            o = variable_buffer_output(o, p2, p.offset_from(p2) as ::core::ffi::c_long as size_t);
        } else if p >= p2 && *p as ::core::ffi::c_int == '.' as i32 {
            o = variable_buffer_output(o, p2, p.offset_from(p2) as ::core::ffi::c_long as size_t);
        } else if is_dir != 0 {
            o = variable_buffer_output(o, b"./\0" as *const u8 as *const ::core::ffi::c_char, 2);
        } else {
            o = variable_buffer_output(o, p2, len);
        }
        o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
        doneany = 1;
    }
    if doneany != 0 {
        o = o.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    o
}
unsafe fn func_addsuffix_addprefix(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let fixlen: size_t = strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as size_t;
    let mut list_iterator: *const ::core::ffi::c_char =
        *argv.offset(1 as ::core::ffi::c_int as isize);
    let is_addprefix: ::core::ffi::c_int = (*funcname.offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'p' as i32) as ::core::ffi::c_int;
    let is_addsuffix: ::core::ffi::c_int = (is_addprefix == 0) as ::core::ffi::c_int;
    let mut doneany: ::core::ffi::c_int = 0;
    let mut p: *const ::core::ffi::c_char;
    let mut len: size_t = 0;
    loop {
        p = find_next_token(&raw mut list_iterator, &raw mut len);
        if p.is_null() {
            break;
        }
        if is_addprefix != 0 {
            o = variable_buffer_output(o, *argv.offset(0 as ::core::ffi::c_int as isize), fixlen);
        }
        o = variable_buffer_output(o, p, len);
        if is_addsuffix != 0 {
            o = variable_buffer_output(o, *argv.offset(0 as ::core::ffi::c_int as isize), fixlen);
        }
        o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
        doneany = 1;
    }
    if doneany != 0 {
        o = o.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    o
}
unsafe fn func_subst(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    o = subst_expand(
        o,
        *argv.offset(2 as ::core::ffi::c_int as isize),
        *argv.offset(0 as ::core::ffi::c_int as isize),
        *argv.offset(1 as ::core::ffi::c_int as isize),
        strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as size_t,
        strlen(*argv.offset(1 as ::core::ffi::c_int as isize)) as size_t,
        0,
    );
    o
}
unsafe fn func_firstword(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut i: size_t = 0;
    let mut words: *const ::core::ffi::c_char = *argv.offset(0 as ::core::ffi::c_int as isize);
    let p: *const ::core::ffi::c_char = find_next_token(&raw mut words, &raw mut i);
    if !p.is_null() {
        o = variable_buffer_output(o, p, i);
    }
    o
}
unsafe fn func_lastword(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut i: size_t = 0;
    let mut words: *const ::core::ffi::c_char = *argv.offset(0 as ::core::ffi::c_int as isize);
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut t: *const ::core::ffi::c_char;
    loop {
        t = find_next_token(&raw mut words, &raw mut i);
        if t.is_null() {
            break;
        }
        p = t;
    }
    if !p.is_null() {
        o = variable_buffer_output(o, p, i);
    }
    o
}
unsafe fn func_words(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_uint = 0;
    let mut word_iterator: *const ::core::ffi::c_char =
        *argv.offset(0 as ::core::ffi::c_int as isize);
    let mut buf: [::core::ffi::c_char; 22] = [0; 22];
    while !find_next_token(&raw mut word_iterator, ::core::ptr::null_mut::<size_t>()).is_null() {
        i = i.wrapping_add(1);
    }
    o = variable_buffer_output(
        o,
        &raw mut buf as *mut ::core::ffi::c_char,
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%u\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        ) as size_t,
    );
    o
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn strip_whitespace(
    begpp: *mut *const ::core::ffi::c_char,
    endpp: *mut *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    while *begpp <= *endpp
        && *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(**begpp as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
            != 0
    {
        *begpp = (*begpp).offset(1 as ::core::ffi::c_int as isize);
    }
    while *endpp >= *begpp
        && *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(**endpp as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
            != 0
    {
        *endpp = (*endpp).offset(-(1 as ::core::ffi::c_int) as isize);
    }
    *begpp as *mut ::core::ffi::c_char
}
unsafe extern "C" fn parse_numeric(
    s: *const ::core::ffi::c_char,
    msg: *const ::core::ffi::c_char,
) -> ::core::ffi::c_longlong {
    let mut beg: *const ::core::ffi::c_char = s;
    let mut end: *const ::core::ffi::c_char = s
        .offset(strlen(s) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut endp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let num: ::core::ffi::c_longlong;
    strip_whitespace(&raw mut beg, &raw mut end);
    if beg > end {
        fatal(
            *expanding_var,
            strlen(msg) as size_t,
            b"%s: empty value\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        );
    }
    *__errno_location() = 0;
    num = strtoll(beg, &raw mut endp, 10);
    if *__errno_location() == ERANGE {
        fatal(
            *expanding_var,
            (strlen(msg) as size_t).wrapping_add(strlen(s) as size_t),
            b"%s: '%s' out of range\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
            s,
        );
    } else if endp == beg as *mut ::core::ffi::c_char || endp <= end as *mut ::core::ffi::c_char {
        fatal(
            *expanding_var,
            (strlen(msg) as size_t).wrapping_add(strlen(s) as size_t),
            b"%s: '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
            s,
        );
    }
    num
}
unsafe fn func_word(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut end_p: *const ::core::ffi::c_char;
    let mut p: *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_longlong;
    i = parse_numeric(
        *argv.offset(0 as ::core::ffi::c_int as isize),
        b"invalid first argument to 'word' function\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if i < 1 as ::core::ffi::c_longlong {
        fatal(
            *expanding_var,
            0,
            b"first argument to 'word' function must be greater than 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    end_p = *argv.offset(1 as ::core::ffi::c_int as isize);
    loop {
        p = find_next_token(&raw mut end_p, ::core::ptr::null_mut::<size_t>());
        if p.is_null() {
            break;
        }
        i -= 1;
        if i == 0 as ::core::ffi::c_longlong {
            break;
        }
    }
    if i == 0 as ::core::ffi::c_longlong {
        o = variable_buffer_output(o, p, end_p.offset_from(p) as ::core::ffi::c_long as size_t);
    }
    o
}
unsafe fn func_wordlist(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut buf: [::core::ffi::c_char; 23] = [0; 23];
    let mut start: ::core::ffi::c_longlong;
    let stop: ::core::ffi::c_longlong;
    let mut count: ::core::ffi::c_longlong;
    let badfirst: *const ::core::ffi::c_char = b"invalid first argument to 'wordlist' function\0"
        as *const u8 as *const ::core::ffi::c_char;
    let badsecond: *const ::core::ffi::c_char = b"invalid second argument to 'wordlist' function\0"
        as *const u8 as *const ::core::ffi::c_char;
    start = parse_numeric(*argv.offset(0 as ::core::ffi::c_int as isize), badfirst);
    if start < 1 as ::core::ffi::c_longlong {
        fatal(
            *expanding_var,
            (strlen(badfirst) as size_t)
                .wrapping_add(
                    strlen(make_lltoa(start, &raw mut buf as *mut ::core::ffi::c_char)) as size_t,
                ),
            b"%s: '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            badfirst,
            make_lltoa(start, &raw mut buf as *mut ::core::ffi::c_char),
        );
    }
    stop = parse_numeric(*argv.offset(1 as ::core::ffi::c_int as isize), badsecond);
    if stop < 0 as ::core::ffi::c_longlong {
        fatal(
            *expanding_var,
            (strlen(badsecond) as size_t)
                .wrapping_add(
                    strlen(make_lltoa(stop, &raw mut buf as *mut ::core::ffi::c_char)) as size_t,
                ),
            b"%s: '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            badsecond,
            make_lltoa(stop, &raw mut buf as *mut ::core::ffi::c_char),
        );
    }
    count = stop - start + 1 as ::core::ffi::c_longlong;
    if count > 0 as ::core::ffi::c_longlong {
        let mut p: *const ::core::ffi::c_char;
        let mut end_p: *const ::core::ffi::c_char = *argv.offset(2 as ::core::ffi::c_int as isize);
        loop {
            p = find_next_token(&raw mut end_p, ::core::ptr::null_mut::<size_t>());
            if !(!p.is_null() && {
                start -= 1;
                start != 0
            }) {
                break;
            }
        }
        if !p.is_null() {
            loop {
                count -= 1;
                if !(count != 0
                    && !find_next_token(&raw mut end_p, ::core::ptr::null_mut::<size_t>())
                        .is_null())
                {
                    break;
                }
            }
            o = variable_buffer_output(o, p, end_p.offset_from(p) as ::core::ffi::c_long as size_t);
        }
    }
    o
}
unsafe fn func_findstring(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if !strstr(
        *argv.offset(1 as ::core::ffi::c_int as isize),
        *argv.offset(0 as ::core::ffi::c_int as isize),
    )
    .is_null()
    {
        o = variable_buffer_output(
            o,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as size_t,
        );
    }
    o
}
unsafe fn func_foreach(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let varname = ExpandedArg::new(
        *argv.offset(0 as ::core::ffi::c_int as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let list = ExpandedArg::new(
        *argv.offset(1 as ::core::ffi::c_int as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let body: *const ::core::ffi::c_char = *argv.offset(2 as ::core::ffi::c_int as isize);
    let mut doneany: ::core::ffi::c_int = 0;
    let mut list_iterator: *const ::core::ffi::c_char = list.as_ptr();
    let mut p: *const ::core::ffi::c_char;
    let mut len: size_t = 0;
    let var: *mut variable;
    let vp: *mut ::core::ffi::c_char = next_token(varname.as_ptr());
    *end_of_token(vp).offset(0 as ::core::ffi::c_int as isize) = 0;
    push_new_variable_scope();
    var = define_variable_in_set(
        vp,
        strlen(vp) as size_t,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    loop {
        p = find_next_token(&raw mut list_iterator, &raw mut len);
        if p.is_null() {
            break;
        }
        free((*var).value as *mut ::core::ffi::c_void);
        (*var).value = xstrndup(p, len);
        let result = ExpandedArg::from_raw(allocated_expand_string_for_file(
            body,
            ::core::ptr::null_mut::<file>(),
        ));
        o = variable_buffer_output(o, result.as_ptr(), strlen(result.as_ptr()) as size_t);
        o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
        doneany = 1;
    }
    if doneany != 0 {
        o = o.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    pop_variable_scope();
    o
}
unsafe fn func_let(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let varnames = ExpandedArg::new(
        *argv.offset(0 as ::core::ffi::c_int as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let list = ExpandedArg::new(
        *argv.offset(1 as ::core::ffi::c_int as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let body: *const ::core::ffi::c_char = *argv.offset(2 as ::core::ffi::c_int as isize);
    let mut vp: *const ::core::ffi::c_char;
    let mut vp_next: *const ::core::ffi::c_char = varnames.as_ptr();
    let mut list_iterator: *const ::core::ffi::c_char = list.as_ptr();
    let mut vlen: size_t = 0;
    push_new_variable_scope();
    vp = find_next_token(&raw mut vp_next, &raw mut vlen);
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*vp_next as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        != 0
    {
        vp_next = vp_next.offset(1 as ::core::ffi::c_int as isize);
    }
    while *vp_next as ::core::ffi::c_int != 0 {
        let mut len: size_t = 0;
        let p: *mut ::core::ffi::c_char = find_next_token(&raw mut list_iterator, &raw mut len);
        if !p.is_null() && *list_iterator as ::core::ffi::c_int != 0 {
            list_iterator = list_iterator.offset(1 as ::core::ffi::c_int as isize);
            *p.offset(len as isize) = 0;
        }
        define_variable_in_set(
            vp,
            vlen,
            if !p.is_null() {
                p as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            o_automatic,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
        vp = find_next_token(&raw mut vp_next, &raw mut vlen);
        while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*vp_next as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
            != 0
        {
            vp_next = vp_next.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    if !vp.is_null() {
        define_variable_in_set(
            vp,
            vlen,
            next_token(list_iterator),
            o_automatic,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
    }
    o = expand_string_buf(o, body, SIZE_MAX as size_t);
    pop_variable_scope();
    o.offset(strlen(o) as isize)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn a_word_hash_1(key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0;
    let mut _key_: *const ::core::ffi::c_uchar =
        (*(key as *const a_word)).str_0 as *const ::core::ffi::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as ::core::ffi::c_ulong);
    _result_
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn a_word_hash_2(mut _key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0;
    _result_
}
unsafe fn a_word_hash_cmp(
    x: *const ::core::ffi::c_void,
    y: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let ax: *const a_word = x as *const a_word;
    let ay: *const a_word = y as *const a_word;
    if (*ax).length != (*ay).length {
        return if (*ax).length > (*ay).length {
            1
        } else {
            -(1 as ::core::ffi::c_int)
        };
    }
    if (*ax).str_0 == (*ay).str_0 {
        0
    } else {
        memcmp(
            (*ax).str_0 as *const ::core::ffi::c_void,
            (*ay).str_0 as *const ::core::ffi::c_void,
            (*ax).length as size_t,
        )
    }
}
unsafe fn func_filter_filterout(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let words: *mut a_word;
    let word_end: *mut a_word;
    let mut wp: *mut a_word;
    let patterns: *mut a_pattern;
    let pat_end: *mut a_pattern;
    let mut pp: *mut a_pattern;
    let mut pat_count: ::core::ffi::c_ulong = 0;
    let mut word_count: ::core::ffi::c_ulong = 0;
    let mut a_word_table: hash_table = hash_table {
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
    let is_filter: ::core::ffi::c_int = (*funcname.offset(
        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize).wrapping_sub(1 as usize)
            as isize,
    ) as ::core::ffi::c_int
        == 0) as ::core::ffi::c_int;
    let mut cp: *const ::core::ffi::c_char;
    let mut literals: ::core::ffi::c_int = 0;
    let hashing: ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_char;
    let mut len: size_t = 0;
    let mut doneany: ::core::ffi::c_int = 0;
    cp = *argv.offset(1 as ::core::ffi::c_int as isize);
    loop {
        p = find_next_token(&raw mut cp, ::core::ptr::null_mut::<size_t>());
        if p.is_null() {
            break;
        }
        word_count = word_count.wrapping_add(1);
    }
    if word_count == 0 {
        return o;
    }
    // Owned, zero-initialized word table (was an xcalloc'd array freed at
    // the end). Fixed capacity keeps the backing pointer stable while the
    // hash table below stores pointers into it.
    let mut words_vec: Vec<a_word> = Vec::with_capacity(word_count as usize);
    words_vec.resize_with(word_count as usize, || unsafe { ::core::mem::zeroed() });
    words = words_vec.as_mut_ptr();
    word_end = words.offset(word_count as isize);
    cp = *argv.offset(0 as ::core::ffi::c_int as isize);
    loop {
        p = find_next_token(&raw mut cp, ::core::ptr::null_mut::<size_t>());
        if p.is_null() {
            break;
        }
        pat_count = pat_count.wrapping_add(1);
    }
    let mut patterns_vec: Vec<a_pattern> = Vec::with_capacity(pat_count as usize);
    patterns_vec.resize_with(pat_count as usize, || unsafe { ::core::mem::zeroed() });
    patterns = patterns_vec.as_mut_ptr();
    pat_end = patterns.offset(pat_count as isize);
    cp = *argv.offset(0 as ::core::ffi::c_int as isize);
    pp = patterns;
    loop {
        p = find_next_token(&raw mut cp, &raw mut len);
        if p.is_null() {
            break;
        }
        if *cp as ::core::ffi::c_int != 0 {
            cp = cp.offset(1 as ::core::ffi::c_int as isize);
        }
        *p.offset(len as isize) = 0;
        (*pp).str_0 = p;
        (*pp).percent = find_percent(p);
        if (*pp).percent.is_null() {
            literals += 1;
        }
        (*pp).length = strlen((*pp).str_0) as size_t;
        pp = pp.offset(1 as ::core::ffi::c_int as isize);
    }
    cp = *argv.offset(1 as ::core::ffi::c_int as isize);
    wp = words;
    loop {
        p = find_next_token(&raw mut cp, &raw mut len);
        if p.is_null() {
            break;
        }
        if *cp as ::core::ffi::c_int != 0 {
            cp = cp.offset(1 as ::core::ffi::c_int as isize);
        }
        *p.offset(len as isize) = 0;
        (*wp).str_0 = p;
        (*wp).length = len;
        wp = wp.offset(1 as ::core::ffi::c_int as isize);
    }
    hashing = (literals > 1 && (literals as ::core::ffi::c_ulong).wrapping_mul(word_count) >= 10)
        as ::core::ffi::c_int;
    if hashing != 0 {
        hash_init(
            &raw mut a_word_table,
            word_count,
            Some(a_word_hash_1),
            Some(a_word_hash_2),
            Some(a_word_hash_cmp),
        );
        wp = words;
        while wp < word_end {
            let owp: *mut a_word =
                hash_insert(&raw mut a_word_table, wp as *const ::core::ffi::c_void) as *mut a_word;
            if !owp.is_null() {
                (*wp).chain = owp;
            }
            wp = wp.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    pp = patterns;
    while pp < pat_end {
        if !(*pp).percent.is_null() {
            wp = words;
            while wp < word_end {
                (*wp).matched |= pattern_matches((*pp).str_0, (*pp).percent, (*wp).str_0);
                wp = wp.offset(1 as ::core::ffi::c_int as isize);
            }
        } else if hashing != 0 {
            let mut a_word_key: a_word = a_word {
                chain: ::core::ptr::null_mut::<a_word>(),
                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                length: 0,
                matched: 0,
            };
            a_word_key.str_0 = (*pp).str_0;
            a_word_key.length = (*pp).length;
            wp = hash_find_item(
                &raw mut a_word_table,
                &raw mut a_word_key as *const ::core::ffi::c_void,
            ) as *mut a_word;
            while !wp.is_null() {
                (*wp).matched |= 1;
                wp = (*wp).chain;
            }
        } else {
            wp = words;
            while wp < word_end {
                (*wp).matched |= ((*wp).length == (*pp).length
                    && memcmp(
                        (*pp).str_0 as *const ::core::ffi::c_void,
                        (*wp).str_0 as *const ::core::ffi::c_void,
                        (*wp).length as size_t,
                    ) == 0) as ::core::ffi::c_int;
                wp = wp.offset(1 as ::core::ffi::c_int as isize);
            }
        }
        pp = pp.offset(1 as ::core::ffi::c_int as isize);
    }
    wp = words;
    while wp < word_end {
        if if is_filter != 0 {
            (*wp).matched
        } else {
            ((*wp).matched == 0) as ::core::ffi::c_int
        } != 0
        {
            o = variable_buffer_output(o, (*wp).str_0, strlen((*wp).str_0) as size_t);
            o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
            doneany = 1;
        }
        wp = wp.offset(1 as ::core::ffi::c_int as isize);
    }
    if doneany != 0 {
        o = o.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    if hashing != 0 {
        hash_free(&raw mut a_word_table, 0);
    }
    o
}
unsafe fn func_strip(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char = *argv.offset(0 as ::core::ffi::c_int as isize);
    let mut doneany: ::core::ffi::c_int = 0;
    while *p as ::core::ffi::c_int != 0 {
        let mut i: ::core::ffi::c_int;
        let word_start: *const ::core::ffi::c_char;
        while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
            != 0
        {
            p = p.offset(1 as ::core::ffi::c_int as isize);
        }
        word_start = p;
        i = 0;
        while *p as ::core::ffi::c_int != 0
            && !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                != 0)
        {
            p = p.offset(1 as ::core::ffi::c_int as isize);
            i += 1;
        }
        if i == 0 {
            break;
        }
        o = variable_buffer_output(o, word_start, i as size_t);
        o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
        doneany = 1;
    }
    if doneany != 0 {
        o = o.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    o
}
unsafe fn func_error(
    o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    match *funcname as ::core::ffi::c_int {
        101 => {
            fatal(
                reading_file,
                strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
        }
        119 => {
            error(
                reading_file,
                strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
        }
        105 => {
            // $(info ...): build "<arg>\n\0" in an owned buffer instead of a
            // malloc/memcpy/free sequence.
            let src = *argv.offset(0 as ::core::ffi::c_int as isize);
            let len = strlen(src) as usize;
            let mut msg = Vec::<u8>::with_capacity(len + 2);
            msg.extend_from_slice(::core::slice::from_raw_parts(src as *const u8, len));
            msg.push(b'\n');
            msg.push(0);
            outputs(0, msg.as_ptr() as *const ::core::ffi::c_char);
        }
        _ => {
            fatal(
                *expanding_var,
                strlen(funcname) as size_t,
                b"INTERNAL: func_error: '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                funcname,
            );
        }
    }
    o
}
unsafe fn func_sort(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut t: *const ::core::ffi::c_char;
    let mut wordi: ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_char;
    let mut len: size_t = 0;
    t = *argv.offset(0 as ::core::ffi::c_int as isize);
    wordi = 0;
    loop {
        p = find_next_token(&raw mut t, ::core::ptr::null_mut::<size_t>());
        if p.is_null() {
            break;
        }
        t = t.offset(1 as ::core::ffi::c_int as isize);
        wordi += 1;
    }
    // Owned word-pointer table (was an xmalloc'd array freed at the end).
    let mut words: Vec<*mut ::core::ffi::c_char> =
        Vec::with_capacity(if wordi == 0 { 1 } else { wordi as usize });
    t = *argv.offset(0 as ::core::ffi::c_int as isize);
    wordi = 0;
    loop {
        p = find_next_token(&raw mut t, &raw mut len);
        if p.is_null() {
            break;
        }
        t = t.offset(1 as ::core::ffi::c_int as isize);
        *p.offset(len as isize) = 0;
        wordi += 1;
        words.push(p);
    }
    if wordi != 0 {
        let mut i: ::core::ffi::c_int;
        qsort(
            words.as_mut_ptr() as *mut ::core::ffi::c_void,
            wordi as size_t,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
            Some(
                alpha_compare
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        i = 0;
        while i < wordi {
            len = strlen(words[i as usize]) as size_t;
            if i == wordi - 1
                || strlen(words[(i + 1) as usize]) != len
                || memcmp(
                    words[i as usize] as *const ::core::ffi::c_void,
                    words[(i + 1) as usize] as *const ::core::ffi::c_void,
                    len as size_t,
                ) != 0
            {
                o = variable_buffer_output(o, words[i as usize], len);
                o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
            }
            i += 1;
        }
        o = o.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    o
}
unsafe extern "C" fn parse_textint(
    number: *const ::core::ffi::c_char,
    msg: *const ::core::ffi::c_char,
    sign: *mut ::core::ffi::c_int,
    numstart: *mut *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let after_sign: *const ::core::ffi::c_char;
    let after_number: *const ::core::ffi::c_char;
    let mut p: *const ::core::ffi::c_char = next_token(number);
    let negative: ::core::ffi::c_int =
        (*p as ::core::ffi::c_int == '-' as i32) as ::core::ffi::c_int;
    let nonzero: ::core::ffi::c_int;
    if *p as ::core::ffi::c_int == 0 {
        fatal(
            *expanding_var,
            strlen(msg) as size_t,
            b"%s: empty value\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        );
    }
    p = p.offset(
        (negative != 0 || *p as ::core::ffi::c_int == '+' as i32) as ::core::ffi::c_int as isize,
    );
    after_sign = p;
    while *p as ::core::ffi::c_int == '0' as i32 {
        p = p.offset(1 as ::core::ffi::c_int as isize);
    }
    *numstart = p;
    while (*p as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint) <= 9 {
        p = p.offset(1 as ::core::ffi::c_int as isize);
    }
    after_number = p;
    nonzero = (*numstart != after_number) as ::core::ffi::c_int;
    *sign = if negative != 0 { -nonzero } else { nonzero };
    if after_number == after_sign || *next_token(p) as ::core::ffi::c_int != 0 {
        fatal(
            *expanding_var,
            (strlen(msg) as size_t).wrapping_add(strlen(number) as size_t),
            b"%s: '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
            number,
        );
    }
    after_number
}
unsafe fn func_intcmp(
    mut o: *mut ::core::ffi::c_char,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut lsign: ::core::ffi::c_int = 0;
    let mut rsign: ::core::ffi::c_int = 0;
    let mut lnum: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut rnum: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let lhs_str = ExpandedArg::new(
        *argv.offset(0 as ::core::ffi::c_int as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let rhs_str = ExpandedArg::new(
        *argv.offset(1 as ::core::ffi::c_int as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let llim: *const ::core::ffi::c_char = parse_textint(
        lhs_str.as_ptr(),
        b"non-numeric first argument to 'intcmp' function\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut lsign,
        &raw mut lnum,
    );
    let rlim: *const ::core::ffi::c_char = parse_textint(
        rhs_str.as_ptr(),
        b"non-numeric second argument to 'intcmp' function\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut rsign,
        &raw mut rnum,
    );
    let llen: ptrdiff_t = llim.offset_from(lnum) as ptrdiff_t;
    let rlen: ptrdiff_t = rlim.offset_from(rnum) as ptrdiff_t;
    let mut cmp: ::core::ffi::c_int = lsign - rsign;
    if cmp == 0 {
        cmp = (llen > rlen) as ::core::ffi::c_int - (llen < rlen) as ::core::ffi::c_int;
        if cmp == 0 {
            cmp = memcmp(
                lnum as *const ::core::ffi::c_void,
                rnum as *const ::core::ffi::c_void,
                llen as size_t,
            );
        }
        if lsign < 0 {
            cmp *= -(1 as ::core::ffi::c_int);
        }
    }
    argv = argv.offset(2 as ::core::ffi::c_int as isize);
    if (*argv).is_null() && cmp == 0 {
        if lsign < 0 {
            o = variable_buffer_output(o, b"-\0" as *const u8 as *const ::core::ffi::c_char, 1);
        }
        o = variable_buffer_output(
            o,
            lnum.offset(-((lsign == 0) as ::core::ffi::c_int as isize)),
            (llen + (lsign == 0) as ::core::ffi::c_int as ptrdiff_t) as size_t,
        );
    }
    if !(*argv).is_null() && cmp >= 0 {
        argv = argv.offset(1 as ::core::ffi::c_int as isize);
        if cmp > 0
            && !(*argv).is_null()
            && !(*argv.offset(1 as ::core::ffi::c_int as isize)).is_null()
        {
            argv = argv.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    if !(*argv).is_null() {
        let expansion = ExpandedArg::new(*argv, ::core::ptr::null::<::core::ffi::c_char>());
        o = variable_buffer_output(o, expansion.as_ptr(), strlen(expansion.as_ptr()) as size_t);
    }
    o
}
unsafe fn func_if(
    mut o: *mut ::core::ffi::c_char,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut begp: *const ::core::ffi::c_char = *argv.offset(0 as ::core::ffi::c_int as isize);
    let mut endp: *const ::core::ffi::c_char = begp
        .offset(strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut result: ::core::ffi::c_int = 0;
    strip_whitespace(&raw mut begp, &raw mut endp);
    if begp <= endp {
        let expansion = ExpandedArg::new(begp, endp.offset(1 as ::core::ffi::c_int as isize));
        result = (*expansion.as_ptr().offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            != 0) as ::core::ffi::c_int;
    }
    argv = argv.offset((1 + (result == 0) as ::core::ffi::c_int) as isize);
    if !(*argv).is_null() {
        let expansion = ExpandedArg::new(*argv, ::core::ptr::null::<::core::ffi::c_char>());
        o = variable_buffer_output(o, expansion.as_ptr(), strlen(expansion.as_ptr()) as size_t);
    }
    o
}
unsafe fn func_or(
    mut o: *mut ::core::ffi::c_char,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    while !(*argv).is_null() {
        let mut begp: *const ::core::ffi::c_char = *argv;
        let mut endp: *const ::core::ffi::c_char = begp
            .offset(strlen(*argv) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        strip_whitespace(&raw mut begp, &raw mut endp);
        if !(begp > endp) {
            let expansion = ExpandedArg::new(begp, endp.offset(1 as ::core::ffi::c_int as isize));
            let result = strlen(expansion.as_ptr()) as size_t;
            if result != 0 {
                o = variable_buffer_output(o, expansion.as_ptr(), result);
                break;
            }
        }
        argv = argv.offset(1 as ::core::ffi::c_int as isize);
    }
    o
}
unsafe fn func_and(
    mut o: *mut ::core::ffi::c_char,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    loop {
        let mut begp: *const ::core::ffi::c_char = *argv;
        let mut endp: *const ::core::ffi::c_char = begp
            .offset(strlen(*argv) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        strip_whitespace(&raw mut begp, &raw mut endp);
        if begp > endp {
            return o;
        }
        let expansion = ExpandedArg::new(begp, endp.offset(1 as ::core::ffi::c_int as isize));
        let result = strlen(expansion.as_ptr()) as size_t;
        if result == 0 {
            break;
        }
        argv = argv.offset(1 as ::core::ffi::c_int as isize);
        if (*argv).is_null() {
            o = variable_buffer_output(o, expansion.as_ptr(), result);
            break;
        }
        // More arguments remain: drop this expansion and evaluate the next.
    }
    o
}
unsafe fn func_wildcard(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let p: *mut ::core::ffi::c_char = string_glob(*argv.offset(0 as ::core::ffi::c_int as isize));
    o = variable_buffer_output(o, p, strlen(p) as size_t);
    o
}
unsafe fn func_eval(
    o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    install_variable_buffer(&raw mut buf, &raw mut len);
    eval_buffer(
        *argv.offset(0 as ::core::ffi::c_int as isize),
        ::core::ptr::null::<Floc>(),
    );
    restore_variable_buffer(buf, len);
    o
}
unsafe fn func_value(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let v: *mut variable = lookup_variable(
        *argv.offset(0 as ::core::ffi::c_int as isize),
        strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as size_t,
    );
    if !v.is_null() {
        o = variable_buffer_output(o, (*v).value, strlen((*v).value) as size_t);
    }
    o
}
unsafe extern "C" fn fold_newlines(
    buffer: *mut ::core::ffi::c_char,
    length: *mut size_t,
    trim_newlines: ::core::ffi::c_int,
) {
    let mut dst: *mut ::core::ffi::c_char = buffer;
    let mut src: *mut ::core::ffi::c_char = buffer;
    let mut last_nonnl: *mut ::core::ffi::c_char =
        buffer.offset(-(1 as ::core::ffi::c_int as isize));
    *src.offset(*length as isize) = 0;
    while *src as ::core::ffi::c_int != 0 {
        if !(*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\r' as i32
            && *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32)
        {
            if *src as ::core::ffi::c_int == '\n' as i32 {
                let fresh0 = dst;
                dst = dst.offset(1 as ::core::ffi::c_int as isize);
                *fresh0 = ' ' as i32 as ::core::ffi::c_char;
            } else {
                last_nonnl = dst;
                let fresh1 = dst;
                dst = dst.offset(1 as ::core::ffi::c_int as isize);
                *fresh1 = *src;
            }
        }
        src = src.offset(1 as ::core::ffi::c_int as isize);
    }
    if trim_newlines == 0 && last_nonnl < dst.offset(-(2 as ::core::ffi::c_int as isize)) {
        last_nonnl = dst.offset(-(2 as ::core::ffi::c_int as isize));
    }
    last_nonnl = last_nonnl.offset(1 as ::core::ffi::c_int as isize);
    *last_nonnl = 0;
    *length = last_nonnl.offset_from(buffer) as ::core::ffi::c_long as size_t;
}
pub static mut shell_function_pid: pid_t = 0 as pid_t;
static mut shell_function_completed: ::core::ffi::c_int = 0;
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn shell_completed(mut exit_code: ::core::ffi::c_int, exit_sig: ::core::ffi::c_int) {
    let mut buf: [::core::ffi::c_char; 22] = [0; 22];
    shell_function_pid = 0 as ::core::ffi::c_int as pid_t;
    if exit_sig == 0 && exit_code == 127 {
        shell_function_completed = -(1 as ::core::ffi::c_int);
    } else {
        shell_function_completed = 1;
    }
    if exit_code == 0 && exit_sig > 0 {
        exit_code = 128 + exit_sig;
    }
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        exit_code,
    );
    define_variable_in_set(
        b".SHELLSTATUS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        &raw mut buf as *mut ::core::ffi::c_char,
        o_override,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn func_shell_base(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    trim_newlines: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut child: childbase = childbase {
        cmd_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        environment: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        output: output {
            out: 0,
            err: 0,
            syncout: [0; 1],
            c2rust_padding: [0; 3],
        },
    };
    let mut batch_filename: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let errfd: ::core::ffi::c_int;
    let command_argv: *mut *mut ::core::ffi::c_char;
    let mut pipedes: [::core::ffi::c_int; 2] = [0; 2];
    let pid: pid_t;
    command_argv = construct_command_argv(
        *argv.offset(0 as ::core::ffi::c_int as isize),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ::core::ptr::null_mut::<file>(),
        0,
        &raw mut batch_filename,
    );
    if command_argv.is_null() {
        return o;
    }
    crate::output::output_start();
    errfd = if !output_context.is_null() && (*output_context).err >= 0 {
        (*output_context).err
    } else {
        fileno(stderr)
    };
    child.environment = target_environment(::core::ptr::null_mut::<file>(), 0);
    if pipe(&raw mut pipedes as *mut ::core::ffi::c_int) < 0 {
        error(
            reading_file,
            strlen(strerror(*__errno_location())) as size_t,
            b"pipe: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    } else {
        fd_noinherit(pipedes[1 as ::core::ffi::c_int as usize]);
        fd_noinherit(pipedes[0 as ::core::ffi::c_int as usize]);
        child
            .output
            .set_syncout(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        child.output.out = pipedes[1 as ::core::ffi::c_int as usize];
        child.output.err = errfd;
        pid = child_execute_job(&raw mut child, 1, command_argv);
        if pid < 0 {
            shell_completed(127, 0);
        } else {
            let mut maxlen: size_t;
            let mut i: size_t;
            let mut cc: ::core::ffi::c_int;
            shell_function_pid = pid;
            shell_function_completed = 0;
            if pipedes[1 as ::core::ffi::c_int as usize] >= 0 {
                close(pipedes[1 as ::core::ffi::c_int as usize]);
            }
            maxlen = 200;
            // Owned read buffer (was xmalloc/xrealloc/free). `i` tracks the
            // filled length and `maxlen` the usable size. The Vec is kept fully
            // initialized (len == capacity) so a growth preserves the bytes
            // already read rather than only the [0, len) prefix.
            let mut buffer: Vec<u8> = vec![0u8; maxlen.wrapping_add(1) as usize];
            i = 0;
            loop {
                if i == maxlen {
                    maxlen = maxlen.wrapping_add(512);
                    buffer.resize(maxlen.wrapping_add(1) as usize, 0);
                }
                loop {
                    cc = read(
                        pipedes[0 as ::core::ffi::c_int as usize],
                        buffer.as_mut_ptr().add(i as usize) as *mut ::core::ffi::c_void,
                        (maxlen as size_t).wrapping_sub(i as size_t),
                    ) as ::core::ffi::c_int;
                    if !(cc == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                        break;
                    }
                }
                if cc <= 0 {
                    break;
                }
                i = i.wrapping_add(cc as size_t);
            }
            *buffer.as_mut_ptr().add(i as usize) = 0;
            close(pipedes[0 as ::core::ffi::c_int as usize]);
            while shell_function_completed == 0 {
                reap_children(1, 0);
            }
            if !batch_filename.is_null() {
                if 0x2 as ::core::ffi::c_int & db_level != 0 {
                    printf(
                        b"Cleaning up temporary batch file %s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        batch_filename,
                    );
                    fflush(stdout);
                }
                remove(batch_filename);
                free(batch_filename as *mut ::core::ffi::c_void);
            }
            shell_function_pid = 0 as ::core::ffi::c_int as pid_t;
            fold_newlines(
                buffer.as_mut_ptr() as *mut ::core::ffi::c_char,
                &raw mut i,
                trim_newlines,
            );
            o = variable_buffer_output(o, buffer.as_mut_ptr() as *mut ::core::ffi::c_char, i);
        }
    }
    if !command_argv.is_null() {
        free(*command_argv.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void);
        free(command_argv as *mut ::core::ffi::c_void);
    }
    free_childbase(&raw mut child);
    o
}
unsafe fn func_shell(
    o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    func_shell_base(o, argv, 1)
}
pub const ROOT_LEN: ::core::ffi::c_int = 1;
unsafe extern "C" fn abspath(
    mut name: *const ::core::ffi::c_char,
    apath: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut dest: *mut ::core::ffi::c_char;
    let mut start: *const ::core::ffi::c_char;
    let mut end: *const ::core::ffi::c_char;
    let apath_limit: *const ::core::ffi::c_char;
    let root_len: ::core::ffi::c_ulong = ROOT_LEN as ::core::ffi::c_ulong;
    if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    apath_limit = apath.offset(GET_PATH_MAX as isize);
    if !(*name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32) {
        if starting_directory.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        strcpy(apath, starting_directory);
        dest = strchr(apath, 0);
    } else {
        memcpy(
            apath as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            root_len as size_t,
        );
        *apath.offset(root_len as isize) = 0;
        dest = apath.offset(root_len as isize);
        name = name.offset(root_len as isize);
    }
    end = name;
    start = end;
    while *start as ::core::ffi::c_int != 0 {
        let len: ptrdiff_t;
        while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*start as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & 0x8000 as ::core::ffi::c_int
            != 0
        {
            start = start.offset(1 as ::core::ffi::c_int as isize);
        }
        end = start;
        while !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*end as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x8000 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
            != 0)
        {
            end = end.offset(1 as ::core::ffi::c_int as isize);
        }
        len = end.offset_from(start) as ::core::ffi::c_long as ptrdiff_t;
        if len == 0 as ptrdiff_t {
            break;
        }
        if !(len == 1 as ptrdiff_t
            && *start.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32)
        {
            if len == 2 as ptrdiff_t
                && *start.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '.' as i32
                && *start.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '.' as i32
            {
                if dest > apath.offset(root_len as isize) {
                    dest = dest.offset(-(1 as ::core::ffi::c_int) as isize);
                    while !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                        .offset(*dest.offset(-(1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int
                        & 0x8000 as ::core::ffi::c_int
                        != 0)
                    {
                        dest = dest.offset(-(1 as ::core::ffi::c_int) as isize);
                    }
                }
            } else {
                if !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                    .offset(*dest.offset(-(1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_uchar as isize)
                    as ::core::ffi::c_int
                    & 0x8000 as ::core::ffi::c_int
                    != 0)
                {
                    let fresh5 = dest;
                    dest = dest.offset(1 as ::core::ffi::c_int as isize);
                    *fresh5 = '/' as i32 as ::core::ffi::c_char;
                }
                if apath_limit.offset_from(dest) as ptrdiff_t <= len {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                dest = mempcpy(
                    dest as *mut ::core::ffi::c_void,
                    start as *const ::core::ffi::c_void,
                    len as size_t,
                ) as *mut ::core::ffi::c_char;
                *dest = 0;
            }
        }
        start = end;
    }
    if dest > apath.offset(root_len as isize)
        && *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort).offset(
            *dest.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar as isize,
        ) as ::core::ffi::c_int
            & 0x8000 as ::core::ffi::c_int
            != 0
    {
        dest = dest.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    *dest = 0;
    apath
}
unsafe fn func_realpath(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char = *argv.offset(0 as ::core::ffi::c_int as isize);
    let mut path: *const ::core::ffi::c_char;
    let mut doneany: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    loop {
        path = find_next_token(&raw mut p, &raw mut len);
        if path.is_null() {
            break;
        }
        if len < GET_PATH_MAX as size_t {
            let mut rp: *mut ::core::ffi::c_char;
            let inend: *mut ::core::ffi::c_char;
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
            let mut in_0: [::core::ffi::c_char; 4097] = [0; 4097];
            let mut out: [::core::ffi::c_char; 4097] = [0; 4097];
            inend = mempcpy(
                &raw mut in_0 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                path as *const ::core::ffi::c_void,
                len as size_t,
            ) as *mut ::core::ffi::c_char;
            *inend = 0;
            loop {
                *__errno_location() = 0;
                rp = realpath(
                    &raw mut in_0 as *mut ::core::ffi::c_char,
                    &raw mut out as *mut ::core::ffi::c_char,
                );
                if !(rp.is_null() && *__errno_location() == EINTR) {
                    break;
                }
            }
            if !rp.is_null() {
                let mut r: ::core::ffi::c_int;
                loop {
                    r = stat(&raw mut out as *mut ::core::ffi::c_char, &raw mut st);
                    if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                        break;
                    }
                }
                if r == 0 {
                    o = variable_buffer_output(
                        o,
                        &raw mut out as *mut ::core::ffi::c_char,
                        strlen(&raw mut out as *mut ::core::ffi::c_char) as size_t,
                    );
                    o = variable_buffer_output(
                        o,
                        b" \0" as *const u8 as *const ::core::ffi::c_char,
                        1,
                    );
                    doneany = 1;
                }
            }
        }
    }
    if doneany != 0 {
        o = o.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    o
}
unsafe fn func_file(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut fn_0: *mut ::core::ffi::c_char = *argv.offset(0 as ::core::ffi::c_int as isize);
    if *fn_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '>' as i32 {
        let len: size_t;
        let end: *const ::core::ffi::c_char;
        let start: *const ::core::ffi::c_char;
        let nm: *mut ::core::ffi::c_char;
        let mut fp: *mut FILE;
        let mut mode: *const ::core::ffi::c_char =
            b"w\0" as *const u8 as *const ::core::ffi::c_char;
        fn_0 = fn_0.offset(1 as ::core::ffi::c_int as isize);
        if *fn_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '>' as i32 {
            mode = b"a\0" as *const u8 as *const ::core::ffi::c_char;
            fn_0 = fn_0.offset(1 as ::core::ffi::c_int as isize);
        }
        start = next_token(fn_0);
        if *start.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0 {
            fatal(
                *expanding_var,
                0,
                b"file: missing filename\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        end = end_of_token(start);
        len = end.offset_from(start) as ::core::ffi::c_long as size_t;
        alloca_allocations.push(::std::vec::from_elem(0, len.wrapping_add(1) as usize));
        nm = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        memcpy(
            nm as *mut ::core::ffi::c_void,
            start as *const ::core::ffi::c_void,
            len as size_t,
        );
        *nm.offset(len as isize) = 0;
        loop {
            *__errno_location() = 0;
            fp = fopen(nm, mode) as *mut FILE;
            if !(fp.is_null() && *__errno_location() == EINTR) {
                break;
            }
        }
        if fp.is_null() {
            fatal(
                reading_file,
                (strlen(nm) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"open: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                nm,
                strerror(*__errno_location()),
            );
        }
        command_count = command_count.wrapping_add(1);
        if !(*argv.offset(1 as ::core::ffi::c_int as isize)).is_null() {
            let l: size_t = strlen(*argv.offset(1 as ::core::ffi::c_int as isize)) as size_t;
            let nl: ::core::ffi::c_int = (l == 0
                || *(*argv.offset(1 as ::core::ffi::c_int as isize))
                    .offset(l.wrapping_sub(1) as isize) as ::core::ffi::c_int
                    != '\n' as i32) as ::core::ffi::c_int;
            if fputs(*argv.offset(1 as ::core::ffi::c_int as isize), fp) == EOF
                || nl != 0 && fputc('\n' as i32, fp) == EOF
            {
                fatal(
                    reading_file,
                    (strlen(nm) as size_t)
                        .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                    b"write: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    nm,
                    strerror(*__errno_location()),
                );
            }
        }
        if fclose(fp) != 0 {
            fatal(
                reading_file,
                (strlen(nm) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"close: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                nm,
                strerror(*__errno_location()),
            );
        }
    } else if *fn_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '<' as i32 {
        let mut n: size_t = 0;
        let len_0: size_t;
        let end_0: *const ::core::ffi::c_char;
        let start_0: *const ::core::ffi::c_char;
        let nm_0: *mut ::core::ffi::c_char;
        let mut fp_0: *mut FILE;
        start_0 = next_token(fn_0.offset(1 as ::core::ffi::c_int as isize));
        if *start_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0 {
            fatal(
                *expanding_var,
                0,
                b"file: missing filename\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !(*argv.offset(1 as ::core::ffi::c_int as isize)).is_null() {
            fatal(
                *expanding_var,
                0,
                b"file: too many arguments\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        end_0 = end_of_token(start_0);
        len_0 = end_0.offset_from(start_0) as ::core::ffi::c_long as size_t;
        alloca_allocations.push(::std::vec::from_elem(0, len_0.wrapping_add(1) as usize));
        nm_0 = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        memcpy(
            nm_0 as *mut ::core::ffi::c_void,
            start_0 as *const ::core::ffi::c_void,
            len_0 as size_t,
        );
        *nm_0.offset(len_0 as isize) = 0;
        loop {
            *__errno_location() = 0;
            fp_0 = fopen(nm_0, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
            if !(fp_0.is_null() && *__errno_location() == EINTR) {
                break;
            }
        }
        if fp_0.is_null() {
            if *__errno_location() == ENOENT {
                if 0x2 as ::core::ffi::c_int & db_level != 0 {
                    printf(
                        b"file: Failed to open '%s': %s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        nm_0,
                        strerror(*__errno_location()),
                    );
                    fflush(stdout);
                }
                return o;
            }
            fatal(
                reading_file,
                (strlen(nm_0) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"open: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                nm_0,
                strerror(*__errno_location()),
            );
        }
        loop {
            let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
            let l_0: size_t = fread(
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                1,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
                fp_0,
            ) as size_t;
            if l_0 > 0 {
                o = variable_buffer_output(o, &raw mut buf as *mut ::core::ffi::c_char, l_0);
                n = n.wrapping_add(l_0);
            }
            if ferror(fp_0) != 0 && *__errno_location() != EINTR {
                fatal(
                    reading_file,
                    (strlen(nm_0) as size_t)
                        .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                    b"read: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    nm_0,
                    strerror(*__errno_location()),
                );
            }
            if feof(fp_0) != 0 {
                break;
            }
        }
        if fclose(fp_0) != 0 {
            fatal(
                reading_file,
                (strlen(nm_0) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"close: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                nm_0,
                strerror(*__errno_location()),
            );
        }
        if n != 0
            && *o.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == '\n' as i32
        {
            o = o.offset(
                -((1 + (n > 1
                    && *o.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                        == '\r' as i32) as ::core::ffi::c_int) as isize),
            );
        }
    } else {
        fatal(
            *expanding_var,
            strlen(fn_0) as size_t,
            b"file: invalid file operation: %s\0" as *const u8 as *const ::core::ffi::c_char,
            fn_0,
        );
    }
    o
}
unsafe fn func_abspath(
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char = *argv.offset(0 as ::core::ffi::c_int as isize);
    let mut path: *const ::core::ffi::c_char;
    let mut doneany: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    loop {
        path = find_next_token(&raw mut p, &raw mut len);
        if path.is_null() {
            break;
        }
        if len < GET_PATH_MAX as size_t {
            let mut in_0: [::core::ffi::c_char; 4097] = [0; 4097];
            let mut out: [::core::ffi::c_char; 4097] = [0; 4097];
            let inend: *mut ::core::ffi::c_char = mempcpy(
                &raw mut in_0 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                path as *const ::core::ffi::c_void,
                len as size_t,
            ) as *mut ::core::ffi::c_char;
            *inend = 0;
            if !abspath(
                &raw mut in_0 as *mut ::core::ffi::c_char,
                &raw mut out as *mut ::core::ffi::c_char,
            )
            .is_null()
            {
                o = variable_buffer_output(
                    o,
                    &raw mut out as *mut ::core::ffi::c_char,
                    strlen(&raw mut out as *mut ::core::ffi::c_char) as size_t,
                );
                o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
                doneany = 1;
            }
        }
    }
    if doneany != 0 {
        o = o.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    o
}
/// Build a `function_table_entry` at compile time. Replaces the c2rust-
/// translated `run_static_initializers` constructor that ran ~1000 lines of
/// runtime bitfield-setter calls. The bitfield byte layout matches
/// `function_table_entry`'s `BitfieldStruct` derive: bit 0 = `expand_args`,
/// bit 1 = `alloc_fn`, bit 2 = `adds_command`. All static-table entries set
/// only `expand_args`; `alloc_fn` and `adds_command` are zero.
const fn ft_entry(
    name: &'static [u8],
    min: ::core::ffi::c_uchar,
    max: ::core::ffi::c_uchar,
    expand: u8,
    func: unsafe fn(
        *mut ::core::ffi::c_char,
        *mut *mut ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char,
) -> function_table_entry {
    function_table_entry {
        fptr: C2RustUnnamed {
            func_ptr: Some(func),
        },
        name: name.as_ptr() as *const ::core::ffi::c_char,
        len: (name.len() - 1) as ::core::ffi::c_uchar,
        minimum_args: min,
        maximum_args: max,
        expand_args_alloc_fn_adds_command: [expand & 1],
        c2rust_padding: [0; 4],
    }
}

static mut function_table_init: [function_table_entry; 38] = [
    ft_entry(b"abspath\0", 0, 1, 1, func_abspath),
    ft_entry(b"addprefix\0", 2, 2, 1, func_addsuffix_addprefix),
    ft_entry(b"addsuffix\0", 2, 2, 1, func_addsuffix_addprefix),
    ft_entry(b"and\0", 1, 0, 0, func_and),
    ft_entry(b"basename\0", 0, 1, 1, func_basename_dir),
    ft_entry(b"call\0", 1, 0, 1, func_call),
    ft_entry(b"dir\0", 0, 1, 1, func_basename_dir),
    ft_entry(b"error\0", 0, 1, 1, func_error),
    ft_entry(b"eval\0", 0, 1, 1, func_eval),
    ft_entry(b"file\0", 1, 2, 1, func_file),
    ft_entry(b"filter\0", 2, 2, 1, func_filter_filterout),
    ft_entry(b"filter-out\0", 2, 2, 1, func_filter_filterout),
    ft_entry(b"findstring\0", 2, 2, 1, func_findstring),
    ft_entry(b"firstword\0", 0, 1, 1, func_firstword),
    ft_entry(b"flavor\0", 0, 1, 1, func_flavor),
    ft_entry(b"foreach\0", 3, 3, 0, func_foreach),
    ft_entry(b"if\0", 2, 3, 0, func_if),
    ft_entry(b"info\0", 0, 1, 1, func_error),
    ft_entry(b"intcmp\0", 2, 5, 0, func_intcmp),
    ft_entry(b"join\0", 2, 2, 1, func_join),
    ft_entry(b"lastword\0", 0, 1, 1, func_lastword),
    ft_entry(b"let\0", 3, 3, 0, func_let),
    ft_entry(b"notdir\0", 0, 1, 1, func_notdir_suffix),
    ft_entry(b"or\0", 1, 0, 0, func_or),
    ft_entry(b"origin\0", 0, 1, 1, func_origin),
    ft_entry(b"patsubst\0", 3, 3, 1, func_patsubst),
    ft_entry(b"realpath\0", 0, 1, 1, func_realpath),
    ft_entry(b"shell\0", 0, 1, 1, func_shell),
    ft_entry(b"sort\0", 0, 1, 1, func_sort),
    ft_entry(b"strip\0", 0, 1, 1, func_strip),
    ft_entry(b"subst\0", 3, 3, 1, func_subst),
    ft_entry(b"suffix\0", 0, 1, 1, func_notdir_suffix),
    ft_entry(b"value\0", 0, 1, 1, func_value),
    ft_entry(b"warning\0", 0, 1, 1, func_error),
    ft_entry(b"wildcard\0", 0, 1, 1, func_wildcard),
    ft_entry(b"word\0", 2, 2, 1, func_word),
    ft_entry(b"wordlist\0", 3, 3, 1, func_wordlist),
    ft_entry(b"words\0", 0, 1, 1, func_words),
];
unsafe extern "C" fn expand_builtin_function(
    mut o: *mut ::core::ffi::c_char,
    argc: ::core::ffi::c_uint,
    argv: *mut *mut ::core::ffi::c_char,
    entry_p: *const function_table_entry,
) -> *mut ::core::ffi::c_char {
    let p: *mut ::core::ffi::c_char;
    if argc < (*entry_p).minimum_args as ::core::ffi::c_uint {
        fatal(
            *expanding_var,
            strlen((*entry_p).name) as size_t,
            b"insufficient number of arguments (%u) to function '%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            argc,
            (*entry_p).name,
        );
    }
    if argc == 0 && (*entry_p).alloc_fn() == 0 {
        return o;
    }
    if (*entry_p).fptr.func_ptr.is_none() {
        fatal(
            *expanding_var,
            strlen((*entry_p).name) as size_t,
            b"unimplemented on this platform: function '%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            (*entry_p).name,
        );
    }
    if (*entry_p).adds_command() != 0 {
        command_count = command_count.wrapping_add(1);
    }
    if (*entry_p).alloc_fn() == 0 {
        return (*entry_p).fptr.func_ptr.expect("non-null function pointer")(
            o,
            argv,
            (*entry_p).name,
        );
    }
    p = (*entry_p)
        .fptr
        .alloc_func_ptr
        .expect("non-null function pointer")((*entry_p).name, argc, argv);
    if !p.is_null() {
        o = variable_buffer_output(o, p, strlen(p) as size_t);
        free(p as *mut ::core::ffi::c_void);
    }
    o
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn handle_function(
    op: *mut *mut ::core::ffi::c_char,
    stringp: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let entry_p: *const function_table_entry;
    let openparen: ::core::ffi::c_char = *(*stringp).offset(0 as ::core::ffi::c_int as isize);
    let closeparen: ::core::ffi::c_char = (if openparen as ::core::ffi::c_int == '(' as i32 {
        ')' as i32
    } else {
        '}' as i32
    }) as ::core::ffi::c_char;
    let mut beg: *const ::core::ffi::c_char;
    let mut end: *const ::core::ffi::c_char;
    let mut count: ::core::ffi::c_int = 0;
    let mut abeg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let argv: *mut *mut ::core::ffi::c_char;
    let mut argvp: *mut *mut ::core::ffi::c_char;
    let mut nargs: ::core::ffi::c_uint;
    beg = (*stringp).offset(1 as ::core::ffi::c_int as isize);
    entry_p = lookup_function(beg);
    if entry_p.is_null() {
        return 0;
    }
    beg = beg.offset((*entry_p).len as ::core::ffi::c_int as isize);
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*beg as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        != 0
    {
        beg = beg.offset(1 as ::core::ffi::c_int as isize);
    }
    nargs = 1;
    end = beg;
    while *end as ::core::ffi::c_int != 0 {
        if *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*end as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x80 as ::core::ffi::c_int | 0x400 as ::core::ffi::c_int)
            != 0
        {
            if *end as ::core::ffi::c_int == ',' as i32 {
                nargs = nargs.wrapping_add(1);
            } else if *end as ::core::ffi::c_int == openparen as ::core::ffi::c_int {
                count += 1;
            } else if *end as ::core::ffi::c_int == closeparen as ::core::ffi::c_int && {
                count -= 1;
                count < 0
            } {
                break;
            }
        }
        end = end.offset(1 as ::core::ffi::c_int as isize);
    }
    if count >= 0 {
        fatal(
            *expanding_var,
            strlen((*entry_p).name) as size_t,
            b"unterminated call to function '%s': missing '%c'\0" as *const u8
                as *const ::core::ffi::c_char,
            (*entry_p).name,
            closeparen as ::core::ffi::c_int,
        );
    }
    *stringp = end;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            .wrapping_mul(nargs.wrapping_add(2) as usize) as usize,
    ));
    argv = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_char;
    argvp = argv;
    if (*entry_p).expand_args() != 0 {
        let mut p: *const ::core::ffi::c_char;
        p = beg;
        nargs = 0;
        while p <= end {
            let mut next: *const ::core::ffi::c_char;
            nargs = nargs.wrapping_add(1);
            if nargs == (*entry_p).maximum_args as ::core::ffi::c_uint || {
                next = find_next_argument(openparen, closeparen, p, end);
                next.is_null()
            } {
                next = end;
            }
            *argvp = expand_argument(p, next);
            p = next.offset(1 as ::core::ffi::c_int as isize);
            argvp = argvp.offset(1 as ::core::ffi::c_int as isize);
        }
    } else {
        let len: size_t = end.offset_from(beg) as ::core::ffi::c_long as size_t;
        let mut p_0: *mut ::core::ffi::c_char;
        let aend: *mut ::core::ffi::c_char;
        abeg = xmalloc(len.wrapping_add(1)) as *mut ::core::ffi::c_char;
        aend = mempcpy(
            abeg as *mut ::core::ffi::c_void,
            beg as *const ::core::ffi::c_void,
            len as size_t,
        ) as *mut ::core::ffi::c_char;
        *aend = 0;
        p_0 = abeg;
        nargs = 0;
        while p_0 <= aend {
            let mut next_0: *mut ::core::ffi::c_char;
            nargs = nargs.wrapping_add(1);
            if nargs == (*entry_p).maximum_args as ::core::ffi::c_uint || {
                next_0 = find_next_argument(openparen, closeparen, p_0, aend);
                next_0.is_null()
            } {
                next_0 = aend;
            }
            *argvp = p_0;
            *next_0 = 0;
            p_0 = next_0.offset(1 as ::core::ffi::c_int as isize);
            argvp = argvp.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    *argvp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *op = expand_builtin_function(*op, nargs, argv, entry_p);
    if (*entry_p).expand_args() != 0 {
        argvp = argv;
        while !(*argvp).is_null() {
            free(*argvp as *mut ::core::ffi::c_void);
            argvp = argvp.offset(1 as ::core::ffi::c_int as isize);
        }
    } else {
        free(abeg as *mut ::core::ffi::c_void);
    }
    1
}
unsafe fn func_call(
    mut o: *mut ::core::ffi::c_char,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    static mut max_args: ::core::ffi::c_uint = 0;
    let fname: *mut ::core::ffi::c_char;
    let flen: size_t;
    let mut i: ::core::ffi::c_uint;
    let saved_args: ::core::ffi::c_int;
    let entry_p: *const function_table_entry;
    let v: *mut variable;
    fname = next_token(*argv.offset(0 as ::core::ffi::c_int as isize));
    *end_of_token(fname).offset(0 as ::core::ffi::c_int as isize) = 0;
    if *fname as ::core::ffi::c_int == 0 {
        return o;
    }
    entry_p = lookup_function(fname);
    if !entry_p.is_null() {
        i = 0;
        while !(*argv.offset(i.wrapping_add(1) as isize)).is_null() {
            i = i.wrapping_add(1);
        }
        return expand_builtin_function(
            o,
            i,
            argv.offset(1 as ::core::ffi::c_int as isize),
            entry_p,
        );
    }
    flen = strlen(fname) as size_t;
    v = lookup_variable(fname, flen);
    if v.is_null() {
        warn_undefined(fname, flen);
    }
    if v.is_null() || *(*v).value as ::core::ffi::c_int == 0 {
        return o;
    }
    push_new_variable_scope();
    i = 0;
    while !(*argv).is_null() {
        let mut num: [::core::ffi::c_char; 22] = [0; 22];
        define_variable_in_set(
            &raw mut num as *mut ::core::ffi::c_char,
            sprintf(
                &raw mut num as *mut ::core::ffi::c_char,
                b"%u\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            ) as size_t,
            *argv,
            o_automatic,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
        i = i.wrapping_add(1);
        argv = argv.offset(1 as ::core::ffi::c_int as isize);
    }
    while i < max_args {
        let mut num_0: [::core::ffi::c_char; 22] = [0; 22];
        define_variable_in_set(
            &raw mut num_0 as *mut ::core::ffi::c_char,
            sprintf(
                &raw mut num_0 as *mut ::core::ffi::c_char,
                b"%u\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            ) as size_t,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            o_automatic,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
        i = i.wrapping_add(1);
    }
    (*v).set_exp_count(EXP_COUNT_MAX as ::core::ffi::c_uint as ::core::ffi::c_uint);
    saved_args = max_args as ::core::ffi::c_int;
    max_args = i;
    o = expand_variable_output(o, fname, flen);
    max_args = saved_args as ::core::ffi::c_uint;
    (*v).set_exp_count(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    pop_variable_scope();
    o.offset(strlen(o) as isize)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn define_new_function(
    flocp: *const Floc,
    name: *const ::core::ffi::c_char,
    min: ::core::ffi::c_uint,
    max: ::core::ffi::c_uint,
    flags: ::core::ffi::c_uint,
    func: gmk_func_ptr,
) {
    let mut e: *const ::core::ffi::c_char = name;
    let mut ent: *mut function_table_entry;
    let len: size_t;
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*e as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & 0x2000 as ::core::ffi::c_int
        != 0
    {
        e = e.offset(1 as ::core::ffi::c_int as isize);
    }
    len = e.offset_from(name) as ::core::ffi::c_long as size_t;
    if len == 0 {
        fatal(
            flocp,
            0,
            b"empty function name\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *name as ::core::ffi::c_int == '.' as i32 || *e as ::core::ffi::c_int != 0 {
        fatal(
            flocp,
            strlen(name) as size_t,
            b"invalid function name: %s\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
    }
    if len > 255 {
        fatal(
            flocp,
            strlen(name) as size_t,
            b"function name too long: %s\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
    }
    if min > 255 {
        fatal(
            flocp,
            INTSTR_LENGTH.wrapping_add(strlen(name) as size_t),
            b"invalid minimum argument count (%u) for function %s\0" as *const u8
                as *const ::core::ffi::c_char,
            min,
            name,
        );
    }
    if max > 255 || max != 0 && max < min {
        fatal(
            flocp,
            INTSTR_LENGTH.wrapping_add(strlen(name) as size_t),
            b"invalid maximum argument count (%u) for function %s\0" as *const u8
                as *const ::core::ffi::c_char,
            max,
            name,
        );
    }
    ent = xmalloc(::core::mem::size_of::<function_table_entry>() as size_t)
        as *mut function_table_entry;
    (*ent).name = strcache_add(name);
    (*ent).len = len as ::core::ffi::c_uchar;
    (*ent).minimum_args = min as ::core::ffi::c_uchar;
    (*ent).maximum_args = max as ::core::ffi::c_uchar;
    (*ent).set_expand_args(
        (if flags & 0x1 as ::core::ffi::c_uint != 0 {
            0
        } else {
            1
        }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
    (*ent).set_alloc_fn(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*ent).set_adds_command(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*ent).fptr.alloc_func_ptr = func;
    ent = hash_insert(&raw mut function_table, ent as *const ::core::ffi::c_void)
        as *mut function_table_entry;
    free(ent as *mut ::core::ffi::c_void);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn hash_init_function_table() {
    hash_init(
        &raw mut function_table,
        (::core::mem::size_of::<[function_table_entry; 38]>() as ::core::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<function_table_entry>() as ::core::ffi::c_ulong)
            .wrapping_mul(2),
        Some(function_table_entry_hash_1),
        Some(function_table_entry_hash_2),
        Some(function_table_entry_hash_cmp),
    );
    hash_load(
        &raw mut function_table,
        &raw const function_table_init as *const function_table_entry as *const ::core::ffi::c_void,
        (::core::mem::size_of::<[function_table_entry; 38]>() as ::core::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<function_table_entry>() as ::core::ffi::c_ulong),
        ::core::mem::size_of::<function_table_entry>() as ::core::ffi::c_ulong,
    );
}

#[cfg(test)]
mod ft_init_tests {
    use super::*;

    /// Verify that the bitfield byte we wrote in `ft_entry` round-trips
    /// through the `BitfieldStruct`-generated getters. This catches any
    /// drift between our hand-packed byte layout and the c2rust_bitfields
    /// crate's expected encoding.
    #[test]
    fn bitfield_byte_matches_getters() {
        let entries = unsafe {
            std::slice::from_raw_parts(
                (&raw const function_table_init).cast::<function_table_entry>(),
                38,
            )
        };
        for (i, e) in entries.iter().enumerate() {
            let byte = e.expand_args_alloc_fn_adds_command[0];
            let exp = e.expand_args();
            let alloc = e.alloc_fn();
            let adds = e.adds_command();
            let reconstructed = (exp & 1) | ((alloc & 1) << 1) | ((adds & 1) << 2);
            assert_eq!(
                byte as u32, reconstructed,
                "idx {i}: byte {byte:#04x} != getters(exp={exp}, alloc={alloc}, adds={adds})"
            );
            assert!(
                alloc == 0 && adds == 0,
                "idx {i}: alloc/adds expected zero in static table"
            );
        }
    }
}
