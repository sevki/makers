//! The open-addressed hash table used for make's file, variable, and
//! directory tables, plus the Jenkins lookup3 hash.
//!
//! Port of `hash.c`. The table stores raw `void *` items and C function
//! pointers because every consumer (file.rs, variable.rs, dir.rs, ...) is
//! still keyed on interned C strings; the layout of `hash_table` is shared
//! through `#[repr(C)]`.

use ::core::ffi::{c_char, c_double, c_int, c_uchar, c_uint, c_ulong, c_void};
use ::core::ptr::null_mut;

use libc::{exit, free, memcpy, qsort, strlen};

use crate::ffi_types::size_t;
use crate::misc::{xcalloc, xmalloc};
use crate::stdio::FILE;

extern "C" {
    static mut stderr: *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
}

pub type __compar_fn_t = Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>;
pub type hash_func_t = Option<unsafe extern "C" fn(*const c_void) -> c_ulong>;
pub type hash_cmp_func_t = Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>;
pub type hash_map_func_t = Option<unsafe extern "C" fn(*const c_void)>;
pub type hash_map_arg_func_t = Option<unsafe extern "C" fn(*const c_void, *mut c_void)>;
pub type qsort_cmp_t = Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>;

/// An open-addressed (double-hashed) table of `void *` items. Deleted
/// slots hold [`hash_deleted_item`]; empty slots hold null.
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    /// Total slots (a power of two).
    pub ht_size: c_ulong,
    /// Fill threshold that triggers a rehash (15/16 of the size).
    pub ht_capacity: c_ulong,
    /// Items in the table.
    pub ht_fill: c_ulong,
    /// Slots that are neither full nor deleted.
    pub ht_empty_slots: c_ulong,
    pub ht_collisions: c_ulong,
    pub ht_lookups: c_ulong,
    pub ht_rehashes: c_uint,
    #[bitfield(name = "ht_in_map", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub ht_in_map: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}

pub const MAKE_TROUBLE: c_int = 1;

/// Sentinel stored in slots whose item was deleted (its own address, so it
/// can never equal a real item).
#[no_mangle]
pub static mut hash_deleted_item: *const c_void =
    &raw const hash_deleted_item as *mut *const c_void as *const c_void;

/// Is `item` a real entry (not empty, not the deleted sentinel)?
unsafe fn is_real_item(item: *const c_void) -> bool {
    !item.is_null() && item != hash_deleted_item
}

/// Initialize `ht` with at least `size` slots (rounded up to a power of
/// two) and the given hash/compare callbacks.
///
/// # Safety
/// `ht` must point to writable storage; the callbacks must be non-null and
/// valid for the items later stored.
#[no_mangle]
pub unsafe extern "C" fn hash_init(
    ht: *mut hash_table,
    size: c_ulong,
    hash_1: hash_func_t,
    hash_2: hash_func_t,
    hash_cmp: hash_cmp_func_t,
) {
    (*ht).ht_size = round_up_2(size);
    (*ht).ht_empty_slots = (*ht).ht_size;
    (*ht).ht_vec = xcalloc(::core::mem::size_of::<*mut c_void>() * (*ht).ht_size as size_t)
        as *mut *mut c_void;
    if (*ht).ht_vec.is_null() {
        fprintf(
            stderr,
            c"can't allocate %lu bytes for hash table: memory exhausted".as_ptr(),
            (*ht).ht_size * ::core::mem::size_of::<*mut c_void>() as c_ulong,
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
#[no_mangle]
pub unsafe extern "C" fn hash_load(
    ht: *mut hash_table,
    item_table: *const c_void,
    cardinality: c_ulong,
    size: c_ulong,
) {
    let mut items: *const c_char = item_table as *const c_char;
    for _ in 0..cardinality {
        hash_insert(ht, items as *const c_void);
        items = items.offset(size as isize);
    }
}

/// Return the slot for `key`: the item's slot if present, otherwise the
/// slot where it would be inserted (reusing the first deleted slot seen).
///
/// # Safety
/// `ht` must be initialized and `key` valid for its callbacks.
#[no_mangle]
pub unsafe extern "C" fn hash_find_slot(
    ht: *mut hash_table,
    key: *const c_void,
) -> *mut *mut c_void {
    let mut deleted_slot: *mut *mut c_void = null_mut();
    let mut hash_2: c_uint = 0;
    let mut hash_1 = (*ht).ht_hash_1.expect("hash table without ht_hash_1")(key) as c_uint;

    (*ht).ht_lookups = (*ht).ht_lookups.wrapping_add(1);
    loop {
        // ht_size is a power of two, so this is "hash_1 % size".
        hash_1 = (hash_1 as c_ulong & ((*ht).ht_size - 1)) as c_uint;
        let slot = (*ht)
            .ht_vec
            .add(hash_1 as usize)
            .as_mut()
            .expect("hash table without a slot vector");

        if (*slot).is_null() {
            return if !deleted_slot.is_null() {
                deleted_slot
            } else {
                &raw mut *slot
            };
        }
        if ::core::ptr::eq(*slot, hash_deleted_item as *mut c_void) {
            if deleted_slot.is_null() {
                deleted_slot = &raw mut *slot;
            }
        } else {
            if ::core::ptr::eq(key, *slot) {
                return &raw mut *slot;
            }
            if (*ht).ht_compare.expect("hash table without ht_compare")(key, *slot) == 0 {
                return &raw mut *slot;
            }
            (*ht).ht_collisions = (*ht).ht_collisions.wrapping_add(1);
        }

        // Probe again with the secondary hash (forced odd, so it is
        // coprime with the power-of-two size).
        if hash_2 == 0 {
            hash_2 = ((*ht).ht_hash_2.expect("hash table without ht_hash_2")(key) | 1) as c_uint;
        }
        hash_1 = hash_1.wrapping_add(hash_2);
    }
}

/// Return the item matching `key`, or null.
///
/// # Safety
/// `ht` must be initialized and `key` valid for its callbacks.
#[no_mangle]
pub unsafe extern "C" fn hash_find_item(ht: *mut hash_table, key: *const c_void) -> *mut c_void {
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
#[no_mangle]
pub unsafe extern "C" fn hash_insert(ht: *mut hash_table, item: *const c_void) -> *mut c_void {
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
#[no_mangle]
pub unsafe extern "C" fn hash_insert_at(
    ht: *mut hash_table,
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
#[no_mangle]
pub unsafe extern "C" fn hash_delete(ht: *mut hash_table, item: *const c_void) -> *mut c_void {
    let slot = hash_find_slot(ht, item);
    hash_delete_at(ht, slot as *const c_void)
}

/// Delete whatever occupies `slot` (from [`hash_find_slot`]). Returns the
/// deleted item or null.
///
/// # Safety
/// `slot` must come from `hash_find_slot` on this table with no
/// intervening modification.
#[no_mangle]
pub unsafe extern "C" fn hash_delete_at(ht: *mut hash_table, slot: *const c_void) -> *mut c_void {
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
#[no_mangle]
pub unsafe extern "C" fn hash_free_items(ht: *mut hash_table) {
    assert!((*ht).ht_in_map() == 0, "hash table modified during mapping");
    for i in 0..(*ht).ht_size as usize {
        let vec = (*ht).ht_vec.add(i);
        if is_real_item(*vec) {
            free(*vec);
        }
        *vec = null_mut();
    }
    (*ht).ht_fill = 0;
    (*ht).ht_empty_slots = (*ht).ht_size;
}

/// Clear the table without freeing the items, resetting the statistics.
///
/// # Safety
/// `ht` must be initialized.
#[no_mangle]
pub unsafe extern "C" fn hash_delete_items(ht: *mut hash_table) {
    assert!((*ht).ht_in_map() == 0, "hash table modified during mapping");
    for i in 0..(*ht).ht_size as usize {
        *(*ht).ht_vec.add(i) = null_mut();
    }
    (*ht).ht_fill = 0;
    (*ht).ht_collisions = 0;
    (*ht).ht_lookups = 0;
    (*ht).ht_rehashes = 0;
    (*ht).ht_empty_slots = (*ht).ht_size;
}

/// Free the table's vector (and, when `free_items`, the items too).
///
/// # Safety
/// `ht` must be initialized; with `free_items` every stored item must be
/// an owned allocation.
#[no_mangle]
pub unsafe extern "C" fn hash_free(ht: *mut hash_table, free_items: c_int) {
    assert!((*ht).ht_in_map() == 0, "hash table modified during mapping");
    if free_items != 0 {
        hash_free_items(ht);
    } else {
        (*ht).ht_fill = 0;
        (*ht).ht_empty_slots = (*ht).ht_size;
    }
    free((*ht).ht_vec as *mut c_void);
    (*ht).ht_vec = null_mut();
    (*ht).ht_capacity = 0;
}

/// Call `map` on every item. The table must not be modified while mapping.
///
/// # Safety
/// `ht` must be initialized and `map` non-null.
#[no_mangle]
pub unsafe extern "C" fn hash_map(ht: *mut hash_table, map: hash_map_func_t) {
    let map = map.expect("hash_map without callback");
    (*ht).set_ht_in_map(1);
    for i in 0..(*ht).ht_size as usize {
        let slot = (*ht).ht_vec.add(i);
        if is_real_item(*slot) {
            map(*slot);
        }
    }
    (*ht).set_ht_in_map(0);
}

/// Call `map(item, arg)` on every item. The table must not be modified
/// while mapping.
///
/// # Safety
/// `ht` must be initialized and `map` non-null.
#[no_mangle]
pub unsafe extern "C" fn hash_map_arg(
    ht: *mut hash_table,
    map: hash_map_arg_func_t,
    arg: *mut c_void,
) {
    let map = map.expect("hash_map_arg without callback");
    (*ht).set_ht_in_map(1);
    for i in 0..(*ht).ht_size as usize {
        let slot = (*ht).ht_vec.add(i);
        if is_real_item(*slot) {
            map(*slot, arg);
        }
    }
    (*ht).set_ht_in_map(0);
}

/// Re-bucket every item, doubling the size when the table is at capacity
/// (also used as-is to flush deleted slots).
///
/// # Safety
/// `ht` must be initialized.
#[no_mangle]
pub unsafe extern "C" fn hash_rehash(ht: *mut hash_table) {
    let old_ht_size = (*ht).ht_size;
    let old_vec = (*ht).ht_vec;

    if (*ht).ht_fill >= (*ht).ht_capacity {
        (*ht).ht_size *= 2;
        (*ht).ht_capacity = (*ht).ht_size - ((*ht).ht_size >> 4);
    }
    (*ht).ht_rehashes = (*ht).ht_rehashes.wrapping_add(1);
    (*ht).ht_vec = xcalloc(::core::mem::size_of::<*mut c_void>() * (*ht).ht_size as size_t)
        as *mut *mut c_void;

    for i in 0..old_ht_size as usize {
        let ovp = old_vec.add(i);
        if is_real_item(*ovp) {
            let slot = hash_find_slot(ht, *ovp);
            *slot = *ovp;
        }
    }
    (*ht).ht_empty_slots = (*ht).ht_size - (*ht).ht_fill;
    free(old_vec as *mut c_void);
}

/// Print load/rehash/collision statistics to `out_file` (used by
/// `make -p`).
///
/// # Safety
/// `ht` must be initialized and `out_file` an open stream.
#[no_mangle]
pub unsafe extern "C" fn hash_print_stats(ht: *mut hash_table, out_file: *mut FILE) {
    fprintf(
        out_file,
        c"Load=%lu/%lu=%.0f%%, ".as_ptr(),
        (*ht).ht_fill,
        (*ht).ht_size,
        100.0f64 * (*ht).ht_fill as c_double / (*ht).ht_size as c_double,
    );
    fprintf(out_file, c"Rehash=%u, ".as_ptr(), (*ht).ht_rehashes);
    fprintf(
        out_file,
        c"Collisions=%lu/%lu=%.0f%%".as_ptr(),
        (*ht).ht_collisions,
        (*ht).ht_lookups,
        if (*ht).ht_lookups != 0 {
            100.0f64 * (*ht).ht_collisions as c_double / (*ht).ht_lookups as c_double
        } else {
            0.0f64
        },
    );
}

/// Dump the items into `vector_0` (allocated when null) as a
/// null-terminated array, optionally qsorted with `compare`. Returns the
/// vector.
///
/// # Safety
/// `vector_0` must be null or have room for `ht_fill + 1` pointers.
#[no_mangle]
pub unsafe extern "C" fn hash_dump(
    ht: *mut hash_table,
    mut vector_0: *mut *mut c_void,
    compare: qsort_cmp_t,
) -> *mut *mut c_void {
    if vector_0.is_null() {
        vector_0 = xmalloc(::core::mem::size_of::<*mut c_void>() * ((*ht).ht_fill as size_t + 1))
            as *mut *mut c_void;
    }

    let mut vector = vector_0;
    for i in 0..(*ht).ht_size as usize {
        let slot = (*ht).ht_vec.add(i);
        if is_real_item(*slot) {
            *vector = *slot;
            vector = vector.add(1);
        }
    }
    *vector = null_mut();

    if compare.is_some() {
        qsort(
            vector_0 as *mut c_void,
            (*ht).ht_fill as size_t,
            ::core::mem::size_of::<*mut c_void>(),
            compare,
        );
    }
    vector_0
}

/// Round up to the next power of two by bit-smearing. Note this is NOT
/// `next_power_of_two`: exact powers of two are doubled (16 -> 32), which
/// the table relies on for its capacity margin.
///
/// # Safety
/// Always safe; unsafe only for C-API signature compatibility.
#[no_mangle]
pub unsafe extern "C" fn round_up_2(mut n: c_ulong) -> c_ulong {
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

/// Read a little-endian word from `k` (an unaligned load).
unsafe fn load_word(k: *const c_uchar) -> c_uint {
    let mut val: c_uint = 0;
    memcpy(&raw mut val as *mut c_void, k as *const c_void, UINTSZ);
    val
}

/// Hash `length` bytes at `k`.
///
/// # Safety
/// `k` must be valid for reads of `length` bytes.
#[no_mangle]
pub unsafe extern "C" fn jhash(mut k: *const c_uchar, mut length: c_int) -> c_uint {
    let mut c = JHASH_INITVAL.wrapping_add(length as c_uint);
    let mut b = c;
    let mut a = b;

    while length > 12 {
        a = a.wrapping_add(load_word(k));
        b = b.wrapping_add(load_word(k.add(4)));
        c = c.wrapping_add(load_word(k.add(8)));
        jhash_mix!(a, b, c);
        length -= 12;
        k = k.add(12);
    }

    if length == 0 {
        return c;
    }
    if length > 8 {
        a = a.wrapping_add(load_word(k));
        length -= 4;
        k = k.add(4);
    }
    if length > 4 {
        b = b.wrapping_add(load_word(k));
        length -= 4;
        k = k.add(4);
    }
    if length == 4 {
        c = c.wrapping_add((*k.add(3) as c_uint) << 24);
    }
    if length >= 3 {
        c = c.wrapping_add((*k.add(2) as c_uint) << 16);
    }
    if length >= 2 {
        c = c.wrapping_add((*k.add(1) as c_uint) << 8);
    }
    c = c.wrapping_add(*k as c_uint);

    jhash_final!(a, b, c);
    c
}

/// Read the next word of a NUL-terminated string without reading past
/// `klen` remaining bytes, and report which bytes (if any) are NUL via the
/// SWAR has-zero trick.
unsafe fn load_string_word(k: *const c_uchar, klen: size_t) -> (c_uint, c_uint) {
    let mut val: c_uint = 0;
    memcpy(
        &raw mut val as *mut c_void,
        k as *const c_void,
        if klen >= UINTSZ { UINTSZ } else { klen },
    );
    let have_nul = val.wrapping_sub(0x01010101) & !val & 0x80808080;
    (val, have_nul)
}

/// Add `val`'s bytes that precede its first NUL into `acc`.
fn add_until_nul(acc: c_uint, val: c_uint, have_nul: c_uint) -> c_uint {
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

/// Hash the NUL-terminated string `k` (lookup3 over words, mixing in the
/// consumed length at the end).
///
/// # Safety
/// `k` must be a valid NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn jhash_string(mut k: *const c_uchar) -> c_uint {
    let start: *const c_uchar = k;
    let mut klen: size_t = strlen(k as *const c_char);

    let mut a = JHASH_INITVAL;
    let mut b = JHASH_INITVAL;
    let mut c = JHASH_INITVAL;

    // Consume the string a word at a time into the three lanes, mixing
    // after every third word, until a word containing the NUL is seen.
    'words: loop {
        for lane in 0..3 {
            let (val, have_nul) = load_string_word(k, klen);
            let acc = match lane {
                0 => &mut a,
                1 => &mut b,
                _ => &mut c,
            };
            *acc = add_until_nul(*acc, val, have_nul);
            if have_nul != 0 {
                break 'words;
            }
            k = k.add(UINTSZ);
            assert!(klen >= UINTSZ, "jhash_string ran past the terminator");
            klen -= UINTSZ;
        }
        jhash_mix!(a, b, c);
    }

    jhash_final!(a, b, c);
    c.wrapping_add(k.offset_from(start) as c_uint)
}
