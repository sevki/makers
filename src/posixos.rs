use libc::{__errno_location, close, dup, fcntl, free, open, perror, pipe, printf, sprintf, sscanf, strcmp, strerror, unlink};
use crate::stdio::{FILE};
pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __pid_t, __sig_atomic_t, __syscall_slong_t, __time_t, __uid_t, mode_t, pid_t, sig_atomic_t,
    size_t, ssize_t, uintmax_t,
};
extern "C" {
    fn pselect(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn mkfifo(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn tmpfile() -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn error(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn fatal(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn pfatal_with_name(_: *const ::core::ffi::c_char) -> !;
    fn perror_with_name(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char);
    fn make_pid() -> pid_t;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xstrdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn get_tmpdir() -> *const ::core::ffi::c_char;
    fn get_tmpfd(_: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut handling_fatal_signal: sig_atomic_t;
    static mut db_level: ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub use crate::sys_stat::timespec;
pub type __fd_mask = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub use crate::sys_stat::stat;
use crate::floc::Floc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: ::core::ffi::c_short,
    pub l_whence: ::core::ffi::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
pub const js_none: js_type = 0;
pub type js_type = ::core::ffi::c_uint;
pub const js_fifo: js_type = 2;
pub const js_pipe: js_type = 1;
pub const __NFDBITS: ::core::ffi::c_int =
    8 * ::core::mem::size_of::<__fd_mask>() as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4;
pub const EBADF: ::core::ffi::c_int = 9;
pub const EAGAIN: ::core::ffi::c_int = 11;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INTSTR_LENGTH: usize = (53 as usize)
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22 as usize)
    .wrapping_add(3 as usize);
pub const F_SETLKW64: ::core::ffi::c_int = 7;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const F_SETLKW: ::core::ffi::c_int = F_SETLKW64;
pub const F_GETFD: ::core::ffi::c_int = 1;
pub const F_GETFL: ::core::ffi::c_int = 3;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1;
pub const F_WRLCK: ::core::ffi::c_int = 1;
pub const F_UNLCK: ::core::ffi::c_int = 2;
pub const SEEK_SET: ::core::ffi::c_int = 0;
pub const IO_UNKNOWN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const IO_COMBINED_OUTERR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const IO_STDIN_OK: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const IO_STDOUT_OK: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const IO_STDERR_OK: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn check_io_state() -> ::core::ffi::c_uint {
    static mut state: ::core::ffi::c_uint = IO_UNKNOWN as ::core::ffi::c_uint;
    if state != IO_UNKNOWN as ::core::ffi::c_uint {
        return state;
    }
    if fcntl(fileno(stdin), F_GETFD) != -1 || *__errno_location() != EBADF {
        state |= IO_STDIN_OK as ::core::ffi::c_uint;
    }
    if fcntl(fileno(stdout), F_GETFD) != -1 || *__errno_location() != EBADF
    {
        state |= IO_STDOUT_OK as ::core::ffi::c_uint;
    }
    if fcntl(fileno(stderr), F_GETFD) != -1 || *__errno_location() != EBADF
    {
        state |= IO_STDERR_OK as ::core::ffi::c_uint;
    }
    if state & (0x8 as ::core::ffi::c_int | 0x10 as ::core::ffi::c_int) as ::core::ffi::c_uint
        == (0x8 as ::core::ffi::c_int | 0x10 as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        let mut stbuf_o: stat = unsafe { ::core::mem::zeroed() };
        let mut stbuf_e: stat = unsafe { ::core::mem::zeroed() };
        if fstat(fileno(stdout), &raw mut stbuf_o) == 0
            && fstat(fileno(stderr), &raw mut stbuf_e) == 0
            && stbuf_o.st_dev == stbuf_e.st_dev
            && stbuf_o.st_ino == stbuf_e.st_ino
        {
            state |= IO_COMBINED_OUTERR as ::core::ffi::c_uint;
        }
    }
    state
}
pub const FIFO_PREFIX: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"fifo:\0") };
static mut job_root: ::core::ffi::c_uchar = 0;
static mut job_fds: [::core::ffi::c_int; 2] =
    [-1, -1];
static mut job_rfd: ::core::ffi::c_int = -1;
static mut token: ::core::ffi::c_char = '+' as i32 as ::core::ffi::c_char;
static mut js_type: js_type = js_none;
static mut fifo_name: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
unsafe fn make_job_rfd() -> ::core::ffi::c_int {
    0
}

/// Retry-on-EINTR wrapper around fcntl. Returns the result on success or after a
/// non-EINTR failure.
unsafe fn fcntl_retry(fd: ::core::ffi::c_int, cmd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    loop {
        let r = fcntl(fd, cmd);
        if !(r == -1 && *__errno_location() == EINTR) {
            return r;
        }
    }
}

unsafe fn fcntl_set_retry(
    fd: ::core::ffi::c_int,
    cmd: ::core::ffi::c_int,
    arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    loop {
        let r = fcntl(fd, cmd, arg);
        if !(r == -1 && *__errno_location() == EINTR) {
            return r;
        }
    }
}

unsafe fn force_blocking(fd: ::core::ffi::c_int, blocking: ::core::ffi::c_int) {
    let flags = fcntl_retry(fd, F_GETFL);
    if flags < 0 {
        return;
    }
    let new_flags = if blocking != 0 {
        flags & !O_NONBLOCK
    } else {
        flags | O_NONBLOCK
    };
    // F_SETFL = 4. Not declared elsewhere in this file; F_GETFL is.
    if fcntl_set_retry(fd, 4, new_flags) < 0 {
        pfatal_with_name(b"fcntl(O_NONBLOCK)\0" as *const u8 as *const ::core::ffi::c_char);
    }
}

unsafe fn set_blocking(fd: ::core::ffi::c_int, blocking: ::core::ffi::c_int) {
    force_blocking(fd, blocking);
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_setup(
    slots: ::core::ffi::c_int,
    style: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut r: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    job_root = 1;
    if style.is_null()
        || strcmp(style, b"fifo\0" as *const u8 as *const ::core::ffi::c_char)
            == 0
    {
        let tmpdir: *const ::core::ffi::c_char = get_tmpdir();
        fifo_name = xmalloc(
            (strlen(tmpdir) as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t)
                        .wrapping_sub(1),
                )
                .wrapping_add(INTSTR_LENGTH)
                .wrapping_add(2),
        ) as *mut ::core::ffi::c_char;
        sprintf(
            fifo_name,
            b"%s/GmFIFO%03lld\0" as *const u8 as *const ::core::ffi::c_char,
            tmpdir,
            make_pid() as ::core::ffi::c_longlong,
        );
        loop {
            r = mkfifo(fifo_name, 0o600 as __mode_t);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            perror_with_name(
                b"jobserver mkfifo: \0" as *const u8 as *const ::core::ffi::c_char,
                fifo_name,
            );
            free(fifo_name as *mut ::core::ffi::c_void);
            fifo_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            loop {
                job_fds[0 as usize] = open(
                    fifo_name,
                    0 | 0o4000 as ::core::ffi::c_int,
                );
                if !(job_fds[0 as usize] == -1
                    && *__errno_location() == EINTR)
                {
                    break;
                }
            }
            if job_fds[0 as usize] < 0 {
                fatal(
                    ::core::ptr::null_mut::<Floc>(),
                    (strlen(fifo_name) as size_t)
                        .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                    b"cannot open jobserver %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    fifo_name,
                    strerror(*__errno_location()),
                );
            }
            loop {
                job_fds[1 as usize] =
                    open(fifo_name, 0o1 as ::core::ffi::c_int);
                if !(job_fds[1 as usize] == -1
                    && *__errno_location() == EINTR)
                {
                    break;
                }
            }
            if job_fds[0 as usize] < 0 {
                fatal(
                    ::core::ptr::null_mut::<Floc>(),
                    (strlen(fifo_name) as size_t)
                        .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                    b"cannot open jobserver %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    fifo_name,
                    strerror(*__errno_location()),
                );
            }
            js_type = js_fifo;
        }
    }
    if js_type as ::core::ffi::c_uint == js_none as ::core::ffi::c_int as ::core::ffi::c_uint {
        if !style.is_null()
            && strcmp(style, b"pipe\0" as *const u8 as *const ::core::ffi::c_char)
                != 0
        {
            fatal(
                ::core::ptr::null_mut::<Floc>(),
                strlen(style) as size_t,
                b"unknown jobserver auth style '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                style,
            );
        }
        loop {
            r = pipe(&raw mut job_fds as *mut ::core::ffi::c_int);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            pfatal_with_name(b"creating jobs pipe\0" as *const u8 as *const ::core::ffi::c_char);
        }
        js_type = js_pipe;
    }
    fd_noinherit(job_fds[0 as usize]);
    fd_noinherit(job_fds[1 as usize]);
    if make_job_rfd() < 0 {
        pfatal_with_name(b"duping jobs pipe\0" as *const u8 as *const ::core::ffi::c_char);
    }
    force_blocking(
        job_fds[1 as usize],
        0,
    );
    k = 0;
    while k < slots {
        loop {
            r = write(
                job_fds[1 as usize],
                &raw mut token as *const ::core::ffi::c_void,
                1,
            ) as ::core::ffi::c_int;
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r != 1 {
            if *__errno_location() != EAGAIN {
                pfatal_with_name(
                    b"init jobserver pipe\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            fatal(
                ::core::ptr::null_mut::<Floc>(),
                INTSTR_LENGTH.wrapping_mul(2),
                b"requested job count (%d) is larger than system limit (%d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                slots + 1,
                k,
            );
        }
        k += 1;
    }
    force_blocking(
        job_fds[1 as usize],
        1,
    );
    set_blocking(
        job_fds[0 as usize],
        0,
    );
    1
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_parse_auth(
    auth: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut rfd: ::core::ffi::c_int = 0;
    let mut wfd: ::core::ffi::c_int = 0;
    if strncmp(
        auth,
        FIFO_PREFIX.as_ptr(),
        (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
    ) == 0
    {
        fifo_name = xstrdup(auth.offset(
            (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize).wrapping_sub(1 as usize)
                as isize,
        ));
        loop {
            job_fds[0 as usize] = open(fifo_name, 0);
            if !(job_fds[0 as usize] == -1
                && *__errno_location() == EINTR)
            {
                break;
            }
        }
        if job_fds[0 as usize] < 0 {
            error(
                ::core::ptr::null_mut::<Floc>(),
                (strlen(fifo_name) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"cannot open jobserver %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                fifo_name,
                strerror(*__errno_location()),
            );
            return 0;
        }
        loop {
            job_fds[1 as usize] = open(fifo_name, 0o1 as ::core::ffi::c_int);
            if !(job_fds[1 as usize] == -1
                && *__errno_location() == EINTR)
            {
                break;
            }
        }
        if job_fds[1 as usize] < 0 {
            error(
                ::core::ptr::null_mut::<Floc>(),
                (strlen(fifo_name) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                b"cannot open jobserver %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                fifo_name,
                strerror(*__errno_location()),
            );
            return 0;
        }
        js_type = js_fifo;
    } else if sscanf(
        auth,
        b"%d,%d\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut rfd,
        &raw mut wfd,
    ) == 2
    {
        if rfd == -2 || wfd == -2 {
            return 0;
        }
        if !(fcntl(rfd, F_GETFD) != -1)
            || !(fcntl(wfd, F_GETFD) != -1)
        {
            return 0;
        }
        job_fds[0 as usize] = rfd;
        job_fds[1 as usize] = wfd;
        js_type = js_pipe;
    } else {
        error(
            ::core::ptr::null_mut::<Floc>(),
            strlen(auth) as size_t,
            b"invalid --jobserver-auth string '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            auth,
        );
        return 0;
    }
    if make_job_rfd() < 0 {
        if *__errno_location() != EBADF {
            pfatal_with_name(b"jobserver readfd\0" as *const u8 as *const ::core::ffi::c_char);
        }
        jobserver_clear();
        return 0;
    }
    set_blocking(
        job_fds[0 as usize],
        0,
    );
    fd_noinherit(job_fds[0 as usize]);
    fd_noinherit(job_fds[1 as usize]);
    1
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_get_auth() -> *mut ::core::ffi::c_char {
    let mut auth: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if js_type as ::core::ffi::c_uint == js_fifo as ::core::ffi::c_int as ::core::ffi::c_uint {
        auth = xmalloc(
            (strlen(fifo_name) as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t)
                        .wrapping_sub(1),
                )
                .wrapping_add(1),
        ) as *mut ::core::ffi::c_char;
        sprintf(
            auth,
            b"fifo:%s\0" as *const u8 as *const ::core::ffi::c_char,
            fifo_name,
        );
    } else {
        auth = xmalloc(
            INTSTR_LENGTH
                .wrapping_mul(2)
                .wrapping_add(2),
        ) as *mut ::core::ffi::c_char;
        sprintf(
            auth,
            b"%d,%d\0" as *const u8 as *const ::core::ffi::c_char,
            job_fds[0 as usize],
            job_fds[1 as usize],
        );
    }
    auth
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_get_invalid_auth() -> *const ::core::ffi::c_char {
    if js_type as ::core::ffi::c_uint == js_fifo as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    b" --jobserver-auth=-2,-2\0" as *const u8 as *const ::core::ffi::c_char
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_enabled() -> ::core::ffi::c_uint {
    (js_type as ::core::ffi::c_uint != js_none as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int as ::core::ffi::c_uint
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_clear() {
    if job_fds[0 as usize] >= 0 {
        close(job_fds[0 as usize]);
    }
    if job_fds[1 as usize] >= 0 {
        close(job_fds[1 as usize]);
    }
    if job_rfd >= 0 {
        close(job_rfd);
    }
    job_rfd = -1;
    job_fds[1 as usize] = job_rfd;
    job_fds[0 as usize] = job_fds[1 as usize];
    if !fifo_name.is_null() {
        if job_root != 0 {
            let mut r: ::core::ffi::c_int = 0;
            loop {
                r = unlink(fifo_name);
                if !(r == -1 && *__errno_location() == EINTR) {
                    break;
                }
            }
        }
        if handling_fatal_signal == 0 {
            free(fifo_name as *mut ::core::ffi::c_void);
            fifo_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    js_type = js_none;
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_release(is_fatal: ::core::ffi::c_int) {
    let mut r: ::core::ffi::c_int = 0;
    loop {
        r = write(
            job_fds[1 as usize],
            &raw mut token as *const ::core::ffi::c_void,
            1,
        ) as ::core::ffi::c_int;
        if !(r == -1 && *__errno_location() == EINTR) {
            break;
        }
    }
    if r != 1 {
        if is_fatal != 0 {
            pfatal_with_name(b"write jobserver\0" as *const u8 as *const ::core::ffi::c_char);
        }
        perror_with_name(
            b"write\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_acquire_all() -> ::core::ffi::c_uint {
    let mut r: ::core::ffi::c_int = 0;
    let mut tokens: ::core::ffi::c_uint = 0;
    set_blocking(
        job_fds[0 as usize],
        1,
    );
    close(job_fds[1 as usize]);
    job_fds[1 as usize] = -1;
    loop {
        let mut intake: ::core::ffi::c_char = 0;
        loop {
            r = read(
                job_fds[0 as usize],
                &raw mut intake as *mut ::core::ffi::c_void,
                1,
            ) as ::core::ffi::c_int;
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r != 1 {
            break;
        }
        tokens = tokens.wrapping_add(1);
    }
    if 0x4 as ::core::ffi::c_int & db_level != 0 {
        printf(
            b"Acquired all %u jobserver tokens.\n\0" as *const u8 as *const ::core::ffi::c_char,
            tokens,
        );
        fflush(stdout);
    }
    jobserver_clear();
    tokens
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_pre_child(recursive: ::core::ffi::c_int) {
    if recursive != 0
        && js_type as ::core::ffi::c_uint == js_pipe as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fd_inherit(job_fds[0 as usize]);
        fd_inherit(job_fds[1 as usize]);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_post_child(recursive: ::core::ffi::c_int) {
    if recursive != 0
        && js_type as ::core::ffi::c_uint == js_pipe as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fd_noinherit(job_fds[0 as usize]);
        fd_noinherit(job_fds[1 as usize]);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_signal() {
    if job_rfd >= 0 {
        close(job_rfd);
        job_rfd = -1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_pre_acquire() {
    if job_rfd < 0
        && job_fds[0 as usize] >= 0
        && make_job_rfd() < 0
    {
        pfatal_with_name(b"duping jobs pipe\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_acquire(timeout: ::core::ffi::c_int) -> ::core::ffi::c_uint {
    let mut spec: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut specp: *mut timespec = ::core::ptr::null_mut::<timespec>();
    let mut empty: sigset_t = __sigset_t { __val: [0; 16] };
    sigemptyset(&raw mut empty);
    if timeout != 0 {
        spec.tv_sec = 1 as __time_t;
        spec.tv_nsec = 0 as __syscall_slong_t;
        specp = &raw mut spec;
    }
    loop {
        let mut readfds: fd_set = fd_set { fds_bits: [0; 16] };
        let mut r: ::core::ffi::c_int = 0;
        let mut intake: ::core::ffi::c_char = 0;
        let mut __i: ::core::ffi::c_uint = 0;
        let mut __arr: *mut fd_set = &raw mut readfds;
        __i = 0;
        while (__i as usize)
            < (::core::mem::size_of::<fd_set>() as usize)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize)
        {
            (*__arr).fds_bits[__i as usize] = 0 as __fd_mask;
            __i = __i.wrapping_add(1);
        }
        readfds.fds_bits[(job_fds[0 as usize] / __NFDBITS) as usize] |=
            ((1) << job_fds[0 as usize] % __NFDBITS)
                as __fd_mask;
        r = pselect(
            job_fds[0 as usize] + 1,
            &raw mut readfds,
            ::core::ptr::null_mut::<fd_set>(),
            ::core::ptr::null_mut::<fd_set>(),
            specp,
            &raw mut empty,
        );
        if r < 0 {
            match *__errno_location() {
                EINTR => return 0,
                EBADF => {
                    fatal(
                        ::core::ptr::null_mut::<Floc>(),
                        0,
                        b"job server shut down\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                _ => {
                    pfatal_with_name(
                        b"pselect jobs pipe\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        if r == 0 {
            return 0;
        }
        loop {
            r = read(
                job_fds[0 as usize],
                &raw mut intake as *mut ::core::ffi::c_void,
                1,
            ) as ::core::ffi::c_int;
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            if *__errno_location() == EAGAIN {
                continue;
            }
            pfatal_with_name(b"read jobs pipe\0" as *const u8 as *const ::core::ffi::c_char);
        } else {
            return (r > 0) as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
    }
}
pub const MUTEX_PREFIX: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"fnm:\0") };
static mut osync_handle: ::core::ffi::c_int = -1;
static mut osync_tmpfile: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut sync_root: ::core::ffi::c_uint = 0;
#[no_mangle]
pub unsafe extern "C" fn osync_enabled() -> ::core::ffi::c_uint {
    (osync_handle >= 0) as ::core::ffi::c_int as ::core::ffi::c_uint
}
#[no_mangle]
pub unsafe extern "C" fn osync_setup() {
    osync_handle = get_tmpfd(&raw mut osync_tmpfile);
    fd_noinherit(osync_handle);
    sync_root = 1;
}
#[no_mangle]
pub unsafe extern "C" fn osync_get_mutex() -> *mut ::core::ffi::c_char {
    let mut mutex: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if osync_enabled() != 0 {
        mutex = xmalloc(
            (strlen(osync_tmpfile) as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t)
                        .wrapping_sub(1),
                )
                .wrapping_add(1),
        ) as *mut ::core::ffi::c_char;
        sprintf(
            mutex,
            b"fnm:%s\0" as *const u8 as *const ::core::ffi::c_char,
            osync_tmpfile,
        );
    }
    mutex
}
#[no_mangle]
pub unsafe extern "C" fn osync_parse_mutex(
    mutex: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    if strncmp(
        mutex,
        MUTEX_PREFIX.as_ptr(),
        (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1),
    ) != 0
    {
        error(
            ::core::ptr::null_mut::<Floc>(),
            strlen(mutex) as size_t,
            b"invalid --sync-mutex string '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            mutex,
        );
        return 0;
    }
    free(osync_tmpfile as *mut ::core::ffi::c_void);
    osync_tmpfile = xstrdup(mutex.offset(
        (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize).wrapping_sub(1 as usize)
            as isize,
    ));
    loop {
        osync_handle = open(osync_tmpfile, 0o1 as ::core::ffi::c_int);
        if !(osync_handle == -1 && *__errno_location() == EINTR) {
            break;
        }
    }
    if osync_handle < 0 {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            (strlen(osync_tmpfile) as size_t)
                .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
            b"cannot open output sync mutex %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
            osync_tmpfile,
            strerror(*__errno_location()),
        );
    }
    fd_noinherit(osync_handle);
    1
}
#[no_mangle]
pub unsafe extern "C" fn osync_clear() {
    if osync_handle >= 0 {
        close(osync_handle);
        osync_handle = -1;
    }
    if sync_root != 0 && !osync_tmpfile.is_null() {
        let mut r: ::core::ffi::c_int = 0;
        loop {
            r = unlink(osync_tmpfile);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        free(osync_tmpfile as *mut ::core::ffi::c_void);
        osync_tmpfile = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn osync_acquire() -> ::core::ffi::c_uint {
    if osync_enabled() != 0 {
        let mut fl: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        fl.l_type = F_WRLCK as ::core::ffi::c_short;
        fl.l_whence = SEEK_SET as ::core::ffi::c_short;
        fl.l_start = 0 as __off_t;
        fl.l_len = 1 as __off_t;
        if fcntl(osync_handle, F_SETLKW, &raw mut fl) == -1 {
            perror(b"fcntl()\0" as *const u8 as *const ::core::ffi::c_char);
            return 0;
        }
    }
    1
}
#[no_mangle]
pub unsafe extern "C" fn osync_release() {
    if osync_enabled() != 0 {
        let mut fl: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        fl.l_type = F_UNLCK as ::core::ffi::c_short;
        fl.l_whence = SEEK_SET as ::core::ffi::c_short;
        fl.l_start = 0 as __off_t;
        fl.l_len = 1 as __off_t;
        if fcntl(osync_handle, F_SETLKW, &raw mut fl) == -1 {
            perror(b"fcntl()\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_bad_stdin() -> ::core::ffi::c_int {
    static mut bad_stdin: ::core::ffi::c_int = -1;
    if bad_stdin == -1 {
        let mut pd: [::core::ffi::c_int; 2] = [0; 2];
        if pipe(&raw mut pd as *mut ::core::ffi::c_int) == 0 {
            close(pd[1 as usize]);
            bad_stdin = pd[0 as usize];
            fd_noinherit(bad_stdin);
        }
    }
    bad_stdin
}
#[no_mangle]
pub unsafe extern "C" fn fd_inherit(fd: ::core::ffi::c_int) {
    let mut flags: ::core::ffi::c_int = 0;
    loop {
        flags = fcntl(fd, 1);
        if !(flags == -1 && *__errno_location() == EINTR) {
            break;
        }
    }
    if flags >= 0 {
        let mut r: ::core::ffi::c_int = 0;
        flags &= !FD_CLOEXEC;
        loop {
            r = fcntl(fd, 2, flags);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn fd_noinherit(fd: ::core::ffi::c_int) {
    let mut flags: ::core::ffi::c_int = 0;
    loop {
        flags = fcntl(fd, 1);
        if !(flags == -1 && *__errno_location() == EINTR) {
            break;
        }
    }
    if flags >= 0 {
        let mut r: ::core::ffi::c_int = 0;
        flags |= FD_CLOEXEC;
        loop {
            r = fcntl(fd, 2, flags);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn fd_set_append(fd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = -1;
    let mut stbuf: stat = unsafe { ::core::mem::zeroed() };
    if fstat(fd, &raw mut stbuf) == 0
        && stbuf.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
    {
        flags = fcntl(fd, F_GETFL, 0);
        if flags >= 0 {
            let mut r: ::core::ffi::c_int = 0;
            loop {
                r = fcntl(
                    fd,
                    4,
                    flags | 0o2000 as ::core::ffi::c_int,
                );
                if !(r == -1 && *__errno_location() == EINTR) {
                    break;
                }
            }
        }
    }
    flags
}
#[no_mangle]
pub unsafe extern "C" fn fd_reset_append(
    fd: ::core::ffi::c_int,
    flags: ::core::ffi::c_int,
) {
    if flags >= 0 {
        let mut r: ::core::ffi::c_int = 0;
        loop {
            r = fcntl(fd, 4, flags);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn os_anontmp() -> ::core::ffi::c_int {
    let tdir: *const ::core::ffi::c_char = get_tmpdir();
    let mut fd: ::core::ffi::c_int = -1;
    static mut tmpfile_works: ::core::ffi::c_uint = 1;
    if tmpfile_works != 0 {
        loop {
            fd = open(
                tdir,
                0o2 as ::core::ffi::c_int
                    | (0o20000000 as ::core::ffi::c_int | 0o200000 as ::core::ffi::c_int)
                    | 0o200 as ::core::ffi::c_int,
                0o600 as ::core::ffi::c_int,
            );
            if !(fd == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if fd >= 0 {
            return fd;
        }
        if 0x1 as ::core::ffi::c_int & db_level != 0 {
            printf(
                b"Cannot open '%s' with O_TMPFILE: %s.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                tdir,
                strerror(*__errno_location()),
            );
            fflush(stdout);
        }
        tmpfile_works = 0;
    }
    if *tdir as ::core::ffi::c_int
        == *(b"/tmp\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
        && (*tdir as ::core::ffi::c_int == 0
            || strcmp(
                tdir.offset(1 as isize),
                (b"/tmp\0" as *const u8 as *const ::core::ffi::c_char)
                    .offset(1 as isize),
            ) == 0)
    {
        let mask: mode_t = umask(0o77 as __mode_t) as mode_t;
        let mut tfile: *mut FILE = ::core::ptr::null_mut::<FILE>();
        loop {
            *__errno_location() = 0;
            tfile = tmpfile();
            if !(tfile.is_null() && *__errno_location() == EINTR) {
                break;
            }
        }
        if tfile.is_null() {
            error(
                ::core::ptr::null_mut::<Floc>(),
                strlen(strerror(*__errno_location())) as size_t,
                b"tmpfile: %s\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            return -1;
        }
        umask(mask as __mode_t);
        loop {
            fd = dup(fileno(tfile));
            if !(fd == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if fd < 0 {
            error(
                ::core::ptr::null_mut::<Floc>(),
                strlen(strerror(*__errno_location())) as size_t,
                b"dup: %s\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        fclose(tfile);
    }
    fd
}
