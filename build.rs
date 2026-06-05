// The Rust port no longer links gnulib's lib/libgnu.a. Every gnulib symbol the
// translated code references (glob, globfree, fnmatch, getloadavg, ...) is
// provided by the C library, and the one remaining gnulib-only symbol,
// find_in_given_path, is reimplemented in src/findprog.rs.
fn main() {}
