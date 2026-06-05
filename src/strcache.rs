//! String interning cache.
//!
//! GNU make interns every file name, variable name, and dependency string so
//! that equal strings share a single stable, NUL-terminated buffer and can be
//! compared by pointer identity. The original C implementation — faithfully
//! reproduced by the c2rust port — hand-rolled a linked list of fixed-size
//! buffers (`struct strcache` / `struct hugestring`) plus a separate
//! open-addressed `hash_table`, driven by `xmalloc` and raw pointer
//! arithmetic.
//!
//! This is a from-scratch Rust reimplementation: a [`HashSet`] keyed on the
//! string bytes, backed by leaked allocations. make never frees interned
//! strings — they live for the whole process — so leaking each buffer is the
//! intended ownership model, and it gives us the stable address the rest of
//! the program relies on.

use std::collections::HashSet;

use ::core::ffi::{c_char, c_int, CStr};

use crate::ffi_types::size_t;

/// Interns byte strings into stable, NUL-terminated, never-freed buffers.
///
/// `table` maps a string's bytes (without the trailing NUL) to the leaked
/// buffer holding them, so identical inputs resolve to the same pointer.
/// `addrs` records every pointer handed out so [`strcache_iscached`] can answer
/// membership queries without dereferencing a possibly-foreign pointer.
#[derive(Default)]
struct Interner {
    table: HashSet<&'static [u8]>,
    addrs: HashSet<usize>,
    /// Total interning requests (cache hits + misses) — a hit-rate numerator.
    adds: u64,
    /// Bytes of backing storage allocated, including each trailing NUL.
    bytes: u64,
}

impl Interner {
    /// Return the canonical pointer for `s`, allocating a stable copy on first
    /// sight. The returned pointer addresses a NUL-terminated buffer valid for
    /// the rest of the process.
    fn intern(&mut self, s: &[u8]) -> *const c_char {
        self.adds += 1;
        if let Some(&existing) = self.table.get(s) {
            return existing.as_ptr().cast();
        }
        // Stable storage: a NUL-terminated buffer we deliberately leak so the
        // pointer stays valid forever. The key borrows the leaked bytes minus
        // that trailing NUL; leaking means the heap allocation never moves, so
        // the address we hand out is stable even as the HashSet rehashes.
        let mut buf = Vec::with_capacity(s.len() + 1);
        buf.extend_from_slice(s);
        buf.push(0);
        let leaked: &'static [u8] = Vec::leak(buf);
        let key = &leaked[..s.len()];
        self.bytes += leaked.len() as u64;
        self.table.insert(key);
        self.addrs.insert(key.as_ptr() as usize);
        key.as_ptr().cast()
    }

    /// True if `p` is a pointer this interner previously returned.
    fn contains(&self, p: *const c_char) -> bool {
        self.addrs.contains(&(p as usize))
    }
}

/// Process-wide interner. make's runtime state is single-threaded, so a
/// `static mut` accessed through one helper matches the convention used for
/// the other global caches in this crate (see `shuffle::config`).
static mut INTERNER: Option<Interner> = None;

fn interner() -> &'static mut Interner {
    unsafe { INTERNER.get_or_insert_with(Interner::default) }
}

/// Pre-create the cache and reserve room, mirroring the original 8000-slot
/// hash table. Interning is lazy, so this is only an optimization.
pub fn strcache_init() {
    interner().table.reserve(8000);
}

/// Intern the NUL-terminated C string `str` and return the canonical pointer.
///
/// # Safety
///
/// `str` must point to a valid NUL-terminated C string.
pub unsafe fn strcache_add(str: *const c_char) -> *const c_char {
    interner().intern(CStr::from_ptr(str).to_bytes())
}

/// Intern the first `len` bytes of `str` and return the canonical pointer. The
/// input need not be NUL-terminated — the cache stores its own copy.
///
/// # Safety
///
/// `str` must be valid for reads of `len` bytes.
pub unsafe fn strcache_add_len(str: *const c_char, len: size_t) -> *const c_char {
    let bytes = ::core::slice::from_raw_parts(str.cast::<u8>(), len as usize);
    interner().intern(bytes)
}

/// Returns nonzero if `str` is a pointer previously handed out by the cache.
///
/// Does not dereference `str`, so it is sound to call on any pointer value
/// (matching the original, which compared pointer ranges rather than reading
/// the string).
pub fn strcache_iscached(str: *const c_char) -> c_int {
    interner().contains(str) as c_int
}

/// Print cache statistics, prefixed with `prefix`. Used by `make -p`.
///
/// # Safety
///
/// `prefix` must point to a valid NUL-terminated C string.
pub unsafe fn strcache_print_stats(prefix: *const c_char) {
    let prefix = CStr::from_ptr(prefix).to_string_lossy();
    let it = interner();
    let strings = it.table.len() as u64;
    let avg = if strings > 0 { it.bytes / strings } else { 0 };
    let hit_rate = if it.adds > 0 {
        100 * it.adds.saturating_sub(strings) / it.adds
    } else {
        0
    };
    // Route through C stdio (like the rest of make's output) so the stats
    // interleave correctly with the surrounding `make -p` dump.
    let out = format!(
        "\n{prefix} strcache: strings = {strings} / storage = {bytes} B / avg = {avg} B\n\
         {prefix} strcache performance: lookups = {adds} / hit rate = {hit_rate}%\n\0",
        bytes = it.bytes,
        adds = it.adds,
    );
    libc::printf(b"%s\0".as_ptr().cast(), out.as_ptr());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interns_equal_strings_to_one_pointer() {
        let mut it = Interner::default();
        let a = it.intern(b"foo");
        let b = it.intern(b"foo");
        assert_eq!(a, b, "equal strings must share a pointer");

        let c = it.intern(b"bar");
        assert_ne!(a, c, "distinct strings get distinct pointers");

        // The returned pointer is a NUL-terminated copy of the input.
        unsafe {
            assert_eq!(CStr::from_ptr(a).to_bytes(), b"foo");
            assert_eq!(CStr::from_ptr(c).to_bytes(), b"bar");
        }
    }

    #[test]
    fn add_len_ignores_trailing_bytes() {
        let mut it = Interner::default();
        // Intern only the first 3 bytes of a longer, non-terminated buffer.
        let p = it.intern(&b"foobar"[..3]);
        unsafe {
            assert_eq!(CStr::from_ptr(p).to_bytes(), b"foo");
        }
        // Must collide with the canonical "foo".
        assert_eq!(p, it.intern(b"foo"));
    }

    #[test]
    fn iscached_matches_only_returned_pointers() {
        let mut it = Interner::default();
        let p = it.intern(b"cached");
        assert!(it.contains(p));
        // A foreign pointer is never reported as cached.
        let foreign = b"cached\0".as_ptr().cast::<c_char>();
        assert!(!it.contains(foreign));
    }

    #[test]
    fn empty_string_round_trips() {
        let mut it = Interner::default();
        let e = it.intern(b"");
        unsafe {
            assert_eq!(CStr::from_ptr(e).to_bytes(), b"");
        }
        assert_eq!(e, it.intern(b""), "empty string is interned once");
        assert!(it.contains(e));
    }
}
