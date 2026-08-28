//! The one filesystem interface.
//!
//! Every filesystem operation in this crate is meant to go through this
//! module, and this module is implemented on `std::fs`/`std::io` — never on
//! `libc`. That single indirection is what lets one body of code serve both
//! deployment shapes:
//!
//! - **native**: `std::fs` lowers to the ordinary platform syscalls, exactly
//!   what the hand-declared `extern "C" { fn stat(...) }` blocks used to do.
//! - **wasm (`wasm32-wasip1`, e.g. inside a Cloudflare Worker)**: `std::fs`
//!   lowers to WASI directly — `path_filestat_get`, `fd_filestat_get`,
//!   `fd_read`, `fd_readdir` — with no C library and no POSIX syscall layer
//!   underneath. A host that satisfies the WASI filesystem imports (a
//!   virtualised or overlay filesystem in a Worker, say) is then the whole
//!   filesystem, with nothing to emulate at the libc level.
//!
//! The libc-shaped alternative does not survive that second shape. A
//! `#[repr(C)] struct stat` mirroring glibc's `struct stat64` (the former
//! `crate::sys_stat`) is an x86_64-Linux ABI record that no WASI call ever
//! populates, and `opendir`/`readdir` have no WASI equivalent at all. Rather
//! than stub those out per-platform, the operations are expressed once here
//! in terms of what make actually needs.
//!
//! # Paths
//!
//! Callers traffic in bytes, because makefiles do: a target name is a byte
//! string, not necessarily UTF-8. [`path`] and [`path_from_c`] convert to
//! `&Path` without a lossy round-trip, and this module is the crate's single
//! home for the byte-oriented `OsStr` conversions that make that possible
//! (`std::os::unix::ffi` on unix, `std::os::wasi::ffi` on wasm), so call
//! sites don't each repeat the `#[cfg]` pair.

use std::{
    ffi::{CStr, OsStr},
    fs,
    io,
    mem::ManuallyDrop,
    os::fd::{FromRawFd, RawFd},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(unix)]
pub use std::os::unix::ffi::{OsStrExt, OsStringExt};
#[cfg(target_os = "wasi")]
pub use std::os::wasi::ffi::{OsStrExt, OsStringExt};

/// Identifies a file independently of the path used to reach it, so two names
/// for one file compare equal.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileId {
    pub dev: u64,
    pub ino: u64,
}

/// The device/inode pair for `m`, where the platform exposes one.
///
/// WASI does report `dev`/`ino` in its `filestat`, but Rust surfaces them
/// only through `std::os::wasi::fs::MetadataExt`, which is still unstable
/// (`wasi_ext`, rust-lang/rust#71213). Rather than pin a nightly toolchain
/// for a cache key, wasm reports `None` and callers fall back to keying by
/// path — a missed sharing opportunity between two names for one directory,
/// never a wrong answer. When `wasi_ext` stabilises this becomes a `cfg`
/// arm like the unix one and the fallback goes away.
#[cfg(unix)]
fn file_id(m: &fs::Metadata) -> Option<FileId> {
    use std::os::unix::fs::MetadataExt;
    Some(FileId {
        dev: m.dev(),
        ino: m.ino(),
    })
}

#[cfg(not(unix))]
fn file_id(_m: &fs::Metadata) -> Option<FileId> {
    None
}

/// Borrow `bytes` as a filesystem path, with no UTF-8 validation or lossy
/// conversion: the bytes are the path.
pub fn path(bytes: &[u8]) -> &Path {
    Path::new(OsStr::from_bytes(bytes))
}

/// Borrow a NUL-terminated C string as a filesystem path.
///
/// The `*const c_char` call sites that remain are FFI-adjacent leftovers; this
/// is the one place that converts, so the raw pointer expires here instead of
/// spreading further into the filesystem layer.
///
/// # Safety
///
/// `p` must be non-null and point at a valid NUL-terminated C string that
/// stays alive for the returned borrow.
pub unsafe fn path_from_c<'a>(p: *const ::core::ffi::c_char) -> &'a Path {
    path(CStr::from_ptr(p).to_bytes())
}

/// A filesystem timestamp, as whole seconds since the Unix epoch plus a
/// nanosecond part. Negative `secs` is a pre-epoch time.
///
/// make compares and prints mtimes rather than doing calendar arithmetic on
/// them, so this stays a plain pair rather than growing a date type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Timestamp {
    pub secs: i64,
    pub nanos: u32,
}

impl Timestamp {
    /// Split a `SystemTime` into seconds/nanoseconds about the epoch,
    /// handling pre-epoch times (where `duration_since` reports the error
    /// side) rather than clamping them to zero.
    fn from_system_time(t: SystemTime) -> Self {
        match t.duration_since(UNIX_EPOCH) {
            Ok(d) => Timestamp {
                secs: d.as_secs() as i64,
                nanos: d.subsec_nanos(),
            },
            Err(e) => {
                // `t` is before the epoch: the error carries how far before.
                let d = e.duration();
                let (secs, nanos) = if d.subsec_nanos() == 0 {
                    (-(d.as_secs() as i64), 0)
                } else {
                    // Borrow a second so the nanosecond part stays positive,
                    // matching the `tv_sec`/`tv_nsec` convention.
                    (
                        -(d.as_secs() as i64) - 1,
                        1_000_000_000 - d.subsec_nanos(),
                    )
                };
                Timestamp { secs, nanos }
            }
        }
    }
}

/// What a directory entry is. Mirrors the `S_IFMT` tests make actually
/// performs — regular/directory/symlink — with everything else (fifos,
/// sockets, devices) collapsed into [`FileKind::Other`], since make only ever
/// asks whether a name is one of the three.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FileKind {
    File,
    Dir,
    Symlink,
    Other,
}

impl FileKind {
    /// Classify a `std::fs::FileType`.
    pub fn of(ft: fs::FileType) -> Self {
        if ft.is_symlink() {
            FileKind::Symlink
        } else if ft.is_dir() {
            FileKind::Dir
        } else if ft.is_file() {
            FileKind::File
        } else {
            FileKind::Other
        }
    }
}

/// What make needs to know about a file.
///
/// Deliberately much narrower than `struct stat`: the whole crate only ever
/// consumed the modification time, the file type, the size, and the
/// device/inode pair that keys the directory cache. Everything else in the
/// C struct was carried for ABI fidelity to a syscall this code no longer
/// makes.
#[derive(Copy, Clone, Debug)]
pub struct Metadata {
    id: Option<FileId>,
    len: u64,
    mtime: Option<Timestamp>,
    kind: FileKind,
}

impl Metadata {
    fn from_std(m: &fs::Metadata) -> Self {
        let kind = FileKind::of(m.file_type());
        Metadata {
            id: file_id(m),
            len: m.len(),
            // A filesystem that cannot report an mtime is not an error here;
            // callers treat "no timestamp" the same as an unknown one.
            mtime: m.modified().ok().map(Timestamp::from_system_time),
            kind,
        }
    }

    /// Identity of this file independent of the path used to reach it, which
    /// is how the directory cache notices that two names are the same
    /// directory. `None` where the platform does not expose one — see
    /// [`file_id`]; callers must then fall back to keying by path.
    pub fn id(&self) -> Option<FileId> {
        self.id
    }

    /// Size in bytes.
    pub fn len(&self) -> u64 {
        self.len
    }

    /// Whether the file is empty. (Present because clippy asks for it
    /// alongside `len`; make itself only compares sizes.)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Modification time, or `None` if this filesystem does not report one.
    pub fn modified(&self) -> Option<Timestamp> {
        self.mtime
    }

    pub fn kind(&self) -> FileKind {
        self.kind
    }

    pub fn is_file(&self) -> bool {
        self.kind == FileKind::File
    }

    pub fn is_dir(&self) -> bool {
        self.kind == FileKind::Dir
    }

    pub fn is_symlink(&self) -> bool {
        self.kind == FileKind::Symlink
    }
}

/// Run `op`, retrying while it reports `EINTR`.
///
/// The C original wrapped its filesystem calls in `EINTRLOOP`; keeping that
/// policy here means the retry is stated once for the whole crate instead of
/// being open-coded at each call site (and silently forgotten at some of
/// them).
fn retry_eintr<T>(mut op: impl FnMut() -> io::Result<T>) -> io::Result<T> {
    loop {
        match op() {
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            other => return other,
        }
    }
}

/// Metadata for `p`, following symlinks (`stat`).
pub fn metadata(p: &Path) -> io::Result<Metadata> {
    retry_eintr(|| fs::metadata(p)).map(|m| Metadata::from_std(&m))
}

/// Metadata for `p` itself, not following a final symlink (`lstat`).
pub fn symlink_metadata(p: &Path) -> io::Result<Metadata> {
    retry_eintr(|| fs::symlink_metadata(p)).map(|m| Metadata::from_std(&m))
}

/// Metadata for an already-open file (`fstat`).
pub fn metadata_of(f: &fs::File) -> io::Result<Metadata> {
    f.metadata().map(|m| Metadata::from_std(&m))
}

/// Metadata for a borrowed raw descriptor (`fstat`).
///
/// The descriptor stays owned by the caller: the `File` wrapper is
/// `ManuallyDrop`, so returning does not close `fd`.
///
/// # Safety
///
/// `fd` must be a valid open descriptor for the duration of the call.
pub unsafe fn metadata_of_fd(fd: RawFd) -> io::Result<Metadata> {
    let f = ManuallyDrop::new(fs::File::from_raw_fd(fd));
    metadata_of(&f)
}

/// Whether `p` exists, following symlinks.
///
/// Any error — including a permission failure partway down the path — reads
/// as "not there", which is what make's existence probes want.
pub fn exists(p: &Path) -> bool {
    fs::metadata(p).is_ok()
}

/// Read the target of a symlink.
pub fn read_link(p: &Path) -> io::Result<Vec<u8>> {
    fs::read_link(p).map(|t| t.into_os_string().into_vec())
}

/// Delete a file.
pub fn remove_file(p: &Path) -> io::Result<()> {
    fs::remove_file(p)
}

/// Rename a file, replacing `to` if it exists.
pub fn rename(from: &Path, to: &Path) -> io::Result<()> {
    fs::rename(from, to)
}

/// Read a whole file.
pub fn read(p: &Path) -> io::Result<Vec<u8>> {
    fs::read(p)
}

/// One entry from [`read_dir`].
#[derive(Clone, Debug)]
pub struct DirEntry {
    /// The entry's name within its directory, as bytes — never a full path.
    pub name: Vec<u8>,
    /// The entry's type, when the directory listing reported it without a
    /// second lookup. `None` means the caller must [`symlink_metadata`] the
    /// name to find out.
    pub kind: Option<FileKind>,
}

/// List a directory.
///
/// Returns the whole listing rather than an iterator: make's directory cache
/// slurps a directory in one go and then answers questions from the snapshot,
/// and a materialised listing keeps no descriptor open across that.
pub fn read_dir(p: &Path) -> io::Result<Vec<DirEntry>> {
    let mut entries = Vec::new();
    for e in fs::read_dir(p)? {
        let e = e?;
        entries.push(DirEntry {
            name: e.file_name().into_vec(),
            // `file_type` is free when the readdir result carried a type and
            // a lookup otherwise; either way a failure here just means the
            // caller has to ask separately.
            kind: e.file_type().ok().map(FileKind::of),
        });
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// A scratch directory that removes itself, so these tests leave nothing
    /// behind and don't depend on each other's leftovers.
    struct TempDir(std::path::PathBuf);

    impl TempDir {
        fn new(tag: &str) -> Self {
            let p = std::env::temp_dir().join(format!(
                "makers_fs_test_{}_{}",
                tag,
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&p);
            fs::create_dir_all(&p).unwrap();
            TempDir(p)
        }
        fn join(&self, name: &str) -> std::path::PathBuf {
            self.0.join(name)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn path_borrows_bytes_without_lossy_conversion() {
        // A non-UTF-8 byte that a `String` round-trip would have replaced.
        let raw = b"caf\xe9.o";
        assert_eq!(path(raw).as_os_str().as_bytes(), raw);
    }

    #[test]
    fn path_from_c_stops_at_the_nul() {
        let c = c"dir/file.o";
        let p = unsafe { path_from_c(c.as_ptr()) };
        assert_eq!(p.as_os_str().as_bytes(), b"dir/file.o");
    }

    #[test]
    fn metadata_reports_size_and_regular_file() {
        let d = TempDir::new("meta");
        let f = d.join("a.o");
        fs::write(&f, b"0123456789").unwrap();

        let m = metadata(&f).unwrap();
        assert_eq!(m.len(), 10);
        assert!(!m.is_empty());
        assert_eq!(m.kind(), FileKind::File);
        assert!(m.is_file() && !m.is_dir() && !m.is_symlink());
        assert!(m.modified().is_some());
    }

    #[test]
    fn metadata_reports_directories() {
        let d = TempDir::new("dir");
        let m = metadata(&d.0).unwrap();
        assert_eq!(m.kind(), FileKind::Dir);
        assert!(m.is_dir() && !m.is_file());
    }

    #[test]
    fn file_id_identifies_the_same_file_through_two_names() {
        let d = TempDir::new("fileid");
        let a = d.join("a.o");
        fs::write(&a, b"x").unwrap();
        // Reach the same file through a redundant path component.
        let b = d.0.join(".").join("a.o");
        let other = d.join("b.o");
        fs::write(&other, b"x").unwrap();

        let (ma, mb, mo) = (
            metadata(&a).unwrap(),
            metadata(&b).unwrap(),
            metadata(&other).unwrap(),
        );

        match ma.id() {
            // Where identity is available, two names for one file agree and
            // two distinct files differ.
            Some(_) => {
                assert_eq!(ma.id(), mb.id());
                assert_ne!(ma.id(), mo.id());
            }
            // Where it isn't, it must be absent consistently rather than
            // fabricated — callers key by path instead.
            None => {
                assert!(mb.id().is_none() && mo.id().is_none());
            }
        }
    }

    #[test]
    fn metadata_of_fd_does_not_close_the_descriptor() {
        use std::os::fd::AsRawFd;

        let d = TempDir::new("fstat");
        let f = d.join("a.o");
        fs::write(&f, b"12345").unwrap();
        let file = fs::File::open(&f).unwrap();

        let m = unsafe { metadata_of_fd(file.as_raw_fd()) }.unwrap();
        assert_eq!(m.len(), 5);
        // Still usable: `metadata_of_fd` must not have taken ownership.
        assert_eq!(metadata_of(&file).unwrap().len(), 5);
    }

    #[test]
    fn exists_is_false_for_a_missing_path() {
        let d = TempDir::new("exists");
        let f = d.join("a.o");
        assert!(!exists(&f));
        fs::write(&f, b"x").unwrap();
        assert!(exists(&f));
    }

    #[test]
    fn read_dir_lists_names_with_types() {
        let d = TempDir::new("readdir");
        fs::write(d.join("a.o"), b"x").unwrap();
        fs::create_dir(d.join("sub")).unwrap();

        let mut got: Vec<(Vec<u8>, Option<FileKind>)> = read_dir(&d.0)
            .unwrap()
            .into_iter()
            .map(|e| (e.name, e.kind))
            .collect();
        got.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(
            got,
            vec![
                (b"a.o".to_vec(), Some(FileKind::File)),
                (b"sub".to_vec(), Some(FileKind::Dir)),
            ]
        );
    }

    #[test]
    fn remove_and_rename_move_files() {
        let d = TempDir::new("mutate");
        let (a, b) = (d.join("a.o"), d.join("b.o"));
        fs::write(&a, b"x").unwrap();

        rename(&a, &b).unwrap();
        assert!(!exists(&a) && exists(&b));

        remove_file(&b).unwrap();
        assert!(!exists(&b));
    }

    #[test]
    fn missing_paths_report_an_error_rather_than_a_sentinel() {
        let d = TempDir::new("missing");
        assert!(metadata(&d.join("nope.o")).is_err());
        assert!(symlink_metadata(&d.join("nope.o")).is_err());
        assert!(read_dir(&d.join("nope")).is_err());
    }

    #[test]
    fn timestamp_splits_post_epoch_times() {
        let t = UNIX_EPOCH + Duration::new(1_700_000_000, 250);
        assert_eq!(
            Timestamp::from_system_time(t),
            Timestamp {
                secs: 1_700_000_000,
                nanos: 250
            }
        );
    }

    #[test]
    fn timestamp_splits_pre_epoch_times_keeping_nanos_positive() {
        // Two whole seconds before the epoch: no borrow needed.
        assert_eq!(
            Timestamp::from_system_time(UNIX_EPOCH - Duration::new(2, 0)),
            Timestamp {
                secs: -2,
                nanos: 0
            }
        );
        // 1.5s before the epoch is -2s + 0.5s, so the nanosecond part stays
        // positive and the second borrows, matching `tv_sec`/`tv_nsec`.
        assert_eq!(
            Timestamp::from_system_time(UNIX_EPOCH - Duration::new(1, 500_000_000)),
            Timestamp {
                secs: -2,
                nanos: 500_000_000
            }
        );
    }

    #[test]
    fn timestamps_order_by_time() {
        let early = Timestamp::from_system_time(UNIX_EPOCH + Duration::new(10, 0));
        let late = Timestamp::from_system_time(UNIX_EPOCH + Duration::new(10, 1));
        assert!(early < late);
    }
}
