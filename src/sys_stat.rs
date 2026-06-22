// Canonical FFI types for <sys/stat.h>.
//
// `stat` here is byte-for-byte equivalent to glibc's `struct stat64` on
// x86_64 Linux (which is what autoconf produces with _FILE_OFFSET_BITS=64).
// We hand-roll the struct rather than `pub use libc::stat64` because (a) the
// libc crate's `stat64` flattens `st_mtim`/`st_atim`/`st_ctim` into separate
// `*time`/`*time_nsec` fields, and existing call sites use the timespec
// grouping, and (b) `libc::stat64` is also a function symbol, which collides
// with local `extern fn stat` declarations.
//
// On 32-bit Linux glibc this layout still matches the compiler's view of
// `struct stat` under _FILE_OFFSET_BITS=64. Other targets (musl, BSD, macOS)
// would need their own definition.

pub use libc::timespec;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: libc::dev_t,
    pub st_ino: libc::ino64_t,
    pub st_nlink: libc::nlink_t,
    pub st_mode: libc::mode_t,
    pub st_uid: libc::uid_t,
    pub st_gid: libc::gid_t,
    pub __pad0: i32,
    pub st_rdev: libc::dev_t,
    pub st_size: libc::off_t,
    pub st_blksize: libc::blksize_t,
    pub st_blocks: libc::blkcnt64_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [::core::ffi::c_long; 3],
}
