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

use ::core::ffi::{c_char, c_int};

/// True if `path` is executable (`X_OK`) and is not a directory, matching the
/// gnulib check that skips directories whose search bit happens to be set.
unsafe fn is_executable_file(path: *const c_char) -> bool {
    if libc::eaccess(path, libc::X_OK) != 0 {
        return false;
    }
    let mut st: libc::stat = ::core::mem::zeroed();
    if libc::stat(path, &mut st) < 0 {
        return false;
    }
    (st.st_mode & libc::S_IFMT) != libc::S_IFDIR
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
/// buffer. `base` is prepended (with a separating `/`) only when non-null,
/// matching gnulib's resolution of relative path elements against `directory`.
unsafe fn join_path(
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
    // optional (base + '/') + dir + '/' + prog + '\0'
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
    // Copy the trailing NUL along with the program name.
    libc::memcpy(buf.add(off) as *mut _, progname as *const _, prog_len + 1);
    buf
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
    if !libc::strchr(progname, b'/' as c_int).is_null() {
        if optimize_for_exec {
            return progname;
        }
        if !directory.is_null() && *progname != b'/' as c_char {
            // Relative name resolved against `directory`: "directory/progname".
            return join_path(
                ::core::ptr::null(),
                directory,
                libc::strlen(directory),
                progname,
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
        // `elem` is never empty (empty PATH entries map to "."), so the byte
        // pointer and length are a valid `(dir, dir_len)` pair for join_path.
        let dir = elem.as_ptr() as *const c_char;
        let dir_len = elem.len();

        let base = if !directory.is_null() && elem[0] != b'/' {
            directory
        } else {
            ::core::ptr::null()
        };
        let candidate = join_path(base, dir, dir_len, progname);
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
