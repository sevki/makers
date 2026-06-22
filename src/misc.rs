//! Miscellaneous generally-useful functions: checked allocation, token
//! scanning, temporary files, and small string helpers.
//!
//! Port of `misc.c`. Most helpers still traffic in raw C strings because
//! their callers (the makefile reader, the variable expander, the job
//! runner) are still C-shaped.

use ::core::ffi::{c_char, c_longlong, c_uint, c_ulonglong, c_void};
use ::core::ptr::{null, null_mut};

use std::sync::atomic::{AtomicU32, Ordering};

use libc::{
    __errno_location, calloc, free, getenv, getpid, malloc, memcpy, mkstemp, putchar, read,
    realloc, sleep, sprintf, stpcpy, strcmp, strcpy, strdup, strerror, strlen, strndup, umask,
    unlink, write, EINTR,
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
    fn stat(file: *const c_char, buf: *mut stat) -> i32;
    static mut stderr: *mut FILE;
    fn fclose(stream: *mut FILE) -> i32;
    fn fflush(stream: *mut FILE) -> i32;
    fn fopen(filename: *const c_char, modes: *const c_char) -> *mut FILE;
    fn fdopen(fd: i32, modes: *const c_char) -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> i32;
    fn vsprintf(s: *mut c_char, format: *const c_char, arg: ::core::ffi::VaList) -> i32;
    fn time(timer: *mut time_t) -> time_t;
}

/// Character-class bits in `stopchar_map` (see `makeint.h`).
const MAP_NUL: i32 = 0x0001;
const MAP_BLANK: i32 = 0x0002;
const MAP_NEWLINE: i32 = 0x0004;
const MAP_VARSEP: i32 = 0x0080;
const MAP_DIRSEP: i32 = 0x8000;
const MAP_SPACE: i32 = MAP_BLANK | MAP_NEWLINE;

/// `STOP_SET (c, mask)` from `makeint.h`: is `c` in any of the character
/// classes selected by `mask`?
fn stop_set(c: c_char, mask: i32) -> bool {
    stopchar_map()[c as u8 as usize] as i32 & mask != 0
}

/// File-type test from `S_ISDIR`: `(mode & S_IFMT) == S_IFDIR`.
const S_IFMT: __mode_t = 0o170000;
const S_IFDIR: __mode_t = 0o040000;

#[inline]
unsafe fn free_ns(n: *mut nameseq) {
    free(n as *mut c_void);
}

/// Why [`make_toui`] rejected its input, mirroring C `make_toui`'s two error
/// messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MakeToUiError {
    /// The string was empty (C "Missing value").
    Missing,
    /// No digits, or trailing non-digit characters (C "Invalid value").
    Invalid,
}

/// Parse the leading base-10 unsigned integer of `bytes` with C `strtoul`
/// semantics: skip leading whitespace and an optional `+`/`-` sign, then take
/// the run of decimal digits. Returns `Err(Missing)` for empty input and
/// `Err(Invalid)` when there are no digits or any trailing non-digit characters
/// remain (the C "Missing value"/"Invalid value" cases), else `Ok(value)`.
///
/// On this target `c_ulong` is `u64`, so the magnitude is accumulated in `u64`
/// and saturated exactly as `strtoul` clamps to `ULONG_MAX`, then truncated to
/// `u32` like the `(unsigned int)` cast in C `make_toui`; a `-` sign wraps
/// through that cast the same way.
pub(crate) fn parse_uint_strtoul(bytes: &[u8]) -> Result<u32, MakeToUiError> {
    if bytes.is_empty() {
        return Err(MakeToUiError::Missing);
    }
    // C-locale isspace, which `strtoul` skips: space, \t, \n, \v, \f, \r.
    const WS: &[u8] = b" \t\n\x0b\x0c\r";
    let mut rest = bytes;
    while let [c, tail @ ..] = rest {
        if WS.contains(c) {
            rest = tail;
        } else {
            break;
        }
    }
    let negate = matches!(rest.first(), Some(b'-'));
    if matches!(rest.first(), Some(b'+' | b'-')) {
        rest = &rest[1..];
    }
    let ndigits = rest.iter().take_while(|b| b.is_ascii_digit()).count();
    if ndigits == 0 || ndigits != rest.len() {
        return Err(MakeToUiError::Invalid);
    }
    let mut mag: u64 = 0;
    for &d in &rest[..ndigits] {
        mag = mag.saturating_mul(10).saturating_add(u64::from(d - b'0'));
    }
    Ok(if negate { 0u64.wrapping_sub(mag) } else { mag } as u32)
}

/// Safe port of make's `make_toui`: parse `s` as a base-10 unsigned integer,
/// returning `Err` for the C "Missing value"/"Invalid value" cases. See
/// [`parse_uint_strtoul`] for the exact `strtoul`-matching semantics.
pub fn make_toui(s: &::core::ffi::CStr) -> Result<u32, MakeToUiError> {
    parse_uint_strtoul(s.to_bytes())
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
pub fn make_seed(seed: c_uint) {
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
pub unsafe extern "C" fn alpha_compare(v1: *const c_void, v2: *const c_void) -> i32 {
    let s1: *const c_char = *(v1 as *mut *mut c_char);
    let s2: *const c_char = *(v2 as *mut *mut c_char);
    if *s1 != *s2 {
        return *s1 as i32 - *s2 as i32;
    }
    strcmp(s1, s2)
}

/// Collapse backslash-newline continuations in `buf` in place, returning the
/// new length. `is_blank` classifies `MAP_BLANK` bytes (space/tab) and
/// `posix_pedantic` selects POSIX whitespace handling.
///
/// Pure port of [`collapse_continuations`]: the write cursor `out` never
/// overtakes the read cursor `in_0`, so the in-place rewrite — and the backward
/// scans over the already-written output — are sound. Mirrors the C exactly,
/// including the trailing-backslash halving (a run is shortened by copying
/// fewer bytes), the odd-`$` escape (`$\n` collapses to nothing), and the
/// non-POSIX trimming of whitespace before a folded newline.
fn collapse_continuations_bytes(
    buf: &mut [u8],
    posix: bool,
    is_blank: impl Fn(u8) -> bool,
) -> usize {
    let mut out = 0usize;
    let mut in_0 = 0usize;
    // First newline, or leave the line untouched when there is none.
    let mut q = match buf.iter().position(|&c| c == b'\n') {
        Some(i) => i,
        None => return buf.len(),
    };
    loop {
        let p = q;
        // Count the preceding backslashes: `i` ends as 1 - (their count).
        let mut i: i32;
        if p > 0 && buf[p - 1] == b'\\' {
            i = -2;
            while p as isize + i as isize >= 0 && buf[(p as isize + i as isize) as usize] == b'\\' {
                i -= 1;
            }
            i += 1;
        } else {
            i = 0;
        }

        // Output up to the newline, halving any trailing backslash run.
        let out_line_length = (p as isize - in_0 as isize + i as isize - (i / 2) as isize) as usize;
        if out != in_0 {
            buf.copy_within(in_0..in_0 + out_line_length, out);
        }
        out += out_line_length;
        in_0 = q + 1;

        if i & 1 != 0 {
            // Escaped newline: skip it and any leading whitespace on the next
            // line.
            while in_0 < buf.len() && is_blank(buf[in_0]) {
                in_0 += 1;
            }

            // A newline preceded by an odd number of '$'s is escaped: `$\n`
            // turns into nothing rather than a space.
            let mut dp = out;
            while dp > 0 && buf[dp - 1] == b'$' {
                dp -= 1;
            }
            let dollar = !(out - dp).is_multiple_of(2);
            if dollar {
                out -= 1;
            }

            // Unless in POSIX mode, also collapse preceding whitespace.
            if !posix {
                while out > 0 && is_blank(buf[out - 1]) {
                    out -= 1;
                }
            }

            if !dollar {
                buf[out] = b' ';
                out += 1;
            }
        } else {
            // The newline was not escaped: keep it.
            buf[out] = b'\n';
            out += 1;
        }

        match buf[in_0..].iter().position(|&c| c == b'\n') {
            Some(rel) => q = in_0 + rel,
            None => break,
        }
    }

    // Copy the remaining tail down.
    let tail = buf.len() - in_0;
    if out != in_0 {
        buf.copy_within(in_0..buf.len(), out);
    }
    out + tail
}

/// Discard each backslash-newline and any following white space, in place.
/// Backslash-backslash-newline pairs become backslash-newlines.
/// # Safety
/// `line` must be a valid, writable NUL-terminated string.
pub unsafe fn collapse_continuations(line: *mut c_char) {
    let len = strlen(line);
    // Include the existing NUL slot so the new terminator is written by
    // indexing rather than raw pointer arithmetic.
    let buf = ::core::slice::from_raw_parts_mut(line as *mut u8, len + 1);
    let new_len = collapse_continuations_bytes(&mut buf[..len], posix_pedantic(), |c| {
        stop_set(c as c_char, MAP_BLANK)
    });
    buf[new_len] = 0;
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

/// Limited INDEX: search the byte slice `hay` for the first occurrence of the
/// byte `c`, returning its index, or `None` if absent.
///
/// `hay` is the original `s..limit` range; `Option<usize>` replaces the
/// original pointer/null result. An empty slice yields `None`, which also
/// removes the null/empty-range hazard the old pointer signature carried.
pub fn lindex(hay: &[u8], c: u8) -> Option<usize> {
    hay.iter().position(|&b| b == c)
}

/// Return the offset within `s` of the first whitespace byte (make's
/// `MAP_SPACE` class). `s` should be the bytes of a NUL-terminated string
/// *without* the trailing NUL (e.g. via `&buf[..strlen]`); the end of the
/// slice plays the role of the NUL terminator, so a token that runs to the
/// end of `s` yields `s.len()`. This is the offset form of make's
/// `end_of_token`, which returns the address of the first whitespace-or-NUL.
pub fn end_of_token(s: &[u8]) -> usize {
    s.iter()
        .position(|&b| stop_set(b as c_char, MAP_SPACE))
        .unwrap_or(s.len())
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

/// `bytes` starts at the character after a `$` that introduces a variable
/// reference; return the number of bytes consumed, i.e. the offset of the first
/// character after the `$(...)`/`${...}`/`$X` reference, honoring nested
/// parentheses or braces. `bytes` must contain the terminating NUL slot.
pub fn skip_reference(bytes: &[u8]) -> usize {
    let openparen = bytes[0] as c_char;
    let mut count: i32 = 1;

    if openparen == 0 {
        return 0;
    }
    let closeparen: c_char = if openparen as i32 == '(' as i32 {
        ')' as c_char
    } else if openparen as i32 == '{' as i32 {
        '}' as c_char
    } else {
        // Single-character reference like $X.
        return 1;
    };

    let mut p = 0usize;
    loop {
        p += 1;
        let c = bytes[p] as c_char;
        // MAP_VARSEP marks ()/{} characters; skip everything else quickly.
        if !stop_set(c, MAP_NUL | MAP_VARSEP) {
            continue;
        }
        if c == 0 {
            break;
        }
        if c == openparen {
            count += 1;
        } else if c == closeparen {
            count -= 1;
            if count == 0 {
                p += 1;
                break;
            }
        }
    }
    p
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
    // `find_next_token` is called once per token over a whitespace-separated
    // list, so it must NOT measure the whole remaining suffix (e.g. via
    // `strlen` + `end_of_token`) per call — that would regress tokenization
    // from O(n) to O(n^2). Instead, scan only the current token with a
    // pointer-based walk that stops at the first blank/NUL, exactly as the
    // original inline `end_of_token` did.
    let end = end_of_token_raw(p);
    *ptr = end;
    if !lengthptr.is_null() {
        *lengthptr = end.offset_from(p) as size_t;
    }
    p as *mut c_char
}

/// Walk `p` to the first whitespace-or-NUL byte and return its address. This
/// is the pointer-based equivalent of the public safe `end_of_token`, kept
/// private and used ONLY by `find_next_token`'s per-token hot path so that
/// tokenizing a whitespace-separated list stays O(n) (no `strlen` over the
/// whole suffix per token).
/// # Safety
/// `p` must be a valid NUL-terminated string.
unsafe fn end_of_token_raw(mut p: *const c_char) -> *mut c_char {
    while !stop_set(*p, MAP_SPACE | MAP_NUL) {
        p = p.add(1);
    }
    p as *mut c_char
}

/// Write `len` bytes from `buffer` to `fd`, retrying on EINTR and short
/// writes. Returns `len` on success or -1 on failure.
/// # Safety
/// `buffer` must be valid for reads of `len` bytes; `fd` must be open.
pub unsafe fn writebuf(fd: i32, buffer: *const c_void, len: size_t) -> ssize_t {
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
pub unsafe fn readbuf(fd: i32, buffer: *mut c_void, mut len: size_t) -> ssize_t {
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
            let mut r: i32;
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
pub unsafe fn get_tmpfd(name: *mut *mut c_char) -> i32 {
    let mut fd: i32;

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
        let mut r: i32;
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
    let tmpfile_mode: *const c_char = c"wb+".as_ptr();

    let name = name.as_mut().expect("get_tmpfile: name must be non-null");

    let fd = get_tmpfd(name);
    if fd < 0 {
        return null_mut();
    }

    assert!(
        !name.is_null(),
        "get_tmpfile: temporary file name must be set"
    );

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

#[cfg(test)]
mod make_toui_tests {
    use super::{make_toui, MakeToUiError};

    #[test]
    fn parses_plain_decimals() {
        assert_eq!(make_toui(c"0"), Ok(0));
        assert_eq!(make_toui(c"42"), Ok(42));
        assert_eq!(make_toui(c"007"), Ok(7));
        assert_eq!(make_toui(c"4294967295"), Ok(u32::MAX));
    }

    #[test]
    fn skips_leading_ws_and_sign() {
        assert_eq!(make_toui(c"  12"), Ok(12));
        assert_eq!(make_toui(c"\t\n5"), Ok(5));
        assert_eq!(make_toui(c"+5"), Ok(5));
        // strtoul negates into the unsigned range, then the cast truncates.
        assert_eq!(make_toui(c"-1"), Ok(u32::MAX));
    }

    #[test]
    fn reports_missing_and_invalid() {
        assert_eq!(make_toui(c""), Err(MakeToUiError::Missing)); // empty
        assert_eq!(make_toui(c"abc"), Err(MakeToUiError::Invalid)); // no digits
        assert_eq!(make_toui(c"+"), Err(MakeToUiError::Invalid)); // lone sign
        assert_eq!(make_toui(c"-"), Err(MakeToUiError::Invalid));
        assert_eq!(make_toui(c"12abc"), Err(MakeToUiError::Invalid)); // trailing junk
        assert_eq!(make_toui(c"12 "), Err(MakeToUiError::Invalid)); // trailing ws kept
        assert_eq!(make_toui(c"   "), Err(MakeToUiError::Invalid)); // ws-only is not Missing
    }

    #[test]
    fn overflow_saturates_then_truncates() {
        // On this (64-bit) target `c_ulong` is `u64`: strtoul accumulates in
        // u64 and the `(unsigned int)` cast truncates, so a value that is valid
        // in u64 but overflows u32 wraps rather than erroring.
        assert_eq!(make_toui(c"4294967296"), Ok(0)); // u32::MAX + 1 -> 0
                                                     // True u64 overflow saturates to ULONG_MAX, whose low 32 bits are all-ones.
        assert_eq!(make_toui(c"99999999999999999999999"), Ok(u32::MAX));
    }
}

#[cfg(test)]
mod alpha_compare_tests {
    use super::alpha_compare;
    use ::core::ffi::{c_char, c_void, CStr};

    // `alpha_compare` takes `const void *` arguments that each point to a
    // `char *` (the qsort element type), so pass the address of a string
    // pointer.
    unsafe fn cmp(a: &CStr, b: &CStr) -> i32 {
        let pa: *const c_char = a.as_ptr();
        let pb: *const c_char = b.as_ptr();
        alpha_compare(
            (&pa as *const *const c_char).cast::<c_void>(),
            (&pb as *const *const c_char).cast::<c_void>(),
        )
    }

    #[test]
    fn equal_first_byte_falls_back_to_strcmp() {
        unsafe {
            assert!(cmp(c"abc", c"abd") < 0);
            assert_eq!(cmp(c"abc", c"abc"), 0);
            assert!(cmp(c"abd", c"abc") > 0);
            // A NUL (end of the shorter string) sorts before any byte.
            assert!(cmp(c"ab", c"abc") < 0);
        }
    }

    #[test]
    fn differing_first_byte_orders_by_that_byte() {
        unsafe {
            // 'B' (66) sorts before 'a' (97): the first-byte fast path.
            assert!(cmp(c"B", c"a") < 0);
            assert!(cmp(c"a", c"B") > 0);
        }
    }
}

#[cfg(test)]
mod collapse_continuations_tests {
    use super::collapse_continuations_bytes;

    /// Collapse a copy of `s` (MAP_BLANK = space/tab) and return the result.
    fn collapse(s: &[u8], posix: bool) -> Vec<u8> {
        let mut buf = s.to_vec();
        let n = collapse_continuations_bytes(&mut buf, posix, |c| c == b' ' || c == b'\t');
        buf.truncate(n);
        buf
    }

    #[test]
    fn no_newline_is_unchanged() {
        assert_eq!(collapse(b"abc", false), b"abc");
        assert_eq!(collapse(b"", false), b"");
    }

    #[test]
    fn unescaped_newline_is_kept() {
        assert_eq!(collapse(b"a\nb", false), b"a\nb");
    }

    #[test]
    fn backslash_newline_folds_to_space() {
        assert_eq!(collapse(b"a\\\nb", false), b"a b");
        // Leading whitespace on the continued line is dropped.
        assert_eq!(collapse(b"a\\\n\t b", false), b"a b");
        // Multiple continuations in a row.
        assert_eq!(collapse(b"a\\\nb\\\nc", false), b"a b c");
    }

    #[test]
    fn even_backslash_run_keeps_the_newline() {
        // `\\` is an escaped backslash, so the newline is literal: one
        // backslash survives and the newline is kept.
        assert_eq!(collapse(b"a\\\\\nb", false), b"a\\\nb");
    }

    #[test]
    fn whitespace_before_fold_trimmed_only_outside_posix() {
        // Non-POSIX collapses the space preceding the fold into the single
        // separator; POSIX keeps it (original space + folded space).
        assert_eq!(collapse(b"a \\\nb", false), b"a b");
        assert_eq!(collapse(b"a \\\nb", true), b"a  b");
    }

    #[test]
    fn odd_dollar_escapes_the_continuation_to_nothing() {
        // A '$' immediately before the backslash-newline (odd count) escapes
        // it: both the '$' and the continuation vanish.
        assert_eq!(collapse(b"a$\\\nb", false), b"ab");
    }
}

#[cfg(test)]
mod next_token_tests {
    use super::next_token;
    use crate::make_main::initialize_stopchar_map;
    use std::ffi::{c_char, CStr, CString};

    /// `next_token` advances past leading blanks/newlines and returns the first
    /// non-whitespace character (or the terminating NUL for all-blank input).
    #[test]
    fn skips_leading_whitespace() {
        unsafe {
            initialize_stopchar_map();

            // Leading spaces and a tab are skipped to the first real char.
            let s = CString::new("  \t foo").unwrap();
            let p = next_token(s.as_ptr());
            assert_eq!(CStr::from_ptr(p).to_bytes(), b"foo");

            // No leading whitespace: returns the start unchanged.
            let s2 = CString::new("bar").unwrap();
            let p2 = next_token(s2.as_ptr());
            assert_eq!(p2 as *const c_char, s2.as_ptr());

            // All whitespace: lands on the terminating NUL (empty token).
            let s3 = CString::new("   ").unwrap();
            let p3 = next_token(s3.as_ptr());
            assert_eq!(*p3, 0);
        }
    }
}

#[cfg(test)]
mod lindex_unsafe_oracle {
    use super::lindex;
    use ::core::ffi::c_char;
    use ::core::ptr::null_mut;

    /// Verbatim copy of the original c2rust-derived `lindex`, preserved as a
    /// behavioral oracle for the safe slice-based rewrite.
    ///
    /// # Safety
    /// `s..limit` must be a valid readable range.
    unsafe fn lindex_oracle(mut s: *const c_char, limit: *const c_char, c: i32) -> *mut c_char {
        while s < limit {
            if matches!(s.as_ref(), Some(&b) if b as i32 == c) {
                return s as *mut c_char;
            }
            s = s.add(1);
        }
        null_mut()
    }

    /// Drive representative inputs through both the safe `lindex` and the
    /// preserved unsafe oracle, asserting they agree. The safe function's
    /// `Option<usize>` is mapped back to the pointer the oracle returns:
    /// `Some(i)` -> `s.add(i)`, `None` -> null.
    ///
    /// The safe API searches for a `u8`; the oracle compares the (signed)
    /// `c_char` sign-extended to `i32`. To exercise that boundary faithfully
    /// we feed the oracle `(c as i8) as i32`, which is exactly the value its
    /// `b as i32` comparison produces for byte `b == c` (this is what proves
    /// sign-extension parity for the high byte `0xff`).
    #[test]
    fn matches_oracle() {
        let cases: &[(&[u8], u8)] = &[
            (b"hello", b'l'),    // first match mid-string
            (b"hello", b'h'),    // match at start
            (b"hello", b'o'),    // match at last byte
            (b"hello", b'z'),    // no match
            (b"", b'a'),         // empty range
            (b"a\0b", 0),        // searching for NUL, embedded
            (b"aaa", b'a'),      // returns the first of repeats
            (b"\xff\x01", 0xff), // high byte (sign-extension parity)
        ];

        for &(buf, c) in cases {
            let s = buf.as_ptr() as *const c_char;
            let c_oracle = (c as i8) as i32;
            // Exercise every prefix length so the boundary is covered.
            for len in 0..=buf.len() {
                let safe = lindex(&buf[..len], c);
                // SAFETY: `s..s.add(len)` is within `buf`'s allocation.
                let oracle = unsafe { lindex_oracle(s, s.add(len), c_oracle) };
                let safe_ptr = match safe {
                    // SAFETY: `i < len`, so `s.add(i)` is in bounds.
                    Some(i) => (unsafe { s.add(i) }) as *mut c_char,
                    None => null_mut(),
                };
                assert_eq!(safe_ptr, oracle, "mismatch buf={buf:?} c={c} len={len}");
            }
        }
    }
}

#[cfg(test)]
mod skip_reference_unsafe_oracle {
    use super::skip_reference;
    use ::core::ffi::c_char;

    // Re-derive the helpers the oracle needs, identical to the module ones.
    const MAP_NUL: i32 = 0x0001;
    const MAP_VARSEP: i32 = 0x0080;

    fn stop_set(c: c_char, mask: i32) -> bool {
        crate::make_main::stopchar_map()[c as u8 as usize] as i32 & mask != 0
    }

    /// Verbatim copy of the original c2rust-derived `skip_reference`, preserved
    /// as a behavioral oracle for the safe slice-based rewrite.
    ///
    /// # Safety
    /// `p` must be a valid NUL-terminated string.
    unsafe fn skip_reference_oracle(mut p: *const c_char) -> *mut c_char {
        let openparen: c_char = *p;
        let mut count: i32 = 1;

        if openparen == 0 {
            return p as *mut c_char;
        }
        let closeparen: c_char = if openparen as i32 == '(' as i32 {
            ')' as c_char
        } else if openparen as i32 == '{' as i32 {
            '}' as c_char
        } else {
            return p.add(1) as *mut c_char;
        };

        loop {
            p = p.add(1);
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

    /// Drive representative inputs (the byte just past a `$`) through both the
    /// safe `skip_reference` and the preserved unsafe oracle, asserting that
    /// they return byte-identical results.
    #[test]
    fn matches_oracle() {
        // The oracle and the safe implementation both read the global stopchar
        // map; initialize it up front so this test passes in isolation rather
        // than relying on another test having seeded the global.
        crate::make_main::initialize_stopchar_map();

        // Each case is NUL-terminated; the character at index 0 is the one
        // following the `$`.
        let cases: &[&[u8]] = &[
            b"\0",            // empty / bare `$`
            b"X\0",           // single-char reference `$X`
            b")\0",           // lone closer
            b"(foo)\0",       // `$(foo)`
            b"{foo}\0",       // `${foo}`
            b"(foo\0",        // unterminated paren ref
            b"(a$(b)c)\0",    // nested parens
            b"(a${b}c)\0",    // nested mixed braces inside parens
            b"{a$(b)c}\0",    // nested parens inside braces
            b"((()))\0",      // deep nesting
            b"(unbalanced\0", // runs to NUL
            b"(a)b)\0",       // stops at first balanced closer
            b"( )\0",         // whitespace inside
        ];

        for &buf in cases {
            // Run the safe API and map its offset back to the pointer the
            // oracle returns, then compare against the unsafe oracle directly.
            let offset = skip_reference(buf);
            let safe = unsafe { buf.as_ptr().add(offset) as *mut c_char };
            let oracle = unsafe { skip_reference_oracle(buf.as_ptr() as *const c_char) };
            assert_eq!(safe, oracle, "mismatch buf={buf:?}");
        }
    }
}

#[cfg(test)]
mod end_of_token_unsafe_oracle {
    use super::{end_of_token, MAP_NUL, MAP_SPACE};
    use ::core::ffi::c_char;

    fn stop_set(c: c_char, mask: i32) -> bool {
        crate::make_main::stopchar_map()[c as u8 as usize] as i32 & mask != 0
    }

    /// Verbatim copy of the original c2rust-derived `end_of_token`, preserved
    /// as a behavioral oracle for the safe offset-based rewrite.
    ///
    /// # Safety
    /// `s` must be a valid NUL-terminated string.
    unsafe fn end_of_token_oracle(mut s: *const c_char) -> *mut c_char {
        while !stop_set(*s, MAP_SPACE | MAP_NUL) {
            s = s.add(1);
        }
        s as *mut c_char
    }

    /// Drive representative NUL-terminated inputs through both the safe
    /// `end_of_token` (fed the bytes up to the NUL) and the preserved unsafe
    /// oracle, asserting they agree on the token-end position. The safe API's
    /// returned offset is mapped back to the pointer the oracle returns via
    /// `s.add(offset)`.
    #[test]
    fn matches_oracle() {
        crate::make_main::initialize_stopchar_map();

        let cases: &[&[u8]] = &[
            b"\0",            // empty: token end at offset 0
            b"foo\0",         // whole string is the token
            b"foo bar\0",     // stops at the space
            b"foo\tbar\0",    // stops at the tab (MAP_BLANK)
            b"foo\nbar\0",    // stops at the newline (MAP_NEWLINE)
            b" foo\0",        // leading space: token end at offset 0
            b"a b c\0",       // stops at the first space
            b"\xff\x01 z\0",  // high bytes then a space
            b"nospacehere\0", // runs to the NUL
        ];

        for &buf in cases {
            // `buf` is NUL-terminated; feed the bytes up to (not including) the
            // NUL to the safe API, matching how the real callers slice.
            let strlen = buf.iter().position(|&b| b == 0).unwrap();
            let offset = end_of_token(&buf[..strlen]);
            // SAFETY: `offset <= strlen`, so `s.add(offset)` is in bounds.
            let safe = unsafe { buf.as_ptr().add(offset) as *mut c_char };
            let oracle = unsafe { end_of_token_oracle(buf.as_ptr() as *const c_char) };
            assert_eq!(safe, oracle, "mismatch buf={buf:?}");
        }
    }
}
