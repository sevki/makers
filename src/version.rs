pub const PACKAGE_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
pub const MAKE_HOST: &str = "x86_64-pc-linux-gnu\0";
#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "musl"))]
pub const MAKE_HOST: &str = "x86_64-unknown-linux-musl\0";
#[cfg(all(target_arch = "aarch64", target_os = "linux", target_env = "gnu"))]
pub const MAKE_HOST: &str = "aarch64-unknown-linux-gnu\0";
#[cfg(all(target_arch = "aarch64", target_os = "linux", target_env = "musl"))]
pub const MAKE_HOST: &str = "aarch64-unknown-linux-musl\0";
#[cfg(all(target_arch = "x86_64", target_os = "macos"))]
pub const MAKE_HOST: &str = "x86_64-apple-darwin\0";
#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
pub const MAKE_HOST: &str = "aarch64-apple-darwin\0";
#[cfg(not(any(
    all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"),
    all(target_arch = "x86_64", target_os = "linux", target_env = "musl"),
    all(target_arch = "aarch64", target_os = "linux", target_env = "gnu"),
    all(target_arch = "aarch64", target_os = "linux", target_env = "musl"),
    all(target_arch = "x86_64", target_os = "macos"),
    all(target_arch = "aarch64", target_os = "macos"),
)))]
pub const MAKE_HOST: &str = "unknown-unknown-unknown\0";

pub fn version_string() -> *const ::core::ffi::c_char {
    PACKAGE_VERSION.as_ptr().cast()
}

pub fn make_host() -> *const ::core::ffi::c_char {
    MAKE_HOST.as_ptr().cast()
}
