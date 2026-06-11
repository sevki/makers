pub use crate::file::enter_file;
pub use crate::file::lookup_file;
pub use crate::remake::f_mtime;
pub use crate::file::{CommandState, UpdateStatus};
use libc::{fnmatch, free, strchr};

pub use crate::ffi_types::{__time_t, intmax_t, size_t, time_t, uintmax_t};
use crate::file::{Dep, File};
use crate::misc::{xcalloc, xstrdup};
use crate::strcache::strcache_add;
extern "C" {
    pub type variable_set_list;
    pub type commands;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
use crate::floc::Floc;

pub type ar_member_func_t = Option<
    unsafe extern "C" fn(
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
        ::core::ffi::c_long,
        ::core::ffi::c_long,
        ::core::ffi::c_long,
        intmax_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_uint,
        *const ::core::ffi::c_void,
    ) -> intmax_t,
>;
use crate::arscan::{ar_member_touch, ar_name_equal, ar_scan};
use crate::dir::file_exists_p;
pub use crate::file::nameseq;
use crate::file::{enter_file, lookup_file};
use crate::misc::{alpha_compare, concat};
use crate::output::{error, fatal, perror_with_name};
use crate::remake::f_mtime;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ar_glob_state {
    pub arname: *const ::core::ffi::c_char,
    pub pattern: *const ::core::ffi::c_char,
    pub chain: *mut T,
    pub n: ::core::ffi::c_uint,
}
pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FNM_PATHNAME: ::core::ffi::c_int = (1) << 0;
pub const FNM_PERIOD: ::core::ffi::c_int = (1) << 2;
/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_name(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let p: *const ::core::ffi::c_char = strchr(name, '(' as i32);
    let end: *const ::core::ffi::c_char;
    if p.is_null() || p == name {
        return 0;
    }
    end = p
        .offset(strlen(p) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    if *end as ::core::ffi::c_int != ')' as i32 || end == p.offset(1 as ::core::ffi::c_int as isize)
    {
        return 0;
    }
    if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '(' as i32
        && *end.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == ')' as i32
    {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            strlen(name) as size_t,
            b"attempt to use unsupported feature: '%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
        );
    }
    1
}
/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_parse_name(
    name: *const ::core::ffi::c_char,
    arname_p: *mut *mut ::core::ffi::c_char,
    memname_p: *mut *mut ::core::ffi::c_char,
) {
    let mut p: *mut ::core::ffi::c_char;
    *arname_p = xstrdup(name);
    p = strchr(*arname_p, '(' as i32);
    if p.is_null() {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            strlen(*arname_p) as size_t,
            b"INTERNAL: ar_parse_name: bad name '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            *arname_p,
        );
    }
    let fresh0 = p;
    p = p.offset(1 as ::core::ffi::c_int as isize);
    *fresh0 = 0;
    *p.offset(strlen(p).wrapping_sub(1) as isize) = 0;
    *memname_p = p;
}
unsafe extern "C" fn ar_member_date_1(
    mut _desc: ::core::ffi::c_int,
    mem: *const ::core::ffi::c_char,
    truncated: ::core::ffi::c_int,
    mut _hdrpos: ::core::ffi::c_long,
    mut _datapos: ::core::ffi::c_long,
    mut _size: ::core::ffi::c_long,
    date: intmax_t,
    mut _uid: ::core::ffi::c_int,
    mut _gid: ::core::ffi::c_int,
    mut _mode: ::core::ffi::c_uint,
    name: *const ::core::ffi::c_void,
) -> intmax_t {
    if ar_name_equal(name as *const ::core::ffi::c_char, mem, truncated) != 0 {
        date
    } else {
        0 as intmax_t
    }
}
/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_member_date(name: *const ::core::ffi::c_char) -> time_t {
    let mut arname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut memname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let val: intmax_t;
    ar_parse_name(name, &raw mut arname, &raw mut memname);
    let mut arfile: *mut File;
    arfile = lookup_file(arname);
    if arfile.is_null() && file_exists_p(arname) != 0 {
        arfile = enter_file(strcache_add(arname));
    }
    if !arfile.is_null() {
        f_mtime(arfile, 0);
    }
    val = ar_scan(
        arname,
        Some(
            ar_member_date_1
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_long,
                    ::core::ffi::c_long,
                    ::core::ffi::c_long,
                    intmax_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_uint,
                    *const ::core::ffi::c_void,
                ) -> intmax_t,
        ),
        memname as *const ::core::ffi::c_void,
    );
    free(arname as *mut ::core::ffi::c_void);
    if (0 as intmax_t) < val
        && val
            <= (if (0 as ::core::ffi::c_int as time_t) < -(1 as ::core::ffi::c_int) as time_t {
                -(1 as ::core::ffi::c_int) as time_t
            } else {
                (((1 as ::core::ffi::c_int as time_t)
                    << (::core::mem::size_of::<time_t>() as usize)
                        .wrapping_mul(CHAR_BIT as usize)
                        .wrapping_sub(2 as usize))
                    - 1 as time_t)
                    * 2 as time_t
                    + 1 as time_t
            }) as intmax_t
    {
        val as time_t
    } else {
        -(1 as ::core::ffi::c_int) as time_t
    }
}
/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_touch(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut arname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut memname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: ::core::ffi::c_int;
    ar_parse_name(name, &raw mut arname, &raw mut memname);
    let arfile: *mut File;
    arfile = enter_file(strcache_add(arname));
    f_mtime(arfile, 0);
    val = 1;
    match ar_member_touch(arname, memname) {
        -1 => {
            error(
                ::core::ptr::null_mut::<Floc>(),
                strlen(arname) as size_t,
                b"touch: archive '%s' does not exist\0" as *const u8 as *const ::core::ffi::c_char,
                arname,
            );
        }
        -2 => {
            error(
                ::core::ptr::null_mut::<Floc>(),
                strlen(arname) as size_t,
                b"touch: '%s' is not a valid archive\0" as *const u8 as *const ::core::ffi::c_char,
                arname,
            );
        }
        -3 => {
            perror_with_name(
                b"touch: \0" as *const u8 as *const ::core::ffi::c_char,
                arname,
            );
        }
        1 => {
            error(
                ::core::ptr::null_mut::<Floc>(),
                (strlen(memname) as size_t).wrapping_add(strlen(arname) as size_t),
                b"touch: member '%s' does not exist in '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                memname,
                arname,
            );
        }
        0 => {
            val = 0;
        }
        _ => {
            error(
                ::core::ptr::null_mut::<Floc>(),
                strlen(name) as size_t,
                b"touch: bad return code from ar_member_touch on '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                name,
            );
        }
    }
    free(arname as *mut ::core::ffi::c_void);
    val
}
unsafe extern "C" fn ar_glob_match<T: SeqNode>(
    mut _desc: ::core::ffi::c_int,
    mem: *const ::core::ffi::c_char,
    mut _truncated: ::core::ffi::c_int,
    mut _hdrpos: ::core::ffi::c_long,
    mut _datapos: ::core::ffi::c_long,
    mut _size: ::core::ffi::c_long,
    mut _date: intmax_t,
    mut _uid: ::core::ffi::c_int,
    mut _gid: ::core::ffi::c_int,
    mut _mode: ::core::ffi::c_uint,
    arg: *const ::core::ffi::c_void,
) -> intmax_t {
    let state: *mut ArGlobState<T> = arg as *mut ArGlobState<T>;
    if fnmatch((*state).pattern, mem, FNM_PATHNAME | FNM_PERIOD) == 0 {
        let new: *mut T = T::alloc();
        T::set_name(new, strcache_add(concat(
            4,
            (*state).arname,
            b"(\0" as *const u8 as *const ::core::ffi::c_char,
            mem,
            b")\0" as *const u8 as *const ::core::ffi::c_char,
        )));
        T::set_next(new, (*state).chain);
        (*state).chain = new;
        (*state).n = (*state).n.wrapping_add(1);
    }
    0 as intmax_t
}
unsafe extern "C" fn ar_glob_pattern_p(
    pattern: *const ::core::ffi::c_char,
    quote: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char;
    let mut opened: ::core::ffi::c_int = 0;
    p = pattern;
    while *p as ::core::ffi::c_int != 0 {
        match *p as ::core::ffi::c_int {
            63 | 42 => return 1,
            92 => {
                if quote != 0 {
                    p = p.offset(1 as ::core::ffi::c_int as isize);
                }
            }
            91 => {
                opened = 1;
            }
            93 => {
                if opened != 0 {
                    return 1;
                }
            }
            _ => {}
        }
        p = p.offset(1 as ::core::ffi::c_int as isize);
    }
    0
}
/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_glob(
    arname: *const ::core::ffi::c_char,
    member_pattern: *const ::core::ffi::c_char,
) -> *mut T {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut state: ArGlobState<T> = ArGlobState {
        arname: ::core::ptr::null::<::core::ffi::c_char>(),
        pattern: ::core::ptr::null::<::core::ffi::c_char>(),
        chain: ::core::ptr::null_mut::<T>(),
        n: 0,
    };
    let mut n: *mut T;
    let names: *mut *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_uint;
    if ar_glob_pattern_p(member_pattern, 1) == 0 {
        return ::core::ptr::null_mut::<T>();
    }
    state.arname = arname;
    state.pattern = member_pattern;
    state.chain = ::core::ptr::null_mut::<T>();
    state.n = 0;
    ar_scan(
        arname,
        Some(
            ar_glob_match::<T>
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_long,
                    ::core::ffi::c_long,
                    ::core::ffi::c_long,
                    intmax_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_uint,
                    *const ::core::ffi::c_void,
                ) -> intmax_t,
        ),
        &raw mut state as *const ::core::ffi::c_void,
    );
    if state.chain.is_null() {
        return ::core::ptr::null_mut::<T>();
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (state.n as usize)
            .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
            as usize,
    ));
    names = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *const ::core::ffi::c_char;
    i = 0;
    n = state.chain;
    while !n.is_null() {
        let fresh1 = i;
        i = i.wrapping_add(1);
        let fresh2 = &mut (*names.offset(fresh1 as isize));
        *fresh2 = T::name(n);
        n = T::next(n);
    }
    qsort(
        names as *mut ::core::ffi::c_void,
        i as size_t,
        ::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t,
        Some(
            alpha_compare
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    i = 0;
    n = state.chain;
    while !n.is_null() {
        let fresh3 = i;
        i = i.wrapping_add(1);
        T::set_name(n, *names.offset(fresh3 as isize));
        n = T::next(n);
    }
    state.chain
}
pub const __CHAR_BIT__: ::core::ffi::c_int = 8;
