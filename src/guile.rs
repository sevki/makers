use crate::floc_types::floc;

#[no_mangle]
pub unsafe extern "C" fn guile_gmake_setup(mut _flocp: *const floc) -> ::core::ffi::c_int {
    1
}
