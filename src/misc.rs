use libc::{__errno_location, free, getenv, mkstemp, putchar, sleep, sprintf, stpcpy, strchr, strcmp, strcpy, strdup, strerror, strtoul, unlink};
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type variable_set_list;
    pub type commands;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vsprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
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
    fn strndup(__string: *const ::core::ffi::c_char, __n: size_t) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn error(flocp: *const floc, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn out_of_memory() -> !;
    static mut stopchar_map: [::core::ffi::c_ushort; 0];
    static mut posix_pedantic: ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn os_anontmp() -> ::core::ffi::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type __gnuc_va_list = __builtin_va_list;
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
pub type va_list = __gnuc_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const ::core::ffi::c_char,
    pub lineno: ::core::ffi::c_ulong,
    pub offset: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const ::core::ffi::c_char,
}
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const P_tmpdir: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"/tmp\0") };
pub const EINTR: ::core::ffi::c_int = 4;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const DEFAULT_TMPDIR: [::core::ffi::c_char; 5] = P_tmpdir;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*b"FILE *get_tmpfile(char **)\0")
};
#[inline]

unsafe extern "C" fn free_ns(mut n: *mut nameseq) {
    free(n as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn make_toui(
    mut str: *const ::core::ffi::c_char,
    mut error_0: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: ::core::ffi::c_ulong = strtoul(str, &raw mut end, 10);
    if !error_0.is_null() {
        if *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0 {
            *error_0 = b"Missing value\0" as *const u8 as *const ::core::ffi::c_char;
        } else if *end as ::core::ffi::c_int != 0 {
            *error_0 = b"Invalid value\0" as *const u8 as *const ::core::ffi::c_char;
        } else {
            *error_0 = ::core::ptr::null::<::core::ffi::c_char>();
        }
    }
    val as ::core::ffi::c_uint
}
#[no_mangle]
pub unsafe extern "C" fn make_lltoa(
    mut val: ::core::ffi::c_longlong,
    mut buf: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    sprintf(
        buf,
        b"%lld\0" as *const u8 as *const ::core::ffi::c_char,
        val,
    );
    buf
}
#[no_mangle]
pub unsafe extern "C" fn make_ulltoa(
    mut val: ::core::ffi::c_ulonglong,
    mut buf: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    sprintf(
        buf,
        b"%llu\0" as *const u8 as *const ::core::ffi::c_char,
        val,
    );
    buf
}
static mut mk_state: ::core::ffi::c_uint = 0;
#[no_mangle]
pub unsafe extern "C" fn make_seed(mut seed: ::core::ffi::c_uint) {
    mk_state = seed;
}
#[no_mangle]
pub unsafe extern "C" fn make_rand() -> ::core::ffi::c_uint {
    if mk_state == 0 {
        mk_state = ((time(::core::ptr::null_mut::<time_t>()) ^ make_pid() as time_t)
            as ::core::ffi::c_uint)
            .wrapping_add(1);
    }
    mk_state ^= mk_state << 13;
    mk_state ^= mk_state >> 17;
    mk_state ^= mk_state << 5;
    mk_state
}
#[no_mangle]
pub unsafe extern "C" fn alpha_compare(
    mut v1: *const ::core::ffi::c_void,
    mut v2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut s1: *const ::core::ffi::c_char = *(v1 as *mut *mut ::core::ffi::c_char);
    let mut s2: *const ::core::ffi::c_char = *(v2 as *mut *mut ::core::ffi::c_char);
    if *s1 as ::core::ffi::c_int != *s2 as ::core::ffi::c_int {
        return *s1 as ::core::ffi::c_int - *s2 as ::core::ffi::c_int;
    }
    strcmp(s1, s2)
}
#[no_mangle]
pub unsafe extern "C" fn collapse_continuations(mut line: *mut ::core::ffi::c_char) {
    let mut out: *mut ::core::ffi::c_char = line;
    let mut in_0: *mut ::core::ffi::c_char = line;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    q = strchr(in_0, '\n' as i32);
    if q.is_null() {
        return;
    }
    loop {
        let mut p: *mut ::core::ffi::c_char = q;
        let mut i: ::core::ffi::c_int = 0;
        let mut out_line_length: size_t = 0;
        if q > line
            && *q.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == '\\' as i32 {
            i = -(2 as ::core::ffi::c_int);
            while p.offset(i as isize) as *mut ::core::ffi::c_char >= line
                && *p.offset(i as isize) as ::core::ffi::c_int == '\\' as i32
            {
                i -= 1;
            }
            i += 1;
        } else {
            i = 0;
        }
        out_line_length = (p.offset_from(in_0) as ::core::ffi::c_long + i as ::core::ffi::c_long
            - (i / 2) as ::core::ffi::c_long)
            as size_t;
        if out != in_0 {
            memmove(
                out as *mut ::core::ffi::c_void,
                in_0 as *const ::core::ffi::c_void,
                out_line_length as size_t,
            );
        }
        out = out.offset(out_line_length as isize);
        in_0 = q.offset(1 as ::core::ffi::c_int as isize);
        if i & 1 != 0 {
            let mut dollar: ::core::ffi::c_uint = 0;
            while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                .offset(*in_0 as ::core::ffi::c_uchar as isize)
                as ::core::ffi::c_int
                & 0x2 as ::core::ffi::c_int
                != 0
            {
                in_0 = in_0.offset(1 as ::core::ffi::c_int as isize);
            }
            let mut dp: *const ::core::ffi::c_char = out;
            while dp > line as *const ::core::ffi::c_char
                && *dp.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == '$' as i32
            {
                dp = dp.offset(-(1 as ::core::ffi::c_int) as isize);
            }
            dollar = (out.offset_from(dp) as ::core::ffi::c_long % 2)
                as ::core::ffi::c_uint;
            if dollar != 0 {
                out = out.offset(-(1 as ::core::ffi::c_int) as isize);
            }
            if posix_pedantic == 0 {
                while out > line
                    && *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                        .offset(*out.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int
                        & 0x2 as ::core::ffi::c_int
                        != 0
                {
                    out = out.offset(-(1 as ::core::ffi::c_int) as isize);
                }
            }
            if dollar == 0 {
                let fresh1 = out;
                out = out.offset(1 as ::core::ffi::c_int as isize);
                *fresh1 = ' ' as i32 as ::core::ffi::c_char;
            }
        } else {
            let fresh2 = out;
            out = out.offset(1 as ::core::ffi::c_int as isize);
            *fresh2 = '\n' as i32 as ::core::ffi::c_char;
        }
        q = strchr(in_0, '\n' as i32);
        if q.is_null() {
            break;
        }
    }
    memmove(
        out as *mut ::core::ffi::c_void,
        in_0 as *const ::core::ffi::c_void,
        strlen(in_0).wrapping_add(1),
    );
}
#[no_mangle]
pub unsafe extern "C" fn print_spaces(mut n: ::core::ffi::c_uint) {
    loop {
        let fresh4 = n;
        n = n.wrapping_sub(1);
        if !(fresh4 > 0) {
            break;
        }
        putchar(' ' as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn concat(
    mut num: ::core::ffi::c_uint,
    mut args: ...
) -> *const ::core::ffi::c_char {
    static mut rlen: size_t = 0;
    static mut result: *mut ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    let mut ri: size_t = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    loop {
        let fresh0 = num;
        num = num.wrapping_sub(1);
        if !(fresh0 > 0) {
            break;
        }
        let mut s: *const ::core::ffi::c_char = args_0.arg::<*const ::core::ffi::c_char>();
        let mut l: size_t = if s.is_null() {
            0
        } else {
            strlen(s) as size_t
        };
        if l == 0 {
            continue;
        }
        if ri.wrapping_add(l) > rlen {
            rlen = (if rlen != 0 { rlen } else { 60 })
                .wrapping_add(l)
                .wrapping_mul(2);
            result = xrealloc(result as *mut ::core::ffi::c_void, rlen) as *mut ::core::ffi::c_char;
        }
        memcpy(
            result.offset(ri as isize) as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            l as size_t,
        );
        ri = ri.wrapping_add(l);
    }
    if ri == rlen {
        rlen = (if rlen != 0 { rlen } else { 60 }).wrapping_mul(2);
        result = xrealloc(result as *mut ::core::ffi::c_void, rlen) as *mut ::core::ffi::c_char;
    }
    *result.offset(ri as isize) = 0;
    result
}
#[no_mangle]
pub unsafe extern "C" fn make_pid() -> pid_t {
    getpid() as pid_t
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    let mut result: *mut ::core::ffi::c_void = malloc(if size != 0 {
        size as size_t
    } else {
        1
    });
    if result.is_null() {
        out_of_memory();
    }
    result
}
#[no_mangle]
pub unsafe extern "C" fn xcalloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    let mut result: *mut ::core::ffi::c_void = calloc(
        if size != 0 {
            size as size_t
        } else {
            1
        },
        1,
    );
    if result.is_null() {
        out_of_memory();
    }
    result
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut result: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if size == 0 {
        size = 1;
    }
    result = if !ptr.is_null() {
        realloc(ptr, size as size_t)
    } else {
        malloc(size as size_t)
    };
    if result.is_null() {
        out_of_memory();
    }
    result
}
#[no_mangle]
pub unsafe extern "C" fn xstrdup(mut ptr: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    result = strdup(ptr);
    if result.is_null() {
        out_of_memory();
    }
    result
}
#[no_mangle]
pub unsafe extern "C" fn xstrndup(
    mut str: *const ::core::ffi::c_char,
    mut length: size_t,
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    result = strndup(str, length as size_t);
    if result.is_null() {
        out_of_memory();
    }
    result
}
#[no_mangle]
pub unsafe extern "C" fn lindex(
    mut s: *const ::core::ffi::c_char,
    mut limit: *const ::core::ffi::c_char,
    mut c: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    while s < limit {
        let fresh3 = s;
        s = s.offset(1 as ::core::ffi::c_int as isize);
        if *fresh3 as ::core::ffi::c_int == c {
            return s.offset(-(1 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_char;
        }
    }
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
#[no_mangle]
pub unsafe extern "C" fn end_of_token(
    mut s: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    while !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*s as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
        != 0)
    {
        s = s.offset(1 as ::core::ffi::c_int as isize);
    }
    s as *mut ::core::ffi::c_char
}
#[no_mangle]
pub unsafe extern "C" fn next_token(mut s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*s as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        != 0
    {
        s = s.offset(1 as ::core::ffi::c_int as isize);
    }
    s as *mut ::core::ffi::c_char
}
#[no_mangle]
pub unsafe extern "C" fn skip_reference(
    mut p: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut openparen: ::core::ffi::c_char = *p;
    let mut closeparen: ::core::ffi::c_char = 0;
    let mut count: ::core::ffi::c_int = 1;
    if openparen as ::core::ffi::c_int == 0 {
        return p as *mut ::core::ffi::c_char;
    }
    if openparen as ::core::ffi::c_int == '(' as i32 {
        closeparen = ')' as i32 as ::core::ffi::c_char;
    } else if openparen as ::core::ffi::c_int == '{' as i32 {
        closeparen = '}' as i32 as ::core::ffi::c_char;
    } else {
        return p.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
    }
    loop {
        p = p.offset(1 as ::core::ffi::c_int as isize);
        if !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x1 as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int)
            != 0)
        {
            continue;
        }
        if *p as ::core::ffi::c_int == 0 {
            break;
        }
        if *p as ::core::ffi::c_int == openparen as ::core::ffi::c_int {
            count += 1;
        } else {
            if !(*p as ::core::ffi::c_int == closeparen as ::core::ffi::c_int && {
                count -= 1;
                count == 0
            }) {
                continue;
            }
            p = p.offset(1 as ::core::ffi::c_int as isize);
            break;
        }
    }
    p as *mut ::core::ffi::c_char
}
#[no_mangle]
pub unsafe extern "C" fn find_next_token(
    mut ptr: *mut *const ::core::ffi::c_char,
    mut lengthptr: *mut size_t,
) -> *mut ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char = next_token(*ptr);
    if *p as ::core::ffi::c_int == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *ptr = end_of_token(p);
    if !lengthptr.is_null() {
        *lengthptr = (*ptr).offset_from(p) as ::core::ffi::c_long as size_t;
    }
    p as *mut ::core::ffi::c_char
}
#[no_mangle]
pub unsafe extern "C" fn writebuf(
    mut fd: ::core::ffi::c_int,
    mut buffer: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut msg: *const ::core::ffi::c_char = buffer as *const ::core::ffi::c_char;
    let mut l: size_t = len;
    while l != 0 {
        let mut r: ssize_t = 0;
        loop {
            r = write(fd, msg as *const ::core::ffi::c_void, l as size_t);
            if !(r == -(1 as ::core::ffi::c_int) as ssize_t && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 as ssize_t {
            return -(1 as ::core::ffi::c_int) as ssize_t;
        }
        l = l.wrapping_sub(r as size_t);
        msg = msg.offset(r as isize);
    }
    len as ssize_t
}
#[no_mangle]
pub unsafe extern "C" fn readbuf(
    mut fd: ::core::ffi::c_int,
    mut buffer: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut msg: *mut ::core::ffi::c_char = buffer as *mut ::core::ffi::c_char;
    while len != 0 {
        let mut r: ssize_t = 0;
        loop {
            r = read(fd, msg as *mut ::core::ffi::c_void, len as size_t);
            if !(r == -(1 as ::core::ffi::c_int) as ssize_t && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 as ssize_t {
            return -(1 as ::core::ffi::c_int) as ssize_t;
        }
        if r == 0 as ssize_t {
            break;
        }
        len = len.wrapping_sub(r as size_t);
        msg = msg.offset(r as isize);
    }
    msg.offset_from(buffer as *mut ::core::ffi::c_char) as ::core::ffi::c_long as ssize_t
}
#[no_mangle]
pub unsafe extern "C" fn copy_dep(mut d: *const dep) -> *mut dep {
    let mut new: *mut dep = ::core::ptr::null_mut::<dep>();
    if !d.is_null() {
        new = xmalloc(::core::mem::size_of::<dep>() as size_t) as *mut dep;
        memcpy(
            new as *mut ::core::ffi::c_void,
            d as *const ::core::ffi::c_void,
            ::core::mem::size_of::<dep>() as size_t,
        );
        if (*new).need_2nd_expansion() != 0 {
            (*new).name = xstrdup((*new).name);
        }
        (*new).next = ::core::ptr::null_mut::<dep>();
    }
    new
}
#[no_mangle]
pub unsafe extern "C" fn copy_dep_chain(mut d: *const dep) -> *mut dep {
    let mut firstnew: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut lastnew: *mut dep = ::core::ptr::null_mut::<dep>();
    while !d.is_null() {
        let mut c: *mut dep = copy_dep(d);
        if firstnew.is_null() {
            lastnew = c;
            firstnew = lastnew;
        } else {
            (*lastnew).next = c;
            lastnew = (*lastnew).next;
        }
        d = (*d).next;
    }
    firstnew
}
#[no_mangle]
pub unsafe extern "C" fn free_ns_chain(mut ns: *mut nameseq) {
    while !ns.is_null() {
        let mut t: *mut nameseq = ns;
        ns = (*ns).next;
        free_ns(t);
    }
}
#[no_mangle]
pub unsafe fn spin(mut type_0: *const ::core::ffi::c_char) {
    let mut filenm: [::core::ffi::c_char; 256] = [0; 256];
    let mut dummy: stat = stat {
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
    sprintf(
        &raw mut filenm as *mut ::core::ffi::c_char,
        b".make-spin-%s\0" as *const u8 as *const ::core::ffi::c_char,
        type_0,
    );
    if stat(&raw mut filenm as *mut ::core::ffi::c_char, &raw mut dummy) == 0
    {
        fprintf(
            stderr,
            b"SPIN on %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut filenm as *mut ::core::ffi::c_char,
        );
        loop {
            sleep(1);
            if !(stat(&raw mut filenm as *mut ::core::ffi::c_char, &raw mut dummy)
                == 0)
            {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dbg(mut fmt: *const ::core::ffi::c_char, mut args: ...) {
    let mut fp: *mut FILE = fopen(
        b"/tmp/gmkdebug.log\0" as *const u8 as *const ::core::ffi::c_char,
        b"a+\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    args_0 = args.clone();
    vsprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        fmt,
        args_0.as_va_list(),
    );
    fprintf(
        fp,
        b"%u: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        make_pid() as ::core::ffi::c_uint,
        &raw mut buf as *mut ::core::ffi::c_char,
    );
    fflush(fp);
    fclose(fp);
}
pub const DEFAULT_TMPFILE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GmXXXXXX\0") };
#[no_mangle]
pub unsafe fn get_tmpdir() -> *const ::core::ffi::c_char {
    static mut tmpdir: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if tmpdir.is_null() {
        let mut tlist: [*const ::core::ffi::c_char; 3] = [
            b"MAKE_TMPDIR\0" as *const u8 as *const ::core::ffi::c_char,
            b"TMPDIR\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        let mut tp: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        let mut found: ::core::ffi::c_uint = 0;
        tp = &raw mut tlist as *mut *const ::core::ffi::c_char;
        while !(*tp).is_null() {
            tmpdir = getenv(*tp);
            if !tmpdir.is_null() && *tmpdir as ::core::ffi::c_int != 0 {
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
                let mut r: ::core::ffi::c_int = 0;
                found = 1;
                loop {
                    r = stat(tmpdir, &raw mut st);
                    if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                        break;
                    }
                }
                if r < 0 {
                    error(
                        ::core::ptr::null_mut::<floc>(),
                        (strlen(*tp) as size_t)
                            .wrapping_add(strlen(tmpdir) as size_t)
                            .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                        b"%s value %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                        *tp,
                        tmpdir,
                        strerror(*__errno_location()),
                    );
                } else if !(st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t) {
                    error(
                        ::core::ptr::null_mut::<floc>(),
                        (strlen(*tp) as size_t).wrapping_add(strlen(tmpdir) as size_t),
                        b"%s value %s: not a directory\0" as *const u8
                            as *const ::core::ffi::c_char,
                        *tp,
                        tmpdir,
                    );
                } else {
                    return tmpdir;
                }
            }
            tp = tp.offset(1 as ::core::ffi::c_int as isize);
        }
        tmpdir = DEFAULT_TMPDIR.as_ptr();
        if found != 0 {
            error(
                ::core::ptr::null_mut::<floc>(),
                strlen(tmpdir) as size_t,
                b"using default temporary directory '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                tmpdir,
            );
        }
    }
    tmpdir
}
#[no_mangle]
pub unsafe extern "C" fn get_tmptemplate() -> *mut ::core::ffi::c_char {
    let mut tmpdir: *const ::core::ffi::c_char = get_tmpdir();
    let mut template: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    template = xmalloc(
        (strlen(tmpdir) as size_t)
            .wrapping_add(
                (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t)
                    .wrapping_sub(1),
            )
            .wrapping_add(2),
    ) as *mut ::core::ffi::c_char;
    cp = stpcpy(template, tmpdir);
    if !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*cp.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & 0x8000 as ::core::ffi::c_int
        != 0)
    {
        let fresh5 = cp;
        cp = cp.offset(1 as ::core::ffi::c_int as isize);
        *fresh5 = '/' as i32 as ::core::ffi::c_char;
    }
    strcpy(cp, DEFAULT_TMPFILE.as_ptr());
    template
}
#[no_mangle]
pub unsafe extern "C" fn get_tmpfd(mut name: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut tmpnm: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mask: mode_t = 0;
    if !name.is_null() {
        *name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        fd = os_anontmp();
        if fd >= 0 {
            return fd;
        }
    }
    mask = umask(0o77 as __mode_t) as mode_t;
    tmpnm = get_tmptemplate();
    loop {
        fd = mkstemp(tmpnm);
        if !(fd == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if fd < 0 {
        error(
            ::core::ptr::null_mut::<floc>(),
            (strlen(tmpnm) as size_t).wrapping_add(strlen(strerror(*__errno_location())) as size_t),
            b"cannot create temporary file %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
            tmpnm,
            strerror(*__errno_location()),
        );
        free(tmpnm as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    if !name.is_null() {
        *name = tmpnm;
    } else {
        let mut r: ::core::ffi::c_int = 0;
        loop {
            r = unlink(tmpnm);
            if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            error(
                ::core::ptr::null_mut::<floc>(),
                (strlen(tmpnm) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"cannot unlink temporary file %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                tmpnm,
                strerror(*__errno_location()),
            );
        }
        free(tmpnm as *mut ::core::ffi::c_void);
    }
    umask(mask as __mode_t);
    fd
}
#[no_mangle]
pub unsafe extern "C" fn get_tmpfile(mut name: *mut *mut ::core::ffi::c_char) -> *mut FILE {
    let mut tmpfile_mode: *const ::core::ffi::c_char =
        b"wb+\0" as *const u8 as *const ::core::ffi::c_char;
    let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut fd: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !name.is_null() {
        } else {
            __assert_fail(
                b"name\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/misc.c\0" as *const u8 as *const ::core::ffi::c_char,
                827,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    fd = get_tmpfd(name);
    if fd < 0 {
        return ::core::ptr::null_mut::<FILE>();
    }
    '_c2rust_label_0: {
        if !(*name).is_null() {
        } else {
            __assert_fail(
                b"*name\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/misc.c\0" as *const u8 as *const ::core::ffi::c_char,
                831,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    loop {
        *__errno_location() = 0;
        file = fdopen(fd, tmpfile_mode);
        if !(file.is_null() && *__errno_location() == EINTR) {
            break;
        }
    }
    if file.is_null() {
        error(
            ::core::ptr::null_mut::<floc>(),
            (strlen(*name) as size_t).wrapping_add(strlen(strerror(*__errno_location())) as size_t),
            b"fdopen: temporary file %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
            *name,
            strerror(*__errno_location()),
        );
    }
    file
}
