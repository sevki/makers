//! Searching the `vpath` / `VPATH` / `GPATH` directory search paths.
//!
//! Port of `vpath.c`. The search-path data structures are still C-shaped
//! (`xmalloc`-owned, nul-terminated string arrays handed out by the string
//! cache) because they are shared with `read.rs`, `remake.rs`, and
//! `implicit.rs` through `extern "C"` boundaries.

use ::core::ffi::{c_char, c_uint, c_void, CStr};
use ::core::ptr::{null, null_mut};

use libc::{__errno_location, free, strcmp, strlen};

use crate::dir::{dir_file_exists_p, dir_name};
use crate::expand::expand_variable_buf;
use crate::ffi_types::{size_t, uintmax_t};
use crate::file::{file_timestamp_cons, lookup_file, system_time_from_unix};
use crate::function::pattern_matches;
use crate::make_main::stopchar_map;
use crate::misc::xmalloc;
use crate::read::find_percent;
use crate::strcache::{strcache_add, strcache_add_len};
use crate::sys_stat::stat;

extern "C" {
    fn stat(file: *const c_char, buf: *mut stat) -> i32;
}

/// Character-class bits in `stopchar_map` (see `makeint.h`).
const MAP_BLANK: i32 = 0x0002;
const MAP_NEWLINE: i32 = 0x0004;
const MAP_COLON: i32 = 0x0040;
const MAP_SPACE: i32 = MAP_BLANK | MAP_NEWLINE;
/// On POSIX the search-path separator is `:`.
const MAP_PATHSEP: i32 = MAP_COLON;
const PATH_SEPARATOR_CHAR: i32 = ':' as i32;

/// `STOP_SET (c, mask)` from `makeint.h`: is `c` in any of the character
/// classes selected by `mask`?
fn stop_set(c: u8, mask: i32) -> bool {
    stopchar_map()[c as usize] as i32 & mask != 0
}

/// Borrow a NUL-terminated C string as a byte slice (without the NUL).
unsafe fn cstr_bytes<'a>(s: *const c_char) -> &'a [u8] {
    ::core::slice::from_raw_parts(s as *const u8, strlen(s))
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
pub struct Vpath {
    next: *mut Vpath,
    /// The pattern to match, in the string cache.
    pattern: *const c_char,
    /// Pointer into `pattern` at the `%`, or null if there is none.
    percent: *const c_char,
    patlen: size_t,
    /// Array of cached directory names, owned via `xmalloc`. It carries a
    /// trailing null entry for legacy reasons, but `npaths` is authoritative.
    searchpath: *mut *const c_char,
    /// Number of directory entries in `searchpath`.
    npaths: size_t,
    /// Length of the longest entry in `searchpath`.
    maxlen: size_t,
}

/// The `searchpath` directory entries of a `Vpath` as a slice.
unsafe fn searchpath_entries<'a>(path: *const Vpath) -> &'a [*const c_char] {
    ::core::slice::from_raw_parts((*path).searchpath as *const *const c_char, (*path).npaths)
}

/// The byte offset of the `%` within a pattern, or `None` when there is none.
/// The `%`, when present, points into `pattern`, so its position is the
/// difference of the two byte lengths — recovered here with safe slice
/// arithmetic rather than raw address subtraction. Used to compare the `%`
/// positions of two patterns during de-duplication.
fn percent_off(pattern: &CStr, percent: Option<&CStr>) -> Option<usize> {
    percent.map(|p| pattern.to_bytes().len() - p.to_bytes().len())
}

/// Reverse the chain of vpath directives and build the `VPATH`/`GPATH`
/// pseudo-vpaths from the variables' current values.
///
/// # Safety
/// Must run single-threaded: it reads and writes the module's vpath chains
/// and expands make variables through the global variable tables.
pub unsafe fn build_vpath_lists(
    ctx: &crate::execctx::ExecContext,
) -> Result<(), crate::build_result::BuildError> {
    reverse_vpath_chain(ctx);
    install_variable_vpaths(ctx)
}

/// Reverse the directive-built chain so vpaths are searched in the order their
/// directives appeared in the makefile.
///
/// # Safety
/// The chain links must be valid and the caller single-threaded with respect
/// to them.
unsafe fn reverse_vpath_chain(ctx: &crate::execctx::ExecContext) {
    let mut reversed: *mut Vpath = null_mut();
    let mut old = ctx.vpaths.0.get();
    while !old.is_null() {
        let next = (*old).next;
        (*old).next = reversed;
        reversed = old;
        old = next;
    }
    ctx.vpaths.0.set(reversed);
}

/// Build the `VPATH`/`GPATH` chains from the variables of those names. Split
/// from [`build_vpath_lists`] so the `Result` seam #442 introduced sits in one
/// small function rather than adding branches to the chain reversal above.
///
/// # Safety
/// As [`build_vpath_lists`].
unsafe fn install_variable_vpaths(
    ctx: &crate::execctx::ExecContext,
) -> Result<(), crate::build_result::BuildError> {
    if let Some(list) = vpath_from_variable(ctx, b"VPATH\0")? {
        ctx.general_vpath.0.set(list);
    }
    if let Some(list) = vpath_from_variable(ctx, b"GPATH\0")? {
        ctx.gpaths.0.set(list);
    }
    Ok(())
}

/// Expand the named make variable and build a `%`-pattern vpath chain from
/// its value, leaving the directive-built `vpaths` chain untouched. Returns
/// `None` when the value is empty or whitespace-only.
unsafe fn vpath_from_variable(
    ctx: &crate::execctx::ExecContext,
    name: &[u8],
) -> Result<Option<*mut Vpath>, crate::build_result::BuildError> {
    // `map` rather than `?`: everything downstream of the expansion is in
    // `vpath_chain_from_expansion`, so the `Result` seam #442 introduced does
    // not add a branch to the chain-building logic itself.
    expand_variable_buf(
        ctx,
        null_mut(),
        name.as_ptr() as *const c_char,
        (name.len() - 1) as size_t,
    )
    .and_then(|p| vpath_chain_from_expansion(ctx, p))
}

/// Build a `%`-pattern vpath chain from an already-expanded search path.
/// Returns `None` when the value is empty or whitespace-only.
///
/// # Safety
/// `p` must be a live NUL-terminated buffer inside the variable buffer, and
/// the caller must be single-threaded with respect to the vpath chains.
unsafe fn vpath_chain_from_expansion(
    ctx: &crate::execctx::ExecContext,
    p: *mut c_char,
) -> Result<Option<*mut Vpath>, crate::build_result::BuildError> {
    // The expansion lives in a mutable buffer that construct_vpath_list may
    // overwrite in place; view it (plus its NUL) as a byte slice.
    let len = strlen(p);
    let buf = ::core::slice::from_raw_parts_mut(p as *mut u8, len + 1);
    // Skip leading whitespace; an all-whitespace (or empty) value is ignored.
    let start = buf[..len]
        .iter()
        .position(|&c| !stop_set(c, MAP_SPACE))
        .unwrap_or(len);
    if start == len {
        return Ok(None);
    }
    let saved = ctx.vpaths.0.get();
    ctx.vpaths.0.set(null_mut());
    let mut pattern = *b"%\0";
    // The saved chain is restored before a rejection escapes, so a refused
    // `dir_name` never leaves the module's vpath list swapped out.
    let built = construct_vpath_list(
        ctx,
        pattern.as_mut_ptr() as *mut c_char,
        buf[start..].as_mut_ptr() as *mut c_char,
    );
    let list = ctx.vpaths.0.get();
    ctx.vpaths.0.set(saved);
    built?;
    Ok(Some(list))
}

/// Construct the `Vpath` listing for the pattern and search path given.
///
/// If `dirpath` is null, remove all previous listings with the same pattern.
/// If `pattern` is null too, remove all `Vpath` listings. The existing
/// chains' contents are not freed beyond the listing structures themselves,
/// since the string-cache strings may still be referenced elsewhere.
///
/// # Safety
/// `pattern` and `dirpath` must be null or valid nul-terminated strings,
/// and the caller must be single-threaded with respect to the vpath chains.
pub unsafe fn construct_vpath_list(
    ctx: &crate::execctx::ExecContext,
    pattern: *mut c_char,
    dirpath: *mut c_char,
) -> Result<(), crate::build_result::BuildError> {
    let percent: *const c_char = if pattern.is_null() {
        null()
    } else {
        find_percent(pattern)
    };

    if dirpath.is_null() {
        // Remove matching listings from the chain.
        let mut lastpath: *mut Vpath = null_mut();
        let mut path = ctx.vpaths.0.get();
        while !path.is_null() {
            let next = (*path).next;
            let matches = pattern.is_null()
                || (percent_off(
                    CStr::from_ptr(pattern),
                    (!percent.is_null()).then(|| CStr::from_ptr(percent)),
                ) == percent_off(
                    CStr::from_ptr((*path).pattern),
                    (!(*path).percent.is_null()).then(|| CStr::from_ptr((*path).percent)),
                ) && strcmp(pattern, (*path).pattern) == 0);
            if matches {
                // Unlink and free this entry.
                if let Some(lp) = lastpath.as_mut() {
                    lp.next = next;
                } else {
                    ctx.vpaths.0.set(next);
                }
                free((*path).searchpath as *mut c_void);
                free(path as *mut c_void);
            } else {
                lastpath = path;
            }
            path = next;
        }
        return Ok(());
    }

    // Tokenize the search path into its directory entries.
    let bytes = cstr_bytes(dirpath);
    let mut entries: Vec<*const c_char> = Vec::new();
    let mut maxvpath: size_t = 0;
    let mut i = 0;
    // Skip leading separators and blanks.
    while i < bytes.len() && stop_set(bytes[i], MAP_BLANK | MAP_PATHSEP) {
        i += 1;
    }
    while i < bytes.len() {
        // Find the end of this entry.
        let start = i;
        while i < bytes.len()
            && bytes[i] as i32 != PATH_SEPARATOR_CHAR
            && !stop_set(bytes[i], MAP_BLANK)
        {
            i += 1;
        }
        let mut len = i - start;
        // Omit a trailing slash, unless the entry is just "/".
        if len > 1 && bytes[start + len - 1] == b'/' {
            len -= 1;
        }
        // Skip "." entries: searching "." is implicit.
        if len > 1 || bytes[start] != b'.' {
            let cached =
                strcache_add_len(ctx, bytes[start..].as_ptr() as *const c_char, len as size_t);
            entries.push(dir_name(ctx, cached)?);
            if len as size_t > maxvpath {
                maxvpath = len as size_t;
            }
        }
        // Skip over separators and blanks between entries.
        while i < bytes.len() && stop_set(bytes[i], MAP_BLANK | MAP_PATHSEP) {
            i += 1;
        }
    }

    if entries.is_empty() {
        // There were no entries; forget the whole thing.
        return Ok(());
    }

    // Copy the gathered entries into an xmalloc'd, null-terminated array
    // (freed with free() when the listing is later removed).
    let n = entries.len();
    let searchpath =
        xmalloc((n + 1) * ::core::mem::size_of::<*const c_char>()) as *mut *const c_char;
    let slots = ::core::slice::from_raw_parts_mut(searchpath, n + 1);
    slots[..n].copy_from_slice(&entries);
    slots[n] = null();

    let path = xmalloc(::core::mem::size_of::<Vpath>()) as *mut Vpath;
    (*path).searchpath = searchpath;
    (*path).npaths = n;
    (*path).maxlen = maxvpath;
    (*path).next = ctx.vpaths.0.get();
    ctx.vpaths.0.set(path);
    (*path).pattern = strcache_add(ctx, pattern);
    (*path).patlen = strlen(pattern);
    // `find_percent` already unquoted `pattern` in place, and the cached copy
    // is byte-identical, so the `%` sits at the same offset. Reuse that
    // offset rather than re-parsing: a second `find_percent` pass would
    // re-unquote the string and mutate the shared cache entry.
    (*path).percent = if percent.is_null() {
        null()
    } else {
        let off = percent as usize - pattern as usize;
        cstr_bytes((*path).pattern)[off..].as_ptr() as *const c_char
    };
    Ok(())
}

/// Search the `GPATH` list for a pathname (`file` of length `len`, which is
/// not null-terminated). Returns `true` if it is there, `false` if not.
pub fn gpath_search(ctx: &crate::execctx::ExecContext, file: &[u8]) -> bool {
    // SAFETY: `gpaths` is only read here; the slices it hands out
    // (`searchpath_entries`, `cstr_bytes`) borrow that still-C-shaped data
    // for the duration of the comparison.
    unsafe {
        let gpaths = ctx.gpaths.0.get();
        if !gpaths.is_null() && file.len() <= (*gpaths).maxlen {
            for &entry in searchpath_entries(gpaths) {
                // The GPATH entry must equal exactly the first `len` bytes.
                if cstr_bytes(entry) == file {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod percent_off_unsafe_oracle {
    use super::*;
    use std::ffi::CString;

    /// Verbatim pre-conversion implementation, preserved as a differential
    /// oracle: recovers the `%` offset by raw address subtraction.
    fn percent_off_oracle(pattern: *const c_char, percent: *const c_char) -> Option<usize> {
        if percent.is_null() {
            None
        } else {
            Some(percent as usize - pattern as usize)
        }
    }

    /// The safe slice-length form recovers the same offset as the original
    /// raw-pointer subtraction, for a `%` at varying positions and for the
    /// no-`%` (null) case.
    #[test]
    fn safe_matches_oracle() {
        for pat in [
            &b"%.o"[..],
            &b"src/%.o"[..],
            &b"a/very/long/path/%/tail"[..],
            &b"no-percent-here"[..],
            &b"%"[..],
        ] {
            let c = CString::new(pat).unwrap();
            let base = c.as_ptr();
            // Mirror find_percent: the first unquoted '%', or null.
            let pct_ptr = match pat.iter().position(|&b| b == b'%') {
                Some(i) => unsafe { base.add(i) },
                None => ::core::ptr::null(),
            };

            let oracle = percent_off_oracle(base, pct_ptr);
            let safe = percent_off(
                c.as_c_str(),
                (!pct_ptr.is_null()).then(|| unsafe { CStr::from_ptr(pct_ptr) }),
            );
            assert_eq!(safe, oracle, "pattern {:?}", String::from_utf8_lossy(pat));
        }
    }

    /// Spot-check the absolute offsets the safe form yields.
    #[test]
    fn offsets_are_correct() {
        let c = CString::new("src/%.o").unwrap();
        let pct = c.as_bytes().iter().position(|&b| b == b'%').unwrap();
        let percent = unsafe { CStr::from_ptr(c.as_ptr().add(pct)) };
        assert_eq!(percent_off(c.as_c_str(), Some(percent)), Some(4));
        assert_eq!(percent_off(c.as_c_str(), None), None);
    }
}

#[cfg(test)]
mod gpath_search_unsafe_oracle {
    use super::*;

    /// Verbatim pre-conversion implementation, preserved as a differential
    /// oracle. Operates on a raw pointer + length and returns `i32`.
    unsafe fn gpath_search_oracle(
        ctx: &crate::execctx::ExecContext,
        file: *const c_char,
        len: size_t,
    ) -> i32 {
        let gpaths = ctx.gpaths.0.get();
        if !gpaths.is_null() && len <= (*gpaths).maxlen {
            let needle = ::core::slice::from_raw_parts(file as *const u8, len);
            for &entry in searchpath_entries(gpaths) {
                // The GPATH entry must equal exactly the first `len` bytes.
                if cstr_bytes(entry) == needle {
                    return 1;
                }
            }
        }
        0
    }

    /// Build a `Vpath` whose `searchpath` holds the given NUL-terminated
    /// entries and publish it as `ctx.gpaths` for the test. The backing
    /// storage is leaked deliberately so it outlives the comparison.
    fn install_gpaths(ctx: &crate::execctx::ExecContext, entries: &[&[u8]]) {
        let mut ptrs: Vec<*const c_char> = entries
            .iter()
            .map(|e| {
                let mut buf = e.to_vec();
                buf.push(0);
                Box::leak(buf.into_boxed_slice()).as_ptr() as *const c_char
            })
            .collect();
        let maxlen = entries.iter().map(|e| e.len()).max().unwrap_or(0);
        let npaths = ptrs.len();
        let searchpath = ptrs.as_mut_ptr();
        Box::leak(ptrs.into_boxed_slice());
        let vp = Box::new(Vpath {
            next: null_mut(),
            pattern: null(),
            percent: null(),
            patlen: 0,
            searchpath,
            npaths,
            maxlen,
        });
        ctx.gpaths.0.set(Box::leak(vp));
    }

    #[test]
    fn safe_matches_oracle() {
        let ctx = crate::execctx::ExecContext::default();
        install_gpaths(&ctx, &[b"src", b"include", b"a/b/c"]);
        let cases: &[&[u8]] = &[
            b"src",
            b"include",
            b"a/b/c",
            b"a/b",         // shorter than an entry, not a match
            b"srcx",        // longer, exceeds maxlen check or differs
            b"",            // empty needle
            b"includ",      // prefix only
            b"a/b/c/d/e/f", // longer than maxlen
        ];
        for &needle in cases {
            let safe = gpath_search(&ctx, needle);
            let oracle = unsafe {
                gpath_search_oracle(&ctx, needle.as_ptr() as *const c_char, needle.len())
            };
            assert_eq!(
                safe,
                oracle != 0,
                "mismatch for {:?}",
                ::core::str::from_utf8(needle)
            );
        }
    }
}

/// Search the given `Vpath` list for a directory where `file` exists. If it
/// is found, return the cached full pathname, storing the file's modtime
/// into `*mtime_ptr` (when non-null) and the index of the matching search
/// path into `*path_index` (when non-null). Returns null if not found.
unsafe fn selective_vpath_search(
    ctx: &crate::execctx::ExecContext,
    path: *mut Vpath,
    file: *const c_char,
    mut mtime_ptr: *mut uintmax_t,
    path_index: *mut c_uint,
) -> Result<*const c_char, crate::build_result::BuildError> {
    let maxvpath = (*path).maxlen;

    // If and only if *FILE is NOT a target, accept prospective files that
    // don't exist but are mentioned in a makefile.
    let not_target = match lookup_file(ctx, cstr_bytes(file)) {
        Some(id) => {
            let node = ctx.filenodes.get(id).expect("looked-up file is interned");
            let g = node.lock().expect("file node lock poisoned");
            !g.is_target
        }
        None => true,
    };

    // Split *FILE into a directory prefix and a name-within-directory:
    // NAME_DPLEN is the length of the prefix, FNAME_START indexes the
    // name-within-directory, and FLEN is its length.
    let file_bytes = cstr_bytes(file);
    let (name_dplen, fname_start) = match file_bytes.iter().rposition(|&b| b == b'/') {
        Some(slash) => (slash, slash + 1),
        None => (0, 0),
    };
    let filename = file_bytes[fname_start..].as_ptr() as *const c_char;
    let fname_bytes = &file_bytes[fname_start..];
    let flen = fname_bytes.len();

    // Scratch buffer with room for the biggest VPATH entry, a slash, the
    // directory prefix that came with *FILE, another slash (not always
    // needed), the filename, and a null terminator.
    let mut name_buf = vec![0u8; maxvpath + 1 + name_dplen + 1 + flen + 1];

    // Try each VPATH entry.
    for (i, &entry) in searchpath_entries(path).iter().enumerate() {
        let entry_b = cstr_bytes(entry);

        // Lay down "<entry>[/<dirprefix>]/<filename>\0" into the scratch buffer;
        // P is the index of the separator before the filename.
        let p = lay_down_vpath_name(
            &mut name_buf,
            entry_b,
            &file_bytes,
            name_dplen,
            fname_bytes,
            flen,
        );
        let name = name_buf.as_mut_ptr() as *mut c_char;

        // Check whether the file is mentioned in a makefile. If *FILE is
        // not a target, that is enough for us to decide this file exists.
        // If *FILE is a target, the file must also be mentioned as a target
        // to be chosen.
        let mut exists = false;
        if let Some(id) = lookup_file(ctx, cstr_bytes(name)) {
            let node = ctx.filenodes.get(id).expect("looked-up file is interned");
            let g = node.lock().expect("file node lock poisoned");
            exists = not_target || g.is_target;
            // Preserve the special -W / -o timestamps.
            if exists && (g.last_mtime == OLD_MTIME || g.last_mtime == NEW_MTIME) {
                if let Some(slot) = mtime_ptr.as_mut() {
                    *slot = g.last_mtime;
                    mtime_ptr = null_mut();
                }
            }
        }

        let mut exists_in_cache = false;
        if !exists {
            // The file wasn't mentioned in the makefile. Clobber a null into
            // NAME at the last slash, so NAME is the directory to look in
            // (the directory cache knows it already), and ask the cache
            // whether the file exists there.
            name_buf[p] = 0;
            exists = dir_file_exists_p(ctx, name, filename)? != 0;
            exists_in_cache = exists;
        }

        if exists {
            // Put the slash back in NAME.
            name_buf[p] = b'/';

            if exists_in_cache {
                // The directory cache may be out of date; check that the
                // file really exists in the filesystem, because higher
                // levels get confused otherwise.
                let mut st: stat = ::core::mem::zeroed();
                let mut e: i32;
                loop {
                    e = stat(name, &mut st);
                    if !(e == -1 && *__errno_location() == libc::EINTR) {
                        break;
                    }
                }
                if e != 0 {
                    // Stale cache entry: keep searching the remaining vpath
                    // entries instead of returning it.
                    exists = false;
                } else if let Some(slot) = mtime_ptr.as_mut() {
                    *slot = file_timestamp_cons(
                        ctx,
                        name,
                        system_time_from_unix(st.st_mtim.tv_sec as i64, st.st_mtim.tv_nsec as u32),
                    );
                    mtime_ptr = null_mut();
                }
            }
        }

        if exists {
            // We found a file. If mtime_ptr wasn't set above, record
            // UNKNOWN_MTIME to say so.
            if let Some(slot) = mtime_ptr.as_mut() {
                *slot = UNKNOWN_MTIME;
            }
            if let Some(slot) = path_index.as_mut() {
                *slot = i as c_uint;
            }
            return Ok(strcache_add_len(ctx, name, (p + 1 + flen) as size_t));
        }
    }

    Ok(null())
}

/// Compose `"<entry>[/<dirprefix>]/<filename>\0"` into `name_buf` and return the
/// index of the separator before the filename (the c2rust `p`). Split out of
/// [`selective_vpath_search`] so its prefix/separator branching lives on its own.
fn lay_down_vpath_name(
    name_buf: &mut [u8],
    entry_b: &[u8],
    file_bytes: &[u8],
    name_dplen: usize,
    fname_bytes: &[u8],
    flen: usize,
) -> usize {
    let mut p = entry_b.len();
    name_buf[..p].copy_from_slice(entry_b);
    if name_dplen > 0 {
        name_buf[p] = b'/';
        p += 1;
        name_buf[p..p + name_dplen].copy_from_slice(&file_bytes[..name_dplen]);
        p += name_dplen;
    }
    // Now add the name-within-directory at the end of NAME.
    if p != 0 && name_buf[p - 1] != b'/' {
        name_buf[p] = b'/';
        name_buf[p + 1..p + 1 + flen].copy_from_slice(fname_bytes);
        name_buf[p + 1 + flen] = 0;
    } else {
        name_buf[p..p + flen].copy_from_slice(fname_bytes);
        name_buf[p + flen] = 0;
    }
    p
}

/// Search the VPATH list whose pattern matches `file` for a directory where
/// `file` exists. On success returns the cached full pathname and fills
/// `*mtime_ptr`, `*vpath_index`, and `*path_index` as for
/// [`selective_vpath_search`]; returns null if not found.
///
/// # Safety
/// `file` must be a valid nul-terminated string; `mtime_ptr`, `vpath_index`,
/// and `path_index` must each be null or valid for writes. When
/// `vpath_index` is non-null, `path_index` must be non-null too.
pub unsafe fn vpath_search(
    ctx: &crate::execctx::ExecContext,
    file: *const c_char,
    mtime_ptr: *mut uintmax_t,
    vpath_index: *mut c_uint,
    path_index: *mut c_uint,
) -> Result<*const c_char, crate::build_result::BuildError> {
    // Absolute names need no vpath search.
    let file_ref = file
        .as_ref()
        .expect("vpath_search requires a non-null file");
    let vpaths = ctx.vpaths.0.get();
    let general_vpath = ctx.general_vpath.0.get();
    if *file_ref == '/' as c_char || (vpaths.is_null() && general_vpath.is_null()) {
        return Ok(null());
    }

    if !vpath_index.is_null() {
        *vpath_index = 0;
        // The contract pairs a non-null `vpath_index` with a non-null
        // `path_index`; write through a checked reference so the deref is
        // validated rather than assumed.
        if let Some(slot) = path_index.as_mut() {
            *slot = 0;
        }
    }

    let mut v = vpaths;
    while !v.is_null() {
        if pattern_matches((*v).pattern, (*v).percent, file) != 0 {
            let p = selective_vpath_search(ctx, v, file, mtime_ptr, path_index)?;
            if !p.is_null() {
                return Ok(p);
            }
        }
        if !vpath_index.is_null() {
            *vpath_index += 1;
        }
        v = (*v).next;
    }

    if !general_vpath.is_null() {
        let p = selective_vpath_search(ctx, general_vpath, file, mtime_ptr, path_index)?;
        if !p.is_null() {
            return Ok(p);
        }
    }

    Ok(null())
}

/// Print the data base of VPATH search paths.
///
/// # Safety
/// Must run single-threaded: it reads the module's vpath chains.
pub unsafe fn print_vpath_data_base(ctx: &crate::execctx::ExecContext) {
    crate::output::trace_out(b"\n# VPATH Search Paths\n\n");

    let vpaths = ctx.vpaths.0.get();
    let general_vpath = ctx.general_vpath.0.get();
    let mut nvpaths: c_uint = 0;
    let mut v = vpaths;
    while !v.is_null() {
        nvpaths += 1;
        crate::output::trace_parts(&[b"vpath ", CStr::from_ptr((*v).pattern).to_bytes(), b" "]);
        print_search_path(v);
        v = (*v).next;
    }

    if vpaths.is_null() {
        crate::output::trace_out(b"# No 'vpath' search paths.\n");
    } else {
        crate::output::trace_parts(&[
            b"\n# ",
            nvpaths.to_string().as_bytes(),
            b" 'vpath' search paths.\n",
        ]);
    }

    if general_vpath.is_null() {
        crate::output::trace_out(b"\n# No general ('VPATH' variable) search path.\n");
    } else {
        crate::output::trace_out(b"\n# General ('VPATH' variable) search path:\n# ");
        print_search_path(general_vpath);
    }
}

/// Print a `Vpath`'s directory entries separated by the path separator,
/// ending with a newline.
unsafe fn print_search_path(path: *const Vpath) {
    let entries = searchpath_entries(path);
    for (idx, &entry) in entries.iter().enumerate() {
        let sep = if idx + 1 == entries.len() {
            b'\n'
        } else {
            PATH_SEPARATOR_CHAR as u8
        };
        crate::output::trace_parts(&[CStr::from_ptr(entry).to_bytes(), &[sep]]);
    }
}

#[cfg(test)]
mod vpath_variable_rejection_tests {
    //! Since #442 `vpath_from_variable` and `build_vpath_lists` return
    //! `Result`: a `VPATH`/`GPATH` value that cannot be expanded comes back as
    //! a rejection instead of ending the process from inside the vpath setup.

    use super::{build_vpath_lists, vpath_from_variable};
    use crate::build_result::BuildError;
    use std::ffi::CString;

    /// Define `name` as a recursive global variable holding `value`.
    ///
    /// # Safety
    /// `ctx` must have its global variable set initialized.
    unsafe fn define_recursive(ctx: &crate::execctx::ExecContext, name: &str, value: &str) {
        let cname = CString::new(name).unwrap();
        let cvalue = CString::new(value).unwrap();
        crate::variable::define_variable_in_set(
            ctx,
            cname.as_ptr(),
            name.len() as crate::ffi_types::size_t,
            cvalue.as_ptr(),
            crate::variable::o_file,
            1,
            ctx.variable_globals.global_variable_set.as_ptr(),
            ::core::ptr::null::<crate::floc::Floc>(),
        )
        .expect("test fixture defines a well-formed name");
    }

    fn fresh_ctx() -> crate::execctx::ExecContext {
        crate::make_main::initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();
        // SAFETY: fresh context; each table is initialized once.
        unsafe {
            crate::function::hash_init_function_table(&ctx);
            crate::variable::init_hash_global_variable_set(&ctx);
        }
        ctx
    }

    /// `$(word 1)` is a builtin called with the wrong number of arguments, so
    /// expanding it is refused — and the refusal now travels out of the vpath
    /// setup rather than exiting.
    #[test]
    fn rejected_vpath_value_propagates() {
        let ctx = fresh_ctx();
        // SAFETY: NUL-terminated names; single-threaded fresh context.
        unsafe {
            define_recursive(&ctx, "VPATH", "$(word 1)");
            assert!(matches!(
                vpath_from_variable(&ctx, b"VPATH\0"),
                Err(BuildError::Failure)
            ));
            assert!(
                matches!(build_vpath_lists(&ctx), Err(BuildError::Failure)),
                "the rejection must reach `build_vpath_lists`, which `main_0` \
                 propagates"
            );
        }
    }

    /// A well-formed value still builds its chain, and an empty or
    /// whitespace-only one is ignored rather than treated as a failure.
    #[test]
    fn well_formed_and_empty_values_still_build() {
        let ctx = fresh_ctx();
        // SAFETY: as above.
        unsafe {
            define_recursive(&ctx, "VPATH", "   ");
            assert!(
                vpath_from_variable(&ctx, b"VPATH\0")
                    .expect("well-formed")
                    .is_none(),
                "a whitespace-only value yields no chain"
            );

            define_recursive(&ctx, "GPATH", "src:include");
            assert!(
                vpath_from_variable(&ctx, b"GPATH\0")
                    .expect("well-formed")
                    .is_some(),
                "a real search path yields a chain"
            );
            assert!(build_vpath_lists(&ctx).is_ok());
        }
    }
}
