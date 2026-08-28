#[derive(Copy, Clone)]
#[repr(C)]
pub struct Floc {
    pub filenm: *const ::core::ffi::c_char,
    pub lineno: u64,
    pub offset: u64,
}
