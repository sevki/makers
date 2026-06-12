pub use crate::ffi_types::{size_t, uintmax_t};
use crate::file::{Commands, File, VariableSet, VariableSetList};
use crate::misc::{lindex, xmalloc, xrealloc, xstrdup, xstrndup};
use crate::stdio::FILE;
use libc::{free, printf, strchr};
extern "C" {
    pub type dep;
    static mut environ: *mut *mut ::core::ffi::c_char;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}

/// Owns a `malloc`ed C string and frees it on drop, replacing the manual
/// `xstrdup`/`expand_argument` + `free` ownership pairs in this module.
struct OwnedCStr(*mut ::core::ffi::c_char);

impl OwnedCStr {
    /// Borrow the underlying NUL-terminated buffer.
    fn as_ptr(&self) -> *mut ::core::ffi::c_char {
        self.0
    }
}

impl Drop for OwnedCStr {
    fn drop(&mut self) {
        unsafe { free(self.0 as *mut ::core::ffi::c_void) }
    }
}

pub type file = File;
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
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
pub type hash_table = crate::hash::hash_table;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
pub type commands = Commands;
use crate::floc::Floc;
use crate::function::{handle_function, patsubst_expand_pat};
use crate::make_main::{db_level, stopchar_map};
use crate::output::fatal;
use crate::read::{find_percent, reading_file};
use crate::variable::{
    current_variable_set_list, env_recursion, install_file_context, lookup_variable,
    lookup_variable_in_set, restore_file_context, warn_undefined,
};

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
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub static mut expanding_var: *mut *const Floc = &raw const reading_file as *mut *const Floc;
pub const VARIABLE_BUFFER_ZONE: ::core::ffi::c_int = 5;
static mut variable_buffer_length: size_t = 0;
pub static mut variable_buffer: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn variable_buffer_output(
    mut ptr: *mut ::core::ffi::c_char,
    string: *const ::core::ffi::c_char,
    length: size_t,
) -> *mut ::core::ffi::c_char {
    let newlen: size_t =
        length.wrapping_add(ptr.offset_from(variable_buffer) as ::core::ffi::c_long as size_t);
    if ptr >= variable_buffer {
    } else {
        panic!("assertion failed: ptr >= variable_buffer");
    };
    if ptr < variable_buffer.offset(variable_buffer_length as isize) {
    } else {
        panic!("assertion failed: ptr < variable_buffer + variable_buffer_length");
    };
    if newlen
        .wrapping_add(VARIABLE_BUFFER_ZONE as size_t)
        .wrapping_add(1)
        > variable_buffer_length
    {
        let offset: size_t = ptr.offset_from(variable_buffer) as ::core::ffi::c_long as size_t;
        variable_buffer_length =
            if newlen.wrapping_add(100) > (2 as size_t).wrapping_mul(variable_buffer_length) {
                newlen.wrapping_add(100)
            } else {
                (2 as size_t).wrapping_mul(variable_buffer_length)
            };
        variable_buffer = xrealloc(
            variable_buffer as *mut ::core::ffi::c_void,
            variable_buffer_length.wrapping_add(1),
        ) as *mut ::core::ffi::c_char;
        ptr = variable_buffer.offset(offset as isize);
    }
    ptr = mempcpy(
        ptr as *mut ::core::ffi::c_void,
        string as *const ::core::ffi::c_void,
        length as size_t,
    ) as *mut ::core::ffi::c_char;
    *ptr = 0;
    ptr
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn initialize_variable_output() -> *mut ::core::ffi::c_char {
    if variable_buffer.is_null() {
        variable_buffer_length = 200;
        variable_buffer = xmalloc(variable_buffer_length) as *mut ::core::ffi::c_char;
    }
    *variable_buffer.offset(0 as ::core::ffi::c_int as isize) = 0;
    variable_buffer
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn install_variable_buffer(bufp: *mut *mut ::core::ffi::c_char, lenp: *mut size_t) {
    *bufp = variable_buffer;
    *lenp = variable_buffer_length;
    variable_buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    initialize_variable_output();
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn restore_variable_buffer(buf: *mut ::core::ffi::c_char, len: size_t) {
    free(variable_buffer as *mut ::core::ffi::c_void);
    variable_buffer = buf;
    variable_buffer_length = len;
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn swap_variable_buffer(
    buf: *mut ::core::ffi::c_char,
    len: size_t,
) -> *mut ::core::ffi::c_char {
    let p: *mut ::core::ffi::c_char = variable_buffer;
    variable_buffer = buf;
    variable_buffer_length = len;
    p
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn recursively_expand_for_file(
    v: *mut variable,
    file: *mut file,
) -> *mut ::core::ffi::c_char {
    let value: *mut ::core::ffi::c_char;
    let mut this_var: *const Floc;
    let saved_varp: *mut *const Floc;
    let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut set_reading: ::core::ffi::c_int = 0;
    let nl: size_t = strlen((*v).name) as size_t;
    let mut parent: *mut variable = ::core::ptr::null_mut::<variable>();
    if (*v).expanding() as ::core::ffi::c_int != 0 && env_recursion != 0 {
        let mut ep: *mut *mut ::core::ffi::c_char;
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
            if strncmp(*ep, (*v).name, nl as size_t) == 0
                && *(*ep).offset(nl as isize) as ::core::ffi::c_int == '=' as i32
            {
                return xstrdup((*ep).offset(nl as isize).offset(1));
            }
            ep = ep.offset(1 as ::core::ffi::c_int as isize);
        }
        return xstrdup(b"\0" as *const u8 as *const ::core::ffi::c_char);
    }
    saved_varp = expanding_var;
    if !(*v).fileinfo.filenm.is_null() {
        this_var = &raw mut (*v).fileinfo;
        expanding_var = &raw mut this_var;
    }
    if reading_file.is_null() {
        set_reading = 1;
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
        (*v).set_exp_count((*v).exp_count() - 1);
    }
    if !file.is_null() {
        install_file_context(file, &raw mut savev, ::core::ptr::null_mut::<*const Floc>());
    }
    (*v).set_expanding(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if (*v).append() != 0 {
        let mut sl: *mut variable_set_list;
        sl = current_variable_set_list;
        while !sl.is_null() && parent.is_null() {
            let vp: *mut variable = lookup_variable_in_set((*v).name, nl, (*sl).set);
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
        reading_file = ::core::ptr::null::<Floc>();
    }
    if !file.is_null() {
        restore_file_context(savev, ::core::ptr::null::<Floc>());
    }
    expanding_var = saved_varp;
    value
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_variable_output(
    mut ptr: *mut ::core::ffi::c_char,
    name: *const ::core::ffi::c_char,
    length: size_t,
) -> *mut ::core::ffi::c_char {
    let v: *mut variable;
    let recursive: ::core::ffi::c_uint;
    let value: *mut ::core::ffi::c_char;
    v = lookup_variable(name, length);
    if v.is_null() {
        warn_undefined(name, length);
    }
    if v.is_null()
        || *(*v).value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0
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
    ptr
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_variable_buf(
    mut buf: *mut ::core::ffi::c_char,
    name: *const ::core::ffi::c_char,
    length: size_t,
) -> *mut ::core::ffi::c_char {
    let offs: size_t;
    if buf.is_null() {
        buf = initialize_variable_output();
    }
    if buf >= variable_buffer {
    } else {
        panic!("assertion failed: buf >= variable_buffer");
    };
    if buf < variable_buffer.offset(variable_buffer_length as isize) {
    } else {
        panic!("assertion failed: buf < variable_buffer + variable_buffer_length");
    };
    offs = buf.offset_from(variable_buffer) as ::core::ffi::c_long as size_t;
    expand_variable_output(buf, name, length);
    variable_buffer.offset(offs as isize)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn allocated_expand_variable(
    name: *const ::core::ffi::c_char,
    length: size_t,
) -> *mut ::core::ffi::c_char {
    let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut olen: size_t = 0;
    install_variable_buffer(&raw mut obuf, &raw mut olen);
    expand_variable_output(variable_buffer, name, length);
    swap_variable_buffer(obuf, olen)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn allocated_expand_variable_for_file(
    name: *const ::core::ffi::c_char,
    length: size_t,
    file: *mut file,
) -> *mut ::core::ffi::c_char {
    let result: *mut ::core::ffi::c_char;
    let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut savef: *const Floc = ::core::ptr::null::<Floc>();
    if file.is_null() {
        return allocated_expand_variable(name, length);
    }
    install_file_context(file, &raw mut savev, &raw mut savef);
    result = allocated_expand_variable(name, length);
    restore_file_context(savev, savef);
    result
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_string_buf(
    mut buf: *mut ::core::ffi::c_char,
    string: *const ::core::ffi::c_char,
    length: size_t,
) -> *mut ::core::ffi::c_char {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut v: *mut variable;
    let mut p: *const ::core::ffi::c_char;
    let mut p1: *const ::core::ffi::c_char;
    let mut o: *mut ::core::ffi::c_char;
    let line_offset: size_t;
    if buf.is_null() {
        buf = initialize_variable_output();
    }
    o = buf;
    line_offset = buf.offset_from(variable_buffer) as ::core::ffi::c_long as size_t;
    if length == 0 {
        return variable_buffer;
    }
    let save = OwnedCStr(if length == SIZE_MAX as size_t {
        xstrdup(string)
    } else {
        xstrndup(string, length)
    });
    p = save.as_ptr();
    loop {
        p1 = strchr(p, '$' as i32);
        o = variable_buffer_output(
            o,
            p,
            if !p1.is_null() {
                p1.offset_from(p) as ::core::ffi::c_long as size_t
            } else {
                (strlen(p) as size_t).wrapping_add(1)
            },
        );
        if p1.is_null() {
            break;
        }
        p = p1.offset(1 as ::core::ffi::c_int as isize);
        match *p as ::core::ffi::c_int {
            36 | 0 => {
                o = variable_buffer_output(o, p1, 1);
            }
            40 | 123 => {
                let openparen: ::core::ffi::c_char = *p;
                let closeparen: ::core::ffi::c_char =
                    (if openparen as ::core::ffi::c_int == '(' as i32 {
                        ')' as i32
                    } else {
                        '}' as i32
                    }) as ::core::ffi::c_char;
                let mut beg: *const ::core::ffi::c_char =
                    p.offset(1 as ::core::ffi::c_int as isize);
                let mut abeg: Option<OwnedCStr> = None;
                let mut end: *const ::core::ffi::c_char;
                let mut colon: *const ::core::ffi::c_char;
                if !(handle_function(&raw mut o, &raw mut p) != 0) {
                    end = strchr(beg, closeparen as ::core::ffi::c_int);
                    if end.is_null() {
                        fatal(
                            *expanding_var,
                            0,
                            b"unterminated variable reference\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    p1 = lindex(beg, end, '$' as i32);
                    if !p1.is_null() {
                        let mut count: ::core::ffi::c_int = 1;
                        p = beg;
                        while *p as ::core::ffi::c_int != 0 {
                            if *p as ::core::ffi::c_int == openparen as ::core::ffi::c_int {
                                count += 1;
                            } else if *p as ::core::ffi::c_int == closeparen as ::core::ffi::c_int
                                && {
                                    count -= 1;
                                    count == 0
                                }
                            {
                                break;
                            }
                            p = p.offset(1 as ::core::ffi::c_int as isize);
                        }
                        if count == 0 {
                            let owned = OwnedCStr(expand_argument(beg, p));
                            beg = owned.as_ptr();
                            abeg = Some(owned);
                            end = strchr(beg, 0);
                        }
                    } else {
                        p = end;
                    }
                    colon = lindex(beg, end, ':' as i32);
                    if !colon.is_null() {
                        let subst_beg: *const ::core::ffi::c_char =
                            colon.offset(1 as ::core::ffi::c_int as isize);
                        let subst_end: *const ::core::ffi::c_char =
                            lindex(subst_beg, end, '=' as i32);
                        if subst_end.is_null() {
                            colon = ::core::ptr::null::<::core::ffi::c_char>();
                        } else {
                            let replace_beg: *const ::core::ffi::c_char =
                                subst_end.offset(1 as ::core::ffi::c_int as isize);
                            let replace_end: *const ::core::ffi::c_char = end;
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
                            if !v.is_null() && *(*v).value as ::core::ffi::c_int != 0 {
                                let mut pattern: *mut ::core::ffi::c_char;
                                let mut replace: *mut ::core::ffi::c_char;
                                let mut ppercent: *mut ::core::ffi::c_char;
                                let mut rpercent: *mut ::core::ffi::c_char;
                                let value: *mut ::core::ffi::c_char = if (*v).recursive()
                                    as ::core::ffi::c_int
                                    != 0
                                {
                                    recursively_expand_for_file(v, ::core::ptr::null_mut::<file>())
                                } else {
                                    (*v).value
                                };
                                alloca_allocations.push(::std::vec::from_elem(
                                    0,
                                    (subst_end.offset_from(subst_beg) as ::core::ffi::c_long + 2)
                                        as ::core::ffi::c_ulong
                                        as usize,
                                ));
                                pattern = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                    as *mut ::core::ffi::c_char;
                                let fresh0 = pattern;
                                pattern = pattern.offset(1 as ::core::ffi::c_int as isize);
                                *fresh0 = '%' as i32 as ::core::ffi::c_char;
                                memcpy(
                                    pattern as *mut ::core::ffi::c_void,
                                    subst_beg as *const ::core::ffi::c_void,
                                    subst_end.offset_from(subst_beg) as ::core::ffi::c_long
                                        as size_t,
                                );
                                *pattern.offset(subst_end.offset_from(subst_beg)
                                    as ::core::ffi::c_long
                                    as isize) = 0;
                                alloca_allocations.push(::std::vec::from_elem(
                                    0,
                                    (replace_end.offset_from(replace_beg) as ::core::ffi::c_long
                                        + 2)
                                        as ::core::ffi::c_ulong
                                        as usize,
                                ));
                                replace = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                    as *mut ::core::ffi::c_char;
                                let fresh1 = replace;
                                replace = replace.offset(1 as ::core::ffi::c_int as isize);
                                *fresh1 = '%' as i32 as ::core::ffi::c_char;
                                memcpy(
                                    replace as *mut ::core::ffi::c_void,
                                    replace_beg as *const ::core::ffi::c_void,
                                    replace_end.offset_from(replace_beg) as ::core::ffi::c_long
                                        as size_t,
                                );
                                *replace.offset(replace_end.offset_from(replace_beg)
                                    as ::core::ffi::c_long
                                    as isize) = 0;
                                ppercent = find_percent(pattern);
                                if !ppercent.is_null() {
                                    ppercent = ppercent.offset(1 as ::core::ffi::c_int as isize);
                                    rpercent = find_percent(replace);
                                    if !rpercent.is_null() {
                                        rpercent =
                                            rpercent.offset(1 as ::core::ffi::c_int as isize);
                                    }
                                } else {
                                    ppercent = pattern;
                                    rpercent = replace;
                                    pattern = pattern.offset(-(1 as ::core::ffi::c_int) as isize);
                                    replace = replace.offset(-(1 as ::core::ffi::c_int) as isize);
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
                    // Free the expanded reference here, as the C code did.
                    drop(abeg);
                }
            }
            _ => {
                if !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort).offset(
                    *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar as isize,
                ) as ::core::ffi::c_int
                    & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                    != 0)
                {
                    o = expand_variable_output(o, p, 1);
                }
            }
        }
        if *p as ::core::ffi::c_int == 0 {
            break;
        }
        p = p.offset(1 as ::core::ffi::c_int as isize);
    }
    variable_buffer.offset(line_offset as isize)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_argument(
    str: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if str == end {
        return xstrdup(b"\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if end.is_null() || *end as ::core::ffi::c_int == 0 {
        return allocated_expand_string_for_file(str, ::core::ptr::null_mut::<file>());
    }
    // Copy the [str, end) slice into an owned, NUL-terminated buffer (the C
    // code chose alloca vs xmalloc by length; an owned Vec covers both).
    let len = end.offset_from(str) as ::core::ffi::c_long as size_t;
    let mut tmp_buf: Vec<u8> = ::std::vec::from_elem(0u8, (len as usize).wrapping_add(1));
    let tmp = tmp_buf.as_mut_ptr() as *mut ::core::ffi::c_char;
    memcpy(
        tmp as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        len,
    );
    *tmp.offset(len as isize) = 0;
    allocated_expand_string_for_file(tmp, ::core::ptr::null_mut::<file>())
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_string_for_file(
    string: *const ::core::ffi::c_char,
    file: *mut file,
) -> *mut ::core::ffi::c_char {
    let result: *mut ::core::ffi::c_char;
    let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut savef: *const Floc = ::core::ptr::null::<Floc>();
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
    result
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn allocated_expand_string_for_file(
    string: *const ::core::ffi::c_char,
    file: *mut file,
) -> *mut ::core::ffi::c_char {
    let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut olen: size_t = 0;
    install_variable_buffer(&raw mut obuf, &raw mut olen);
    expand_string_for_file(string, file);
    swap_variable_buffer(obuf, olen)
}
unsafe extern "C" fn variable_append(
    name: *const ::core::ffi::c_char,
    length: size_t,
    set: *const variable_set_list,
    local: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let v: *const variable;
    let mut buf: *mut ::core::ffi::c_char;
    let nextlocal: ::core::ffi::c_int;
    if set.is_null() {
        return initialize_variable_output();
    }
    nextlocal = (local != 0 && (*set).next_is_parent == 0) as ::core::ffi::c_int;
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
        buf = variable_buffer_output(buf, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
    }
    if (*v).recursive() == 0 {
        return variable_buffer_output(buf, (*v).value, strlen((*v).value) as size_t);
    }
    buf = expand_string_buf(buf, (*v).value, strlen((*v).value) as size_t);
    buf.offset(strlen(buf) as isize)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn allocated_variable_append(v: *const variable) -> *mut ::core::ffi::c_char {
    let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut olen: size_t = 0;
    install_variable_buffer(&raw mut obuf, &raw mut olen);
    variable_append(
        (*v).name,
        strlen((*v).name) as size_t,
        current_variable_set_list,
        1,
    );
    swap_variable_buffer(obuf, olen)
}
