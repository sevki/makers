//! Stubs for the `load` directive: dynamic-object loading is not
//! supported in this port, so loading fails (unless `noerror`) and
//! unloading is an internal error.
//!
//! Port of the no-`MAKE_LOAD` branch of `load.c`.

use crate::fatal;
pub use crate::ffi_types::size_t;
use crate::floc::Floc;

pub type file = crate::file::File;

/// Report that `load` is unsupported, dying unless `noerror` is set.
///
/// # Safety
///
/// `flocp` must be null or point to a valid location record.
pub unsafe fn load_file(
    flocp: *const Floc,
    _file: *mut file,
    noerror: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if noerror == 0 {
        fatal!(flocp.as_ref(), "'load' is not supported on this platform");
    }
    0
}

/// Always an internal error: nothing can have been loaded.
///
/// # Safety
///
/// Never returns; safe to call with any argument (it is ignored).
pub unsafe fn unload_file(_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    fatal!(None, "INTERNAL: cannot unload when load is not supported")
}

/// No-op: there is never anything to unload.
///
/// # Safety
///
/// Always safe; unsafe only to match the caller's expectations.
pub unsafe fn unload_all() {}
