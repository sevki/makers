use ::core::ffi::CStr;

/// A source location. `filenm` is an interned name (see
/// `crate::strcache::strcache_intern`), whose storage is never freed, so the
/// location is `Copy` and `Send`; `None` means "no source file".
#[derive(Copy, Clone)]
pub struct Floc {
    pub filenm: Option<&'static CStr>,
    pub lineno: u64,
    pub offset: u64,
}
