//! Searching the `vpath` / `VPATH` / `GPATH` directory search paths.
//!
//! Port of `vpath.c`. The search-path data structures are still C-shaped
//! (`xmalloc`-owned, nul-terminated string arrays handed out by the string
//! cache) because they are shared with `read.rs`, `remake.rs`, and
//! `implicit.rs` through `extern "C"` boundaries.

use ::core::ffi::{c_char, c_int, c_uint, c_void};
use ::core::ptr::{null, null_mut};

use libc::{
    __errno_location, free, memcpy, mempcpy, printf, puts, strcmp, strlen, strncmp, strrchr,
};

use crate::dir::{dir_file_exists_p, dir_name};
use crate::expand::expand_variable_buf;
use crate::ffi_types::{size_t, time_t, uintmax_t};
use crate::file::{file_timestamp_cons, lookup_file};
use crate::function::pattern_matches;
use crate::make_main::stopchar_map;
use crate::misc::{xmalloc, xrealloc};
use crate::read::find_percent;
use crate::stdio::FILE;
use crate::strcache::{strcache_add, strcache_add_len};
use crate::sys_stat::stat;

extern "C" {
    fn stat(file: *const c_char, buf: *mut stat) -> c_int;
    static mut stdout: *mut FILE;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
}

/// Character-class bits in `stopchar_map` (see `makeint.h`).
const MAP_BLANK: c_int = 0x0002;
const MAP_NEWLINE: c_int = 0x0004;
const MAP_COLON: c_int = 0x0040;
const MAP_SPACE: c_int = MAP_BLANK | MAP_NEWLINE;
/// On POSIX the search-path separator is `:`.
const MAP_PATHSEP: c_int = MAP_COLON;
const PATH_SEPARATOR_CHAR: c_int = ':' as i32;

/// `STOP_SET (c, mask)` from `makeint.h`: is `c` in any of the character
/// classes selected by `mask`?
unsafe fn stop_set(c: c_char, mask: c_int) -> bool {
    stopchar_map[c as u8 as usize] as c_int & mask != 0
}

/// `file->last_mtime` value meaning the modtime has not been checked yet
/// (`UNKNOWN_MTIME` in `filedef.h`).
const UNKNOWN_MTIME: uintmax_t = 0;
/// `file->last_mtime` of a file pretended old via `-o` (`OLD_MTIME`).
const OLD_MTIME: uintmax_t = 2;
/// `file->last_mtime` of a file pretended new via `-W` (`NEW_MTIME`): the
/// largest representable timestamp.
const NEW_MTIME: uintmax_t = uintmax_t::MAX;

/// One element of the `vpath` directive chain: a `%` pattern plus the
/// directories to search for files matching it.
#[repr(C)]
struct Vpath {
    next: *mut Vpath,
    /// The pattern to match, in the string cache.
    pattern: *const c_char,
    /// Pointer into `pattern` at the `%`, or null if there is none.
    percent: *const c_char,
    patlen: size_t,
    /// Null-terminated array of cached directory names, owned via `xmalloc`.
    searchpath: *mut *const c_char,
    /// Length of the longest entry in `searchpath`.
    maxlen: size_t,
}

/// The chain built from `vpath` directives, in reverse parse order until
/// `build_vpath_lists` reverses it.
static mut vpaths: *mut Vpath = null_mut();
/// The pseudo-vpath built from the `VPATH` variable, searched for every file.
static mut general_vpath: *mut Vpath = null_mut();
/// The pseudo-vpath built from the `GPATH` variable.
static mut gpaths: *mut Vpath = null_mut();

/// Reverse the chain of vpath directives and build the `VPATH`/`GPATH`
/// pseudo-vpaths from the variables' current values.
#[no_mangle]
pub unsafe fn build_vpath_lists() {
    // Reverse the chain so vpaths are searched in the order their
    // directives appeared in the makefile.
    let mut reversed: *mut Vpath = null_mut();
    let mut old = vpaths;
    while !old.is_null() {
        let next = (*old).next;
        (*old).next = reversed;
        reversed = old;
        old = next;
    }
    vpaths = reversed;

    if let Some(list) = vpath_from_variable(b"VPATH\0") {
        general_vpath = list;
    }
    if let Some(list) = vpath_from_variable(b"GPATH\0") {
        gpaths = list;
    }
}

/// Expand the named make variable and build a `%`-pattern vpath chain from
/// its value, leaving the directive-built `vpaths` chain untouched. Returns
/// `None` when the value is empty or whitespace-only.
unsafe fn vpath_from_variable(name: &[u8]) -> Option<*mut Vpath> {
    let mut p = expand_variable_buf(
        null_mut(),
        name.as_ptr() as *const c_char,
        (name.len() - 1) as size_t,
    );
    while stop_set(*p, MAP_SPACE) {
        p = p.add(1);
    }
    if *p == 0 {
        return None;
    }
    let saved = vpaths;
    vpaths = null_mut();
    let mut pattern = *b"%\0";
    construct_vpath_list(pattern.as_mut_ptr() as *mut c_char, p);
    let list = vpaths;
    vpaths = saved;
    Some(list)
}

/// Construct the `Vpath` listing for the pattern and search path given.
/// `pattern` and `dirpath` may be overwritten in place.
///
/// If `dirpath` is null, remove all previous listings with the same pattern.
/// If `pattern` is null too, remove all `Vpath` listings. The existing
/// chains' contents are not freed beyond the listing structures themselves,
/// since the string-cache strings may still be referenced elsewhere.
#[no_mangle]
pub unsafe extern "C" fn construct_vpath_list(pattern: *mut c_char, mut dirpath: *mut c_char) {
    let mut percent: *const c_char = null();
    if !pattern.is_null() {
        percent = find_percent(pattern);
    }

    if dirpath.is_null() {
        // Remove matching listings from the chain.
        let mut lastpath: *mut Vpath = null_mut();
        let mut path = vpaths;
        while !path.is_null() {
            let next = (*path).next;
            let matches = pattern.is_null()
                || ((percent.is_null() && (*path).percent.is_null()
                    || percent.offset_from(pattern)
                        == (*path).percent.offset_from((*path).pattern))
                    && strcmp(pattern, (*path).pattern) == 0);
            if matches {
                // Unlink and free this entry.
                if let Some(lp) = lastpath.as_mut() {
                    lp.next = next;
                } else {
                    vpaths = next;
                }
                free((*path).searchpath as *mut c_void);
                free(path as *mut c_void);
            } else {
                lastpath = path;
            }
            path = next;
        }
        return;
    }

    // Skip over any initial separators and blanks.
    while stop_set(*dirpath, MAP_BLANK | MAP_PATHSEP) {
        dirpath = dirpath.add(1);
    }

    // Figure out the maximum number of VPATH entries and allocate a vector
    // for them: one for every separator plus one for the final entry, plus
    // one for the null terminator.
    let mut maxelem: c_uint = 2;
    let mut p = dirpath;
    while *p != 0 {
        let c = *p;
        p = p.add(1);
        if stop_set(c, MAP_BLANK | MAP_PATHSEP) {
            maxelem += 1;
        }
    }

    let mut searchpath =
        xmalloc(maxelem as size_t * ::core::mem::size_of::<*const c_char>()) as *mut *const c_char;
    let mut maxvpath: size_t = 0;
    let mut elem: c_uint = 0;

    p = dirpath;
    while *p != 0 {
        // Find the end of this entry.
        let v = p;
        while *p != 0 && *p as c_int != PATH_SEPARATOR_CHAR && !stop_set(*p, MAP_BLANK) {
            p = p.add(1);
        }

        let mut len = p.offset_from(v) as size_t;
        // Omit a trailing slash, unless the entry is just "/".
        if len > 1 && *p.sub(1) as c_int == '/' as i32 {
            len -= 1;
        }

        // Skip "." entries: searching "." is implicit.
        if len > 1 || *v as c_int != '.' as i32 {
            *searchpath.offset(elem as isize) = dir_name(strcache_add_len(v, len));
            elem += 1;
            if len > maxvpath {
                maxvpath = len;
            }
        }

        // Skip over separators and blanks between entries.
        while stop_set(*p, MAP_BLANK | MAP_PATHSEP) {
            p = p.add(1);
        }
    }

    if elem > 0 {
        // Usually fewer entries than estimated; shrink, keeping room for
        // the null terminator.
        if elem < maxelem - 1 {
            searchpath = xrealloc(
                searchpath as *mut c_void,
                (elem as size_t + 1) * ::core::mem::size_of::<*const c_char>(),
            ) as *mut *const c_char;
        }
        *searchpath.offset(elem as isize) = null();

        let path = xmalloc(::core::mem::size_of::<Vpath>()) as *mut Vpath;
        (*path).searchpath = searchpath;
        (*path).maxlen = maxvpath;
        (*path).next = vpaths;
        vpaths = path;
        (*path).pattern = strcache_add(pattern);
        (*path).patlen = strlen(pattern);
        (*path).percent = if !percent.is_null() {
            (*path).pattern.offset(percent.offset_from(pattern))
        } else {
            null()
        };
    } else {
        // There were no entries; forget the whole thing.
        free(searchpath as *mut c_void);
    }
}

/// Search the `GPATH` list for a pathname (`file` of length `len`, which is
/// not null-terminated). Returns 1 if it is there, 0 if not.
#[no_mangle]
pub unsafe extern "C" fn gpath_search(file: *const c_char, len: size_t) -> c_int {
    if !gpaths.is_null() && len <= (*gpaths).maxlen {
        let mut gp = (*gpaths).searchpath;
        while !(*gp).is_null() {
            if strncmp(*gp, file, len) == 0 && *(*gp).offset(len as isize) == 0 {
                return 1;
            }
            gp = gp.add(1);
        }
    }
    0
}

/// Search the given `Vpath` list for a directory where `file` exists. If it
/// is found, return the cached full pathname, storing the file's modtime
/// into `*mtime_ptr` (when non-null) and the index of the matching search
/// path into `*path_index` (when non-null). Returns null if not found.
unsafe fn selective_vpath_search(
    path: *mut Vpath,
    file: *const c_char,
    mut mtime_ptr: *mut uintmax_t,
    path_index: *mut c_uint,
) -> *const c_char {
    let searchpath = (*path).searchpath;
    let maxvpath = (*path).maxlen;

    // If and only if *FILE is NOT a target, accept prospective files that
    // don't exist but are mentioned in a makefile.
    let not_target = {
        let f = lookup_file(file);
        f.is_null() || (*f).is_target() == 0
    };

    // Split *FILE into a directory prefix and a name-within-directory:
    // NAME_DPLEN is the length of the prefix, FILENAME points at the
    // name-within-directory, and FLEN is its length.
    let mut flen = strlen(file);
    let n = strrchr(file, '/' as i32);
    let name_dplen: size_t = if n.is_null() {
        0
    } else {
        n.offset_from(file) as size_t
    };
    let filename = if name_dplen > 0 { n.add(1) } else { file };
    if name_dplen > 0 {
        flen -= name_dplen + 1;
    }

    // Scratch buffer with room for the biggest VPATH entry, a slash, the
    // directory prefix that came with *FILE, another slash (not always
    // needed), the filename, and a null terminator.
    let mut name_buf = vec![0u8; maxvpath + 1 + name_dplen + 1 + flen + 1];
    let name = name_buf.as_mut_ptr() as *mut c_char;

    // Try each VPATH entry.
    let mut i: c_uint = 0;
    while !(*searchpath.offset(i as isize)).is_null() {
        let entry = *searchpath.offset(i as isize);

        // Put the next VPATH entry into NAME at P and advance P past it.
        let mut p = name;
        p = mempcpy(p as *mut c_void, entry as *const c_void, strlen(entry)) as *mut c_char;

        // Add the directory prefix already in *FILE.
        if name_dplen > 0 {
            *p = '/' as c_char;
            p = p.add(1);
            p = mempcpy(p as *mut c_void, file as *const c_void, name_dplen) as *mut c_char;
        }

        // Now add the name-within-directory at the end of NAME.
        if p != name && *p.sub(1) as c_int != '/' as i32 {
            *p = '/' as c_char;
            memcpy(p.add(1) as *mut c_void, filename as *const c_void, flen + 1);
        } else {
            memcpy(p as *mut c_void, filename as *const c_void, flen + 1);
        }

        // Check whether the file is mentioned in a makefile. If *FILE is
        // not a target, that is enough for us to decide this file exists.
        // If *FILE is a target, the file must also be mentioned as a target
        // to be chosen.
        let mut exists = false;
        let f = lookup_file(name);
        if !f.is_null() {
            exists = not_target || (*f).is_target() != 0;
            // Preserve the special -W / -o timestamps.
            if exists
                && !mtime_ptr.is_null()
                && ((*f).last_mtime == OLD_MTIME || (*f).last_mtime == NEW_MTIME)
            {
                *mtime_ptr = (*f).last_mtime;
                mtime_ptr = null_mut();
            }
        }

        let mut exists_in_cache = false;
        if !exists {
            // The file wasn't mentioned in the makefile. Clobber a null
            // into NAME at the last slash, so NAME is the directory to look
            // in (the directory cache knows it already), and ask the cache
            // whether the file exists there.
            *p = 0;
            exists = dir_file_exists_p(name, filename) != 0;
            exists_in_cache = exists;
        }

        if exists {
            // Put the slash back in NAME.
            *p = '/' as c_char;

            if exists_in_cache {
                // The directory cache may be out of date; check that the
                // file really exists in the filesystem, because higher
                // levels get confused otherwise.
                let mut st: stat = ::core::mem::zeroed();
                let mut e: c_int;
                loop {
                    e = stat(name, &mut st);
                    if !(e == -1 && *__errno_location() == libc::EINTR) {
                        break;
                    }
                }
                if e != 0 {
                    // Stale cache entry: keep searching the remaining
                    // vpath entries instead of returning it.
                    exists = false;
                } else if !mtime_ptr.is_null() {
                    *mtime_ptr =
                        file_timestamp_cons(name, st.st_mtim.tv_sec as time_t, st.st_mtim.tv_nsec);
                    mtime_ptr = null_mut();
                }
            }
        }

        if exists {
            // We found a file. If mtime_ptr wasn't set above, record
            // UNKNOWN_MTIME to say so.
            if !mtime_ptr.is_null() {
                *mtime_ptr = UNKNOWN_MTIME;
            }
            if !path_index.is_null() {
                *path_index = i;
            }
            return strcache_add_len(name, p.add(1).offset_from(name) as size_t + flen);
        }

        i += 1;
    }

    null()
}

/// Search the VPATH list whose pattern matches `file` for a directory where
/// `file` exists. On success returns the cached full pathname and fills
/// `*mtime_ptr`, `*vpath_index`, and `*path_index` as for
/// [`selective_vpath_search`]; returns null if not found.
#[no_mangle]
pub unsafe extern "C" fn vpath_search(
    file: *const c_char,
    mtime_ptr: *mut uintmax_t,
    vpath_index: *mut c_uint,
    path_index: *mut c_uint,
) -> *const c_char {
    // Absolute names need no vpath search.
    if *file == '/' as c_char || (vpaths.is_null() && general_vpath.is_null()) {
        return null();
    }

    if !vpath_index.is_null() {
        *vpath_index = 0;
        *path_index = 0;
    }

    let mut v = vpaths;
    while !v.is_null() {
        if pattern_matches((*v).pattern, (*v).percent, file) != 0 {
            let p = selective_vpath_search(v, file, mtime_ptr, path_index);
            if !p.is_null() {
                return p;
            }
        }
        if !vpath_index.is_null() {
            *vpath_index += 1;
        }
        v = (*v).next;
    }

    if !general_vpath.is_null() {
        let p = selective_vpath_search(general_vpath, file, mtime_ptr, path_index);
        if !p.is_null() {
            return p;
        }
    }

    null()
}

/// Print the data base of VPATH search paths.
#[no_mangle]
pub unsafe fn print_vpath_data_base() {
    puts(b"\n# VPATH Search Paths\n\0".as_ptr() as *const c_char);

    let mut nvpaths: c_uint = 0;
    let mut v = vpaths;
    while !v.is_null() {
        nvpaths += 1;
        printf(b"vpath %s \0".as_ptr() as *const c_char, (*v).pattern);
        print_search_path((*v).searchpath);
        v = (*v).next;
    }

    if vpaths.is_null() {
        puts(b"# No 'vpath' search paths.\0".as_ptr() as *const c_char);
    } else {
        printf(
            b"\n# %u 'vpath' search paths.\n\0".as_ptr() as *const c_char,
            nvpaths,
        );
    }

    if general_vpath.is_null() {
        puts(b"\n# No general ('VPATH' variable) search path.\0".as_ptr() as *const c_char);
    } else {
        fputs(
            b"\n# General ('VPATH' variable) search path:\n# \0".as_ptr() as *const c_char,
            stdout,
        );
        print_search_path((*general_vpath).searchpath);
    }
}

/// Print a null-terminated array of directory names separated by the path
/// separator, ending with a newline.
unsafe fn print_search_path(path: *mut *const c_char) {
    let mut i: isize = 0;
    while !(*path.offset(i)).is_null() {
        let sep = if (*path.offset(i + 1)).is_null() {
            '\n' as c_int
        } else {
            PATH_SEPARATOR_CHAR
        };
        printf(b"%s%c\0".as_ptr() as *const c_char, *path.offset(i), sep);
        i += 1;
    }
}
