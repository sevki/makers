use crate::floc::Floc;

/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn guile_gmake_setup(mut _flocp: *const Floc) -> ::core::ffi::c_int {
    1
}
