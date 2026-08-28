//! Rust replacement for gnulib's `findprog-in` module.
//!
//! `find_in_given_path` is the one symbol the Rust port still pulled from
//! `lib/libgnu.a`; every other gnulib symbol the translated code references
//! (`glob`, `globfree`, `fnmatch`, `getloadavg`, ...) resolves directly from
//! the C library. Providing it here lets the binary link against libc alone
//! and drops the static `gnu` archive from the build.
//!
//! The behaviour mirrors gnulib `lib/findprog-in.c` for the Unix case, which
//! is all the single call site (`job.rs`) exercises: it passes
//! `directory == NULL` and `optimize_for_exec == false`. The `directory`
//! and `optimize_for_exec` code paths are implemented for completeness so the
//! function remains a faithful drop-in for the C ABI symbol.

use ::core::ffi::{c_char, CStr};

/// True if `path` is executable (`X_OK`) and is not a directory, matching the
/// gnulib check that skips directories whose search bit happens to be set.
unsafe fn is_executable_file(path: *const c_char) -> bool {
    #[cfg(unix)]
    let eaccess = libc::eaccess;
    #[cfg(target_family = "wasm")]
    let eaccess = crate::compat::eaccess;
    if eaccess(path, libc::X_OK) != 0 {
        return false;
    }
    // An unreadable path is not executable, so a metadata failure is a plain
    // `false` here, exactly as the `stat < 0` arm was.
    crate::fs::metadata(crate::fs::path_from_c(path)).is_ok_and(|m| !m.is_dir())
}

/// Copy a NUL-terminated C string into a fresh `malloc`ed buffer so the caller
/// can release it with `free`. Returns NULL on allocation failure.
unsafe fn dup_cstr(s: *const c_char) -> *mut c_char {
    let len = libc::strlen(s);
    let buf = libc::malloc(len + 1) as *mut c_char;
    if !buf.is_null() {
        libc::memcpy(buf as *mut _, s as *const _, len + 1);
    }
    buf
}

/// Build `"[base/]dir/progname"` in a freshly `malloc`ed, NUL-terminated
/// buffer the caller releases with `free`. `base` is prepended (with a
/// separating `/`) only when `Some`, matching gnulib's resolution of relative
/// path elements against `directory`. Returns NULL on allocation failure.
fn join_path(base: Option<&CStr>, dir: &[u8], progname: &CStr) -> *mut c_char {
    const SEP: &[u8] = b"/";
    const NUL: &[u8] = b"\0";
    // The path pieces in order: optional `base` + `/`, then `dir` + `/` +
    // `progname` + NUL. Reserve and append each piece *fallibly* so OOM yields
    // NULL exactly as the C `malloc` path did, rather than aborting through
    // Rust's infallible allocator. Reserving per piece (rather than a single
    // precomputed total) keeps the size logic free of standalone arithmetic.
    let mut buf: Vec<u8> = Vec::new();
    let pieces = base
        .map(CStr::to_bytes)
        .into_iter()
        .flat_map(|b| [b, SEP])
        .chain([dir, SEP, progname.to_bytes(), NUL]);
    for piece in pieces {
        if buf.try_reserve(piece.len()).is_err() {
            return ::core::ptr::null_mut();
        }
        buf.extend_from_slice(piece);
    }

    // Hand the assembled bytes to the C caller in a `malloc`ed buffer it frees.
    // SAFETY: `out`, when non-null, is `buf.len()` bytes; the copy stays in
    // bounds of both buffers. NULL is returned on OOM, as the C code did.
    unsafe {
        let out = libc::malloc(buf.len()) as *mut c_char;
        if !out.is_null() {
            libc::memcpy(out.cast(), buf.as_ptr().cast(), buf.len());
        }
        out
    }
}

/// Split a `:`-separated search `path` into its directory elements, mapping
/// each empty element to "." (the current directory), exactly as gnulib's
/// PATH walk does. Pure and lazy: borrows the byte view and yields sub-slices
/// on demand (no eager allocation), so the caller can stop at the first match,
/// with no pointer cursor or length arithmetic.
fn path_dir_elements(path: &[u8]) -> impl Iterator<Item = &[u8]> {
    path.split(|&b| b == b':').map(|elem| {
        if elem.is_empty() {
            b".".as_slice()
        } else {
            elem
        }
    })
}

/// Locate `progname` using the directory list `path` (a `:`-separated string).
///
/// Returns a `malloc`ed pathname to the executable (owned by the caller), the
/// `progname` pointer itself when `optimize_for_exec` lets the caller exec it
/// directly, or NULL with `errno` set when nothing executable is found.
///
/// # Safety
///
/// `progname` must be a valid NUL-terminated C string. `path` and `directory`
/// must each be either NULL or a valid NUL-terminated C string. A non-NULL,
/// non-identity return value is a `malloc`ed buffer the caller must `free`.
pub unsafe fn find_in_given_path(
    progname: *const c_char,
    path: *const c_char,
    directory: *const c_char,
    optimize_for_exec: bool,
) -> *const c_char {
    // A name containing a slash is a path itself; it is never searched in PATH.
    if !libc::strchr(progname, b'/' as i32).is_null() {
        if optimize_for_exec {
            return progname;
        }
        if !directory.is_null() && *progname != b'/' as c_char {
            // Relative name resolved against `directory`: "directory/progname".
            return join_path(
                None,
                CStr::from_ptr(directory).to_bytes(),
                CStr::from_ptr(progname),
            ) as *const c_char;
        }
        return dup_cstr(progname) as *const c_char;
    }

    if path.is_null() {
        *libc::__errno_location() = libc::ENOENT;
        return ::core::ptr::null();
    }

    // ENOENT unless we hit an existing-but-non-executable match (EACCES).
    let mut failure_errno = libc::ENOENT;
    let path_bytes = ::core::ffi::CStr::from_ptr(path).to_bytes();
    for elem in path_dir_elements(path_bytes) {
        // `elem` is never empty (empty PATH entries map to "."), so it is a
        // valid `dir` slice for join_path.
        let base = if !directory.is_null() && elem[0] != b'/' {
            Some(CStr::from_ptr(directory))
        } else {
            None
        };
        let candidate = join_path(base, elem, CStr::from_ptr(progname));
        if !candidate.is_null() {
            if is_executable_file(candidate) {
                return candidate as *const c_char;
            }
            if *libc::__errno_location() == libc::EACCES {
                failure_errno = libc::EACCES;
            }
            libc::free(candidate as *mut _);
        }
    }

    *libc::__errno_location() = failure_errno;
    ::core::ptr::null()
}

#[cfg(test)]
mod join_path_unsafe_oracle {
    //! `join_path` now assembles its path in a `Vec<u8>` with bounds-checked
    //! appends and takes safe `Option<&CStr>`/`&[u8]`/`&CStr` inputs. This keeps
    //! the verbatim c2rust pointer-walked implementation as a differential
    //! oracle and asserts both produce identical NUL-terminated buffers
    //! (AGENTS rule 3).
    use super::join_path;
    use ::core::ffi::{c_char, CStr};

    /// Verbatim c2rust-era implementation: `malloc` plus offset-walked copies.
    unsafe fn oracle(
        base: *const c_char,
        dir: *const c_char,
        dir_len: usize,
        progname: *const c_char,
    ) -> *mut c_char {
        let base_len = if base.is_null() {
            0
        } else {
            libc::strlen(base)
        };
        let prog_len = libc::strlen(progname);
        let total = if base.is_null() { 0 } else { base_len + 1 } + dir_len + 1 + prog_len + 1;
        let buf = libc::malloc(total) as *mut c_char;
        if buf.is_null() {
            return buf;
        }
        let mut off = 0usize;
        if !base.is_null() {
            libc::memcpy(buf.add(off) as *mut _, base as *const _, base_len);
            off += base_len;
            *buf.add(off) = b'/' as c_char;
            off += 1;
        }
        libc::memcpy(buf.add(off) as *mut _, dir as *const _, dir_len);
        off += dir_len;
        *buf.add(off) = b'/' as c_char;
        off += 1;
        libc::memcpy(buf.add(off) as *mut _, progname as *const _, prog_len + 1);
        buf
    }

    /// Drive both implementations and assert identical built paths.
    fn check(base: Option<&CStr>, dir: &[u8], progname: &CStr) {
        let safe = join_path(base, dir, progname);
        // SAFETY: the oracle returns a `malloc`ed NUL-terminated buffer.
        let oracle_buf = unsafe {
            oracle(
                base.map_or(::core::ptr::null(), CStr::as_ptr),
                dir.as_ptr() as *const c_char,
                dir.len(),
                progname.as_ptr(),
            )
        };
        assert!(!safe.is_null() && !oracle_buf.is_null());
        // SAFETY: both are NUL-terminated; copy the bytes out before freeing.
        let (s, o) = unsafe {
            (
                CStr::from_ptr(safe).to_bytes().to_vec(),
                CStr::from_ptr(oracle_buf).to_bytes().to_vec(),
            )
        };
        assert_eq!(s, o, "dir={dir:?}");
        // SAFETY: both buffers came from `malloc`.
        unsafe {
            libc::free(safe.cast());
            libc::free(oracle_buf.cast());
        }
    }

    #[test]
    fn differential() {
        // No base: "dir/prog".
        check(None, b"bin", c"ls");
        check(None, b".", c"make");
        check(None, b"/usr/local/bin", c"gcc");
        // With base: "base/dir/prog".
        check(Some(c"/work"), b"src", c"cc");
        check(Some(c"/a/b"), b"rel", c"prog");
    }
}

#[cfg(test)]
mod path_dir_elements_tests {
    use super::path_dir_elements;

    fn elements(path: &[u8]) -> Vec<&[u8]> {
        path_dir_elements(path).collect()
    }

    #[test]
    fn single_element() {
        assert_eq!(elements(b"/usr/bin"), vec![b"/usr/bin".as_slice()]);
    }

    #[test]
    fn multiple_elements() {
        assert_eq!(
            elements(b"/usr/bin:/bin:/usr/local/bin"),
            vec![
                b"/usr/bin".as_slice(),
                b"/bin".as_slice(),
                b"/usr/local/bin".as_slice(),
            ]
        );
    }

    #[test]
    fn empty_element_becomes_dot() {
        // A leading, embedded, or trailing empty element all denote ".".
        assert_eq!(
            elements(b":/bin:"),
            vec![b".".as_slice(), b"/bin".as_slice(), b".".as_slice()]
        );
    }

    #[test]
    fn empty_path_is_single_dot() {
        assert_eq!(elements(b""), vec![b".".as_slice()]);
    }

    #[test]
    fn consecutive_separators_each_dot() {
        assert_eq!(
            elements(b"a::b"),
            vec![b"a".as_slice(), b".".as_slice(), b"b".as_slice()]
        );
    }
}
