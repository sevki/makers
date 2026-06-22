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
pub unsafe fn load_file(flocp: *const Floc, _file: *mut file, noerror: i32) -> i32 {
    if noerror == 0 {
        fatal!(flocp.as_ref(), "'load' is not supported on this platform");
    }
    0
}

/// Always an internal error: nothing can have been loaded.
///
/// Safe: the `_name` argument is ignored (never dereferenced) and the
/// function never returns, so it carries no safety preconditions despite its
/// C-ABI signature.
pub fn unload_file(_name: *const ::core::ffi::c_char) -> i32 {
    fatal!(None, "INTERNAL: cannot unload when load is not supported")
}

/// No-op: there is never anything to unload.
///
/// Safe: empty body, no preconditions.
pub fn unload_all() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unload_all_is_a_safe_noop() {
        // Callable from safe code (no `unsafe`); does nothing.
        unload_all();
    }
}
