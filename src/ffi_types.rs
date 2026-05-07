pub type size_t = usize;
pub type __size_t = usize;
pub type ssize_t = isize;
pub type ptrdiff_t = isize;

pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __clock_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __sig_atomic_t = ::core::ffi::c_int;

pub type off_t = __off_t;
pub type dev_t = __dev_t;
pub type ino_t = __ino_t;
pub type time_t = __time_t;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
pub type clockid_t = __clockid_t;
pub type sig_atomic_t = __sig_atomic_t;

pub type intmax_t = ::libc::intmax_t;
pub type uintmax_t = ::libc::uintmax_t;
