//! wasm32 stand-ins for POSIX APIs that `libc` does not expose on WASI.
//!
//! This module is compiled in for any `target_family = "wasm"` target, but
//! only `wasm32-wasip1` is exercised end-to-end (`cargo check --target
//! wasm32-wasip1 --lib`); the byte-oriented `OsStr`/`OsString` call sites
//! elsewhere in the crate use `std::os::wasi::ffi`, which `wasm32-unknown-
//! unknown` does not provide, so that target is not currently supported even
//! though this module alone would build for it. Process spawning, signal
//! delivery, POSIX file locking, and a handful of terminal/session queries
//! are an accepted, tracked architectural gap on wasm (make cannot fork or
//! exec a recipe there); every stand-in below reports failure or a inert
//! default rather than doing real work, and is compiled in only for wasm —
//! unix keeps using the real `libc` items unchanged. `stpcpy` and `fnmatch`
//! are the exceptions: both are pure, OS-independent computations (a buffer
//! copy and a glob matcher), so they are implemented for real rather than
//! stubbed out.

use ::core::ffi::{c_char, c_double, c_int, c_long, c_short};

// ---------------------------------------------------------------------------
// Signals. wasm delivers none of these; `kill`/`signal` report failure and
// `sigemptyset` is a no-op over the (already inert) `sigset_t`.

pub const SIGHUP: c_int = 1;
pub const SIGINT: c_int = 2;
pub const SIGQUIT: c_int = 3;
pub const SIGTERM: c_int = 15;

pub type sighandler_t = usize;
pub const SIG_DFL: sighandler_t = 0;

/// # Safety
/// No preconditions: this never delivers a real signal.
pub unsafe fn kill(_pid: libc::pid_t, _sig: c_int) -> c_int {
    -1
}

/// # Safety
/// No preconditions: this never installs a real handler.
pub unsafe fn signal(_signum: c_int, _handler: sighandler_t) -> sighandler_t {
    SIG_DFL
}

/// # Safety
/// `set` is never dereferenced.
pub unsafe fn sigemptyset(_set: *mut libc::sigset_t) -> c_int {
    0
}

// ---------------------------------------------------------------------------
// Process identity/control.

/// wasm has no process id; a fixed non-zero value keeps callers that only
/// compare or print it working without special-casing.
pub fn getpid() -> libc::pid_t {
    1
}

/// wasm has no load-average concept.
///
/// # Safety
/// `loadavg` is never dereferenced.
pub unsafe fn getloadavg(_loadavg: *mut c_double, _nelem: c_int) -> c_int {
    -1
}

// ---------------------------------------------------------------------------
// Pipes / fifos / advisory locks — all part of the accepted job-control gap.

/// # Safety
/// `fds` is never dereferenced.
pub unsafe fn pipe(_fds: *mut c_int) -> c_int {
    -1
}

/// # Safety
/// Neither argument is dereferenced.
pub unsafe fn mkfifo(_path: *const c_char, _mode: libc::mode_t) -> c_int {
    -1
}

/// Layout-compatible stand-in for `struct flock` (WASI's `libc` has none):
/// only the fields make's output-sync locking sets are present.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: c_long,
    pub l_len: c_long,
    pub l_pid: libc::pid_t,
}

pub const F_SETLKW: c_int = 7;
pub const F_UNLCK: c_int = 2;
pub const F_WRLCK: c_int = 1;
pub const O_TMPFILE: c_int = 0;

/// # Safety
/// None of the pointer arguments are dereferenced.
pub unsafe fn pselect(
    _nfds: c_int,
    _readfds: *mut libc::fd_set,
    _writefds: *mut libc::fd_set,
    _errorfds: *mut libc::fd_set,
    _timeout: *const libc::timespec,
    _sigmask: *const libc::sigset_t,
) -> c_int {
    -1
}

// ---------------------------------------------------------------------------
// Temp files / permissions.

/// # Safety
/// `template` is never dereferenced.
pub unsafe fn mkstemp(_template: *mut c_char) -> c_int {
    -1
}

pub fn umask(_mask: libc::mode_t) -> libc::mode_t {
    0
}

// ---------------------------------------------------------------------------
// Terminal / session / access queries with no WASI equivalent.

/// # Safety
/// `fd` carries no preconditions; nothing is dereferenced.
pub unsafe fn ttyname(_fd: c_int) -> *mut c_char {
    ::core::ptr::null_mut()
}

/// # Safety
/// `sig` carries no preconditions; nothing is dereferenced.
pub unsafe fn strsignal(_sig: c_int) -> *mut c_char {
    ::core::ptr::null_mut()
}

/// # Safety
/// No preconditions.
pub unsafe fn getlogin() -> *mut c_char {
    ::core::ptr::null_mut()
}

/// # Safety
/// `path` is never dereferenced.
pub unsafe fn eaccess(_path: *const c_char, _mode: c_int) -> c_int {
    -1
}

// ---------------------------------------------------------------------------
// `stpcpy` — a pure buffer copy, not OS-dependent, so implemented for real.

/// Copy the NUL-terminated string at `src` into `dst` (including the NUL)
/// and return a pointer to the terminator, exactly like glibc's `stpcpy`.
///
/// # Safety
/// `src` must be a valid NUL-terminated C string; `dst` must be valid for
/// writes of at least `strlen(src) + 1` bytes.
pub unsafe fn stpcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char {
    let mut d = dst;
    let mut s = src;
    loop {
        *d = *s;
        if *s == 0 {
            return d;
        }
        d = d.add(1);
        s = s.add(1);
    }
}

// ---------------------------------------------------------------------------
// `fnmatch` — a pure glob matcher, not OS-dependent, so implemented for real.
//
// Supports the subset of POSIX fnmatch(3) this crate's callers actually use:
// `*`, `?`, and `[...]`/`[!...]` bracket expressions (with `a-z` ranges), plus
// the `FNM_PATHNAME` and `FNM_PERIOD` flags. No `FNM_NOESCAPE`/`FNM_CASEFOLD`
// support since no caller passes them.

/// Returned by [`fnmatch`] when `string` does not match `pattern`, matching
/// glibc's `FNM_NOMATCH`.
pub const FNM_NOMATCH: c_int = 1;

const FNM_PATHNAME: c_int = 1 << 0;
const FNM_PERIOD: c_int = 1 << 2;

/// `string` is "at a leading position" for [`FNM_PERIOD`] purposes: the very
/// start of the string, or (under `FNM_PATHNAME`) right after a `/`.
fn is_leading(text: &[u8], pos: usize, flags: c_int) -> bool {
    pos == 0 || (flags & FNM_PATHNAME != 0 && pos > 0 && text[pos - 1] == b'/')
}

fn bracket_match(class: &[u8], c: u8) -> bool {
    let (negate, class) = match class.first() {
        Some(b'!') | Some(b'^') => (true, &class[1..]),
        _ => (false, class),
    };
    let mut i = 0;
    let mut hit = false;
    while i < class.len() {
        if i + 2 < class.len() && class[i + 1] == b'-' {
            if class[i] <= c && c <= class[i + 2] {
                hit = true;
            }
            i += 3;
        } else {
            if class[i] == c {
                hit = true;
            }
            i += 1;
        }
    }
    hit != negate
}

/// `pi`/`ti` are absolute indices into `pat`/`text` (never re-sliced on
/// recursion) so [`is_leading`]'s `/`-lookback stays correct at every depth.
fn do_match(pat: &[u8], mut pi: usize, text: &[u8], mut ti: usize, flags: c_int) -> bool {
    while pi < pat.len() {
        match pat[pi] {
            b'*' => {
                while pi < pat.len() && pat[pi] == b'*' {
                    pi += 1;
                }
                // A leading period can't be absorbed into a `*`'s wildcard
                // span; only a zero-length expansion (leaving the dot for
                // the rest of the pattern to match explicitly) is allowed.
                let dot_blocks_star = flags & FNM_PERIOD != 0
                    && ti < text.len()
                    && text[ti] == b'.'
                    && is_leading(text, ti, flags);
                let end_k = if dot_blocks_star { ti } else { text.len() };
                for k in ti..=end_k {
                    if flags & FNM_PATHNAME != 0 && text[ti..k].contains(&b'/') {
                        break;
                    }
                    if do_match(pat, pi, text, k, flags) {
                        return true;
                    }
                }
                return false;
            }
            b'?' => {
                if ti >= text.len() || (flags & FNM_PATHNAME != 0 && text[ti] == b'/') {
                    return false;
                }
                if flags & FNM_PERIOD != 0 && text[ti] == b'.' && is_leading(text, ti, flags) {
                    return false;
                }
                pi += 1;
                ti += 1;
            }
            b'[' => {
                let Some(close) = pat[pi + 1..].iter().position(|&b| b == b']').map(|i| i + pi + 1)
                else {
                    // No closing `]`: treat `[` as a literal, like glibc does.
                    if ti >= text.len() || text[ti] != b'[' {
                        return false;
                    }
                    pi += 1;
                    ti += 1;
                    continue;
                };
                if ti >= text.len() || (flags & FNM_PATHNAME != 0 && text[ti] == b'/') {
                    return false;
                }
                if flags & FNM_PERIOD != 0 && text[ti] == b'.' && is_leading(text, ti, flags) {
                    return false;
                }
                if !bracket_match(&pat[pi + 1..close], text[ti]) {
                    return false;
                }
                pi = close + 1;
                ti += 1;
            }
            c => {
                if ti >= text.len() || text[ti] != c {
                    return false;
                }
                pi += 1;
                ti += 1;
            }
        }
    }
    ti == text.len()
}

/// Match `string` against the glob `pattern`, returning `0` on a match and
/// [`FNM_NOMATCH`] otherwise — the subset of glibc's `fnmatch(3)` that
/// archive-member glob matching (`ar.rs`) actually exercises.
///
/// # Safety
/// `pattern` and `string` must be valid NUL-terminated C strings.
pub unsafe fn fnmatch(pattern: *const c_char, string: *const c_char, flags: c_int) -> c_int {
    let pat = ::core::ffi::CStr::from_ptr(pattern).to_bytes();
    let text = ::core::ffi::CStr::from_ptr(string).to_bytes();
    if do_match(pat, 0, text, 0, flags) {
        0
    } else {
        FNM_NOMATCH
    }
}

#[cfg(test)]
mod tests {
    use super::{fnmatch, stpcpy, FNM_NOMATCH, FNM_PATHNAME, FNM_PERIOD};
    use std::ffi::CString;

    fn matches(pattern: &str, text: &str, flags: ::core::ffi::c_int) -> bool {
        let pattern = CString::new(pattern).unwrap();
        let text = CString::new(text).unwrap();
        unsafe { fnmatch(pattern.as_ptr(), text.as_ptr(), flags) == 0 }
    }

    /// Copies `src` (as a NUL-terminated C string) into a same-sized `dst`
    /// buffer via `stpcpy`, returning `(dst contents, offset of the
    /// returned pointer from the start of dst)`.
    fn run(src: &[u8]) -> (Vec<u8>, isize) {
        let mut src_buf: Vec<i8> = src.iter().map(|&b| b as i8).collect();
        src_buf.push(0);
        let mut dst_buf: Vec<i8> = vec![-1; src_buf.len()];
        let ret = unsafe { stpcpy(dst_buf.as_mut_ptr(), src_buf.as_ptr()) };
        let offset = unsafe { ret.offset_from(dst_buf.as_ptr()) };
        let dst_bytes = dst_buf.iter().map(|&b| b as u8).collect();
        (dst_bytes, offset)
    }

    #[test]
    fn copies_nonempty_string_and_returns_pointer_to_terminator() {
        let (dst, offset) = run(b"hello");
        assert_eq!(dst, b"hello\0");
        assert_eq!(offset, 5);
    }

    #[test]
    fn copies_empty_string_and_returns_pointer_to_start() {
        let (dst, offset) = run(b"");
        assert_eq!(dst, b"\0");
        assert_eq!(offset, 0);
    }

    #[test]
    fn fnmatch_literal_and_star_and_question() {
        assert!(matches("foo.o", "foo.o", 0));
        assert!(!matches("foo.o", "foo.c", 0));
        assert!(matches("*.o", "foo.o", 0));
        assert!(matches("*.o", "a/b.o", 0));
        assert!(matches("fo?.o", "foo.o", 0));
        assert!(!matches("fo?.o", "fooo.o", 0));
    }

    #[test]
    fn fnmatch_bracket_expressions() {
        assert!(matches("[abc].o", "b.o", 0));
        assert!(!matches("[abc].o", "d.o", 0));
        assert!(matches("[a-z].o", "m.o", 0));
        assert!(matches("[!a-z].o", "M.o", 0));
        assert!(!matches("[!a-z].o", "m.o", 0));
    }

    #[test]
    fn fnmatch_pathname_flag_restricts_wildcards_to_path_segment() {
        assert!(matches("*.o", "a/b.o", 0));
        assert!(!matches("*.o", "a/b.o", FNM_PATHNAME));
        assert!(matches("*/*.o", "a/b.o", FNM_PATHNAME));
        assert!(matches("?/b.o", "a/b.o", FNM_PATHNAME));
        assert!(!matches("a?b.o", "a/b.o", FNM_PATHNAME));
    }

    #[test]
    fn fnmatch_period_flag_requires_explicit_leading_dot() {
        assert!(matches(".*", ".hidden", FNM_PERIOD));
        assert!(!matches("*", ".hidden", FNM_PERIOD));
        assert!(matches("*", ".hidden", 0));
        assert!(!matches("?hidden", ".hidden", FNM_PERIOD));
    }

    #[test]
    fn fnmatch_returns_fnm_nomatch_constant_on_mismatch() {
        let pattern = CString::new("*.o").unwrap();
        let text = CString::new("foo.c").unwrap();
        assert_eq!(
            unsafe { fnmatch(pattern.as_ptr(), text.as_ptr(), 0) },
            FNM_NOMATCH
        );
    }
}
