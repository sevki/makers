//! Directory-contents cache: remembers which files exist in each
//! directory (keyed by device/inode so symlinked paths share an entry)
//! plus "impossible" targets that make tried and failed to build, and
//! serves glob() from the cache via [`dir_setup_glob`].
//!
//! Port of `dir.c`.

pub use crate::ffi_types::{__ino_t, __off_t, __size_t, dev_t, ino_t, size_t, time_t};
use crate::floc::Floc;
use crate::make_main::db_level;
use crate::misc::xrealloc;
use crate::output::{fatal_err, FmtArg};
use crate::strcache::strcache_add_len;

use ::core::ffi::{c_char, c_long, c_uchar, c_uint, c_ushort, c_void};
use ::core::ptr::{null, null_mut};
use rustc_hash::FxHashMap;

use libc::{__errno_location, closedir, memcpy, opendir, readdir, strerror, strlen, DIR, EINTR};

pub use crate::sys_stat::{stat, timespec};

extern "C" {
    fn stat(file: *const c_char, buf: *mut stat) -> i32;
    fn lstat(file: *const c_char, buf: *mut stat) -> i32;
}

/// `glob_t` as laid out by gnulib's glob with `GLOB_ALTDIRFUNC` support;
/// only the callback fields matter to [`dir_setup_glob`].
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut c_char,
    pub gl_offs: __size_t,
    pub gl_flags: i32,
    pub gl_closedir: Option<unsafe extern "C" fn(*mut c_void)>,
    pub gl_readdir: Option<unsafe extern "C" fn(*mut c_void) -> *mut dirent>,
    pub gl_opendir: Option<unsafe extern "C" fn(*const c_char) -> *mut c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(*const c_char, *mut stat) -> i32>,
    pub gl_stat: Option<unsafe extern "C" fn(*const c_char, *mut stat) -> i32>,
}

/// `struct dirent` layout handed back to glob by [`read_dirstream`]; the
/// name is truncated/extended to the actual length, so this header is
/// what counts.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256],
}

/// A directory name; `contents` points to the dev/ino-keyed cache entry
/// shared by every name for the same directory.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory {
    pub name: *const c_char,
    /// `Options::command_count` when this name was last stat'd (used only when the
    /// directory could not be stat'd, so there is no `contents`).
    pub counter: u64,
    pub contents: *mut DirectoryContents,
}

/// One cached directory entry: the file's `d_type` plus whether it is an
/// "impossible" target (make tried and failed to build it). The name is the
/// [`DirectoryContents::dirfiles`] map key, so it is not stored here.
#[derive(Copy, Clone, Debug)]
pub struct DirFileEntry {
    pub type_0: c_uchar,
    pub impossible: bool,
}

/// The actual cached contents of a directory, keyed by device and inode.
///
/// `dirfiles` is an idiomatic [`FxHashMap`] from a directory entry's name bytes
/// (no NUL) to its [`DirFileEntry`], replacing the c2rust FFI `HashTable` and
/// its `dirfile_hash_*` callbacks. `None` means the directory could not be
/// opened (the former null `ht_vec`); `Some` (even empty) means it was.
pub struct DirectoryContents {
    pub dev: dev_t,
    pub ino: ino_t,
    pub dirfiles: Option<FxHashMap<Box<[u8]>, DirFileEntry>>,
    /// `Options::command_count` when the contents were last read.
    pub counter: u64,
    /// Open stream while the directory is still being read lazily.
    pub dirstream: *mut DIR,
}

/// Glob cursor handed out by `open_dirstream`: an owned snapshot of the
/// directory's non-impossible entries (name + `d_type`) taken once when the
/// stream opens, plus the index of the next one to yield. Snapshotting keeps
/// `read_dirstream` O(1) per call (O(N) total) instead of re-walking the cache
/// map every call, and decouples the cursor from the cache's lifetime. It is
/// `Box`-allocated and freed by [`close_dirstream`] (not libc `free`), so it
/// can own heap data.
pub struct DirStream {
    pub entries: Vec<(Box<[u8]>, c_uchar)>,
    pub index: usize,
}

/// `DB_VERBOSE`: `-d`-style debug output enabled in `db_level`.
const DB_VERBOSE: i32 = 0x2;

pub const MAX_OPEN_DIRECTORIES: i32 = 10;

/// Forget everything cached about `dc`, closing its stream if open.
fn clear_directory_contents(ctx: &crate::execctx::ExecContext, dc: &mut DirectoryContents) {
    dc.counter = 0;
    if !dc.dirstream.is_null() {
        ctx.open_directories.set(ctx.open_directories.get() - 1);
        // SAFETY: `dirstream` is non-null here and was returned by `opendir`.
        unsafe { closedir(dc.dirstream) };
        dc.dirstream = null_mut();
    }
    // Drop any cached entries; the next `find_directory` reopens the stream
    // and installs a fresh map.
    dc.dirfiles = None;
}

/// Look up (or create) the cache entry for directory `name`, refreshing
/// it when a new command has run since it was cached.
///
/// # Safety
///
/// `name` must be a NUL-terminated path.
pub unsafe fn find_directory(
    ctx: &crate::execctx::ExecContext,
    name: *const c_char,
) -> Result<*mut directory, crate::build_result::BuildError> {
    // Look the directory up by its name bytes in the idiomatic `FxHashMap`
    // cache on the context.
    let key: Box<[u8]> = ::core::ffi::CStr::from_ptr(name).to_bytes().into();
    let dir: *mut directory = {
        let mut table = ctx.directories.0.borrow_mut();
        if let Some(boxed) = table.get_mut(&key) {
            // Cache hit: still valid unless a command has run since. Prefer the
            // shared contents counter: another name for the same directory may
            // have refreshed it already this command.
            let ctr = match boxed.contents.as_ref() {
                Some(dc) => dc.counter,
                None => boxed.counter,
            };
            if ctr == crate::make_main::opt_command_count(ctx) {
                // Valid hit. The `Box` keeps the entry at a stable heap address,
                // so this raw pointer outlives the released map borrow.
                return Ok((&mut **boxed) as *mut directory);
            }
            if DB_VERBOSE & db_level(ctx) != 0 {
                crate::output::trace_parts(&[
                    b"Directory ",
                    ::core::ffi::CStr::from_ptr(name).to_bytes(),
                    b" cache invalidated (count ",
                    ctr.to_string().as_bytes(),
                    b" != command ",
                    crate::make_main::opt_command_count(ctx)
                        .to_string()
                        .as_bytes(),
                    b")\n",
                ]);
            }
            if let Some(contents) = boxed.contents.as_mut() {
                clear_directory_contents(ctx, contents);
            }
            (&mut **boxed) as *mut directory
        } else {
            let len = strlen(name);
            // SAFETY: a zeroed `directory` is a valid (inert) entry — null
            // `contents`, zero `counter`; we set its interned name immediately.
            let mut new: Box<directory> = Box::new(::core::mem::zeroed());
            new.name = strcache_add_len(ctx, name, len);
            let boxed = table.entry(key).or_insert(new);
            (&mut **boxed) as *mut directory
        }
    };
    let dir_ref = dir.as_mut().expect("directory entry just selected");
    dir_ref.contents = null_mut();
    dir_ref.counter = crate::make_main::opt_command_count(ctx);

    let mut st: stat = ::core::mem::zeroed();
    let mut r;
    loop {
        r = stat(name, &raw mut st);
        if !(r == -1 && *__errno_location() == EINTR) {
            break;
        }
    }
    if r < 0 {
        // Couldn't stat the directory; leave a contents-less entry.
        return Ok(dir);
    }

    // Directory contents are shared across names via the dev/ino key, held in
    // the idiomatic `FxHashMap` cache on the context.
    let dev = st.st_dev as dev_t;
    let ino = st.st_ino as ino_t;
    let dc: *mut DirectoryContents = {
        let mut table = ctx.directory_contents.0.borrow_mut();
        let entry = table.entry((dev, ino)).or_insert_with(|| {
            // Freshly created, matching the former `xcalloc`: no file map yet
            // (`None`), no open stream, zero `counter`.
            Box::new(DirectoryContents {
                dev,
                ino,
                dirfiles: None,
                counter: 0,
                dirstream: null_mut(),
            })
        });
        // The `Box` keeps the contents at a stable heap address across later
        // map inserts/rehashes, so this raw pointer (stored in `directory.contents`
        // and handed to the glob dirstream) stays valid for the run.
        (&mut **entry) as *mut DirectoryContents
    };
    let dc = dc.as_mut().expect("DirectoryContents entry just selected");
    dir_ref.contents = dc;

    if dc.counter != crate::make_main::opt_command_count(ctx) {
        if dc.counter != 0 {
            clear_directory_contents(ctx, dc);
        }
        dc.counter = crate::make_main::opt_command_count(ctx);
        loop {
            *__errno_location() = 0;
            dc.dirstream = opendir(name);
            if !(dc.dirstream.is_null() && *__errno_location() == EINTR) {
                break;
            }
        }
        if dc.dirstream.is_null() {
            // Unreadable: cache that fact with no file map.
            dc.dirfiles = None;
        } else {
            dc.dirfiles = Some(FxHashMap::default());
            ctx.open_directories.set(ctx.open_directories.get() + 1);
            if ctx.open_directories.get() == MAX_OPEN_DIRECTORIES as u32 {
                // Too many streams open: read this one to completion now.
                // `map` rather than `?`: the entry is fully built either way,
                // so the read's verdict is threaded out without branching here.
                return dir_contents_file_exists_p(ctx, dir, null()).map(|_| dir);
            }
        }
    }
    Ok(dir)
}

/// Does `filename` exist in `dir`? Reads the directory incrementally,
/// caching every entry seen; a null `filename` reads to the end.
unsafe fn dir_contents_file_exists_p(
    ctx: &crate::execctx::ExecContext,
    dir: *mut directory,
    filename: *const c_char,
) -> Result<i32, crate::build_result::BuildError> {
    let dir = dir.as_ref().expect("dir_contents_file_exists_p: null dir");
    let Some(dc) = dir.contents.as_mut() else {
        // The directory could not be stat'd.
        return Ok(0);
    };
    if dc.dirfiles.is_none() {
        // The directory could not be opened.
        return Ok(0);
    }

    if !filename.is_null() {
        let key = ::core::ffi::CStr::from_ptr(filename).to_bytes();
        if key.is_empty() {
            // Checking for the directory itself; it exists.
            return Ok(1);
        }
        if let Some(entry) = dc.dirfiles.as_ref().and_then(|m| m.get(key)) {
            return Ok((!entry.impossible) as i32);
        }
    }

    if dc.dirstream.is_null() {
        // The directory has been read in full and the name wasn't there.
        return Ok(0);
    }

    // Keep reading entries (caching each one) until we hit the name or
    // exhaust the directory.
    let mut d: *mut libc::dirent;
    loop {
        loop {
            *__errno_location() = 0;
            d = readdir(dc.dirstream);
            if !(d.is_null() && *__errno_location() == EINTR) {
                break;
            }
        }
        let Some(entry) = d.as_mut() else {
            if *__errno_location() != 0 {
                // The scan is abandoned mid-directory: the stream stays open
                // and cached entries stay cached, exactly as they were before
                // this failure, so a later scan of the same directory resumes
                // where this one stopped.
                return Err(fatal_err(
                    ctx,
                    null::<Floc>(),
                    0,
                    c"readdir %s: %s".as_ptr(),
                    &[
                        FmtArg::Str(dir.name),
                        FmtArg::Str(strerror(*__errno_location())),
                    ],
                ));
            }
            break;
        };
        if entry.d_ino == 0 {
            continue;
        }

        let d_name = entry.d_name.as_mut_ptr();
        let name = ::core::ffi::CStr::from_ptr(d_name).to_bytes();
        // Insert (overwriting), matching the C `hash_insert_at`: actually seeing
        // the file during a scan clears any stale `impossible` marker a prior
        // `file_impossible` recorded for the same name.
        dc.dirfiles
            .as_mut()
            .expect("dirfiles is Some when reading")
            .insert(
                Box::from(name),
                DirFileEntry {
                    type_0: entry.d_type,
                    impossible: false,
                },
            );
        // Early exit once we have cached the name we were asked about.
        if !filename.is_null() && name == ::core::ffi::CStr::from_ptr(filename).to_bytes() {
            return Ok(1);
        }
    }

    // Reached the end of the directory: the stream is exhausted.
    if d.is_null() {
        ctx.open_directories.set(ctx.open_directories.get() - 1);
        closedir(dc.dirstream);
        dc.dirstream = null_mut();
    }
    Ok(0)
}

/// Does `filename` exist in directory `dirname`?
///
/// # Safety
///
/// Both must be NUL-terminated; the directory tables must be initialized.
pub unsafe fn dir_file_exists_p(
    ctx: &crate::execctx::ExecContext,
    dirname: *const c_char,
    filename: *const c_char,
) -> Result<i32, crate::build_result::BuildError> {
    // `and_then` rather than `?`: the lookup's verdict is the whole function,
    // so threading it through keeps this frame branch-free.
    find_directory(ctx, dirname).and_then(|d| dir_contents_file_exists_p(ctx, d, filename))
}

/// Compute how `name` splits at its final slash.
///
/// Returns `(dirname, base_offset)` where `dirname` is the directory part as
/// a NUL-terminated byte buffer (a lone leading slash becomes `/`) and the
/// basename begins at `name[base_offset]`. Returns `None` when there is no
/// slash. Pure: operates purely on the byte view, with no pointer state.
fn split_dir_parts(name: &[u8]) -> Option<(Vec<u8>, usize)> {
    let slash = name.iter().rposition(|&b| b == b'/')?;
    let mut dirname = if slash == 0 {
        b"/".to_vec()
    } else {
        name[..slash].to_vec()
    };
    dirname.push(0);
    Some((dirname, slash + 1))
}

/// Split `name` at its final slash, returning the owned NUL-terminated
/// `dirname` buffer and the basename as a `&CStr` borrowed from `name`.
/// `None` when `name` has no slash.
fn split_dir(name: &::core::ffi::CStr) -> Option<(Vec<u8>, &::core::ffi::CStr)> {
    let (dirname, base_off) = split_dir_parts(name.to_bytes())?;
    // The basename runs from `base_off` to `name`'s own terminator, so the tail
    // of `to_bytes_with_nul` is itself a valid NUL-terminated C string (a `&CStr`
    // borrowing `name`) — no pointer arithmetic needed.
    let base = ::core::ffi::CStr::from_bytes_with_nul(&name.to_bytes_with_nul()[base_off..])
        .expect("base_off indexes within name, whose only NUL is its terminator");
    Some((dirname, base))
}

/// Does file `name` (with optional directory part) exist?
///
/// # Safety
///
/// `name` must be NUL-terminated; the directory tables must be
/// initialized.
pub unsafe fn file_exists_p(
    ctx: &crate::execctx::ExecContext,
    name: *const c_char,
) -> Result<i32, crate::build_result::BuildError> {
    // `and_then` rather than `?`: the classification's verdict feeds straight
    // into the arm choice, so the seam costs this frame no decision point.
    crate::ar::ar_name_err(ctx, ::core::ffi::CStr::from_ptr(name)).and_then(|is_ar| {
        if is_ar {
            ar_member_exists_p(ctx, name)
        } else {
            dir_lookup_exists_p(ctx, name)
        }
    })
}

/// Does the plain (non-archive) path `name` exist, per the directory cache?
/// Split out of [`file_exists_p`] so its two arms stay one decision each.
///
/// # Safety
/// As [`file_exists_p`].
unsafe fn dir_lookup_exists_p(
    ctx: &crate::execctx::ExecContext,
    name: *const c_char,
) -> Result<i32, crate::build_result::BuildError> {
    match split_dir(::core::ffi::CStr::from_ptr(name)) {
        None => dir_file_exists_p(ctx, c".".as_ptr(), name),
        Some((dirname, base)) => dir_file_exists_p(ctx, dirname.as_ptr().cast(), base.as_ptr()),
    }
}

/// Does the `archive(member)` reference `name` name an existing member?
/// Split out of [`file_exists_p`] so the archive arm's `Result` seam does not
/// add a decision point to the directory-lookup path.
///
/// # Safety
/// As [`file_exists_p`].
unsafe fn ar_member_exists_p(
    ctx: &crate::execctx::ExecContext,
    name: *const c_char,
) -> Result<i32, crate::build_result::BuildError> {
    crate::ar::ar_member_date(ctx, name).map(|d| (d != -1) as i32)
}

/// Record that `filename` is an impossible target: make tried to build it
/// and couldn't, so don't consider it again this command.
///
/// # Safety
///
/// `filename` must be NUL-terminated; the directory tables must be
/// initialized.
/// Split `filename` at its final slash and look the directory part up,
/// returning it alongside the base name. Split out of [`file_impossible`] so
/// both arms share one lookup and the `Result` seam costs that frame no
/// decision point.
///
/// # Safety
/// `filename` must be NUL-terminated; the directory tables must be initialized.
unsafe fn split_for_directory(
    ctx: &crate::execctx::ExecContext,
    filename: *const c_char,
) -> Result<(*mut directory, *const c_char), crate::build_result::BuildError> {
    match split_dir(::core::ffi::CStr::from_ptr(filename)) {
        None => find_directory(ctx, c".".as_ptr()).map(|d| (d, filename)),
        Some((dirname, base)) => {
            find_directory(ctx, dirname.as_ptr().cast()).map(|d| (d, base.as_ptr()))
        }
    }
}

/// Record that `filename` is an impossible target: make tried to build it
/// and couldn't, so don't consider it again this command.
///
/// # Safety
///
/// `filename` must be NUL-terminated; the directory tables must be
/// initialized.
pub unsafe fn file_impossible(
    ctx: &crate::execctx::ExecContext,
    filename: *const c_char,
) -> Result<(), crate::build_result::BuildError> {
    let (dir, filename) = split_for_directory(ctx, filename)?;
    let dir = dir.as_mut().expect("find_directory never returns null");

    if dir.contents.is_null() {
        // The directory was never stat'd or couldn't be; create a standalone
        // contents entry just to hold impossible names. It is not in the dev/ino
        // table (there is no stat to key it by), so leak it like the former
        // `xcalloc` did — the cache lives for the whole run.
        dir.contents = Box::into_raw(Box::new(DirectoryContents {
            dev: 0,
            ino: 0,
            dirfiles: None,
            counter: 0,
            dirstream: null_mut(),
        }));
    }
    let dc = dir.contents.as_mut().expect("just ensured non-null");
    let key = ::core::ffi::CStr::from_ptr(filename).to_bytes();
    // First record wins, matching the FFI table's no-replace insert: an
    // existing (real) entry for this name is left untouched.
    dc.dirfiles
        .get_or_insert_with(FxHashMap::default)
        .entry(Box::from(key))
        .or_insert(DirFileEntry {
            type_0: 0,
            impossible: true,
        });
    Ok(())
}

/// Has `filename` been recorded as impossible?
///
/// # Safety
///
/// `filename` must be NUL-terminated; the directory tables must be
/// initialized.
/// The cached contents of directory `name`, or null when it has none. Split
/// out of [`file_impossible_p`] so both arms share one lookup.
///
/// # Safety
/// `name` must be NUL-terminated; the directory tables must be initialized.
unsafe fn dir_contents_of(
    ctx: &crate::execctx::ExecContext,
    name: *const c_char,
) -> Result<*mut DirectoryContents, crate::build_result::BuildError> {
    find_directory(ctx, name).map(|d| d.as_ref().map_or(::core::ptr::null_mut(), |d| d.contents))
}

/// Has `filename` been recorded as impossible?
///
/// # Safety
///
/// `filename` must be NUL-terminated; the directory tables must be
/// initialized.
pub unsafe fn file_impossible_p(
    ctx: &crate::execctx::ExecContext,
    filename: *const c_char,
) -> Result<i32, crate::build_result::BuildError> {
    // `and_then` rather than `?`: the lookup's verdict is the whole function,
    // so threading it through keeps this frame branch-free.
    split_for_contents(ctx, filename).map(|(dir, base)| impossible_flag(dir, base))
}

/// Split `filename` at its final slash and return the directory part's cached
/// contents (null when it has none) alongside the base name. Split out of
/// [`file_impossible_p`] so both arms share one lookup.
///
/// # Safety
/// As [`file_impossible_p`].
unsafe fn split_for_contents(
    ctx: &crate::execctx::ExecContext,
    filename: *const c_char,
) -> Result<(*mut DirectoryContents, *const c_char), crate::build_result::BuildError> {
    match split_dir(::core::ffi::CStr::from_ptr(filename)) {
        None => dir_contents_of(ctx, c".".as_ptr()).map(|d| (d, filename)),
        Some((dirname, base)) => {
            dir_contents_of(ctx, dirname.as_ptr().cast()).map(|d| (d, base.as_ptr()))
        }
    }
}

/// Is `filename` marked impossible in `dir`'s cached contents?
///
/// # Safety
/// `dir` must be null or a live contents entry; `filename` NUL-terminated.
unsafe fn impossible_flag(dir: *mut DirectoryContents, filename: *const c_char) -> i32 {
    let Some(dir) = dir.as_mut() else { return 0 };
    let key = ::core::ffi::CStr::from_ptr(filename).to_bytes();
    match dir.dirfiles.as_ref().and_then(|m| m.get(key)) {
        Some(entry) => entry.impossible as i32,
        None => 0,
    }
}

/// Return the canonical (interned) name for directory `dir`.
///
/// # Safety
///
/// `dir` must be NUL-terminated; the directory tables must be
/// initialized.
pub unsafe fn dir_name(
    ctx: &crate::execctx::ExecContext,
    dir: *const c_char,
) -> Result<*const c_char, crate::build_result::BuildError> {
    find_directory(ctx, dir).map(|d| d.as_ref().expect("find_directory never returns null").name)
}

/// Print `n`, or `word` when `n` is zero (the "No files" / "no
/// impossibilities" phrasing in the data base dump).
fn print_count(n: c_uint, zero_word: &[u8]) {
    if n == 0 {
        crate::output::trace_out(zero_word);
    } else {
        crate::output::trace_out(n.to_string().as_bytes());
    }
}

/// Print the directory cache for `make -p`.
///
/// # Safety
///
/// The directory tables must be initialized.
pub unsafe fn print_dir_data_base(ctx: &crate::execctx::ExecContext) {
    crate::output::trace_out(b"\n# Directories\n\n");

    let mut files: c_uint = 0;
    let mut impossible: c_uint = 0;
    // Borrow the name-keyed table; it is not mutated while printing.
    let table = ctx.directories.0.borrow();
    for boxed in table.values() {
        let dir: &directory = boxed;
        if dir.contents.is_null() {
            crate::output::trace_parts(&[
                b"# ",
                ::core::ffi::CStr::from_ptr(dir.name).to_bytes(),
                b": could not be stat'd.\n",
            ]);
            continue;
        }
        let dc = dir.contents.as_ref().expect("checked non-null above");
        let Some(dirfiles) = dc.dirfiles.as_ref() else {
            crate::output::trace_parts(&[
                b"# ",
                ::core::ffi::CStr::from_ptr(dir.name).to_bytes(),
                b" (device ",
                (dc.dev as c_long).to_string().as_bytes(),
                b", inode ",
                (dc.ino as c_long).to_string().as_bytes(),
                b"): could not be opened.\n",
            ]);
            continue;
        };

        let mut f: c_uint = 0;
        let mut im: c_uint = 0;
        for entry in dirfiles.values() {
            if entry.impossible {
                im += 1;
            } else {
                f += 1;
            }
        }
        crate::output::trace_parts(&[
            b"# ",
            ::core::ffi::CStr::from_ptr(dir.name).to_bytes(),
            b" (device ",
            (dc.dev as c_long).to_string().as_bytes(),
            b", inode ",
            (dc.ino as c_long).to_string().as_bytes(),
            b"): ",
        ]);
        print_count(f, b"No");
        crate::output::trace_out(b" files, ");
        print_count(im, b"no");
        crate::output::trace_out(b" impossibilities");
        if dc.dirstream.is_null() {
            crate::output::trace_out(b".\n");
        } else {
            crate::output::trace_out(b" so far.\n");
        }
        files += f;
        impossible += im;
    }

    crate::output::trace_out(b"\n# ");
    print_count(files, b"No");
    crate::output::trace_out(b" files, ");
    print_count(impossible, b"no");
    crate::output::trace_parts(&[
        b" impossibilities in ",
        (table.len() as u64).to_string().as_bytes(),
        b" directories.\n",
    ]);
}

/// glob `opendir` callback: position a cursor over the cached contents of
/// `directory`, reading it to completion first.
///
/// Safe to call: the C glob machinery invokes it through a function pointer, and
/// the directory-cache FFI it drives is confined to the inner `unsafe` block.
extern "C" fn open_dirstream(directory: *const c_char) -> *mut c_void {
    // This is a glob `gl_opendir` callback invoked by the C glob machinery; its
    // C-ABI signature cannot carry the owned `ExecContext`. The directory cache
    // it populates lives on that context, so we reach the live per-run context
    // through the `CTX_PTR` borrow channel (installed for the extent of
    // `main_0`), exactly as `with_options` does for `Options`.
    // `open_dirstream` returns `*mut c_void` to a C caller: there is no Rust
    // frame between here and the glob machinery to carry a `Result`, so the
    // whole body's verdict bridges once, here, rather than at each fallible
    // call inside it. This is the one site in this cone that keeps bridging
    // (#432 Phase B, #539).
    crate::make_main::with_exec_context(|ctx| unsafe { open_dirstream_cached(ctx, directory) })
        .unwrap_or_else(|e| crate::output::exit_on_err(e))
}

/// The body of [`open_dirstream`], with the fallible directory-cache reads
/// left propagating so the callback's C boundary is the only bridge.
///
/// # Safety
/// As [`open_dirstream`]: `directory` must be NUL-terminated.
unsafe fn open_dirstream_cached(
    ctx: &crate::execctx::ExecContext,
    directory: *const c_char,
) -> Result<*mut c_void, crate::build_result::BuildError> {
    {
        let dir = find_directory(ctx, directory)?
            .as_mut()
            .expect("find_directory never returns null");
        let Some(dc) = dir.contents.as_mut() else {
            // The directory could not be stat'd.
            return Ok(null_mut());
        };
        if dc.dirfiles.is_none() {
            // The directory could not be opened.
            return Ok(null_mut());
        }
        // Read it all in now so the cache is complete.
        dir_contents_file_exists_p(ctx, &raw mut *dir, null())?;

        // Snapshot the non-impossible entries once. The cache is fully read and
        // is not mutated during the glob, so a flat `Vec` lets `read_dirstream`
        // advance in O(1) per call (O(N) total) and frees it from the cache's
        // lifetime.
        let entries: Vec<(Box<[u8]>, c_uchar)> = dc
            .dirfiles
            .as_ref()
            .expect("dirfiles is Some (checked above)")
            .iter()
            .filter(|(_, e)| !e.impossible)
            .map(|(name, e)| (name.clone(), e.type_0))
            .collect();
        Ok(Box::into_raw(Box::new(DirStream { entries, index: 0 })).cast())
    }
}

/// glob `readdir` callback: synthesize a `dirent` for the next cached
/// (non-impossible) file.
///
/// Safe to call: the C glob machinery invokes it through a function pointer with
/// a `stream` it obtained from `open_dirstream`, and the pointer work is
/// confined to the inner `unsafe` block. The returned dirent lives in the
/// per-run context's reused scratch buffer that the next call overwrites.
pub extern "C" fn read_dirstream(stream: *mut c_void) -> *mut dirent {
    // The reused dirent scratch buffer (the former process-global `static mut
    // buf`/`bufsz`) lives on the per-run `ExecContext`. This glob `gl_readdir`
    // callback's C-ABI signature cannot carry an `&ExecContext`, so it reaches
    // the live context through the `CTX_PTR` borrow channel, exactly as the
    // sibling `gl_opendir` callback `open_dirstream` does.
    crate::make_main::with_exec_context(|ctx| unsafe {
        let ds = (stream as *mut DirStream)
            .as_mut()
            .expect("read_dirstream: null stream");

        // O(1): index straight into the snapshot taken by `open_dirstream`.
        let Some((name, type_0)) = ds.entries.get(ds.index) else {
            return null_mut();
        };
        ds.index += 1;

        // Grow the dirent buffer to hold the name plus its NUL (the d_name
        // field's declared 256 bytes are replaced by the real length).
        let len = name.len() + 1;
        let sz = ::core::mem::size_of::<dirent>() as size_t
            - ::core::mem::size_of::<[c_char; 256]>() as size_t
            + len;
        if sz > ctx.read_dirstream_bufsz.get() {
            let bufsz = (ctx.read_dirstream_bufsz.get() * 2).max(sz);
            ctx.read_dirstream_bufsz.set(bufsz);
            ctx.read_dirstream_buf
                .set(xrealloc(ctx.read_dirstream_buf.get() as *mut c_void, bufsz) as *mut c_char);
        }
        let d = (ctx.read_dirstream_buf.get() as *mut dirent)
            .as_mut()
            .expect("xrealloc never returns null");
        d.d_ino = 1;
        d.d_type = *type_0;
        memcpy(
            d.d_name.as_mut_ptr().cast(),
            name.as_ptr() as *const c_void,
            name.len(),
        );
        // NUL-terminate (the snapshot name has no terminator).
        *d.d_name.as_mut_ptr().add(name.len()) = 0;
        &raw mut *d
    })
}

/// glob `closedir` callback: free the cursor [`open_dirstream`] allocated.
///
/// Replaces the former `gl_closedir = free`: the snapshot owns heap data, so it
/// must be dropped through `Box`, not `free`d as a POD.
extern "C" fn close_dirstream(stream: *mut c_void) {
    if stream.is_null() {
        return;
    }
    // SAFETY: a non-null `stream` was produced by `Box::into_raw` in
    // `open_dirstream`; glob hands each one back here exactly once.
    drop(unsafe { Box::from_raw(stream as *mut DirStream) });
}

/// Point `gl` at the directory cache so glob() reads from it instead of
/// the filesystem.
///
/// # Safety
///
/// `gl` must be a valid `glob_t` matching this layout.
pub unsafe fn dir_setup_glob(gl: *mut glob_t) {
    let gl = gl.as_mut().expect("dir_setup_glob: null glob_t");
    gl.gl_offs = 0;
    gl.gl_opendir = Some(open_dirstream);
    gl.gl_readdir = Some(read_dirstream);
    gl.gl_closedir = Some(close_dirstream);
    gl.gl_lstat = Some(lstat);
    gl.gl_stat = Some(stat);
}

#[cfg(test)]
mod split_dir_tests {
    use super::split_dir_parts;

    #[test]
    fn no_slash_returns_none() {
        assert_eq!(split_dir_parts(b"foo.c"), None);
        assert_eq!(split_dir_parts(b""), None);
    }

    #[test]
    fn leading_slash_only_becomes_root() {
        // "/foo": the only slash is at index 0, so the dirname is "/".
        assert_eq!(split_dir_parts(b"/foo"), Some((b"/\0".to_vec(), 1)));
    }

    #[test]
    fn nested_path_splits_at_final_slash() {
        // "a/b/c": final slash at index 3, dirname "a/b", base offset 4.
        assert_eq!(split_dir_parts(b"a/b/c"), Some((b"a/b\0".to_vec(), 4)));
    }

    #[test]
    fn trailing_slash_yields_empty_basename_offset() {
        // "dir/": slash at index 3, base offset 4 points at the NUL.
        assert_eq!(split_dir_parts(b"dir/"), Some((b"dir\0".to_vec(), 4)));
    }

    #[test]
    fn absolute_nested_keeps_leading_slash_in_dirname() {
        // "/usr/bin": final slash at index 4, dirname "/usr", base offset 5.
        assert_eq!(split_dir_parts(b"/usr/bin"), Some((b"/usr\0".to_vec(), 5)));
    }

    /// The safe `split_dir` wrapper returns the owned NUL-terminated `dirname`
    /// buffer and the basename as a `&CStr` borrowed from `name` (the former
    /// `name.add(base_off)` pointer), `None` when there is no slash.
    #[test]
    fn split_dir_yields_dirname_buffer_and_borrowed_basename() {
        use super::split_dir;
        assert!(split_dir(c"foo.c").is_none());

        let (dir, base) = split_dir(c"a/b/c").unwrap();
        assert_eq!(dir, b"a/b\0");
        assert_eq!(base.to_bytes(), b"c");

        let (dir, base) = split_dir(c"/foo").unwrap();
        assert_eq!(dir, b"/\0");
        assert_eq!(base.to_bytes(), b"foo");

        // Trailing slash: the basename is the empty string at the terminator.
        let (dir, base) = split_dir(c"dir/").unwrap();
        assert_eq!(dir, b"dir\0");
        assert_eq!(base.to_bytes(), b"");
    }
}

#[cfg(test)]
mod split_dir_unsafe_oracle {
    //! `split_dir` was an `unsafe fn` over `*const c_char`; this keeps the
    //! verbatim c2rust-era pointer implementation as a differential oracle and
    //! asserts the safe `&CStr` version produces the identical dirname buffer
    //! and basename bytes (AGENTS rule 3).
    use super::{split_dir, split_dir_parts};
    use ::core::ffi::{c_char, CStr};

    /// Verbatim pre-conversion implementation: returns the owned dirname buffer,
    /// a pointer into it, and the basename via `name.add(base_off)`.
    unsafe fn oracle(name: *const c_char) -> Option<(Vec<u8>, *const c_char, *const c_char)> {
        let (buf, base_off) = split_dir_parts(CStr::from_ptr(name).to_bytes())?;
        let dirname = buf.as_ptr() as *const c_char;
        Some((buf, dirname, name.add(base_off)))
    }

    /// Drive both implementations over `name` and assert identical results.
    fn check(name: &CStr) {
        let safe = split_dir(name);
        // SAFETY: `name` is a valid NUL-terminated C string.
        let oracle_res = unsafe { oracle(name.as_ptr()) };
        match (safe, oracle_res) {
            (None, None) => {}
            (Some((safe_dir, safe_base)), Some((oracle_dir, _dirp, basep))) => {
                assert_eq!(safe_dir, oracle_dir, "dirname for {name:?}");
                // SAFETY: `basep` points into `name`, a valid NUL-terminated string.
                let oracle_base = unsafe { CStr::from_ptr(basep) };
                assert_eq!(
                    safe_base.to_bytes(),
                    oracle_base.to_bytes(),
                    "basename for {name:?}"
                );
            }
            (s, o) => panic!(
                "split_dir disagreed on {name:?}: safe.is_some()={}, oracle.is_some()={}",
                s.is_some(),
                o.is_some()
            ),
        }
    }

    #[test]
    fn differential() {
        // No slash (None), nested, root, absolute-nested, and trailing-slash
        // (empty basename at the terminator).
        check(c"foo.c");
        check(c"a/b/c");
        check(c"/foo");
        check(c"/usr/bin");
        check(c"dir/");
        check(c"/");
        check(c"a/b/");
    }
}

#[cfg(test)]
mod open_directories_tests {
    use crate::execctx::ExecContext;

    /// The open-stream counter now lives on `ExecContext` as a `Cell<u32>`
    /// (no global). A fresh context starts at zero, and the `+= 1`/`-= 1`
    /// stream open/close bookkeeping that `find_directory` /
    /// `dir_contents_file_exists_p` / `clear_directory_contents` perform maps
    /// onto `get()`/`set()` exactly as the former `static mut` did.
    #[test]
    fn counter_tracks_open_and_close() {
        let ctx = ExecContext::default();
        assert_eq!(ctx.open_directories.get(), 0);

        ctx.open_directories.set(ctx.open_directories.get() + 1);
        ctx.open_directories.set(ctx.open_directories.get() + 1);
        assert_eq!(ctx.open_directories.get(), 2);

        ctx.open_directories.set(ctx.open_directories.get() - 1);
        assert_eq!(ctx.open_directories.get(), 1);

        // A second, independent context keeps its own count.
        assert_eq!(ExecContext::default().open_directories.get(), 0);
    }
}
