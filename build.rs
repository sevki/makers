#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={manifest}/lib");
    println!("cargo:rustc-link-lib=static=gnu");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
