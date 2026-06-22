//! Output handling: the `message`/`error`/`fatal` printers and the
//! output-sync machinery that captures recipe output into temp files and
//! dumps it atomically.
//!
//! Port of `output.c`. The variadic printers keep their C ABI because they
//! are called with printf-style argument lists from all over the crate; the
//! [`msg`] submodule provides native-Rust counterparts.

use ::core::ffi::{c_char, c_int, c_uint, c_void};
use ::core::ptr::{null, null_mut};

use libc::{
    __errno_location, close, exit, ftruncate, lseek, perror, read, sprintf, strerror, strlen,
    EINTR, SEEK_END, SEEK_SET,
};

use crate::ffi_types::{__off_t, size_t, uintmax_t};
use crate::floc::Floc;
use crate::make_main::{
    die, makelevel, output_sync, print_data_base_flag, program, should_print_dir,
    starting_directory,
};
use crate::misc::{get_tmpfd, writebuf, xrealloc};
use crate::posixos::{
    check_io_state, fd_noinherit, fd_reset_append, fd_set_append, osync_acquire, osync_clear,
    osync_release,
};
use crate::stdio::FILE;

extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn fwrite(ptr: *const c_void, size: size_t, n: size_t, s: *mut FILE) -> ::core::ffi::c_ulong;
    fn fileno(stream: *mut FILE) -> c_int;
    fn mempcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
}

/// Per-target output state: temp-file descriptors for stdout/stderr while
/// output sync is active.
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct output {
    pub out: c_int,
    pub err: c_int,
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

pub const OUTPUT_SYNC_NONE: c_int = 0;
pub const OUTPUT_SYNC_RECURSE: c_int = 3;
pub const MAKE_FAILURE: c_int = 2;

/// `check_io_state` bits (see os.h).
const IO_COMBINED_OUTERR: c_uint = 0x0002;
const IO_STDOUT_OK: c_uint = 0x0008;
const IO_STDERR_OK: c_uint = 0x0010;

pub static mut output_context: *mut output = ::core::ptr::null::<output>() as *mut output;
pub static mut stdio_traced: ::core::ffi::c_uint = 0;
pub const OUTPUT_NONE: ::core::ffi::c_int = -1;
unsafe extern "C" fn _outputs(
    out: *mut output,
    is_err: ::core::ffi::c_int,
    msg: *const ::core::ffi::c_char,
) {
    if !out.is_null() && (*out).syncout() as ::core::ffi::c_int != 0 {
        let fd: ::core::ffi::c_int = if is_err != 0 { (*out).err } else { (*out).out };
        if fd != OUTPUT_NONE {
            let len: size_t = strlen(msg) as size_t;
            let mut r: ::core::ffi::c_int;
            loop {
                r = lseek(fd, 0, 2) as ::core::ffi::c_int;
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
pub unsafe fn log_working_directory(entering: ::core::ffi::c_int) -> ::core::ffi::c_int {
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
    if print_data_base_flag != 0 {
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
pub unsafe fn pump_from_tmp(from: ::core::ffi::c_int, to: *mut FILE) {
    static mut buffer: [::core::ffi::c_char; 8192] = [0; 8192];
    if lseek(from, 0, SEEK_SET) == -1 as __off_t {
        perror(c"lseek()".as_ptr());
    }
    loop {
        let mut len: ::core::ffi::c_int;
        loop {
            len = read(
                from,
                &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as size_t,
            ) as ::core::ffi::c_int;
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
pub unsafe fn output_tmpfd() -> ::core::ffi::c_int {
    let fd: ::core::ffi::c_int = get_tmpfd(null_mut());
    fd_set_append(fd);
    fd
}
/// Set up the output-sync temp files for `out`, disabling output sync
/// (with a message) on failure.
///
/// # Safety
/// `out` must point to a valid `output`; must run single-threaded.
pub unsafe fn setup_tmpfile(out: *mut output) {
    static mut in_setup: ::core::ffi::c_uint = 0;
    if in_setup != 0 {
        return;
    }
    in_setup = 1;
    let io_state: ::core::ffi::c_uint = check_io_state();
    // The block falls through to the error handler below on any failure;
    // reaching its end is the success path (the C code used `goto`).
    'setup: {
        if io_state & (IO_STDOUT_OK | IO_STDERR_OK) == 0 {
            perror_with_name(c"output-sync suppressed: ".as_ptr(), c"stderr".as_ptr());
            break 'setup;
        }
        if io_state & IO_STDOUT_OK != 0 {
            let fd: ::core::ffi::c_int = output_tmpfd();
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
                let fd_0: ::core::ffi::c_int = output_tmpfd();
                if fd_0 < 0 {
                    break 'setup;
                }
                fd_noinherit(fd_0);
                (*out).err = fd_0;
            }
        }
        in_setup = 0;
        return;
    }
    error(
        null::<Floc>(),
        c"cannot open output-sync lock file: suppressing output-sync".as_ptr(),
        &[],
    );
    output_close(out);
    output_sync = OUTPUT_SYNC_NONE;
    osync_clear();
    in_setup = 0;
}
/// Dump any captured output under the output-sync lock and truncate the
/// temp files for reuse.
///
/// # Safety
/// `out` must point to a valid `output`; must run single-threaded.
pub unsafe fn output_dump(out: *mut output) {
    let outfd_not_empty: ::core::ffi::c_int =
        ((*out).out != OUTPUT_NONE && lseek((*out).out, 0, SEEK_END) > 0) as ::core::ffi::c_int;
    let errfd_not_empty: ::core::ffi::c_int =
        ((*out).err != OUTPUT_NONE && lseek((*out).err, 0, SEEK_END) > 0) as ::core::ffi::c_int;
    if outfd_not_empty != 0 || errfd_not_empty != 0 {
        let mut traced: ::core::ffi::c_int = 0;
        if osync_acquire() == 0 {
            error(
                null::<Floc>(),
                c"warning: cannot acquire output lock: disabling output sync".as_ptr(),
                &[],
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
            let mut e: ::core::ffi::c_int;
            lseek((*out).out, 0, SEEK_SET);
            loop {
                e = ftruncate((*out).out, 0);
                if !(e == -1 && *__errno_location() == EINTR) {
                    break;
                }
            }
        }
        if (*out).err != OUTPUT_NONE && (*out).err != (*out).out {
            let mut e_0: ::core::ffi::c_int;
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
static mut stdout_flags: ::core::ffi::c_int = -1;
static mut stderr_flags: ::core::ffi::c_int = -1;
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
            (output_sync != 0) as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        return;
    }
    stdout_flags = fd_set_append(fileno(stdout));
    stderr_flags = fd_set_append(fileno(stderr));
}
/// Dump and close `out`'s temp files (or, when null, restore
/// stdout/stderr).
///
/// # Safety
/// `out` must be null or point to a valid `output`; must run
/// single-threaded.
pub unsafe fn output_close(out: *mut output) {
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
/// Lazily set up output sync and the enter-directory trace before the
/// first real output.
///
/// # Safety
/// Must run single-threaded: touches output and trace globals.
pub unsafe fn output_start() {
    if !output_context.is_null()
        && (*output_context).syncout() as ::core::ffi::c_int != 0
        && !((*output_context).out >= 0 || (*output_context).err >= 0)
    {
        setup_tmpfile(output_context);
    }
    if (output_sync == OUTPUT_SYNC_NONE || output_sync == OUTPUT_SYNC_RECURSE)
        && stdio_traced == 0
        && should_print_dir() != 0
    {
        stdio_traced = log_working_directory(1) as ::core::ffi::c_uint;
    }
}
/// Write `msg` to stdout or stderr (or the sync temp file), starting
/// output first.
///
/// # Safety
/// `msg` must be null or a valid NUL-terminated string.
pub unsafe fn outputs(is_err: ::core::ffi::c_int, msg: *const ::core::ffi::c_char) {
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

unsafe fn push_cstr(out: &mut Vec<u8>, s: *const ::core::ffi::c_char) {
    if !s.is_null() {
        out.extend_from_slice(::core::ffi::CStr::from_ptr(s).to_bytes());
    }
}

unsafe fn push_program_prefix(out: &mut Vec<u8>, fatal_marker: bool) {
    push_cstr(out, program);
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

unsafe fn push_error_prefix(out: &mut Vec<u8>, flocp: *const Floc, fatal_marker: bool) {
    if !flocp.is_null() && !(*flocp).filenm.is_null() {
        push_cstr(out, (*flocp).filenm);
        out.push(b':');
        out.extend_from_slice(
            (*flocp)
                .lineno
                .wrapping_add((*flocp).offset)
                .to_string()
                .as_bytes(),
        );
        out.extend_from_slice(b": ");
        if fatal_marker {
            out.extend_from_slice(b"*** ");
        }
    } else {
        push_program_prefix(out, fatal_marker);
    }
}

unsafe fn write_formatted(is_err: bool, mut out: Vec<u8>) {
    out.push(0);
    outputs(
        if is_err { 1 } else { 0 },
        out.as_ptr() as *const ::core::ffi::c_char,
    );
}

/// printf-subset message to stdout, optionally prefixed with the program name.
///
/// # Safety
/// `fmt` must be a valid NUL-terminated format string. The format
/// specifiers must match `args`.
#[no_mangle]
pub unsafe extern "C" fn message(
    prefix: ::core::ffi::c_int,
    fmt: *const ::core::ffi::c_char,
    args: &[FmtArg],
) {
    let mut out = Vec::new();
    if prefix != 0 {
        push_program_prefix(&mut out, false);
    }
    vformat_into(&mut out, fmt, args);
    out.push(b'\n');
    write_formatted(false, out);
}

/// printf-subset error to stderr with a file:line or program prefix.
///
/// # Safety
/// `flocp` must be null or valid. `fmt` must be a valid NUL-terminated
/// format string and the format specifiers must match `args`.
#[no_mangle]
pub unsafe extern "C" fn error(
    flocp: *const Floc,
    fmt: *const ::core::ffi::c_char,
    args: &[FmtArg],
) {
    let mut out = Vec::new();
    push_error_prefix(&mut out, flocp, false);
    vformat_into(&mut out, fmt, args);
    out.push(b'\n');
    write_formatted(true, out);
}

/// Like [`error`] but adds the `*** ` marker and `.  Stop.` suffix, then
/// dies with `MAKE_FAILURE`.
///
/// # Safety
/// Same contract as [`error`].
pub unsafe extern "C" fn fatal(
    flocp: *const Floc,
    fmt: *const ::core::ffi::c_char,
    args: &[FmtArg],
) -> ! {
    let mut out = Vec::new();
    push_error_prefix(&mut out, flocp, true);
    vformat_into(&mut out, fmt, args);
    out.extend_from_slice(b".  Stop.\n");
    write_formatted(true, out);
    die(MAKE_FAILURE);
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
    push_cstr(&mut out, prefix);
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
pub unsafe fn perror_with_name(str: *const ::core::ffi::c_char, name: *const ::core::ffi::c_char) {
    let err: *const ::core::ffi::c_char = strerror(*__errno_location());
    error(
        null::<Floc>(),
        c"%s%s: %s".as_ptr(),
        &[FmtArg::Str(str), FmtArg::Str(name), FmtArg::Str(err)],
    );
}
/// Report `name`: strerror(errno) via [`fatal`] and die.
///
/// # Safety
/// `name` must be a valid NUL-terminated string.
pub unsafe fn pfatal_with_name(name: *const ::core::ffi::c_char) -> ! {
    let err: *const ::core::ffi::c_char = strerror(*__errno_location());
    fatal(
        null::<Floc>(),
        c"%s: %s".as_ptr(),
        &[FmtArg::Str(name), FmtArg::Str(err)],
    );
}
/// Print the out-of-memory message without allocating and exit with
/// `MAKE_FAILURE`.
///
/// # Safety
/// Always safe to call; unsafe only for C-API signature compatibility.
pub unsafe fn out_of_memory() -> ! {
    writebuf(
        fileno(stdout),
        program as *const ::core::ffi::c_void,
        strlen(program) as size_t,
    );
    writebuf(
        fileno(stdout),
        c": *** virtual memory exhausted\n".as_ptr() as *const ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t).wrapping_sub(1),
    );
    exit(MAKE_FAILURE);
}

/// Native-Rust counterparts to the variadic C-ABI `message`/`error`/`fatal`
/// in this module. Callers build their formatted message with `format!`
/// (or any `Display` source) and hand a `&str` here; the prefix and suffix
/// are added in idiomatic Rust.
///
/// Compatibility note: the variadic extern "C" versions still live above
/// for legacy call sites; both produce identical output formats.
pub mod msg {
    use super::{die, makelevel, outputs, program, MAKE_FAILURE};
    use crate::floc::Floc;

    fn program_name() -> String {
        // SAFETY: `program` is set during make startup and lives for the
        // process lifetime; we read it as a NUL-terminated C string.
        unsafe { ::core::ffi::CStr::from_ptr(program) }
            .to_string_lossy()
            .into_owned()
    }

    fn build_prefix(loc: Option<&Floc>, fatal_marker: bool) -> String {
        let marker = if fatal_marker { "*** " } else { "" };
        // SAFETY: `(*flocp).filenm` is a NUL-terminated C string when non-null.
        unsafe {
            match loc {
                Some(f) if !f.filenm.is_null() => {
                    let fnm = ::core::ffi::CStr::from_ptr(f.filenm).to_string_lossy();
                    format!("{}:{}: {}", fnm, f.lineno.wrapping_add(f.offset), marker)
                }
                _ => {
                    let lvl = makelevel;
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

    fn write_line(line: String, is_err: bool) {
        let mut bytes = line.into_bytes();
        bytes.push(0);
        // SAFETY: `outputs` reads up to the trailing NUL we just appended.
        unsafe {
            outputs(
                if is_err { 1 } else { 0 },
                bytes.as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }

    /// Print `msg` to stdout with a trailing newline. If `with_prefix`,
    /// prepend the make program name (and `[LEVEL]` when nested).
    pub fn message(with_prefix: bool, msg: &str) {
        let line = if with_prefix {
            format!("{}{msg}\n", build_prefix(None, false))
        } else {
            format!("{msg}\n")
        };
        write_line(line, false);
    }

    /// Print `msg` to stderr with the make program/file:line prefix and a
    /// trailing newline.
    pub fn error(loc: Option<&Floc>, msg: &str) {
        let line = format!("{}{msg}\n", build_prefix(loc, false));
        write_line(line, true);
    }

    /// Print `msg` to stderr with the make program/file:line prefix plus
    /// the `*** ` fatal marker, append `.  Stop.\n`, and exit with
    /// `MAKE_FAILURE`.
    pub fn fatal(loc: Option<&Floc>, msg: &str) -> ! {
        let line = format!("{}{msg}.  Stop.\n", build_prefix(loc, true));
        write_line(line, true);
        // SAFETY: `die` is the make-process exit point and never returns.
        unsafe { die(MAKE_FAILURE) }
    }
}
