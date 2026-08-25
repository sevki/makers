//! Output handling: the `message`/`error`/`fatal` printers and the
//! output-sync machinery that captures recipe output into temp files and
//! dumps it atomically.
//!
//! Port of `output.c`. The variadic printers keep their C ABI because they
//! are called with printf-style argument lists from all over the crate; the
//! [`msg`] submodule provides native-Rust counterparts.

use {
    ::core::{ffi::c_uint, ptr::null},
    std::sync::atomic::Ordering,
};

use libc::{__errno_location, close, ftruncate, lseek, strerror, EINTR, SEEK_END, SEEK_SET};

use crate::{
    execctx::ExecContext,
    ffi_types::{size_t, uintmax_t},
    floc::Floc,
    misc::{open_anon_tmpfd, writebuf},
    posixos::{
        check_io_state,
        fd_noinherit,
        fd_reset_append,
        fd_set_append,
        osync_acquire,
        osync_clear,
        osync_release,
    },
};

/// Per-target output state: temp-file descriptors for stdout/stderr while
/// output sync is active.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output {
    pub out: i32,
    pub err: i32,
    pub(crate) syncout: ::core::ffi::c_uint,
}

impl output {
    pub fn syncout(&self) -> ::core::ffi::c_uint {
        self.syncout
    }

    pub fn set_syncout(&mut self, val: ::core::ffi::c_uint) {
        self.syncout = val;
    }
}

/// Bytes needed to print an integer of type `uintmax_t`: digits (53/22
/// approximates bits-to-decimal-digits), sign, and NUL (from makeint.h).
pub const INTSTR_LENGTH: usize = 53 * ::core::mem::size_of::<uintmax_t>() / 22 + 3;

pub const OUTPUT_SYNC_NONE: i32 = 0;
pub const OUTPUT_SYNC_RECURSE: i32 = 3;
pub const MAKE_FAILURE: i32 = 2;

/// `check_io_state` bits (see os.h).
const IO_COMBINED_OUTERR: c_uint = 0x0002;
const IO_STDOUT_OK: c_uint = 0x0008;
const IO_STDERR_OK: c_uint = 0x0010;

/// The active output-sync target (the former `static mut output_context`),
/// now owned per-run on `ExecContext`. Reads resolve the *live* context
/// through the `CTX_PTR` borrow channel rather than the `&ExecContext` a
/// printer was handed: the printers are reachable with a throwaway context
/// (plugin ABI, the fatal-signal handler's prefix-free path) and must still
/// route through the real run's sync state, exactly as the process global
/// did. Null (straight-to-stdio) when no context is installed — startup
/// before `main_0` and bare unit tests, where the former global was null
/// too.
pub fn output_context() -> *mut output {
    crate::entry::try_with_exec_context(|c| c.output_context.0.get())
        .unwrap_or(::core::ptr::null_mut())
}

/// Set the active output-sync target on the live run (see
/// [`output_context`]). A no-op when no context is installed, mirroring the
/// null-global steady state outside `main_0`'s extent.
pub fn set_output_context(value: *mut output) {
    let _ = crate::entry::try_with_exec_context(|c| c.output_context.0.set(value));
}
/// Whether the working-directory "Entering directory" trace has been emitted.
///
/// This is a one-shot latch — set once make logs the trace, so the matching
/// "Leaving directory" is emitted and `MAKE_RESTARTS` is prefixed with `-`. It
/// lives on the owned per-run `Options` (the former `STDIO_TRACED` global
/// atomic), reached through the `with_options`/`OPTIONS_PTR` channel: the
/// `output_start` writer runs on the shared output path, reachable from the
/// `gmk_eval` throwaway-context path, so both ends resolve to `main_0`'s real
/// run state rather than a throwaway `ExecContext`.
pub fn stdio_traced(ctx: &ExecContext) -> bool {
    crate::entry::with_options(ctx, |o| o.stdio_traced.get())
}

/// Record whether the working-directory enter trace has been emitted.
pub fn set_stdio_traced(ctx: &ExecContext, value: bool) {
    crate::entry::with_options(ctx, |o| o.stdio_traced.set(value));
}
pub const OUTPUT_NONE: i32 = -1;
/// errno of the first failed write to Rust stdout, 0 if none — the
/// process-global sticky error libc kept in `ferror(stdout)`, read by the
/// `close_stdout` atexit handler (which is why this isn't on `ExecContext`).
static STDOUT_ERRNO: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

/// Record a failed stdout write (keeps the first errno, like `ferror`'s
/// sticky flag).
pub fn record_stdout_error(e: &std::io::Error) {
    let _ = STDOUT_ERRNO.compare_exchange(
        0,
        e.raw_os_error().unwrap_or(libc::EIO),
        Ordering::Relaxed,
        Ordering::Relaxed,
    );
}

/// errno of the first failed stdout write, 0 if stdout never failed.
pub fn stdout_error() -> i32 {
    STDOUT_ERRNO.load(Ordering::Relaxed)
}

/// Emit a debug/trace line through `ctx`'s stdout sink and flush — the C
/// `printf` + `fflush(stdout)` pattern of the `-d` traces. Callers format the
/// bytes themselves. This is the entry point a host running several sessions
/// in one process wants: give each session's `ExecContext` its own `Out`
/// (an in-memory buffer, a per-connection socket, ...) and its trace/recipe
/// output never touches another session's. Only `Out` needs a bound here —
/// this function never touches `ctx.stderr` — but `Err` still has to appear
/// in the signature since it's part of `ExecContext`'s type.
pub fn trace_out_ctx<Out: std::io::Write, Err: std::io::Write>(
    ctx: &ExecContext<Out, Err>,
    bytes: &[u8],
) {
    let mut o = ctx.stdout.borrow_mut();
    // `StdoutSink::write`/`flush` already feed `record_stdout_error` for the
    // default sink; a non-default `Out` reports its own failures how it
    // sees fit; there is no process-wide stdout to be sticky about there.
    let _ = o.write_all(bytes).and_then(|()| o.flush());
}

/// Ctx-less compatibility wrapper for the callers not yet converted to carry
/// an `&ExecContext` (hash.rs, misc.rs's `spin`, ...). Reaches the live
/// `main_0` context through the same borrow channel `output_context`/
/// `stdio_traced` use, so it lands on the *installed* session's sink rather
/// than always meaning "the process's real stdout"; falls back to a bare
/// default sink only when no context is installed at all (startup, bare unit
/// tests).
pub fn trace_out(bytes: &[u8]) {
    if crate::entry::try_with_exec_context(|ctx| trace_out_ctx(ctx, bytes)).is_some() {
        return;
    }
    use std::io::Write;
    let mut o = std::io::stdout();
    if let Err(e) = o.write_all(bytes).and_then(|()| o.flush()) {
        record_stdout_error(&e);
    }
}

/// Concatenate byte pieces into one line and emit it via [`trace_out`] —
/// the multi-argument `printf` debug-trace shape.
pub fn trace_parts(parts: &[&[u8]]) {
    let mut msg = Vec::with_capacity(parts.iter().map(|p| p.len()).sum());
    for p in parts {
        msg.extend_from_slice(p);
    }
    trace_out(&msg);
}
/// `msg` and `out` are already-safe Rust types at this boundary (a `&CStr`
/// and `Option<&mut output>`) — no raw pointers cross into this function, so
/// unlike its c2rust-translated ancestor it needs no `unsafe fn` marker;
/// only the sync-fd fast path's raw syscalls (no safe std equivalent for
/// append-without-a-`File`-handle) stay behind a narrow `unsafe` block.
fn _outputs(ctx: &ExecContext, out: Option<&mut output>, is_err: bool, msg: &::core::ffi::CStr) {
    let bytes = msg.to_bytes();
    if let Some(out) = out {
        if out.syncout() as i32 != 0 {
            let fd: i32 = if is_err { out.err } else { out.out };
            if fd != OUTPUT_NONE {
                let len: size_t = bytes.len() as size_t;
                // SAFETY: `fd` is a valid, open descriptor the output-sync
                // machinery owns for the run's duration.
                unsafe {
                    let mut r: i32;
                    loop {
                        r = lseek(fd, 0, 2) as i32;
                        if !(r == -1 && *__errno_location() == EINTR) {
                            break;
                        }
                    }
                    writebuf(fd, bytes.as_ptr() as *const ::core::ffi::c_void, len);
                }
                return;
            }
        }
    }
    // Every non-synced make message lands here: write it through ctx's
    // stdout/stderr sink and flush, exactly the fputs+fflush the C did.
    // Reading the sink off `ctx` (rather than a fresh `std::io::stdout()`
    // handle) is what lets a buffer-backed session's messages land in its
    // own sink instead of the process's real stdout.
    use std::io::Write;
    if is_err {
        let mut f = ctx.stderr.borrow_mut();
        let _ = f.write_all(bytes);
        let _ = f.flush();
    } else {
        let mut f = ctx.stdout.borrow_mut();
        if let Err(e) = f.write_all(bytes).and_then(|()| f.flush()) {
            record_stdout_error(&e);
        }
    }
}
/// Print an entering/leaving-directory line (returns 1).
///
/// # Safety
/// `ctx.program`/`ctx.starting_directory` must be null or valid
/// NUL-terminated strings (the c2rust pointer contract they always carry).
pub unsafe fn log_working_directory(ctx: &ExecContext, entering: i32) -> i32 {
    let makelevel = ctx.makelevel();
    // `program` is null only on context-less paths handed a throwaway
    // `ExecContext` (plugin ABI); the C original could not get here with a
    // null `program` global. Fall back to the plain name like
    // `msg::program_name` rather than passing null to the formatter.
    let program = ctx.program.0.get();
    let program = if program.is_null() {
        c"make".as_ptr()
    } else {
        program
    };
    let starting_directory = ctx.starting_directory.0.get();
    let fmt: *const ::core::ffi::c_char;
    if makelevel == 0 {
        if starting_directory.is_null() {
            if entering != 0 {
                fmt = c"%s: Entering an unknown directory\n".as_ptr();
            } else {
                fmt = c"%s: Leaving an unknown directory\n".as_ptr();
            }
        } else if entering != 0 {
            fmt = c"%s: Entering directory '%s'\n".as_ptr();
        } else {
            fmt = c"%s: Leaving directory '%s'\n".as_ptr();
        }
    } else if starting_directory.is_null() {
        if entering != 0 {
            fmt = c"%s[%u]: Entering an unknown directory\n".as_ptr();
        } else {
            fmt = c"%s[%u]: Leaving an unknown directory\n".as_ptr();
        }
    } else if entering != 0 {
        fmt = c"%s[%u]: Entering directory '%s'\n".as_ptr();
    } else {
        fmt = c"%s[%u]: Leaving directory '%s'\n".as_ptr();
    }
    // The line is built in an owned buffer per call (the former grow-only
    // `static mut buf`/`len` pair); `_outputs` copies the bytes before
    // returning, so the local's lifetime is enough.
    let mut line: Vec<u8> = Vec::new();
    if crate::entry::opt_print_data_base(ctx) {
        line.extend_from_slice(b"# ");
    }
    // The `%u` slot only exists in the `makelevel != 0` formats and the `%s`
    // directory slot only when `starting_directory` is non-null; extra args
    // are ignored by the formatter, matching the sprintf call ladder.
    let args: &[FmtArg] = if makelevel == 0 {
        &[FmtArg::Str(program), FmtArg::Str(starting_directory)]
    } else {
        &[
            FmtArg::Str(program),
            FmtArg::Uint(makelevel as u64),
            FmtArg::Str(starting_directory),
        ]
    };
    vformat_into(&mut line, fmt, args);
    line.push(0);
    // `line` was just built with exactly one trailing NUL and no interior
    // one (the format strings above never embed one), so this always
    // succeeds.
    let line = ::core::ffi::CStr::from_bytes_with_nul(&line).expect("no interior NUL");
    _outputs(ctx, None, false, line);
    1
}
/// `perror`'s exact output — `<what>: <strerror(errno)>\n` — for the pump's
/// error paths, sourcing errno from the `io::Error`. Generic over the
/// destination (rather than reaching into `ExecContext` itself) so a caller
/// whose pump destination *is* the stderr sink can reuse that same already
/// -borrowed handle instead of taking a second, conflicting `RefCell`
/// borrow of it.
fn pump_perror<W: std::io::Write>(w: &mut W, what: &str, err: &std::io::Error) {
    // SAFETY: `strerror` returns a static NUL-terminated message for any
    // errno value.
    let es = unsafe { ::core::ffi::CStr::from_ptr(strerror(err.raw_os_error().unwrap_or(0))) };
    let _ = w.write_all(what.as_bytes());
    let _ = w.write_all(b": ");
    let _ = w.write_all(es.to_bytes());
    let _ = w.write_all(b"\n");
}
/// The pump's copy loop over any seekable reader and writer: rewind, then
/// copy 8 KiB chunks until EOF, retrying `Interrupted` reads and stopping
/// (after a perror-style line, via the caller-supplied `report_err`) on any
/// other read or write error — exactly the C `lseek`/`read`/`fwrite` loop's
/// behavior and error text. `report_err` (rather than a `pump_perror` call
/// baked in here) lets each caller decide where the diagnostic goes without
/// `pump_copy` itself needing to know about `ExecContext` or risk a second
/// borrow of a sink already held as `dst`.
fn pump_copy<R, W>(
    src: &mut R,
    dst: &mut W,
    dst_is_stdout: bool,
    mut report_err: impl FnMut(&mut W, &str, &std::io::Error),
) where
    R: std::io::Read + std::io::Seek,
    W: std::io::Write,
{
    if let Err(e) = src.seek(std::io::SeekFrom::Start(0)) {
        report_err(dst, "lseek()", &e);
    }
    let mut buffer = [0u8; 8192];
    loop {
        let len = loop {
            match src.read(&mut buffer) {
                Ok(n) => break n,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => {
                    report_err(dst, "read()", &e);
                    break 0;
                }
            }
        };
        if len == 0 {
            break;
        }
        // Keep the C error text: the write is what fwrite() did. A failed
        // stdout write also set `ferror(stdout)` in C; keep that visible to
        // the `close_stdout` exit check via the sticky errno.
        if let Err(e) = dst.write_all(&buffer[..len]).and_then(|()| dst.flush()) {
            report_err(dst, "fwrite()", &e);
            if dst_is_stdout {
                record_stdout_error(&e);
            }
            break;
        }
    }
}
/// Copy everything from fd `from` to stdout or stderr, from the beginning.
///
/// # Safety
/// `from` must be an open, seekable fd this function may read (ownership is
/// borrowed, not taken).
pub unsafe fn pump_from_tmp(ctx: &ExecContext, from: i32, to_stderr: bool) {
    use std::os::fd::FromRawFd;
    // No libc printers remain, so there is no libc stream buffer to flush
    // ahead of the pump; `pump_copy` writes through the same Rust stream the
    // rest of make uses, which keeps ordering inherent (as the C fwrite's
    // shared FILE buffer once did).
    // Borrow the fd as a File without taking ownership.
    let mut src = ::core::mem::ManuallyDrop::new(std::fs::File::from_raw_fd(from));
    if to_stderr {
        // `dst` already *is* the borrowed stderr sink; report through it
        // directly rather than taking a second borrow of `ctx.stderr`.
        pump_copy(
            &mut *src,
            &mut *ctx.stderr.borrow_mut(),
            false,
            |dst, what, e| pump_perror(dst, what, e),
        );
    } else {
        // `dst` is the stdout sink here, so the stderr diagnostic borrows
        // the separate `ctx.stderr` `RefCell` freely.
        pump_copy(
            &mut *src,
            &mut *ctx.stdout.borrow_mut(),
            true,
            |_dst, what, e| pump_perror(&mut *ctx.stderr.borrow_mut(), what, e),
        );
    }
}
/// Create an anonymous temp fd in append mode for output sync.
///
/// # Safety
/// Always safe; unsafe for C-API compatibility.
pub unsafe fn output_tmpfd(ctx: &ExecContext) -> i32 {
    let fd: i32 = open_anon_tmpfd(ctx);
    fd_set_append(fd);
    fd
}
/// Set up the output-sync temp files for `out`, disabling output sync
/// (with a message) on failure.
///
/// # Safety
/// `out` must point to a valid `output`; must run single-threaded.
pub unsafe fn setup_tmpfile(ctx: &ExecContext, out: *mut output) {
    // Guards against re-entrant tmpfile setup (the C code's recursion check).
    if ctx.output_in_setup.0.load(Ordering::Relaxed) {
        return;
    }
    ctx.output_in_setup.0.store(true, Ordering::Relaxed);
    let io_state: ::core::ffi::c_uint = check_io_state(ctx);
    // The block falls through to the error handler below on any failure;
    // reaching its end is the success path (the C code used `goto`).
    'setup: {
        if io_state & (IO_STDOUT_OK | IO_STDERR_OK) == 0 {
            perror_with_name(
                ctx,
                c"output-sync suppressed: ".as_ptr(),
                c"stderr".as_ptr(),
            );
            break 'setup;
        }
        if io_state & IO_STDOUT_OK != 0 {
            let fd: i32 = output_tmpfd(ctx);
            if fd < 0 {
                break 'setup;
            }
            fd_noinherit(fd);
            (*out).out = fd;
        }
        if io_state & IO_STDERR_OK != 0 {
            if (*out).out != OUTPUT_NONE && io_state & IO_COMBINED_OUTERR != 0 {
                (*out).err = (*out).out;
            } else {
                let fd_0: i32 = output_tmpfd(ctx);
                if fd_0 < 0 {
                    break 'setup;
                }
                fd_noinherit(fd_0);
                (*out).err = fd_0;
            }
        }
        ctx.output_in_setup.0.store(false, Ordering::Relaxed);
        return;
    }
    error(
        ctx,
        null::<Floc>(),
        0,
        c"cannot open output-sync lock file: suppressing output-sync".as_ptr(),
        &[],
    );
    output_close(ctx, out);
    crate::entry::with_options(ctx, |o| o.output_sync.set(OUTPUT_SYNC_NONE));
    osync_clear();
    ctx.output_in_setup.0.store(false, Ordering::Relaxed);
}
/// Dump any captured output under the output-sync lock and truncate the
/// temp files for reuse.
///
/// # Safety
/// `out` must point to a valid `output`; must run single-threaded.
pub unsafe fn output_dump(ctx: &ExecContext, out: *mut output) {
    let outfd_not_empty: i32 =
        ((*out).out != OUTPUT_NONE && lseek((*out).out, 0, SEEK_END) > 0) as i32;
    let errfd_not_empty: i32 =
        ((*out).err != OUTPUT_NONE && lseek((*out).err, 0, SEEK_END) > 0) as i32;
    if outfd_not_empty != 0 || errfd_not_empty != 0 {
        let mut traced: i32 = 0;
        if osync_acquire(ctx) == 0 {
            error(
                ctx,
                null::<Floc>(),
                0,
                c"warning: cannot acquire output lock: disabling output sync".as_ptr(),
                &[],
            );
            osync_clear();
        }
        if crate::entry::opt_output_sync(ctx) != OUTPUT_SYNC_RECURSE
            && crate::entry::should_print_dir_mirror(ctx) != 0
        {
            traced = log_working_directory(ctx, 1);
        }
        if outfd_not_empty != 0 {
            pump_from_tmp(ctx, (*out).out, false);
        }
        if errfd_not_empty != 0 && (*out).err != (*out).out {
            pump_from_tmp(ctx, (*out).err, true);
        }
        if traced != 0 {
            log_working_directory(ctx, 0);
        }
        osync_release(ctx);
        if (*out).out != OUTPUT_NONE {
            let mut e: i32;
            lseek((*out).out, 0, SEEK_SET);
            loop {
                e = ftruncate((*out).out, 0);
                if !(e == -1 && *__errno_location() == EINTR) {
                    break;
                }
            }
        }
        if (*out).err != OUTPUT_NONE && (*out).err != (*out).out {
            let mut e_0: i32;
            lseek((*out).err, 0, SEEK_SET);
            loop {
                e_0 = ftruncate((*out).err, 0);
                if !(e_0 == -1 && *__errno_location() == EINTR) {
                    break;
                }
            }
        }
    }
}
/// Initialize `out` (or, when null, switch stdout/stderr to append mode).
///
/// # Safety
/// `out` must be null or point to a valid `output`; must run
/// single-threaded.
pub unsafe fn output_init(ctx: &ExecContext, out: *mut output) {
    if !out.is_null() {
        (*out).err = OUTPUT_NONE;
        (*out).out = (*out).err;
        (*out).set_syncout(
            (crate::entry::opt_output_sync(ctx) != 0) as i32 as ::core::ffi::c_uint
                as ::core::ffi::c_uint,
        );
        return;
    }
    ctx.stdout_flags
        .0
        .store(fd_set_append(libc::STDOUT_FILENO), Ordering::Relaxed);
    ctx.stderr_flags
        .0
        .store(fd_set_append(libc::STDERR_FILENO), Ordering::Relaxed);
}
/// Dump and close `out`'s temp files (or, when null, restore
/// stdout/stderr).
///
/// # Safety
/// `out` must be null or point to a valid `output`; must run
/// single-threaded.
pub unsafe fn output_close(ctx: &ExecContext, out: *mut output) {
    if out.is_null() {
        if stdio_traced(ctx) {
            log_working_directory(ctx, 0);
        }
        fd_reset_append(
            libc::STDOUT_FILENO,
            ctx.stdout_flags.0.load(Ordering::Relaxed),
        );
        fd_reset_append(
            libc::STDERR_FILENO,
            ctx.stderr_flags.0.load(Ordering::Relaxed),
        );
        return;
    }
    output_dump(ctx, out);
    if (*out).out >= 0 {
        close((*out).out);
    }
    if (*out).err >= 0 && (*out).err != (*out).out {
        close((*out).err);
    }
    output_init(ctx, out);
}
/// Lazily set up output sync and the enter-directory trace before the
/// first real output.
///
/// # Safety
/// Must run single-threaded: touches output and trace globals.
pub unsafe fn output_start(ctx: &ExecContext) {
    let osync = output_context();
    if !osync.is_null()
        && (*osync).syncout() as i32 != 0
        && !((*osync).out >= 0 || (*osync).err >= 0)
    {
        setup_tmpfile(ctx, osync);
    }
    if (crate::entry::opt_output_sync(ctx) == OUTPUT_SYNC_NONE
        || crate::entry::opt_output_sync(ctx) == OUTPUT_SYNC_RECURSE)
        && !stdio_traced(ctx)
        && crate::entry::should_print_dir_mirror(ctx) != 0
    {
        set_stdio_traced(ctx, log_working_directory(ctx, 1) != 0);
    }
}
/// Write `msg` to stdout or stderr (or the sync temp file), starting
/// output first.
///
/// # Safety
/// `msg` must be null or a valid NUL-terminated string.
pub unsafe fn outputs(ctx: &ExecContext, is_err: i32, msg: *const ::core::ffi::c_char) {
    if msg.is_null() || *msg as i32 == 0 {
        return;
    }
    output_start(ctx);
    _outputs(
        ctx,
        output_context().as_mut(),
        is_err != 0,
        ::core::ffi::CStr::from_ptr(msg),
    );
}
// The former shared, growing printer buffer (`static mut fmtbuf` and its
// `get_buffer` accessor) is gone: each printer builds its line in an owned
// `Vec<u8>` and hands `outputs` a pointer into it — `outputs` copies the
// bytes before returning, so no allocation outlives its call.
/// One argument to the printf-subset formatter that replaced C varargs.
#[derive(Copy, Clone)]
pub enum FmtArg {
    Str(*const ::core::ffi::c_char),
    Int(i64),
    Uint(u64),
    Ptr(*const ::core::ffi::c_void),
}

/// Render printf-style `fmt` with `args` into `out`, byte-for-byte like the
/// C printf subset this codebase uses: %s %d %i %u %x %c %p %% with flags,
/// width and precision (including `*`), and h/l/ll/z length modifiers.
pub unsafe fn vformat_into(out: &mut Vec<u8>, fmt: *const ::core::ffi::c_char, args: &[FmtArg]) {
    let bytes = ::core::ffi::CStr::from_ptr(fmt).to_bytes();
    let mut args = args.iter();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'%' {
            out.push(bytes[i]);
            i += 1;
            continue;
        }
        i += 1;
        let mut zero_pad = false;
        let mut left = false;
        while i < bytes.len() {
            match bytes[i] {
                b'0' => zero_pad = true,
                b'-' => left = true,
                _ => break,
            }
            i += 1;
        }
        let mut width = 0usize;
        if i < bytes.len() && bytes[i] == b'*' {
            if let Some(FmtArg::Int(n)) = args.next() {
                width = (*n).max(0) as usize;
            }
            i += 1;
        } else {
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                width = width * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }
        }
        let mut precision: Option<usize> = None;
        if i < bytes.len() && bytes[i] == b'.' {
            i += 1;
            if i < bytes.len() && bytes[i] == b'*' {
                if let Some(FmtArg::Int(n)) = args.next() {
                    precision = Some((*n).max(0) as usize);
                }
                i += 1;
            } else {
                let mut p = 0usize;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    p = p * 10 + (bytes[i] - b'0') as usize;
                    i += 1;
                }
                precision = Some(p);
            }
        }
        while i < bytes.len() && matches!(bytes[i], b'l' | b'h' | b'z') {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let conv = bytes[i];
        i += 1;
        let piece: Vec<u8> = match conv {
            b'%' => vec![b'%'],
            b's' => {
                let p = match args.next() {
                    Some(FmtArg::Str(p)) => *p,
                    _ => ::core::ptr::null(),
                };
                let mut v = if p.is_null() {
                    b"(null)".to_vec()
                } else {
                    ::core::ffi::CStr::from_ptr(p).to_bytes().to_vec()
                };
                if let Some(prec) = precision {
                    v.truncate(prec);
                }
                v
            }
            b'd' | b'i' => {
                let n = match args.next() {
                    Some(FmtArg::Int(n)) => *n,
                    Some(FmtArg::Uint(n)) => *n as i64,
                    _ => 0,
                };
                n.to_string().into_bytes()
            }
            b'u' => {
                let n = match args.next() {
                    Some(FmtArg::Uint(n)) => *n,
                    Some(FmtArg::Int(n)) => *n as u64,
                    _ => 0,
                };
                n.to_string().into_bytes()
            }
            b'x' => {
                let n = match args.next() {
                    Some(FmtArg::Uint(n)) => *n,
                    Some(FmtArg::Int(n)) => *n as u64,
                    _ => 0,
                };
                format!("{:x}", n).into_bytes()
            }
            b'c' => {
                let n = match args.next() {
                    Some(FmtArg::Int(n)) => *n,
                    Some(FmtArg::Uint(n)) => *n as i64,
                    _ => 0,
                };
                vec![n as u8]
            }
            b'p' => {
                let p = match args.next() {
                    Some(FmtArg::Ptr(p)) => *p as usize,
                    Some(FmtArg::Str(p)) => *p as usize,
                    _ => 0,
                };
                format!("0x{:x}", p).into_bytes()
            }
            other => {
                out.push(b'%');
                vec![other]
            }
        };
        if piece.len() < width {
            let pad = width - piece.len();
            if left {
                out.extend_from_slice(&piece);
                out.extend(::core::iter::repeat(b' ').take(pad));
            } else {
                let fill = if zero_pad { b'0' } else { b' ' };
                out.extend(::core::iter::repeat(fill).take(pad));
                out.extend_from_slice(&piece);
            }
        } else {
            out.extend_from_slice(&piece);
        }
    }
}

/// Append `s`'s bytes to `out`, doing nothing when `s` is `None` (the old
/// NULL-pointer sentinel).
fn push_cstr(out: &mut Vec<u8>, s: Option<&::core::ffi::CStr>) {
    if let Some(s) = s {
        out.extend_from_slice(s.to_bytes());
    }
}

unsafe fn push_program_prefix(
    ctx: &ExecContext,
    out: &mut Vec<u8>,
    makelevel: u32,
    fatal_marker: bool,
) {
    let program = ctx.program.0.get();
    push_cstr(
        out,
        (!program.is_null()).then(|| ::core::ffi::CStr::from_ptr(program)),
    );
    if makelevel == 0 {
        out.extend_from_slice(b": ");
    } else {
        out.push(b'[');
        out.extend_from_slice(makelevel.to_string().as_bytes());
        out.extend_from_slice(b"]: ");
    }
    if fatal_marker {
        out.extend_from_slice(b"*** ");
    }
}

unsafe fn push_error_prefix(
    ctx: &ExecContext,
    out: &mut Vec<u8>,
    flocp: *const Floc,
    makelevel: u32,
    fatal_marker: bool,
) {
    if let Some(fl) = flocp.as_ref().filter(|fl| !fl.filenm.is_null()) {
        // `filenm` is non-null in this arm, so it is always `Some`.
        push_cstr(out, Some(::core::ffi::CStr::from_ptr(fl.filenm)));
        out.push(b':');
        out.extend_from_slice(fl.lineno.wrapping_add(fl.offset).to_string().as_bytes());
        out.extend_from_slice(b": ");
        if fatal_marker {
            out.extend_from_slice(b"*** ");
        }
    } else {
        push_program_prefix(ctx, out, makelevel, fatal_marker);
    }
}

/// printf-subset message to stdout, optionally prefixed with the program name.
///
/// # Safety
/// `fmt` must be a valid NUL-terminated format string. The format
/// specifiers must match `args`.
pub unsafe fn message(
    ctx: &ExecContext,
    prefix: i32,
    _len: size_t,
    fmt: *const ::core::ffi::c_char,
    args: &[FmtArg],
) {
    let makelevel = ctx.makelevel();
    let mut out: Vec<u8> = Vec::new();
    if prefix != 0 {
        push_program_prefix(ctx, &mut out, makelevel, false);
    }
    vformat_into(&mut out, fmt, args);
    out.push(b'\n');
    out.push(0);
    outputs(ctx, 0, out.as_ptr() as *const ::core::ffi::c_char);
}

/// printf-subset error to stderr with a file:line or program prefix.
///
/// # Safety
/// `flocp` must be null or valid. `fmt` must be a valid NUL-terminated
/// format string and the format specifiers must match `args`.
pub unsafe fn error(
    ctx: &ExecContext,
    flocp: *const Floc,
    _len: size_t,
    fmt: *const ::core::ffi::c_char,
    args: &[FmtArg],
) {
    let makelevel = ctx.makelevel();
    let mut out: Vec<u8> = Vec::new();
    push_error_prefix(ctx, &mut out, flocp, makelevel, false);
    vformat_into(&mut out, fmt, args);
    out.push(b'\n');
    out.push(0);
    outputs(ctx, 1, out.as_ptr() as *const ::core::ffi::c_char);
}

/// Like [`error`] but adds the `*** ` marker and `.  Stop.` suffix, then
/// dies with `MAKE_FAILURE`.
///
/// # Safety
/// Same contract as [`error`].
pub unsafe fn fatal(
    ctx: &ExecContext,
    flocp: *const Floc,
    len: size_t,
    fmt: *const ::core::ffi::c_char,
    args: &[FmtArg],
) -> ! {
    // Share the single formatting+cleanup path with `fatal_err` so the two
    // entry points can never drift (#432 Phase B): format, write, and run
    // `die_cleanup` exactly as before, then exit on the returned status.
    let e = fatal_err(ctx, flocp, len, fmt, args);
    std::process::exit(e.exit_code());
}

/// Non-diverging counterpart to [`fatal`]: does byte-identical work through
/// message formatting/writing, runs the same `die_cleanup` side effects at
/// the same logical point `fatal` runs them today, then *returns*
/// [`crate::build_result::BuildError::Failure`] instead of exiting the
/// process. This lets a leaf function propagate the fatal condition as a
/// `Result` to a caller that has already been migrated off `process::exit`,
/// while `fatal` itself becomes a thin wrapper so both entry points share one
/// formatting+cleanup path and can never drift (#432 Phase B).
///
/// # Safety
/// Same contract as [`error`]/[`fatal`].
pub unsafe fn fatal_err(
    ctx: &ExecContext,
    flocp: *const Floc,
    _len: size_t,
    fmt: *const ::core::ffi::c_char,
    args: &[FmtArg],
) -> crate::build_result::BuildError {
    let makelevel = ctx.makelevel();
    let mut out: Vec<u8> = Vec::new();
    push_error_prefix(ctx, &mut out, flocp, makelevel, true);
    vformat_into(&mut out, fmt, args);
    out.extend_from_slice(b".  Stop.\n");
    out.push(0);
    outputs(ctx, 1, out.as_ptr() as *const ::core::ffi::c_char);
    crate::entry::die_cleanup(ctx, MAKE_FAILURE);
    crate::build_result::BuildError::Failure
}

/// Bridge for call sites not yet converted to return `Result` that call an
/// already-converted leaf: `foo(...).unwrap_or_else(exit_on_err)` reproduces
/// today's exact exit behavior, since [`fatal_err`] already ran
/// `die_cleanup` before handing back the error (`ctx.dying`'s swap-once guard
/// makes a stray double `die_cleanup` call harmless, but every fatal path
/// only ever calls it once, inside `fatal_err`).
pub fn exit_on_err(e: crate::build_result::BuildError) -> ! {
    std::process::exit(e.exit_code())
}

/// Report `str``name`: strerror(errno) via [`error`].
///
/// # Safety
/// `str` and `name` must be valid NUL-terminated strings.
pub unsafe fn perror_with_name(
    ctx: &ExecContext,
    str: *const ::core::ffi::c_char,
    name: *const ::core::ffi::c_char,
) {
    let err: *const ::core::ffi::c_char = strerror(*__errno_location());
    error(
        ctx,
        null::<Floc>(),
        0,
        c"%s%s: %s".as_ptr(),
        &[FmtArg::Str(str), FmtArg::Str(name), FmtArg::Str(err)],
    );
}
/// Non-diverging counterpart to [`pfatal_with_name`]: same message, but
/// returns the fatal condition as a [`crate::build_result::BuildError`]
/// (via [`fatal_err`]) instead of exiting, for callers already migrated off
/// `process::exit` (#432 Phase B).
///
/// # Safety
/// `name` must be a valid NUL-terminated string.
pub unsafe fn pfatal_with_name_err(
    ctx: &ExecContext,
    name: *const ::core::ffi::c_char,
) -> crate::build_result::BuildError {
    let err: *const ::core::ffi::c_char = strerror(*__errno_location());
    fatal_err(
        ctx,
        null::<Floc>(),
        0,
        c"%s: %s".as_ptr(),
        &[FmtArg::Str(name), FmtArg::Str(err)],
    )
}
/// Report `name`: strerror(errno) via [`fatal`] and die.
///
/// # Safety
/// `name` must be a valid NUL-terminated string.
pub unsafe fn pfatal_with_name(ctx: &ExecContext, name: *const ::core::ffi::c_char) -> ! {
    let e = pfatal_with_name_err(ctx, name);
    std::process::exit(e.exit_code());
}
/// Print the out-of-memory message without allocating and exit with
/// `MAKE_FAILURE`.
pub fn out_of_memory() -> ! {
    // Allocation failure carries no `&ExecContext`, so reach the live one
    // through the borrow channel (same one `trace_out` falls back through)
    // and write through its stdout sink; this can fire before startup
    // installs a context, in which case fall back to the plain program name
    // and the real process stdout — matching `trace_out`'s own fallback.
    crate::entry::try_with_exec_context(write_oom_message_from_ctx)
        .unwrap_or_else(|| write_oom_message(&mut std::io::stdout().lock(), "make"));
    std::process::exit(MAKE_FAILURE)
}
fn write_oom_message_from_ctx(ctx: &crate::execctx::ExecContext) {
    let mut out = ctx.stdout.borrow_mut();
    write_oom_message(&mut *out, &msg::program_name(ctx));
}
fn write_oom_message(out: &mut impl std::io::Write, prog: &str) {
    #[allow(clippy::write_with_newline)]
    let _ = write!(out, "{prog}: *** virtual memory exhausted\n");
    let _ = out.flush();
}

/// Native-Rust counterparts to the variadic C-ABI `message`/`error`/`fatal`
/// in this module. Callers build their formatted message with `format!`
/// (or any `Display` source) and hand a `&str` here; the prefix and suffix
/// are added in idiomatic Rust.
///
/// Compatibility note: the variadic extern "C" versions still live above
/// for legacy call sites; both produce identical output formats.
pub mod msg {
    use {
        super::{outputs, MAKE_FAILURE},
        crate::{execctx::ExecContext, floc::Floc},
    };

    pub(crate) fn program_name(ctx: &ExecContext) -> String {
        let p = ctx.program.0.get();
        if p.is_null() {
            // Startup derives the name from argv[0] before anything can
            // print; a null only occurs pre-startup or on a bare test
            // context, where the plain name is the right prefix.
            "make".to_string()
        } else {
            // SAFETY: a non-null `program` is a NUL-terminated C string that
            // outlives the run (argv or 'static storage backs it).
            unsafe { ::core::ffi::CStr::from_ptr(p) }
                .to_string_lossy()
                .into_owned()
        }
    }

    fn build_prefix(ctx: &ExecContext, loc: Option<&Floc>, fatal_marker: bool) -> String {
        let marker = if fatal_marker { "*** " } else { "" };
        // SAFETY: `(*flocp).filenm` is a NUL-terminated C string when non-null.
        unsafe {
            match loc {
                Some(f) if !f.filenm.is_null() => {
                    let fnm = ::core::ffi::CStr::from_ptr(f.filenm).to_string_lossy();
                    format!("{}:{}: {}", fnm, f.lineno.wrapping_add(f.offset), marker)
                }
                _ => {
                    let lvl = ctx.makelevel();
                    let prog = program_name(ctx);
                    if lvl == 0 {
                        format!("{prog}: {marker}")
                    } else {
                        format!("{prog}[{lvl}]: {marker}")
                    }
                }
            }
        }
    }

    fn write_line(ctx: &ExecContext, line: String, is_err: bool) {
        let mut bytes = line.into_bytes();
        bytes.push(0);
        // SAFETY: `outputs` reads up to the trailing NUL we just appended.
        unsafe {
            outputs(
                ctx,
                if is_err { 1 } else { 0 },
                bytes.as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }

    /// Print `msg` to stdout with a trailing newline. If `with_prefix`,
    /// prepend the make program name (and `[LEVEL]` when nested).
    pub fn message(ctx: &ExecContext, with_prefix: bool, msg: &str) {
        let line = if with_prefix {
            format!("{}{msg}\n", build_prefix(ctx, None, false))
        } else {
            format!("{msg}\n")
        };
        write_line(ctx, line, false);
    }

    /// Print `msg` to stderr with the make program/file:line prefix and a
    /// trailing newline.
    pub fn error(ctx: &ExecContext, loc: Option<&Floc>, msg: &str) {
        let line = format!("{}{msg}\n", build_prefix(ctx, loc, false));
        write_line(ctx, line, true);
    }

    /// Non-diverging counterpart to [`fatal`]: does byte-identical
    /// formatting/writing and runs the same `die_cleanup` side effects, then
    /// *returns* [`crate::build_result::BuildError::Failure`] instead of
    /// exiting, for callers already migrated off `process::exit` (#432
    /// Phase B). `fatal` becomes a thin wrapper so both share one path.
    pub fn fatal_err(
        ctx: &ExecContext,
        loc: Option<&Floc>,
        msg: &str,
    ) -> crate::build_result::BuildError {
        let line = format!("{}{msg}.  Stop.\n", build_prefix(ctx, loc, true));
        write_line(ctx, line, true);
        crate::entry::die_cleanup(ctx, MAKE_FAILURE);
        crate::build_result::BuildError::Failure
    }

    /// Print `msg` to stderr with the make program/file:line prefix plus
    /// the `*** ` fatal marker, append `.  Stop.\n`, and exit with
    /// `MAKE_FAILURE`.
    pub fn fatal(ctx: &ExecContext, loc: Option<&Floc>, msg: &str) -> ! {
        let e = fatal_err(ctx, loc, msg);
        ::std::process::exit(e.exit_code())
    }
}

/// Print a `<prefix><msg>` line to stderr, formatting `msg` with `format!`
/// syntax. Safe wrapper over [`msg::error`] — `$loc` is an `Option<&Floc>`.
#[macro_export]
macro_rules! error {
    ($ctx:expr, $loc:expr, $($arg:tt)*) => {
        $crate::output::msg::error($ctx, $loc, &::std::format!($($arg)*))
    };
}

/// Print a fatal `<prefix>*** <msg>.  Stop.` line to stderr and exit, formatting
/// `msg` with `format!` syntax. Safe wrapper over [`msg::fatal`]; never returns.
/// `$loc` is an `Option<&Floc>`.
#[macro_export]
macro_rules! fatal {
    ($ctx:expr, $loc:expr, $($arg:tt)*) => {
        $crate::output::msg::fatal($ctx, $loc, &::std::format!($($arg)*))
    };
}

#[cfg(test)]
mod fatal_err_tests {
    use super::*;

    /// [`pfatal_with_name_err`] runs the same `die_cleanup` side effects as
    /// diverging [`pfatal_with_name`] but returns
    /// `BuildError::Failure` instead of exiting, and marks the context
    /// dying (the swap-once guard `die_cleanup` uses to run cleanup exactly
    /// once) — the #432 Phase B non-diverging leaf this slice adds.
    #[test]
    fn pfatal_with_name_err_returns_failure_and_marks_dying() {
        let ctx = ExecContext::default();
        assert!(!ctx.dying.0.load(::std::sync::atomic::Ordering::Relaxed));

        let e = unsafe { pfatal_with_name_err(&ctx, c"probe".as_ptr()) };

        assert_eq!(e, crate::build_result::BuildError::Failure);
        assert!(ctx.dying.0.load(::std::sync::atomic::Ordering::Relaxed));
    }

    /// [`msg::fatal_err`] (the safe-API counterpart) likewise returns
    /// `BuildError::Failure` and runs cleanup exactly once, without
    /// exiting the test process the way diverging [`msg::fatal`] would.
    #[test]
    fn msg_fatal_err_returns_failure_and_marks_dying() {
        let ctx = ExecContext::default();

        let e = msg::fatal_err(&ctx, None, "probe");

        assert_eq!(e, crate::build_result::BuildError::Failure);
        assert!(ctx.dying.0.load(::std::sync::atomic::Ordering::Relaxed));
    }
}

#[cfg(test)]
mod log_working_directory_tests {
    //! #461 review: the plugin ABI can reach `log_working_directory` with a
    //! throwaway `ExecContext` whose `program` is null; the trace must fall
    //! back to the plain "make" name instead of handing null to `strlen`.

    #[test]
    fn tolerates_null_program_context() {
        let ctx = crate::execctx::ExecContext::default();
        assert!(ctx.program.0.get().is_null());
        // SAFETY: single-threaded test; the options channel is installed above.
        let traced = unsafe { super::log_working_directory(&ctx, 1) };
        assert_eq!(traced, 1);
    }

    /// The other three `makelevel`/`starting_directory` format-string
    /// combinations `tolerates_null_program_context` doesn't reach: a
    /// non-null `starting_directory` (the `'%s'`-quoted forms) at both
    /// `makelevel == 0` and `makelevel != 0`, on both `entering` values.
    #[test]
    fn covers_every_makelevel_and_directory_combination() {
        let dir = ::std::ffi::CString::new("/tmp/build").unwrap();
        for makelevel in [0, 2] {
            let ctx = crate::execctx::ExecContext::new(crate::execctx::Config {
                makelevel,
                ..Default::default()
            });
            ctx.starting_directory
                .0
                .set(dir.as_ptr() as *mut ::core::ffi::c_char);
            for entering in [1, 0] {
                // SAFETY: single-threaded test; a valid NUL-terminated
                // `starting_directory` is installed above.
                let traced = unsafe { super::log_working_directory(&ctx, entering) };
                assert_eq!(traced, 1);
            }
        }
    }
}

#[cfg(test)]
mod output_context_tests {
    //! The former `static mut output_context` is per-run state reached over
    //! the `CTX_PTR` borrow channel; the accessors must round-trip through
    //! the installed context so every printer — including ones handed a
    //! throwaway `ExecContext` — sees the same sync target.

    #[test]
    fn accessors_round_trip_through_the_live_context() {
        let _ctx = crate::entry::install_default_exec_context_for_test();
        let mut record = super::output {
            out: 7,
            err: 8,
            syncout: 1,
        };
        super::set_output_context(&raw mut record);
        assert_eq!(super::output_context(), &raw mut record);
        // The throwaway-context printers read the same live value: the getter
        // takes no `&ExecContext` at all, so there is nothing else it could
        // consult.
        super::set_output_context(::core::ptr::null_mut());
        assert!(super::output_context().is_null());
    }
}

#[cfg(test)]
mod outputs_tests {
    use {
        super::{
            _outputs,
            output,
            pump_copy,
            pump_perror,
            write_oom_message,
            ExecContext,
            OUTPUT_NONE,
        },
        std::ffi::CString,
    };

    /// `write_oom_message` formats exactly `<prog>: *** virtual memory
    /// exhausted\n` and flushes — the no-alloc body `out_of_memory` uses on
    /// both its ctx-found and ctx-less fallback paths.
    #[test]
    fn write_oom_message_formats_prog_and_flushes() {
        let mut out: Vec<u8> = Vec::new();
        write_oom_message(&mut out, "make");
        assert_eq!(out, b"make: *** virtual memory exhausted\n");
    }

    /// `outputs` is a no-op on a null or empty message — the early return
    /// before it ever reaches `_outputs`.
    #[test]
    fn outputs_is_a_noop_on_null_or_empty_message() {
        let ctx = ExecContext::default();
        unsafe {
            super::outputs(&ctx, 0, ::core::ptr::null());
            super::outputs(&ctx, 0, c"".as_ptr());
        }
    }

    /// The sticky stdout errno keeps the first recorded error, like libc's
    /// `ferror` flag: a later failure must not overwrite it. One test owns
    /// the process-global so orderings can't race.
    #[test]
    fn record_stdout_error_is_sticky_and_keeps_first_errno() {
        assert_eq!(super::stdout_error(), 0);
        super::record_stdout_error(&std::io::Error::from_raw_os_error(libc::ENOSPC));
        assert_eq!(super::stdout_error(), libc::ENOSPC);
        super::record_stdout_error(&std::io::Error::from_raw_os_error(libc::EPIPE));
        assert_eq!(super::stdout_error(), libc::ENOSPC);
    }

    /// `pump_copy` rewinds first and copies everything: a cursor left
    /// mid-stream still yields the full content, matching the C pump's
    /// `lseek(0)`-then-read loop.
    #[test]
    fn pump_copy_rewinds_and_copies_everything() {
        let mut src = std::io::Cursor::new(b"synced child output\n".to_vec());
        src.set_position(7);
        let mut dst: Vec<u8> = Vec::new();
        pump_copy(&mut src, &mut dst, false, |_, _, _| {});
        assert_eq!(dst, b"synced child output\n");
    }

    /// A reader that yields Interrupted once, then data, then EOF: the
    /// EINTR retry keeps pumping (the C loop's `errno == EINTR` retry).
    #[test]
    fn pump_copy_retries_interrupted_reads() {
        struct EintrOnce {
            hits: u32,
            data: std::io::Cursor<Vec<u8>>,
        }
        impl std::io::Read for EintrOnce {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                if self.hits == 0 {
                    self.hits = 1;
                    return Err(std::io::Error::from(std::io::ErrorKind::Interrupted));
                }
                self.data.read(buf)
            }
        }
        impl std::io::Seek for EintrOnce {
            fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
                self.data.seek(pos)
            }
        }
        let mut src = EintrOnce {
            hits: 0,
            data: std::io::Cursor::new(b"after-eintr".to_vec()),
        };
        let mut dst: Vec<u8> = Vec::new();
        pump_copy(&mut src, &mut dst, false, |_, _, _| {});
        assert_eq!(dst, b"after-eintr");
    }

    /// A failing writer stops the pump after the perror line, like the C
    /// loop's `fwrite() < 1` break; partial output stays written.
    #[test]
    fn pump_copy_stops_on_write_error() {
        struct FailAfterFirst {
            wrote: bool,
            sink: Vec<u8>,
        }
        impl std::io::Write for FailAfterFirst {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                if self.wrote {
                    return Err(std::io::Error::from_raw_os_error(libc::ENOSPC));
                }
                self.wrote = true;
                self.sink.extend_from_slice(buf);
                Ok(buf.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        // Two >8KiB-boundary chunks force two writes; the second fails.
        let mut src = std::io::Cursor::new(vec![b'x'; 8192 + 16]);
        let mut dst = FailAfterFirst {
            wrote: false,
            sink: Vec::new(),
        };
        let mut errs: Vec<u8> = Vec::new();
        pump_copy(&mut src, &mut dst, false, |_, what, e| {
            pump_perror(&mut errs, what, e)
        });
        assert_eq!(dst.sink.len(), 8192, "first chunk written, then stopped");
        assert!(
            String::from_utf8_lossy(&errs).starts_with("fwrite(): "),
            "report_err ran for the write failure: {:?}",
            String::from_utf8_lossy(&errs)
        );
    }

    /// A read error (not EINTR) ends the pump after the perror line, like
    /// the C loop's `len < 0` break.
    #[test]
    fn pump_copy_stops_on_read_error() {
        struct BadRead;
        impl std::io::Read for BadRead {
            fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::from_raw_os_error(libc::EIO))
            }
        }
        impl std::io::Seek for BadRead {
            fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
                Err(std::io::Error::from_raw_os_error(libc::ESPIPE))
            }
        }
        let mut dst: Vec<u8> = Vec::new();
        let mut errs: Vec<u8> = Vec::new();
        pump_copy(&mut BadRead, &mut dst, false, |_, what, e| {
            pump_perror(&mut errs, what, e)
        });
        assert!(dst.is_empty(), "nothing pumped from a failing reader");
        // `BadRead` fails both `seek` and `read`, so `report_err` runs for
        // both the doomed rewind and the read itself.
        assert!(
            String::from_utf8_lossy(&errs).contains("read(): "),
            "report_err ran for the read failure: {:?}",
            String::from_utf8_lossy(&errs)
        );
    }

    /// `pump_perror` formats exactly `<what>: <strerror(errno)>\n`.
    #[test]
    fn pump_perror_formats_what_and_strerror() {
        let mut out: Vec<u8> = Vec::new();
        pump_perror(
            &mut out,
            "lseek()",
            &std::io::Error::from_raw_os_error(libc::ENOENT),
        );
        assert_eq!(out, b"lseek(): No such file or directory\n");
    }

    /// The non-synced fallback writes through `ctx.stderr` when `is_err` is
    /// set. `_outputs` is pinned to the default sink type (real stdout/
    /// stderr) — genericizing it would cascade into `outputs`/`error`/
    /// `fatal`/`message` and everything *they* call, well beyond this
    /// slice's scope — so an empty message keeps the test's real stderr
    /// clean while still covering the branch.
    #[test]
    fn outputs_fallback_writes_stderr_branch() {
        let ctx = ExecContext::default();
        _outputs(&ctx, None, true, c"");
    }

    /// A unique temp path; the file is created by `open_temp_fd`.
    fn temp_path(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("outputs-{tag}-{nanos}-{}", std::process::id()))
    }

    /// Open `path` for read/write, creating/truncating it, returning a raw fd.
    unsafe fn open_temp_fd(path: &std::path::Path) -> i32 {
        let c = CString::new(path.to_str().unwrap()).unwrap();
        let fd = libc::open(
            c.as_ptr(),
            libc::O_RDWR | libc::O_CREAT | libc::O_TRUNC,
            0o600,
        );
        assert!(fd >= 0, "open temp file");
        fd
    }

    /// Read the whole file at `path` to a string.
    fn read_all(path: &std::path::Path) -> String {
        std::fs::read_to_string(path).expect("read temp file")
    }

    /// Build a zeroed `output` with `syncout` enabled and its stdout/stderr
    /// descriptors pointed at two raw fds.
    unsafe fn sync_output(out_fd: i32, err_fd: i32) -> output {
        let mut o: output = ::core::mem::zeroed();
        o.out = out_fd;
        o.err = err_fd;
        o.set_syncout(1);
        o
    }

    /// With output-sync active and a valid stdout descriptor, `_outputs`
    /// appends the message to that descriptor (the sync/`writebuf` path). The
    /// stderr selection routes to the `err` fd instead. Uses real temp files so
    /// the bytes can be read back; no dependence on the global stdio.
    #[test]
    fn writes_to_sync_descriptor_per_stream() {
        let out_path = temp_path("out");
        let err_path = temp_path("err");
        let ctx = ExecContext::default();
        let mut o = unsafe {
            let out_fd = open_temp_fd(&out_path);
            let err_fd = open_temp_fd(&err_path);
            sync_output(out_fd, err_fd)
        };
        _outputs(&ctx, Some(&mut o), false, c"to-stdout\n");
        _outputs(&ctx, Some(&mut o), true, c"to-stderr\n");
        unsafe {
            libc::close(o.out);
            libc::close(o.err);
        }
        assert_eq!(
            read_all(&out_path),
            "to-stdout\n",
            "is_err==0 writes to the out descriptor"
        );
        assert_eq!(
            read_all(&err_path),
            "to-stderr\n",
            "is_err!=0 writes to the err descriptor"
        );
        let _ = std::fs::remove_file(&out_path);
        let _ = std::fs::remove_file(&err_path);
    }

    /// When the selected descriptor is `OUTPUT_NONE`, the sync fast-path is
    /// skipped and the call falls through to the stdio writer. Driving it
    /// with an empty message keeps the test's real stdout clean while still
    /// exercising the `fd == OUTPUT_NONE` branch and the write/flush tail.
    #[test]
    fn falls_through_when_descriptor_is_none() {
        let ctx = ExecContext::default();
        let mut o = unsafe { sync_output(OUTPUT_NONE, OUTPUT_NONE) };
        _outputs(&ctx, Some(&mut o), false, c"");
    }

    /// A null `output` (no sync context) goes straight to the ctx sink.
    #[test]
    fn null_output_uses_stdio() {
        let ctx = ExecContext::default();
        _outputs(&ctx, None, false, c"");
    }
}

#[cfg(test)]
mod push_cstr_unsafe_oracle {
    //! `push_cstr` now takes `Option<&CStr>` — the NULL-pointer sentinel became
    //! `None`. This keeps the verbatim c2rust pointer-based implementation as a
    //! differential oracle and asserts both append identical bytes onto a
    //! seeded, non-empty buffer (AGENTS rule 3).

    /// Original c2rust pointer-based implementation, preserved verbatim.
    unsafe fn push_cstr(out: &mut Vec<u8>, s: *const ::core::ffi::c_char) {
        if !s.is_null() {
            out.extend_from_slice(::core::ffi::CStr::from_ptr(s).to_bytes());
        }
    }

    #[test]
    fn matches_oracle() {
        let cases: &[Option<&::core::ffi::CStr>] = &[
            None,
            Some(c""),
            Some(c"make"),
            Some(c"foo.mk:12: "),
            Some(c"x"),
        ];
        for &s in cases {
            // Seed both buffers so a replace-instead-of-append mutant is caught.
            let mut safe = vec![b'<'];
            super::push_cstr(&mut safe, s);

            let mut oracle = vec![b'<'];
            let p = s.map_or(::core::ptr::null(), |c| c.as_ptr());
            // SAFETY: `p` is null or a valid NUL-terminated C string.
            unsafe { push_cstr(&mut oracle, p) };

            assert_eq!(safe, oracle, "mismatch for input {s:?}");
        }
    }
}
