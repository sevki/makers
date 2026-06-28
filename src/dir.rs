//! Directory-contents cache: remembers which files exist in each
//! directory (keyed by device/inode so symlinked paths share an entry)
//! plus "impossible" targets that make tried and failed to build, and
//! serves glob() from the cache via [`dir_setup_glob`].
//!
//! Port of `dir.c`.

pub use crate::ffi_types::{__ino_t, __off_t, __size_t, dev_t, ino_t, size_t, time_t};
use crate::floc::Floc;
use crate::hash::{
    hash_find_item, hash_find_slot, hash_free, hash_init, hash_insert, hash_insert_at, hash_table,
    is_real_item, jhash_string,
};
use crate::make_main::db_level;
use crate::misc::{xcalloc, xmalloc, xrealloc};
use crate::output::{fatal, FmtArg};
use crate::stdio::FILE;
use crate::strcache::strcache_add_len;

use ::core::ffi::{c_char, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use ::core::ptr::{null, null_mut};

use libc::{
    __errno_location, closedir, free, memcpy, opendir, printf, puts, readdir, strcmp, strerror,
    strlen, DIR, EINTR,
};

pub use crate::sys_stat::{stat, timespec};

extern "C" {
    fn stat(file: *const c_char, buf: *mut stat) -> i32;
    fn lstat(file: *const c_char, buf: *mut stat) -> i32;
    static mut stdout: *mut FILE;
    fn fflush(stream: *mut FILE) -> i32;
    fn fputs(s: *const c_char, stream: *mut FILE) -> i32;
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
    pub counter: c_ulong,
    pub contents: *mut directory_contents,
}

/// The actual cached contents of a directory, keyed by device and inode.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory_contents {
    pub dev: dev_t,
    pub ino: ino_t,
    /// Table of [`dirfile`] entries (files seen plus impossible names).
    pub dirfiles: hash_table,
    /// `Options::command_count` when the contents were last read.
    pub counter: c_ulong,
    /// Open stream while the directory is still being read lazily.
    pub dirstream: *mut DIR,
}

/// One cached directory entry (or impossible target name).
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirfile {
    pub name: *const c_char,
    pub length: size_t,
    pub impossible: c_short,
    pub type_0: c_uchar,
}

/// Glob cursor handed out by `open_dirstream`.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirstream {
    pub contents: *mut directory_contents,
    pub dirfile_slot: *mut *mut dirfile,
}

/// `DB_VERBOSE`: `-d`-style debug output enabled in `db_level`.
const DB_VERBOSE: i32 = 0x2;

pub const MAX_OPEN_DIRECTORIES: i32 = 10;
pub const DIRFILE_BUCKETS: i32 = 107;

/// Forget everything cached about `dc`, closing its stream if open.
fn clear_directory_contents(ctx: &crate::execctx::ExecContext, dc: &mut directory_contents) {
    dc.counter = 0;
    if !dc.dirstream.is_null() {
        ctx.open_directories.set(ctx.open_directories.get() - 1);
        // SAFETY: `dirstream` is non-null here and was returned by `opendir`.
        unsafe { closedir(dc.dirstream) };
        dc.dirstream = null_mut();
    }
    if !dc.dirfiles.ht_vec.is_null() {
        // SAFETY: `dirfiles` is an initialized hash table owned by `dc`.
        unsafe { hash_free(&raw mut dc.dirfiles, 1) };
    }
}

/// Hash a [`dirfile`] key by name.
///
/// # Safety
///
/// `key` must point to a `dirfile` whose name is NUL-terminated.
pub unsafe fn dirfile_hash_1(key: *const c_void) -> c_ulong {
    let key = (key as *const dirfile)
        .as_ref()
        .expect("hash callback got a null key");
    jhash_string(::core::ffi::CStr::from_ptr(key.name).to_bytes()) as c_ulong
}

/// Secondary hash for [`dirfile`] keys; always zero, kept for the
/// callback ABI. Never dereferences `key`; any pointer value is acceptable.
pub fn dirfile_hash_2(_key: *const c_void) -> c_ulong {
    0
}

/// Order two directory-entry names the way the dirfile hash table expects:
/// shorter names sort first, and names of equal length sort by their raw bytes.
///
/// This is the safe, pointer-free core of [`dirfile_hash_cmp`]. The C original
/// compared `length` first and only fell back to a `strcmp` when the lengths
/// matched; for two equal-length, NUL-terminated names that is exactly
/// `len`-then-bytes ordering, since the `strcmp` ranks them by their first
/// differing byte.
fn dirfile_cmp(x_name: &[u8], y_name: &[u8]) -> ::core::cmp::Ordering {
    x_name
        .len()
        .cmp(&y_name.len())
        .then_with(|| x_name.cmp(y_name))
}

unsafe fn dirfile_hash_cmp(xv: *const c_void, yv: *const c_void) -> i32 {
    let x: &dirfile = &*(xv as *const dirfile);
    let y: &dirfile = &*(yv as *const dirfile);
    let x_name = ::core::slice::from_raw_parts(x.name as *const u8, x.length);
    let y_name = ::core::slice::from_raw_parts(y.name as *const u8, y.length);
    // `Ordering` is `#[repr(i8)]` with `Less = -1`, `Equal = 0`, `Greater = 1`,
    // so the cast reproduces the C callback's tri-state result without a branch.
    dirfile_cmp(x_name, y_name) as i32
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
) -> *mut directory {
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
            if ctr == crate::make_main::opt_command_count() {
                // Valid hit. The `Box` keeps the entry at a stable heap address,
                // so this raw pointer outlives the released map borrow.
                return (&mut **boxed) as *mut directory;
            }
            if DB_VERBOSE & db_level != 0 {
                printf(
                    c"Directory %s cache invalidated (count %lu != command %lu)\n".as_ptr(),
                    name,
                    ctr,
                    crate::make_main::opt_command_count(),
                );
                fflush(stdout);
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
            new.name = strcache_add_len(name, len);
            let boxed = table.entry(key).or_insert(new);
            (&mut **boxed) as *mut directory
        }
    };
    let dir_ref = dir.as_mut().expect("directory entry just selected");
    dir_ref.contents = null_mut();
    dir_ref.counter = crate::make_main::opt_command_count();

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
        return dir;
    }

    // Directory contents are shared across names via the dev/ino key, held in
    // the idiomatic `FxHashMap` cache on the context.
    let dev = st.st_dev as dev_t;
    let ino = st.st_ino as ino_t;
    let dc: *mut directory_contents = {
        let mut table = ctx.directory_contents.0.borrow_mut();
        let entry = table.entry((dev, ino)).or_insert_with(|| {
            // An all-zero `directory_contents` is the valid "freshly created"
            // state the former `xcalloc` produced: null `dirfiles` vec, null
            // `dirstream`, zero `counter`.
            // SAFETY: `directory_contents` is a `repr(C)` POD whose all-zero bit
            // pattern is a valid value (matching the C `xcalloc`).
            let mut dc: Box<directory_contents> = Box::new(::core::mem::zeroed());
            dc.dev = dev;
            dc.ino = ino;
            dc
        });
        // The `Box` keeps the contents at a stable heap address across later
        // map inserts/rehashes, so this raw pointer (stored in `directory.contents`
        // and handed to the glob dirstream) stays valid for the run.
        (&mut **entry) as *mut directory_contents
    };
    let dc = dc.as_mut().expect("directory_contents entry just selected");
    dir_ref.contents = dc;

    if dc.counter != crate::make_main::opt_command_count() {
        if dc.counter != 0 {
            clear_directory_contents(ctx, dc);
        }
        dc.counter = crate::make_main::opt_command_count();
        loop {
            *__errno_location() = 0;
            dc.dirstream = opendir(name);
            if !(dc.dirstream.is_null() && *__errno_location() == EINTR) {
                break;
            }
        }
        if dc.dirstream.is_null() {
            // Unreadable: cache that fact with a null file table.
            dc.dirfiles.ht_vec = null_mut();
        } else {
            hash_init(
                &raw mut dc.dirfiles,
                DIRFILE_BUCKETS as c_ulong,
                Some(dirfile_hash_1),
                Some(dirfile_hash_2),
                Some(dirfile_hash_cmp),
            );
            ctx.open_directories.set(ctx.open_directories.get() + 1);
            if ctx.open_directories.get() == MAX_OPEN_DIRECTORIES as u32 {
                // Too many streams open: read this one to completion now.
                dir_contents_file_exists_p(ctx, dir, null());
            }
        }
    }
    dir
}

/// Does `filename` exist in `dir`? Reads the directory incrementally,
/// caching every entry seen; a null `filename` reads to the end.
unsafe fn dir_contents_file_exists_p(
    ctx: &crate::execctx::ExecContext,
    dir: *mut directory,
    filename: *const c_char,
) -> i32 {
    let dir = dir.as_ref().expect("dir_contents_file_exists_p: null dir");
    let Some(dc) = dir.contents.as_mut() else {
        // The directory could not be stat'd.
        return 0;
    };
    if dc.dirfiles.ht_vec.is_null() {
        // The directory could not be opened.
        return 0;
    }

    if !filename.is_null() {
        if *filename == 0 {
            // Checking for the directory itself; it exists.
            return 1;
        }
        let mut dirfile_key: dirfile = ::core::mem::zeroed();
        dirfile_key.name = filename;
        dirfile_key.length = strlen(filename);
        let df =
            hash_find_item(&raw mut dc.dirfiles, (&raw const dirfile_key).cast()) as *const dirfile;
        if let Some(df) = df.as_ref() {
            return (df.impossible == 0) as i32;
        }
    }

    if dc.dirstream.is_null() {
        // The directory has been read in full and the name wasn't there.
        return 0;
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
                fatal(
                    ctx,
                    null::<Floc>(),
                    0,
                    c"readdir %s: %s".as_ptr(),
                    &[
                        FmtArg::Str(dir.name),
                        FmtArg::Str(strerror(*__errno_location())),
                    ],
                );
            }
            break;
        };
        if entry.d_ino == 0 {
            continue;
        }

        let d_name = entry.d_name.as_mut_ptr();
        let len = strlen(d_name);
        let mut dirfile_key: dirfile = ::core::mem::zeroed();
        dirfile_key.name = d_name;
        dirfile_key.length = len;
        let dirfile_slot = (hash_find_slot(&raw mut dc.dirfiles, (&raw const dirfile_key).cast())
            as *mut *mut dirfile)
            .as_mut()
            .expect("hash_find_slot always returns a slot");
        let df = (xmalloc(::core::mem::size_of::<dirfile>() as size_t) as *mut dirfile)
            .as_mut()
            .expect("xmalloc never returns null");
        df.name = strcache_add_len(d_name, len);
        df.type_0 = entry.d_type;
        df.length = len;
        df.impossible = 0;
        hash_insert_at(
            &raw mut dc.dirfiles,
            (&raw const *df).cast(),
            (&raw mut *dirfile_slot).cast(),
        );
        // streq-style early exit: first bytes match, then the rest.
        if !filename.is_null()
            && *d_name == *filename
            && (*d_name == 0 || strcmp(d_name.add(1), filename.add(1)) == 0)
        {
            return 1;
        }
    }

    // Reached the end of the directory: the stream is exhausted.
    if d.is_null() {
        ctx.open_directories.set(ctx.open_directories.get() - 1);
        closedir(dc.dirstream);
        dc.dirstream = null_mut();
    }
    0
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
) -> i32 {
    dir_contents_file_exists_p(ctx, find_directory(ctx, dirname), filename)
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
pub unsafe fn file_exists_p(ctx: &crate::execctx::ExecContext, name: *const c_char) -> i32 {
    if crate::ar::ar_name(ctx, ::core::ffi::CStr::from_ptr(name)) {
        return (crate::ar::ar_member_date(ctx, name) != -1) as i32;
    }
    match split_dir(::core::ffi::CStr::from_ptr(name)) {
        None => dir_file_exists_p(ctx, c".".as_ptr(), name),
        Some((dirname, base)) => dir_file_exists_p(ctx, dirname.as_ptr().cast(), base.as_ptr()),
    }
}

/// Record that `filename` is an impossible target: make tried to build it
/// and couldn't, so don't consider it again this command.
///
/// # Safety
///
/// `filename` must be NUL-terminated; the directory tables must be
/// initialized.
pub unsafe fn file_impossible(ctx: &crate::execctx::ExecContext, filename: *const c_char) {
    let (dir, filename) = match split_dir(::core::ffi::CStr::from_ptr(filename)) {
        None => (find_directory(ctx, c".".as_ptr()), filename),
        Some((dirname, base)) => (find_directory(ctx, dirname.as_ptr().cast()), base.as_ptr()),
    };
    let dir = dir.as_mut().expect("find_directory never returns null");

    if dir.contents.is_null() {
        // The directory was never stat'd or couldn't be; create a
        // contents entry just to hold impossible names.
        dir.contents = xcalloc(::core::mem::size_of::<directory_contents>() as size_t)
            as *mut directory_contents;
    }
    let dc = dir.contents.as_mut().expect("just ensured non-null");
    if dc.dirfiles.ht_vec.is_null() {
        hash_init(
            &raw mut dc.dirfiles,
            DIRFILE_BUCKETS as c_ulong,
            Some(dirfile_hash_1),
            Some(dirfile_hash_2),
            Some(dirfile_hash_cmp),
        );
    }

    let new = (xmalloc(::core::mem::size_of::<dirfile>() as size_t) as *mut dirfile)
        .as_mut()
        .expect("xmalloc never returns null");
    new.length = strlen(filename);
    new.name = strcache_add_len(filename, new.length);
    new.impossible = 1;
    hash_insert(&raw mut dc.dirfiles, (&raw const *new).cast());
}

/// Has `filename` been recorded as impossible?
///
/// # Safety
///
/// `filename` must be NUL-terminated; the directory tables must be
/// initialized.
pub unsafe fn file_impossible_p(ctx: &crate::execctx::ExecContext, filename: *const c_char) -> i32 {
    let (dir, filename) = match split_dir(::core::ffi::CStr::from_ptr(filename)) {
        None => {
            let dir_ptr = find_directory(ctx, c".".as_ptr());
            let contents = dir_ptr
                .as_ref()
                .map_or(::core::ptr::null_mut(), |d| d.contents);
            (contents, filename)
        }
        Some((dirname, base)) => {
            let dir_ptr = find_directory(ctx, dirname.as_ptr().cast());
            let contents = dir_ptr
                .as_ref()
                .map_or(::core::ptr::null_mut(), |d| d.contents);
            (contents, base.as_ptr())
        }
    };
    let Some(dir) = dir.as_mut() else { return 0 };
    if dir.dirfiles.ht_vec.is_null() {
        return 0;
    }

    let mut dirfile_key: dirfile = ::core::mem::zeroed();
    dirfile_key.name = filename;
    dirfile_key.length = strlen(filename);
    let df =
        hash_find_item(&raw mut dir.dirfiles, (&raw const dirfile_key).cast()) as *const dirfile;
    match df.as_ref() {
        Some(df) => df.impossible as i32,
        None => 0,
    }
}

/// Return the canonical (interned) name for directory `dir`.
///
/// # Safety
///
/// `dir` must be NUL-terminated; the directory tables must be
/// initialized.
pub unsafe fn dir_name(ctx: &crate::execctx::ExecContext, dir: *const c_char) -> *const c_char {
    find_directory(ctx, dir)
        .as_ref()
        .expect("find_directory never returns null")
        .name
}

/// Print `n`, or `word` when `n` is zero (the "No files" / "no
/// impossibilities" phrasing in the data base dump).
unsafe fn print_count(n: c_uint, zero_word: *const c_char) {
    if n == 0 {
        fputs(zero_word, stdout);
    } else {
        printf(c"%u".as_ptr(), n);
    }
}

/// Print the directory cache for `make -p`.
///
/// # Safety
///
/// The directory tables must be initialized and stdout valid.
pub unsafe fn print_dir_data_base(ctx: &crate::execctx::ExecContext) {
    puts(c"\n# Directories\n".as_ptr());

    let mut files: c_uint = 0;
    let mut impossible: c_uint = 0;
    // Borrow the name-keyed table; it is not mutated while printing.
    let table = ctx.directories.0.borrow();
    for boxed in table.values() {
        let dir: &directory = boxed;
        if dir.contents.is_null() {
            printf(c"# %s: could not be stat'd.\n".as_ptr(), dir.name);
            continue;
        }
        let dc = dir.contents.as_ref().expect("checked non-null above");
        if dc.dirfiles.ht_vec.is_null() {
            printf(
                c"# %s (device %ld, inode %ld): could not be opened.\n".as_ptr(),
                dir.name,
                dc.dev as c_long,
                dc.ino as c_long,
            );
            continue;
        }

        let mut f: c_uint = 0;
        let mut im: c_uint = 0;
        for j in 0..dc.dirfiles.ht_size as usize {
            let df = *dc.dirfiles.ht_vec.add(j) as *const dirfile;
            if is_real_item(df as *const c_void) {
                let df = df.as_ref().expect("slot holds a real entry");
                if df.impossible != 0 {
                    im += 1;
                } else {
                    f += 1;
                }
            }
        }
        printf(
            c"# %s (device %ld, inode %ld): ".as_ptr(),
            dir.name,
            dc.dev as c_long,
            dc.ino as c_long,
        );
        print_count(f, c"No".as_ptr());
        fputs(c" files, ".as_ptr(), stdout);
        print_count(im, c"no".as_ptr());
        fputs(c" impossibilities".as_ptr(), stdout);
        if dc.dirstream.is_null() {
            puts(c".".as_ptr());
        } else {
            puts(c" so far.".as_ptr());
        }
        files += f;
        impossible += im;
    }

    fputs(c"\n# ".as_ptr(), stdout);
    print_count(files, c"No".as_ptr());
    fputs(c" files, ".as_ptr(), stdout);
    print_count(impossible, c"no".as_ptr());
    printf(
        c" impossibilities in %lu directories.\n".as_ptr(),
        table.len() as c_ulong,
    );
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
    crate::make_main::with_exec_context(|ctx| unsafe {
        let dir = find_directory(ctx, directory)
            .as_mut()
            .expect("find_directory never returns null");
        let Some(dc) = dir.contents.as_mut() else {
            // The directory could not be stat'd.
            return null_mut();
        };
        if dc.dirfiles.ht_vec.is_null() {
            // The directory could not be opened.
            return null_mut();
        }
        // Read it all in now so the cache is complete.
        dir_contents_file_exists_p(ctx, &raw mut *dir, null());

        let new = (xmalloc(::core::mem::size_of::<dirstream>() as size_t) as *mut dirstream)
            .as_mut()
            .expect("xmalloc never returns null");
        new.contents = &raw mut *dc;
        new.dirfile_slot = dc.dirfiles.ht_vec as *mut *mut dirfile;
        (&raw mut *new).cast()
    })
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
        let ds = (stream as *mut dirstream)
            .as_mut()
            .expect("read_dirstream: null stream");
        let dc = ds.contents.as_ref().expect("dirstream always has contents");
        let dirfile_end =
            (dc.dirfiles.ht_vec as *mut *mut dirfile).add(dc.dirfiles.ht_size as usize);

        while ds.dirfile_slot < dirfile_end {
            let slot = ds.dirfile_slot;
            ds.dirfile_slot = slot.add(1);
            let df = *slot;
            if !is_real_item(df as *const c_void) {
                continue;
            }
            let df = df.as_ref().expect("slot holds a real entry");
            if df.impossible == 0 {
                // Grow the dirent buffer to hold the name (the d_name field's
                // declared 256 bytes are replaced by the real length).
                let len = df.length + 1;
                let sz = ::core::mem::size_of::<dirent>() as size_t
                    - ::core::mem::size_of::<[c_char; 256]>() as size_t
                    + len;
                if sz > ctx.read_dirstream_bufsz.get() {
                    let bufsz = (ctx.read_dirstream_bufsz.get() * 2).max(sz);
                    ctx.read_dirstream_bufsz.set(bufsz);
                    ctx.read_dirstream_buf
                        .set(xrealloc(ctx.read_dirstream_buf.get() as *mut c_void, bufsz)
                            as *mut c_char);
                }
                let d = (ctx.read_dirstream_buf.get() as *mut dirent)
                    .as_mut()
                    .expect("xrealloc never returns null");
                d.d_ino = 1;
                d.d_type = df.type_0;
                memcpy(d.d_name.as_mut_ptr().cast(), df.name as *const c_void, len);
                return &raw mut *d;
            }
        }
        null_mut()
    })
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
    gl.gl_closedir = Some(free);
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

#[cfg(test)]
mod dirfile_cmp_tests {
    use super::{dirfile, dirfile_cmp, dirfile_hash_cmp};
    use ::core::cmp::Ordering;
    use ::core::ffi::{c_char, c_void};

    /// Verbatim preservation of the original raw-pointer `dirfile_hash_cmp`
    /// callback (length, then interned-pointer identity, then `strcmp`),
    /// retained as a differential oracle per the project rule for swapping an
    /// unsafe implementation for a safe one.
    ///
    /// SAFETY: callers pass pointers to live `dirfile` values whose `name`
    /// fields address NUL-terminated buffers — the contract the hash table
    /// handed the original callback.
    unsafe fn dirfile_hash_cmp_unsafe_oracle(xv: *const c_void, yv: *const c_void) -> i32 {
        let x = (xv as *const dirfile)
            .as_ref()
            .expect("hash callback got a null key");
        let y = (yv as *const dirfile)
            .as_ref()
            .expect("hash callback got a null key");
        // Compare lengths first (cheap), then interned pointers, then bytes.
        let result = x.length.wrapping_sub(y.length) as i32;
        if result != 0 {
            return result;
        }
        if ::core::ptr::eq(x.name, y.name) {
            0
        } else {
            libc::strcmp(x.name, y.name)
        }
    }

    /// Build a `dirfile` whose `name`/`length` denote `buf`. `buf` must be
    /// NUL-terminated (so the oracle's `strcmp` has a terminator); `length`
    /// excludes the NUL, matching how the cache stores interned names.
    fn dirfile_for(buf: &[u8]) -> dirfile {
        // SAFETY: a zeroed `dirfile` is a valid (inert) value; the tests only
        // read its `name`/`length` fields.
        let mut d: dirfile = unsafe { ::core::mem::zeroed() };
        d.name = buf.as_ptr() as *const c_char;
        d.length = (buf.len() - 1) as crate::ffi_types::size_t;
        d
    }

    /// Drive the production callback with two NUL-terminated name buffers.
    fn callback(x: &[u8], y: &[u8]) -> i32 {
        let dx = dirfile_for(x);
        let dy = dirfile_for(y);
        // SAFETY: both dirfiles address live, NUL-terminated buffers.
        unsafe {
            dirfile_hash_cmp(
                &raw const dx as *const c_void,
                &raw const dy as *const c_void,
            )
        }
    }

    /// Drive the verbatim oracle with two NUL-terminated name buffers.
    fn oracle(x: &[u8], y: &[u8]) -> i32 {
        let dx = dirfile_for(x);
        let dy = dirfile_for(y);
        // SAFETY: both dirfiles address live, NUL-terminated buffers.
        unsafe {
            dirfile_hash_cmp_unsafe_oracle(
                &raw const dx as *const c_void,
                &raw const dy as *const c_void,
            )
        }
    }

    // Each sample is NUL-terminated; the stored `length` is taken as `len - 1`.
    const SAMPLES: &[&[u8]] = &[
        b"\0",
        b"a\0",
        b"b\0",
        b"A\0",
        b"ab\0",
        b"ba\0",
        b"abc\0",
        b"abd\0",
        b"src\0",
        b"build\0",
        b"Makefile\0",
        b"makefile\0",
    ];

    #[test]
    fn callback_matches_oracle_in_sign() {
        for &x in SAMPLES {
            for &y in SAMPLES {
                assert_eq!(
                    callback(x, y).signum(),
                    oracle(x, y).signum(),
                    "sign mismatch for {x:?} vs {y:?}",
                );
            }
        }
    }

    #[test]
    fn pure_core_matches_callback_sign() {
        for &x in SAMPLES {
            for &y in SAMPLES {
                let expected = match dirfile_cmp(&x[..x.len() - 1], &y[..y.len() - 1]) {
                    Ordering::Less => -1,
                    Ordering::Greater => 1,
                    Ordering::Equal => 0,
                };
                assert_eq!(callback(x, y).signum(), expected, "{x:?} vs {y:?}");
            }
        }
    }

    #[test]
    fn orders_by_length_then_bytes() {
        // Shorter names sort first, whatever their bytes.
        assert_eq!(dirfile_cmp(b"zzz", b"aaaa"), Ordering::Less);
        // Equal length falls back to lexicographic byte order.
        assert_eq!(dirfile_cmp(b"abc", b"abd"), Ordering::Less);
        assert_eq!(dirfile_cmp(b"abd", b"abc"), Ordering::Greater);
        assert_eq!(dirfile_cmp(b"abc", b"abc"), Ordering::Equal);
    }

    #[test]
    fn is_antisymmetric() {
        for &x in SAMPLES {
            for &y in SAMPLES {
                let xn = &x[..x.len() - 1];
                let yn = &y[..y.len() - 1];
                assert_eq!(
                    dirfile_cmp(xn, yn),
                    dirfile_cmp(yn, xn).reverse(),
                    "{x:?} vs {y:?}",
                );
            }
        }
    }
}
