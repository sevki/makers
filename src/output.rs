//! Output handling: the `message`/`error`/`fatal` printers and the
//! output-sync machinery that captures recipe output into temp files and
//! dumps it atomically.
//!
//! Port of `output.c`. The variadic printers keep their C ABI because they
//! are called with printf-style argument lists from all over the crate; the
//! [`msg`] submodule provides native-Rust counterparts.

use ::core::ffi::{c_char, c_uint, c_void};
use ::core::ptr::{null, null_mut};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use libc::{
    __errno_location, close, ftruncate, lseek, perror, read, sprintf, strerror, strlen,
    EINTR, SEEK_END, SEEK_SET,
};

use crate::execctx::ExecContext;
use crate::ffi_types::{__off_t, size_t, uintmax_t};
use crate::floc::Floc;
use crate::make_main::{die, program, starting_directory};
use crate::misc::{open_anon_tmpfd, writebuf, xrealloc};
use crate::posixos::{
    check_io_state, fd_noinherit, fd_reset_append, fd_set_append, osync_acquire, osync_clear,
    osync_release,
};
use crate::stdio::FILE;

extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(stream: *mut FILE) -> i32;
    fn fputs(s: *const c_char, stream: *mut FILE) -> i32;
    fn fwrite(ptr: *const c_void, size: size_t, n: size_t, s: *mut FILE) -> ::core::ffi::c_ulong;
    fn fileno(stream: *mut FILE) -> i32;
}

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

/// The shared, growing buffer used by the printf-style printers.
#[derive(Copy, Clone)]
#[repr(C)]
struct fmtstring {
    buffer: *mut c_char,
    size: size_t,
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

pub static mut output_context: *mut output = ::core::ptr::null::<output>() as *mut output;
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
unsafe extern "C" fn _outputs(out: *mut output, is_err: i32, msg: *const ::core::ffi::c_char) {
    if !out.is_null() && (*out).syncout() as i32 != 0 {
        let fd: i32 = if is_err != 0 { (*out).err } else { (*out).out };
        if fd != OUTPUT_NONE {
            let len: size_t = strlen(msg) as size_t;
            let mut r: i32;
            loop {
                r = lseek(fd, 0, 2) as i32;
                if !(r == -1 && *__errno_location() == EINTR) {
                    break;
                }
            }
            writebuf(fd, msg as *const ::core::ffi::c_void, len);
            return;
        }
    }
    let f: *mut FILE = if is_err != 0 { stderr } else { stdout };
    fputs(msg, f);
    fflush(f);
}
/// Print an entering/leaving-directory line (returns 1).
///
/// # Safety
/// Must run single-threaded: reads make globals and a static buffer.
pub unsafe fn log_working_directory(ctx: &ExecContext, entering: i32) -> i32 {
    let makelevel = ctx.makelevel();
    static mut buf: *mut ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    static mut len: size_t = 0;
    let mut need: size_t;
    let fmt: *const ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char;
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
    need = need.wrapping_add(strlen(fmt) as size_t);
    if need > len {
        buf = xrealloc(buf as *mut ::core::ffi::c_void, need) as *mut ::core::ffi::c_char;
        len = need;
    }
    p = buf;
    if crate::make_main::opt_print_data_base() {
        let fresh0 = p;
        p = p.add(1);
        *fresh0 = '#' as i32 as ::core::ffi::c_char;
        let fresh1 = p;
        p = p.add(1);
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
    _outputs(null_mut(), 0, buf);
    1
}
/// Copy everything from fd `from` to stream `to`, from the beginning.
///
/// # Safety
/// `from` must be an open, seekable fd and `to` an open stream; uses a
/// static buffer, so must run single-threaded.
pub unsafe fn pump_from_tmp(from: i32, to: *mut FILE) {
    static mut buffer: [::core::ffi::c_char; 8192] = [0; 8192];
    if lseek(from, 0, SEEK_SET) == -1 as __off_t {
        perror(c"lseek()".as_ptr());
    }
    loop {
        let mut len: i32;
        loop {
            len = read(
                from,
                &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as size_t,
            ) as i32;
            if !(len == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if len < 0 {
            perror(c"read()".as_ptr());
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
            perror(c"fwrite()".as_ptr());
            break;
        } else {
            fflush(to);
        }
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
    // Atomic so the read/write are plain safe ops; setup runs single-threaded,
    // so `Relaxed` preserves the original program order.
    static IN_SETUP: AtomicBool = AtomicBool::new(false);
    if IN_SETUP.load(Ordering::Relaxed) {
        return;
    }
    IN_SETUP.store(true, Ordering::Relaxed);
    let io_state: ::core::ffi::c_uint = check_io_state();
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
        IN_SETUP.store(false, Ordering::Relaxed);
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
    crate::make_main::with_options(|o| o.output_sync.set(OUTPUT_SYNC_NONE));
    osync_clear();
    IN_SETUP.store(false, Ordering::Relaxed);
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
        if osync_acquire() == 0 {
            error(
                ctx,
                null::<Floc>(),
                0,
                c"warning: cannot acquire output lock: disabling output sync".as_ptr(),
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
            pump_from_tmp((*out).out, stdout);
        }
        if errfd_not_empty != 0 && (*out).err != (*out).out {
            pump_from_tmp((*out).err, stderr);
        }
        if traced != 0 {
            log_working_directory(ctx, 0);
        }
        osync_release();
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
/// Saved `O_APPEND`-state of stdout/stderr while output-sync redirects them,
/// restored by `output_close`. Atomic so the read/write are plain safe ops;
/// output setup/teardown runs single-threaded, so `Relaxed` preserves the
/// original program order.
static STDOUT_FLAGS: AtomicI32 = AtomicI32::new(-1);
static STDERR_FLAGS: AtomicI32 = AtomicI32::new(-1);
/// Initialize `out` (or, when null, switch stdout/stderr to append mode).
///
/// # Safety
/// `out` must be null or point to a valid `output`; must run
/// single-threaded.
pub unsafe fn output_init(out: *mut output) {
    if !out.is_null() {
        (*out).err = OUTPUT_NONE;
        (*out).out = (*out).err;
        (*out).set_syncout(
            (crate::make_main::opt_output_sync() != 0) as i32 as ::core::ffi::c_uint
                as ::core::ffi::c_uint,
        );
        return;
    }
    STDOUT_FLAGS.store(fd_set_append(fileno(stdout)), Ordering::Relaxed);
    STDERR_FLAGS.store(fd_set_append(fileno(stderr)), Ordering::Relaxed);
}
/// Dump and close `out`'s temp files (or, when null, restore
/// stdout/stderr).
///
/// # Safety
/// `out` must be null or point to a valid `output`; must run
/// single-threaded.
pub unsafe fn output_close(ctx: &ExecContext, out: *mut output) {
    if out.is_null() {
        if stdio_traced() {
            log_working_directory(ctx, 0);
        }
        fd_reset_append(fileno(stdout), STDOUT_FLAGS.load(Ordering::Relaxed));
        fd_reset_append(fileno(stderr), STDERR_FLAGS.load(Ordering::Relaxed));
        return;
    }
    output_dump(ctx, out);
    if (*out).out >= 0 {
        close((*out).out);
    }
    if (*out).err >= 0 && (*out).err != (*out).out {
        close((*out).err);
    }
    output_init(out);
}
/// Lazily set up output sync and the enter-directory trace before the
/// first real output.
///
/// # Safety
/// Must run single-threaded: touches output and trace globals.
pub unsafe fn output_start(ctx: &ExecContext) {
    if !output_context.is_null()
        && (*output_context).syncout() as i32 != 0
        && !((*output_context).out >= 0 || (*output_context).err >= 0)
    {
        setup_tmpfile(ctx, output_context);
    }
    if (crate::make_main::opt_output_sync() == OUTPUT_SYNC_NONE
        || crate::make_main::opt_output_sync() == OUTPUT_SYNC_RECURSE)
        && !stdio_traced()
        && crate::make_main::should_print_dir_mirror(ctx) != 0
    {
        set_stdio_traced(log_working_directory(ctx, 1) != 0);
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
    _outputs(output_context, is_err, msg);
}
static mut fmtbuf: fmtstring = fmtstring {
    buffer: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    size: 0,
};
/// Return the shared format buffer, grown to at least `need` bytes.
///
/// # Safety
/// Must run single-threaded; the buffer is shared between calls.
pub unsafe fn get_buffer(need: size_t) -> *mut ::core::ffi::c_char {
    if need > fmtbuf.size {
        fmtbuf.size = fmtbuf.size.wrapping_add(need.wrapping_mul(2));
        fmtbuf.buffer = xrealloc(fmtbuf.buffer as *mut ::core::ffi::c_void, fmtbuf.size)
            as *mut ::core::ffi::c_char;
    }
    *fmtbuf.buffer.add(need - 1) = 0;
    fmtbuf.buffer
}
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

unsafe fn push_program_prefix(out: &mut Vec<u8>, makelevel: u32, fatal_marker: bool) {
    push_cstr(out, (!program.is_null()).then(|| ::core::ffi::CStr::from_ptr(program)));
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
    out: &mut Vec<u8>,
    flocp: *const Floc,
    makelevel: u32,
    fatal_marker: bool,
) {
    if let Some(fl) = flocp.as_ref().filter(|fl| !fl.filenm.is_null()) {
        // `filenm` is non-null in this arm, so it is always `Some`.
        push_cstr(out, Some(::core::ffi::CStr::from_ptr(fl.filenm)));
        out.push(b':');
        out.extend_from_slice(
            fl.lineno
                .wrapping_add(fl.offset)
                .to_string()
                .as_bytes(),
        );
        out.extend_from_slice(b": ");
        if fatal_marker {
            out.extend_from_slice(b"*** ");
        }
    } else {
        push_program_prefix(out, makelevel, fatal_marker);
    }
}

/// printf-subset message to stdout, optionally prefixed with the program name.
///
/// # Safety
/// `fmt` must be a valid NUL-terminated format string. The format
/// specifiers must match `args`.
#[no_mangle]
pub unsafe extern "C" fn message(
    ctx: &ExecContext,
    prefix: i32,
    _len: size_t,
    fmt: *const ::core::ffi::c_char,
    args: &[FmtArg],
) {
    let makelevel = ctx.makelevel();
    let mut out: Vec<u8> = Vec::new();
    if prefix != 0 {
        push_program_prefix(&mut out, makelevel, false);
    }
    vformat_into(&mut out, fmt, args);
    out.push(b'\n');
    out.push(0);
    let start: *mut ::core::ffi::c_char = get_buffer(out.len() as size_t);
    ::core::ptr::copy_nonoverlapping(out.as_ptr(), start as *mut u8, out.len());
    outputs(ctx, 0, start);
}

/// printf-subset error to stderr with a file:line or program prefix.
///
/// # Safety
/// `flocp` must be null or valid. `fmt` must be a valid NUL-terminated
/// format string and the format specifiers must match `args`.
#[no_mangle]
pub unsafe extern "C" fn error(
    ctx: &ExecContext,
    flocp: *const Floc,
    _len: size_t,
    fmt: *const ::core::ffi::c_char,
    args: &[FmtArg],
) {
    let makelevel = ctx.makelevel();
    let mut out: Vec<u8> = Vec::new();
    push_error_prefix(&mut out, flocp, makelevel, false);
    vformat_into(&mut out, fmt, args);
    out.push(b'\n');
    out.push(0);
    let start: *mut ::core::ffi::c_char = get_buffer(out.len() as size_t);
    ::core::ptr::copy_nonoverlapping(out.as_ptr(), start as *mut u8, out.len());
    outputs(ctx, 1, start);
}

/// Like [`error`] but adds the `*** ` marker and `.  Stop.` suffix, then
/// dies with `MAKE_FAILURE`.
///
/// # Safety
/// Same contract as [`error`].
pub unsafe extern "C" fn fatal(
    ctx: &ExecContext,
    flocp: *const Floc,
    _len: size_t,
    fmt: *const ::core::ffi::c_char,
    args: &[FmtArg],
) -> ! {
    let makelevel = ctx.makelevel();
    let mut out: Vec<u8> = Vec::new();
    push_error_prefix(&mut out, flocp, makelevel, true);
    vformat_into(&mut out, fmt, args);
    out.extend_from_slice(b".  Stop.\n");
    out.push(0);
    let start: *mut ::core::ffi::c_char = get_buffer(out.len() as size_t);
    ::core::ptr::copy_nonoverlapping(out.as_ptr(), start as *mut u8, out.len());
    outputs(ctx, 1, start);
    die(ctx, MAKE_FAILURE);
}

/// Format into the shared buffer with an optional prefix and return it.
///
/// # Safety
/// `prefix` may be null. `fmt` must be a valid NUL-terminated format string
/// and the format specifiers must match `args`. The returned buffer is shared.
#[no_mangle]
pub unsafe extern "C" fn format(
    prefix: *const ::core::ffi::c_char,
    fmt: *const ::core::ffi::c_char,
    args: &[FmtArg],
) -> *mut ::core::ffi::c_char {
    let mut out = Vec::new();
    push_cstr(&mut out, (!prefix.is_null()).then(|| ::core::ffi::CStr::from_ptr(prefix)));
    vformat_into(&mut out, fmt, args);
    out.push(0);
    let buf = get_buffer(out.len() as size_t);
    ::core::ptr::copy_nonoverlapping(out.as_ptr(), buf as *mut u8, out.len());
    buf
}

/// Backwards-compatible name for callers migrated from the old printf helper.
///
/// # Safety
/// Same contract as [`format`].
pub unsafe fn format_message(
    prefix: *const ::core::ffi::c_char,
    fmt: *const ::core::ffi::c_char,
    args: &[FmtArg],
) -> *mut ::core::ffi::c_char {
    format(prefix, fmt, args)
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
/// Report `name`: strerror(errno) via [`fatal`] and die.
///
/// # Safety
/// `name` must be a valid NUL-terminated string.
pub unsafe fn pfatal_with_name(ctx: &ExecContext, name: *const ::core::ffi::c_char) -> ! {
    let err: *const ::core::ffi::c_char = strerror(*__errno_location());
    fatal(
        ctx,
        null::<Floc>(),
        0,
        c"%s: %s".as_ptr(),
        &[FmtArg::Str(name), FmtArg::Str(err)],
    );
}
/// Print the out-of-memory message without allocating and exit with
/// `MAKE_FAILURE`.
pub fn out_of_memory() -> ! {
    use std::io::Write;
    // The program-name read is the one unavoidable C-global access; it is
    // already encapsulated (with its SAFETY note) in `msg::program_name`.
    let prog = msg::program_name();
    let mut out = std::io::stdout().lock();
    #[allow(clippy::write_with_newline)]
    let _ = write!(out, "{prog}: *** virtual memory exhausted\n");
    let _ = out.flush();
    std::process::exit(MAKE_FAILURE)
}

/// Native-Rust counterparts to the variadic C-ABI `message`/`error`/`fatal`
/// in this module. Callers build their formatted message with `format!`
/// (or any `Display` source) and hand a `&str` here; the prefix and suffix
/// are added in idiomatic Rust.
///
/// Compatibility note: the variadic extern "C" versions still live above
/// for legacy call sites; both produce identical output formats.
pub mod msg {
    use super::{die, outputs, program, MAKE_FAILURE};
    use crate::execctx::ExecContext;
    use crate::floc::Floc;

    pub(crate) fn program_name() -> String {
        // SAFETY: `program` is set during make startup and lives for the
        // process lifetime; we read it as a NUL-terminated C string.
        unsafe { ::core::ffi::CStr::from_ptr(program) }
            .to_string_lossy()
            .into_owned()
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
                    let prog = program_name();
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

    /// Print `msg` to stderr with the make program/file:line prefix plus
    /// the `*** ` fatal marker, append `.  Stop.\n`, and exit with
    /// `MAKE_FAILURE`.
    pub fn fatal(ctx: &ExecContext, loc: Option<&Floc>, msg: &str) -> ! {
        let line = format!("{}{msg}.  Stop.\n", build_prefix(ctx, loc, true));
        write_line(ctx, line, true);
        // SAFETY: `die` is the make-process exit point and never returns.
        unsafe { die(ctx, MAKE_FAILURE) }
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
mod outputs_tests {
    use super::{_outputs, output, OUTPUT_NONE};
    use std::ffi::CString;

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
        unsafe {
            let out_fd = open_temp_fd(&out_path);
            let err_fd = open_temp_fd(&err_path);
            let o = sync_output(out_fd, err_fd);

            _outputs(
                &o as *const output as *mut output,
                0,
                c"to-stdout\n".as_ptr(),
            );
            _outputs(
                &o as *const output as *mut output,
                1,
                c"to-stderr\n".as_ptr(),
            );
            libc::close(out_fd);
            libc::close(err_fd);
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
    /// `fd == OUTPUT_NONE` branch and the `fputs`/`fflush` tail.
    #[test]
    fn falls_through_when_descriptor_is_none() {
        unsafe {
            let o = sync_output(OUTPUT_NONE, OUTPUT_NONE);
            _outputs(&o as *const output as *mut output, 0, c"".as_ptr());
        }
    }

    /// A null `output` (no sync context) goes straight to the stdio writer.
    #[test]
    fn null_output_uses_stdio() {
        unsafe {
            _outputs(::core::ptr::null_mut(), 0, c"".as_ptr());
        }
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
