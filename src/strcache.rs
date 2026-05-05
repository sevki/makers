use libc::{printf, strcmp};

use crate::stdio::{FILE};
extern "C" {
    static mut stdout: *mut FILE;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn hash_init(
        ht: *mut hash_table,
        size: ::core::ffi::c_ulong,
        hash_1: hash_func_t,
        hash_2: hash_func_t,
        hash_cmp: hash_cmp_func_t,
    );
    fn hash_find_slot(
        ht: *mut hash_table,
        key: *const ::core::ffi::c_void,
    ) -> *mut *mut ::core::ffi::c_void;
    fn hash_insert_at(
        ht: *mut hash_table,
        item: *const ::core::ffi::c_void,
        slot: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn hash_print_stats(ht: *mut hash_table, out_FILE: *mut FILE);
    fn jhash_string(key: *const ::core::ffi::c_uchar) -> ::core::ffi::c_uint;
    static mut hash_deleted_item: *const ::core::ffi::c_void;
}
pub type size_t = usize;
pub type hash_table = crate::hash::hash_table;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
pub type sc_buflen_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strcache {
    pub next: *mut strcache,
    pub end: sc_buflen_t,
    pub bytesfree: sc_buflen_t,
    pub count: sc_buflen_t,
    pub buffer: [::core::ffi::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hugestring {
    pub next: *mut hugestring,
    pub buffer: [::core::ffi::c_char; 1],
}
pub const USHRT_MAX: ::core::ffi::c_int =
    __SHRT_MAX__ * 2 + 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 40] = unsafe {
    ::core::mem::transmute::<[u8; 40], [::core::ffi::c_char; 40]>(
        *b"void strcache_print_stats(const char *)\0",
    )
};
pub const CACHE_BUFFER_OFFSET: ::core::ffi::c_ulong = 14;
pub const BUFSIZE: usize = (8192 as usize)
    .wrapping_sub((2 as usize).wrapping_mul(::core::mem::size_of::<size_t>() as usize))
    .wrapping_sub(CACHE_BUFFER_OFFSET as usize);
static mut strcache: *mut strcache = ::core::ptr::null::<strcache>() as *mut strcache;
static mut fullcache: *mut strcache = ::core::ptr::null::<strcache>() as *mut strcache;
static mut total_buffers: ::core::ffi::c_ulong = 0;
static mut total_strings: ::core::ffi::c_ulong = 0;
static mut total_size: ::core::ffi::c_ulong = 0;
unsafe extern "C" fn new_cache(
    mut head: *mut *mut strcache,
    mut buflen: sc_buflen_t,
) -> *mut strcache {
    let mut new: *mut strcache =
        xmalloc((buflen as size_t).wrapping_add(CACHE_BUFFER_OFFSET as size_t)) as *mut strcache;
    (*new).end = 0 as sc_buflen_t;
    (*new).count = 0 as sc_buflen_t;
    (*new).bytesfree = buflen;
    (*new).next = *head;
    *head = new;
    total_buffers = total_buffers.wrapping_add(1);
    new
}
unsafe extern "C" fn copy_string(
    mut sp: *mut strcache,
    mut str: *const ::core::ffi::c_char,
    mut len: sc_buflen_t,
) -> *const ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = (&raw mut (*sp).buffer as *mut ::core::ffi::c_char)
        .offset((*sp).end as isize)
        as *mut ::core::ffi::c_char;
    memmove(
        res as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        len as size_t,
    );
    let fresh0 = len;
    len = len.wrapping_add(1);
    *res.offset(fresh0 as isize) = 0;
    (*sp).end = ((*sp).end as ::core::ffi::c_int + len as ::core::ffi::c_int) as sc_buflen_t;
    (*sp).bytesfree =
        ((*sp).bytesfree as ::core::ffi::c_int - len as ::core::ffi::c_int) as sc_buflen_t;
    (*sp).count = (*sp).count.wrapping_add(1);
    res
}
unsafe extern "C" fn add_string(
    mut str: *const ::core::ffi::c_char,
    mut len: sc_buflen_t,
) -> *const ::core::ffi::c_char {
    let mut res: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut sp: *mut strcache = ::core::ptr::null_mut::<strcache>();
    let mut spp: *mut *mut strcache = &raw mut strcache;
    let mut sz: sc_buflen_t = (len as ::core::ffi::c_int + 1) as sc_buflen_t;
    total_strings = total_strings.wrapping_add(1);
    total_size = total_size.wrapping_add(sz as ::core::ffi::c_ulong);
    if sz as usize > BUFSIZE {
        sp = new_cache(&raw mut fullcache, sz);
        return copy_string(sp, str, len);
    }
    while !(*spp).is_null() {
        if (**spp).bytesfree as ::core::ffi::c_int > sz as ::core::ffi::c_int {
            break;
        }
        spp = &raw mut (**spp).next;
    }
    sp = *spp;
    if sp.is_null() {
        sp = new_cache(&raw mut strcache, BUFSIZE as sc_buflen_t);
        spp = &raw mut strcache;
    }
    res = copy_string(sp, str, len);
    if total_strings > 20
        && ((*sp).bytesfree as ::core::ffi::c_ulong)
            < total_size
                .wrapping_div(total_strings)
                .wrapping_add(1)
    {
        *spp = (*sp).next;
        (*sp).next = fullcache;
        fullcache = sp;
    }
    res
}
static mut hugestrings: *mut hugestring = ::core::ptr::null::<hugestring>() as *mut hugestring;
unsafe extern "C" fn add_hugestring(
    mut str: *const ::core::ffi::c_char,
    mut len: size_t,
) -> *const ::core::ffi::c_char {
    let mut new: *mut hugestring =
        xmalloc((::core::mem::size_of::<hugestring>() as size_t).wrapping_add(len))
            as *mut hugestring;
    memcpy(
        &raw mut (*new).buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        len as size_t,
    );
    *(&raw mut (*new).buffer as *mut ::core::ffi::c_char).offset(len as isize) =
        0;
    (*new).next = hugestrings;
    hugestrings = new;
    &raw mut (*new).buffer as *mut ::core::ffi::c_char
}
#[no_mangle]
pub unsafe extern "C" fn str_hash_1(mut key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0;
    let mut _key_: *const ::core::ffi::c_uchar =
        key as *const ::core::ffi::c_char as *const ::core::ffi::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as ::core::ffi::c_ulong);
    _result_
}
#[no_mangle]
pub unsafe extern "C" fn str_hash_2(mut _key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0;
    _result_
}
unsafe extern "C" fn str_hash_cmp(
    mut x: *const ::core::ffi::c_void,
    mut y: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if x as *const ::core::ffi::c_char == y as *const ::core::ffi::c_char {
        0
    } else {
        strcmp(
            x as *const ::core::ffi::c_char,
            y as *const ::core::ffi::c_char,
        )
    }
}
static mut strings: hash_table = hash_table {
    ht_vec: ::core::ptr::null::<*mut ::core::ffi::c_void>() as *mut *mut ::core::ffi::c_void,
    ht_hash_1: None,
    ht_hash_2: None,
    ht_compare: None,
    ht_size: 0,
    ht_capacity: 0,
    ht_fill: 0,
    ht_empty_slots: 0,
    ht_collisions: 0,
    ht_lookups: 0,
    ht_rehashes: 0,
    ht_in_map: [0; 1],
    c2rust_padding: [0; 3],
};
static mut total_adds: ::core::ffi::c_ulong = 0;
unsafe extern "C" fn add_hash(
    mut str: *const ::core::ffi::c_char,
    mut len: size_t,
) -> *const ::core::ffi::c_char {
    let mut slot: *const *mut ::core::ffi::c_char = ::core::ptr::null::<*mut ::core::ffi::c_char>();
    let mut key: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if len > (USHRT_MAX - 1) as size_t {
        return add_hugestring(str, len);
    }
    slot = hash_find_slot(&raw mut strings, str as *const ::core::ffi::c_void)
        as *const *mut ::core::ffi::c_char;
    key = *slot;
    total_adds = total_adds.wrapping_add(1);
    if !(key.is_null()
        || key as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
    {
        return key;
    }
    key = add_string(str, len as sc_buflen_t);
    hash_insert_at(
        &raw mut strings,
        key as *const ::core::ffi::c_void,
        slot as *const ::core::ffi::c_void,
    );
    key
}
#[no_mangle]
pub unsafe extern "C" fn strcache_iscached(
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut sp: *mut strcache = ::core::ptr::null_mut::<strcache>();
    sp = strcache;
    while !sp.is_null() {
        if str >= &raw mut (*sp).buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_char
            && str
                < (&raw mut (*sp).buffer as *mut ::core::ffi::c_char)
                    .offset((*sp).end as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char
        {
            return 1;
        }
        sp = (*sp).next;
    }
    sp = fullcache;
    while !sp.is_null() {
        if str >= &raw mut (*sp).buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_char
            && str
                < (&raw mut (*sp).buffer as *mut ::core::ffi::c_char)
                    .offset((*sp).end as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char
        {
            return 1;
        }
        sp = (*sp).next;
    }
    let mut hp: *mut hugestring = ::core::ptr::null_mut::<hugestring>();
    hp = hugestrings;
    while !hp.is_null() {
        if str == &raw mut (*hp).buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_char {
            return 1;
        }
        hp = (*hp).next;
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn strcache_add(
    mut str: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    add_hash(str, strlen(str) as size_t)
}
#[no_mangle]
pub unsafe extern "C" fn strcache_add_len(
    mut str: *const ::core::ffi::c_char,
    mut len: size_t,
) -> *const ::core::ffi::c_char {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    if *str.offset(len as isize) as ::core::ffi::c_int != 0 {
        alloca_allocations.push(::std::vec::from_elem(
            0,
            len.wrapping_add(1) as usize,
        ));
        let mut key: *mut ::core::ffi::c_char =
            alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        memcpy(
            key as *mut ::core::ffi::c_void,
            str as *const ::core::ffi::c_void,
            len as size_t,
        );
        *key.offset(len as isize) = 0;
        str = key;
    }
    add_hash(str, len)
}
#[no_mangle]
pub unsafe fn strcache_init() {
    hash_init(
        &raw mut strings,
        8000 as ::core::ffi::c_ulong,
        Some(
            str_hash_1 as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            str_hash_2 as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            str_hash_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe fn strcache_print_stats(mut prefix: *const ::core::ffi::c_char) {
    let mut sp: *const strcache = ::core::ptr::null::<strcache>();
    let mut numbuffs: ::core::ffi::c_ulong = 0;
    let mut fullbuffs: ::core::ffi::c_ulong = 0;
    let mut totfree: ::core::ffi::c_ulong = 0;
    let mut maxfree: ::core::ffi::c_ulong = 0;
    let mut minfree: ::core::ffi::c_ulong = BUFSIZE as ::core::ffi::c_ulong;
    if strcache.is_null() {
        printf(
            b"\n%s No strcache buffers\n\0" as *const u8 as *const ::core::ffi::c_char,
            prefix,
        );
        return;
    }
    sp = (*strcache).next;
    while !sp.is_null() {
        let mut bf: sc_buflen_t = (*sp).bytesfree;
        totfree = totfree.wrapping_add(bf as ::core::ffi::c_ulong);
        maxfree = if bf as ::core::ffi::c_ulong > maxfree {
            bf as ::core::ffi::c_ulong
        } else {
            maxfree
        };
        minfree = if (bf as ::core::ffi::c_ulong) < minfree {
            bf as ::core::ffi::c_ulong
        } else {
            minfree
        };
        numbuffs = numbuffs.wrapping_add(1);
        sp = (*sp).next;
    }
    sp = fullcache;
    while !sp.is_null() {
        let mut bf_0: sc_buflen_t = (*sp).bytesfree;
        totfree = totfree.wrapping_add(bf_0 as ::core::ffi::c_ulong);
        maxfree = if bf_0 as ::core::ffi::c_ulong > maxfree {
            bf_0 as ::core::ffi::c_ulong
        } else {
            maxfree
        };
        minfree = if (bf_0 as ::core::ffi::c_ulong) < minfree {
            bf_0 as ::core::ffi::c_ulong
        } else {
            minfree
        };
        numbuffs = numbuffs.wrapping_add(1);
        fullbuffs = fullbuffs.wrapping_add(1);
        sp = (*sp).next;
    }
    '_c2rust_label: {
        if total_buffers == numbuffs.wrapping_add(1) {
        } else {
            __assert_fail(
                b"total_buffers == numbuffs + 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/strcache.c\0" as *const u8 as *const ::core::ffi::c_char,
                302,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    printf(
        b"\n%s strcache buffers: %lu (%lu) / strings = %lu / storage = %lu B / avg = %lu B\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        prefix,
        numbuffs.wrapping_add(1),
        fullbuffs,
        total_strings,
        total_size,
        total_size.wrapping_div(total_strings),
    );
    printf(
        b"%s current buf: size = %hu B / used = %hu B / count = %hu / avg = %u B\n\0" as *const u8
            as *const ::core::ffi::c_char,
        prefix,
        BUFSIZE as sc_buflen_t as ::core::ffi::c_int,
        (*strcache).end as ::core::ffi::c_int,
        (*strcache).count as ::core::ffi::c_int,
        ((*strcache).end as ::core::ffi::c_int / (*strcache).count as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
    if numbuffs != 0 {
        let mut sz: ::core::ffi::c_ulong =
            total_size.wrapping_sub((*strcache).end as ::core::ffi::c_ulong);
        let mut cnt: ::core::ffi::c_ulong =
            total_strings.wrapping_sub((*strcache).count as ::core::ffi::c_ulong);
        let mut avgfree: sc_buflen_t = totfree.wrapping_div(numbuffs) as sc_buflen_t;
        printf(
            b"%s other used: total = %lu B / count = %lu / avg = %lu B\n\0" as *const u8
                as *const ::core::ffi::c_char,
            prefix,
            sz,
            cnt,
            sz.wrapping_div(cnt),
        );
        printf(
            b"%s other free: total = %lu B / max = %lu B / min = %lu B / avg = %hu B\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            prefix,
            totfree,
            maxfree,
            minfree,
            avgfree as ::core::ffi::c_int,
        );
    }
    printf(
        b"\n%s strcache performance: lookups = %lu / hit rate = %lu%%\n\0" as *const u8
            as *const ::core::ffi::c_char,
        prefix,
        total_adds,
        (100.0f64 * total_adds.wrapping_sub(total_strings) as ::core::ffi::c_double
            / total_adds as ::core::ffi::c_double) as ::core::ffi::c_ulong,
    );
    fputs(
        b"# hash-table stats:\n# \0" as *const u8 as *const ::core::ffi::c_char,
        stdout,
    );
    hash_print_stats(&raw mut strings, stdout);
}
pub const __SHRT_MAX__: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
