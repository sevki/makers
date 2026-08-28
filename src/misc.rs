//! Miscellaneous generally-useful functions: checked allocation, token
//! scanning, temporary files, and small string helpers.
//!
//! Port of `misc.c`. Most helpers still traffic in raw C strings because
//! their callers (the makefile reader, the variable expander, the job
//! runner) are still C-shaped.

use ::core::{
    ffi::{c_char, c_longlong, c_uint, c_ulonglong, c_void},
    ptr::{null, null_mut},
};
#[cfg(target_family = "wasm")]
use crate::compat::{getpid, mkstemp, stpcpy, umask};
#[cfg(unix)]
use libc::{getpid, mkstemp, stpcpy, umask};

use libc::{
    __errno_location,
    calloc,
    free,
    getenv,
    malloc,
    realloc,
    sleep,
    sprintf,
    strcpy,
    strdup,
    strerror,
    strlen,
    strndup,
    EINTR,
};

use crate::{
    entry::{posix_pedantic, stopchar_map},
    ffi_types::{__mode_t, mode_t, pid_t, size_t, ssize_t},
    file::nameseq,
    floc::Floc,
    output::{error, out_of_memory, FmtArg},
    posixos::os_anontmp,
    stdio::FILE,
    sys_stat::stat,
};

extern "C" {
    fn stat(file: *const c_char, buf: *mut stat) -> i32;
    static mut stderr: *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> i32;
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

/// Order two strings the way make's `alpha_compare` does: by the signed
/// difference of their first bytes when those differ (`char` is signed on the
/// supported targets), otherwise by a `strcmp`-equivalent unsigned byte
/// comparison.
///
/// This replaces the c2rust `qsort` comparator `unsafe extern "C" fn
/// alpha_compare(*const c_void, *const c_void) -> i32` — a safe `Ordering` over
/// byte slices is what a Rust `sort_by` wants, and it lets callers drop the
/// `void*`/`qsort` FFI entirely. The original is preserved verbatim as a test
/// oracle (see `alpha_compare_tests`). An absent first byte (empty slice) is
/// treated as the NUL the C code reads at `*s`, so the ordering matches
/// `alpha_compare` for every input, including empty operands.
pub(crate) fn alpha_cmp(a: &[u8], b: &[u8]) -> ::core::cmp::Ordering {
    let c1 = a.first().copied().unwrap_or(0);
    let c2 = b.first().copied().unwrap_or(0);
    if c1 != c2 {
        // Promote through `c_char` so the first differing byte's sign follows
        // the target's `char` signedness, exactly as `*s1 as i32 - *s2 as i32`.
        (c1 as c_char as i32).cmp(&(c2 as c_char as i32))
    } else {
        // Equal first byte (or both empty): `strcmp`, i.e. unsigned
        // lexicographic order. These strings carry no interior NUL, so the
        // shorter-is-a-prefix case matches `strcmp` reaching the terminator.
        a.cmp(b)
    }
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
pub unsafe fn collapse_continuations(ctx: &crate::execctx::ExecContext, line: *mut c_char) {
    let len = strlen(line);
    // Include the existing NUL slot so the new terminator is written by
    // indexing rather than raw pointer arithmetic.
    let buf = ::core::slice::from_raw_parts_mut(line as *mut u8, len + 1);
    let new_len = collapse_continuations_bytes(&mut buf[..len], posix_pedantic(ctx), |c| {
        stop_set(c as c_char, MAP_BLANK)
    });
    buf[new_len] = 0;
}

/// Write `n` spaces to stdout.
/// # Safety
/// Always safe; unsafe only for C-API signature compatibility.
pub fn print_spaces(n: c_uint) {
    use std::io::Write;
    // No flush: the caller's trace line follows in the same Rust stdout
    // buffer and flushes both together, as the C putchar+printf+fflush did.
    let _ = std::io::stdout().write_all(&vec![b' '; n as usize]);
}

/// Concatenate byte strings, returning the joined bytes as an owned,
/// NUL-terminated buffer. Empty arguments contribute nothing.
///
/// Safe and pure: no raw pointers, no shared state, no `ctx`. Callers
/// bridging from C strings build `args` with [`cstr_bytes_or_empty`].
pub fn concat(args: &[&[u8]]) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    for &s in args {
        if !s.is_empty() {
            buf.extend_from_slice(s);
        }
    }
    buf.push(0);
    buf
}

/// View a NUL-terminated C string as a byte slice up to (not including) its
/// terminator, or an empty slice for a null pointer — the FFI-boundary
/// counterpart callers use to build [`concat`]'s `args`.
///
/// # Safety
/// `p` must be null or point to a valid NUL-terminated C string.
pub unsafe fn cstr_bytes_or_empty<'a>(p: *const c_char) -> &'a [u8] {
    if p.is_null() {
        &[]
    } else {
        ::core::ffi::CStr::from_ptr(p).to_bytes()
    }
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
    assert!(!s.is_null(), "next_token: s must not be null");
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
    assert!(!ptr.is_null(), "find_next_token: ptr must not be null");
    let p: *const c_char = next_token(*ptr);
    assert!(
        !p.is_null(),
        "find_next_token: token address must not be null"
    );
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

/// unlink(2) via `std::fs::remove_file`, with the C call sites' EINTR retry
/// folded in. Returns 0/-1 like the C call and leaves errno set on failure so
/// the callers' perror-style paths print identical bytes.
/// # Safety
/// `name` must be a valid NUL-terminated path.
pub unsafe fn unlink_c(name: *const c_char) -> i32 {
    #[cfg(unix)]
    use std::os::unix::ffi::OsStrExt;
    #[cfg(target_os = "wasi")]
    use std::os::wasi::ffi::OsStrExt;
    let os = ::std::ffi::OsStr::from_bytes(::core::ffi::CStr::from_ptr(name).to_bytes());
    loop {
        match ::std::fs::remove_file(os) {
            Ok(()) => return 0,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => {
                *__errno_location() = e.raw_os_error().unwrap_or(0);
                return -1;
            }
        }
    }
}

/// Write `len` bytes from `buffer` to `fd`, retrying on EINTR and short
/// writes (`write_all`'s exact contract). Returns `len` on success or -1 on
/// failure; errno is left set by the failing write(2).
/// # Safety
/// `buffer` must be valid for reads of `len` bytes; `fd` must be open.
pub unsafe fn writebuf(fd: i32, buffer: *const c_void, len: size_t) -> ssize_t {
    use std::io::Write;
    let bytes = ::core::slice::from_raw_parts(buffer as *const u8, len);
    // Borrow the fd as a File without taking ownership.
    let mut f =
        ::core::mem::ManuallyDrop::new(<std::fs::File as std::os::fd::FromRawFd>::from_raw_fd(fd));
    match f.write_all(bytes) {
        Ok(()) => len as ssize_t,
        Err(_) => -1,
    }
}

/// Read up to `len` bytes from `fd` into `buffer`, retrying on EINTR and
/// short reads. Returns the number of bytes read (stopping early only at
/// EOF), or -1 on any failure — even after a partial read, like the C loop.
/// # Safety
/// `buffer` must be valid for writes of `len` bytes; `fd` must be open.
pub unsafe fn readbuf(fd: i32, buffer: *mut c_void, len: size_t) -> ssize_t {
    use std::io::Read;
    let buf = ::core::slice::from_raw_parts_mut(buffer as *mut u8, len);
    // Borrow the fd as a File without taking ownership.
    let mut f =
        ::core::mem::ManuallyDrop::new(<std::fs::File as std::os::fd::FromRawFd>::from_raw_fd(fd));
    let mut done = 0usize;
    while done < buf.len() {
        match f.read(&mut buf[done..]) {
            Ok(0) => break,
            Ok(n) => done += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => return -1,
        }
    }
    done as ssize_t
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

    sprintf(filenm.as_mut_ptr(), c".make-spin-%s".as_ptr(), type_0);
    let spinfile = crate::fs::path_from_c(filenm.as_ptr());
    if crate::fs::exists(spinfile) {
        fprintf(stderr, c"SPIN on %s\n".as_ptr(), filenm.as_ptr());
        while crate::fs::exists(spinfile) {
            sleep(1);
        }
    }
}

/// Debugging aid: append a line, tagged with the PID, to `/tmp/gmkdebug.log`.
///
/// # Safety
/// `msg` must be null or a valid NUL-terminated string.
/// Coalesce a possibly-null debug string to a printable `(null)` sentinel.
fn or_null_sentinel(msg: *const c_char) -> *const c_char {
    if msg.is_null() {
        c"(null)".as_ptr()
    } else {
        msg
    }
}

pub unsafe fn dbg(msg: *const c_char) {
    use std::io::Write;
    let Ok(mut fp) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/gmkdebug.log")
    else {
        return;
    };
    let text = ::core::ffi::CStr::from_ptr(or_null_sentinel(msg)).to_bytes();
    let _ = write!(fp, "{}: ", make_pid() as c_uint);
    let _ = fp.write_all(text);
    let _ = fp.write_all(b"\n");
}

const DEFAULT_TMPDIR: &::core::ffi::CStr = c"/tmp";
const DEFAULT_TMPFILE: &::core::ffi::CStr = c"GmXXXXXX";

/// Outcome of probing one `$TMPDIR`-style environment variable.
enum TmpdirCandidate {
    /// Unset or empty: skip silently.
    Unset,
    /// Set and pointing at a usable directory.
    Usable(*const c_char),
    /// Set but unusable (missing, stat error, or not a directory): already
    /// reported, and means the caller should fall back to the default.
    Invalid,
}

/// `stat(2)` retried across `EINTR`, mirroring the C `EINTRLOOP`. Pulled out of
/// `eval_tmpdir_var` so that probe isn't carrying the bare retry loop.
///
/// # Safety
///
/// `path` must be a valid NUL-terminated C string and `st` a valid `stat`
/// pointer for the duration of the call.
unsafe fn stat_retrying_eintr(path: *const c_char, st: *mut stat) -> i32 {
    loop {
        let r = stat(path, st);
        if !(r == -1 && *__errno_location() == EINTR) {
            return r;
        }
    }
}

/// Probe one candidate environment variable, reporting any set-but-unusable
/// value via `error`. Pulled out of `get_tmpdir` to keep that function flat.
unsafe fn eval_tmpdir_var(
    ctx: &crate::execctx::ExecContext,
    var: &::core::ffi::CStr,
) -> TmpdirCandidate {
    let val = getenv(var.as_ptr());
    if val.is_null() || *val == 0 {
        return TmpdirCandidate::Unset;
    }

    let mut st: stat = ::core::mem::zeroed();
    let r = stat_retrying_eintr(val, &mut st);
    if r < 0 {
        error(
            ctx,
            null::<Floc>(),
            var.count_bytes() + strlen(val) + strlen(strerror(*__errno_location())),
            c"%s value %s: %s".as_ptr(),
            &[
                FmtArg::Str((var.as_ptr()) as *const ::core::ffi::c_char),
                FmtArg::Str((val) as *const ::core::ffi::c_char),
                FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char),
            ],
        );
        return TmpdirCandidate::Invalid;
    }
    if st.st_mode & S_IFMT != S_IFDIR {
        error(
            ctx,
            null::<Floc>(),
            var.count_bytes() + strlen(val),
            c"%s value %s: not a directory".as_ptr(),
            &[
                FmtArg::Str((var.as_ptr()) as *const ::core::ffi::c_char),
                FmtArg::Str((val) as *const ::core::ffi::c_char),
            ],
        );
        return TmpdirCandidate::Invalid;
    }
    TmpdirCandidate::Usable(val)
}

/// Return the directory for temporary files: `$MAKE_TMPDIR`, `$TMPDIR`, or
/// the default, in that order, warning about set-but-unusable values. The
/// result is computed once per run and cached on `ctx.tmpdir`.
///
/// # Safety
/// Reads the environment; the caller's `ExecContext` must not be shared
/// across concurrent callers without synchronization.
pub unsafe fn get_tmpdir(ctx: &crate::execctx::ExecContext) -> *const c_char {
    if ctx.tmpdir.0.get().is_null() {
        let mut found = false;
        for var in [c"MAKE_TMPDIR", c"TMPDIR"] {
            match eval_tmpdir_var(ctx, var) {
                TmpdirCandidate::Unset => {}
                TmpdirCandidate::Usable(val) => {
                    ctx.tmpdir.0.set(val);
                    return val;
                }
                TmpdirCandidate::Invalid => found = true,
            }
        }

        let tmpdir = DEFAULT_TMPDIR.as_ptr();
        ctx.tmpdir.0.set(tmpdir);
        if found {
            error(
                ctx,
                null::<Floc>(),
                0,
                c"using default temporary directory '%s'".as_ptr(),
                &[FmtArg::Str(tmpdir)],
            );
        }
    }
    ctx.tmpdir.0.get()
}

/// Build an `xmalloc`'d mkstemp template `<tmpdir>/GmXXXXXX`.
/// # Safety
/// Must run single-threaded (uses `get_tmpdir`); caller frees the result.
pub unsafe fn get_tmptemplate(ctx: &crate::execctx::ExecContext) -> *mut c_char {
    let tmpdir = get_tmpdir(ctx);

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
/// Create a named temporary file with `mkstemp` and return its descriptor
/// together with the `xmalloc`'d file name the caller owns (and frees). On
/// failure reports the error and returns `(-1, null)`.
///
/// Replaces the c2rust `get_tmpfd(name: *mut *mut c_char)` out-parameter: the
/// descriptor and the name are the result, so callers no longer thread a
/// pointer-to-pointer, and the "name is null ⇒ anonymous" overload is gone (see
/// [`open_anon_tmpfd`]).
///
/// # Safety
/// Always safe in practice; `unsafe` only for the libc temp-file calls. The
/// caller takes ownership of the returned name.
pub unsafe fn open_named_tmpfd(ctx: &crate::execctx::ExecContext) -> (i32, *mut c_char) {
    // Make sure the temporary file is never readable by other users.
    let mask: mode_t = umask(0o77);
    let tmpnm = get_tmptemplate(ctx);
    let mut fd: i32;
    loop {
        fd = mkstemp(tmpnm);
        if !(fd == -1 && *__errno_location() == EINTR) {
            break;
        }
    }

    if fd < 0 {
        error(
            ctx,
            null::<Floc>(),
            0,
            c"cannot create temporary file %s: %s".as_ptr(),
            &[
                FmtArg::Str(tmpnm),
                FmtArg::Str(strerror(*__errno_location())),
            ],
        );
        free(tmpnm as *mut c_void);
        // Note: like the original, `umask` is intentionally left at the
        // restrictive value on this failure path.
        return (-1, null_mut());
    }

    umask(mask);
    (fd, tmpnm)
}

/// Create a temporary file descriptor with no visible name: an OS anonymous
/// temp file where available, otherwise an `mkstemp` file unlinked immediately.
/// On failure reports the error and returns `-1`.
///
/// # Safety
/// Always safe in practice; `unsafe` only for the libc temp-file calls.
pub unsafe fn open_anon_tmpfd(ctx: &crate::execctx::ExecContext) -> i32 {
    // If there's an OS-specific way to get an anonymous temp file, use it.
    let fd = os_anontmp(ctx);
    if fd >= 0 {
        return fd;
    }

    let (fd, tmpnm) = open_named_tmpfd(ctx);
    if fd < 0 {
        // `open_named_tmpfd` already reported the error and freed the name.
        return -1;
    }

    // Unlink immediately so the file has no name; `umask` only affects the
    // already-completed creation, so restoring it before the unlink (as
    // `open_named_tmpfd` does) is equivalent to the original's order.
    let r = unlink_c(tmpnm);
    if r < 0 {
        error(
            ctx,
            null::<Floc>(),
            0,
            c"cannot unlink temporary file %s: %s".as_ptr(),
            &[
                FmtArg::Str(tmpnm),
                FmtArg::Str(strerror(*__errno_location())),
            ],
        );
    }
    free(tmpnm as *mut c_void);
    fd
}

/// Create a read-write temporary file (the former `fopen "wb+"`); `*name`
/// receives the `xmalloc`'d file name. Returns `None` on failure (after
/// `open_named_tmpfd` reported the error). The C version's separate
/// `fdopen` failure path is gone: wrapping an already-open fd in a
/// `std::fs::File` cannot fail.
///
/// # Safety
/// `name` must be non-null and valid for writes; the caller takes ownership
/// of `*name`.
pub unsafe fn get_tmpfile(
    ctx: &crate::execctx::ExecContext,
    name: *mut *mut c_char,
) -> Option<std::fs::File> {
    let name = name.as_mut().expect("get_tmpfile: name must be non-null");

    let (fd, tmpnm) = open_named_tmpfd(ctx);
    *name = tmpnm;
    if fd < 0 {
        return None;
    }
    // SAFETY: `open_named_tmpfd` hands back exclusive ownership of an open
    // fd; the `File` now owns it and closes it on drop.
    Some(<std::fs::File as std::os::fd::FromRawFd>::from_raw_fd(fd))
}

#[cfg(test)]
mod bufio_tests {
    use {
        super::{readbuf, writebuf},
        std::os::fd::AsRawFd,
    };

    /// writebuf writes everything and readbuf reads it back, stopping at
    /// EOF with a short count (the C loops' contract, now write_all/read).
    #[test]
    fn writebuf_readbuf_round_trip_and_short_read_at_eof() {
        use std::io::Seek;
        let mut f = tempfile_for_test();
        let data = b"raw fd round trip";
        unsafe {
            let n = writebuf(f.as_raw_fd(), data.as_ptr().cast(), data.len());
            assert_eq!(n, data.len() as isize);
            f.rewind().expect("rewind");
            let mut buf = [0u8; 64];
            let n = readbuf(f.as_raw_fd(), buf.as_mut_ptr().cast(), buf.len());
            assert_eq!(n, data.len() as isize, "short count at EOF, not -1");
            assert_eq!(&buf[..data.len()], data);
        }
    }

    /// Both return -1 on a descriptor opened the wrong way (EBADF from the
    /// kernel — the C loops' failure path), without touching valid fds.
    #[test]
    fn writebuf_readbuf_report_failure_as_minus_one() {
        let rd = std::fs::File::open("/dev/null").expect("open ro");
        let wr = std::fs::OpenOptions::new()
            .write(true)
            .open("/dev/null")
            .expect("open wo");
        let mut buf = [0u8; 4];
        unsafe {
            assert_eq!(writebuf(rd.as_raw_fd(), buf.as_ptr().cast(), buf.len()), -1);
            assert_eq!(
                readbuf(wr.as_raw_fd(), buf.as_mut_ptr().cast(), buf.len()),
                -1
            );
        }
    }

    fn tempfile_for_test() -> std::fs::File {
        let ctx = crate::execctx::ExecContext::default();
        let fd = unsafe { super::open_anon_tmpfd(&ctx) };
        assert!(fd >= 0, "open_anon_tmpfd failed");
        // SAFETY: fresh fd owned by this test.
        unsafe { <std::fs::File as std::os::fd::FromRawFd>::from_raw_fd(fd) }
    }
}

#[cfg(test)]
mod tmpfile_tests {
    use {
        super::{dbg, get_tmpfile, open_anon_tmpfd, open_named_tmpfd},
        ::core::{ffi::c_char, ptr::null_mut},
    };

    // `get_tmpfile` drives the whole temp-file chain (`open_named_tmpfd` ->
    // `get_tmptemplate` -> `get_tmpdir`): it must hand back a read-write
    // `std::fs::File` and an owned, non-null name. Round-trip a write
    // through it to prove the descriptor is real, then clean up.
    #[test]
    fn get_tmpfile_round_trips_and_names_the_file() {
        use std::io::{Read, Seek, Write};
        let ctx = crate::execctx::ExecContext::default();
        let mut name: *mut c_char = null_mut();
        unsafe {
            let mut fp = get_tmpfile(&ctx, &mut name).expect("get_tmpfile returned no file");
            assert!(!name.is_null(), "get_tmpfile left the name unset");

            let data = b"crap-coverage-probe\n";
            fp.write_all(data).expect("write to temp file");
            fp.rewind().expect("rewind temp file");

            let mut buf = [0u8; 32];
            fp.read_exact(&mut buf[..data.len()]).expect("read back");
            assert_eq!(&buf[..data.len()], data);

            drop(fp);
            assert_eq!(
                libc::unlink(name),
                0,
                "temp file should still exist to unlink"
            );
            libc::free(name.cast());
        }
    }

    // The anonymous path takes the `os_anontmp` / unlink-immediately branch and
    // just returns a bare descriptor with no visible name.
    #[test]
    fn open_anon_tmpfd_returns_open_descriptor() {
        let ctx = crate::execctx::ExecContext::default();
        unsafe {
            let fd = open_anon_tmpfd(&ctx);
            assert!(fd >= 0, "anonymous open_anon_tmpfd failed: {fd}");
            libc::close(fd);
        }
    }

    // The named path returns both a live descriptor and the owned, non-null
    // name; the file must exist on disk at that name and the descriptor must be
    // writable. This is the replacement for the old `*mut *mut c_char`
    // out-parameter.
    #[test]
    fn open_named_tmpfd_returns_descriptor_and_name() {
        let ctx = crate::execctx::ExecContext::default();
        unsafe {
            let (fd, name) = open_named_tmpfd(&ctx);
            assert!(fd >= 0, "open_named_tmpfd failed: {fd}");
            assert!(!name.is_null(), "open_named_tmpfd left the name unset");

            let data = b"named-tmpfd-probe\n";
            let wrote = libc::write(fd, data.as_ptr().cast(), data.len());
            assert_eq!(wrote, data.len() as isize);

            // The name must refer to the very file we just wrote.
            assert_eq!(libc::unlink(name), 0, "named temp file should exist");
            libc::close(fd);
            libc::free(name.cast());
        }
    }

    // `dbg` appends a PID-tagged line to /tmp/gmkdebug.log; exercise both the
    // real-message and null-message branches and confirm the line lands.
    #[test]
    fn dbg_appends_tagged_line() {
        const MARKER: &::core::ffi::CStr = c"dbg_unit_probe_marker";
        unsafe {
            dbg(MARKER.as_ptr());
            dbg(null_mut());
        }
        let log = std::fs::read_to_string("/tmp/gmkdebug.log")
            .expect("dbg should have created /tmp/gmkdebug.log");
        assert!(
            log.contains("dbg_unit_probe_marker"),
            "dbg did not write the marker line"
        );
        assert!(
            log.contains("(null)"),
            "dbg did not write the null-message line"
        );
    }
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
    use {
        super::alpha_cmp,
        ::core::{
            cmp::Ordering,
            ffi::{c_char, c_void},
        },
    };

    /// Verbatim pre-refactor `qsort` comparator, preserved as the behavior
    /// oracle (AGENTS.md: keep the original `unsafe` code as a `#[cfg(test)]`
    /// oracle and assert the safe replacement agrees with it).
    unsafe extern "C" fn alpha_compare_unsafe_oracle(v1: *const c_void, v2: *const c_void) -> i32 {
        let s1: *const c_char = *(v1 as *mut *mut c_char);
        let s2: *const c_char = *(v2 as *mut *mut c_char);
        if *s1 != *s2 {
            return *s1 as i32 - *s2 as i32;
        }
        libc::strcmp(s1, s2)
    }

    /// Drive two NUL-terminated byte strings through the oracle, matching how
    /// `qsort` invoked it: each element is a `char *`, so pass the address of a
    /// string pointer.
    unsafe fn oracle(a: &[u8], b: &[u8]) -> Ordering {
        let ca: Vec<u8> = a.iter().copied().chain([0]).collect();
        let cb: Vec<u8> = b.iter().copied().chain([0]).collect();
        let pa: *const c_char = ca.as_ptr() as *const c_char;
        let pb: *const c_char = cb.as_ptr() as *const c_char;
        let r = alpha_compare_unsafe_oracle(
            (&pa as *const *const c_char).cast::<c_void>(),
            (&pb as *const *const c_char).cast::<c_void>(),
        );
        r.cmp(&0)
    }

    #[test]
    fn equal_first_byte_falls_back_to_strcmp() {
        assert_eq!(alpha_cmp(b"abc", b"abd"), Ordering::Less);
        assert_eq!(alpha_cmp(b"abc", b"abc"), Ordering::Equal);
        assert_eq!(alpha_cmp(b"abd", b"abc"), Ordering::Greater);
        // A NUL (end of the shorter string) sorts before any byte.
        assert_eq!(alpha_cmp(b"ab", b"abc"), Ordering::Less);
    }

    #[test]
    fn differing_first_byte_orders_by_that_byte() {
        // 'B' (66) sorts before 'a' (97): the first-byte fast path.
        assert_eq!(alpha_cmp(b"B", b"a"), Ordering::Less);
        assert_eq!(alpha_cmp(b"a", b"B"), Ordering::Greater);
    }

    /// The safe `alpha_cmp` must return the same ordering as the preserved
    /// unsafe `qsort` comparator for every pair — including empty operands and
    /// high (sign-bit) bytes, where the signed-first-byte vs unsigned-`strcmp`
    /// split is observable.
    #[test]
    fn matches_unsafe_oracle() {
        let samples: &[&[u8]] = &[
            b"", b"a", b"B", b"ab", b"abc", b"abd", b"abcd", b"\x80", b"\x80a", b"\x01", b"\xff",
            b"\x7f", b"A\x80",
        ];
        for &a in samples {
            for &b in samples {
                let safe = alpha_cmp(a, b);
                let unsafe_ord = unsafe { oracle(a, b) };
                assert_eq!(
                    safe, unsafe_ord,
                    "alpha_cmp({a:?}, {b:?}) = {safe:?} but oracle = {unsafe_ord:?}"
                );
            }
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
    use {
        super::next_token,
        crate::entry::initialize_stopchar_map,
        std::ffi::{c_char, CStr, CString},
    };

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
    use {
        super::lindex,
        ::core::{ffi::c_char, ptr::null_mut},
    };

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
    use {super::skip_reference, ::core::ffi::c_char};

    // Re-derive the helpers the oracle needs, identical to the module ones.
    const MAP_NUL: i32 = 0x0001;
    const MAP_VARSEP: i32 = 0x0080;

    fn stop_set(c: c_char, mask: i32) -> bool {
        crate::entry::stopchar_map()[c as u8 as usize] as i32 & mask != 0
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
        crate::entry::initialize_stopchar_map();

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
    use {
        super::{end_of_token, MAP_NUL, MAP_SPACE},
        ::core::ffi::c_char,
    };

    fn stop_set(c: c_char, mask: i32) -> bool {
        crate::entry::stopchar_map()[c as u8 as usize] as i32 & mask != 0
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
        crate::entry::initialize_stopchar_map();

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

#[cfg(test)]
mod eval_tmpdir_var_tests {
    use super::{eval_tmpdir_var, TmpdirCandidate};

    /// An unset variable yields `Unset`; a variable pointing at a real directory
    /// yields `Usable` carrying its value. Drives the common (non-error) arms of
    /// the probe and, through them, the `EINTR`-retrying `stat` wrapper — without
    /// touching the `error`-reporting path.
    #[test]
    fn unset_then_usable_directory() {
        let ctx = crate::execctx::ExecContext::default();

        std::env::remove_var("MAKE_PROBE_TMPDIR_UNSET");
        // SAFETY: pass the c2rust probe a NUL-terminated name; it only reads the
        // process environment and `stat`s the value.
        let unset = unsafe { eval_tmpdir_var(&ctx, c"MAKE_PROBE_TMPDIR_UNSET") };
        assert!(
            matches!(unset, TmpdirCandidate::Unset),
            "an unset variable is skipped",
        );

        std::env::set_var("MAKE_PROBE_TMPDIR_DIR", "/tmp");
        // SAFETY: as above; `/tmp` is a real directory, so the success arm runs.
        let usable = unsafe { eval_tmpdir_var(&ctx, c"MAKE_PROBE_TMPDIR_DIR") };
        match usable {
            // SAFETY: `Usable` carries the `getenv` value pointer, valid until
            // the next environment mutation; we read it before removing the var.
            TmpdirCandidate::Usable(p) => {
                assert_eq!(
                    unsafe { ::core::ffi::CStr::from_ptr(p) }.to_bytes(),
                    b"/tmp",
                    "a usable directory returns its value",
                )
            }
            _ => panic!("expected a usable directory"),
        }
        std::env::remove_var("MAKE_PROBE_TMPDIR_DIR");
    }
}

#[cfg(test)]
mod concat_tests {
    use super::{concat, cstr_bytes_or_empty};

    #[test]
    fn skips_empty_and_joins_the_rest() {
        // Exercises concat's empty-arg skip and multi-arg join, plus the
        // trailing NUL terminator.
        let long =
            b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz1234";
        assert!(long.len() > 60, "long arg exercises a multi-arg join");
        let out = concat(&[b"hello", b"", long]);
        assert!(out.ends_with(&[0]), "buffer is NUL-terminated");
        let bytes = &out[..out.len() - 1];
        assert!(bytes.starts_with(b"hello"));
        // Empty args contribute nothing; total is "hello" + the long arg.
        assert_eq!(bytes.len(), 5 + long.len());
    }

    #[test]
    fn returns_a_fresh_buffer_each_call() {
        // No shared/reused state (unlike the former `ctx`-owned scratch
        // buffer): each call is an independent, safely owned `Vec<u8>`.
        let first = concat(&[b"hi"]);
        let second = concat(&[b"bye"]);
        assert_eq!(first, b"hi\0");
        assert_eq!(second, b"bye\0");
    }

    #[test]
    fn cstr_bytes_or_empty_handles_null_and_valid_strings() {
        unsafe {
            assert_eq!(cstr_bytes_or_empty(::core::ptr::null()), b"");
            assert_eq!(cstr_bytes_or_empty(c"hello".as_ptr()), b"hello");
        }
    }
}

#[cfg(test)]
mod concat_unsafe_oracle {
    use {
        super::{cstr_bytes_or_empty, strlen},
        ::core::ffi::c_char,
    };

    /// Verbatim pre-conversion implementation (the last `unsafe fn concat`,
    /// before it became the safe, pure `super::concat`), preserved as a
    /// differential test oracle. The scratch buffer is a plain local
    /// `RefCell` argument rather than `ctx.concat_buffer` — that field no
    /// longer exists on `ExecContext` after this conversion, and the
    /// buffer's storage location isn't what's under test; the
    /// null/empty-skip and growth/terminator algorithm is.
    unsafe fn concat(
        buf_cell: &::core::cell::RefCell<Vec<u8>>,
        args: &[*const c_char],
    ) -> *const c_char {
        let mut buf = buf_cell.borrow_mut();
        buf.clear();
        for &s in args {
            if s.is_null() {
                continue;
            }
            let l = strlen(s);
            if l == 0 {
                continue;
            }
            buf.extend_from_slice(::core::slice::from_raw_parts(s as *const u8, l));
        }
        buf.push(0);
        buf.as_ptr() as *const c_char
    }

    #[test]
    fn safe_matches_oracle_over_representative_inputs() {
        let buf_cell = ::core::cell::RefCell::new(Vec::new());
        let long =
            c"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz1234";
        let hello = c"hello";
        let empty = c"";
        let hi = c"hi";
        let bye = c"bye";
        unsafe {
            let cases: [Vec<*const c_char>; 5] = [
                vec![
                    hello.as_ptr(),
                    ::core::ptr::null(),
                    empty.as_ptr(),
                    long.as_ptr(),
                ],
                vec![hi.as_ptr()],
                vec![bye.as_ptr()],
                vec![::core::ptr::null()],
                vec![],
            ];
            for case in &cases {
                let safe_args: Vec<&[u8]> = case.iter().map(|&p| cstr_bytes_or_empty(p)).collect();
                let safe_result = super::concat(&safe_args);
                let oracle_ptr = concat(&buf_cell, case);
                let oracle_bytes = ::core::ffi::CStr::from_ptr(oracle_ptr).to_bytes_with_nul();
                assert_eq!(safe_result, oracle_bytes, "mismatch for {case:?}");
            }
        }
    }
}

#[cfg(test)]
mod free_ns_tests {
    use {
        super::{free_ns, free_ns_chain},
        crate::file::NameSeq,
    };

    /// `free_ns_chain` must walk and free every node in a `next`-linked chain
    /// (which exercises `free_ns` on each), and a single-node chain is the base
    /// case. The nodes are `malloc`-allocated so the C `free` inside is valid.
    #[test]
    fn free_ns_chain_walks_and_frees_every_node() {
        unsafe {
            let sz = ::core::mem::size_of::<NameSeq>();
            let a = libc::malloc(sz) as *mut NameSeq;
            let b = libc::malloc(sz) as *mut NameSeq;
            (*a).next = b;
            (*a).name = ::core::ptr::null();
            (*b).next = ::core::ptr::null_mut();
            (*b).name = ::core::ptr::null();
            free_ns_chain(a as *mut crate::file::nameseq);

            // Single node, freed directly through `free_ns`.
            let c = libc::malloc(sz) as *mut NameSeq;
            (*c).next = ::core::ptr::null_mut();
            (*c).name = ::core::ptr::null();
            free_ns(c as *mut crate::file::nameseq);
        }
    }
}
