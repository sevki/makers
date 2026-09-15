//! POSIX-specific glue: jobserver (pipe/fifo token protocol), the
//! output-sync mutex, and fd helpers.
//!
//! Port of `posixos.c`. The jobserver/output-sync state lives on
//! `ExecContext` (not a process-wide global) behind C-shaped accessors,
//! because `job.rs` and `main.rs` drive it through the original entry
//! points.

use ::core::{
    ffi::{c_char, c_longlong, c_uint, c_void, CStr},
    ptr::{null, null_mut},
};

use std::sync::atomic::Ordering;

use libc::{
    __errno_location, close, fcntl, free, open, perror, read, sigset_t, sprintf, sscanf,
    strcmp, strerror, strlen, strncmp, timespec, write, EAGAIN, EBADF, EINTR, FD_CLOEXEC, FD_SET,
    FD_ZERO, F_GETFD, F_GETFL, F_SETFD, F_SETFL, O_APPEND, O_EXCL, O_NONBLOCK, O_RDONLY, O_RDWR,
    O_WRONLY, SEEK_SET,
};
// `flock`/`mkfifo`/`pipe`/`pselect`/`sigemptyset` and the `F_SETLKW`/
// `F_UNLCK`/`F_WRLCK`/`O_TMPFILE` constants are part of the jobserver/
// output-sync POSIX surface that WASI does not expose; see
// `crate::compat` for the wasm stand-ins (accepted architectural gap —
// job control does not work on wasm, it only needs to compile there).
#[cfg(target_family = "wasm")]
use crate::compat::{
    flock, mkfifo, pipe, pselect, sigemptyset, F_SETLKW, F_UNLCK, F_WRLCK, O_TMPFILE,
};
#[cfg(unix)]
use libc::{flock, mkfifo, pipe, pselect, sigemptyset, F_SETLKW, F_UNLCK, F_WRLCK, O_TMPFILE};

use crate::{
    commands::handling_fatal_signal,
    entry::db_level,
    floc::Floc,
    misc::{get_tmpdir, make_pid, open_named_tmpfd, xmalloc, xstrdup},
    output::{
        error,
        fatal_err,
        perror_with_name,
        pfatal_with_name,
        pfatal_with_name_err,
        FmtArg,
        INTSTR_LENGTH,
    },
};

/// `check_io_state` bits (see os.h).
pub const IO_UNKNOWN: i32 = 0x1;
pub const IO_COMBINED_OUTERR: i32 = 0x2;
pub const IO_STDIN_OK: i32 = 0x4;
pub const IO_STDOUT_OK: i32 = 0x8;
pub const IO_STDERR_OK: i32 = 0x10;

/// Which validity bits hold for stdin/stdout/stderr, computed once.
///
/// # Safety
/// Reads errno after fcntl; always sound, unsafe for C-API compatibility.
pub unsafe fn check_io_state(ctx: &crate::execctx::ExecContext) -> c_uint {
    let mut state = ctx.io_state.0.load(Ordering::Relaxed);
    if state != IO_UNKNOWN as c_uint {
        return state;
    }

    // The C original probed `fileno(stdin/stdout/stderr)`; make never
    // reopens the standard streams, so those are the process's fds 0/1/2.
    if fcntl(libc::STDIN_FILENO, F_GETFD) != -1 || *__errno_location() != EBADF {
        state |= IO_STDIN_OK as c_uint;
    }
    if fcntl(libc::STDOUT_FILENO, F_GETFD) != -1 || *__errno_location() != EBADF {
        state |= IO_STDOUT_OK as c_uint;
    }
    if fcntl(libc::STDERR_FILENO, F_GETFD) != -1 || *__errno_location() != EBADF {
        state |= IO_STDERR_OK as c_uint;
    }

    // If stdout and stderr are both usable, check whether they refer to the
    // same file.
    if state & (IO_STDOUT_OK | IO_STDERR_OK) as c_uint == (IO_STDOUT_OK | IO_STDERR_OK) as c_uint {
        // They are one destination only if the platform reports a file
        // identity for both and the two agree. Where it reports none (see
        // `crate::fs::file_id`) `zip` yields `None` and they count as
        // distinct: the cost is output that is not merged, where guessing the
        // other way would merge unrelated streams.
        let file_id = |fd| unsafe { crate::fs::metadata_of_fd(fd) }.ok().and_then(|m| m.id());
        if file_id(libc::STDOUT_FILENO)
            .zip(file_id(libc::STDERR_FILENO))
            .is_some_and(|(out, err)| out == err)
        {
            state |= IO_COMBINED_OUTERR as c_uint;
        }
    }

    ctx.io_state.0.store(state, Ordering::Relaxed);
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

fn js_type_get(ctx: &crate::execctx::ExecContext) -> JsType {
    match ctx.js_type.0.load(Ordering::Relaxed) {
        x if x == JsType::Pipe as u8 => JsType::Pipe,
        x if x == JsType::Fifo as u8 => JsType::Fifo,
        _ => JsType::None,
    }
}

fn js_type_set(ctx: &crate::execctx::ExecContext, t: JsType) {
    ctx.js_type.0.store(t as u8, Ordering::Relaxed);
}

fn job_rfd(ctx: &crate::execctx::ExecContext) -> i32 {
    ctx.job_rfd.0.load(Ordering::Relaxed)
}
/// The token character written for each available job slot. Never
/// reassigned after this initializer, so unlike `job_fds`/`fifo_name` it
/// needs no `ExecContext` slot at all.
const token: c_char = b'+' as c_char;

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

/// Set or clear `O_NONBLOCK` on `fd`. Returns `Err(BuildError::Failure)`
/// on a fatal `fcntl` failure instead of exiting (#432 Phase B, #540:
/// `std::process::exit` belongs only in `bin/make.rs`'s `main()`).
unsafe fn set_blocking(
    ctx: &crate::execctx::ExecContext,
    fd: i32,
    blocking: bool,
) -> Result<(), crate::build_result::BuildError> {
    let flags = fcntl_retry(fd, F_GETFL);
    if flags < 0 {
        return Ok(());
    }
    let new_flags = if blocking {
        flags & !O_NONBLOCK
    } else {
        flags | O_NONBLOCK
    };
    if fcntl_set_retry(fd, F_SETFL, new_flags) < 0 {
        return Err(pfatal_with_name_err(ctx, c"fcntl(O_NONBLOCK)".as_ptr()));
    }
    Ok(())
}

/// Create the jobserver (fifo if possible, else an anonymous pipe) with
/// `slots` available tokens. Returns `Ok(1)` on success, or the
/// [`crate::build_result::BuildError`] a fatal setup failure produced
/// (#432 Phase B: this only runs once at startup, from a single call site
/// already inside `main_0`'s `Result` chain).
///
/// # Safety
/// `style` must be null or a valid NUL-terminated string; must run
/// single-threaded during startup.
pub unsafe fn jobserver_setup(
    ctx: &crate::execctx::ExecContext,
    slots: i32,
    style: *const c_char,
) -> Result<c_uint, crate::build_result::BuildError> {
    let mut r: i32;

    ctx.job_root.0.store(true, Ordering::Relaxed);

    if style.is_null() || strcmp(style, c"fifo".as_ptr()) == 0 {
        let tmpdir = get_tmpdir(ctx);
        let fifo_name =
            xmalloc(strlen(tmpdir) + FIFO_PREFIX.to_bytes().len() + 1 + INTSTR_LENGTH + 2)
                as *mut c_char;
        sprintf(
            fifo_name,
            c"%s/GmFIFO%03lld".as_ptr(),
            tmpdir,
            make_pid() as c_longlong,
        );
        ctx.fifo_name.0.set(fifo_name);

        loop {
            r = mkfifo(fifo_name, 0o600);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            perror_with_name(ctx, c"jobserver mkfifo: ".as_ptr(), fifo_name);
            free(fifo_name as *mut c_void);
            ctx.fifo_name.0.set(null_mut());
        } else {
            let mut fds = ctx.job_fds.0.get();
            loop {
                fds[0] = open(fifo_name, O_NONBLOCK);
                if !(fds[0] == -1 && *__errno_location() == EINTR) {
                    break;
                }
            }
            ctx.job_fds.0.set(fds);
            if fds[0] < 0 {
                return Err(fatal_err(
                    ctx,
                    null::<Floc>(),
                    0,
                    c"cannot open jobserver %s: %s".as_ptr(),
                    &[
                        FmtArg::Str(fifo_name),
                        FmtArg::Str(strerror(*__errno_location())),
                    ],
                ));
            }
            loop {
                fds[1] = open(fifo_name, O_WRONLY);
                if !(fds[1] == -1 && *__errno_location() == EINTR) {
                    break;
                }
            }
            ctx.job_fds.0.set(fds);
            if fds[0] < 0 {
                return Err(fatal_err(
                    ctx,
                    null::<Floc>(),
                    0,
                    c"cannot open jobserver %s: %s".as_ptr(),
                    &[
                        FmtArg::Str(fifo_name),
                        FmtArg::Str(strerror(*__errno_location())),
                    ],
                ));
            }
            js_type_set(ctx, JsType::Fifo);
        }
    }

    if js_type_get(ctx) == JsType::None {
        if !style.is_null() && strcmp(style, c"pipe".as_ptr()) != 0 {
            return Err(fatal_err(
                ctx,
                null::<Floc>(),
                0,
                c"unknown jobserver auth style '%s'".as_ptr(),
                &[FmtArg::Str(style)],
            ));
        }
        let mut fds = ctx.job_fds.0.get();
        loop {
            r = pipe(fds.as_mut_ptr());
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        ctx.job_fds.0.set(fds);
        if r < 0 {
            return Err(pfatal_with_name_err(ctx, c"creating jobs pipe".as_ptr()));
        }
        js_type_set(ctx, JsType::Pipe);
    }

    let fds = ctx.job_fds.0.get();
    fd_noinherit(fds[0]);
    fd_noinherit(fds[1]);
    if make_job_rfd() < 0 {
        return Err(pfatal_with_name_err(ctx, c"duping jobs pipe".as_ptr()));
    }

    // Fill the pipe with tokens, one per slot, without blocking so we can
    // detect when the requested job count exceeds the pipe capacity.
    set_blocking(ctx, fds[1], false)?;
    let token_byte: c_char = token;
    for k in 0..slots {
        loop {
            r = write(fds[1], &raw const token_byte as *const c_void, 1) as i32;
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r != 1 {
            if *__errno_location() != EAGAIN {
                return Err(pfatal_with_name_err(ctx, c"init jobserver pipe".as_ptr()));
            }
            return Err(fatal_err(
                ctx,
                null::<Floc>(),
                0,
                c"requested job count (%d) is larger than system limit (%d)".as_ptr(),
                &[FmtArg::Int((slots + 1) as i64), FmtArg::Int(k as i64)],
            ));
        }
    }
    set_blocking(ctx, fds[1], true)?;
    set_blocking(ctx, fds[0], false)?;

    Ok(1)
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
) -> Result<c_uint, crate::build_result::BuildError> {
    let mut rfd: i32 = 0;
    let mut wfd: i32 = 0;

    if strncmp(auth, FIFO_PREFIX.as_ptr(), FIFO_PREFIX.to_bytes().len()) == 0 {
        let fifo_name = xstrdup(auth.add(FIFO_PREFIX.to_bytes().len()));
        ctx.fifo_name.0.set(fifo_name);
        let mut fds = ctx.job_fds.0.get();
        loop {
            fds[0] = open(fifo_name, O_RDONLY);
            if !(fds[0] == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        ctx.job_fds.0.set(fds);
        if fds[0] < 0 {
            error(
                ctx,
                null::<Floc>(),
                0,
                c"cannot open jobserver %s: %s".as_ptr(),
                &[
                    FmtArg::Str(fifo_name),
                    FmtArg::Str(strerror(*__errno_location())),
                ],
            );
            return Ok(0);
        }
        loop {
            fds[1] = open(fifo_name, O_WRONLY);
            if !(fds[1] == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        ctx.job_fds.0.set(fds);
        if fds[1] < 0 {
            error(
                ctx,
                null::<Floc>(),
                0,
                c"cannot open jobserver %s: %s".as_ptr(),
                &[
                    FmtArg::Str(fifo_name),
                    FmtArg::Str(strerror(*__errno_location())),
                ],
            );
            return Ok(0);
        }
        js_type_set(ctx, JsType::Fifo);
    } else if sscanf(auth, c"%d,%d".as_ptr(), &mut rfd, &mut wfd) == 2 {
        // A simple pipe; reject the "invalid" marker and dead descriptors.
        if rfd == -2 || wfd == -2 {
            return Ok(0);
        }
        if fcntl(rfd, F_GETFD) == -1 || fcntl(wfd, F_GETFD) == -1 {
            return Ok(0);
        }
        ctx.job_fds.0.set([rfd, wfd]);
        js_type_set(ctx, JsType::Pipe);
    } else {
        error(
            ctx,
            null::<Floc>(),
            0,
            c"invalid --jobserver-auth string '%s'".as_ptr(),
            &[FmtArg::Str(auth)],
        );
        return Ok(0);
    }

    if make_job_rfd() < 0 {
        if *__errno_location() != EBADF {
            return Err(pfatal_with_name_err(ctx, c"jobserver readfd".as_ptr()));
        }
        jobserver_clear();
        return Ok(0);
    }

    let fds = ctx.job_fds.0.get();
    set_blocking(ctx, fds[0], false)?;
    fd_noinherit(fds[0]);
    fd_noinherit(fds[1]);
    Ok(1)
}

/// Return an `xmalloc`'d `--jobserver-auth` value describing this
/// jobserver.
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_get_auth(ctx: &crate::execctx::ExecContext) -> *mut c_char {
    if js_type_get(ctx) == JsType::Fifo {
        let fifo_name = ctx.fifo_name.0.get();
        let auth = xmalloc(strlen(fifo_name) + FIFO_PREFIX.to_bytes().len() + 1) as *mut c_char;
        sprintf(auth, c"fifo:%s".as_ptr(), fifo_name);
        auth
    } else {
        let fds = ctx.job_fds.0.get();
        let auth = xmalloc(INTSTR_LENGTH * 2 + 2) as *mut c_char;
        sprintf(auth, c"%d,%d".as_ptr(), fds[0], fds[1]);
        auth
    }
}

/// The auth value handed to non-recursive children so they detect — and
/// warn about — using the jobserver without a `+` prefix. Fifo-style
/// jobservers have no such marker.
pub fn jobserver_get_invalid_auth(ctx: &crate::execctx::ExecContext) -> *const c_char {
    if js_type_get(ctx) == JsType::Fifo {
        return null();
    }
    c" --jobserver-auth=-2,-2".as_ptr()
}

/// Whether a jobserver is active.
pub fn jobserver_enabled(ctx: &crate::execctx::ExecContext) -> c_uint {
    (js_type_get(ctx) != JsType::None) as c_uint
}

/// Close down the jobserver, unlinking the fifo if we created it.
///
/// # Safety
/// Must run single-threaded. Also called from the fatal-signal path (where
/// it avoids freeing), so it reaches `main_0`'s live context through the
/// `CTX_PTR` borrow channel rather than taking `&ExecContext` — and, since
/// bare unit tests (and the fallback allocator path) may run with no
/// context installed at all, `try_with_exec_context` treats that as "no
/// fds/fifo to clear" rather than panicking, matching the former statics'
/// all-default behavior outside `main_0`.
pub unsafe fn jobserver_clear() {
    crate::entry::try_with_exec_context(|ctx| {
        let fds = ctx.job_fds.0.get();
        if fds[0] >= 0 {
            close(fds[0]);
        }
        if fds[1] >= 0 {
            close(fds[1]);
        }
        ctx.job_fds.0.set([-1, -1]);

        let fifo_name = ctx.fifo_name.0.get();
        if !fifo_name.is_null() {
            if ctx.job_root.0.load(Ordering::Relaxed) {
                let _ = crate::misc::unlink_c(fifo_name);
            }
            if !handling_fatal_signal(ctx) {
                free(fifo_name as *mut c_void);
                ctx.fifo_name.0.set(null_mut());
            }
        }

        let rfd = job_rfd(ctx);
        if rfd >= 0 {
            close(rfd);
        }
        ctx.job_rfd.0.store(-1, Ordering::Relaxed);

        js_type_set(ctx, JsType::None);
    });
}

/// Return a token to the jobserver. When `is_fatal`, die on failure;
/// otherwise just report it.
///
/// Deliberately left calling the diverging [`pfatal_with_name`] rather than
/// [`crate::output::pfatal_with_name_err`) — unlike every other call in this
/// file, its only `is_fatal` caller is job.rs's `release_jobserver_token`
/// (job.rs:1132), which already calls `fatal()` directly itself and sits in
/// `reap_children`'s 7+-entry-point fan-in explicitly called out in #432 as
/// "the hard part — do that last." Converting this one in isolation
/// wouldn't remove a `process::exit` from that call chain (the caller's own
/// `fatal()` would still fire), so it stays paired with the job.rs pass
/// (#441) rather than this slice (#432, #538, #540).
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_release(ctx: &crate::execctx::ExecContext, is_fatal: i32) {
    let mut r: i32;
    let wfd = ctx.job_fds.0.get()[1];
    let token_byte: c_char = token;
    loop {
        r = write(wfd, &raw const token_byte as *const c_void, 1) as i32;
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
/// Called from the `die_cleanup`/`clean_jobserver` shutdown path — make is
/// already on its way out by the time this runs.
///
/// `set_blocking`'s failure is therefore *reported, not fatal*: it has
/// already printed its `fcntl(O_NONBLOCK)` diagnostic (via
/// `pfatal_with_name_err`, which formats and prints before returning the
/// error), and terminating the process here would take down every other
/// tenant of an embedded `Session` — the exit belongs to `bin/make.rs`
/// alone (#442). Draining continues on the fds we have; a jobserver pipe we
/// could not put back into blocking mode at worst returns fewer tokens,
/// which the caller already diagnoses ("exiting with %u jobserver tokens
/// available; should be %u!").
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_acquire_all(ctx: &crate::execctx::ExecContext) -> c_uint {
    let mut tokens: c_uint = 0;

    // Close the write side so the read below sees EOF once the pipe drains.
    let mut fds = ctx.job_fds.0.get();
    // Diagnostic already emitted by `pfatal_with_name_err`; see above for why
    // this does not exit.
    let _blocking = set_blocking(ctx, fds[0], true);
    close(fds[1]);
    fds[1] = -1;
    ctx.job_fds.0.set(fds);

    loop {
        let mut intake: c_char = 0;
        let mut r: i32;
        loop {
            r = read(fds[0], &mut intake as *mut c_char as *mut c_void, 1) as i32;
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r != 1 {
            break;
        }
        tokens += 1;
    }

    if 0x4 & db_level(ctx) != 0 {
        crate::output::trace_out(format!("Acquired all {} jobserver tokens.\n", tokens).as_bytes());
    }

    jobserver_clear();
    tokens
}

/// Re-share the pipe fds with a recursive child (fifo jobservers pass the
/// path instead).
///
/// # Safety
/// Must run single-threaded around fork/exec.
pub unsafe fn jobserver_pre_child(ctx: &crate::execctx::ExecContext, recursive: i32) {
    if recursive != 0 && js_type_get(ctx) == JsType::Pipe {
        let fds = ctx.job_fds.0.get();
        fd_inherit(fds[0]);
        fd_inherit(fds[1]);
    }
}

/// Undo [`jobserver_pre_child`].
///
/// # Safety
/// Must run single-threaded around fork/exec.
pub unsafe fn jobserver_post_child(ctx: &crate::execctx::ExecContext, recursive: i32) {
    if recursive != 0 && js_type_get(ctx) == JsType::Pipe {
        let fds = ctx.job_fds.0.get();
        fd_noinherit(fds[0]);
        fd_noinherit(fds[1]);
    }
}

/// Called from the SIGCHLD handler: close the private read dup so a
/// blocked acquire wakes up. Async-signal-safe (only `close`). Reaches
/// `ExecContext` through the `CTX_PTR` borrow channel since a real signal
/// handler cannot carry an extra parameter, matching `child_handler`.
pub fn jobserver_signal() {
    crate::entry::try_with_exec_context(|ctx| {
        let rfd = job_rfd(ctx);
        if rfd >= 0 {
            // SAFETY: `close` is async-signal-safe, and closing a file
            // descriptor is not a Rust memory-safety hazard; any `i32` is a
            // valid argument.
            unsafe { close(rfd) };
            ctx.job_rfd.0.store(-1, Ordering::Relaxed);
        }
    });
}

/// Re-create the private read dup before waiting for a token. Returns
/// `Err(BuildError::Failure)` instead of exiting on a fatal dup failure
/// (#432 Phase B, per review on #540: `std::process::exit` belongs only in
/// `bin/make.rs`'s `main()` — no leaf function calls it directly anymore).
/// Its only caller is job.rs's scheduling loop (`new_job`), which isn't
/// itself `Result`-returning yet (that's #441's job.rs pass), so the call
/// site bridges through [`crate::output::exit_on_err`] to keep today's
/// exact exit behavior.
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_pre_acquire(
    ctx: &crate::execctx::ExecContext,
) -> Result<(), crate::build_result::BuildError> {
    if job_rfd(ctx) < 0 && ctx.job_fds.0.get()[0] >= 0 && make_job_rfd() < 0 {
        return Err(pfatal_with_name_err(ctx, c"duping jobs pipe".as_ptr()));
    }
    Ok(())
}

/// Wait (with pselect) for a token; with `timeout` nonzero give up after a
/// second. Returns `Ok(1)` if a token was read, `Ok(0)` on timeout/
/// interrupt, or `Err(BuildError::Failure)` on a fatal jobserver failure —
/// same non-diverging rule as [`jobserver_pre_acquire`] (#432 Phase B, #540).
///
/// # Safety
/// The jobserver must be set up; must run single-threaded.
pub unsafe fn jobserver_acquire(
    ctx: &crate::execctx::ExecContext,
    timeout: i32,
) -> Result<c_uint, crate::build_result::BuildError> {
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

    let rfd = ctx.job_fds.0.get()[0];
    loop {
        let mut readfds: libc::fd_set = ::core::mem::zeroed();
        FD_ZERO(&mut readfds);
        FD_SET(rfd, &mut readfds);

        let mut r = pselect(rfd + 1, &mut readfds, null_mut(), null_mut(), specp, &empty);
        if r < 0 {
            match *__errno_location() {
                EINTR => return Ok(0),
                EBADF => {
                    // The read side was closed by jobserver_signal().
                    return Err(fatal_err(
                        ctx,
                        null::<Floc>(),
                        0,
                        c"job server shut down".as_ptr(),
                        &[],
                    ));
                }
                _ => return Err(pfatal_with_name_err(ctx, c"pselect jobs pipe".as_ptr())),
            }
        }
        if r == 0 {
            return Ok(0);
        }

        let mut intake: c_char = 0;
        loop {
            r = read(rfd, &mut intake as *mut c_char as *mut c_void, 1) as i32;
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
            return Err(pfatal_with_name_err(ctx, c"read jobs pipe".as_ptr()));
        }
        return Ok((r > 0) as c_uint);
    }
}

// ---------------------------------------------------------------------------
// Output-sync mutex.

const MUTEX_PREFIX: &CStr = c"fnm:";

/// Whether the output-sync mutex is available.
pub fn osync_enabled(ctx: &crate::execctx::ExecContext) -> c_uint {
    (ctx.osync_handle.0.load(Ordering::Relaxed) >= 0) as c_uint
}

/// Create the output-sync lock file.
///
/// # Safety
/// Must run single-threaded during startup.
pub unsafe fn osync_setup(ctx: &crate::execctx::ExecContext) {
    let (h, nm) = open_named_tmpfd(ctx);
    ctx.osync_tmpfile.0.set(nm);
    ctx.osync_handle.0.store(h, Ordering::Relaxed);
    fd_noinherit(h);
    ctx.sync_root.0.store(true, Ordering::Relaxed);
}

/// Return an `xmalloc`'d `--sync-mutex` value (`fnm:<path>`) or null when
/// output sync is off.
///
/// # Safety
/// Must run single-threaded.
pub unsafe fn osync_get_mutex(ctx: &crate::execctx::ExecContext) -> *mut c_char {
    if osync_enabled(ctx) == 0 {
        return null_mut();
    }
    let osync_tmpfile = ctx.osync_tmpfile.0.get();
    let mutex = xmalloc(strlen(osync_tmpfile) + MUTEX_PREFIX.to_bytes().len() + 1) as *mut c_char;
    sprintf(mutex, c"fnm:%s".as_ptr(), osync_tmpfile);
    mutex
}

/// Adopt the output-sync mutex described by an inherited `--sync-mutex`
/// value. Returns `Ok(1)` on success, `Ok(0)` on a malformed value, or the
/// [`crate::build_result::BuildError`] a fatal open failure produced (#432
/// Phase B). Both call sites are startup-only; the one inside `main_0`
/// propagates with `?`, the other (`decode_output_sync_flags`, not yet
/// `Result`-returning) bridges through [`crate::output::exit_on_err`] to
/// keep today's exact exit behavior.
///
/// # Safety
/// `mutex` must be a valid NUL-terminated string; must run single-threaded
/// during startup.
pub unsafe fn osync_parse_mutex(
    ctx: &crate::execctx::ExecContext,
    mutex: *const c_char,
) -> Result<c_uint, crate::build_result::BuildError> {
    if strncmp(mutex, MUTEX_PREFIX.as_ptr(), MUTEX_PREFIX.to_bytes().len()) != 0 {
        error(
            ctx,
            null::<Floc>(),
            0,
            c"invalid --sync-mutex string '%s'".as_ptr(),
            &[FmtArg::Str(mutex)],
        );
        return Ok(0);
    }

    free(ctx.osync_tmpfile.0.get() as *mut c_void);
    let osync_tmpfile = xstrdup(mutex.add(MUTEX_PREFIX.to_bytes().len()));
    ctx.osync_tmpfile.0.set(osync_tmpfile);

    loop {
        let h = open(osync_tmpfile, O_WRONLY);
        ctx.osync_handle.0.store(h, Ordering::Relaxed);
        if !(h == -1 && *__errno_location() == EINTR) {
            break;
        }
    }
    if ctx.osync_handle.0.load(Ordering::Relaxed) < 0 {
        return Err(fatal_err(
            ctx,
            null::<Floc>(),
            0,
            c"cannot open output sync mutex %s: %s".as_ptr(),
            &[
                FmtArg::Str(osync_tmpfile),
                FmtArg::Str(strerror(*__errno_location())),
            ],
        ));
    }
    fd_noinherit(ctx.osync_handle.0.load(Ordering::Relaxed));
    Ok(1)
}

/// Close the output-sync mutex, unlinking the file if we created it.
///
/// # Safety
/// Must run single-threaded. Also called from the fatal-signal path, so it
/// reaches `main_0`'s live context through the `CTX_PTR` borrow channel
/// rather than taking `&ExecContext` — and, since bare unit tests may run
/// with no context installed, `try_with_exec_context` treats that as "no
/// tmpfile to clear" rather than panicking.
pub unsafe fn osync_clear() {
    crate::entry::try_with_exec_context(|ctx| {
        let h = ctx.osync_handle.0.load(Ordering::Relaxed);
        if h >= 0 {
            close(h);
            ctx.osync_handle.0.store(-1, Ordering::Relaxed);
        }
        let osync_tmpfile = ctx.osync_tmpfile.0.get();
        if ctx.sync_root.0.load(Ordering::Relaxed) && !osync_tmpfile.is_null() {
            let _ = crate::misc::unlink_c(osync_tmpfile);
            free(osync_tmpfile as *mut c_void);
            ctx.osync_tmpfile.0.set(null_mut());
        }
    });
}

/// Take the output-sync lock (a write lock on the first byte). Returns 0
/// if locking failed and output sync should be disabled.
///
/// # Safety
/// Must run single-threaded.
pub unsafe fn osync_acquire(ctx: &crate::execctx::ExecContext) -> c_uint {
    if osync_enabled(ctx) != 0 {
        let mut fl: flock = ::core::mem::zeroed();
        fl.l_type = F_WRLCK as ::core::ffi::c_short;
        fl.l_whence = SEEK_SET as ::core::ffi::c_short;
        fl.l_start = 0;
        fl.l_len = 1;
        if fcntl(
            ctx.osync_handle.0.load(Ordering::Relaxed),
            F_SETLKW,
            &mut fl,
        ) == -1
        {
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
pub unsafe fn osync_release(ctx: &crate::execctx::ExecContext) {
    if osync_enabled(ctx) != 0 {
        let mut fl: flock = ::core::mem::zeroed();
        fl.l_type = F_UNLCK as ::core::ffi::c_short;
        fl.l_whence = SEEK_SET as ::core::ffi::c_short;
        fl.l_start = 0;
        fl.l_len = 1;
        if fcntl(
            ctx.osync_handle.0.load(Ordering::Relaxed),
            F_SETLKW,
            &mut fl,
        ) == -1
        {
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
pub unsafe fn get_bad_stdin(ctx: &crate::execctx::ExecContext) -> i32 {
    let cached = ctx.bad_stdin.0.load(Ordering::Relaxed);
    if cached != -1 {
        return cached;
    }

    let mut pd: [i32; 2] = [0; 2];
    if pipe(pd.as_mut_ptr()) == 0 {
        // Close the write side so reads see EOF.
        close(pd[1]);
        fd_noinherit(pd[0]);
        match ctx
            .bad_stdin
            .0
            .compare_exchange(-1, pd[0], Ordering::Relaxed, Ordering::Relaxed)
        {
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
    // Only a regular file can meaningfully be put in append mode; a pipe or
    // terminal is left as it is.
    if crate::fs::metadata_of_fd(fd).is_ok_and(|m| m.is_file()) {
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

    if ctx.tmpfile_works.0.load(Ordering::Relaxed) {
        loop {
            fd = open(tdir, O_RDWR | O_TMPFILE | O_EXCL, 0o600_i32);
            if !(fd == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if fd >= 0 {
            return fd;
        }
        if 0x1 & db_level(ctx) != 0 {
            let err = ::core::ffi::CStr::from_ptr(strerror(*__errno_location()));
            let mut msg = Vec::with_capacity(64);
            msg.extend_from_slice(b"Cannot open '");
            msg.extend_from_slice(::core::ffi::CStr::from_ptr(tdir).to_bytes());
            msg.extend_from_slice(b"' with O_TMPFILE: ");
            msg.extend_from_slice(err.to_bytes());
            msg.extend_from_slice(b".\n");
            crate::output::trace_out(&msg);
        }
        ctx.tmpfile_works.0.store(false, Ordering::Relaxed);
    }

    // tmpfile() used the system default temp dir, so only fall back to this
    // when that's where we want the file anyway. The std replacement does
    // what tmpfile(3) does — create an unnamed (immediately unlinked) file
    // in /tmp, mode 0600 — and keeps the C 'tmpfile: <strerror>' error
    // bytes. The separate 'dup:' error path is gone: we own the fd, so no
    // dup happens (C could hit it only by exhausting fds on the dup after
    // a successful tmpfile; that now fails at the open with 'tmpfile:').
    if strcmp(tdir, c"/tmp".as_ptr()) == 0 {
        match anon_unlinked_tmp() {
            Ok(file) => fd = <std::fs::File as std::os::fd::IntoRawFd>::into_raw_fd(file),
            Err(e) => {
                error(
                    ctx,
                    null::<Floc>(),
                    0,
                    c"tmpfile: %s".as_ptr(),
                    &[FmtArg::Str(strerror(e.raw_os_error().unwrap_or(0)))],
                );
                return -1;
            }
        }
    }
    fd
}

/// tmpfile(3) via std::fs: create a fresh 0600 file in /tmp with a pid- and
/// counter-qualified name (create_new retries collisions), then unlink it so
/// only the descriptor remains.
fn anon_unlinked_tmp() -> std::io::Result<std::fs::File> {
    #[cfg(unix)]
    use std::os::unix::fs::OpenOptionsExt;
    static SEQ: ::core::sync::atomic::AtomicU32 = ::core::sync::atomic::AtomicU32::new(0);
    let pid = std::process::id();
    loop {
        let seq = SEQ.fetch_add(1, Ordering::Relaxed);
        let path = format!("/tmp/GmAnon{pid}-{seq}");
        let mut opts = std::fs::OpenOptions::new();
        opts.read(true).write(true).create_new(true);
        // WASI's `OpenOptions` has no `mode()` extension (no POSIX permission
        // bits over its capability-based filesystem API); the file is opened
        // with the target's default permissions there instead.
        #[cfg(unix)]
        opts.mode(0o600);
        match opts.open(&path) {
            Ok(f) => {
                let _ = std::fs::remove_file(&path);
                return Ok(f);
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The tmpfile(3) replacement hands back a read-write descriptor whose
    /// path is already gone: data round-trips through the fd, and /tmp
    /// holds no GmAnon file for this pid afterwards.
    #[test]
    fn anon_unlinked_tmp_round_trips_and_leaves_no_name() {
        use std::io::{Read, Seek, Write};
        let mut f = anon_unlinked_tmp().expect("anon tmp file");
        f.write_all(b"osync probe").expect("write");
        f.rewind().expect("rewind");
        let mut back = String::new();
        f.read_to_string(&mut back).expect("read");
        assert_eq!(back, "osync probe");
        let pid = std::process::id();
        let leftover = std::fs::read_dir("/tmp")
            .expect("read /tmp")
            .filter_map(|e| e.ok())
            .any(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with(&format!("GmAnon{pid}-"))
            });
        assert!(!leftover, "temp name should be unlinked immediately");
    }

    /// `osync_enabled` reflects the sign of the output-sync handle: a
    /// negative handle (the unset default) means disabled, a non-negative
    /// fd means enabled. Each test now gets its own `ExecContext`, so no
    /// shared-state lock or save/restore dance is needed.
    #[test]
    fn osync_enabled_tracks_handle_sign() {
        let ctx = crate::execctx::ExecContext::default();

        ctx.osync_handle.0.store(-1, Ordering::Relaxed);
        assert_eq!(osync_enabled(&ctx), 0, "negative handle is disabled");

        ctx.osync_handle.0.store(0, Ordering::Relaxed);
        assert_eq!(osync_enabled(&ctx), 1, "fd 0 is enabled");

        ctx.osync_handle.0.store(7, Ordering::Relaxed);
        assert_eq!(osync_enabled(&ctx), 1, "positive fd is enabled");
    }

    /// `jobserver_enabled` is true for any active style and false for
    /// `None`, and `ctx.js_type` round-trips through `js_type_get`/
    /// `js_type_set` for every variant.
    #[test]
    fn jobserver_enabled_tracks_js_type() {
        let ctx = crate::execctx::ExecContext::default();

        js_type_set(&ctx, JsType::None);
        assert!(js_type_get(&ctx) == JsType::None);
        assert_eq!(jobserver_enabled(&ctx), 0, "None is disabled");

        js_type_set(&ctx, JsType::Pipe);
        assert!(js_type_get(&ctx) == JsType::Pipe);
        assert_eq!(jobserver_enabled(&ctx), 1, "Pipe is enabled");

        js_type_set(&ctx, JsType::Fifo);
        assert!(js_type_get(&ctx) == JsType::Fifo);
        assert_eq!(jobserver_enabled(&ctx), 1, "Fifo is enabled");
    }

    /// `jobserver_get_invalid_auth` returns null for fifo jobservers (which
    /// carry no `+`-prefix marker) and the sentinel `--jobserver-auth=-2,-2`
    /// string otherwise.
    #[test]
    fn invalid_auth_is_null_only_for_fifo() {
        let ctx = crate::execctx::ExecContext::default();

        js_type_set(&ctx, JsType::Fifo);
        assert!(
            jobserver_get_invalid_auth(&ctx).is_null(),
            "fifo has no marker"
        );

        for t in [JsType::None, JsType::Pipe] {
            js_type_set(&ctx, t);
            let p = jobserver_get_invalid_auth(&ctx);
            assert!(!p.is_null(), "non-fifo returns the sentinel auth");
            // SAFETY: `p` points at a `&'static CStr` literal when non-null.
            let s = unsafe { CStr::from_ptr(p) };
            assert_eq!(s.to_bytes(), b" --jobserver-auth=-2,-2");
        }
    }

    /// `jobserver_acquire_all` drains every token sitting in the jobserver
    /// pipe and reports how many it got, closing the write side first so the
    /// read loop sees EOF once the pipe is empty.
    ///
    /// Also pins the teardown-path contract from #442: the function returns a
    /// count rather than terminating, so a `set_blocking` failure on the way
    /// out can be reported without taking the process (and, in an embedded
    /// `Session`, every other tenant) down with it.
    #[test]
    fn jobserver_acquire_all_drains_the_pipe_and_counts_tokens() {
        let ctx = crate::execctx::ExecContext::default();

        let mut fds: [i32; 2] = [-1, -1];
        assert_eq!(
            unsafe { libc::pipe(fds.as_mut_ptr()) },
            0,
            "pipe() for the fake jobserver"
        );

        // Seed the pipe the way `jobserver_setup` would: one byte per token.
        const TOKENS: usize = 5;
        let seed = [b'+'; TOKENS];
        assert_eq!(
            unsafe { libc::write(fds[1], seed.as_ptr() as *const libc::c_void, TOKENS) },
            TOKENS as isize,
            "seed the jobserver tokens"
        );

        // `-d`'s jobserver bit (0x4) also exercises the trace line.
        crate::entry::set_db_level(&ctx, 0x4);

        ctx.job_fds.0.set(fds);
        let tokens = unsafe { jobserver_acquire_all(&ctx) };
        assert_eq!(tokens as usize, TOKENS, "every seeded token is drained");

        // The write side is closed by the drain; only the read side is left.
        let left = ctx.job_fds.0.get();
        assert_eq!(left[1], -1, "write side closed and recorded as -1");
        unsafe { libc::close(left[0]) };
    }

    /// An empty jobserver pipe drains to zero tokens rather than blocking:
    /// closing the write side first makes the very first read see EOF. The
    /// untraced (`-d` off) path is the default here, covering the other side
    /// of the debug branch above.
    #[test]
    fn jobserver_acquire_all_reports_zero_for_an_empty_pipe() {
        let ctx = crate::execctx::ExecContext::default();

        let mut fds: [i32; 2] = [-1, -1];
        assert_eq!(
            unsafe { libc::pipe(fds.as_mut_ptr()) },
            0,
            "pipe() for the fake jobserver"
        );

        ctx.job_fds.0.set(fds);
        let tokens = unsafe { jobserver_acquire_all(&ctx) };
        assert_eq!(tokens, 0, "nothing to drain");

        unsafe { libc::close(ctx.job_fds.0.get()[0]) };
    }

    /// `job_rfd()` reflects `ctx.job_rfd`: negative when there is no private
    /// read dup (the default), the fd value once set.
    #[test]
    fn job_rfd_tracks_atomic() {
        let ctx = crate::execctx::ExecContext::default();
        assert_eq!(job_rfd(&ctx), -1, "default is unset");

        ctx.job_rfd.0.store(5, Ordering::Relaxed);
        assert_eq!(job_rfd(&ctx), 5, "reflects the stored fd");
    }

    /// `jobserver_signal` is a safe `fn` reached directly from the `SIGCHLD`
    /// handler with no `&ExecContext` parameter, so it resolves the live
    /// context through the `CTX_PTR` borrow channel (thread-local, so this
    /// doesn't race other tests' own contexts). It's a no-op when no private
    /// read dup is installed (`job_rfd < 0`).
    #[test]
    fn jobserver_signal_is_noop_when_unset() {
        let _ctx = crate::entry::install_default_exec_context_for_test();
        crate::entry::with_exec_context(|ctx| ctx.job_rfd.0.store(-1, Ordering::Relaxed));

        jobserver_signal();

        assert_eq!(
            crate::entry::with_exec_context(|ctx| ctx.job_rfd.0.load(Ordering::Relaxed)),
            -1,
            "stays unset; nothing was closed"
        );
    }

    /// `jobserver_pre_child`/`jobserver_post_child` only touch the fds when
    /// both `recursive` is set and the active style is `Pipe` — the other
    /// two conditions (non-recursive, or recursive but fifo-style) must
    /// short-circuit as no-ops. Drives a real pipe through all three
    /// branches so the `FD_CLOEXEC` toggle is actually observed via
    /// `fcntl`, not just inferred from the guard logic.
    #[test]
    fn pre_post_child_toggle_cloexec_only_for_recursive_pipe() {
        let ctx = crate::execctx::ExecContext::default();

        let mut fds = [-1i32; 2];
        assert_eq!(unsafe { libc::pipe(fds.as_mut_ptr()) }, 0);
        ctx.job_fds.0.set(fds);
        unsafe {
            fd_noinherit(fds[0]);
            fd_noinherit(fds[1]);
        }
        let cloexec = |fd: i32| unsafe { fcntl_retry(fd, F_GETFD) } & FD_CLOEXEC;

        // recursive == 0: no-op regardless of style.
        js_type_set(&ctx, JsType::Pipe);
        unsafe { jobserver_pre_child(&ctx, 0) };
        assert_eq!(cloexec(fds[0]), FD_CLOEXEC, "non-recursive is a no-op");

        // recursive != 0 but fifo-style: no-op.
        js_type_set(&ctx, JsType::Fifo);
        unsafe { jobserver_pre_child(&ctx, 1) };
        assert_eq!(cloexec(fds[0]), FD_CLOEXEC, "fifo style is a no-op");

        // recursive != 0 and pipe-style: actually clears FD_CLOEXEC.
        js_type_set(&ctx, JsType::Pipe);
        unsafe { jobserver_pre_child(&ctx, 1) };
        assert_eq!(cloexec(fds[0]), 0, "pre_child inherits both fds");
        assert_eq!(cloexec(fds[1]), 0, "pre_child inherits both fds");

        // jobserver_post_child undoes it under the same conditions.
        unsafe { jobserver_post_child(&ctx, 1) };
        assert_eq!(cloexec(fds[0]), FD_CLOEXEC, "post_child restores cloexec");
        assert_eq!(cloexec(fds[1]), FD_CLOEXEC, "post_child restores cloexec");

        unsafe {
            close(fds[0]);
            close(fds[1]);
        }
    }

    /// A closed read fd makes `pselect` fail with `EBADF` — the
    /// "jobserver_signal() closed the read side" case — which is fatal:
    /// `Err(BuildError::Failure)`, not a plain `Ok(0)`.
    ///
    /// The read end is first moved to a high descriptor number and closed
    /// *there*: `pipe()`/`open()` always hand back the lowest free fd, so
    /// closing a low one and expecting it to stay invalid races any test
    /// that opens a descriptor concurrently (which is how this used to flake
    /// once posixos grew a second pipe-driving test).
    #[test]
    fn jobserver_acquire_fatals_on_closed_read_fd() {
        let ctx = crate::execctx::ExecContext::default();
        let mut fds = [-1i32; 2];
        assert_eq!(unsafe { pipe(fds.as_mut_ptr()) }, 0);
        let high = unsafe { libc::fcntl(fds[0], libc::F_DUPFD, 900) };
        assert!(high >= 900, "relocate the read end above the busy range");
        unsafe {
            close(fds[0]);
            close(high);
        }
        fds[0] = high;
        ctx.job_fds.0.set(fds);

        let result = unsafe { jobserver_acquire(&ctx, 0) };

        assert_eq!(result, Err(crate::build_result::BuildError::Failure));

        unsafe { close(fds[1]) };
    }

    /// `jobserver_acquire` reads a token byte off `ctx.job_fds`'s read end. A
    /// byte already sitting in the pipe makes `pselect` return immediately
    /// readable, and the following `read` picks it up: a token was
    /// acquired. Drives the read pipe directly (no integration harness),
    /// so this branch has direct, deterministic coverage instead of relying
    /// on indirect exercise from parallel-build integration tests.
    #[test]
    fn acquire_reads_a_pending_token_byte() {
        let ctx = crate::execctx::ExecContext::default();
        let mut fds = [-1i32; 2];
        assert_eq!(unsafe { pipe(fds.as_mut_ptr()) }, 0);
        ctx.job_fds.0.set(fds);

        let token_byte = b'+' as c_char;
        assert_eq!(
            unsafe { write(fds[1], &token_byte as *const c_char as *const c_void, 1) },
            1
        );

        assert_eq!(
            unsafe { jobserver_acquire(&ctx, 0) },
            Ok(1),
            "token was read"
        );

        unsafe {
            close(fds[0]);
            close(fds[1]);
        }
    }

    /// When the write end is closed with nothing pending, the read end
    /// hits EOF: `pselect` still reports it readable, but `read` returns 0
    /// bytes, so `jobserver_acquire` reports no token acquired (`0`) rather
    /// than treating EOF as a token.
    #[test]
    fn acquire_reports_no_token_on_pipe_eof() {
        let ctx = crate::execctx::ExecContext::default();
        let mut fds = [-1i32; 2];
        assert_eq!(unsafe { pipe(fds.as_mut_ptr()) }, 0);
        ctx.job_fds.0.set(fds);
        unsafe { close(fds[1]) };

        assert_eq!(
            unsafe { jobserver_acquire(&ctx, 0) },
            Ok(0),
            "EOF is not a token"
        );

        unsafe { close(fds[0]) };
    }

    /// A nonzero `timeout` arms `pselect`'s own 1-second timer (the
    /// `Alarms don't interrupt pselect` branch) instead of blocking
    /// indefinitely. With a token already pending, `pselect` still returns
    /// readable immediately, so this exercises the timer-arming branch
    /// without actually waiting out the timeout.
    #[test]
    fn acquire_with_nonzero_timeout_still_reads_a_pending_token() {
        let ctx = crate::execctx::ExecContext::default();
        let mut fds = [-1i32; 2];
        assert_eq!(unsafe { pipe(fds.as_mut_ptr()) }, 0);
        ctx.job_fds.0.set(fds);

        let token_byte = b'+' as c_char;
        assert_eq!(
            unsafe { write(fds[1], &token_byte as *const c_char as *const c_void, 1) },
            1
        );

        assert_eq!(
            unsafe { jobserver_acquire(&ctx, 1) },
            Ok(1),
            "token was read under the armed timeout"
        );

        unsafe {
            close(fds[0]);
            close(fds[1]);
        }
    }

    /// A value that's neither `fifo:<path>` nor `<rfd>,<wfd>` is a malformed
    /// `--jobserver-auth` string, not a fatal condition: `Ok(0)`.
    #[test]
    fn jobserver_parse_auth_rejects_malformed_string() {
        let ctx = crate::execctx::ExecContext::default();
        let result = unsafe { jobserver_parse_auth(&ctx, c"not-an-auth-string".as_ptr()) };
        assert_eq!(result, Ok(0));
    }

    /// `-2,-2` is the sentinel a parent make uses to say "no usable
    /// jobserver was inherited" — rejected without touching fds, `Ok(0)`.
    #[test]
    fn jobserver_parse_auth_rejects_invalid_marker() {
        let ctx = crate::execctx::ExecContext::default();
        let result = unsafe { jobserver_parse_auth(&ctx, c"-2,-2".as_ptr()) };
        assert_eq!(result, Ok(0));
    }

    /// A syntactically valid `<rfd>,<wfd>` pair naming closed/nonexistent
    /// descriptors fails the `fcntl(F_GETFD)` liveness check, `Ok(0)`.
    #[test]
    fn jobserver_parse_auth_rejects_dead_descriptors() {
        let ctx = crate::execctx::ExecContext::default();
        let result = unsafe { jobserver_parse_auth(&ctx, c"12345,12346".as_ptr()) };
        assert_eq!(result, Ok(0));
    }

    /// A `fifo:<path>` value naming a path that can't be opened hits the
    /// "cannot open jobserver" `error()`-then-`Ok(0)` path (not fatal —
    /// only fcntl-after-open failures reach `pfatal_with_name_err`).
    #[test]
    fn jobserver_parse_auth_reports_unopenable_fifo() {
        let ctx = crate::execctx::ExecContext::default();
        let result = unsafe {
            jobserver_parse_auth(
                &ctx,
                c"fifo:/nonexistent-dir-for-jobserver-test/fifo".as_ptr(),
            )
        };
        assert_eq!(result, Ok(0));
    }

    /// A live `<rfd>,<wfd>` pair (a real pipe) is adopted successfully:
    /// `Ok(1)`, `job_fds` set to the pair, and the read side left
    /// non-blocking (exercises the success path through `set_blocking`/
    /// `fd_noinherit`).
    #[test]
    fn jobserver_parse_auth_adopts_a_live_pipe() {
        let ctx = crate::execctx::ExecContext::default();
        let mut fds = [-1i32; 2];
        assert_eq!(unsafe { pipe(fds.as_mut_ptr()) }, 0);

        let auth = std::ffi::CString::new(format!("{},{}", fds[0], fds[1])).unwrap();
        let result = unsafe { jobserver_parse_auth(&ctx, auth.as_ptr()) };

        assert_eq!(result, Ok(1));
        assert_eq!(ctx.job_fds.0.get(), fds);
        assert!(js_type_get(&ctx) == JsType::Pipe);

        unsafe {
            close(fds[0]);
            close(fds[1]);
        }
    }

    /// `osync_parse_mutex` rejects a value without the `fnm:` prefix without
    /// touching any state, returning `Ok(0)` (a malformed value, not a fatal
    /// condition).
    #[test]
    fn osync_parse_mutex_rejects_missing_prefix() {
        let ctx = crate::execctx::ExecContext::default();
        let result = unsafe { osync_parse_mutex(&ctx, c"not-a-mutex-string".as_ptr()) };
        assert_eq!(result, Ok(0));
    }

    /// A well-formed `fnm:<path>` value whose path can't be opened (parent
    /// directory doesn't exist) hits the `fatal_err` path (#432 Phase B):
    /// `osync_parse_mutex` returns `Err(BuildError::Failure)` instead of
    /// exiting, and the context is marked dying by the shared `die_cleanup`
    /// it runs on the way out.
    #[test]
    fn osync_parse_mutex_fatals_on_unopenable_path() {
        let ctx = crate::execctx::ExecContext::default();
        let mutex = c"fnm:/nonexistent-dir-for-osync-test/mutex";

        let result = unsafe { osync_parse_mutex(&ctx, mutex.as_ptr()) };

        assert_eq!(result, Err(crate::build_result::BuildError::Failure));
        assert!(ctx.dying.0.load(Ordering::Relaxed));
    }

    /// `jobserver_setup` with a `style` that is neither `fifo` nor `pipe`
    /// (and null-style is the fifo default, so this only fires once fifo
    /// setup was skipped by naming an unknown style) hits the "unknown
    /// jobserver auth style" `fatal_err` path (#432 Phase B) — the one
    /// `jobserver_setup` failure controllable without forcing a real fd
    /// exhaustion.
    #[test]
    fn jobserver_setup_rejects_unknown_style() {
        let ctx = crate::execctx::ExecContext::default();

        let result = unsafe { jobserver_setup(&ctx, 1, c"bogus-style".as_ptr()) };

        assert_eq!(result, Err(crate::build_result::BuildError::Failure));
        assert!(ctx.dying.0.load(Ordering::Relaxed));
    }
}
