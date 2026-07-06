//! Output handling: the `message`/`error`/`fatal` printers and the
//! output-sync machinery that captures recipe output into temp files and
//! dumps it atomically.
//!
//! Port of `output.c`. The printers keep a printf-style calling convention
//! (a C format string plus a `FmtArg` slice, not real C ABI/varargs) because
//! they are called that way from all over the crate; the [`msg`] submodule
//! provides native-Rust counterparts.
//!
//! Every public function here takes safe types (`&CStr`, `Option<&Floc>`,
//! `Option<&mut output>`) rather than raw pointers. The current output-sync
//! target is a process-wide, pointer-based handle (`output_context`) owned
//! by whichever file holds the live `ExecContext`; resolving that raw
//! pointer to a reference is therefore each *caller's* job (`unsafe {
//! output_context().as_mut() }`), not this file's — the one exception is
//! `fatal`/`pfatal_with_name`/`msg::fatal`'s call into `die`, a pre-existing,
//! enormous shutdown subsystem (main.rs) out of scope for this file.

use ::core::ffi::{c_uint, CStr};
use std::io::{Read, Seek, Write};
use std::sync::atomic::Ordering;

use crate::execctx::ExecContext;
use crate::ffi_types::{size_t, uintmax_t};
use crate::floc::Floc;
use crate::make_main::die;
use crate::misc::open_anon_tmpfd;
use crate::posixos::{
    check_io_state, fd_noinherit, fd_reset_append, fd_set_append, osync_acquire, osync_clear,
    osync_release,
};

/// Per-target output state: temp-file descriptors for stdout/stderr while
/// output sync is active.
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct output {
    pub out: i32,
    pub err: i32,
    #[bitfield(name = "syncout", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub syncout: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
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
    crate::make_main::try_with_exec_context(|c| c.output_context.0.get())
        .unwrap_or(::core::ptr::null_mut())
}

/// Set the active output-sync target on the live run (see
/// [`output_context`]). A no-op when no context is installed, mirroring the
/// null-global steady state outside `main_0`'s extent.
pub fn set_output_context(value: *mut output) {
    let _ = crate::make_main::try_with_exec_context(|c| c.output_context.0.set(value));
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
pub fn stdio_traced() -> bool {
    crate::make_main::with_options(|o| o.stdio_traced.get())
}

/// Record whether the working-directory enter trace has been emitted.
pub fn set_stdio_traced(value: bool) {
    crate::make_main::with_options(|o| o.stdio_traced.set(value));
}
pub const OUTPUT_NONE: i32 = -1;

/// Run `f` on a `File` view of the raw fd `fd` without taking ownership of
/// it: `f`'s `File` is released back to a raw fd afterward instead of being
/// closed on drop (unlike a plain `File::from_raw_fd(fd)` whose drop would
/// close a descriptor this function doesn't own).
///
/// # Safety
/// `fd` must be a valid, open file descriptor.
unsafe fn with_borrowed_fd<T>(fd: i32, f: impl FnOnce(&mut std::fs::File) -> T) -> T {
    use std::os::unix::io::{FromRawFd, IntoRawFd};
    let mut file = std::fs::File::from_raw_fd(fd);
    let result = f(&mut file);
    let _ = file.into_raw_fd();
    result
}

/// Write `msg` to `out`'s sync descriptor if one is active, else to real
/// stdout/stderr.
fn _outputs(out: Option<&mut output>, is_err: i32, msg: &CStr) {
    let bytes = msg.to_bytes();
    if let Some(o) = out {
        if o.syncout() as i32 != 0 {
            let fd: i32 = if is_err != 0 { o.err } else { o.out };
            if fd != OUTPUT_NONE {
                // SAFETY: `fd` is either `out.out`/`out.err`, an fd this
                // module opened itself via `output_tmpfd`.
                unsafe {
                    with_borrowed_fd(fd, |file| {
                        let _ = file.seek(std::io::SeekFrom::End(0));
                        let _ = file.write_all(bytes);
                    });
                }
                return;
            }
        }
    }
    if is_err != 0 {
        let mut w = std::io::stderr().lock();
        let _ = w.write_all(bytes);
        let _ = w.flush();
    } else {
        let mut w = std::io::stdout().lock();
        let _ = w.write_all(bytes);
        let _ = w.flush();
    }
}
/// Print an entering/leaving-directory line (returns 1).
pub fn log_working_directory(ctx: &ExecContext, entering: i32) -> i32 {
    let makelevel = ctx.makelevel();
    // `program` is null only on context-less paths handed a throwaway
    // `ExecContext` (plugin ABI); the C original could not get here with a
    // null `program` global. Fall back to the plain name like
    // `msg::program_name` rather than passing null to the formatter.
    let program: &CStr = ctx.program.as_cstr().unwrap_or(c"make");
    let has_dir = ctx.starting_directory.as_cstr().is_some();
    // Harmless placeholder when there's no starting directory: the formats
    // below have no `%s` slot for it in that case, so it's never consumed.
    let starting_directory: &CStr = ctx.starting_directory.as_cstr().unwrap_or(c"");
    let fmt: &CStr;
    if makelevel == 0 {
        if !has_dir {
            fmt = if entering != 0 {
                c"%s: Entering an unknown directory\n"
            } else {
                c"%s: Leaving an unknown directory\n"
            };
        } else if entering != 0 {
            fmt = c"%s: Entering directory '%s'\n";
        } else {
            fmt = c"%s: Leaving directory '%s'\n";
        }
    } else if !has_dir {
        fmt = if entering != 0 {
            c"%s[%u]: Entering an unknown directory\n"
        } else {
            c"%s[%u]: Leaving an unknown directory\n"
        };
    } else if entering != 0 {
        fmt = c"%s[%u]: Entering directory '%s'\n";
    } else {
        fmt = c"%s[%u]: Leaving directory '%s'\n";
    }
    // The line is built in an owned buffer per call (the former grow-only
    // `static mut buf`/`len` pair); `_outputs` copies the bytes before
    // returning, so the local's lifetime is enough.
    let mut line: Vec<u8> = Vec::new();
    if crate::make_main::opt_print_data_base() {
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
    let msg = CStr::from_bytes_with_nul(&line).unwrap();
    _outputs(None, 0, msg);
    1
}
/// Copy everything from fd `from` to stdout (`is_err == false`) or stderr
/// (`is_err == true`), from the beginning.
pub fn pump_from_tmp(from: i32, is_err: bool) {
    // SAFETY: `from` is an open, seekable fd, per this function's contract.
    unsafe {
        with_borrowed_fd(from, |file| {
            if let Err(e) = file.seek(std::io::SeekFrom::Start(0)) {
                eprintln!("lseek(): {e}");
            }
            let mut buffer = [0u8; 8192];
            loop {
                let len = match file.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("read(): {e}");
                        break;
                    }
                };
                let chunk = &buffer[..len];
                let result = if is_err {
                    let mut w = std::io::stderr().lock();
                    w.write_all(chunk).and_then(|_| w.flush())
                } else {
                    let mut w = std::io::stdout().lock();
                    w.write_all(chunk).and_then(|_| w.flush())
                };
                if let Err(e) = result {
                    eprintln!("write(): {e}");
                    break;
                }
            }
        });
    }
}
/// Create an anonymous temp fd in append mode for output sync.
pub fn output_tmpfd(ctx: &ExecContext) -> i32 {
    let fd: i32 = open_anon_tmpfd(ctx);
    fd_set_append(fd);
    fd
}
/// Set up the output-sync temp files for `out`, disabling output sync
/// (with a message) on failure.
///
/// # Safety
/// Must run single-threaded.
pub fn setup_tmpfile(ctx: &ExecContext, out: &mut output) {
    // Guards against re-entrant tmpfile setup (the C code's recursion check).
    if ctx.output_in_setup.0.load(Ordering::Relaxed) {
        return;
    }
    ctx.output_in_setup.0.store(true, Ordering::Relaxed);
    let io_state: c_uint = check_io_state(ctx);
    // The block falls through to the error handler below on any failure;
    // reaching its end is the success path (the C code used `goto`).
    'setup: {
        if io_state & (IO_STDOUT_OK | IO_STDERR_OK) == 0 {
            perror_with_name(ctx, Some(&mut *out), c"output-sync suppressed: ", c"stderr");
            break 'setup;
        }
        if io_state & IO_STDOUT_OK != 0 {
            let fd: i32 = output_tmpfd(ctx);
            if fd < 0 {
                break 'setup;
            }
            fd_noinherit(fd);
            out.out = fd;
        }
        if io_state & IO_STDERR_OK != 0 {
            if out.out != OUTPUT_NONE && io_state & IO_COMBINED_OUTERR != 0 {
                out.err = out.out;
            } else {
                let fd_0: i32 = output_tmpfd(ctx);
                if fd_0 < 0 {
                    break 'setup;
                }
                fd_noinherit(fd_0);
                out.err = fd_0;
            }
        }
        ctx.output_in_setup.0.store(false, Ordering::Relaxed);
        return;
    }
    error(
        ctx,
        Some(&mut *out),
        None,
        0,
        c"cannot open output-sync lock file: suppressing output-sync",
        &[],
    );
    output_close(ctx, Some(&mut *out));
    crate::make_main::with_options(|o| o.output_sync.set(OUTPUT_SYNC_NONE));
    osync_clear();
    ctx.output_in_setup.0.store(false, Ordering::Relaxed);
}
/// Dump any captured output under the output-sync lock and truncate the
/// temp files for reuse.
///
/// # Safety
/// Must run single-threaded.
pub fn output_dump(ctx: &ExecContext, out: &mut output) {
    // Current size of the fd's underlying file, via `SeekFrom::End(0)` (the
    // former `lseek(fd, 0, SEEK_END)`).
    let fd_size = |fd: i32| -> i64 {
        // SAFETY: `fd` is `out.out`/`out.err`, an fd this module opened
        // itself.
        unsafe { with_borrowed_fd(fd, |file| file.seek(std::io::SeekFrom::End(0)).unwrap_or(0) as i64) }
    };
    let outfd_not_empty: i32 = (out.out != OUTPUT_NONE && fd_size(out.out) > 0) as i32;
    let errfd_not_empty: i32 = (out.err != OUTPUT_NONE && fd_size(out.err) > 0) as i32;
    if outfd_not_empty != 0 || errfd_not_empty != 0 {
        let mut traced: i32 = 0;
        if osync_acquire(ctx) == 0 {
            error(
                ctx,
                Some(&mut *out),
                None,
                0,
                c"warning: cannot acquire output lock: disabling output sync",
                &[],
            );
            osync_clear();
        }
        if crate::make_main::opt_output_sync() != OUTPUT_SYNC_RECURSE
            && crate::make_main::should_print_dir_mirror(ctx) != 0
        {
            traced = log_working_directory(ctx, 1);
        }
        if outfd_not_empty != 0 {
            pump_from_tmp(out.out, false);
        }
        if errfd_not_empty != 0 && out.err != out.out {
            pump_from_tmp(out.err, true);
        }
        if traced != 0 {
            log_working_directory(ctx, 0);
        }
        osync_release(ctx);
        let truncate = |fd: i32| {
            // SAFETY: `fd` is `out.out`/`out.err`, an fd this module opened
            // itself.
            unsafe {
                with_borrowed_fd(fd, |file| {
                    let _ = file.seek(std::io::SeekFrom::Start(0));
                    let _ = file.set_len(0);
                });
            }
        };
        if out.out != OUTPUT_NONE {
            truncate(out.out);
        }
        if out.err != OUTPUT_NONE && out.err != out.out {
            truncate(out.err);
        }
    }
}
/// Initialize `out` (or, when `None`, switch stdout/stderr to append mode).
///
/// # Safety
/// Must run single-threaded.
pub fn output_init(ctx: &ExecContext, out: Option<&mut output>) {
    if let Some(out) = out {
        out.err = OUTPUT_NONE;
        out.out = out.err;
        out.set_syncout((crate::make_main::opt_output_sync() != 0) as i32 as c_uint);
        return;
    }
    use std::os::unix::io::AsRawFd;
    ctx.stdout_flags.0.store(
        fd_set_append(std::io::stdout().as_raw_fd()),
        Ordering::Relaxed,
    );
    ctx.stderr_flags.0.store(
        fd_set_append(std::io::stderr().as_raw_fd()),
        Ordering::Relaxed,
    );
}
/// Dump and close `out`'s temp files (or, when `None`, restore
/// stdout/stderr).
///
/// # Safety
/// Must run single-threaded.
pub fn output_close(ctx: &ExecContext, out: Option<&mut output>) {
    let Some(out) = out else {
        if stdio_traced() {
            log_working_directory(ctx, 0);
        }
        use std::os::unix::io::AsRawFd;
        fd_reset_append(
            std::io::stdout().as_raw_fd(),
            ctx.stdout_flags.0.load(Ordering::Relaxed),
        );
        fd_reset_append(
            std::io::stderr().as_raw_fd(),
            ctx.stderr_flags.0.load(Ordering::Relaxed),
        );
        return;
    };
    output_dump(ctx, out);
    // SAFETY: `out.out`/`out.err` are either `OUTPUT_NONE` (checked below)
    // or fds this module opened itself via `output_tmpfd`.
    unsafe {
        use std::os::unix::io::FromRawFd;
        if out.out >= 0 {
            drop(std::fs::File::from_raw_fd(out.out));
        }
        if out.err >= 0 && out.err != out.out {
            drop(std::fs::File::from_raw_fd(out.err));
        }
    }
    output_init(ctx, Some(out));
}
/// Lazily set up output sync and the enter-directory trace before the
/// first real output.
///
/// # Safety
/// Must run single-threaded: touches output and trace globals.
pub fn output_start(ctx: &ExecContext, mut osync: Option<&mut output>) {
    if let Some(o) = osync.as_deref_mut() {
        if o.syncout() as i32 != 0 && !(o.out >= 0 || o.err >= 0) {
            setup_tmpfile(ctx, o);
        }
    }
    if (crate::make_main::opt_output_sync() == OUTPUT_SYNC_NONE
        || crate::make_main::opt_output_sync() == OUTPUT_SYNC_RECURSE)
        && !stdio_traced()
        && crate::make_main::should_print_dir_mirror(ctx) != 0
    {
        set_stdio_traced(log_working_directory(ctx, 1) != 0);
    }
}
/// Write `msg` to stdout or stderr (or `osync`'s sync temp file), starting
/// output first. `osync` is the caller's already-resolved view of the
/// current output-sync target (see [`output_context`]).
pub fn outputs(ctx: &ExecContext, mut osync: Option<&mut output>, is_err: i32, msg: &CStr) {
    if msg.to_bytes().is_empty() {
        return;
    }
    output_start(ctx, osync.as_deref_mut());
    _outputs(osync, is_err, msg);
}
// The former shared, growing printer buffer (`static mut fmtbuf` and its
// `get_buffer` accessor) is gone: each printer builds its line in an owned
// `Vec<u8>` and hands `outputs` a pointer into it — `outputs` copies the
// bytes before returning, so no allocation outlives its call.
/// One argument to the printf-subset formatter that replaced C varargs.
#[derive(Copy, Clone)]
pub enum FmtArg<'a> {
    Str(&'a CStr),
    Int(i64),
    Uint(u64),
    Ptr(*const ::core::ffi::c_void),
}

/// Render printf-style `fmt` with `args` into `out`, byte-for-byte like the
/// C printf subset this codebase uses: %s %d %i %u %x %c %p %% with flags,
/// width and precision (including `*`), and h/l/ll/z length modifiers.
pub fn vformat_into(out: &mut Vec<u8>, fmt: &CStr, args: &[FmtArg]) {
    let bytes = fmt.to_bytes();
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
                let mut v = match args.next() {
                    Some(FmtArg::Str(s)) => s.to_bytes().to_vec(),
                    _ => b"(null)".to_vec(),
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
                    Some(FmtArg::Str(s)) => s.as_ptr() as usize,
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

fn push_program_prefix(ctx: &ExecContext, out: &mut Vec<u8>, makelevel: u32, fatal_marker: bool) {
    push_cstr(out, ctx.program.as_cstr());
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

fn push_error_prefix(
    ctx: &ExecContext,
    out: &mut Vec<u8>,
    flocp: Option<&Floc>,
    makelevel: u32,
    fatal_marker: bool,
) {
    match flocp.and_then(|fl| fl.filenm_cstr().map(|fnm| (fnm, fl))) {
        Some((fnm, fl)) => {
            push_cstr(out, Some(fnm));
            out.push(b':');
            out.extend_from_slice(fl.lineno.wrapping_add(fl.offset).to_string().as_bytes());
            out.extend_from_slice(b": ");
            if fatal_marker {
                out.extend_from_slice(b"*** ");
            }
        }
        None => push_program_prefix(ctx, out, makelevel, fatal_marker),
    }
}

/// printf-subset message to stdout, optionally prefixed with the program name.
pub fn message(
    ctx: &ExecContext,
    osync: Option<&mut output>,
    prefix: i32,
    _len: size_t,
    fmt: &CStr,
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
    let msg = CStr::from_bytes_with_nul(&out).unwrap();
    outputs(ctx, osync, 0, msg);
}

/// printf-subset error to stderr with a file:line or program prefix.
pub fn error(
    ctx: &ExecContext,
    osync: Option<&mut output>,
    flocp: Option<&Floc>,
    _len: size_t,
    fmt: &CStr,
    args: &[FmtArg],
) {
    let makelevel = ctx.makelevel();
    let mut out: Vec<u8> = Vec::new();
    push_error_prefix(ctx, &mut out, flocp, makelevel, false);
    vformat_into(&mut out, fmt, args);
    out.push(b'\n');
    out.push(0);
    let msg = CStr::from_bytes_with_nul(&out).unwrap();
    outputs(ctx, osync, 1, msg);
}

/// Like [`error`] but adds the `*** ` marker and `.  Stop.` suffix, then
/// dies with `MAKE_FAILURE`.
pub fn fatal(
    ctx: &ExecContext,
    osync: Option<&mut output>,
    flocp: Option<&Floc>,
    _len: size_t,
    fmt: &CStr,
    args: &[FmtArg],
) -> ! {
    let makelevel = ctx.makelevel();
    let mut out: Vec<u8> = Vec::new();
    push_error_prefix(ctx, &mut out, flocp, makelevel, true);
    vformat_into(&mut out, fmt, args);
    out.extend_from_slice(b".  Stop.\n");
    out.push(0);
    let msg = CStr::from_bytes_with_nul(&out).unwrap();
    outputs(ctx, osync, 1, msg);
    // SAFETY: `die` is the make-process exit point (main.rs's whole
    // shutdown sequence — reap_children, remove_intermediates,
    // print_data_base, clean_jobserver, etc.), a pre-existing subsystem out
    // of scope for this file's unsafe-removal; this is the one deliberate
    // exception.
    unsafe { die(ctx, MAKE_FAILURE) }
}

/// The current errno's message, matching `strerror`'s wording exactly:
/// `std::io::Error`'s `Display` is `strerror`'s text plus a trailing
/// `" (os error N)"` (its `sys::os::error_string` is `strerror_r`-backed on
/// Unix), so strip that suffix back off rather than reaching for `strerror`
/// directly.
fn os_error_message() -> String {
    let full = std::io::Error::last_os_error().to_string();
    match full.find(" (os error ") {
        Some(idx) => full[..idx].to_string(),
        None => full,
    }
}

/// Report `str_``name`: strerror(errno) via [`error`].
pub fn perror_with_name(ctx: &ExecContext, osync: Option<&mut output>, str_: &CStr, name: &CStr) {
    let err = ::std::ffi::CString::new(os_error_message()).unwrap_or_default();
    error(
        ctx,
        osync,
        None,
        0,
        c"%s%s: %s",
        &[
            FmtArg::Str(str_),
            FmtArg::Str(name),
            FmtArg::Str(err.as_c_str()),
        ],
    );
}
/// Report `name`: strerror(errno) via [`fatal`] and die.
pub fn pfatal_with_name(ctx: &ExecContext, osync: Option<&mut output>, name: &CStr) -> ! {
    let err = ::std::ffi::CString::new(os_error_message()).unwrap_or_default();
    fatal(
        ctx,
        osync,
        None,
        0,
        c"%s: %s",
        &[FmtArg::Str(name), FmtArg::Str(err.as_c_str())],
    );
}
/// Print the out-of-memory message without allocating and exit with
/// `MAKE_FAILURE`.
pub fn out_of_memory() -> ! {
    use std::io::Write;
    // Allocation failure carries no `&ExecContext`, so reach the live one
    // through the borrow channel; this can fire before startup installs a
    // context, in which case fall back to the plain program name.
    let prog = crate::make_main::try_with_exec_context(msg::program_name)
        .unwrap_or_else(|| "make".to_string());
    let mut out = std::io::stdout().lock();
    #[allow(clippy::write_with_newline)]
    let _ = write!(out, "{prog}: *** virtual memory exhausted\n");
    let _ = out.flush();
    std::process::exit(MAKE_FAILURE)
}

/// Native-Rust counterparts to the printf-style `message`/`error`/`fatal`
/// in this module. Callers build their formatted message with `format!`
/// (or any `Display` source) and hand a `&str` here; the prefix and suffix
/// are added in idiomatic Rust.
///
/// Compatibility note: the printf-style versions still live above for
/// legacy call sites (called with a C format string and a `FmtArg` slice,
/// not real C varargs); both produce identical output formats.
pub mod msg {
    use super::{die, output, outputs, MAKE_FAILURE};
    use crate::execctx::ExecContext;
    use crate::floc::Floc;

    pub(crate) fn program_name(ctx: &ExecContext) -> String {
        // Startup derives the name from argv[0] before anything can print;
        // a null only occurs pre-startup or on a bare test context, where
        // the plain name is the right prefix.
        match ctx.program.as_cstr() {
            Some(p) => p.to_string_lossy().into_owned(),
            None => "make".to_string(),
        }
    }

    fn build_prefix(ctx: &ExecContext, loc: Option<&Floc>, fatal_marker: bool) -> String {
        let marker = if fatal_marker { "*** " } else { "" };
        match loc.and_then(|f| f.filenm_cstr().map(|fnm| (fnm, f))) {
            Some((fnm, f)) => {
                format!(
                    "{}:{}: {}",
                    fnm.to_string_lossy(),
                    f.lineno.wrapping_add(f.offset),
                    marker
                )
            }
            None => {
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

    fn write_line(ctx: &ExecContext, osync: Option<&mut output>, line: String, is_err: bool) {
        let mut bytes = line.into_bytes();
        bytes.push(0);
        let msg = ::core::ffi::CStr::from_bytes_with_nul(&bytes).unwrap();
        outputs(ctx, osync, if is_err { 1 } else { 0 }, msg);
    }

    /// Print `msg` to stdout with a trailing newline. If `with_prefix`,
    /// prepend the make program name (and `[LEVEL]` when nested).
    pub fn message(ctx: &ExecContext, osync: Option<&mut output>, with_prefix: bool, msg: &str) {
        let line = if with_prefix {
            format!("{}{msg}\n", build_prefix(ctx, None, false))
        } else {
            format!("{msg}\n")
        };
        write_line(ctx, osync, line, false);
    }

    /// Print `msg` to stderr with the make program/file:line prefix and a
    /// trailing newline.
    pub fn error(ctx: &ExecContext, osync: Option<&mut output>, loc: Option<&Floc>, msg: &str) {
        let line = format!("{}{msg}\n", build_prefix(ctx, loc, false));
        write_line(ctx, osync, line, true);
    }

    /// Print `msg` to stderr with the make program/file:line prefix plus
    /// the `*** ` fatal marker, append `.  Stop.\n`, and exit with
    /// `MAKE_FAILURE`.
    pub fn fatal(ctx: &ExecContext, osync: Option<&mut output>, loc: Option<&Floc>, msg: &str) -> ! {
        let line = format!("{}{msg}.  Stop.\n", build_prefix(ctx, loc, true));
        write_line(ctx, osync, line, true);
        // SAFETY: see `output::fatal`'s `die` exception.
        unsafe { die(ctx, MAKE_FAILURE) }
    }

    #[cfg(test)]
    mod program_name_tests {
        use super::program_name;

        #[test]
        fn falls_back_to_make_when_unset() {
            let ctx = crate::execctx::ExecContext::default();
            assert_eq!(program_name(&ctx), "make");
        }

        #[test]
        fn reflects_the_installed_program_name() {
            let ctx = crate::execctx::ExecContext::default();
            ctx.program.0.set(c"mymake".as_ptr());
            assert_eq!(program_name(&ctx), "mymake");
        }
    }
}

/// Print a `<prefix><msg>` line to stderr, formatting `msg` with `format!`
/// syntax. Safe wrapper over [`msg::error`] — `$loc` is an `Option<&Floc>`,
/// `$osync` an `Option<&mut output>` (see [`output_context`]).
#[macro_export]
macro_rules! error {
    ($ctx:expr, $osync:expr, $loc:expr, $($arg:tt)*) => {
        $crate::output::msg::error($ctx, $osync, $loc, &::std::format!($($arg)*))
    };
}

/// Print a fatal `<prefix>*** <msg>.  Stop.` line to stderr and exit, formatting
/// `msg` with `format!` syntax. Safe wrapper over [`msg::fatal`]; never returns.
/// `$loc` is an `Option<&Floc>`, `$osync` an `Option<&mut output>`.
#[macro_export]
macro_rules! fatal {
    ($ctx:expr, $osync:expr, $loc:expr, $($arg:tt)*) => {
        $crate::output::msg::fatal($ctx, $osync, $loc, &::std::format!($($arg)*))
    };
}

#[cfg(test)]
mod log_working_directory_tests {
    //! #461 review: the plugin ABI can reach `log_working_directory` with a
    //! throwaway `ExecContext` whose `program` is null; the trace must fall
    //! back to the plain "make" name instead of handing null to `strlen`.

    #[test]
    fn tolerates_null_program_context() {
        crate::make_main::install_default_options_for_test();
        let ctx = crate::execctx::ExecContext::default();
        assert!(ctx.program.as_cstr().is_none());
        let traced = super::log_working_directory(&ctx, 1);
        assert_eq!(traced, 1);
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
        crate::make_main::install_default_exec_context_for_test();
        let mut record = super::output {
            out: 7,
            err: 8,
            syncout: [1; 1],
            c2rust_padding: [0; 3],
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
    use super::{_outputs, output, OUTPUT_NONE};

    /// A unique temp path; the file is created by `open_temp_fd`.
    fn temp_path(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("outputs-{tag}-{nanos}-{}", std::process::id()))
    }

    /// Open `path` for read/write, creating/truncating it, returning a raw fd
    /// (the caller takes ownership and must close it).
    fn open_temp_fd(path: &std::path::Path) -> i32 {
        use std::os::unix::io::IntoRawFd;
        let file = std::fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("open temp file");
        file.into_raw_fd()
    }

    /// Read the whole file at `path` to a string.
    fn read_all(path: &std::path::Path) -> String {
        std::fs::read_to_string(path).expect("read temp file")
    }

    /// Build an `output` with `syncout` enabled and its stdout/stderr
    /// descriptors pointed at two raw fds.
    fn sync_output(out_fd: i32, err_fd: i32) -> output {
        let mut o = output {
            out: out_fd,
            err: err_fd,
            syncout: [0; 1],
            c2rust_padding: [0; 3],
        };
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
        let out_fd = open_temp_fd(&out_path);
        let err_fd = open_temp_fd(&err_path);
        let mut o = sync_output(out_fd, err_fd);

        _outputs(Some(&mut o), 0, c"to-stdout\n");
        _outputs(Some(&mut o), 1, c"to-stderr\n");
        use std::os::unix::io::FromRawFd;
        // SAFETY: `out_fd`/`err_fd` were opened by `open_temp_fd` above and
        // aren't otherwise closed.
        unsafe {
            drop(std::fs::File::from_raw_fd(out_fd));
            drop(std::fs::File::from_raw_fd(err_fd));
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
    /// skipped and the call falls through to the stdio writer. Driving it with
    /// an empty message keeps the test output clean while still exercising the
    /// `fd == OUTPUT_NONE` branch and the `std::io::stdout()` write+flush tail.
    #[test]
    fn falls_through_when_descriptor_is_none() {
        let mut o = sync_output(OUTPUT_NONE, OUTPUT_NONE);
        _outputs(Some(&mut o), 0, c"");
    }

    /// A null `output` (no sync context) goes straight to the stdio writer.
    #[test]
    fn null_output_uses_stdio() {
        _outputs(None, 0, c"");
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
        let cases: &[Option<&::core::ffi::CStr>] =
            &[None, Some(c""), Some(c"make"), Some(c"foo.mk:12: "), Some(c"x")];
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
