use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
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
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub const MAKE_TROUBLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut hash_deleted_item: *const ::core::ffi::c_void = unsafe {
    &raw const hash_deleted_item as *mut *const ::core::ffi::c_void as *const ::core::ffi::c_void
};
#[no_mangle]
pub unsafe extern "C" fn hash_init(
    mut ht: *mut hash_table,
    mut size: ::core::ffi::c_ulong,
    mut hash_1: hash_func_t,
    mut hash_2: hash_func_t,
    mut hash_cmp: hash_cmp_func_t,
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
        .wrapping_sub((*ht).ht_size.wrapping_div(16 as ::core::ffi::c_ulong));
    (*ht).ht_fill = 0 as ::core::ffi::c_ulong;
    (*ht).ht_collisions = 0 as ::core::ffi::c_ulong;
    (*ht).ht_lookups = 0 as ::core::ffi::c_ulong;
    (*ht).ht_rehashes = 0 as ::core::ffi::c_uint;
    (*ht).set_ht_in_map(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*ht).ht_hash_1 = hash_1;
    (*ht).ht_hash_2 = hash_2;
    (*ht).ht_compare = hash_cmp;
}
#[no_mangle]
pub unsafe extern "C" fn hash_load(
    mut ht: *mut hash_table,
    mut item_table: *const ::core::ffi::c_void,
    mut cardinality: ::core::ffi::c_ulong,
    mut size: ::core::ffi::c_ulong,
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
    mut key: *const ::core::ffi::c_void,
) -> *mut *mut ::core::ffi::c_void {
    let mut slot: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut deleted_slot: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut hash_2: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut hash_1: ::core::ffi::c_uint = Some((*ht).ht_hash_1.expect("non-null function pointer"))
        .expect("non-null function pointer")(key)
        as ::core::ffi::c_uint;
    (*ht).ht_lookups = (*ht).ht_lookups.wrapping_add(1);
    loop {
        hash_1 = (hash_1 as ::core::ffi::c_ulong
            & (*ht).ht_size.wrapping_sub(1 as ::core::ffi::c_ulong))
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
                == 0 as ::core::ffi::c_int
            {
                return slot;
            }
            (*ht).ht_collisions = (*ht).ht_collisions.wrapping_add(1);
        }
        if hash_2 == 0 {
            hash_2 = (Some((*ht).ht_hash_2.expect("non-null function pointer"))
                .expect("non-null function pointer")(key)
                | 1 as ::core::ffi::c_ulong) as ::core::ffi::c_uint;
        }
        hash_1 = hash_1.wrapping_add(hash_2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn hash_find_item(
    mut ht: *mut hash_table,
    mut key: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut slot: *mut *mut ::core::ffi::c_void = hash_find_slot(ht, key);
    return if (*slot).is_null() || *slot == hash_deleted_item as *mut ::core::ffi::c_void {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    } else {
        *slot
    };
}
#[no_mangle]
pub unsafe extern "C" fn hash_insert(
    mut ht: *mut hash_table,
    mut item: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut slot: *mut *mut ::core::ffi::c_void = hash_find_slot(ht, item);
    let mut old_item: *const ::core::ffi::c_void = *slot;
    hash_insert_at(ht, item, slot as *const ::core::ffi::c_void);
    return (if old_item.is_null()
        || old_item as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void
    {
        ::core::ptr::null::<::core::ffi::c_void>()
    } else {
        old_item
    }) as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn hash_insert_at(
    mut ht: *mut hash_table,
    mut item: *const ::core::ffi::c_void,
    mut slot: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut old_item: *const ::core::ffi::c_void = *(slot as *mut *mut ::core::ffi::c_void);
    '_c2rust_label: {
        if (*ht).ht_in_map() == 0 {
        } else {
            __assert_fail(
                b"! ht->ht_in_map\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                144 as ::core::ffi::c_uint,
                b"void *hash_insert_at(struct hash_table *, const void *, const void *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if old_item.is_null()
        || old_item as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void
    {
        (*ht).ht_fill = (*ht).ht_fill.wrapping_add(1);
        if old_item.is_null() {
            (*ht).ht_empty_slots = (*ht).ht_empty_slots.wrapping_sub(1);
        }
        old_item = item;
    }
    let ref mut fresh1 = *(slot as *mut *const ::core::ffi::c_void);
    *fresh1 = item;
    if (*ht).ht_empty_slots < (*ht).ht_size.wrapping_sub((*ht).ht_capacity) {
        hash_rehash(ht);
        return hash_find_slot(ht, item) as *mut ::core::ffi::c_void;
    } else {
        return slot as *mut ::core::ffi::c_void;
    };
}
#[no_mangle]
pub unsafe extern "C" fn hash_delete(
    mut ht: *mut hash_table,
    mut item: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut slot: *mut *mut ::core::ffi::c_void = hash_find_slot(ht, item);
    return hash_delete_at(ht, slot as *const ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn hash_delete_at(
    mut ht: *mut hash_table,
    mut slot: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut item: *mut ::core::ffi::c_void = *(slot as *mut *mut ::core::ffi::c_void);
    if !(item.is_null() || item == hash_deleted_item as *mut ::core::ffi::c_void) {
        let ref mut fresh2 = *(slot as *mut *const ::core::ffi::c_void);
        *fresh2 = hash_deleted_item;
        (*ht).ht_fill = (*ht).ht_fill.wrapping_sub(1);
        return item;
    } else {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn hash_free_items(mut ht: *mut hash_table) {
    let mut vec: *mut *mut ::core::ffi::c_void = (*ht).ht_vec;
    let mut end: *mut *mut ::core::ffi::c_void =
        vec.offset((*ht).ht_size as isize) as *mut *mut ::core::ffi::c_void;
    '_c2rust_label: {
        if (*ht).ht_in_map() == 0 {
        } else {
            __assert_fail(
                b"! ht->ht_in_map\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                191 as ::core::ffi::c_uint,
                b"void hash_free_items(struct hash_table *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    while vec < end {
        let mut item: *mut ::core::ffi::c_void = *vec;
        if !(item.is_null() || item == hash_deleted_item as *mut ::core::ffi::c_void) {
            free(item);
        }
        *vec = ::core::ptr::null_mut::<::core::ffi::c_void>();
        vec = vec.offset(1);
    }
    (*ht).ht_fill = 0 as ::core::ffi::c_ulong;
    (*ht).ht_empty_slots = (*ht).ht_size;
}
#[no_mangle]
pub unsafe extern "C" fn hash_delete_items(mut ht: *mut hash_table) {
    let mut vec: *mut *mut ::core::ffi::c_void = (*ht).ht_vec;
    let mut end: *mut *mut ::core::ffi::c_void =
        vec.offset((*ht).ht_size as isize) as *mut *mut ::core::ffi::c_void;
    '_c2rust_label: {
        if (*ht).ht_in_map() == 0 {
        } else {
            __assert_fail(
                b"! ht->ht_in_map\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_uint,
                b"void hash_delete_items(struct hash_table *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    while vec < end {
        *vec = ::core::ptr::null_mut::<::core::ffi::c_void>();
        vec = vec.offset(1);
    }
    (*ht).ht_fill = 0 as ::core::ffi::c_ulong;
    (*ht).ht_collisions = 0 as ::core::ffi::c_ulong;
    (*ht).ht_lookups = 0 as ::core::ffi::c_ulong;
    (*ht).ht_rehashes = 0 as ::core::ffi::c_uint;
    (*ht).ht_empty_slots = (*ht).ht_size;
}
#[no_mangle]
pub unsafe extern "C" fn hash_free(mut ht: *mut hash_table, mut free_items: ::core::ffi::c_int) {
    '_c2rust_label: {
        if (*ht).ht_in_map() == 0 {
        } else {
            __assert_fail(
                b"! ht->ht_in_map\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                226 as ::core::ffi::c_uint,
                b"void hash_free(struct hash_table *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if free_items != 0 {
        hash_free_items(ht);
    } else {
        (*ht).ht_fill = 0 as ::core::ffi::c_ulong;
        (*ht).ht_empty_slots = (*ht).ht_size;
    }
    free((*ht).ht_vec as *mut ::core::ffi::c_void);
    (*ht).ht_vec = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    (*ht).ht_capacity = 0 as ::core::ffi::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn hash_map(mut ht: *mut hash_table, mut map: hash_map_func_t) {
    let mut slot: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut end: *mut *mut ::core::ffi::c_void =
        (*ht).ht_vec.offset((*ht).ht_size as isize) as *mut *mut ::core::ffi::c_void;
    (*ht).set_ht_in_map(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    slot = (*ht).ht_vec;
    while slot < end {
        if !((*slot).is_null() || *slot == hash_deleted_item as *mut ::core::ffi::c_void) {
            Some(map.expect("non-null function pointer")).expect("non-null function pointer")(
                *slot,
            );
        }
        slot = slot.offset(1);
    }
    (*ht).set_ht_in_map(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn hash_map_arg(
    mut ht: *mut hash_table,
    mut map: hash_map_arg_func_t,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut slot: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut end: *mut *mut ::core::ffi::c_void =
        (*ht).ht_vec.offset((*ht).ht_size as isize) as *mut *mut ::core::ffi::c_void;
    (*ht).set_ht_in_map(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    slot = (*ht).ht_vec;
    while slot < end {
        if !((*slot).is_null() || *slot == hash_deleted_item as *mut ::core::ffi::c_void) {
            Some(map.expect("non-null function pointer")).expect("non-null function pointer")(
                *slot, arg,
            );
        }
        slot = slot.offset(1);
    }
    (*ht).set_ht_in_map(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn hash_rehash(mut ht: *mut hash_table) {
    let mut old_ht_size: ::core::ffi::c_ulong = (*ht).ht_size;
    let mut old_vec: *mut *mut ::core::ffi::c_void = (*ht).ht_vec;
    let mut ovp: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if (*ht).ht_fill >= (*ht).ht_capacity {
        (*ht).ht_size = (*ht).ht_size.wrapping_mul(2 as ::core::ffi::c_ulong);
        (*ht).ht_capacity = (*ht)
            .ht_size
            .wrapping_sub((*ht).ht_size >> 4 as ::core::ffi::c_int);
    }
    (*ht).ht_rehashes = (*ht).ht_rehashes.wrapping_add(1);
    (*ht).ht_vec = xcalloc(
        (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
            .wrapping_mul((*ht).ht_size as size_t),
    ) as *mut *mut ::core::ffi::c_void;
    ovp = old_vec;
    while ovp < old_vec.offset(old_ht_size as isize) as *mut *mut ::core::ffi::c_void {
        if !((*ovp).is_null() || *ovp == hash_deleted_item as *mut ::core::ffi::c_void) {
            let mut slot: *mut *mut ::core::ffi::c_void = hash_find_slot(ht, *ovp);
            *slot = *ovp;
        }
        ovp = ovp.offset(1);
    }
    (*ht).ht_empty_slots = (*ht).ht_size.wrapping_sub((*ht).ht_fill);
    free(old_vec as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn hash_print_stats(mut ht: *mut hash_table, mut out_FILE: *mut FILE) {
    fprintf(
        out_FILE,
        b"Load=%lu/%lu=%.0f%%, \0" as *const u8 as *const ::core::ffi::c_char,
        (*ht).ht_fill,
        (*ht).ht_size,
        100.0f64 * (*ht).ht_fill as ::core::ffi::c_double / (*ht).ht_size as ::core::ffi::c_double,
    );
    fprintf(
        out_FILE,
        b"Rehash=%u, \0" as *const u8 as *const ::core::ffi::c_char,
        (*ht).ht_rehashes,
    );
    fprintf(
        out_FILE,
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
    mut ht: *mut hash_table,
    mut vector_0: *mut *mut ::core::ffi::c_void,
    mut compare: qsort_cmp_t,
) -> *mut *mut ::core::ffi::c_void {
    let mut vector: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut slot: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut end: *mut *mut ::core::ffi::c_void =
        (*ht).ht_vec.offset((*ht).ht_size as isize) as *mut *mut ::core::ffi::c_void;
    if vector_0.is_null() {
        vector_0 = xmalloc(
            (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                .wrapping_mul(((*ht).ht_fill as size_t).wrapping_add(1 as size_t)),
        ) as *mut *mut ::core::ffi::c_void;
    }
    vector = vector_0;
    slot = (*ht).ht_vec;
    while slot < end {
        if !((*slot).is_null() || *slot == hash_deleted_item as *mut ::core::ffi::c_void) {
            let fresh3 = vector;
            vector = vector.offset(1);
            *fresh3 = *slot;
        }
        slot = slot.offset(1);
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
    return vector_0;
}
#[no_mangle]
pub unsafe extern "C" fn round_up_2(mut n: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    n |= n >> 1 as ::core::ffi::c_int;
    n |= n >> 2 as ::core::ffi::c_int;
    n |= n >> 4 as ::core::ffi::c_int;
    n |= n >> 8 as ::core::ffi::c_int;
    n |= n >> 16 as ::core::ffi::c_int;
    n |= n >> 32 as ::core::ffi::c_int;
    return n.wrapping_add(1 as ::core::ffi::c_ulong);
}
pub const JHASH_INITVAL: ::core::ffi::c_uint = 0xdeadbeef as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn jhash(
    mut k: *const ::core::ffi::c_uchar,
    mut length: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut a: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut c: ::core::ffi::c_uint = 0;
    c = JHASH_INITVAL.wrapping_add(length as ::core::ffi::c_uint);
    b = c;
    a = b;
    while length > 12 as ::core::ffi::c_int {
        let mut val: ::core::ffi::c_uint = 0;
        memcpy(
            &raw mut val as *mut ::core::ffi::c_void,
            k as *const ::core::ffi::c_void,
            4 as size_t,
        );
        a = a.wrapping_add(val);
        let mut val_0: ::core::ffi::c_uint = 0;
        memcpy(
            &raw mut val_0 as *mut ::core::ffi::c_void,
            k.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            4 as size_t,
        );
        b = b.wrapping_add(val_0);
        let mut val_1: ::core::ffi::c_uint = 0;
        memcpy(
            &raw mut val_1 as *mut ::core::ffi::c_void,
            k.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            4 as size_t,
        );
        c = c.wrapping_add(val_1);
        a = a.wrapping_sub(c);
        a ^= c << 4 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a << 6 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b << 8 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
        b = b.wrapping_add(a);
        a = a.wrapping_sub(c);
        a ^= c << 16 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a << 19 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int;
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b << 4 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        b = b.wrapping_add(a);
        length -= 12 as ::core::ffi::c_int;
        k = k.offset(12 as ::core::ffi::c_int as isize);
    }
    if length == 0 {
        return c;
    }
    if length > 8 as ::core::ffi::c_int {
        let mut val_2: ::core::ffi::c_uint = 0;
        memcpy(
            &raw mut val_2 as *mut ::core::ffi::c_void,
            k as *const ::core::ffi::c_void,
            4 as size_t,
        );
        a = a.wrapping_add(val_2);
        length -= 4 as ::core::ffi::c_int;
        k = k.offset(4 as ::core::ffi::c_int as isize);
    }
    if length > 4 as ::core::ffi::c_int {
        let mut val_3: ::core::ffi::c_uint = 0;
        memcpy(
            &raw mut val_3 as *mut ::core::ffi::c_void,
            k as *const ::core::ffi::c_void,
            4 as size_t,
        );
        b = b.wrapping_add(val_3);
        length -= 4 as ::core::ffi::c_int;
        k = k.offset(4 as ::core::ffi::c_int as isize);
    }
    if length == 4 as ::core::ffi::c_int {
        c = c.wrapping_add(
            (*k.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                << 24 as ::core::ffi::c_int,
        );
    }
    if length >= 3 as ::core::ffi::c_int {
        c = c.wrapping_add(
            (*k.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                << 16 as ::core::ffi::c_int,
        );
    }
    if length >= 2 as ::core::ffi::c_int {
        c = c.wrapping_add(
            (*k.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_int,
        );
    }
    c = c.wrapping_add(*k.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    c ^= b;
    c = c.wrapping_sub(
        b << 14 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int,
    );
    a ^= c;
    a = a.wrapping_sub(
        c << 11 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int,
    );
    b ^= a;
    b = b.wrapping_sub(
        a << 25 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int,
    );
    c ^= b;
    c = c.wrapping_sub(
        b << 16 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int,
    );
    a ^= c;
    a = a.wrapping_sub(
        c << 4 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int,
    );
    b ^= a;
    b = b.wrapping_sub(
        a << 14 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int,
    );
    c ^= b;
    c = c.wrapping_sub(
        b << 24 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 24 as ::core::ffi::c_int,
    );
    return c;
}
pub const UINTSZ: usize = ::core::mem::size_of::<::core::ffi::c_uint>();
#[no_mangle]
pub unsafe extern "C" fn jhash_string(mut k: *const ::core::ffi::c_uchar) -> ::core::ffi::c_uint {
    let mut a: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut c: ::core::ffi::c_uint = 0;
    let mut have_nul: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut start: *const ::core::ffi::c_uchar = k;
    let mut klen: size_t = strlen(k as *const ::core::ffi::c_char) as size_t;
    c = JHASH_INITVAL;
    b = c;
    a = b;
    loop {
        let mut val: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut pn: size_t = klen;
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
            if val & 0xff00 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                a = a.wrapping_add(val & 0xff as ::core::ffi::c_uint);
            } else if val & 0xff0000 as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
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
        '_c2rust_label: {
            if klen >= ::core::mem::size_of::<::core::ffi::c_uint>() as usize {
            } else {
                __assert_fail(
                    b"klen >= UINTSZ\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                    506 as ::core::ffi::c_uint,
                    b"unsigned int jhash_string(const unsigned char *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        klen = (klen as ::core::ffi::c_ulong).wrapping_sub(UINTSZ as ::core::ffi::c_ulong) as size_t
            as size_t;
        let mut val_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut pn_0: size_t = klen;
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
            if val_0 & 0xff00 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                b = b.wrapping_add(val_0 & 0xff as ::core::ffi::c_uint);
            } else if val_0 & 0xff0000 as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
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
        '_c2rust_label_0: {
            if klen >= ::core::mem::size_of::<::core::ffi::c_uint>() as usize {
            } else {
                __assert_fail(
                    b"klen >= UINTSZ\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                    513 as ::core::ffi::c_uint,
                    b"unsigned int jhash_string(const unsigned char *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        klen = (klen as ::core::ffi::c_ulong).wrapping_sub(UINTSZ as ::core::ffi::c_ulong) as size_t
            as size_t;
        let mut val_1: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut pn_1: size_t = klen;
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
            if val_1 & 0xff00 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                c = c.wrapping_add(val_1 & 0xff as ::core::ffi::c_uint);
            } else if val_1 & 0xff0000 as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
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
        '_c2rust_label_1: {
            if klen >= ::core::mem::size_of::<::core::ffi::c_uint>() as usize {
            } else {
                __assert_fail(
                    b"klen >= UINTSZ\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/hash.c\0" as *const u8 as *const ::core::ffi::c_char,
                    520 as ::core::ffi::c_uint,
                    b"unsigned int jhash_string(const unsigned char *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        klen = (klen as ::core::ffi::c_ulong).wrapping_sub(UINTSZ as ::core::ffi::c_ulong) as size_t
            as size_t;
        a = a.wrapping_sub(c);
        a ^= c << 4 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a << 6 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b << 8 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
        b = b.wrapping_add(a);
        a = a.wrapping_sub(c);
        a ^= c << 16 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a << 19 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int;
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b << 4 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        b = b.wrapping_add(a);
    }
    c ^= b;
    c = c.wrapping_sub(
        b << 14 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int,
    );
    a ^= c;
    a = a.wrapping_sub(
        c << 11 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int,
    );
    b ^= a;
    b = b.wrapping_sub(
        a << 25 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int,
    );
    c ^= b;
    c = c.wrapping_sub(
        b << 16 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int,
    );
    a ^= c;
    a = a.wrapping_sub(
        c << 4 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int,
    );
    b ^= a;
    b = b.wrapping_sub(
        a << 14 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int,
    );
    c ^= b;
    c = c.wrapping_sub(
        b << 24 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 24 as ::core::ffi::c_int,
    );
    return c.wrapping_add(k.offset_from(start) as ::core::ffi::c_long as ::core::ffi::c_uint);
}
