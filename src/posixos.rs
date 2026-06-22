//! POSIX-specific glue: jobserver (pipe/fifo token protocol), the
//! output-sync mutex, and fd helpers.
//!
//! Port of `posixos.c`. The jobserver state stays in module-level globals
//! with C-shaped accessors because `job.rs` and `main.rs` drive it through
//! the original entry points.

use ::core::ffi::{c_char, c_int, c_longlong, c_uint, c_void, CStr};
use ::core::ptr::{null, null_mut};

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

use libc::{
    __errno_location, close, dup, fcntl, flock, free, fstat, mkfifo, open, perror, pipe, printf,
    pselect, read, sigemptyset, sigset_t, sprintf, sscanf, strcmp, strerror, strlen, strncmp,
    timespec, tmpfile, umask, unlink, write, EAGAIN, EBADF, EINTR, FD_CLOEXEC, FD_SET, FD_ZERO,
    F_GETFD, F_GETFL, F_SETFD, F_SETFL, F_SETLKW, F_UNLCK, F_WRLCK, O_APPEND, O_EXCL, O_NONBLOCK,
    O_RDONLY, O_RDWR, O_TMPFILE, O_WRONLY, SEEK_SET, S_IFMT, S_IFREG,
};

use crate::commands::handling_fatal_signal;
use crate::ffi_types::mode_t;
use crate::floc::Floc;
use crate::make_main::db_level;
use crate::misc::{get_tmpdir, get_tmpfd, make_pid, xmalloc, xstrdup};
use crate::output::{error, fatal, perror_with_name, pfatal_with_name, FmtArg, INTSTR_LENGTH};
use crate::stdio::FILE;

extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fileno(stream: *mut FILE) -> c_int;
}

/// Stream fileno for `crate::stdio::FILE` streams (libc's `fileno` takes
/// `libc::FILE`, which is a distinct type).
unsafe fn stream_fd(stream: *mut FILE) -> c_int {
    fileno(stream)
}

/// `check_io_state` bits (see os.h).
pub const IO_UNKNOWN: c_int = 0x1;
pub const IO_COMBINED_OUTERR: c_int = 0x2;
pub const IO_STDIN_OK: c_int = 0x4;
pub const IO_STDOUT_OK: c_int = 0x8;
pub const IO_STDERR_OK: c_int = 0x10;

/// Which validity bits hold for stdin/stdout/stderr, computed once.
///
/// # Safety
/// Must run after stdio is initialized; reads the C stdio globals.
pub unsafe fn check_io_state() -> c_uint {
    static IO_STATE: AtomicU32 = AtomicU32::new(IO_UNKNOWN as c_uint);
    let mut state = IO_STATE.load(Ordering::Relaxed);
    if state != IO_UNKNOWN as c_uint {
        return state;
    }

    if fcntl(stream_fd(stdin), F_GETFD) != -1 || *__errno_location() != EBADF {
        state |= IO_STDIN_OK as c_uint;
    }
    if fcntl(stream_fd(stdout), F_GETFD) != -1 || *__errno_location() != EBADF {
        state |= IO_STDOUT_OK as c_uint;
    }
    if fcntl(stream_fd(stderr), F_GETFD) != -1 || *__errno_location() != EBADF {
        state |= IO_STDERR_OK as c_uint;
    }

    // If stdout and stderr are both usable, check whether they refer to the
    // same file.
    if state & (IO_STDOUT_OK | IO_STDERR_OK) as c_uint == (IO_STDOUT_OK | IO_STDERR_OK) as c_uint {
        let mut stbuf_o: libc::stat = ::core::mem::zeroed();
        let mut stbuf_e: libc::stat = ::core::mem::zeroed();
        if fstat(stream_fd(stdout), &mut stbuf_o) == 0
            && fstat(stream_fd(stderr), &mut stbuf_e) == 0
            && stbuf_o.st_dev == stbuf_e.st_dev
            && stbuf_o.st_ino == stbuf_e.st_ino
        {
            state |= IO_COMBINED_OUTERR as c_uint;
        }
    }

    IO_STATE.store(state, Ordering::Relaxed);
    state
}

// ---------------------------------------------------------------------------
// Jobserver.

const FIFO_PREFIX: &CStr = c"fifo:";

#[derive(Copy, Clone, PartialEq, Eq)]
enum JsType {
    None,
    Pipe,
    Fifo,
}

/// True in the process that created the jobserver (and so owns the fifo).
static JOB_ROOT: AtomicBool = AtomicBool::new(false);
/// The token pipe/fifo: `[read end, write end]`.
static mut job_fds: [c_int; 2] = [-1, -1];
/// A private dup of the read side (closed by a fatal signal to wake us).
static mut job_rfd: c_int = -1;
/// The token character written for each available job slot.
static mut token: c_char = b'+' as c_char;
static mut js_type: JsType = JsType::None;
static mut fifo_name: *mut c_char = null_mut();

/// On POSIX with pselect there is no need for a separate read dup; the
/// blocking read is interruptible already.
unsafe fn make_job_rfd() -> c_int {
    0
}

/// Retry-on-EINTR wrapper around `fcntl(fd, cmd)`.
unsafe fn fcntl_retry(fd: c_int, cmd: c_int) -> c_int {
    loop {
        let r = fcntl(fd, cmd);
        if !(r == -1 && *__errno_location() == EINTR) {
            return r;
        }
    }
}

/// Retry-on-EINTR wrapper around `fcntl(fd, cmd, arg)`.
unsafe fn fcntl_set_retry(fd: c_int, cmd: c_int, arg: c_int) -> c_int {
    loop {
        let r = fcntl(fd, cmd, arg);
        if !(r == -1 && *__errno_location() == EINTR) {
            return r;
        }
    }
}

/// Set or clear `O_NONBLOCK` on `fd`, dying on failure.
unsafe fn set_blocking(fd: c_int, blocking: bool) {
    let flags = fcntl_retry(fd, F_GETFL);
    if flags < 0 {
        return;
    }
    let new_flags = if blocking {
        flags & !O_NONBLOCK
    } else {
        flags | O_NONBLOCK
    };
    if fcntl_set_retry(fd, F_SETFL, new_flags) < 0 {
        pfatal_with_name(c"fcntl(O_NONBLOCK)".as_ptr());
    }
}

/// Create the jobserver (fifo if possible, else an anonymous pipe) with
/// `slots` available tokens. Returns 1.
///
/// # Safety
/// `style` must be null or a valid NUL-terminated string; must run
/// single-threaded during startup.
pub unsafe fn jobserver_setup(slots: c_int, style: *const c_char) -> c_uint {
    let mut r: c_int;

    JOB_ROOT.store(true, Ordering::Relaxed);

    if style.is_null() || strcmp(style, c"fifo".as_ptr()) == 0 {
        let tmpdir = get_tmpdir();
        fifo_name = xmalloc(strlen(tmpdir) + FIFO_PREFIX.to_bytes().len() + 1 + INTSTR_LENGTH + 2)
            as *mut c_char;
        sprintf(
            fifo_name,
            c"%s/GmFIFO%03lld".as_ptr(),
            tmpdir,
            make_pid() as c_longlong,
        );

        loop {
            r = mkfifo(fifo_name, 0o600);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            perror_with_name(c"jobserver mkfifo: ".as_ptr(), fifo_name);
            free(fifo_name as *mut c_void);
            fifo_name = null_mut();
        } else {
            loop {
                job_fds[0] = open(fifo_name, O_NONBLOCK);
                if !(job_fds[0] == -1 && *__errno_location() == EINTR) {
                    break;
                }
            }
            if job_fds[0] < 0 {
                fatal(
                    null::<Floc>(),
                    c"cannot open jobserver %s: %s".as_ptr(),
                    &[
                        FmtArg::Str(fifo_name),
                        FmtArg::Str(strerror(*__errno_location())),
                    ],
                );
            }
            loop {
                job_fds[1] = open(fifo_name, O_WRONLY);
                if !(job_fds[1] == -1 && *__errno_location() == EINTR) {
                    break;
                }
            }
            if job_fds[0] < 0 {
                fatal(
                    null::<Floc>(),
                    c"cannot open jobserver %s: %s".as_ptr(),
                    &[
                        FmtArg::Str(fifo_name),
                        FmtArg::Str(strerror(*__errno_location())),
                    ],
                );
            }
            js_type = JsType::Fifo;
        }
    }

    if js_type == JsType::None {
        if !style.is_null() && strcmp(style, c"pipe".as_ptr()) != 0 {
            fatal(
                null::<Floc>(),
                c"unknown jobserver auth style '%s'".as_ptr(),
                &[FmtArg::Str(style)],
            );
        }
        loop {
            r = pipe(&raw mut job_fds as *mut c_int);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            pfatal_with_name(c"creating jobs pipe".as_ptr());
        }
        js_type = JsType::Pipe;
    }

    fd_noinherit(job_fds[0]);
    fd_noinherit(job_fds[1]);
    if make_job_rfd() < 0 {
        pfatal_with_name(c"duping jobs pipe".as_ptr());
    }

    // Fill the pipe with tokens, one per slot, without blocking so we can
    // detect when the requested job count exceeds the pipe capacity.
    set_blocking(job_fds[1], false);
    for k in 0..slots {
        loop {
            r = write(job_fds[1], &raw const token as *const c_void, 1) as c_int;
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r != 1 {
            if *__errno_location() != EAGAIN {
                pfatal_with_name(c"init jobserver pipe".as_ptr());
            }
            fatal(
                null::<Floc>(),
                c"requested job count (%d) is larger than system limit (%d)".as_ptr(),
                &[FmtArg::Int((slots + 1) as i64), FmtArg::Int(k as i64)],
            );
        }
    }
    set_blocking(job_fds[1], true);
    set_blocking(job_fds[0], false);

    1
}

/// Adopt the jobserver described by an inherited `--jobserver-auth` value
/// (`fifo:<path>` or `<rfd>,<wfd>`). Returns 1 on success, 0 if the
/// jobserver is unusable.
///
/// # Safety
/// `auth` must be a valid NUL-terminated string; must run single-threaded
/// during startup.
pub unsafe fn jobserver_parse_auth(auth: *const c_char) -> c_uint {
    let mut rfd: c_int = 0;
    let mut wfd: c_int = 0;

    if strncmp(auth, FIFO_PREFIX.as_ptr(), FIFO_PREFIX.to_bytes().len()) == 0 {
        fifo_name = xstrdup(auth.add(FIFO_PREFIX.to_bytes().len()));
        loop {
            job_fds[0] = open(fifo_name, O_RDONLY);
            if !(job_fds[0] == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if job_fds[0] < 0 {
            error(
                null::<Floc>(),
                c"cannot open jobserver %s: %s".as_ptr(),
                &[
                    FmtArg::Str(fifo_name),
                    FmtArg::Str(strerror(*__errno_location())),
                ],
            );
            return 0;
        }
        loop {
            job_fds[1] = open(fifo_name, O_WRONLY);
            if !(job_fds[1] == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if job_fds[1] < 0 {
            error(
                null::<Floc>(),
                c"cannot open jobserver %s: %s".as_ptr(),
                &[
                    FmtArg::Str(fifo_name),
                    FmtArg::Str(strerror(*__errno_location())),
                ],
            );
            return 0;
        }
        js_type = JsType::Fifo;
    } else if sscanf(auth, c"%d,%d".as_ptr(), &mut rfd, &mut wfd) == 2 {
        // A simple pipe; reject the "invalid" marker and dead descriptors.
        if rfd == -2 || wfd == -2 {
            return 0;
        }
        if fcntl(rfd, F_GETFD) == -1 || fcntl(wfd, F_GETFD) == -1 {
            return 0;
        }
        job_fds[0] = rfd;
        job_fds[1] = wfd;
        js_type = JsType::Pipe;
    } else {
        error(
            null::<Floc>(),
            c"invalid --jobserver-auth string '%s'".as_ptr(),
            &[FmtArg::Str(auth)],
        );
        return 0;
    }

    if make_job_rfd() < 0 {
        if *__errno_location() != EBADF {
            pfatal_with_name(c"jobserver readfd".as_ptr());
        }
        jobserver_clear();
        return 0;
    }

    set_blocking(job_fds[0], false);
    fd_noinherit(job_fds[0]);
    fd_noinherit(job_fds[1]);
    1
}

/// Return an `xmalloc`'d `--jobserver-auth` value describing this
/// jobserver.
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_get_auth() -> *mut c_char {
    if js_type == JsType::Fifo {
        let auth = xmalloc(strlen(fifo_name) + FIFO_PREFIX.to_bytes().len() + 1) as *mut c_char;
        sprintf(auth, c"fifo:%s".as_ptr(), fifo_name);
        auth
    } else {
        let auth = xmalloc(INTSTR_LENGTH * 2 + 2) as *mut c_char;
        sprintf(auth, c"%d,%d".as_ptr(), job_fds[0], job_fds[1]);
        auth
    }
}

/// The auth value handed to non-recursive children so they detect — and
/// warn about — using the jobserver without a `+` prefix. Fifo-style
/// jobservers have no such marker.
///
/// # Safety
/// Always safe; unsafe only for C-API signature compatibility.
pub unsafe fn jobserver_get_invalid_auth() -> *const c_char {
    if js_type == JsType::Fifo {
        return null();
    }
    c" --jobserver-auth=-2,-2".as_ptr()
}

/// Whether a jobserver is active.
///
/// # Safety
/// Always safe; unsafe only for C-API signature compatibility.
pub unsafe fn jobserver_enabled() -> c_uint {
    (js_type != JsType::None) as c_uint
}

/// Close down the jobserver, unlinking the fifo if we created it.
///
/// # Safety
/// Must run single-threaded (also called from the fatal-signal path, where
/// it avoids freeing).
pub unsafe fn jobserver_clear() {
    if job_fds[0] >= 0 {
        close(job_fds[0]);
    }
    if job_fds[1] >= 0 {
        close(job_fds[1]);
    }
    if job_rfd >= 0 {
        close(job_rfd);
    }
    job_fds = [-1, -1];
    job_rfd = -1;

    if !fifo_name.is_null() {
        if JOB_ROOT.load(Ordering::Relaxed) {
            let mut r: c_int;
            loop {
                r = unlink(fifo_name);
                if !(r == -1 && *__errno_location() == EINTR) {
                    break;
                }
            }
        }
        if handling_fatal_signal == 0 {
            free(fifo_name as *mut c_void);
            fifo_name = null_mut();
        }
    }

    js_type = JsType::None;
}

/// Return a token to the jobserver. When `is_fatal`, die on failure;
/// otherwise just report it.
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_release(is_fatal: c_int) {
    let mut r: c_int;
    loop {
        r = write(job_fds[1], &raw const token as *const c_void, 1) as c_int;
        if !(r == -1 && *__errno_location() == EINTR) {
            break;
        }
    }
    if r != 1 {
        if is_fatal != 0 {
            pfatal_with_name(c"write jobserver".as_ptr());
        }
        perror_with_name(c"write".as_ptr(), c"".as_ptr());
    }
}

/// Drain every available token (used before exec'ing a re-invoked make).
/// Returns the number of tokens read.
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_acquire_all() -> c_uint {
    let mut tokens: c_uint = 0;

    // Close the write side so the read below sees EOF once the pipe drains.
    set_blocking(job_fds[0], true);
    close(job_fds[1]);
    job_fds[1] = -1;

    loop {
        let mut intake: c_char = 0;
        let mut r: c_int;
        loop {
            r = read(job_fds[0], &mut intake as *mut c_char as *mut c_void, 1) as c_int;
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r != 1 {
            break;
        }
        tokens += 1;
    }

    if 0x4 & db_level != 0 {
        printf(c"Acquired all %u jobserver tokens.\n".as_ptr(), tokens);
        fflush(stdout);
    }

    jobserver_clear();
    tokens
}

/// Re-share the pipe fds with a recursive child (fifo jobservers pass the
/// path instead).
///
/// # Safety
/// Must run single-threaded around fork/exec.
pub unsafe fn jobserver_pre_child(recursive: c_int) {
    if recursive != 0 && js_type == JsType::Pipe {
        fd_inherit(job_fds[0]);
        fd_inherit(job_fds[1]);
    }
}

/// Undo [`jobserver_pre_child`].
///
/// # Safety
/// Must run single-threaded around fork/exec.
pub unsafe fn jobserver_post_child(recursive: c_int) {
    if recursive != 0 && js_type == JsType::Pipe {
        fd_noinherit(job_fds[0]);
        fd_noinherit(job_fds[1]);
    }
}

/// Called from the SIGCHLD handler: close the private read dup so a
/// blocked acquire wakes up.
///
/// # Safety
/// Async-signal-safe (only `close`).
pub unsafe fn jobserver_signal() {
    if job_rfd >= 0 {
        close(job_rfd);
        job_rfd = -1;
    }
}

/// Re-create the private read dup before waiting for a token.
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_pre_acquire() {
    if job_rfd < 0 && job_fds[0] >= 0 && make_job_rfd() < 0 {
        pfatal_with_name(c"duping jobs pipe".as_ptr());
    }
}

/// Wait (with pselect) for a token; with `timeout` nonzero give up after a
/// second. Returns 1 if a token was read, 0 on timeout/interrupt.
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_acquire(timeout: c_int) -> c_uint {
    let mut spec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut specp: *mut timespec = null_mut();
    let mut empty: sigset_t = ::core::mem::zeroed();
    sigemptyset(&mut empty);

    if timeout != 0 {
        // Alarms don't interrupt pselect, so use its own timeout instead.
        spec.tv_sec = 1;
        spec.tv_nsec = 0;
        specp = &mut spec;
    }

    loop {
        let mut readfds: libc::fd_set = ::core::mem::zeroed();
        FD_ZERO(&mut readfds);
        FD_SET(job_fds[0], &mut readfds);

        let mut r = pselect(
            job_fds[0] + 1,
            &mut readfds,
            null_mut(),
            null_mut(),
            specp,
            &empty,
        );
        if r < 0 {
            match *__errno_location() {
                EINTR => return 0,
                EBADF => {
                    // The read side was closed by jobserver_signal().
                    fatal(null::<Floc>(), c"job server shut down".as_ptr(), &[]);
                }
                _ => pfatal_with_name(c"pselect jobs pipe".as_ptr()),
            }
        }
        if r == 0 {
            return 0;
        }

        let mut intake: c_char = 0;
        loop {
            r = read(job_fds[0], &mut intake as *mut c_char as *mut c_void, 1) as c_int;
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            // The token was taken by another instance between the pselect
            // and the read; wait again.
            if *__errno_location() == EAGAIN {
                continue;
            }
            pfatal_with_name(c"read jobs pipe".as_ptr());
        }
        return (r > 0) as c_uint;
    }
}

// ---------------------------------------------------------------------------
// Output-sync mutex.

const MUTEX_PREFIX: &CStr = c"fnm:";

static mut osync_handle: c_int = -1;
static mut osync_tmpfile: *mut c_char = null_mut();
/// True in the process that created the lock file (and so unlinks it).
static SYNC_ROOT: AtomicBool = AtomicBool::new(false);

/// Whether the output-sync mutex is available.
///
/// # Safety
/// Always safe; unsafe only for C-API signature compatibility.
pub unsafe fn osync_enabled() -> c_uint {
    (osync_handle >= 0) as c_uint
}

/// Create the output-sync lock file.
///
/// # Safety
/// Must run single-threaded during startup.
pub unsafe fn osync_setup() {
    osync_handle = get_tmpfd(&raw mut osync_tmpfile);
    fd_noinherit(osync_handle);
    SYNC_ROOT.store(true, Ordering::Relaxed);
}

/// Return an `xmalloc`'d `--sync-mutex` value (`fnm:<path>`) or null when
/// output sync is off.
///
/// # Safety
/// Must run single-threaded.
pub unsafe fn osync_get_mutex() -> *mut c_char {
    if osync_enabled() == 0 {
        return null_mut();
    }
    let mutex = xmalloc(strlen(osync_tmpfile) + MUTEX_PREFIX.to_bytes().len() + 1) as *mut c_char;
    sprintf(mutex, c"fnm:%s".as_ptr(), osync_tmpfile);
    mutex
}

/// Adopt the output-sync mutex described by an inherited `--sync-mutex`
/// value. Returns 1 on success, 0 on a malformed value.
///
/// # Safety
/// `mutex` must be a valid NUL-terminated string; must run single-threaded
/// during startup.
pub unsafe fn osync_parse_mutex(mutex: *const c_char) -> c_uint {
    if strncmp(mutex, MUTEX_PREFIX.as_ptr(), MUTEX_PREFIX.to_bytes().len()) != 0 {
        error(
            null::<Floc>(),
            c"invalid --sync-mutex string '%s'".as_ptr(),
            &[FmtArg::Str(mutex)],
        );
        return 0;
    }

    free(osync_tmpfile as *mut c_void);
    osync_tmpfile = xstrdup(mutex.add(MUTEX_PREFIX.to_bytes().len()));

    loop {
        osync_handle = open(osync_tmpfile, O_WRONLY);
        if !(osync_handle == -1 && *__errno_location() == EINTR) {
            break;
        }
    }
    if osync_handle < 0 {
        fatal(
            null::<Floc>(),
            c"cannot open output sync mutex %s: %s".as_ptr(),
            &[
                FmtArg::Str(osync_tmpfile),
                FmtArg::Str(strerror(*__errno_location())),
            ],
        );
    }
    fd_noinherit(osync_handle);
    1
}

/// Close the output-sync mutex, unlinking the file if we created it.
///
/// # Safety
/// Must run single-threaded.
pub unsafe fn osync_clear() {
    if osync_handle >= 0 {
        close(osync_handle);
        osync_handle = -1;
    }
    if SYNC_ROOT.load(Ordering::Relaxed) && !osync_tmpfile.is_null() {
        let mut r: c_int;
        loop {
            r = unlink(osync_tmpfile);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        free(osync_tmpfile as *mut c_void);
        osync_tmpfile = null_mut();
    }
}

/// Take the output-sync lock (a write lock on the first byte). Returns 0
/// if locking failed and output sync should be disabled.
///
/// # Safety
/// Must run single-threaded.
pub unsafe fn osync_acquire() -> c_uint {
    if osync_enabled() != 0 {
        let mut fl: flock = ::core::mem::zeroed();
        fl.l_type = F_WRLCK as ::core::ffi::c_short;
        fl.l_whence = SEEK_SET as ::core::ffi::c_short;
        fl.l_start = 0;
        fl.l_len = 1;
        if fcntl(osync_handle, F_SETLKW, &mut fl) == -1 {
            perror(c"fcntl()".as_ptr());
            return 0;
        }
    }
    1
}

/// Release the output-sync lock.
///
/// # Safety
/// Must run single-threaded.
pub unsafe fn osync_release() {
    if osync_enabled() != 0 {
        let mut fl: flock = ::core::mem::zeroed();
        fl.l_type = F_UNLCK as ::core::ffi::c_short;
        fl.l_whence = SEEK_SET as ::core::ffi::c_short;
        fl.l_start = 0;
        fl.l_len = 1;
        if fcntl(osync_handle, F_SETLKW, &mut fl) == -1 {
            perror(c"fcntl()".as_ptr());
        }
    }
}

// ---------------------------------------------------------------------------
// fd helpers.

/// A read fd that always reports EOF, handed to non-interactive children
/// as stdin. Created once and cached.
///
/// # Safety
/// Must run single-threaded the first time.
pub unsafe fn get_bad_stdin() -> c_int {
    static BAD_STDIN: AtomicI32 = AtomicI32::new(-1);
    let cached = BAD_STDIN.load(Ordering::Relaxed);
    if cached != -1 {
        return cached;
    }

    let mut pd: [c_int; 2] = [0; 2];
    if pipe(pd.as_mut_ptr()) == 0 {
        // Close the write side so reads see EOF.
        close(pd[1]);
        fd_noinherit(pd[0]);
        match BAD_STDIN.compare_exchange(-1, pd[0], Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => pd[0],
            Err(existing) => {
                close(pd[0]);
                existing
            }
        }
    } else {
        -1
    }
}

/// Clear `FD_CLOEXEC` on `fd`.
///
/// # Safety
/// `fd` must be an open descriptor.
pub unsafe fn fd_inherit(fd: c_int) {
    let flags = fcntl_retry(fd, F_GETFD);
    if flags >= 0 {
        fcntl_set_retry(fd, F_SETFD, flags & !FD_CLOEXEC);
    }
}

/// Set `FD_CLOEXEC` on `fd`.
///
/// # Safety
/// `fd` must be an open descriptor.
pub unsafe fn fd_noinherit(fd: c_int) {
    let flags = fcntl_retry(fd, F_GETFD);
    if flags >= 0 {
        fcntl_set_retry(fd, F_SETFD, flags | FD_CLOEXEC);
    }
}

/// If `fd` refers to a regular file, switch it to append mode (so parallel
/// writers don't clobber each other). Returns the previous flags, or -1 if
/// nothing was changed.
///
/// # Safety
/// `fd` must be an open descriptor.
pub unsafe fn fd_set_append(fd: c_int) -> c_int {
    let mut flags: c_int = -1;
    let mut stbuf: libc::stat = ::core::mem::zeroed();
    if fstat(fd, &mut stbuf) == 0 && stbuf.st_mode & S_IFMT == S_IFREG {
        flags = fcntl(fd, F_GETFL, 0);
        if flags >= 0 {
            fcntl_set_retry(fd, F_SETFL, flags | O_APPEND);
        }
    }
    flags
}

/// Restore the flags saved by [`fd_set_append`].
///
/// # Safety
/// `fd` must be an open descriptor; `flags` must come from
/// [`fd_set_append`].
pub unsafe fn fd_reset_append(fd: c_int, flags: c_int) {
    if flags >= 0 {
        fcntl_set_retry(fd, F_SETFL, flags);
    }
}

/// Create an anonymous (unlinked) temp file fd, preferring `O_TMPFILE` and
/// falling back to `tmpfile(3)` when the temp dir is the default `/tmp`.
/// Returns -1 if no anonymous file can be made here.
///
/// # Safety
/// Must run single-threaded (reports errors through the printers).
pub unsafe fn os_anontmp() -> c_int {
    let tdir = get_tmpdir();
    let mut fd: c_int = -1;
    static TMPFILE_WORKS: AtomicBool = AtomicBool::new(true);

    if TMPFILE_WORKS.load(Ordering::Relaxed) {
        loop {
            fd = open(tdir, O_RDWR | O_TMPFILE | O_EXCL, 0o600 as c_int);
            if !(fd == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if fd >= 0 {
            return fd;
        }
        if 0x1 & db_level != 0 {
            printf(
                c"Cannot open '%s' with O_TMPFILE: %s.\n".as_ptr(),
                tdir,
                strerror(*__errno_location()),
            );
            fflush(stdout);
        }
        TMPFILE_WORKS.store(false, Ordering::Relaxed);
    }

    // tmpfile() uses the system default temp dir, so only fall back to it
    // when that's where we want the file anyway.
    if strcmp(tdir, c"/tmp".as_ptr()) == 0 {
        let mask: mode_t = umask(0o77);
        let mut tfile: *mut libc::FILE;
        loop {
            *__errno_location() = 0;
            tfile = tmpfile();
            if !(tfile.is_null() && *__errno_location() == EINTR) {
                break;
            }
        }
        if tfile.is_null() {
            error(
                null::<Floc>(),
                c"tmpfile: %s".as_ptr(),
                &[FmtArg::Str(strerror(*__errno_location()))],
            );
            return -1;
        }
        umask(mask);
        loop {
            fd = dup(libc::fileno(tfile));
            if !(fd == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if fd < 0 {
            error(
                null::<Floc>(),
                c"dup: %s".as_ptr(),
                &[FmtArg::Str(strerror(*__errno_location()))],
            );
        }
        libc::fclose(tfile);
    }
    fd
}
