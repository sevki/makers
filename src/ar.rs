use libc::{fnmatch, free, strchr};

use crate::file::{Dep, File};
pub use crate::ffi_types::{__time_t, intmax_t, size_t, time_t, uintmax_t};
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
    fn concat(_: ::core::ffi::c_uint, ...) -> *const ::core::ffi::c_char;
    fn error(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn fatal(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn perror_with_name(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char);
    fn xcalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xstrdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn alpha_compare(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn ar_scan(
        archive: *const ::core::ffi::c_char,
        function: ar_member_func_t,
        arg: *const ::core::ffi::c_void,
    ) -> intmax_t;
    fn ar_name_equal(
        name: *const ::core::ffi::c_char,
        mem: *const ::core::ffi::c_char,
        truncated: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ar_member_touch(
        arname: *const ::core::ffi::c_char,
        memname: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn file_exists_p(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcache_add(str: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn lookup_file(name: *const ::core::ffi::c_char) -> *mut file;
    fn enter_file(name: *const ::core::ffi::c_char) -> *mut file;
    fn f_mtime(file: *mut file, search: ::core::ffi::c_int) -> uintmax_t;
}
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
pub type dep = Dep;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ar_glob_state {
    pub arname: *const ::core::ffi::c_char,
    pub pattern: *const ::core::ffi::c_char,
    pub size: size_t,
    pub chain: *mut nameseq,
    pub n: ::core::ffi::c_uint,
}
pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FNM_PATHNAME: ::core::ffi::c_int = (1) << 0;
pub const FNM_PERIOD: ::core::ffi::c_int = (1) << 2;
#[no_mangle]
pub unsafe extern "C" fn ar_name(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let p: *const ::core::ffi::c_char = strchr(name, '(' as i32);
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if p.is_null() || p == name {
        return 0;
    }
    end = p
        .offset(strlen(p) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    if *end as ::core::ffi::c_int != ')' as i32 || end == p.offset(1 as ::core::ffi::c_int as isize) {
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
#[no_mangle]
pub unsafe extern "C" fn ar_parse_name(
    name: *const ::core::ffi::c_char,
    arname_p: *mut *mut ::core::ffi::c_char,
    memname_p: *mut *mut ::core::ffi::c_char,
) {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
#[no_mangle]
pub unsafe extern "C" fn ar_member_date(name: *const ::core::ffi::c_char) -> time_t {
    let mut arname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut memname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: intmax_t = 0;
    ar_parse_name(name, &raw mut arname, &raw mut memname);
    let mut arfile: *mut file = ::core::ptr::null_mut::<file>();
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
#[no_mangle]
pub unsafe extern "C" fn ar_touch(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut arname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut memname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: ::core::ffi::c_int = 0;
    ar_parse_name(name, &raw mut arname, &raw mut memname);
    let mut arfile: *mut file = ::core::ptr::null_mut::<file>();
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
unsafe extern "C" fn ar_glob_match(
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
    let mut state: *mut ar_glob_state = arg as *mut ar_glob_state;
    if fnmatch((*state).pattern, mem, FNM_PATHNAME | FNM_PERIOD) == 0 {
        let mut new: *mut nameseq = xcalloc((*state).size) as *mut nameseq;
        (*new).name = strcache_add(concat(
            4,
            (*state).arname,
            b"(\0" as *const u8 as *const ::core::ffi::c_char,
            mem,
            b")\0" as *const u8 as *const ::core::ffi::c_char,
        ));
        (*new).next = (*state).chain;
        (*state).chain = new;
        (*state).n = (*state).n.wrapping_add(1);
    }
    0 as intmax_t
}
unsafe extern "C" fn ar_glob_pattern_p(
    pattern: *const ::core::ffi::c_char,
    quote: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
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
#[no_mangle]
pub unsafe extern "C" fn ar_glob(
    arname: *const ::core::ffi::c_char,
    member_pattern: *const ::core::ffi::c_char,
    size: size_t,
) -> *mut nameseq {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut state: ar_glob_state = ar_glob_state {
        arname: ::core::ptr::null::<::core::ffi::c_char>(),
        pattern: ::core::ptr::null::<::core::ffi::c_char>(),
        size: 0,
        chain: ::core::ptr::null_mut::<nameseq>(),
        n: 0,
    };
    let mut n: *mut nameseq = ::core::ptr::null_mut::<nameseq>();
    let mut names: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut i: ::core::ffi::c_uint = 0;
    if ar_glob_pattern_p(member_pattern, 1) == 0 {
        return ::core::ptr::null_mut::<nameseq>();
    }
    state.arname = arname;
    state.pattern = member_pattern;
    state.size = size;
    state.chain = ::core::ptr::null_mut::<nameseq>();
    state.n = 0;
    ar_scan(
        arname,
        Some(
            ar_glob_match
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
        return ::core::ptr::null_mut::<nameseq>();
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
        let ref mut fresh2 = *names.offset(fresh1 as isize);
        *fresh2 = (*n).name;
        n = (*n).next;
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
        (*n).name = *names.offset(fresh3 as isize);
        n = (*n).next;
    }
    state.chain
}
pub const __CHAR_BIT__: ::core::ffi::c_int = 8;
