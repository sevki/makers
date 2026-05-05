extern "C" {
    pub type file;
    fn fatal(flocp: *const floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const ::core::ffi::c_char,
    pub lineno: ::core::ffi::c_ulong,
    pub offset: ::core::ffi::c_ulong,
}
#[no_mangle]
pub unsafe extern "C" fn load_file(
    mut flocp: *const floc,
    mut _file: *mut file,
    mut noerror: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if noerror == 0 {
        fatal(
            flocp,
            0,
            b"'load' is not supported on this platform\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn unload_file(mut _name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    fatal(
        ::core::ptr::null_mut::<floc>(),
        0,
        b"INTERNAL: cannot unload when load is not supported\0" as *const u8
            as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe fn unload_all() {}
