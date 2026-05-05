pub const MAKE_HOST: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"x86_64-pc-linux-gnu\0")
};
pub const PACKAGE_VERSION: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"4.4.90\0") };
#[no_mangle]
pub static mut version_string: *const ::core::ffi::c_char = PACKAGE_VERSION.as_ptr();
#[no_mangle]
pub static mut make_host: *const ::core::ffi::c_char = MAKE_HOST.as_ptr();
