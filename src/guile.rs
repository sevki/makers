use crate::floc::Floc;

#[no_mangle]
pub unsafe extern "C" fn guile_gmake_setup(mut _flocp: *const Floc) -> ::core::ffi::c_int {
    1
}
