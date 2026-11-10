//! The open-addressed hash table used for make's file, variable, and
//! directory tables, plus the Jenkins lookup3 hash.
//!
//! Port of `hash.c`. The table stores raw `void *` items and C function
//! pointers because every consumer (file.rs, variable.rs, dir.rs, ...) is
//! still keyed on interned C strings; the layout of `HashTable` is shared
//! through `#[repr(C)]`.

use ::core::{
    ffi::{c_double, c_uint, c_void},
    ptr::null_mut,
};

use libc::{exit, free};

use crate::{ffi_types::size_t, misc::xcalloc};

pub type __compar_fn_t = Option<unsafe extern "C" fn(*const c_void, *const c_void) -> i32>;
pub type hash_func_t = Option<unsafe fn(*const c_void) -> u64>;
pub type hash_cmp_func_t = Option<unsafe fn(*const c_void, *const c_void) -> i32>;
pub type hash_map_func_t = Option<unsafe fn(*const c_void)>;
pub type hash_map_arg_func_t = Option<unsafe fn(*const c_void, *mut c_void)>;
pub type qsort_cmp_t = Option<unsafe extern "C" fn(*const c_void, *const c_void) -> i32>;

/// An open-addressed (double-hashed) table of `void *` items. Deleted
/// slots hold [`hash_deleted_item`]; empty slots hold null.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashTable {
    pub ht_vec: *mut *mut c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    /// Total slots (a power of two).
    pub ht_size: u64,
    /// Fill threshold that triggers a rehash (15/16 of the size).
    pub ht_capacity: u64,
    /// Items in the table.
    pub ht_fill: u64,
    /// Slots that are neither full nor deleted.
    pub ht_empty_slots: u64,
    pub ht_collisions: u64,
    pub ht_lookups: u64,
    pub ht_rehashes: c_uint,
    pub(crate) ht_in_map: c_uint,
}

impl HashTable {
    pub fn ht_in_map(&self) -> c_uint {
        self.ht_in_map
    }

    pub fn set_ht_in_map(&mut self, val: c_uint) {
        self.ht_in_map = val;
    }
}

pub const MAKE_TROUBLE: i32 = 1;

/// Sentinel stored in slots whose item was deleted. It is only ever compared
/// for pointer identity (never dereferenced), so — unlike the c2rust
/// translation's self-referential `static mut` (whose value was its own
/// address) — any fixed, non-null value works; a `const` inlines this literal
/// at every use site instead of needing a stable process-wide storage
/// location.
pub const hash_deleted_item: *const c_void = ::core::ptr::dangling::<c_void>();

/// Is `item` a real entry (not empty, not the deleted sentinel)?
///
/// Safe: `item` is never dereferenced — it is only compared against null and
/// the deleted-item sentinel.
pub fn is_real_item(item: *const c_void) -> bool {
    !item.is_null() && item != hash_deleted_item
}

pub(crate) unsafe fn table_slots<'a>(ht: *const HashTable) -> &'a [*mut c_void] {
    let ht = ht.as_ref().expect("hash table pointer is null");
    assert!(!ht.ht_vec.is_null(), "hash table without a slot vector");
    ::core::slice::from_raw_parts(ht.ht_vec, ht.ht_size as usize)
}

unsafe fn table_slots_mut<'a>(ht: *mut HashTable) -> &'a mut [*mut c_void] {
    let ht = ht.as_mut().expect("hash table pointer is null");
    assert!(!ht.ht_vec.is_null(), "hash table without a slot vector");
    ::core::slice::from_raw_parts_mut(ht.ht_vec, ht.ht_size as usize)
}

/// Initialize `ht` with at least `size` slots (rounded up to a power of
/// two) and the given hash/compare callbacks.
///
/// # Safety
/// `ht` must point to writable storage; the callbacks must be non-null and
/// valid for the items later stored.
pub unsafe fn hash_init(
    ht: *mut HashTable,
    size: u64,
    hash_1: hash_func_t,
    hash_2: hash_func_t,
    hash_cmp: hash_cmp_func_t,
) {
    (*ht).ht_size = round_up_2(size);
    (*ht).ht_empty_slots = (*ht).ht_size;
    (*ht).ht_vec = xcalloc(::core::mem::size_of::<*mut c_void>() * (*ht).ht_size as size_t)
        as *mut *mut c_void;
    if (*ht).ht_vec.is_null() {
        use std::io::Write;
        let _ = write!(
            std::io::stderr(),
            "can't allocate {} bytes for hash table: memory exhausted",
            (*ht).ht_size * ::core::mem::size_of::<*mut c_void>() as u64,
        );
        exit(MAKE_TROUBLE);
    }

    (*ht).ht_capacity = (*ht).ht_size - (*ht).ht_size / 16;
    (*ht).ht_fill = 0;
    (*ht).ht_collisions = 0;
    (*ht).ht_lookups = 0;
    (*ht).ht_rehashes = 0;
    (*ht).set_ht_in_map(0);
    (*ht).ht_hash_1 = hash_1;
    (*ht).ht_hash_2 = hash_2;
    (*ht).ht_compare = hash_cmp;
}

/// Load an array of `cardinality` items, each `size` bytes apart, into the
/// table.
///
/// # Safety
/// `item_table` must point to `cardinality * size` valid bytes whose rows
/// are valid items for this table.
pub unsafe fn hash_load(
    ht: *mut HashTable,
    item_table: *const c_void,
    cardinality: u64,
    size: u64,
) {
    if cardinality == 0 {
        return;
    }
    let row_size = size as usize;
    let total_size = (cardinality as usize)
        .checked_mul(row_size)
        .expect("hash_load item table size overflow");
    let items = ::core::slice::from_raw_parts(item_table as *const u8, total_size);
    for item in items.chunks_exact(row_size) {
        hash_insert(ht, item.as_ptr() as *const c_void);
    }
}

/// Return the slot for `key`: the item's slot if present, otherwise the
/// slot where it would be inserted (reusing the first deleted slot seen).
///
/// # Safety
/// `ht` must be initialized and `key` valid for its callbacks.
pub unsafe fn hash_find_slot(ht: *mut HashTable, key: *const c_void) -> *mut *mut c_void {
    // Index of the first deleted slot seen, reused for insertion. Tracking it
    // as an `Option<usize>` rather than a nullable raw pointer keeps the
    // returned slot pointer always valid (never a null sentinel).
    let mut deleted_idx: Option<usize> = None;
    let mut hash_2: c_uint = 0;
    let mut hash_1 = ht
        .as_ref()
        .expect("hash table pointer is null")
        .ht_hash_1
        .expect("hash table without ht_hash_1")(key) as c_uint;

    ht.as_mut().expect("hash table pointer is null").ht_lookups = ht
        .as_ref()
        .expect("hash table pointer is null")
        .ht_lookups
        .wrapping_add(1);
    loop {
        // ht_size is a power of two, so this is "hash_1 % size".
        hash_1 = (hash_1 as u64 & (ht.as_ref().expect("hash table pointer is null").ht_size - 1))
            as c_uint;
        let idx = hash_1 as usize;
        let slot_val = *table_slots_mut(ht)
            .get(idx)
            .expect("hash index within table size");

        if slot_val.is_null() {
            // Empty slot: insert here, or reuse an earlier deleted slot.
            let target = deleted_idx.unwrap_or(idx);
            return &raw mut table_slots_mut(ht)[target];
        }
        if ::core::ptr::eq(slot_val, hash_deleted_item as *mut c_void) {
            if deleted_idx.is_none() {
                deleted_idx = Some(idx);
            }
        } else {
            if ::core::ptr::eq(key, slot_val) {
                return &raw mut table_slots_mut(ht)[idx];
            }
            if ht
                .as_ref()
                .expect("hash table pointer is null")
                .ht_compare
                .expect("hash table without ht_compare")(key, slot_val)
                == 0
            {
                return &raw mut table_slots_mut(ht)[idx];
            }
            ht.as_mut()
                .expect("hash table pointer is null")
                .ht_collisions = ht
                .as_ref()
                .expect("hash table pointer is null")
                .ht_collisions
                .wrapping_add(1);
        }

        // Probe again with the secondary hash (forced odd, so it is
        // coprime with the power-of-two size).
        if hash_2 == 0 {
            hash_2 = (ht
                .as_ref()
                .expect("hash table pointer is null")
                .ht_hash_2
                .expect("hash table without ht_hash_2")(key)
                | 1) as c_uint;
        }
        hash_1 = hash_1.wrapping_add(hash_2);
    }
}

/// Return the item matching `key`, or null.
///
/// # Safety
/// `ht` must be initialized and `key` valid for its callbacks.
pub unsafe fn hash_find_item(ht: *mut HashTable, key: *const c_void) -> *mut c_void {
    let slot = hash_find_slot(ht, key)
        .as_mut()
        .expect("hash_find_slot always returns a slot");
    if is_real_item(*slot) {
        *slot
    } else {
        null_mut()
    }
}

/// Insert `item`, returning the previous item with the same key (or null).
///
/// # Safety
/// `ht` must be initialized and `item` valid for its callbacks and for the
/// table's lifetime.
pub unsafe fn hash_insert(ht: *mut HashTable, item: *const c_void) -> *mut c_void {
    let slot = hash_find_slot(ht, item)
        .as_mut()
        .expect("hash_find_slot always returns a slot");
    let old_item: *mut c_void = *slot;
    hash_insert_at(ht, item, (&raw mut *slot).cast());
    if is_real_item(old_item) {
        old_item
    } else {
        null_mut()
    }
}

/// Insert `item` into the `slot` previously returned by
/// [`hash_find_slot`]. Returns the (possibly moved, after a rehash) slot.
///
/// # Safety
/// `slot` must come from `hash_find_slot` on this table with `item`'s key,
/// with no intervening modification.
pub unsafe fn hash_insert_at(
    ht: *mut HashTable,
    item: *const c_void,
    slot: *const c_void,
) -> *mut c_void {
    let slot = (slot as *mut *const c_void)
        .as_mut()
        .expect("hash_insert_at: null slot");
    let old_item: *const c_void = *slot;

    assert!((*ht).ht_in_map() == 0, "hash table modified during mapping");

    if !is_real_item(old_item) {
        (*ht).ht_fill = (*ht).ht_fill.wrapping_add(1);
        if old_item.is_null() {
            (*ht).ht_empty_slots = (*ht).ht_empty_slots.wrapping_sub(1);
        }
    }
    *slot = item;

    if (*ht).ht_empty_slots < (*ht).ht_size - (*ht).ht_capacity {
        hash_rehash(ht);
        hash_find_slot(ht, item) as *mut c_void
    } else {
        (&raw mut *slot) as *mut c_void
    }
}

/// Delete the item matching `item`'s key. Returns the deleted item or
/// null.
///
/// # Safety
/// `ht` must be initialized and `item` valid for its callbacks.
pub unsafe fn hash_delete(ht: *mut HashTable, item: *const c_void) -> *mut c_void {
    let slot = hash_find_slot(ht, item)
        .as_mut()
        .expect("hash_find_slot always returns a slot");
    hash_delete_at(ht, (&raw mut *slot).cast())
}

/// Delete whatever occupies `slot` (from [`hash_find_slot`]). Returns the
/// deleted item or null.
///
/// # Safety
/// `slot` must come from `hash_find_slot` on this table with no
/// intervening modification.
pub unsafe fn hash_delete_at(ht: *mut HashTable, slot: *const c_void) -> *mut c_void {
    let slot = (slot as *mut *const c_void)
        .as_mut()
        .expect("hash_delete_at: null slot");
    let item = *slot as *mut c_void;
    if is_real_item(item) {
        *slot = hash_deleted_item;
        (*ht).ht_fill = (*ht).ht_fill.wrapping_sub(1);
        item
    } else {
        null_mut()
    }
}

/// `free` every item and clear the table (the vector itself is kept).
///
/// # Safety
/// Every stored item must be an owned `malloc`-family allocation.
pub unsafe fn hash_free_items(ht: *mut HashTable) {
    assert!(
        ht.as_ref().expect("hash table pointer is null").ht_in_map() == 0,
        "hash table modified during mapping"
    );
    for slot in table_slots_mut(ht) {
        if is_real_item(*slot) {
            free(*slot);
        }
        *slot = null_mut();
    }
    let htr = ht.as_mut().expect("hash table pointer is null");
    htr.ht_fill = 0;
    htr.ht_empty_slots = htr.ht_size;
}

/// Clear the table without freeing the items, resetting the statistics.
///
/// # Safety
/// `ht` must be initialized.
pub unsafe fn hash_delete_items(ht: *mut HashTable) {
    assert!(
        ht.as_ref().expect("hash table pointer is null").ht_in_map() == 0,
        "hash table modified during mapping"
    );
    table_slots_mut(ht).fill(null_mut());
    let htr = ht.as_mut().expect("hash table pointer is null");
    htr.ht_fill = 0;
    htr.ht_collisions = 0;
    htr.ht_lookups = 0;
    htr.ht_rehashes = 0;
    htr.ht_empty_slots = htr.ht_size;
}

/// Free the table's vector (and, when `free_items`, the items too).
///
/// # Safety
/// `ht` must be initialized; with `free_items` every stored item must be
/// an owned allocation.
pub unsafe fn hash_free(ht: *mut HashTable, free_items: i32) {
    assert!(
        ht.as_ref().expect("hash table pointer is null").ht_in_map() == 0,
        "hash table modified during mapping"
    );
    if free_items != 0 {
        hash_free_items(ht);
    } else {
        let htr = ht.as_mut().expect("hash table pointer is null");
        htr.ht_fill = 0;
        htr.ht_empty_slots = htr.ht_size;
    }
    let htr = ht.as_mut().expect("hash table pointer is null");
    free(htr.ht_vec as *mut c_void);
    htr.ht_vec = null_mut();
    htr.ht_capacity = 0;
}

/// Call `map` on every item. The table must not be modified while mapping.
///
/// # Safety
/// `ht` must be initialized and `map` non-null.
pub unsafe fn hash_map(ht: *mut HashTable, map: hash_map_func_t) {
    let map = map.expect("hash_map without callback");
    ht.as_mut()
        .expect("hash table pointer is null")
        .set_ht_in_map(1);
    for &item in table_slots(ht) {
        if is_real_item(item) {
            map(item);
        }
    }
    ht.as_mut()
        .expect("hash table pointer is null")
        .set_ht_in_map(0);
}

/// Call `map(item, arg)` on every item. The table must not be modified
/// while mapping.
///
/// # Safety
/// `ht` must be initialized and `map` non-null.
pub unsafe fn hash_map_arg(ht: *mut HashTable, map: hash_map_arg_func_t, arg: *mut c_void) {
    let map = map.expect("hash_map_arg without callback");
    (*ht).set_ht_in_map(1);
    for &item in table_slots(ht) {
        if is_real_item(item) {
            map(item, arg);
        }
    }
    (*ht).set_ht_in_map(0);
}

/// Re-bucket every item, doubling the size when the table is at capacity
/// (also used as-is to flush deleted slots).
///
/// # Safety
/// `ht` must be initialized.
pub unsafe fn hash_rehash(ht: *mut HashTable) {
    let old_ht_size = (*ht).ht_size;
    let old_vec = (*ht).ht_vec;
    let old_slots = ::core::slice::from_raw_parts(old_vec, old_ht_size as usize);

    if (*ht).ht_fill >= (*ht).ht_capacity {
        (*ht).ht_size *= 2;
        (*ht).ht_capacity = (*ht).ht_size - ((*ht).ht_size >> 4);
    }
    (*ht).ht_rehashes = (*ht).ht_rehashes.wrapping_add(1);
    (*ht).ht_vec = xcalloc(::core::mem::size_of::<*mut c_void>() * (*ht).ht_size as size_t)
        as *mut *mut c_void;

    for &old_item in old_slots {
        if is_real_item(old_item) {
            let slot = hash_find_slot(ht, old_item)
                .as_mut()
                .expect("hash_find_slot always returns a slot");
            *slot = old_item;
        }
    }
    (*ht).ht_empty_slots = (*ht).ht_size - (*ht).ht_fill;
    free(old_vec as *mut c_void);
}

/// The `hash_print_stats` line for the given counters. `{:.0}` rounds
/// half-to-even exactly like the C `%.0f` these lines were printed with.
fn hash_stats_string(
    fill: u64,
    size: u64,
    rehashes: c_uint,
    collisions: u64,
    lookups: u64,
) -> String {
    format!(
        "Load={}/{}={:.0}%, Rehash={}, Collisions={}/{}={:.0}%",
        fill,
        size,
        100.0f64 * fill as c_double / size as c_double,
        rehashes,
        collisions,
        lookups,
        if lookups != 0 {
            100.0f64 * collisions as c_double / lookups as c_double
        } else {
            0.0f64
        },
    )
}

/// Print load/rehash/collision statistics to stdout (used by `make -p`);
/// the caller supplies surrounding prefix/newline bytes.
///
/// # Safety
/// `ht` must be initialized.
pub unsafe fn hash_print_stats(ht: *mut HashTable) {
    let stats = hash_stats_string(
        (*ht).ht_fill,
        (*ht).ht_size,
        (*ht).ht_rehashes,
        (*ht).ht_collisions,
        (*ht).ht_lookups,
    );
    crate::output::trace_out(stats.as_bytes());
}

/// Round up to the next power of two by bit-smearing. Note this is NOT
/// `next_power_of_two`: exact powers of two are doubled (16 -> 32), which
/// the table relies on for its capacity margin. Equivalently, the result is
/// `1 << bit_length(n)` (and `1` for `n == 0`).
pub fn round_up_2(mut n: u64) -> u64 {
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n.wrapping_add(1)
}

// ---------------------------------------------------------------------------
// Jenkins lookup3 hash (jhash), as used by the Linux kernel.

pub const JHASH_INITVAL: c_uint = 0xdeadbeef;
pub const UINTSZ: usize = ::core::mem::size_of::<c_uint>();

/// One round of the lookup3 mixer.
macro_rules! jhash_mix {
    ($a:expr, $b:expr, $c:expr) => {
        $a = $a.wrapping_sub($c);
        $a ^= $c.rotate_left(4);
        $c = $c.wrapping_add($b);
        $b = $b.wrapping_sub($a);
        $b ^= $a.rotate_left(6);
        $a = $a.wrapping_add($c);
        $c = $c.wrapping_sub($b);
        $c ^= $b.rotate_left(8);
        $b = $b.wrapping_add($a);
        $a = $a.wrapping_sub($c);
        $a ^= $c.rotate_left(16);
        $c = $c.wrapping_add($b);
        $b = $b.wrapping_sub($a);
        $b ^= $a.rotate_left(19);
        $a = $a.wrapping_add($c);
        $c = $c.wrapping_sub($b);
        $c ^= $b.rotate_left(4);
        $b = $b.wrapping_add($a);
    };
}

/// The lookup3 finalizer.
macro_rules! jhash_final {
    ($a:expr, $b:expr, $c:expr) => {
        $c ^= $b;
        $c = $c.wrapping_sub($b.rotate_left(14));
        $a ^= $c;
        $a = $a.wrapping_sub($c.rotate_left(11));
        $b ^= $a;
        $b = $b.wrapping_sub($a.rotate_left(25));
        $c ^= $b;
        $c = $c.wrapping_sub($b.rotate_left(16));
        $a ^= $c;
        $a = $a.wrapping_sub($c.rotate_left(4));
        $b ^= $a;
        $b = $b.wrapping_sub($a.rotate_left(14));
        $c ^= $b;
        $c = $c.wrapping_sub($b.rotate_left(24));
    };
}

/// Hash the `bytes` slice (the lookup3 `jhash` mix used by make's hash tables).
pub fn jhash(bytes: &[u8]) -> c_uint {
    let mut c = JHASH_INITVAL.wrapping_add(bytes.len() as c_uint);
    let mut b = c;
    let mut a = b;

    let mut blocks = bytes;
    while blocks.len() > 12 {
        a = a.wrapping_add(load_partial_word(&blocks[..4]));
        b = b.wrapping_add(load_partial_word(&blocks[4..8]));
        c = c.wrapping_add(load_partial_word(&blocks[8..12]));
        jhash_mix!(a, b, c);
        blocks = &blocks[12..];
    }

    if blocks.is_empty() {
        return c;
    }
    if blocks.len() > 8 {
        a = a.wrapping_add(load_partial_word(&blocks[..4]));
        blocks = &blocks[4..];
    }
    if blocks.len() > 4 {
        b = b.wrapping_add(load_partial_word(&blocks[..4]));
        blocks = &blocks[4..];
    }
    if blocks.len() == 4 {
        c = c.wrapping_add((blocks[3] as c_uint) << 24);
    }
    if blocks.len() >= 3 {
        c = c.wrapping_add((blocks[2] as c_uint) << 16);
    }
    if blocks.len() >= 2 {
        c = c.wrapping_add((blocks[1] as c_uint) << 8);
    }
    c = c.wrapping_add(blocks[0] as c_uint);

    jhash_final!(a, b, c);
    c
}

fn load_partial_word(bytes: &[u8]) -> c_uint {
    let mut word = [0u8; UINTSZ];
    let len = bytes.len().min(UINTSZ);
    word[..len].copy_from_slice(&bytes[..len]);
    c_uint::from_ne_bytes(word)
}

/// Hash the byte string `bytes` (lookup3 over words, mixing in the consumed
/// length at the end). Callers pass the string content without its NUL, e.g.
/// `CStr::from_ptr(k).to_bytes()`.
pub fn jhash_string(bytes: &[u8]) -> c_uint {
    let mut chunks = bytes.chunks_exact(UINTSZ);

    let mut a = JHASH_INITVAL;
    let mut b = JHASH_INITVAL;
    let mut c = JHASH_INITVAL;

    loop {
        for lane in 0..3 {
            let word = match chunks.next() {
                Some(chunk) => load_partial_word(chunk),
                None => {
                    let remainder = chunks.remainder();
                    if !remainder.is_empty() {
                        let acc = match lane {
                            0 => &mut a,
                            1 => &mut b,
                            _ => &mut c,
                        };
                        *acc = acc.wrapping_add(load_partial_word(remainder));
                    }
                    jhash_final!(a, b, c);
                    return c.wrapping_add((bytes.len() / UINTSZ * UINTSZ) as c_uint);
                }
            };
            match lane {
                0 => a = a.wrapping_add(word),
                1 => b = b.wrapping_add(word),
                _ => c = c.wrapping_add(word),
            }
        }
        jhash_mix!(a, b, c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_real_item_classifies_null_sentinel_and_real() {
        // Safe `fn`: null and the deleted-item sentinel are not real; any other
        // non-null pointer is.
        assert!(!is_real_item(::core::ptr::null()));
        assert!(!is_real_item(hash_deleted_item));
        let local = 0u8;
        let real = (&raw const local).cast::<c_void>();
        assert!(is_real_item(real));
    }

    #[test]
    fn hash_stats_string_matches_c_percent_formatting() {
        // The exact `Load=%lu/%lu=%.0f%%, Rehash=%u, Collisions=%lu/%lu=%.0f%%`
        // bytes glibc printed, including rounding of the percentages.
        assert_eq!(
            hash_stats_string(219, 1024, 0, 31, 251),
            "Load=219/1024=21%, Rehash=0, Collisions=31/251=12%"
        );
        assert_eq!(
            hash_stats_string(1, 2, 3, 2, 3),
            "Load=1/2=50%, Rehash=3, Collisions=2/3=67%"
        );
    }

    #[test]
    fn hash_stats_string_zero_lookups_prints_zero_percent() {
        // The lookups==0 guard takes the 0.0 branch instead of dividing.
        assert_eq!(
            hash_stats_string(0, 16, 0, 0, 0),
            "Load=0/16=0%, Rehash=0, Collisions=0/0=0%"
        );
    }

    #[test]
    fn hash_print_stats_reads_the_table_counters() {
        // Exercise the unsafe wrapper over a real table: it must read the
        // counters without touching the item vector.
        unsafe {
            let mut ht: HashTable = ::core::mem::zeroed();
            hash_init(&raw mut ht, 4, None, None, None);
            hash_print_stats(&raw mut ht);
            hash_free(&raw mut ht, 0);
        }
    }

    #[test]
    fn round_up_2_known_values() {
        // 0 maps to 1; otherwise the result is the next power of two strictly
        // above the input's top set bit, so exact powers of two are doubled.
        assert_eq!(round_up_2(0), 1);
        assert_eq!(round_up_2(1), 2);
        assert_eq!(round_up_2(2), 4);
        assert_eq!(round_up_2(3), 4);
        assert_eq!(round_up_2(15), 16);
        assert_eq!(round_up_2(16), 32);
        assert_eq!(round_up_2(17), 32);
        assert_eq!(round_up_2(1000), 1024);
    }

    #[test]
    fn round_up_2_matches_bit_length_formula() {
        // For every input the result equals `1 << bit_length(n)` (with
        // `bit_length(0) == 0`), which is what the hash table relies on.
        for n in (0u64..4096).chain([
            65_535,
            65_536,
            65_537,
            (1 << 31) - 1,
            1 << 31,
            (1 << 32) + 1,
            (1 << 62) + 12345,
        ]) {
            let expected = 1u64 << (64 - n.leading_zeros());
            assert_eq!(round_up_2(n), expected, "round_up_2({n})");
        }
    }

    fn legacy_jhash(bytes: &[u8]) -> c_uint {
        let mut length = bytes.len();
        let mut offset = 0usize;
        let mut c = JHASH_INITVAL.wrapping_add(length as c_uint);
        let mut b = c;
        let mut a = b;

        while length > 12 {
            a = a.wrapping_add(load_partial_word(&bytes[offset..offset + 4]));
            b = b.wrapping_add(load_partial_word(&bytes[offset + 4..offset + 8]));
            c = c.wrapping_add(load_partial_word(&bytes[offset + 8..offset + 12]));
            jhash_mix!(a, b, c);
            length -= 12;
            offset += 12;
        }

        if length == 0 {
            return c;
        }
        if length > 8 {
            a = a.wrapping_add(load_partial_word(&bytes[offset..offset + 4]));
            length -= 4;
            offset += 4;
        }
        if length > 4 {
            b = b.wrapping_add(load_partial_word(&bytes[offset..offset + 4]));
            length -= 4;
            offset += 4;
        }
        if length == 4 {
            c = c.wrapping_add((bytes[offset + 3] as c_uint) << 24);
        }
        if length >= 3 {
            c = c.wrapping_add((bytes[offset + 2] as c_uint) << 16);
        }
        if length >= 2 {
            c = c.wrapping_add((bytes[offset + 1] as c_uint) << 8);
        }
        c = c.wrapping_add(bytes[offset] as c_uint);

        jhash_final!(a, b, c);
        c
    }

    #[test]
    fn jhash_matches_legacy_pointer_loop() {
        for input in [
            &b""[..],
            b"a",
            b"abc",
            b"abcd",
            b"abcde",
            b"abcdefgh",
            b"abcdefghijkl",
            b"abcdefghijklm",
            b"abcdefghijklmnopqrstuvw",
        ] {
            let actual = jhash(input);
            assert_eq!(actual, legacy_jhash(input), "{input:?}");
        }
    }

    fn legacy_string_word(bytes: &[u8], remaining: usize) -> (c_uint, c_uint) {
        let mut word = [0u8; UINTSZ];
        let len = remaining.min(UINTSZ);
        word[..len].copy_from_slice(&bytes[..len]);
        let val = c_uint::from_ne_bytes(word);
        let have_nul = val.wrapping_sub(0x01010101) & !val & 0x80808080;
        (val, have_nul)
    }

    fn legacy_add_until_nul(acc: c_uint, val: c_uint, have_nul: c_uint) -> c_uint {
        if have_nul == 0 {
            acc.wrapping_add(val)
        } else if val & 0xff != 0 {
            if val & 0xff00 == 0 {
                acc.wrapping_add(val & 0xff)
            } else if val & 0xff0000 == 0 {
                acc.wrapping_add(val & 0xffff)
            } else {
                acc.wrapping_add(val)
            }
        } else {
            acc
        }
    }

    fn legacy_jhash_string(bytes: &[u8]) -> c_uint {
        let mut offset = 0usize;
        let mut remaining = bytes.len();
        let mut a = JHASH_INITVAL;
        let mut b = JHASH_INITVAL;
        let mut c = JHASH_INITVAL;

        'words: loop {
            for lane in 0..3 {
                let (val, have_nul) = legacy_string_word(&bytes[offset..], remaining);
                let acc = match lane {
                    0 => &mut a,
                    1 => &mut b,
                    _ => &mut c,
                };
                *acc = legacy_add_until_nul(*acc, val, have_nul);
                if have_nul != 0 {
                    break 'words;
                }
                offset += UINTSZ;
                assert!(remaining >= UINTSZ);
                remaining -= UINTSZ;
            }
            jhash_mix!(a, b, c);
        }

        jhash_final!(a, b, c);
        c.wrapping_add(offset as c_uint)
    }

    #[test]
    fn jhash_string_matches_legacy_word_loop() {
        for input in [
            "",
            "a",
            "abc",
            "abcd",
            "abcde",
            "abcdefgh",
            "abcdefghijkl",
            "abcdefghijklm",
            "target%pattern",
        ] {
            let actual = jhash_string(input.as_bytes());
            assert_eq!(actual, legacy_jhash_string(input.as_bytes()), "{input:?}");
        }
    }
}
