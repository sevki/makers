//! Output handling: the `message`/`error`/`fatal` printers and the
//! output-sync machinery that captures recipe output into temp files and
//! dumps it atomically.
//!
//! Port of `output.c`. The variadic printers keep their C ABI because they
//! are called with printf-style argument lists from all over the crate; the
//! [`msg`] submodule provides native-Rust counterparts.

use ::core::ffi::{c_char, c_int, c_uint, c_void};
use ::core::ptr::{null, null_mut};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use libc::{
    __errno_location, close, exit, ftruncate, lseek, perror, read, sprintf, strcat, strerror,
    strlen, EINTR, SEEK_END, SEEK_SET,
};

use crate::ffi_types::{__off_t, size_t, uintmax_t};
use crate::floc::Floc;
use crate::make_main::{
    die, makelevel, output_sync, program, should_print_dir, starting_directory,
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
    fn vsprintf(s: *mut c_char, format: *const c_char, arg: ::core::ffi::VaList) -> c_int;
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
/// Set once make has logged the working-directory "Entering directory" trace,
/// so the matching "Leaving directory" is emitted and `MAKE_RESTARTS` is
/// prefixed with `-`. It is a one-shot boolean, stored in an atomic so its
/// reads are plain safe operations; all access is single-threaded, so
/// `Relaxed` preserves the original program order. `pub` because writes also
/// occur in `main.rs`.
pub static STDIO_TRACED: AtomicBool = AtomicBool::new(false);

/// Whether the working-directory enter trace has been emitted.
pub fn stdio_traced() -> bool {
    STDIO_TRACED.load(Ordering::Relaxed)
}
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
    if crate::make_main::FLAGS.print_data_base_flag != 0 {
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
        IN_SETUP.store(false, Ordering::Relaxed);
        return;
    }
    error(
        null::<Floc>(),
        0,
        c"cannot open output-sync lock file: suppressing output-sync".as_ptr(),
    );
    output_close(out);
    output_sync = OUTPUT_SYNC_NONE;
    osync_clear();
    IN_SETUP.store(false, Ordering::Relaxed);
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
                0,
                c"warning: cannot acquire output lock: disabling output sync".as_ptr(),
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
            (output_sync != 0) as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
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
pub unsafe fn output_close(out: *mut output) {
    if out.is_null() {
        if stdio_traced() {
            log_working_directory(0);
        }
        fd_reset_append(fileno(stdout), STDOUT_FLAGS.load(Ordering::Relaxed));
        fd_reset_append(fileno(stderr), STDERR_FLAGS.load(Ordering::Relaxed));
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
        && !stdio_traced()
        && should_print_dir() != 0
    {
        STDIO_TRACED.store(log_working_directory(1) != 0, Ordering::Relaxed);
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
/// printf-style message to stdout, optionally prefixed with the program
/// name; `len` must bound the formatted arguments' length.
///
/// # Safety
/// `fmt` and the variadic arguments must form a valid printf invocation
/// whose expansion fits in `len` extra bytes.
pub unsafe extern "C" fn message(
    prefix: ::core::ffi::c_int,
    mut len: size_t,
    fmt: *const ::core::ffi::c_char,
    args: ...
) {
    len = (len as ::core::ffi::c_ulong).wrapping_add(
        strlen(fmt)
            .wrapping_add(strlen(program))
            .wrapping_add(INTSTR_LENGTH)
            .wrapping_add(4)
            .wrapping_add(1)
            .wrapping_add(1) as ::core::ffi::c_ulong,
    ) as size_t as size_t;
    let start: *mut ::core::ffi::c_char = get_buffer(len);
    let mut p = start;
    if prefix != 0 {
        p = p.offset(
            (if makelevel == 0 {
                sprintf(p, c"%s: ".as_ptr(), program)
            } else {
                sprintf(p, c"%s[%u]: ".as_ptr(), program, makelevel)
            }) as isize,
        );
    }
    let args_0 = args.clone();
    vsprintf(p, fmt, args_0);
    strcat(p, c"\n".as_ptr());
    assert!(
        *start.add(len - 1) == 0,
        "formatted message overran its buffer"
    );
    outputs(0, start);
}
/// printf-style error to stderr with a file:line or program prefix;
/// `len` must bound the formatted arguments' length.
///
/// # Safety
/// `flocp` must be null or valid; `fmt` and the variadic arguments must
/// form a valid printf invocation whose expansion fits in `len` extra
/// bytes.
pub unsafe extern "C" fn error(
    flocp: *const Floc,
    mut len: size_t,
    fmt: *const ::core::ffi::c_char,
    args: ...
) {
    len = (len as ::core::ffi::c_ulong).wrapping_add(
        strlen(fmt)
            .wrapping_add(strlen(program))
            .wrapping_add(if !flocp.is_null() && !(*flocp).filenm.is_null() {
                strlen((*flocp).filenm)
            } else {
                0
            })
            .wrapping_add(INTSTR_LENGTH)
            .wrapping_add(4)
            .wrapping_add(1)
            .wrapping_add(1) as ::core::ffi::c_ulong,
    ) as size_t as size_t;
    let start: *mut ::core::ffi::c_char = get_buffer(len);
    let mut p = start;
    p = p.offset(
        (if !flocp.is_null() && !(*flocp).filenm.is_null() {
            sprintf(
                p,
                c"%s:%lu: ".as_ptr(),
                (*flocp).filenm,
                (*flocp).lineno.wrapping_add((*flocp).offset),
            )
        } else if makelevel == 0 {
            sprintf(p, c"%s: ".as_ptr(), program)
        } else {
            sprintf(p, c"%s[%u]: ".as_ptr(), program, makelevel)
        }) as isize,
    );
    let args_0 = args.clone();
    vsprintf(p, fmt, args_0);
    strcat(p, c"\n".as_ptr());
    assert!(
        *start.add(len - 1) == 0,
        "formatted message overran its buffer"
    );
    outputs(1, start);
}
/// Like [`error`] but adds the `*** ` marker and `.  Stop.` suffix, then
/// dies with `MAKE_FAILURE`.
///
/// # Safety
/// Same contract as [`error`].
pub unsafe extern "C" fn fatal(
    flocp: *const Floc,
    mut len: size_t,
    fmt: *const ::core::ffi::c_char,
    args: ...
) -> ! {
    let stop: *const ::core::ffi::c_char = c".  Stop.\n".as_ptr();
    len = (len as ::core::ffi::c_ulong).wrapping_add(
        strlen(fmt)
            .wrapping_add(strlen(program))
            .wrapping_add(if !flocp.is_null() && !(*flocp).filenm.is_null() {
                strlen((*flocp).filenm)
            } else {
                0
            })
            .wrapping_add(INTSTR_LENGTH)
            .wrapping_add(8)
            .wrapping_add(strlen(stop))
            .wrapping_add(1) as ::core::ffi::c_ulong,
    ) as size_t as size_t;
    let start: *mut ::core::ffi::c_char = get_buffer(len);
    let mut p = start;
    p = p.offset(
        (if !flocp.is_null() && !(*flocp).filenm.is_null() {
            sprintf(
                p,
                c"%s:%lu: *** ".as_ptr(),
                (*flocp).filenm,
                (*flocp).lineno.wrapping_add((*flocp).offset),
            )
        } else if makelevel == 0 {
            sprintf(p, c"%s: *** ".as_ptr(), program)
        } else {
            sprintf(p, c"%s[%u]: *** ".as_ptr(), program, makelevel)
        }) as isize,
    );
    let args_0 = args.clone();
    vsprintf(p, fmt, args_0);
    strcat(p, stop);
    assert!(
        *start.add(len - 1) == 0,
        "formatted message overran its buffer"
    );
    outputs(1, start);
    die(MAKE_FAILURE);
}
/// Format into the shared buffer with an optional prefix and return it.
///
/// # Safety
/// Same printf contract as [`message`]; the returned buffer is shared.
pub unsafe extern "C" fn format(
    prefix: *const ::core::ffi::c_char,
    mut len: size_t,
    fmt: *const ::core::ffi::c_char,
    args: ...
) -> *mut ::core::ffi::c_char {
    let plen: size_t = if !prefix.is_null() {
        strlen(prefix) as size_t
    } else {
        0
    };
    len = len.wrapping_add(strlen(fmt).wrapping_add(plen as size_t).wrapping_add(1) as size_t);
    let start: *mut ::core::ffi::c_char = get_buffer(len);
    let mut p = start;
    if plen != 0 {
        p = mempcpy(
            p as *mut ::core::ffi::c_void,
            prefix as *const ::core::ffi::c_void,
            plen as size_t,
        ) as *mut ::core::ffi::c_char;
    }
    let args_0 = args.clone();
    vsprintf(p, fmt, args_0);
    start
}
/// Report `str``name`: strerror(errno) via [`error`].
///
/// # Safety
/// `str` and `name` must be valid NUL-terminated strings.
pub unsafe fn perror_with_name(str: *const ::core::ffi::c_char, name: *const ::core::ffi::c_char) {
    let err: *const ::core::ffi::c_char = strerror(*__errno_location());
    error(
        null::<Floc>(),
        (strlen(str) as size_t)
            .wrapping_add(strlen(name) as size_t)
            .wrapping_add(strlen(err) as size_t),
        c"%s%s: %s".as_ptr(),
        str,
        name,
        err,
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
        (strlen(name) as size_t).wrapping_add(strlen(err) as size_t),
        c"%s: %s".as_ptr(),
        name,
        err,
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

/// Print a `<prefix><msg>` line to stderr, formatting `msg` with `format!`
/// syntax. Safe wrapper over [`msg::error`] — `$loc` is an `Option<&Floc>`.
#[macro_export]
macro_rules! error {
    ($loc:expr, $($arg:tt)*) => {
        $crate::output::msg::error($loc, &::std::format!($($arg)*))
    };
}

/// Print a fatal `<prefix>*** <msg>.  Stop.` line to stderr and exit, formatting
/// `msg` with `format!` syntax. Safe wrapper over [`msg::fatal`]; never returns.
/// `$loc` is an `Option<&Floc>`.
#[macro_export]
macro_rules! fatal {
    ($loc:expr, $($arg:tt)*) => {
        $crate::output::msg::fatal($loc, &::std::format!($($arg)*))
    };
}
