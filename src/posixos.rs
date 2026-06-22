//! POSIX-specific glue: jobserver (pipe/fifo token protocol), the
//! output-sync mutex, and fd helpers.
//!
//! Port of `posixos.c`. The jobserver state stays in module-level globals
//! with C-shaped accessors because `job.rs` and `main.rs` drive it through
//! the original entry points.

use ::core::ffi::{c_char, c_longlong, c_uint, c_void, CStr};
use ::core::ptr::{null, null_mut};

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU8, Ordering};

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
    fn fflush(stream: *mut FILE) -> i32;
    fn fileno(stream: *mut FILE) -> i32;
}

/// Stream fileno for `crate::stdio::FILE` streams (libc's `fileno` takes
/// `libc::FILE`, which is a distinct type).
unsafe fn stream_fd(stream: *mut FILE) -> i32 {
    fileno(stream)
}

/// `check_io_state` bits (see os.h).
pub const IO_UNKNOWN: i32 = 0x1;
pub const IO_COMBINED_OUTERR: i32 = 0x2;
pub const IO_STDIN_OK: i32 = 0x4;
pub const IO_STDOUT_OK: i32 = 0x8;
pub const IO_STDERR_OK: i32 = 0x10;

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

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum JsType {
    None = 0,
    Pipe = 1,
    Fifo = 2,
}

/// The active jobserver style. Stored as an `AtomicU8` so the pure
/// predicate `jobserver_enabled` can be a safe `fn`; all access is
/// single-threaded (startup / under the job lock), so `Relaxed` ordering
/// preserves the original program order.
static JS_TYPE: AtomicU8 = AtomicU8::new(JsType::None as u8);

fn js_type_get() -> JsType {
    match JS_TYPE.load(Ordering::Relaxed) {
        x if x == JsType::Pipe as u8 => JsType::Pipe,
        x if x == JsType::Fifo as u8 => JsType::Fifo,
        _ => JsType::None,
    }
}

fn js_type_set(t: JsType) {
    JS_TYPE.store(t as u8, Ordering::Relaxed);
}

/// True in the process that created the jobserver (and so owns the fifo).
static JOB_ROOT: AtomicBool = AtomicBool::new(false);
/// The token pipe/fifo: `[read end, write end]`.
static mut job_fds: [i32; 2] = [-1, -1];
/// A private dup of the read side (closed by a fatal signal to wake us).
static JOB_RFD: AtomicI32 = AtomicI32::new(-1);

fn job_rfd() -> i32 {
    JOB_RFD.load(Ordering::Relaxed)
}
/// The token character written for each available job slot.
static mut token: c_char = b'+' as c_char;
static mut fifo_name: *mut c_char = null_mut();

/// On POSIX with pselect there is no need for a separate read dup; the
/// blocking read is interruptible already.
fn make_job_rfd() -> i32 {
    0
}

/// Retry-on-EINTR wrapper around `fcntl(fd, cmd)`.
unsafe fn fcntl_retry(fd: i32, cmd: i32) -> i32 {
    loop {
        let r = fcntl(fd, cmd);
        if !(r == -1 && *__errno_location() == EINTR) {
            return r;
        }
    }
}

/// Retry-on-EINTR wrapper around `fcntl(fd, cmd, arg)`.
unsafe fn fcntl_set_retry(fd: i32, cmd: i32, arg: i32) -> i32 {
    loop {
        let r = fcntl(fd, cmd, arg);
        if !(r == -1 && *__errno_location() == EINTR) {
            return r;
        }
    }
}

/// Set or clear `O_NONBLOCK` on `fd`, dying on failure.
unsafe fn set_blocking(ctx: &crate::execctx::ExecContext, fd: i32, blocking: bool) {
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
        pfatal_with_name(ctx, c"fcntl(O_NONBLOCK)".as_ptr());
    }
}

/// Create the jobserver (fifo if possible, else an anonymous pipe) with
/// `slots` available tokens. Returns 1.
///
/// # Safety
/// `style` must be null or a valid NUL-terminated string; must run
/// single-threaded during startup.
pub unsafe fn jobserver_setup(
    ctx: &crate::execctx::ExecContext,
    slots: i32,
    style: *const c_char,
) -> c_uint {
    let mut r: i32;

    JOB_ROOT.store(true, Ordering::Relaxed);

    if style.is_null() || strcmp(style, c"fifo".as_ptr()) == 0 {
        let tmpdir = get_tmpdir(ctx);
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
            perror_with_name(ctx, c"jobserver mkfifo: ".as_ptr(), fifo_name);
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
                    ctx,
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
                    ctx,
                    null::<Floc>(),
                    c"cannot open jobserver %s: %s".as_ptr(),
                    &[
                        FmtArg::Str(fifo_name),
                        FmtArg::Str(strerror(*__errno_location())),
                    ],
                );
            }
            js_type_set(JsType::Fifo);
        }
    }

    if js_type_get() == JsType::None {
        if !style.is_null() && strcmp(style, c"pipe".as_ptr()) != 0 {
            fatal(
                ctx,
                null::<Floc>(),
                c"unknown jobserver auth style '%s'".as_ptr(),
                &[FmtArg::Str(style)],
            );
        }
        loop {
            r = pipe(&raw mut job_fds as *mut i32);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            pfatal_with_name(ctx, c"creating jobs pipe".as_ptr());
        }
        js_type_set(JsType::Pipe);
    }

    fd_noinherit(job_fds[0]);
    fd_noinherit(job_fds[1]);
    if make_job_rfd() < 0 {
        pfatal_with_name(ctx, c"duping jobs pipe".as_ptr());
    }

    // Fill the pipe with tokens, one per slot, without blocking so we can
    // detect when the requested job count exceeds the pipe capacity.
    set_blocking(ctx, job_fds[1], false);
    for k in 0..slots {
        loop {
            r = write(job_fds[1], &raw const token as *const c_void, 1) as i32;
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r != 1 {
            if *__errno_location() != EAGAIN {
                pfatal_with_name(ctx, c"init jobserver pipe".as_ptr());
            }
            fatal(
                ctx,
                null::<Floc>(),
                c"requested job count (%d) is larger than system limit (%d)".as_ptr(),
                &[FmtArg::Int((slots + 1) as i64), FmtArg::Int(k as i64)],
            );
        }
    }
    set_blocking(ctx, job_fds[1], true);
    set_blocking(ctx, job_fds[0], false);

    1
}

/// Adopt the jobserver described by an inherited `--jobserver-auth` value
/// (`fifo:<path>` or `<rfd>,<wfd>`). Returns 1 on success, 0 if the
/// jobserver is unusable.
///
/// # Safety
/// `auth` must be a valid NUL-terminated string; must run single-threaded
/// during startup.
pub unsafe fn jobserver_parse_auth(
    ctx: &crate::execctx::ExecContext,
    auth: *const c_char,
) -> c_uint {
    let mut rfd: i32 = 0;
    let mut wfd: i32 = 0;

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
                ctx,
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
                ctx,
                null::<Floc>(),
                c"cannot open jobserver %s: %s".as_ptr(),
                &[
                    FmtArg::Str(fifo_name),
                    FmtArg::Str(strerror(*__errno_location())),
                ],
            );
            return 0;
        }
        js_type_set(JsType::Fifo);
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
        js_type_set(JsType::Pipe);
    } else {
        error(
            ctx,
            null::<Floc>(),
            c"invalid --jobserver-auth string '%s'".as_ptr(),
            &[FmtArg::Str(auth)],
        );
        return 0;
    }

    if make_job_rfd() < 0 {
        if *__errno_location() != EBADF {
            pfatal_with_name(ctx, c"jobserver readfd".as_ptr());
        }
        jobserver_clear();
        return 0;
    }

    set_blocking(ctx, job_fds[0], false);
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
    if js_type_get() == JsType::Fifo {
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
pub fn jobserver_get_invalid_auth() -> *const c_char {
    if js_type_get() == JsType::Fifo {
        return null();
    }
    c" --jobserver-auth=-2,-2".as_ptr()
}

/// Whether a jobserver is active.
pub fn jobserver_enabled() -> c_uint {
    (js_type_get() != JsType::None) as c_uint
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
    let rfd = job_rfd();
    if rfd >= 0 {
        close(rfd);
    }
    job_fds = [-1, -1];
    JOB_RFD.store(-1, Ordering::Relaxed);

    if !fifo_name.is_null() {
        if JOB_ROOT.load(Ordering::Relaxed) {
            let mut r: i32;
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

    js_type_set(JsType::None);
}

/// Return a token to the jobserver. When `is_fatal`, die on failure;
/// otherwise just report it.
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_release(ctx: &crate::execctx::ExecContext, is_fatal: i32) {
    let mut r: i32;
    loop {
        r = write(job_fds[1], &raw const token as *const c_void, 1) as i32;
        if !(r == -1 && *__errno_location() == EINTR) {
            break;
        }
    }
    if r != 1 {
        if is_fatal != 0 {
            pfatal_with_name(ctx, c"write jobserver".as_ptr());
        }
        perror_with_name(ctx, c"write".as_ptr(), c"".as_ptr());
    }
}

/// Drain every available token (used before exec'ing a re-invoked make).
/// Returns the number of tokens read.
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_acquire_all(ctx: &crate::execctx::ExecContext) -> c_uint {
    let mut tokens: c_uint = 0;

    // Close the write side so the read below sees EOF once the pipe drains.
    set_blocking(ctx, job_fds[0], true);
    close(job_fds[1]);
    job_fds[1] = -1;

    loop {
        let mut intake: c_char = 0;
        let mut r: i32;
        loop {
            r = read(job_fds[0], &mut intake as *mut c_char as *mut c_void, 1) as i32;
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
pub unsafe fn jobserver_pre_child(recursive: i32) {
    if recursive != 0 && js_type_get() == JsType::Pipe {
        fd_inherit(job_fds[0]);
        fd_inherit(job_fds[1]);
    }
}

/// Undo [`jobserver_pre_child`].
///
/// # Safety
/// Must run single-threaded around fork/exec.
pub unsafe fn jobserver_post_child(recursive: i32) {
    if recursive != 0 && js_type_get() == JsType::Pipe {
        fd_noinherit(job_fds[0]);
        fd_noinherit(job_fds[1]);
    }
}

/// Called from the SIGCHLD handler: close the private read dup so a
/// blocked acquire wakes up. Async-signal-safe (only `close`).
pub fn jobserver_signal() {
    let rfd = job_rfd();
    if rfd >= 0 {
        // SAFETY: `close` is async-signal-safe, and closing a file descriptor
        // is not a Rust memory-safety hazard; any `i32` is a valid argument.
        unsafe { close(rfd) };
        JOB_RFD.store(-1, Ordering::Relaxed);
    }
}

/// Re-create the private read dup before waiting for a token.
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_pre_acquire(ctx: &crate::execctx::ExecContext) {
    if job_rfd() < 0 && job_fds[0] >= 0 && make_job_rfd() < 0 {
        pfatal_with_name(ctx, c"duping jobs pipe".as_ptr());
    }
}

/// Wait (with pselect) for a token; with `timeout` nonzero give up after a
/// second. Returns 1 if a token was read, 0 on timeout/interrupt.
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_acquire(ctx: &crate::execctx::ExecContext, timeout: i32) -> c_uint {
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
                    fatal(ctx, null::<Floc>(), 0, c"job server shut down".as_ptr());
                }
                _ => pfatal_with_name(ctx, c"pselect jobs pipe".as_ptr()),
            }
        }
        if r == 0 {
            return 0;
        }

        let mut intake: c_char = 0;
        loop {
            r = read(job_fds[0], &mut intake as *mut c_char as *mut c_void, 1) as i32;
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
            pfatal_with_name(ctx, c"read jobs pipe".as_ptr());
        }
        return (r > 0) as c_uint;
    }
}

// ---------------------------------------------------------------------------
// Output-sync mutex.

const MUTEX_PREFIX: &CStr = c"fnm:";

static OSYNC_HANDLE: AtomicI32 = AtomicI32::new(-1);
static mut osync_tmpfile: *mut c_char = null_mut();
/// True in the process that created the lock file (and so unlinks it).
static SYNC_ROOT: AtomicBool = AtomicBool::new(false);

/// Whether the output-sync mutex is available.
pub fn osync_enabled() -> c_uint {
    (OSYNC_HANDLE.load(Ordering::Relaxed) >= 0) as c_uint
}

/// Create the output-sync lock file.
///
/// # Safety
/// Must run single-threaded during startup.
pub unsafe fn osync_setup(ctx: &crate::execctx::ExecContext) {
    let h = get_tmpfd(ctx, &raw mut osync_tmpfile);
    OSYNC_HANDLE.store(h, Ordering::Relaxed);
    fd_noinherit(h);
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
pub unsafe fn osync_parse_mutex(ctx: &crate::execctx::ExecContext, mutex: *const c_char) -> c_uint {
    if strncmp(mutex, MUTEX_PREFIX.as_ptr(), MUTEX_PREFIX.to_bytes().len()) != 0 {
        error(
            ctx,
            null::<Floc>(),
            c"invalid --sync-mutex string '%s'".as_ptr(),
            &[FmtArg::Str(mutex)],
        );
        return 0;
    }

    free(osync_tmpfile as *mut c_void);
    osync_tmpfile = xstrdup(mutex.add(MUTEX_PREFIX.to_bytes().len()));

    loop {
        let h = open(osync_tmpfile, O_WRONLY);
        OSYNC_HANDLE.store(h, Ordering::Relaxed);
        if !(h == -1 && *__errno_location() == EINTR) {
            break;
        }
    }
    if OSYNC_HANDLE.load(Ordering::Relaxed) < 0 {
        fatal(
            ctx,
            null::<Floc>(),
            c"cannot open output sync mutex %s: %s".as_ptr(),
            &[
                FmtArg::Str(osync_tmpfile),
                FmtArg::Str(strerror(*__errno_location())),
            ],
        );
    }
    fd_noinherit(OSYNC_HANDLE.load(Ordering::Relaxed));
    1
}

/// Close the output-sync mutex, unlinking the file if we created it.
///
/// # Safety
/// Must run single-threaded.
pub unsafe fn osync_clear() {
    let h = OSYNC_HANDLE.load(Ordering::Relaxed);
    if h >= 0 {
        close(h);
        OSYNC_HANDLE.store(-1, Ordering::Relaxed);
    }
    if SYNC_ROOT.load(Ordering::Relaxed) && !osync_tmpfile.is_null() {
        let mut r: i32;
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
        if fcntl(OSYNC_HANDLE.load(Ordering::Relaxed), F_SETLKW, &mut fl) == -1 {
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
        if fcntl(OSYNC_HANDLE.load(Ordering::Relaxed), F_SETLKW, &mut fl) == -1 {
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
pub unsafe fn get_bad_stdin() -> i32 {
    static BAD_STDIN: AtomicI32 = AtomicI32::new(-1);
    let cached = BAD_STDIN.load(Ordering::Relaxed);
    if cached != -1 {
        return cached;
    }

    let mut pd: [i32; 2] = [0; 2];
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
pub unsafe fn fd_inherit(fd: i32) {
    let flags = fcntl_retry(fd, F_GETFD);
    if flags >= 0 {
        fcntl_set_retry(fd, F_SETFD, flags & !FD_CLOEXEC);
    }
}

/// Set `FD_CLOEXEC` on `fd`.
///
/// # Safety
/// `fd` must be an open descriptor.
pub unsafe fn fd_noinherit(fd: i32) {
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
pub unsafe fn fd_set_append(fd: i32) -> i32 {
    let mut flags: i32 = -1;
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
pub unsafe fn fd_reset_append(fd: i32, flags: i32) {
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
pub unsafe fn os_anontmp(ctx: &crate::execctx::ExecContext) -> i32 {
    let tdir = get_tmpdir(ctx);
    let mut fd: i32 = -1;
    static TMPFILE_WORKS: AtomicBool = AtomicBool::new(true);

    if TMPFILE_WORKS.load(Ordering::Relaxed) {
        loop {
            fd = open(tdir, O_RDWR | O_TMPFILE | O_EXCL, 0o600_i32);
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
                ctx,
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
                ctx,
                null::<Floc>(),
                c"dup: %s".as_ptr(),
                &[FmtArg::Str(strerror(*__errno_location()))],
            );
        }
        libc::fclose(tfile);
    }
    fd
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Serializes tests that mutate the shared `JS_TYPE` global so the
    /// parallel test harness can't interleave a `js_type_set` with another
    /// test's read between set and assert.
    static JS_TYPE_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Serializes tests that mutate the shared `JOB_RFD` global so a parallel
    /// test can't observe (or close) a transient fd another test installed.
    static JOB_RFD_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// `osync_enabled` reflects the sign of the output-sync handle: a
    /// negative handle (the unset default) means disabled, a non-negative
    /// fd means enabled. This exercises the safe predicate over the
    /// `AtomicI32` directly, without touching the FFI setup/clear paths.
    #[test]
    fn osync_enabled_tracks_handle_sign() {
        let saved = OSYNC_HANDLE.load(Ordering::Relaxed);

        OSYNC_HANDLE.store(-1, Ordering::Relaxed);
        assert_eq!(osync_enabled(), 0, "negative handle is disabled");

        OSYNC_HANDLE.store(0, Ordering::Relaxed);
        assert_eq!(osync_enabled(), 1, "fd 0 is enabled");

        OSYNC_HANDLE.store(7, Ordering::Relaxed);
        assert_eq!(osync_enabled(), 1, "positive fd is enabled");

        OSYNC_HANDLE.store(saved, Ordering::Relaxed);
    }

    /// `jobserver_enabled` is true for any active style and false for
    /// `None`, and the `JS_TYPE` atomic round-trips through
    /// `js_type_get`/`js_type_set` for every variant. Exercises the safe
    /// predicate without touching the FFI setup paths.
    #[test]
    fn jobserver_enabled_tracks_js_type() {
        let _guard = JS_TYPE_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let saved = JS_TYPE.load(Ordering::Relaxed);

        js_type_set(JsType::None);
        assert!(js_type_get() == JsType::None);
        assert_eq!(jobserver_enabled(), 0, "None is disabled");

        js_type_set(JsType::Pipe);
        assert!(js_type_get() == JsType::Pipe);
        assert_eq!(jobserver_enabled(), 1, "Pipe is enabled");

        js_type_set(JsType::Fifo);
        assert!(js_type_get() == JsType::Fifo);
        assert_eq!(jobserver_enabled(), 1, "Fifo is enabled");

        JS_TYPE.store(saved, Ordering::Relaxed);
    }

    /// `jobserver_get_invalid_auth` returns null for fifo jobservers (which
    /// carry no `+`-prefix marker) and the sentinel `--jobserver-auth=-2,-2`
    /// string otherwise.
    #[test]
    fn invalid_auth_is_null_only_for_fifo() {
        let _guard = JS_TYPE_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let saved = JS_TYPE.load(Ordering::Relaxed);

        js_type_set(JsType::Fifo);
        assert!(jobserver_get_invalid_auth().is_null(), "fifo has no marker");

        for t in [JsType::None, JsType::Pipe] {
            js_type_set(t);
            let p = jobserver_get_invalid_auth();
            assert!(!p.is_null(), "non-fifo returns the sentinel auth");
            // SAFETY: `p` points at a `&'static CStr` literal when non-null.
            let s = unsafe { CStr::from_ptr(p) };
            assert_eq!(s.to_bytes(), b" --jobserver-auth=-2,-2");
        }

        JS_TYPE.store(saved, Ordering::Relaxed);
    }

    /// `job_rfd()` reflects the `JOB_RFD` atomic: negative when there is no
    /// private read dup (the default), the fd value once set. Restores the
    /// prior value so it stays isolated from other tests.
    #[test]
    fn job_rfd_tracks_atomic() {
        let _guard = JOB_RFD_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let saved = JOB_RFD.load(Ordering::Relaxed);

        JOB_RFD.store(-1, Ordering::Relaxed);
        assert_eq!(job_rfd(), -1, "default is unset");

        JOB_RFD.store(5, Ordering::Relaxed);
        assert_eq!(job_rfd(), 5, "reflects the stored fd");

        JOB_RFD.store(saved, Ordering::Relaxed);
    }

    /// `jobserver_signal` is now a safe `fn`: callable from safe code, and a
    /// no-op when no private read dup is installed (`JOB_RFD < 0`), so it
    /// closes nothing and leaves the atomic unset. Guarded against parallel
    /// `JOB_RFD` mutators so a transient fd can't make it call `close`.
    #[test]
    fn jobserver_signal_is_noop_when_unset() {
        let _guard = JOB_RFD_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let saved = JOB_RFD.load(Ordering::Relaxed);

        JOB_RFD.store(-1, Ordering::Relaxed);
        jobserver_signal();
        assert_eq!(job_rfd(), -1, "stays unset; nothing was closed");

        JOB_RFD.store(saved, Ordering::Relaxed);
    }
}
