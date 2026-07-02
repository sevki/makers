//! String interning cache.
//!
//! GNU make interns every file name, variable name, and dependency string so
//! that equal strings share a single stable, NUL-terminated buffer and can be
//! compared by pointer identity. The original C implementation — faithfully
//! reproduced by the c2rust port — hand-rolled a linked list of fixed-size
//! buffers plus a separate open-addressed `hash_table`.
//!
//! This implementation interns UTF-8 strings through the session salsa
//! database ([`crate::makedb::MakeDb`], owned by the `ExecContext`) and keeps
//! a byte-oriented fallback set for non-UTF-8 names. For both paths, the
//! C-facing canonical pointers are backed by leaked, NUL-terminated,
//! address-stable storage for the lifetime of the process.
//!
//! BOUNDARY: the pointer-compatibility layer (the leaked byte storage, the
//! [`strcache_iscached`] address set, and the stats counter) stays
//! process-global until the last `*const c_char` consumer is gone — canonical
//! pointers must stay valid across the `main_0` context rebuild, and sharing
//! leaked bytes between sessions is semantically harmless (interning is
//! idempotent). The salsa side is per-session.
//!
//! Two things are handled explicitly:
//!
//! * **Non-UTF-8 names.** `salsa` interns Rust `String`s, but make file names
//!   are arbitrary bytes. Valid UTF-8 goes through salsa; non-UTF-8 bytes are
//!   interned faithfully into a local byte set.
//! * **`strcache_iscached`.** It asks whether a *raw pointer* came from the
//!   cache without dereferencing it; salsa has no such query, so every pointer
//!   we hand out is recorded in an address set.

use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use core::ffi::{c_char, CStr};

use crate::ffi_types::size_t;

#[salsa::interned]
struct Utf8String<'db> {
    #[returns(ref)]
    text: String,
}

/// Intern `bytes`, returning the canonical NUL-terminated pointer.
///
/// `addrs` accumulates every pointer handed out (for [`strcache_iscached`]).
/// `db` interns valid UTF-8 strings.
/// `utf8` stores leaked UTF-8 interned bytes.
/// `non_utf8` faithfully interns byte strings that aren't valid UTF-8 and so
/// can't be represented as Rust `String`.
fn intern_into(
    db: &crate::makedb::MakeDb,
    addrs: &mut HashSet<usize>,
    utf8: &mut HashSet<&'static [u8]>,
    non_utf8: &mut HashSet<&'static [u8]>,
    bytes: &[u8],
) -> *const c_char {
    let ptr = match ::core::str::from_utf8(bytes) {
        Ok(s) => intern_utf8(db, utf8, s),
        Err(_) => intern_bytes(non_utf8, bytes),
    };
    addrs.insert(ptr as usize);
    ptr
}

/// UTF-8 path: dedupe the string through salsa, then return the canonical
/// NUL-terminated pointer from leaked byte storage.
fn intern_utf8(
    db: &crate::makedb::MakeDb,
    set: &mut HashSet<&'static [u8]>,
    value: &str,
) -> *const c_char {
    if let Some(&existing) = set.get(value.as_bytes()) {
        return existing.as_ptr().cast();
    }
    let key = Utf8String::new(db, value.to_owned());
    intern_bytes(set, key.text(db).as_bytes())
}

/// Fallback interner for non-UTF-8 byte strings: stable, leaked, NUL-terminated
/// storage with one-pointer-per-distinct-byte-string semantics.
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

static ADDRS: OnceLock<Mutex<HashSet<usize>>> = OnceLock::new();
static UTF8: OnceLock<Mutex<HashSet<&'static [u8]>>> = OnceLock::new();
static NON_UTF8: OnceLock<Mutex<HashSet<&'static [u8]>>> = OnceLock::new();
/// Total interning requests (hits + misses) — the hit-rate numerator.
static ADDS: AtomicU64 = AtomicU64::new(0);

fn addrs() -> &'static Mutex<HashSet<usize>> {
    ADDRS.get_or_init(|| Mutex::new(HashSet::new()))
}

fn utf8() -> &'static Mutex<HashSet<&'static [u8]>> {
    UTF8.get_or_init(|| Mutex::new(HashSet::new()))
}

fn non_utf8() -> &'static Mutex<HashSet<&'static [u8]>> {
    NON_UTF8.get_or_init(|| Mutex::new(HashSet::new()))
}

fn intern(ctx: &crate::execctx::ExecContext, bytes: &[u8]) -> *const c_char {
    ADDS.fetch_add(1, Ordering::Relaxed);
    let mut addrs = addrs().lock().unwrap_or_else(|e| e.into_inner());
    let mut utf8 = utf8().lock().unwrap_or_else(|e| e.into_inner());
    let mut non_utf8 = non_utf8().lock().unwrap_or_else(|e| e.into_inner());
    intern_into(&ctx.db, &mut addrs, &mut utf8, &mut non_utf8, bytes)
}

/// Nothing to set up — interners initialize lazily on first use.
pub fn strcache_init() {}

/// Intern the NUL-terminated C string `str` and return the canonical pointer.
///
/// # Safety
///
/// `str` must point to a valid NUL-terminated C string.
pub unsafe fn strcache_add(ctx: &crate::execctx::ExecContext, str: *const c_char) -> *const c_char {
    intern(ctx, CStr::from_ptr(str).to_bytes())
}

/// Intern the first `len` bytes of `str` and return the canonical pointer. The
/// input need not be NUL-terminated — the cache stores its own copy.
///
/// # Safety
///
/// `str` must be valid for reads of `len` bytes.
pub unsafe fn strcache_add_len(
    ctx: &crate::execctx::ExecContext,
    str: *const c_char,
    len: size_t,
) -> *const c_char {
    intern(ctx, ::core::slice::from_raw_parts(str.cast::<u8>(), len))
}

/// Intern an arbitrary byte slice and return the canonical, NUL-terminated
/// pointer. Safe wrapper around the interner for callers that hold genuine Rust
/// types (e.g. `&[u8]` derived from a `PathBuf`) and must not fabricate a
/// `CString`/`*const c_char` themselves. The cache stores its own copy and
/// appends the trailing NUL internally.
pub fn strcache_add_bytes(ctx: &crate::execctx::ExecContext, bytes: &[u8]) -> *const c_char {
    intern(ctx, bytes)
}

/// Returns nonzero if `str` is a pointer previously handed out by the cache.
///
/// Does not dereference `str`, so it is sound to call on any pointer value
/// (matching the original, which compared pointer ranges rather than reading the
/// string).
pub fn strcache_iscached(str: *const c_char) -> i32 {
    addrs()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .contains(&(str as usize)) as i32
}

/// Print cache statistics, prefixed with `prefix`. Used by `make -p`.
///
/// # Safety
///
/// `prefix` must point to a valid NUL-terminated C string.
pub unsafe fn strcache_print_stats(prefix: *const c_char) {
    let prefix = CStr::from_ptr(prefix).to_string_lossy();
    let utf8 = utf8().lock().unwrap_or_else(|e| e.into_inner());
    let non_utf8 = non_utf8().lock().unwrap_or_else(|e| e.into_inner());
    let strings = (utf8.len() + non_utf8.len()) as u64;
    let bytes = utf8
        .iter()
        .chain(non_utf8.iter())
        .map(|s| (s.len() + 1) as u64)
        .sum::<u64>();
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
        "\n{prefix} strcache: strings = {strings} / nul-buf = {bytes} B / avg = {avg} B\n\
         {prefix} strcache performance: lookups = {adds} / hit rate = {hit_rate}%\n\0",
    );
    libc::printf(b"%s\0".as_ptr().cast(), out.as_ptr());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> (
        crate::makedb::MakeDb,
        HashSet<usize>,
        HashSet<&'static [u8]>,
        HashSet<&'static [u8]>,
    ) {
        (
            crate::makedb::MakeDb::default(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        )
    }

    #[test]
    fn interns_equal_strings_to_one_pointer() {
        let (db, mut a, mut u, mut n) = fresh();
        let p = intern_into(&db, &mut a, &mut u, &mut n, b"strcache-test-foo");
        let q = intern_into(&db, &mut a, &mut u, &mut n, b"strcache-test-foo");
        assert_eq!(p, q, "equal strings must share a pointer");

        let r = intern_into(&db, &mut a, &mut u, &mut n, b"strcache-test-bar");
        assert_ne!(p, r, "distinct strings get distinct pointers");

        unsafe {
            assert_eq!(CStr::from_ptr(p).to_bytes(), b"strcache-test-foo");
            assert_eq!(CStr::from_ptr(r).to_bytes(), b"strcache-test-bar");
        }
        assert!(a.contains(&(p as usize)) && a.contains(&(r as usize)));
    }

    #[test]
    fn add_len_ignores_trailing_bytes() {
        let (db, mut a, mut u, mut n) = fresh();
        // Intern only the first 3 bytes of a longer, non-terminated buffer.
        let p = intern_into(&db, &mut a, &mut u, &mut n, &b"foobar"[..3]);
        unsafe {
            assert_eq!(CStr::from_ptr(p).to_bytes(), b"foo");
        }
        assert_eq!(p, intern_into(&db, &mut a, &mut u, &mut n, b"foo"));
    }

    #[test]
    fn non_utf8_is_interned_faithfully() {
        // The whole reason for the byte fallback: ustr's C constructor would
        // lossily mangle these bytes into U+FFFD. We must store them verbatim.
        let (db, mut a, mut u, mut n) = fresh();
        let raw: &[u8] = b"bad\xff\xfename";
        let p = intern_into(&db, &mut a, &mut u, &mut n, raw);
        unsafe {
            assert_eq!(
                CStr::from_ptr(p).to_bytes(),
                raw,
                "bytes must survive intact"
            );
        }
        // Identity and membership hold for the non-UTF-8 path too.
        assert_eq!(p, intern_into(&db, &mut a, &mut u, &mut n, raw));
        assert!(a.contains(&(p as usize)));
        assert!(!a.contains(&(b"other".as_ptr() as usize)));
    }

    #[test]
    fn empty_string_round_trips() {
        let (db, mut a, mut u, mut n) = fresh();
        let e = intern_into(&db, &mut a, &mut u, &mut n, b"");
        unsafe {
            assert_eq!(CStr::from_ptr(e).to_bytes(), b"");
        }
        assert_eq!(e, intern_into(&db, &mut a, &mut u, &mut n, b""));
    }

    #[test]
    fn iscached_tracks_the_global_cache() {
        // A pointer handed out by the global `strcache_add_bytes` is reported as
        // cached; an unrelated pointer is not.
        let ctx = crate::execctx::ExecContext::default();
        let p = strcache_add_bytes(&ctx, b"strcache-iscached-probe");
        assert_eq!(strcache_iscached(p), 1, "interned pointer must be cached");
        let bogus = 0xdead_beef_usize as *const c_char;
        assert_eq!(strcache_iscached(bogus), 0, "foreign pointer is not cached");
    }
}
