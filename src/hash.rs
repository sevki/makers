use libc::{exit, free};
use ::c2rust_bitfields;
use crate::stdio::{FILE};
pub use crate::ffi_types::size_t;
extern "C" {
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xcalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type hash_func_t =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong>;
pub type hash_cmp_func_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type hash_map_func_t = Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> ()>;
pub type hash_map_arg_func_t =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut ::core::ffi::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: ::core::ffi::c_ulong,
    pub ht_capacity: ::core::ffi::c_ulong,
    pub ht_fill: ::core::ffi::c_ulong,
    pub ht_empty_slots: ::core::ffi::c_ulong,
    pub ht_collisions: ::core::ffi::c_ulong,
    pub ht_lookups: ::core::ffi::c_ulong,
    pub ht_rehashes: ::core::ffi::c_uint,
    #[bitfield(name = "ht_in_map", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub ht_in_map: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type qsort_cmp_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub const MAKE_TROUBLE: ::core::ffi::c_int = 1;
#[no_mangle]
pub static mut hash_deleted_item: *const ::core::ffi::c_void = unsafe {
    &raw const hash_deleted_item as *mut *const ::core::ffi::c_void as *const ::core::ffi::c_void
};
#[no_mangle]
pub unsafe extern "C" fn hash_init(
    mut ht: *mut hash_table,
    size: ::core::ffi::c_ulong,
    hash_1: hash_func_t,
    hash_2: hash_func_t,
    hash_cmp: hash_cmp_func_t,
) {
    (*ht).ht_size = round_up_2(size);
    (*ht).ht_empty_slots = (*ht).ht_size;
    (*ht).ht_vec = xcalloc(
        (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
            .wrapping_mul((*ht).ht_size as size_t),
    ) as *mut *mut ::core::ffi::c_void;
    if (*ht).ht_vec.is_null() {
        fprintf(
            stderr,
            b"can't allocate %lu bytes for hash table: memory exhausted\0" as *const u8
                as *const ::core::ffi::c_char,
            (*ht).ht_size.wrapping_mul(
                ::core::mem::size_of::<*mut ::core::ffi::c_void>() as ::core::ffi::c_ulong
            ),
        );
        exit(MAKE_TROUBLE);
    }
    (*ht).ht_capacity = (*ht)
        .ht_size
        .wrapping_sub((*ht).ht_size.wrapping_div(16));
    (*ht).ht_fill = 0;
    (*ht).ht_collisions = 0;
    (*ht).ht_lookups = 0;
    (*ht).ht_rehashes = 0;
    (*ht).set_ht_in_map(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*ht).ht_hash_1 = hash_1;
    (*ht).ht_hash_2 = hash_2;
    (*ht).ht_compare = hash_cmp;
}
#[no_mangle]
pub unsafe extern "C" fn hash_load(
    ht: *mut hash_table,
    item_table: *const ::core::ffi::c_void,
    mut cardinality: ::core::ffi::c_ulong,
    size: ::core::ffi::c_ulong,
) {
    let mut items: *const ::core::ffi::c_char = item_table as *const ::core::ffi::c_char;
    loop {
        let fresh0 = cardinality;
        cardinality = cardinality.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        hash_insert(ht, items as *const ::core::ffi::c_void);
        items = items.offset(size as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn hash_find_slot(
    mut ht: *mut hash_table,
    key: *const ::core::ffi::c_void,
) -> *mut *mut ::core::ffi::c_void {
    let mut slot: *mut *mut ::core::ffi::c_void;
    let mut deleted_slot: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut hash_2: ::core::ffi::c_uint = 0;
    let mut hash_1: ::core::ffi::c_uint = Some((*ht).ht_hash_1.expect("non-null function pointer"))
        .expect("non-null function pointer")(key)
        as ::core::ffi::c_uint;
    (*ht).ht_lookups = (*ht).ht_lookups.wrapping_add(1);
    loop {
        hash_1 = (hash_1 as ::core::ffi::c_ulong
            & (*ht).ht_size.wrapping_sub(1))
            as ::core::ffi::c_uint;
        slot = (*ht).ht_vec.offset(hash_1 as isize) as *mut *mut ::core::ffi::c_void;
        if (*slot).is_null() {
            return if !deleted_slot.is_null() {
                deleted_slot
            } else {
                slot
            };
        }
        if *slot == hash_deleted_item as *mut ::core::ffi::c_void {
            if deleted_slot.is_null() {
                deleted_slot = slot;
            }
        } else {
            if key == *slot as *const ::core::ffi::c_void {
                return slot;
            }
            if Some((*ht).ht_compare.expect("non-null function pointer"))
                .expect("non-null function pointer")(key, *slot)
                == 0
            {
                return slot;
            }
            (*ht).ht_collisions = (*ht).ht_collisions.wrapping_add(1);
        }
        if hash_2 == 0 {
            hash_2 = (Some((*ht).ht_hash_2.expect("non-null function pointer"))
                .expect("non-null function pointer")(key)
                | 1) as ::core::ffi::c_uint;
        }
        hash_1 = hash_1.wrapping_add(hash_2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn hash_find_item(
    ht: *mut hash_table,
    key: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let slot: *mut *mut ::core::ffi::c_void = hash_find_slot(ht, key);
    if (*slot).is_null() || *slot == hash_deleted_item as *mut ::core::ffi::c_void {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    } else {
        *slot
    }
}
#[no_mangle]
pub unsafe extern "C" fn hash_insert(
    ht: *mut hash_table,
    item: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let slot: *mut *mut ::core::ffi::c_void = hash_find_slot(ht, item);
    let old_item: *const ::core::ffi::c_void = *slot;
    hash_insert_at(ht, item, slot as *const ::core::ffi::c_void);
    (if old_item.is_null()
        || old_item as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void
    {
        ::core::ptr::null::<::core::ffi::c_void>()
    } else {
        old_item
    }) as *mut ::core::ffi::c_void
}
#[no_mangle]
pub unsafe extern "C" fn hash_insert_at(
    mut ht: *mut hash_table,
    item: *const ::core::ffi::c_void,
    slot: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let old_item: *const ::core::ffi::c_void = *(slot as *mut *mut ::core::ffi::c_void);
    if (*ht).ht_in_map() == 0 {
        } else {
            __assert_fail(
                b"! ht->ht_in_map\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                144,
                b"void *hash_insert_at(struct hash_table *, const void *, const void *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        };
    if old_item.is_null()
        || old_item as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void
    {
        (*ht).ht_fill = (*ht).ht_fill.wrapping_add(1);
        if old_item.is_null() {
            (*ht).ht_empty_slots = (*ht).ht_empty_slots.wrapping_sub(1);
        }
    }
    let fresh1 = &mut (*(slot as *mut *const ::core::ffi::c_void));
    *fresh1 = item;
    if (*ht).ht_empty_slots < (*ht).ht_size.wrapping_sub((*ht).ht_capacity) {
        hash_rehash(ht);
        hash_find_slot(ht, item) as *mut ::core::ffi::c_void
    } else {
        slot as *mut ::core::ffi::c_void
    }
}
#[no_mangle]
pub unsafe extern "C" fn hash_delete(
    ht: *mut hash_table,
    item: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let slot: *mut *mut ::core::ffi::c_void = hash_find_slot(ht, item);
    hash_delete_at(ht, slot as *const ::core::ffi::c_void)
}
#[no_mangle]
pub unsafe extern "C" fn hash_delete_at(
    mut ht: *mut hash_table,
    slot: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let item: *mut ::core::ffi::c_void = *(slot as *mut *mut ::core::ffi::c_void);
    if !(item.is_null() || item == hash_deleted_item as *mut ::core::ffi::c_void) {
        let fresh2 = &mut (*(slot as *mut *const ::core::ffi::c_void));
        *fresh2 = hash_deleted_item;
        (*ht).ht_fill = (*ht).ht_fill.wrapping_sub(1);
        item
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    }
}
#[no_mangle]
pub unsafe extern "C" fn hash_free_items(mut ht: *mut hash_table) {
    let mut vec: *mut *mut ::core::ffi::c_void = (*ht).ht_vec;
    let end: *mut *mut ::core::ffi::c_void =
        vec.offset((*ht).ht_size as isize) as *mut *mut ::core::ffi::c_void;
    if (*ht).ht_in_map() == 0 {
        } else {
            __assert_fail(
                b"! ht->ht_in_map\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                191,
                b"void hash_free_items(struct hash_table *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
    while vec < end {
        let item: *mut ::core::ffi::c_void = *vec;
        if !(item.is_null() || item == hash_deleted_item as *mut ::core::ffi::c_void) {
            free(item);
        }
        *vec = ::core::ptr::null_mut::<::core::ffi::c_void>();
        vec = vec.offset(1 as ::core::ffi::c_int as isize);
    }
    (*ht).ht_fill = 0;
    (*ht).ht_empty_slots = (*ht).ht_size;
}
#[no_mangle]
pub unsafe extern "C" fn hash_delete_items(mut ht: *mut hash_table) {
    let mut vec: *mut *mut ::core::ffi::c_void = (*ht).ht_vec;
    let end: *mut *mut ::core::ffi::c_void =
        vec.offset((*ht).ht_size as isize) as *mut *mut ::core::ffi::c_void;
    if (*ht).ht_in_map() == 0 {
        } else {
            __assert_fail(
                b"! ht->ht_in_map\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                211,
                b"void hash_delete_items(struct hash_table *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
    while vec < end {
        *vec = ::core::ptr::null_mut::<::core::ffi::c_void>();
        vec = vec.offset(1 as ::core::ffi::c_int as isize);
    }
    (*ht).ht_fill = 0;
    (*ht).ht_collisions = 0;
    (*ht).ht_lookups = 0;
    (*ht).ht_rehashes = 0;
    (*ht).ht_empty_slots = (*ht).ht_size;
}
#[no_mangle]
pub unsafe extern "C" fn hash_free(mut ht: *mut hash_table, free_items: ::core::ffi::c_int) {
    if (*ht).ht_in_map() == 0 {
        } else {
            __assert_fail(
                b"! ht->ht_in_map\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                226,
                b"void hash_free(struct hash_table *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
    if free_items != 0 {
        hash_free_items(ht);
    } else {
        (*ht).ht_fill = 0;
        (*ht).ht_empty_slots = (*ht).ht_size;
    }
    free((*ht).ht_vec as *mut ::core::ffi::c_void);
    (*ht).ht_vec = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    (*ht).ht_capacity = 0;
}
#[no_mangle]
pub unsafe extern "C" fn hash_map(ht: *mut hash_table, map: hash_map_func_t) {
    let mut slot: *mut *mut ::core::ffi::c_void;
    let end: *mut *mut ::core::ffi::c_void =
        (*ht).ht_vec.offset((*ht).ht_size as isize) as *mut *mut ::core::ffi::c_void;
    (*ht).set_ht_in_map(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    slot = (*ht).ht_vec;
    while slot < end {
        if !((*slot).is_null() || *slot == hash_deleted_item as *mut ::core::ffi::c_void) {
            Some(map.expect("non-null function pointer")).expect("non-null function pointer")(
                *slot,
            );
        }
        slot = slot.offset(1 as ::core::ffi::c_int as isize);
    }
    (*ht).set_ht_in_map(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn hash_map_arg(
    ht: *mut hash_table,
    map: hash_map_arg_func_t,
    arg: *mut ::core::ffi::c_void,
) {
    let mut slot: *mut *mut ::core::ffi::c_void;
    let end: *mut *mut ::core::ffi::c_void =
        (*ht).ht_vec.offset((*ht).ht_size as isize) as *mut *mut ::core::ffi::c_void;
    (*ht).set_ht_in_map(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    slot = (*ht).ht_vec;
    while slot < end {
        if !((*slot).is_null() || *slot == hash_deleted_item as *mut ::core::ffi::c_void) {
            Some(map.expect("non-null function pointer")).expect("non-null function pointer")(
                *slot, arg,
            );
        }
        slot = slot.offset(1 as ::core::ffi::c_int as isize);
    }
    (*ht).set_ht_in_map(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn hash_rehash(mut ht: *mut hash_table) {
    let old_ht_size: ::core::ffi::c_ulong = (*ht).ht_size;
    let old_vec: *mut *mut ::core::ffi::c_void = (*ht).ht_vec;
    let mut ovp: *mut *mut ::core::ffi::c_void;
    if (*ht).ht_fill >= (*ht).ht_capacity {
        (*ht).ht_size = (*ht).ht_size.wrapping_mul(2);
        (*ht).ht_capacity = (*ht)
            .ht_size
            .wrapping_sub((*ht).ht_size >> 4);
    }
    (*ht).ht_rehashes = (*ht).ht_rehashes.wrapping_add(1);
    (*ht).ht_vec = xcalloc(
        (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
            .wrapping_mul((*ht).ht_size as size_t),
    ) as *mut *mut ::core::ffi::c_void;
    ovp = old_vec;
    while ovp < old_vec.offset(old_ht_size as isize) as *mut *mut ::core::ffi::c_void {
        if !((*ovp).is_null() || *ovp == hash_deleted_item as *mut ::core::ffi::c_void) {
            let slot: *mut *mut ::core::ffi::c_void = hash_find_slot(ht, *ovp);
            *slot = *ovp;
        }
        ovp = ovp.offset(1 as ::core::ffi::c_int as isize);
    }
    (*ht).ht_empty_slots = (*ht).ht_size.wrapping_sub((*ht).ht_fill);
    free(old_vec as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn hash_print_stats(ht: *mut hash_table, out_file: *mut FILE) {
    fprintf(
        out_file,
        b"Load=%lu/%lu=%.0f%%, \0" as *const u8 as *const ::core::ffi::c_char,
        (*ht).ht_fill,
        (*ht).ht_size,
        100.0f64 * (*ht).ht_fill as ::core::ffi::c_double / (*ht).ht_size as ::core::ffi::c_double,
    );
    fprintf(
        out_file,
        b"Rehash=%u, \0" as *const u8 as *const ::core::ffi::c_char,
        (*ht).ht_rehashes,
    );
    fprintf(
        out_file,
        b"Collisions=%lu/%lu=%.0f%%\0" as *const u8 as *const ::core::ffi::c_char,
        (*ht).ht_collisions,
        (*ht).ht_lookups,
        if (*ht).ht_lookups != 0 {
            100.0f64 * (*ht).ht_collisions as ::core::ffi::c_double
                / (*ht).ht_lookups as ::core::ffi::c_double
        } else {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn hash_dump(
    ht: *mut hash_table,
    mut vector_0: *mut *mut ::core::ffi::c_void,
    compare: qsort_cmp_t,
) -> *mut *mut ::core::ffi::c_void {
    let mut vector: *mut *mut ::core::ffi::c_void;
    let mut slot: *mut *mut ::core::ffi::c_void;
    let end: *mut *mut ::core::ffi::c_void =
        (*ht).ht_vec.offset((*ht).ht_size as isize) as *mut *mut ::core::ffi::c_void;
    if vector_0.is_null() {
        vector_0 = xmalloc(
            (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                .wrapping_mul(((*ht).ht_fill as size_t).wrapping_add(1)),
        ) as *mut *mut ::core::ffi::c_void;
    }
    vector = vector_0;
    slot = (*ht).ht_vec;
    while slot < end {
        if !((*slot).is_null() || *slot == hash_deleted_item as *mut ::core::ffi::c_void) {
            let fresh3 = vector;
            vector = vector.offset(1 as ::core::ffi::c_int as isize);
            *fresh3 = *slot;
        }
        slot = slot.offset(1 as ::core::ffi::c_int as isize);
    }
    *vector = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if compare.is_some() {
        qsort(
            vector_0 as *mut ::core::ffi::c_void,
            (*ht).ht_fill as size_t,
            ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
            compare as __compar_fn_t,
        );
    }
    vector_0
}
#[no_mangle]
pub unsafe extern "C" fn round_up_2(mut n: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n.wrapping_add(1)
}
pub const JHASH_INITVAL: ::core::ffi::c_uint = 0xdeadbeef as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn jhash(
    mut k: *const ::core::ffi::c_uchar,
    mut length: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut a: ::core::ffi::c_uint;
    let mut b: ::core::ffi::c_uint;
    let mut c: ::core::ffi::c_uint;
    c = JHASH_INITVAL.wrapping_add(length as ::core::ffi::c_uint);
    b = c;
    a = b;
    while length > 12 {
        let mut val: ::core::ffi::c_uint = 0;
        memcpy(
            &raw mut val as *mut ::core::ffi::c_void,
            k as *const ::core::ffi::c_void,
            4,
        );
        a = a.wrapping_add(val);
        let mut val_0: ::core::ffi::c_uint = 0;
        memcpy(
            &raw mut val_0 as *mut ::core::ffi::c_void,
            k.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            4,
        );
        b = b.wrapping_add(val_0);
        let mut val_1: ::core::ffi::c_uint = 0;
        memcpy(
            &raw mut val_1 as *mut ::core::ffi::c_void,
            k.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            4,
        );
        c = c.wrapping_add(val_1);
        a = a.wrapping_sub(c);
        a ^= c << 4 | c >> (32 - 4);
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a << 6 | a >> (32 - 6);
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b << 8 | b >> (32 - 8);
        b = b.wrapping_add(a);
        a = a.wrapping_sub(c);
        a ^= c << 16
            | c >> (32 - 16);
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a << 19
            | a >> (32 - 19);
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b << 4 | b >> (32 - 4);
        b = b.wrapping_add(a);
        length -= 12;
        k = k.offset(12 as ::core::ffi::c_int as isize);
    }
    if length == 0 {
        return c;
    }
    if length > 8 {
        let mut val_2: ::core::ffi::c_uint = 0;
        memcpy(
            &raw mut val_2 as *mut ::core::ffi::c_void,
            k as *const ::core::ffi::c_void,
            4,
        );
        a = a.wrapping_add(val_2);
        length -= 4;
        k = k.offset(4 as ::core::ffi::c_int as isize);
    }
    if length > 4 {
        let mut val_3: ::core::ffi::c_uint = 0;
        memcpy(
            &raw mut val_3 as *mut ::core::ffi::c_void,
            k as *const ::core::ffi::c_void,
            4,
        );
        b = b.wrapping_add(val_3);
        length -= 4;
        k = k.offset(4 as ::core::ffi::c_int as isize);
    }
    if length == 4 {
        c = c.wrapping_add(
            (*k.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                << 24,
        );
    }
    if length >= 3 {
        c = c.wrapping_add(
            (*k.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                << 16,
        );
    }
    if length >= 2 {
        c = c.wrapping_add((*k.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint) << 8,
        );
    }
    c = c.wrapping_add(*k.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    c ^= b;
    c = c.wrapping_sub(
        b << 14 | b >> (32 - 14),
    );
    a ^= c;
    a = a.wrapping_sub(
        c << 11 | c >> (32 - 11),
    );
    b ^= a;
    b = b.wrapping_sub(
        a << 25 | a >> (32 - 25),
    );
    c ^= b;
    c = c.wrapping_sub(
        b << 16 | b >> (32 - 16),
    );
    a ^= c;
    a = a.wrapping_sub(
        c << 4 | c >> (32 - 4),
    );
    b ^= a;
    b = b.wrapping_sub(
        a << 14 | a >> (32 - 14),
    );
    c ^= b;
    c = c.wrapping_sub(
        b << 24 | b >> (32 - 24),
    );
    c
}
pub const UINTSZ: usize = ::core::mem::size_of::<::core::ffi::c_uint>();
#[no_mangle]
pub unsafe extern "C" fn jhash_string(mut k: *const ::core::ffi::c_uchar) -> ::core::ffi::c_uint {
    let mut a: ::core::ffi::c_uint;
    let mut b: ::core::ffi::c_uint;
    let mut c: ::core::ffi::c_uint;
    let mut have_nul: ::core::ffi::c_uint;
    let start: *const ::core::ffi::c_uchar = k;
    let mut klen: size_t = strlen(k as *const ::core::ffi::c_char) as size_t;
    c = JHASH_INITVAL;
    b = c;
    a = b;
    loop {
        let mut val: ::core::ffi::c_uint = 0;
        let pn: size_t = klen;
        if pn >= UINTSZ {
            memcpy(
                &raw mut val as *mut ::core::ffi::c_void,
                k as *const ::core::ffi::c_void,
                UINTSZ,
            );
        } else {
            memcpy(
                &raw mut val as *mut ::core::ffi::c_void,
                k as *const ::core::ffi::c_void,
                pn as size_t,
            );
        }
        have_nul = val.wrapping_sub(0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint)
            & !val
            & 0x80808080 as ::core::ffi::c_uint;
        if have_nul == 0 {
            a = a.wrapping_add(val);
        } else if val & 0xff as ::core::ffi::c_uint != 0 {
            if val & 0xff00 as ::core::ffi::c_uint == 0 {
                a = a.wrapping_add(val & 0xff as ::core::ffi::c_uint);
            } else if val & 0xff0000 as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
            {
                a = a.wrapping_add(val & 0xffff as ::core::ffi::c_uint);
            } else {
                a = a.wrapping_add(val);
            }
        }
        if have_nul != 0 {
            break;
        }
        k = k.offset(UINTSZ as isize);
        if klen >= ::core::mem::size_of::<::core::ffi::c_uint>() as usize {
            } else {
                __assert_fail(
                    b"klen >= UINTSZ\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                    506,
                    b"unsigned int jhash_string(const unsigned char *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
        klen = (klen as ::core::ffi::c_ulong).wrapping_sub(UINTSZ as ::core::ffi::c_ulong) as size_t
            as size_t;
        let mut val_0: ::core::ffi::c_uint = 0;
        let pn_0: size_t = klen;
        if pn_0 >= UINTSZ {
            memcpy(
                &raw mut val_0 as *mut ::core::ffi::c_void,
                k as *const ::core::ffi::c_void,
                UINTSZ,
            );
        } else {
            memcpy(
                &raw mut val_0 as *mut ::core::ffi::c_void,
                k as *const ::core::ffi::c_void,
                pn_0 as size_t,
            );
        }
        have_nul = val_0.wrapping_sub(0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint)
            & !val_0
            & 0x80808080 as ::core::ffi::c_uint;
        if have_nul == 0 {
            b = b.wrapping_add(val_0);
        } else if val_0 & 0xff as ::core::ffi::c_uint != 0 {
            if val_0 & 0xff00 as ::core::ffi::c_uint == 0 {
                b = b.wrapping_add(val_0 & 0xff as ::core::ffi::c_uint);
            } else if val_0 & 0xff0000 as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
            {
                b = b.wrapping_add(val_0 & 0xffff as ::core::ffi::c_uint);
            } else {
                b = b.wrapping_add(val_0);
            }
        }
        if have_nul != 0 {
            break;
        }
        k = k.offset(UINTSZ as isize);
        if klen >= ::core::mem::size_of::<::core::ffi::c_uint>() as usize {
            } else {
                __assert_fail(
                    b"klen >= UINTSZ\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                    513,
                    b"unsigned int jhash_string(const unsigned char *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
        klen = (klen as ::core::ffi::c_ulong).wrapping_sub(UINTSZ as ::core::ffi::c_ulong) as size_t
            as size_t;
        let mut val_1: ::core::ffi::c_uint = 0;
        let pn_1: size_t = klen;
        if pn_1 >= UINTSZ {
            memcpy(
                &raw mut val_1 as *mut ::core::ffi::c_void,
                k as *const ::core::ffi::c_void,
                UINTSZ,
            );
        } else {
            memcpy(
                &raw mut val_1 as *mut ::core::ffi::c_void,
                k as *const ::core::ffi::c_void,
                pn_1 as size_t,
            );
        }
        have_nul = val_1.wrapping_sub(0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint)
            & !val_1
            & 0x80808080 as ::core::ffi::c_uint;
        if have_nul == 0 {
            c = c.wrapping_add(val_1);
        } else if val_1 & 0xff as ::core::ffi::c_uint != 0 {
            if val_1 & 0xff00 as ::core::ffi::c_uint == 0 {
                c = c.wrapping_add(val_1 & 0xff as ::core::ffi::c_uint);
            } else if val_1 & 0xff0000 as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
            {
                c = c.wrapping_add(val_1 & 0xffff as ::core::ffi::c_uint);
            } else {
                c = c.wrapping_add(val_1);
            }
        }
        if have_nul != 0 {
            break;
        }
        k = k.offset(UINTSZ as isize);
        if klen >= ::core::mem::size_of::<::core::ffi::c_uint>() as usize {
            } else {
                __assert_fail(
                    b"klen >= UINTSZ\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                    520,
                    b"unsigned int jhash_string(const unsigned char *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
        klen = (klen as ::core::ffi::c_ulong).wrapping_sub(UINTSZ as ::core::ffi::c_ulong) as size_t
            as size_t;
        a = a.wrapping_sub(c);
        a ^= c << 4 | c >> (32 - 4);
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a << 6 | a >> (32 - 6);
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b << 8 | b >> (32 - 8);
        b = b.wrapping_add(a);
        a = a.wrapping_sub(c);
        a ^= c << 16
            | c >> (32 - 16);
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a << 19
            | a >> (32 - 19);
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b << 4 | b >> (32 - 4);
        b = b.wrapping_add(a);
    }
    c ^= b;
    c = c.wrapping_sub(
        b << 14 | b >> (32 - 14),
    );
    a ^= c;
    a = a.wrapping_sub(
        c << 11 | c >> (32 - 11),
    );
    b ^= a;
    b = b.wrapping_sub(
        a << 25 | a >> (32 - 25),
    );
    c ^= b;
    c = c.wrapping_sub(
        b << 16 | b >> (32 - 16),
    );
    a ^= c;
    a = a.wrapping_sub(
        c << 4 | c >> (32 - 4),
    );
    b ^= a;
    b = b.wrapping_sub(
        a << 14 | a >> (32 - 14),
    );
    c ^= b;
    c = c.wrapping_sub(
        b << 24 | b >> (32 - 24),
    );
    c.wrapping_add(k.offset_from(start) as ::core::ffi::c_long as ::core::ffi::c_uint)
}
