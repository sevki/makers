#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const ::core::ffi::c_char,
    pub lineno: ::core::ffi::c_ulong,
    pub offset: ::core::ffi::c_ulong,
}
#[no_mangle]
pub unsafe extern "C" fn guile_gmake_setup(mut _flocp: *const floc) -> ::core::ffi::c_int {
    1
}
