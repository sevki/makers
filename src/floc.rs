#[derive(Copy, Clone)]
#[repr(C)]
pub struct Floc {
    pub filenm: *const ::core::ffi::c_char,
    pub lineno: ::core::ffi::c_ulong,
    pub offset: ::core::ffi::c_ulong,
}

impl Floc {
    /// Safe view of `filenm` as a C string, or `None` if null. `Floc` is the
    /// one place that owns this pointer's NUL-terminated/lifetime contract,
    /// so it's the place that converts it to a real reference rather than
    /// every reader re-deriving the same unsafety.
    pub fn filenm_cstr(&self) -> Option<&::core::ffi::CStr> {
        (!self.filenm.is_null()).then(|| unsafe { ::core::ffi::CStr::from_ptr(self.filenm) })
    }
}
