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
use crate::make_main::{command_count, db_level};
use crate::misc::{xcalloc, xmalloc, xrealloc};
use crate::output::fatal;
use crate::stdio::FILE;
use crate::strcache::strcache_add_len;

use ::core::ffi::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use ::core::ptr::{null, null_mut};

use libc::{
    __errno_location, closedir, free, memcpy, opendir, printf, puts, readdir, strcmp, strerror,
    strlen, strrchr, DIR, EINTR,
};

pub use crate::sys_stat::{stat, timespec};

extern "C" {
    fn stat(file: *const c_char, buf: *mut stat) -> c_int;
    fn lstat(file: *const c_char, buf: *mut stat) -> c_int;
    static mut stdout: *mut FILE;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
}

/// `glob_t` as laid out by gnulib's glob with `GLOB_ALTDIRFUNC` support;
/// only the callback fields matter to [`dir_setup_glob`].
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut c_char,
    pub gl_offs: __size_t,
    pub gl_flags: c_int,
    pub gl_closedir: Option<unsafe extern "C" fn(*mut c_void)>,
    pub gl_readdir: Option<unsafe extern "C" fn(*mut c_void) -> *mut dirent>,
    pub gl_opendir: Option<unsafe extern "C" fn(*const c_char) -> *mut c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(*const c_char, *mut stat) -> c_int>,
    pub gl_stat: Option<unsafe extern "C" fn(*const c_char, *mut stat) -> c_int>,
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
    /// `command_count` when this name was last stat'd (used only when the
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
    /// `command_count` when the contents were last read.
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
const DB_VERBOSE: c_int = 0x2;

pub const MAX_OPEN_DIRECTORIES: c_int = 10;
pub const DIRECTORY_BUCKETS: c_int = 199;
pub const DIRFILE_BUCKETS: c_int = 107;

static mut open_directories: c_uint = 0;

/// Forget everything cached about `dc`, closing its stream if open.
unsafe fn clear_directory_contents(dc: *mut directory_contents) {
    let dc = dc.as_mut().expect("clear_directory_contents: null entry");
    dc.counter = 0;
    if !dc.dirstream.is_null() {
        open_directories -= 1;
        closedir(dc.dirstream);
        dc.dirstream = null_mut();
    }
    if !dc.dirfiles.ht_vec.is_null() {
        hash_free(&raw mut dc.dirfiles, 1);
    }
}

unsafe extern "C" fn directory_contents_hash_1(key: *const c_void) -> c_ulong {
    let key = key as *const directory_contents;
    (((*key).dev as c_uint) << 4 ^ (*key).ino as c_uint) as c_ulong
}

unsafe extern "C" fn directory_contents_hash_2(key: *const c_void) -> c_ulong {
    let key = key as *const directory_contents;
    (((*key).dev as c_uint) << 4 ^ !(*key).ino as c_uint) as c_ulong
}

unsafe extern "C" fn directory_contents_hash_cmp(xv: *const c_void, yv: *const c_void) -> c_int {
    let x = xv as *const directory_contents;
    let y = yv as *const directory_contents;
    match (*x).ino.cmp(&(*y).ino) {
        ::core::cmp::Ordering::Less => -1,
        ::core::cmp::Ordering::Greater => 1,
        ::core::cmp::Ordering::Equal => match (*x).dev.cmp(&(*y).dev) {
            ::core::cmp::Ordering::Less => -1,
            ::core::cmp::Ordering::Greater => 1,
            ::core::cmp::Ordering::Equal => 0,
        },
    }
}

static mut directory_contents: hash_table = unsafe { ::core::mem::zeroed() };

/// Hash a [`directory`] key by name.
///
/// # Safety
///
/// `key` must point to a `directory` whose name is NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn directory_hash_1(key: *const c_void) -> c_ulong {
    jhash_string((*(key as *const directory)).name as *const c_uchar) as c_ulong
}

/// Secondary hash for [`directory`] keys; always zero, kept for the
/// callback ABI.
///
/// # Safety
///
/// Never dereferences `key`; any pointer value is acceptable.
#[no_mangle]
pub unsafe extern "C" fn directory_hash_2(_key: *const c_void) -> c_ulong {
    0
}

unsafe extern "C" fn directory_hash_cmp(x: *const c_void, y: *const c_void) -> c_int {
    let xn = (*(x as *const directory)).name;
    let yn = (*(y as *const directory)).name;
    // Names are interned, so pointer equality short-circuits the strcmp.
    if ::core::ptr::eq(xn, yn) {
        0
    } else {
        strcmp(xn, yn)
    }
}

static mut directories: hash_table = unsafe { ::core::mem::zeroed() };

/// Hash a [`dirfile`] key by name.
///
/// # Safety
///
/// `key` must point to a `dirfile` whose name is NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn dirfile_hash_1(key: *const c_void) -> c_ulong {
    jhash_string((*(key as *const dirfile)).name as *const c_uchar) as c_ulong
}

/// Secondary hash for [`dirfile`] keys; always zero, kept for the
/// callback ABI.
///
/// # Safety
///
/// Never dereferences `key`; any pointer value is acceptable.
#[no_mangle]
pub unsafe extern "C" fn dirfile_hash_2(_key: *const c_void) -> c_ulong {
    0
}

unsafe extern "C" fn dirfile_hash_cmp(xv: *const c_void, yv: *const c_void) -> c_int {
    let x = xv as *const dirfile;
    let y = yv as *const dirfile;
    // Compare lengths first (cheap), then interned pointers, then bytes.
    let result = (*x).length.wrapping_sub((*y).length) as c_int;
    if result != 0 {
        return result;
    }
    if ::core::ptr::eq((*x).name, (*y).name) {
        0
    } else {
        strcmp((*x).name, (*y).name)
    }
}

/// Look up (or create) the cache entry for directory `name`, refreshing
/// it when a new command has run since it was cached.
///
/// # Safety
///
/// `name` must be a NUL-terminated path; the directory tables must be
/// initialized (see [`hash_init_directories`]).
#[no_mangle]
pub unsafe extern "C" fn find_directory(name: *const c_char) -> *mut directory {
    let mut dir_key: directory = ::core::mem::zeroed();
    dir_key.name = name;
    let dir_slot = (hash_find_slot(&raw mut directories, (&raw const dir_key).cast())
        as *mut *mut directory)
        .as_mut()
        .expect("hash_find_slot always returns a slot");

    let mut dir = *dir_slot;
    if is_real_item(dir as *const c_void) {
        let dir_ref = dir.as_mut().expect("directory slot holds a real entry");
        // Cache hit: still valid unless a command has run since.
        let ctr = dir_ref.counter;
        if ctr == command_count {
            return dir;
        }
        if DB_VERBOSE & db_level != 0 {
            printf(
                c"Directory %s cache invalidated (count %lu != command %lu)\n".as_ptr(),
                name,
                ctr,
                command_count,
            );
            fflush(stdout);
        }
        if !dir_ref.contents.is_null() {
            clear_directory_contents(dir_ref.contents);
        }
    } else {
        let len = strlen(name);
        dir = xmalloc(::core::mem::size_of::<directory>() as size_t) as *mut directory;
        (*dir).name = strcache_add_len(name, len);
        hash_insert_at(
            &raw mut directories,
            dir as *const c_void,
            (&raw mut *dir_slot).cast(),
        );
    }
    let dir_ref = dir.as_mut().expect("directory entry just selected");
    dir_ref.contents = null_mut();
    dir_ref.counter = command_count;

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

    // Directory contents are shared across names via the dev/ino key.
    let mut dc_key: directory_contents = ::core::mem::zeroed();
    dc_key.dev = st.st_dev as dev_t;
    dc_key.ino = st.st_ino as ino_t;
    let dc_slot = (hash_find_slot(&raw mut directory_contents, (&raw const dc_key).cast())
        as *mut *mut directory_contents)
        .as_mut()
        .expect("hash_find_slot always returns a slot");
    let mut dc = *dc_slot;
    if !is_real_item(dc as *const c_void) {
        dc = xcalloc(::core::mem::size_of::<directory_contents>() as size_t)
            as *mut directory_contents;
        *dc = dc_key;
        hash_insert_at(
            &raw mut directory_contents,
            dc as *const c_void,
            (&raw mut *dc_slot).cast(),
        );
    }
    let dc = dc.as_mut().expect("directory_contents entry just selected");
    dir_ref.contents = dc;

    if dc.counter != command_count {
        if dc.counter != 0 {
            clear_directory_contents(dc);
        }
        dc.counter = command_count;
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
                Some(dirfile_hash_1 as unsafe extern "C" fn(*const c_void) -> c_ulong),
                Some(dirfile_hash_2 as unsafe extern "C" fn(*const c_void) -> c_ulong),
                Some(
                    dirfile_hash_cmp as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
                ),
            );
            open_directories += 1;
            if open_directories == MAX_OPEN_DIRECTORIES as c_uint {
                // Too many streams open: read this one to completion now.
                dir_contents_file_exists_p(dir, null());
            }
        }
    }
    dir
}

/// Does `filename` exist in `dir`? Reads the directory incrementally,
/// caching every entry seen; a null `filename` reads to the end.
unsafe fn dir_contents_file_exists_p(dir: *mut directory, filename: *const c_char) -> c_int {
    let dc: *mut directory_contents = (*dir).contents;
    if dc.is_null() || (*dc).dirfiles.ht_vec.is_null() {
        // The directory could not be stat'd or opened.
        return 0;
    }
    let dc = dc.as_mut().expect("checked non-null above");

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
            return (df.impossible == 0) as c_int;
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
                    null::<Floc>(),
                    strlen((*dir).name) + strlen(strerror(*__errno_location())),
                    c"readdir %s: %s".as_ptr(),
                    (*dir).name,
                    strerror(*__errno_location()),
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
        open_directories -= 1;
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
#[no_mangle]
pub unsafe extern "C" fn dir_file_exists_p(
    dirname: *const c_char,
    filename: *const c_char,
) -> c_int {
    dir_contents_file_exists_p(find_directory(dirname), filename)
}

/// Split `name` at its final slash, returning `(dirname, basename)` plus
/// the owned buffer (if any) keeping `dirname` alive.
unsafe fn split_dir(name: *const c_char) -> Option<(Vec<u8>, *const c_char, *const c_char)> {
    let dirend = strrchr(name, '/' as c_int);
    if dirend.is_null() {
        return None;
    }
    let (buf, dirname): (Vec<u8>, *const c_char) = if ::core::ptr::eq(dirend, name as *mut c_char) {
        (Vec::new(), c"/".as_ptr())
    } else {
        let len = dirend.offset_from(name) as usize;
        let mut buf = ::core::slice::from_raw_parts(name as *const u8, len).to_vec();
        buf.push(0);
        let p = buf.as_ptr() as *const c_char;
        (buf, p)
    };
    Some((buf, dirname, dirend.add(1)))
}

/// Does file `name` (with optional directory part) exist?
///
/// # Safety
///
/// `name` must be NUL-terminated; the directory tables must be
/// initialized.
#[no_mangle]
pub unsafe extern "C" fn file_exists_p(name: *const c_char) -> c_int {
    if crate::ar::ar_name(name) != 0 {
        return (crate::ar::ar_member_date(name) != -1) as c_int;
    }
    match split_dir(name) {
        None => dir_file_exists_p(c".".as_ptr(), name),
        Some((_buf, dirname, base)) => dir_file_exists_p(dirname, base),
    }
}

/// Record that `filename` is an impossible target: make tried to build it
/// and couldn't, so don't consider it again this command.
///
/// # Safety
///
/// `filename` must be NUL-terminated; the directory tables must be
/// initialized.
#[no_mangle]
pub unsafe extern "C" fn file_impossible(filename: *const c_char) {
    let (dir, filename) = match split_dir(filename) {
        None => (find_directory(c".".as_ptr()), filename),
        Some((_buf, dirname, base)) => (find_directory(dirname), base),
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
            Some(dirfile_hash_1 as unsafe extern "C" fn(*const c_void) -> c_ulong),
            Some(dirfile_hash_2 as unsafe extern "C" fn(*const c_void) -> c_ulong),
            Some(dirfile_hash_cmp as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int),
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
#[no_mangle]
pub unsafe extern "C" fn file_impossible_p(filename: *const c_char) -> c_int {
    let (dir, filename) = match split_dir(filename) {
        None => {
            let dir_ptr = find_directory(c".".as_ptr());
            let contents = dir_ptr
                .as_ref()
                .map_or(::core::ptr::null_mut(), |d| d.contents);
            (contents, filename)
        }
        Some((_buf, dirname, base)) => {
            let dir_ptr = find_directory(dirname);
            let contents = dir_ptr
                .as_ref()
                .map_or(::core::ptr::null_mut(), |d| d.contents);
            (contents, base)
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
        Some(df) => df.impossible as c_int,
        None => 0,
    }
}

/// Return the canonical (interned) name for directory `dir`.
///
/// # Safety
///
/// `dir` must be NUL-terminated; the directory tables must be
/// initialized.
#[no_mangle]
pub unsafe extern "C" fn dir_name(dir: *const c_char) -> *const c_char {
    (*find_directory(dir)).name
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
#[no_mangle]
pub unsafe fn print_dir_data_base() {
    puts(c"\n# Directories\n".as_ptr());

    let mut files: c_uint = 0;
    let mut impossible: c_uint = 0;
    for i in 0..directories.ht_size as usize {
        let dir = *directories.ht_vec.add(i) as *mut directory;
        if !is_real_item(dir as *const c_void) {
            continue;
        }
        let dir = dir.as_ref().expect("slot holds a real entry");
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
                if (*df).impossible != 0 {
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
        directories.ht_fill,
    );
}

/// glob `opendir` callback: position a cursor over the cached contents of
/// `directory`, reading it to completion first.
unsafe extern "C" fn open_dirstream(directory: *const c_char) -> *mut c_void {
    let dir = find_directory(directory);
    if (*dir).contents.is_null() || (*(*dir).contents).dirfiles.ht_vec.is_null() {
        // The directory could not be stat'd or opened.
        return null_mut();
    }
    // Read it all in now so the cache is complete.
    dir_contents_file_exists_p(dir, null());

    let new = (xmalloc(::core::mem::size_of::<dirstream>() as size_t) as *mut dirstream)
        .as_mut()
        .expect("xmalloc never returns null");
    new.contents = (*dir).contents;
    new.dirfile_slot = (*new.contents).dirfiles.ht_vec as *mut *mut dirfile;
    (&raw mut *new).cast()
}

/// glob `readdir` callback: synthesize a `dirent` for the next cached
/// (non-impossible) file.
///
/// # Safety
///
/// `stream` must come from `open_dirstream`. The returned dirent lives in
/// a static buffer that the next call overwrites.
#[no_mangle]
pub unsafe extern "C" fn read_dirstream(stream: *mut c_void) -> *mut dirent {
    static mut buf: *mut c_char = null_mut();
    static mut bufsz: size_t = 0;

    let ds = stream as *mut dirstream;
    let dc: *mut directory_contents = (*ds).contents;
    let dirfile_end =
        ((*dc).dirfiles.ht_vec as *mut *mut dirfile).add((*dc).dirfiles.ht_size as usize);

    while (*ds).dirfile_slot < dirfile_end {
        let slot = (*ds).dirfile_slot;
        (*ds).dirfile_slot = slot.add(1);
        let df = *slot;
        if is_real_item(df as *const c_void) && (*df).impossible == 0 {
            // Grow the dirent buffer to hold the name (the d_name field's
            // declared 256 bytes are replaced by the real length).
            let len = (*df).length + 1;
            let sz = ::core::mem::size_of::<dirent>() as size_t
                - ::core::mem::size_of::<[c_char; 256]>() as size_t
                + len;
            if sz > bufsz {
                bufsz = (bufsz * 2).max(sz);
                buf = xrealloc(buf as *mut c_void, bufsz) as *mut c_char;
            }
            let d = buf as *mut dirent;
            (*d).d_ino = 1;
            (*d).d_type = (*df).type_0;
            memcpy(
                (*d).d_name.as_mut_ptr().cast(),
                (*df).name as *const c_void,
                len,
            );
            return d;
        }
    }
    null_mut()
}

/// Point `gl` at the directory cache so glob() reads from it instead of
/// the filesystem.
///
/// # Safety
///
/// `gl` must be a valid `glob_t` matching this layout.
#[no_mangle]
pub unsafe extern "C" fn dir_setup_glob(gl: *mut glob_t) {
    let gl = gl.as_mut().expect("dir_setup_glob: null glob_t");
    gl.gl_offs = 0;
    gl.gl_opendir = Some(open_dirstream);
    gl.gl_readdir = Some(read_dirstream);
    gl.gl_closedir = Some(free);
    gl.gl_lstat = Some(lstat);
    gl.gl_stat = Some(stat);
}

/// Initialize the two directory hash tables.
///
/// # Safety
///
/// Must run once, before any other function in this module.
#[no_mangle]
pub unsafe fn hash_init_directories() {
    hash_init(
        &raw mut directories,
        DIRECTORY_BUCKETS as c_ulong,
        Some(directory_hash_1 as unsafe extern "C" fn(*const c_void) -> c_ulong),
        Some(directory_hash_2 as unsafe extern "C" fn(*const c_void) -> c_ulong),
        Some(directory_hash_cmp as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int),
    );
    hash_init(
        &raw mut directory_contents,
        DIRECTORY_BUCKETS as c_ulong,
        Some(directory_contents_hash_1 as unsafe extern "C" fn(*const c_void) -> c_ulong),
        Some(directory_contents_hash_2 as unsafe extern "C" fn(*const c_void) -> c_ulong),
        Some(
            directory_contents_hash_cmp
                as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
        ),
    );
}
