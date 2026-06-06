//! String interning cache.
//!
//! GNU make interns every file name, variable name, and dependency string so
//! that equal strings share a single stable, NUL-terminated buffer and can be
//! compared by pointer identity. The original C implementation — faithfully
//! reproduced by the c2rust port — hand-rolled a linked list of fixed-size
//! buffers plus a separate open-addressed `hash_table`.
//!
//! This implementation is backed by the [`ustr`] global string interner, which
//! stores each unique string once in a leaked, NUL-terminated, address-stable
//! buffer — exactly make's ownership model (interned strings live for the whole
//! process). [`Ustr::as_char_ptr`] hands back the `*const c_char` callers want.
//!
//! Two things `ustr` doesn't cover, handled here:
//!
//! * **Non-UTF-8 names.** `ustr` only interns `&str`, and its C constructor
//!   lossily replaces invalid bytes with U+FFFD — unacceptable for make, whose
//!   file names are arbitrary OS bytes. Valid UTF-8 goes through `ustr`; the
//!   rare non-UTF-8 string is interned faithfully into a local byte set instead.
//! * **`strcache_iscached`.** It asks whether a *raw pointer* came from the
//!   cache without dereferencing it; `ustr` has no such query, so every pointer
//!   we hand out is recorded in an address set.

use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};

use ::core::ffi::{c_char, c_int, CStr};

use ustr::Ustr;

use crate::ffi_types::size_t;

/// Intern `bytes`, returning the canonical NUL-terminated pointer.
///
/// `addrs` accumulates every pointer handed out (for [`strcache_iscached`]).
/// `non_utf8` faithfully interns byte strings that aren't valid UTF-8 and so
/// can't be passed to `ustr` without corruption. Taking both by reference keeps
/// this core testable on local state, independent of the process globals.
fn intern_into(
    addrs: &mut HashSet<usize>,
    non_utf8: &mut HashSet<&'static [u8]>,
    bytes: &[u8],
) -> *const c_char {
    let ptr = match ::core::str::from_utf8(bytes) {
        Ok(s) => Ustr::from(s).as_char_ptr(),
        Err(_) => intern_bytes(non_utf8, bytes),
    };
    addrs.insert(ptr as usize);
    ptr
}

/// Fallback interner for non-UTF-8 byte strings: stable, leaked, NUL-terminated
/// storage with the same one-pointer-per-distinct-string guarantee as `ustr`.
fn intern_bytes(set: &mut HashSet<&'static [u8]>, bytes: &[u8]) -> *const c_char {
    if let Some(&existing) = set.get(bytes) {
        return existing.as_ptr().cast();
    }
    let mut buf = Vec::with_capacity(bytes.len() + 1);
    buf.extend_from_slice(bytes);
    buf.push(0);
    let leaked: &'static [u8] = Vec::leak(buf);
    let key = &leaked[..bytes.len()];
    set.insert(key);
    key.as_ptr().cast()
}

// Process globals. make's runtime state is single-threaded, so `static mut`
// accessed through one helper matches the convention used for the crate's other
// global caches (see `shuffle::config`). `ustr`'s own cache is global already.
static mut ADDRS: Option<HashSet<usize>> = None;
static mut NON_UTF8: Option<HashSet<&'static [u8]>> = None;
/// Total interning requests (hits + misses) — the hit-rate numerator.
static ADDS: AtomicU64 = AtomicU64::new(0);

fn addrs() -> &'static mut HashSet<usize> {
    unsafe { ADDRS.get_or_insert_with(HashSet::new) }
}

fn non_utf8() -> &'static mut HashSet<&'static [u8]> {
    unsafe { NON_UTF8.get_or_insert_with(HashSet::new) }
}

fn intern(bytes: &[u8]) -> *const c_char {
    ADDS.fetch_add(1, Ordering::Relaxed);
    intern_into(addrs(), non_utf8(), bytes)
}

/// Nothing to set up — `ustr`'s cache initializes lazily on first use.
pub fn strcache_init() {}

/// Intern the NUL-terminated C string `str` and return the canonical pointer.
///
/// # Safety
///
/// `str` must point to a valid NUL-terminated C string.
pub unsafe fn strcache_add(str: *const c_char) -> *const c_char {
    intern(CStr::from_ptr(str).to_bytes())
}

/// Intern the first `len` bytes of `str` and return the canonical pointer. The
/// input need not be NUL-terminated — the cache stores its own copy.
///
/// # Safety
///
/// `str` must be valid for reads of `len` bytes.
pub unsafe fn strcache_add_len(str: *const c_char, len: size_t) -> *const c_char {
    intern(::core::slice::from_raw_parts(str.cast::<u8>(), len as usize))
}

/// Returns nonzero if `str` is a pointer previously handed out by the cache.
///
/// Does not dereference `str`, so it is sound to call on any pointer value
/// (matching the original, which compared pointer ranges rather than reading the
/// string).
pub fn strcache_iscached(str: *const c_char) -> c_int {
    addrs().contains(&(str as usize)) as c_int
}

/// Print cache statistics, prefixed with `prefix`. Used by `make -p`.
///
/// # Safety
///
/// `prefix` must point to a valid NUL-terminated C string.
pub unsafe fn strcache_print_stats(prefix: *const c_char) {
    let prefix = CStr::from_ptr(prefix).to_string_lossy();
    let strings = (ustr::num_entries() + non_utf8().len()) as u64;
    let bytes = ustr::total_allocated() as u64;
    let adds = ADDS.load(Ordering::Relaxed);
    let avg = if strings > 0 { bytes / strings } else { 0 };
    let hit_rate = if adds > 0 {
        100 * adds.saturating_sub(strings) / adds
    } else {
        0
    };
    // Route through C stdio (like the rest of make's output) so the stats
    // interleave correctly with the surrounding `make -p` dump.
    let out = format!(
        "\n{prefix} strcache: strings = {strings} / storage = {bytes} B / avg = {avg} B\n\
         {prefix} strcache performance: lookups = {adds} / hit rate = {hit_rate}%\n\0",
    );
    libc::printf(b"%s\0".as_ptr().cast(), out.as_ptr());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> (HashSet<usize>, HashSet<&'static [u8]>) {
        (HashSet::new(), HashSet::new())
    }

    #[test]
    fn interns_equal_strings_to_one_pointer() {
        let (mut a, mut n) = fresh();
        let p = intern_into(&mut a, &mut n, b"strcache-test-foo");
        let q = intern_into(&mut a, &mut n, b"strcache-test-foo");
        assert_eq!(p, q, "equal strings must share a pointer");

        let r = intern_into(&mut a, &mut n, b"strcache-test-bar");
        assert_ne!(p, r, "distinct strings get distinct pointers");

        unsafe {
            assert_eq!(CStr::from_ptr(p).to_bytes(), b"strcache-test-foo");
            assert_eq!(CStr::from_ptr(r).to_bytes(), b"strcache-test-bar");
        }
        assert!(a.contains(&(p as usize)) && a.contains(&(r as usize)));
    }

    #[test]
    fn add_len_ignores_trailing_bytes() {
        let (mut a, mut n) = fresh();
        // Intern only the first 3 bytes of a longer, non-terminated buffer.
        let p = intern_into(&mut a, &mut n, &b"foobar"[..3]);
        unsafe {
            assert_eq!(CStr::from_ptr(p).to_bytes(), b"foo");
        }
        assert_eq!(p, intern_into(&mut a, &mut n, b"foo"));
    }

    #[test]
    fn non_utf8_is_interned_faithfully() {
        // The whole reason for the byte fallback: ustr's C constructor would
        // lossily mangle these bytes into U+FFFD. We must store them verbatim.
        let (mut a, mut n) = fresh();
        let raw: &[u8] = b"bad\xff\xfename";
        let p = intern_into(&mut a, &mut n, raw);
        unsafe {
            assert_eq!(CStr::from_ptr(p).to_bytes(), raw, "bytes must survive intact");
        }
        // Identity and membership hold for the non-UTF-8 path too.
        assert_eq!(p, intern_into(&mut a, &mut n, raw));
        assert!(a.contains(&(p as usize)));
        assert!(!a.contains(&(b"other".as_ptr() as usize)));
    }

    #[test]
    fn empty_string_round_trips() {
        let (mut a, mut n) = fresh();
        let e = intern_into(&mut a, &mut n, b"");
        unsafe {
            assert_eq!(CStr::from_ptr(e).to_bytes(), b"");
        }
        assert_eq!(e, intern_into(&mut a, &mut n, b""));
    }
}
