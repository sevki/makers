use libc::{__errno_location, close, exit, perror, sprintf, strcat, strerror};
use ::c2rust_bitfields;
use crate::ffi_types::{_IO_codecvt, _IO_marker, _IO_wide_data, FILE};
extern "C" {
    fn lseek(__fd: ::core::ffi::c_int, __offset: __off_t, __whence: ::core::ffi::c_int) -> __off_t;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn ftruncate(__fd: ::core::ffi::c_int, __length: __off_t) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn vsprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn should_print_dir() -> ::core::ffi::c_int;
    fn die(_: ::core::ffi::c_int) -> !;
    fn xrealloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn get_tmpfd(_: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn writebuf(_: ::core::ffi::c_int, _: *const ::core::ffi::c_void, _: size_t) -> ssize_t;
    static mut print_data_base_flag: ::core::ffi::c_int;
    static mut output_sync: ::core::ffi::c_int;
    static mut program: *const ::core::ffi::c_char;
    static mut starting_directory: *mut ::core::ffi::c_char;
    static mut makelevel: ::core::ffi::c_uint;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn check_io_state() -> ::core::ffi::c_uint;
    fn fd_noinherit(fd: ::core::ffi::c_int);
    fn fd_set_append(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fd_reset_append(fd: ::core::ffi::c_int, flags: ::core::ffi::c_int);
    fn osync_clear();
    fn osync_acquire() -> ::core::ffi::c_uint;
    fn osync_release();
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
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type uintmax_t = ::libc::uintmax_t;
use crate::floc::Floc;

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct output {
    pub out: ::core::ffi::c_int,
    pub err: ::core::ffi::c_int,
    #[bitfield(name = "syncout", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub syncout: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fmtstring {
    pub buffer: *mut ::core::ffi::c_char,
    pub size: size_t,
}
pub const EINTR: ::core::ffi::c_int = 4;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INTSTR_LENGTH: usize = (53 as usize)
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22 as usize)
    .wrapping_add(3 as usize);
pub const OUTPUT_SYNC_NONE: ::core::ffi::c_int = 0;
pub const OUTPUT_SYNC_RECURSE: ::core::ffi::c_int = 3;
pub const MAKE_FAILURE: ::core::ffi::c_int = 2;
pub const SEEK_SET: ::core::ffi::c_int = 0;
pub const SEEK_END: ::core::ffi::c_int = 2;
#[no_mangle]
pub static mut output_context: *mut output = ::core::ptr::null::<output>() as *mut output;
#[no_mangle]
pub static mut stdio_traced: ::core::ffi::c_uint = 0;
pub const OUTPUT_NONE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C" fn _outputs(
    mut out: *mut output,
    mut is_err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    if !out.is_null() && (*out).syncout() as ::core::ffi::c_int != 0 {
        let mut fd: ::core::ffi::c_int = if is_err != 0 { (*out).err } else { (*out).out };
        if fd != OUTPUT_NONE {
            let mut len: size_t = strlen(msg) as size_t;
            let mut r: ::core::ffi::c_int = 0;
            loop {
                r = lseek(fd, 0 as __off_t, 2) as ::core::ffi::c_int;
                if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                    break;
                }
            }
            writebuf(fd, msg as *const ::core::ffi::c_void, len);
            return;
        }
    }
    f = if is_err != 0 { stderr } else { stdout };
    fputs(msg, f);
    fflush(f);
}
#[no_mangle]
pub unsafe extern "C" fn log_working_directory(mut entering: ::core::ffi::c_int) -> ::core::ffi::c_int {
    static mut buf: *mut ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    static mut len: size_t = 0;
    let mut need: size_t = 0;
    let mut fmt: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    need = strlen(program)
        .wrapping_add(INTSTR_LENGTH)
        .wrapping_add(2)
        .wrapping_add(1) as size_t;
    if !starting_directory.is_null() {
        need = need.wrapping_add(strlen(starting_directory) as size_t);
    }
    if makelevel == 0 {
        if starting_directory.is_null() {
            if entering != 0 {
                fmt = b"%s: Entering an unknown directory\n\0" as *const u8
                    as *const ::core::ffi::c_char;
            } else {
                fmt = b"%s: Leaving an unknown directory\n\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
        } else if entering != 0 {
            fmt = b"%s: Entering directory '%s'\n\0" as *const u8 as *const ::core::ffi::c_char;
        } else {
            fmt = b"%s: Leaving directory '%s'\n\0" as *const u8 as *const ::core::ffi::c_char;
        }
    } else if starting_directory.is_null() {
        if entering != 0 {
            fmt = b"%s[%u]: Entering an unknown directory\n\0" as *const u8
                as *const ::core::ffi::c_char;
        } else {
            fmt = b"%s[%u]: Leaving an unknown directory\n\0" as *const u8
                as *const ::core::ffi::c_char;
        }
    } else if entering != 0 {
        fmt = b"%s[%u]: Entering directory '%s'\n\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        fmt = b"%s[%u]: Leaving directory '%s'\n\0" as *const u8 as *const ::core::ffi::c_char;
    }
    need = need.wrapping_add(strlen(fmt) as size_t);
    if need > len {
        buf = xrealloc(buf as *mut ::core::ffi::c_void, need) as *mut ::core::ffi::c_char;
        len = need;
    }
    p = buf;
    if print_data_base_flag != 0 {
        let fresh0 = p;
        p = p.offset(1 as ::core::ffi::c_int as isize);
        *fresh0 = '#' as i32 as ::core::ffi::c_char;
        let fresh1 = p;
        p = p.offset(1 as ::core::ffi::c_int as isize);
        *fresh1 = ' ' as i32 as ::core::ffi::c_char;
    }
    if makelevel == 0 {
        if starting_directory.is_null() {
            sprintf(p, fmt, program);
        } else {
            sprintf(p, fmt, program, starting_directory);
        }
    } else if starting_directory.is_null() {
        sprintf(p, fmt, program, makelevel);
    } else {
        sprintf(p, fmt, program, makelevel, starting_directory);
    }
    _outputs(
        ::core::ptr::null_mut::<output>(),
        0,
        buf,
    );
    1
}
#[no_mangle]
pub unsafe extern "C" fn pump_from_tmp(mut from: ::core::ffi::c_int, mut to: *mut FILE) {
    static mut buffer: [::core::ffi::c_char; 8192] = [0; 8192];
    if lseek(from, 0 as __off_t, SEEK_SET) == -(1 as ::core::ffi::c_int) as __off_t {
        perror(b"lseek()\0" as *const u8 as *const ::core::ffi::c_char);
    }
    loop {
        let mut len: ::core::ffi::c_int = 0;
        loop {
            len = read(
                from,
                &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as size_t,
            ) as ::core::ffi::c_int;
            if !(len == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                break;
            }
        }
        if len < 0 {
            perror(b"read()\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if len <= 0 {
            break;
        }
        if fwrite(
            &raw mut buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            len as size_t,
            1,
            to,
        ) < 1
        {
            perror(b"fwrite()\0" as *const u8 as *const ::core::ffi::c_char);
            break;
        } else {
            fflush(to);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn output_tmpfd() -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = get_tmpfd(::core::ptr::null_mut::<*mut ::core::ffi::c_char>());
    fd_set_append(fd);
    fd
}
#[no_mangle]
pub unsafe extern "C" fn setup_tmpfile(mut out: *mut output) {
    let mut current_block: u64;
    static mut in_setup: ::core::ffi::c_uint = 0;
    let mut io_state: ::core::ffi::c_uint = 0;
    if in_setup != 0 {
        return;
    }
    in_setup = 1;
    io_state = check_io_state();
    if !(io_state & (0x8 as ::core::ffi::c_int | 0x10 as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0)
    {
        perror_with_name(
            b"output-sync suppressed: \0" as *const u8 as *const ::core::ffi::c_char,
            b"stderr\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        if io_state & 0x8 as ::core::ffi::c_uint != 0 {
            let mut fd: ::core::ffi::c_int = output_tmpfd();
            if fd < 0 {
                current_block = 2479664526570923066;
            } else {
                fd_noinherit(fd);
                (*out).out = fd;
                current_block = 3276175668257526147;
            }
        } else {
            current_block = 3276175668257526147;
        }
        match current_block {
            2479664526570923066 => {}
            _ => {
                if io_state & 0x10 as ::core::ffi::c_uint != 0 {
                    if (*out).out != OUTPUT_NONE
                        && io_state & 0x2 as ::core::ffi::c_uint != 0
                    {
                        (*out).err = (*out).out;
                        current_block = 9606288038608642794;
                    } else {
                        let mut fd_0: ::core::ffi::c_int = output_tmpfd();
                        if fd_0 < 0 {
                            current_block = 2479664526570923066;
                        } else {
                            fd_noinherit(fd_0);
                            (*out).err = fd_0;
                            current_block = 9606288038608642794;
                        }
                    }
                } else {
                    current_block = 9606288038608642794;
                }
                match current_block {
                    2479664526570923066 => {}
                    _ => {
                        in_setup = 0;
                        return;
                    }
                }
            }
        }
    }
    error(
        ::core::ptr::null_mut::<Floc>(),
        0,
        b"cannot open output-sync lock file: suppressing output-sync\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    output_close(out);
    output_sync = OUTPUT_SYNC_NONE;
    osync_clear();
    in_setup = 0;
}
#[no_mangle]
pub unsafe extern "C" fn output_dump(mut out: *mut output) {
    let mut outfd_not_empty: ::core::ffi::c_int = ((*out).out != OUTPUT_NONE
        && lseek((*out).out, 0 as __off_t, SEEK_END) > 0 as __off_t)
        as ::core::ffi::c_int;
    let mut errfd_not_empty: ::core::ffi::c_int = ((*out).err != OUTPUT_NONE
        && lseek((*out).err, 0 as __off_t, SEEK_END) > 0 as __off_t)
        as ::core::ffi::c_int;
    if outfd_not_empty != 0 || errfd_not_empty != 0 {
        let mut traced: ::core::ffi::c_int = 0;
        if osync_acquire() == 0 {
            error(
                ::core::ptr::null_mut::<Floc>(),
                0,
                b"warning: cannot acquire output lock: disabling output sync\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            osync_clear();
        }
        if output_sync != OUTPUT_SYNC_RECURSE && should_print_dir() != 0 {
            traced = log_working_directory(1);
        }
        if outfd_not_empty != 0 {
            pump_from_tmp((*out).out, stdout);
        }
        if errfd_not_empty != 0 && (*out).err != (*out).out {
            pump_from_tmp((*out).err, stderr);
        }
        if traced != 0 {
            log_working_directory(0);
        }
        osync_release();
        if (*out).out != OUTPUT_NONE {
            let mut e: ::core::ffi::c_int = 0;
            lseek((*out).out, 0 as __off_t, SEEK_SET);
            loop {
                e = ftruncate((*out).out, 0 as __off_t);
                if !(e == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                    break;
                }
            }
        }
        if (*out).err != OUTPUT_NONE && (*out).err != (*out).out {
            let mut e_0: ::core::ffi::c_int = 0;
            lseek((*out).err, 0 as __off_t, SEEK_SET);
            loop {
                e_0 = ftruncate((*out).err, 0 as __off_t);
                if !(e_0 == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                    break;
                }
            }
        }
    }
}
static mut stdout_flags: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut stderr_flags: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub unsafe extern "C" fn output_init(mut out: *mut output) {
    if !out.is_null() {
        (*out).err = OUTPUT_NONE;
        (*out).out = (*out).err;
        (*out).set_syncout(
            (output_sync != 0) as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        return;
    }
    stdout_flags = fd_set_append(fileno(stdout));
    stderr_flags = fd_set_append(fileno(stderr));
}
#[no_mangle]
pub unsafe extern "C" fn output_close(mut out: *mut output) {
    if out.is_null() {
        if stdio_traced != 0 {
            log_working_directory(0);
        }
        fd_reset_append(fileno(stdout), stdout_flags);
        fd_reset_append(fileno(stderr), stderr_flags);
        return;
    }
    output_dump(out);
    if (*out).out >= 0 {
        close((*out).out);
    }
    if (*out).err >= 0 && (*out).err != (*out).out {
        close((*out).err);
    }
    output_init(out);
}
#[no_mangle]
pub unsafe extern "C" fn output_start() {
    if !output_context.is_null() && (*output_context).syncout() as ::core::ffi::c_int != 0 {
        if !((*output_context).out >= 0
            || (*output_context).err >= 0)
        {
            setup_tmpfile(output_context);
        }
    }
    if output_sync == OUTPUT_SYNC_NONE || output_sync == OUTPUT_SYNC_RECURSE {
        if stdio_traced == 0 && should_print_dir() != 0 {
            stdio_traced = log_working_directory(1) as ::core::ffi::c_uint;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn outputs(
    mut is_err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    if msg.is_null() || *msg as ::core::ffi::c_int == 0 {
        return;
    }
    output_start();
    _outputs(output_context, is_err, msg);
}
static mut fmtbuf: fmtstring = fmtstring {
    buffer: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    size: 0,
};
#[no_mangle]
pub unsafe extern "C" fn get_buffer(mut need: size_t) -> *mut ::core::ffi::c_char {
    if need > fmtbuf.size {
        fmtbuf.size = fmtbuf.size.wrapping_add(need.wrapping_mul(2));
        fmtbuf.buffer = xrealloc(fmtbuf.buffer as *mut ::core::ffi::c_void, fmtbuf.size)
            as *mut ::core::ffi::c_char;
    }
    *fmtbuf
        .buffer
        .offset(need.wrapping_sub(1) as isize) = 0;
    fmtbuf.buffer
}
#[no_mangle]
pub unsafe extern "C" fn message(
    mut prefix: ::core::ffi::c_int,
    mut len: size_t,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut start: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    len = (len as ::core::ffi::c_ulong).wrapping_add(
        strlen(fmt)
            .wrapping_add(strlen(program))
            .wrapping_add(INTSTR_LENGTH)
            .wrapping_add(4)
            .wrapping_add(1)
            .wrapping_add(1) as ::core::ffi::c_ulong,
    ) as size_t as size_t;
    p = get_buffer(len);
    start = p;
    if prefix != 0 {
        p = p.offset(
            (if makelevel == 0 {
                sprintf(
                    p,
                    b"%s: \0" as *const u8 as *const ::core::ffi::c_char,
                    program,
                )
            } else {
                sprintf(
                    p,
                    b"%s[%u]: \0" as *const u8 as *const ::core::ffi::c_char,
                    program,
                    makelevel,
                )
            }) as isize,
        );
    }
    args_0 = args.clone();
    vsprintf(p, fmt, args_0.as_va_list());
    strcat(p, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label: {
        if *start.offset(len.wrapping_sub(1) as isize) as ::core::ffi::c_int
            == 0
        {
        } else {
            __assert_fail(
                b"start[len-1] == '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/output.c\0" as *const u8 as *const ::core::ffi::c_char,
                440,
                b"void message(int, size_t, const char *, ...)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    outputs(0, start);
}
#[no_mangle]
pub unsafe extern "C" fn error(
    mut flocp: *const Floc,
    mut len: size_t,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut start: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    len = (len as ::core::ffi::c_ulong).wrapping_add(
        strlen(fmt)
            .wrapping_add(strlen(program))
            .wrapping_add(
                if !flocp.is_null() && !(*flocp).filenm.is_null() {
                    strlen((*flocp).filenm)
                } else {
                    0
                },
            )
            .wrapping_add(INTSTR_LENGTH)
            .wrapping_add(4)
            .wrapping_add(1)
            .wrapping_add(1) as ::core::ffi::c_ulong,
    ) as size_t as size_t;
    p = get_buffer(len);
    start = p;
    p = p.offset(
        (if !flocp.is_null() && !(*flocp).filenm.is_null() {
            sprintf(
                p,
                b"%s:%lu: \0" as *const u8 as *const ::core::ffi::c_char,
                (*flocp).filenm,
                (*flocp).lineno.wrapping_add((*flocp).offset),
            )
        } else if makelevel == 0 {
            sprintf(
                p,
                b"%s: \0" as *const u8 as *const ::core::ffi::c_char,
                program,
            )
        } else {
            sprintf(
                p,
                b"%s[%u]: \0" as *const u8 as *const ::core::ffi::c_char,
                program,
                makelevel,
            )
        }) as isize,
    );
    args_0 = args.clone();
    vsprintf(p, fmt, args_0.as_va_list());
    strcat(p, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    '_c2rust_label: {
        if *start.offset(len.wrapping_sub(1) as isize) as ::core::ffi::c_int
            == 0
        {
        } else {
            __assert_fail(
                b"start[len-1] == '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/output.c\0" as *const u8 as *const ::core::ffi::c_char,
                470,
                b"void error(const Floc *, size_t, const char *, ...)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    outputs(1, start);
}
#[no_mangle]
pub unsafe extern "C" fn fatal(
    mut flocp: *const Floc,
    mut len: size_t,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ! {
    let mut stop: *const ::core::ffi::c_char =
        b".  Stop.\n\0" as *const u8 as *const ::core::ffi::c_char;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut start: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    len = (len as ::core::ffi::c_ulong).wrapping_add(
        strlen(fmt)
            .wrapping_add(strlen(program))
            .wrapping_add(
                if !flocp.is_null() && !(*flocp).filenm.is_null() {
                    strlen((*flocp).filenm)
                } else {
                    0
                },
            )
            .wrapping_add(INTSTR_LENGTH)
            .wrapping_add(8)
            .wrapping_add(strlen(stop))
            .wrapping_add(1) as ::core::ffi::c_ulong,
    ) as size_t as size_t;
    p = get_buffer(len);
    start = p;
    p = p.offset(
        (if !flocp.is_null() && !(*flocp).filenm.is_null() {
            sprintf(
                p,
                b"%s:%lu: *** \0" as *const u8 as *const ::core::ffi::c_char,
                (*flocp).filenm,
                (*flocp).lineno.wrapping_add((*flocp).offset),
            )
        } else if makelevel == 0 {
            sprintf(
                p,
                b"%s: *** \0" as *const u8 as *const ::core::ffi::c_char,
                program,
            )
        } else {
            sprintf(
                p,
                b"%s[%u]: *** \0" as *const u8 as *const ::core::ffi::c_char,
                program,
                makelevel,
            )
        }) as isize,
    );
    args_0 = args.clone();
    vsprintf(p, fmt, args_0.as_va_list());
    strcat(p, stop);
    '_c2rust_label: {
        if *start.offset(len.wrapping_sub(1) as isize) as ::core::ffi::c_int
            == 0
        {
        } else {
            __assert_fail(
                b"start[len-1] == '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/output.c\0" as *const u8 as *const ::core::ffi::c_char,
                502,
                b"void fatal(const Floc *, size_t, const char *, ...)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    outputs(1, start);
    die(MAKE_FAILURE);
}
#[no_mangle]
pub unsafe extern "C" fn format(
    mut prefix: *const ::core::ffi::c_char,
    mut len: size_t,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> *mut ::core::ffi::c_char {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut plen: size_t = if !prefix.is_null() {
        strlen(prefix) as size_t
    } else {
        0
    };
    let mut start: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    len = len.wrapping_add(
        strlen(fmt)
            .wrapping_add(plen as size_t)
            .wrapping_add(1) as size_t,
    );
    p = get_buffer(len);
    start = p;
    if plen != 0 {
        p = mempcpy(
            p as *mut ::core::ffi::c_void,
            prefix as *const ::core::ffi::c_void,
            plen as size_t,
        ) as *mut ::core::ffi::c_char;
    }
    args_0 = args.clone();
    vsprintf(p, fmt, args_0.as_va_list());
    start
}
#[no_mangle]
pub unsafe extern "C" fn perror_with_name(
    mut str: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) {
    let mut err: *const ::core::ffi::c_char = strerror(*__errno_location());
    error(
        ::core::ptr::null_mut::<Floc>(),
        (strlen(str) as size_t)
            .wrapping_add(strlen(name) as size_t)
            .wrapping_add(strlen(err) as size_t),
        b"%s%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        str,
        name,
        err,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pfatal_with_name(mut name: *const ::core::ffi::c_char) -> ! {
    let mut err: *const ::core::ffi::c_char = strerror(*__errno_location());
    fatal(
        ::core::ptr::null_mut::<Floc>(),
        (strlen(name) as size_t).wrapping_add(strlen(err) as size_t),
        b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        err,
    );
}
#[no_mangle]
pub unsafe extern "C" fn out_of_memory() -> ! {
    writebuf(
        fileno(stdout),
        program as *const ::core::ffi::c_void,
        strlen(program) as size_t,
    );
    writebuf(
        fileno(stdout),
        b": *** virtual memory exhausted\n\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t).wrapping_sub(1),
    );
    exit(MAKE_FAILURE);
}
