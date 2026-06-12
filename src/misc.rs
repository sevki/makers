//! Miscellaneous generally-useful functions: checked allocation, token
//! scanning, temporary files, and small string helpers.
//!
//! Port of `misc.c`. Most helpers still traffic in raw C strings because
//! their callers (the makefile reader, the variable expander, the job
//! runner) are still C-shaped.

use ::core::ffi::{c_char, c_int, c_longlong, c_uint, c_ulong, c_ulonglong, c_void};
use ::core::ptr::{null, null_mut};

use std::sync::atomic::{AtomicU32, Ordering};

use libc::{
    __errno_location, calloc, free, getenv, getpid, malloc, memcpy, memmove, mkstemp, putchar,
    read, realloc, sleep, sprintf, stpcpy, strchr, strcmp, strcpy, strdup, strerror, strlen,
    strndup, strtoul, umask, unlink, write, EINTR,
};

use crate::ffi_types::{__mode_t, mode_t, pid_t, size_t, ssize_t, time_t};
use crate::file::{nameseq, Dep};
use crate::floc::Floc;
use crate::make_main::{posix_pedantic, stopchar_map};
use crate::output::{error, out_of_memory};
use crate::posixos::os_anontmp;
use crate::stdio::FILE;
use crate::sys_stat::stat;

extern "C" {
    fn stat(file: *const c_char, buf: *mut stat) -> c_int;
    static mut stderr: *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fopen(filename: *const c_char, modes: *const c_char) -> *mut FILE;
    fn fdopen(fd: c_int, modes: *const c_char) -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn vsprintf(s: *mut c_char, format: *const c_char, arg: ::core::ffi::VaList) -> c_int;
    fn time(timer: *mut time_t) -> time_t;
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

/// Character-class bits in `stopchar_map` (see `makeint.h`).
const MAP_NUL: c_int = 0x0001;
const MAP_BLANK: c_int = 0x0002;
const MAP_NEWLINE: c_int = 0x0004;
const MAP_VARSEP: c_int = 0x0080;
const MAP_DIRSEP: c_int = 0x8000;
const MAP_SPACE: c_int = MAP_BLANK | MAP_NEWLINE;

/// `STOP_SET (c, mask)` from `makeint.h`: is `c` in any of the character
/// classes selected by `mask`?
unsafe fn stop_set(c: c_char, mask: c_int) -> bool {
    stopchar_map[c as u8 as usize] as c_int & mask != 0
}

/// File-type test from `S_ISDIR`: `(mode & S_IFMT) == S_IFDIR`.
const S_IFMT: __mode_t = 0o170000;
const S_IFDIR: __mode_t = 0o040000;

#[inline]
unsafe fn free_ns(n: *mut nameseq) {
    free(n as *mut c_void);
}

/// Parse `str` as an unsigned decimal integer. When `error_0` is non-null it
/// receives a static message describing why the value is invalid, or null on
/// success.
/// # Safety
/// `str` must be a valid NUL-terminated string; `error_0` must be null or
/// valid for writes.
pub unsafe fn make_toui(str: *const c_char, error_0: *mut *const c_char) -> c_uint {
    let mut end: *mut c_char = null_mut();
    let val: c_ulong = strtoul(str, &mut end, 10);
    if !error_0.is_null() {
        if *str == 0 {
            *error_0 = c"Missing value".as_ptr();
        } else if matches!(end.as_ref(), Some(&b) if b != 0) {
            *error_0 = c"Invalid value".as_ptr();
        } else {
            *error_0 = null();
        }
    }
    val as c_uint
}

/// Format `val` into `buf` as decimal; returns `buf`.
/// # Safety
/// `buf` must be large enough for the formatted value and its terminator.
pub unsafe fn make_lltoa(val: c_longlong, buf: *mut c_char) -> *mut c_char {
    sprintf(buf, c"%lld".as_ptr(), val);
    buf
}

/// Format `val` into `buf` as decimal; returns `buf`.
/// # Safety
/// `buf` must be large enough for the formatted value and its terminator.
pub unsafe fn make_ulltoa(val: c_ulonglong, buf: *mut c_char) -> *mut c_char {
    sprintf(buf, c"%llu".as_ptr(), val);
    buf
}

static MK_STATE: AtomicU32 = AtomicU32::new(0);

/// Seed the xorshift PRNG used by `--shuffle`.
/// # Safety
/// Always safe; unsafe only for C-API signature compatibility.
pub unsafe fn make_seed(seed: c_uint) {
    MK_STATE.store(seed, Ordering::Relaxed);
}

/// Return the next value from the xorshift PRNG, self-seeding from the time
/// and PID on first use.
/// # Safety
/// Always safe; unsafe only for C-API signature compatibility.
pub unsafe fn make_rand() -> c_uint {
    let mut state = MK_STATE.load(Ordering::Relaxed);
    loop {
        let mut next = if state == 0 {
            ((time(null_mut()) ^ make_pid() as time_t) as c_uint).wrapping_add(1)
        } else {
            state
        };
        next ^= next << 13;
        next ^= next >> 17;
        next ^= next << 5;

        match MK_STATE.compare_exchange_weak(state, next, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return next,
            Err(observed) => state = observed,
        }
    }
}

/// `qsort`-style comparison of two `char *` pointed to by `v1`/`v2`. Compares
/// the first byte inline before falling back to `strcmp`.
///
/// # Safety
/// `v1` and `v2` must point to valid `char *` values that point to valid
/// NUL-terminated strings.
#[no_mangle]
pub unsafe extern "C" fn alpha_compare(v1: *const c_void, v2: *const c_void) -> c_int {
    let s1: *const c_char = *(v1 as *mut *mut c_char);
    let s2: *const c_char = *(v2 as *mut *mut c_char);
    if *s1 != *s2 {
        return *s1 as c_int - *s2 as c_int;
    }
    strcmp(s1, s2)
}

/// Discard each backslash-newline and any following white space, in place.
/// Backslash-backslash-newline pairs become backslash-newlines.
/// # Safety
/// `line` must be a valid, writable NUL-terminated string.
pub unsafe fn collapse_continuations(line: *mut c_char) {
    let mut out: *mut c_char = line;
    let mut in_0: *mut c_char = line;

    let mut q = strchr(in_0, '\n' as i32);
    if q.is_null() {
        return;
    }

    loop {
        let p: *mut c_char = q;
        // Count the preceding backslashes: i becomes -(their count).
        let mut i: c_int;
        if q > line && *q.sub(1) as c_int == '\\' as i32 {
            i = -2;
            while p.offset(i as isize) >= line && *p.offset(i as isize) as c_int == '\\' as i32 {
                i -= 1;
            }
            i += 1;
        } else {
            i = 0;
        }

        // Output the line up to the newline, halving any run of backslashes.
        let out_line_length = (p.offset_from(in_0) + i as isize - (i / 2) as isize) as size_t;
        if out != in_0 {
            memmove(out as *mut c_void, in_0 as *const c_void, out_line_length);
        }
        out = out.add(out_line_length);
        in_0 = q.add(1);

        if i & 1 != 0 {
            // Backslash/newline handling: skip the newline and any leading
            // whitespace on the next line.
            while stop_set(*in_0, MAP_BLANK) {
                in_0 = in_0.add(1);
            }

            // If the newline is preceded by an odd number of '$'s, it was
            // escaped: $\n turns into nothing rather than a space.
            let mut dp: *const c_char = out;
            while dp > line as *const c_char && *dp.sub(1) as c_int == '$' as i32 {
                dp = dp.sub(1);
            }
            let dollar = out.offset_from(dp) % 2 != 0;
            if dollar {
                out = out.sub(1);
            }

            // Unless in POSIX mode, also collapse preceding whitespace.
            if posix_pedantic == 0 {
                while out > line && stop_set(*out.sub(1), MAP_BLANK) {
                    out = out.sub(1);
                }
            }

            if !dollar {
                *out = ' ' as c_char;
                out = out.add(1);
            }
        } else {
            // The newline was not escaped: keep it.
            *out = '\n' as c_char;
            out = out.add(1);
        }

        q = strchr(in_0, '\n' as i32);
        if q.is_null() {
            break;
        }
    }

    memmove(out as *mut c_void, in_0 as *const c_void, strlen(in_0) + 1);
}

/// Write `n` spaces to stdout.
/// # Safety
/// Always safe; unsafe only for C-API signature compatibility.
pub unsafe fn print_spaces(n: c_uint) {
    for _ in 0..n {
        putchar(' ' as i32);
    }
}

/// Concatenate `num` strings into a static (reused, growing) buffer and
/// return it. Null arguments count as empty strings.
///
/// # Safety
/// Each of the `num` variadic arguments must be null or a valid
/// NUL-terminated string. Not reentrant: the returned buffer is shared
/// between calls.
#[no_mangle]
pub unsafe extern "C" fn concat(mut num: c_uint, args: ...) -> *const c_char {
    static mut rlen: size_t = 0;
    static mut result: *mut c_char = null_mut();

    let mut ri: size_t = 0;
    let mut args_0 = args.clone();
    while num > 0 {
        num -= 1;
        let s: *const c_char = args_0.next_arg::<*const c_char>();
        let l: size_t = if s.is_null() { 0 } else { strlen(s) };
        if l == 0 {
            continue;
        }
        if ri + l > rlen {
            rlen = (if rlen != 0 { rlen } else { 60 } + l) * 2;
            result = xrealloc(result as *mut c_void, rlen) as *mut c_char;
        }
        memcpy(result.add(ri) as *mut c_void, s as *const c_void, l);
        ri += l;
    }

    // Get some more memory if we didn't get enough for the terminator.
    if ri == rlen {
        rlen = if rlen != 0 { rlen * 2 } else { 120 };
        result = xrealloc(result as *mut c_void, rlen) as *mut c_char;
    }
    *result.add(ri) = 0;
    result
}

/// # Safety
/// Always safe; unsafe only for C-API signature compatibility.
pub unsafe fn make_pid() -> pid_t {
    getpid() as pid_t
}

/// Like `malloc` but abort via `out_of_memory` instead of returning null.
/// # Safety
/// The caller owns the returned allocation and must free it with `free`.
pub unsafe fn xmalloc(size: size_t) -> *mut c_void {
    // malloc(0) is unpredictable; avoid it.
    let result = malloc(if size != 0 { size } else { 1 });
    if result.is_null() {
        out_of_memory();
    }
    result
}

/// Like `calloc` but abort via `out_of_memory` instead of returning null.
/// # Safety
/// The caller owns the returned allocation and must free it with `free`.
pub unsafe fn xcalloc(size: size_t) -> *mut c_void {
    let result = calloc(if size != 0 { size } else { 1 }, 1);
    if result.is_null() {
        out_of_memory();
    }
    result
}

/// Like `realloc` but abort via `out_of_memory` instead of returning null.
/// # Safety
/// `ptr` must be null or a live `malloc`-family allocation; it is consumed.
pub unsafe fn xrealloc(ptr: *mut c_void, mut size: size_t) -> *mut c_void {
    if size == 0 {
        size = 1;
    }
    let result = if !ptr.is_null() {
        realloc(ptr, size)
    } else {
        malloc(size)
    };
    if result.is_null() {
        out_of_memory();
    }
    result
}

/// Like `strdup` but abort via `out_of_memory` instead of returning null.
/// # Safety
/// `ptr` must be a valid NUL-terminated string.
pub unsafe fn xstrdup(ptr: *const c_char) -> *mut c_char {
    let result = strdup(ptr);
    if result.is_null() {
        out_of_memory();
    }
    result
}

/// Like `strndup` but abort via `out_of_memory` instead of returning null.
/// # Safety
/// `str` must be valid for reads of up to `length` bytes.
pub unsafe fn xstrndup(str: *const c_char, length: size_t) -> *mut c_char {
    let result = strndup(str, length);
    if result.is_null() {
        out_of_memory();
    }
    result
}

/// Limited INDEX: search through the string `s` for the character `c`, but
/// don't go past `limit`. Returns null if `c` is not found before `limit`.
/// # Safety
/// `s..limit` must be a valid readable range.
pub unsafe fn lindex(mut s: *const c_char, limit: *const c_char, c: c_int) -> *mut c_char {
    while s < limit {
        if *s as c_int == c {
            return s as *mut c_char;
        }
        s = s.add(1);
    }
    null_mut()
}

/// Return the address of the first whitespace, NUL, or newline in `s`.
/// # Safety
/// `s` must be a valid NUL-terminated string.
pub unsafe fn end_of_token(mut s: *const c_char) -> *mut c_char {
    while !stop_set(*s, MAP_SPACE | MAP_NUL) {
        s = s.add(1);
    }
    s as *mut c_char
}

/// Return the address of the first nonwhitespace character in `s`.
/// # Safety
/// `s` must be a valid NUL-terminated string.
pub unsafe fn next_token(mut s: *const c_char) -> *mut c_char {
    while stop_set(*s, MAP_SPACE) {
        s = s.add(1);
    }
    s as *mut c_char
}

/// `p` points to the character after a `$` that introduces a variable
/// reference; return the address of the first character after the reference,
/// honoring nested parentheses or braces.
/// # Safety
/// `p` must be a valid NUL-terminated string.
pub unsafe fn skip_reference(mut p: *const c_char) -> *mut c_char {
    let openparen: c_char = *p;
    let mut count: c_int = 1;

    if openparen == 0 {
        return p as *mut c_char;
    }
    let closeparen: c_char = if openparen as c_int == '(' as i32 {
        ')' as c_char
    } else if openparen as c_int == '{' as i32 {
        '}' as c_char
    } else {
        // Single-character reference like $X.
        return p.add(1) as *mut c_char;
    };

    loop {
        p = p.add(1);
        // MAP_VARSEP marks ()/{} characters; skip everything else quickly.
        if !stop_set(*p, MAP_NUL | MAP_VARSEP) {
            continue;
        }
        if *p == 0 {
            break;
        }
        if *p == openparen {
            count += 1;
        } else if *p == closeparen {
            count -= 1;
            if count == 0 {
                p = p.add(1);
                break;
            }
        }
    }
    p as *mut c_char
}

/// Find the next token in `*ptr`, advancing `*ptr` past it. Returns the
/// token's address (and its length in `*lengthptr` when non-null), or null
/// when no token remains.
/// # Safety
/// `*ptr` must be a valid NUL-terminated string; `lengthptr` must be null or
/// valid for writes.
pub unsafe fn find_next_token(ptr: *mut *const c_char, lengthptr: *mut size_t) -> *mut c_char {
    let p: *const c_char = next_token(*ptr);
    if *p == 0 {
        return null_mut();
    }
    *ptr = end_of_token(p);
    if !lengthptr.is_null() {
        *lengthptr = (*ptr).offset_from(p) as size_t;
    }
    p as *mut c_char
}

/// Write `len` bytes from `buffer` to `fd`, retrying on EINTR and short
/// writes. Returns `len` on success or -1 on failure.
/// # Safety
/// `buffer` must be valid for reads of `len` bytes; `fd` must be open.
pub unsafe fn writebuf(fd: c_int, buffer: *const c_void, len: size_t) -> ssize_t {
    let mut msg: *const c_char = buffer as *const c_char;
    let mut l = len;
    while l != 0 {
        let mut r: ssize_t;
        loop {
            r = write(fd, msg as *const c_void, l);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            return -1;
        }
        l -= r as size_t;
        msg = msg.add(r as usize);
    }
    len as ssize_t
}

/// Read up to `len` bytes from `fd` into `buffer`, retrying on EINTR and
/// short reads. Returns the number of bytes read, or -1 on failure.
/// # Safety
/// `buffer` must be valid for writes of `len` bytes; `fd` must be open.
pub unsafe fn readbuf(fd: c_int, buffer: *mut c_void, mut len: size_t) -> ssize_t {
    let mut msg: *mut c_char = buffer as *mut c_char;
    while len != 0 {
        let mut r: ssize_t;
        loop {
            r = read(fd, msg as *mut c_void, len);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            return -1;
        }
        if r == 0 {
            break;
        }
        len -= r as size_t;
        msg = msg.add(r as usize);
    }
    msg.offset_from(buffer as *mut c_char) as ssize_t
}

/// Copy a single `Dep` node (not following `next`), duplicating its name if
/// it still needs second expansion (otherwise the cached name is shared).
/// # Safety
/// `d` must be null or point to a valid `Dep` with a cached or owned name.
pub unsafe fn copy_dep(d: *const Dep) -> *mut Dep {
    if d.is_null() {
        return null_mut();
    }
    let new = xmalloc(::core::mem::size_of::<Dep>()) as *mut Dep;
    memcpy(
        new as *mut c_void,
        d as *const c_void,
        ::core::mem::size_of::<Dep>(),
    );
    if (*new).need_2nd_expansion() != 0 {
        (*new).name = xstrdup((*new).name);
    }
    (*new).next = null_mut();
    new
}

/// Copy an entire `Dep` chain.
/// # Safety
/// `d` must be null or the head of a valid `Dep` chain.
pub unsafe fn copy_dep_chain(mut d: *const Dep) -> *mut Dep {
    let mut firstnew: *mut Dep = null_mut();
    let mut lastnew: *mut Dep = null_mut();
    while let Some(dn) = d.as_ref() {
        let c = copy_dep(d);
        if let Some(ln) = lastnew.as_mut() {
            ln.next = c;
            lastnew = c;
        } else {
            lastnew = c;
            firstnew = lastnew;
        }
        d = dn.next;
    }
    firstnew
}

/// Free a chain of `nameseq` structures (the names themselves are cached and
/// not freed).
/// # Safety
/// `ns` must be null or an `xmalloc`-owned chain; the nodes are freed.
pub unsafe fn free_ns_chain(mut ns: *mut nameseq) {
    while let Some(node) = ns.as_ref() {
        let t = ns;
        ns = node.next;
        free_ns(t);
    }
}

/// Debugging aid: while a file `.make-spin-<type>` exists, sleep in a loop.
/// Lets a developer attach a debugger to this process.
///
/// # Safety
/// `type_0` must be a valid NUL-terminated string.
#[no_mangle]
pub unsafe fn spin(type_0: *const c_char) {
    let mut filenm: [c_char; 256] = [0; 256];
    let mut dummy: stat = ::core::mem::zeroed();

    sprintf(filenm.as_mut_ptr(), c".make-spin-%s".as_ptr(), type_0);
    if stat(filenm.as_ptr(), &mut dummy) == 0 {
        fprintf(stderr, c"SPIN on %s\n".as_ptr(), filenm.as_ptr());
        loop {
            sleep(1);
            if stat(filenm.as_ptr(), &mut dummy) != 0 {
                break;
            }
        }
    }
}

/// Debugging aid: append a printf-formatted line, tagged with the PID, to
/// `/tmp/gmkdebug.log`.
///
/// # Safety
/// `fmt` and the variadic arguments must form a valid printf invocation
/// producing less than 4096 bytes.
#[no_mangle]
pub unsafe extern "C" fn dbg(fmt: *const c_char, args: ...) {
    let fp: *mut FILE = fopen(c"/tmp/gmkdebug.log".as_ptr(), c"a+".as_ptr());
    let mut buf: [c_char; 4096] = [0; 4096];
    let args_0 = args.clone();
    vsprintf(buf.as_mut_ptr(), fmt, args_0);
    fprintf(fp, c"%u: %s\n".as_ptr(), make_pid() as c_uint, buf.as_ptr());
    fflush(fp);
    fclose(fp);
}

const DEFAULT_TMPDIR: &::core::ffi::CStr = c"/tmp";
const DEFAULT_TMPFILE: &::core::ffi::CStr = c"GmXXXXXX";

/// Return the directory for temporary files: `$MAKE_TMPDIR`, `$TMPDIR`, or
/// the default, in that order, warning about set-but-unusable values. The
/// result is computed once and cached.
///
/// # Safety
/// Must run single-threaded: it caches its result in a static and reads the
/// environment.
#[no_mangle]
pub unsafe fn get_tmpdir() -> *const c_char {
    static mut tmpdir: *const c_char = null();

    if tmpdir.is_null() {
        let mut found = false;
        for var in [c"MAKE_TMPDIR", c"TMPDIR"] {
            tmpdir = getenv(var.as_ptr());
            if tmpdir.is_null() || *tmpdir == 0 {
                continue;
            }
            found = true;

            let mut st: stat = ::core::mem::zeroed();
            let mut r: c_int;
            loop {
                r = stat(tmpdir, &mut st);
                if !(r == -1 && *__errno_location() == EINTR) {
                    break;
                }
            }
            if r < 0 {
                error(
                    null::<Floc>(),
                    var.count_bytes() + strlen(tmpdir) + strlen(strerror(*__errno_location())),
                    c"%s value %s: %s".as_ptr(),
                    var.as_ptr(),
                    tmpdir,
                    strerror(*__errno_location()),
                );
            } else if st.st_mode & S_IFMT != S_IFDIR {
                error(
                    null::<Floc>(),
                    var.count_bytes() + strlen(tmpdir),
                    c"%s value %s: not a directory".as_ptr(),
                    var.as_ptr(),
                    tmpdir,
                );
            } else {
                return tmpdir;
            }
        }

        tmpdir = DEFAULT_TMPDIR.as_ptr();
        if found {
            error(
                null::<Floc>(),
                strlen(tmpdir),
                c"using default temporary directory '%s'".as_ptr(),
                tmpdir,
            );
        }
    }
    tmpdir
}

/// Build an `xmalloc`'d mkstemp template `<tmpdir>/GmXXXXXX`.
/// # Safety
/// Must run single-threaded (uses `get_tmpdir`); caller frees the result.
pub unsafe fn get_tmptemplate() -> *mut c_char {
    let tmpdir = get_tmpdir();

    let template = xmalloc(strlen(tmpdir) + DEFAULT_TMPFILE.to_bytes().len() + 2) as *mut c_char;
    let mut cp = stpcpy(template, tmpdir);
    if !stop_set(*cp.sub(1), MAP_DIRSEP) {
        *cp = '/' as c_char;
        cp = cp.add(1);
    }
    strcpy(cp, DEFAULT_TMPFILE.as_ptr());
    template
}

/// Create a temporary file and return its file descriptor, or -1 on failure
/// (after reporting an error). If `name` is null the file is anonymous
/// (unlinked immediately, or created with `os_anontmp`); otherwise `*name`
/// receives the `xmalloc`'d file name.
///
/// # Safety
/// `name` must be null or valid for writes; the caller takes ownership of
/// `*name`.
pub unsafe fn get_tmpfd(name: *mut *mut c_char) -> c_int {
    let mut fd: c_int;

    if !name.is_null() {
        *name = null_mut();
    } else {
        // If there's an OS-specific way to get an anonymous temp file, use it.
        fd = os_anontmp();
        if fd >= 0 {
            return fd;
        }
    }

    // Make sure the temporary file is never readable by other users.
    let mask: mode_t = umask(0o77);
    let tmpnm = get_tmptemplate();
    loop {
        fd = mkstemp(tmpnm);
        if !(fd == -1 && *__errno_location() == EINTR) {
            break;
        }
    }

    if fd < 0 {
        error(
            null::<Floc>(),
            strlen(tmpnm) + strlen(strerror(*__errno_location())),
            c"cannot create temporary file %s: %s".as_ptr(),
            tmpnm,
            strerror(*__errno_location()),
        );
        free(tmpnm as *mut c_void);
        return -1;
    }

    if !name.is_null() {
        *name = tmpnm;
    } else {
        let mut r: c_int;
        loop {
            r = unlink(tmpnm);
            if !(r == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            error(
                null::<Floc>(),
                strlen(tmpnm) + strlen(strerror(*__errno_location())),
                c"cannot unlink temporary file %s: %s".as_ptr(),
                tmpnm,
                strerror(*__errno_location()),
            );
        }
        free(tmpnm as *mut c_void);
    }

    umask(mask);
    fd
}

/// Create a temporary `FILE *` opened `"wb+"`; `*name` receives the
/// `xmalloc`'d file name. Returns null on failure (after reporting an
/// error).
///
/// # Safety
/// `name` must be non-null and valid for writes; the caller takes ownership
/// of `*name`.
pub unsafe fn get_tmpfile(name: *mut *mut c_char) -> *mut FILE {
    const ASSERT_FUNCTION: &::core::ffi::CStr = c"FILE *get_tmpfile(char **)";
    let tmpfile_mode: *const c_char = c"wb+".as_ptr();

    if name.is_null() {
        __assert_fail(
            c"name".as_ptr(),
            c"src/misc.c".as_ptr(),
            827,
            ASSERT_FUNCTION.as_ptr(),
        );
    }

    let fd = get_tmpfd(name);
    if fd < 0 {
        return null_mut();
    }

    if (*name).is_null() {
        __assert_fail(
            c"*name".as_ptr(),
            c"src/misc.c".as_ptr(),
            831,
            ASSERT_FUNCTION.as_ptr(),
        );
    }

    let mut file: *mut FILE;
    loop {
        *__errno_location() = 0;
        file = fdopen(fd, tmpfile_mode);
        if !(file.is_null() && *__errno_location() == EINTR) {
            break;
        }
    }
    if file.is_null() {
        error(
            null::<Floc>(),
            strlen(*name) + strlen(strerror(*__errno_location())),
            c"fdopen: temporary file %s: %s".as_ptr(),
            *name,
            strerror(*__errno_location()),
        );
    }
    file
}
