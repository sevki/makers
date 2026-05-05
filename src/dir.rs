use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn fatal(flocp: *const floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xcalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xrealloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn ar_name(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn ar_member_date(_: *const ::core::ffi::c_char) -> time_t;
    fn strcache_add_len(str: *const ::core::ffi::c_char, len: size_t)
        -> *const ::core::ffi::c_char;
    static mut command_count: ::core::ffi::c_ulong;
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    static mut db_level: ::core::ffi::c_int;
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
    fn hash_find_item(
        ht: *mut hash_table,
        key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn hash_insert(
        ht: *mut hash_table,
        item: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn hash_insert_at(
        ht: *mut hash_table,
        item: *const ::core::ffi::c_void,
        slot: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn hash_free(ht: *mut hash_table, free_items: ::core::ffi::c_int);
    fn jhash_string(key: *const ::core::ffi::c_uchar) -> ::core::ffi::c_uint;
    static mut hash_deleted_item: *const ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type __size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut ::core::ffi::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: ::core::ffi::c_int,
    pub gl_closedir: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut dirent>,
    pub gl_opendir:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void>,
    pub gl_lstat:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int>,
    pub gl_stat:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const ::core::ffi::c_char,
    pub lineno: ::core::ffi::c_ulong,
    pub offset: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory {
    pub name: *const ::core::ffi::c_char,
    pub counter: ::core::ffi::c_ulong,
    pub contents: *mut directory_contents,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory_contents {
    pub dev: dev_t,
    pub ino: ino_t,
    pub dirfiles: hash_table,
    pub counter: ::core::ffi::c_ulong,
    pub dirstream: *mut DIR,
}
pub type DIR = __dirstream;
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
pub type hash_cmp_func_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type hash_func_t =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirfile {
    pub name: *const ::core::ffi::c_char,
    pub length: size_t,
    pub impossible: ::core::ffi::c_short,
    pub type_0: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirstream {
    pub contents: *mut directory_contents,
    pub dirfile_slot: *mut *mut dirfile,
}
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAX_OPEN_DIRECTORIES: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut open_directories: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const DIRECTORY_BUCKETS: ::core::ffi::c_int = 199 as ::core::ffi::c_int;
unsafe extern "C" fn clear_directory_contents(
    mut dc: *mut directory_contents,
) -> *mut directory_contents {
    (*dc).counter = 0 as ::core::ffi::c_ulong;
    if !(*dc).dirstream.is_null() {
        open_directories = open_directories.wrapping_sub(1);
        closedir((*dc).dirstream);
        (*dc).dirstream = ::core::ptr::null_mut::<DIR>();
    }
    if !(*dc).dirfiles.ht_vec.is_null() {
        hash_free(&raw mut (*dc).dirfiles, 1 as ::core::ffi::c_int);
    }
    return ::core::ptr::null_mut::<directory_contents>();
}
unsafe extern "C" fn directory_contents_hash_1(
    mut key_0: *const ::core::ffi::c_void,
) -> ::core::ffi::c_ulong {
    let mut key: *const directory_contents = key_0 as *const directory_contents;
    let mut hash: ::core::ffi::c_ulong = 0;
    hash = (((*key).dev as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int
        ^ (*key).ino as ::core::ffi::c_uint) as ::core::ffi::c_ulong;
    return hash;
}
unsafe extern "C" fn directory_contents_hash_2(
    mut key_0: *const ::core::ffi::c_void,
) -> ::core::ffi::c_ulong {
    let mut key: *const directory_contents = key_0 as *const directory_contents;
    let mut hash: ::core::ffi::c_ulong = 0;
    hash = (((*key).dev as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int
        ^ !(*key).ino as ::core::ffi::c_uint) as ::core::ffi::c_ulong;
    return hash;
}
unsafe extern "C" fn directory_contents_hash_cmp(
    mut xv: *const ::core::ffi::c_void,
    mut yv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut x: *const directory_contents = xv as *const directory_contents;
    let mut y: *const directory_contents = yv as *const directory_contents;
    let mut result: ::core::ffi::c_int = 0;
    result = if (*x).ino < (*y).ino {
        -(1 as ::core::ffi::c_int)
    } else if (*x).ino == (*y).ino {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    if result != 0 {
        return result;
    }
    return if (*x).dev < (*y).dev {
        -(1 as ::core::ffi::c_int)
    } else if (*x).dev == (*y).dev {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
}
static mut directory_contents: hash_table = hash_table {
    ht_vec: ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
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
#[no_mangle]
pub unsafe extern "C" fn directory_hash_1(mut key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut _key_: *const ::core::ffi::c_uchar =
        (*(key as *const directory)).name as *const ::core::ffi::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as ::core::ffi::c_ulong);
    return _result_;
}
#[no_mangle]
pub unsafe extern "C" fn directory_hash_2(mut key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    return _result_;
}
unsafe extern "C" fn directory_hash_cmp(
    mut x: *const ::core::ffi::c_void,
    mut y: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return if (*(x as *const directory)).name == (*(y as *const directory)).name {
        0 as ::core::ffi::c_int
    } else {
        strcmp(
            (*(x as *const directory)).name,
            (*(y as *const directory)).name,
        )
    };
}
static mut directories: hash_table = hash_table {
    ht_vec: ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
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
#[no_mangle]
pub unsafe extern "C" fn dirfile_hash_1(mut key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut _key_: *const ::core::ffi::c_uchar =
        (*(key as *const dirfile)).name as *const ::core::ffi::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as ::core::ffi::c_ulong);
    return _result_;
}
#[no_mangle]
pub unsafe extern "C" fn dirfile_hash_2(mut key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    return _result_;
}
unsafe extern "C" fn dirfile_hash_cmp(
    mut xv: *const ::core::ffi::c_void,
    mut yv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut x: *const dirfile = xv as *const dirfile;
    let mut y: *const dirfile = yv as *const dirfile;
    let mut result: ::core::ffi::c_int =
        (*x).length.wrapping_sub((*y).length) as ::core::ffi::c_int;
    if result != 0 {
        return result;
    }
    return if (*x).name == (*y).name {
        0 as ::core::ffi::c_int
    } else {
        strcmp((*x).name, (*y).name)
    };
}
pub const DIRFILE_BUCKETS: ::core::ffi::c_int = 107 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn find_directory(mut name: *const ::core::ffi::c_char) -> *mut directory {
    let mut dir: *mut directory = ::core::ptr::null_mut::<directory>();
    let mut dir_slot: *mut *mut directory = ::core::ptr::null_mut::<*mut directory>();
    let mut dir_key: directory = directory {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        counter: 0,
        contents: ::core::ptr::null_mut::<directory_contents>(),
    };
    let mut dc: *mut directory_contents = ::core::ptr::null_mut::<directory_contents>();
    let mut dc_slot: *mut *mut directory_contents =
        ::core::ptr::null_mut::<*mut directory_contents>();
    let mut dc_key: directory_contents = directory_contents {
        dev: 0,
        ino: 0,
        dirfiles: hash_table {
            ht_vec: ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
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
        },
        counter: 0,
        dirstream: ::core::ptr::null_mut::<DIR>(),
    };
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut r: ::core::ffi::c_int = 0;
    dir_key.name = name;
    dir_slot = hash_find_slot(
        &raw mut directories,
        &raw mut dir_key as *const ::core::ffi::c_void,
    ) as *mut *mut directory;
    dir = *dir_slot;
    if !(dir.is_null()
        || dir as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
    {
        let mut ctr: ::core::ffi::c_ulong = if !(*dir).contents.is_null() {
            (*(*dir).contents).counter
        } else {
            (*dir).counter
        };
        if ctr == command_count {
            return dir;
        }
        if 0x2 as ::core::ffi::c_int & db_level != 0 {
            printf(
                b"Directory %s cache invalidated (count %lu != command %lu)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                name,
                ctr,
                command_count,
            );
            fflush(stdout);
        }
        if !(*dir).contents.is_null() {
            clear_directory_contents((*dir).contents);
        }
    } else {
        let mut len: size_t = strlen(name) as size_t;
        dir = xmalloc(::core::mem::size_of::<directory>() as size_t) as *mut directory;
        (*dir).name = strcache_add_len(name, len);
        hash_insert_at(
            &raw mut directories,
            dir as *const ::core::ffi::c_void,
            dir_slot as *const ::core::ffi::c_void,
        );
    }
    (*dir).contents = ::core::ptr::null_mut::<directory_contents>();
    (*dir).counter = command_count;
    loop {
        r = stat(name, &raw mut st);
        if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if r < 0 as ::core::ffi::c_int {
        return dir;
    }
    memset(
        &raw mut dc_key as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<directory_contents>() as size_t,
    );
    dc_key.dev = st.st_dev as dev_t;
    dc_key.ino = st.st_ino as ino_t;
    dc_slot = hash_find_slot(
        &raw mut directory_contents,
        &raw mut dc_key as *const ::core::ffi::c_void,
    ) as *mut *mut directory_contents;
    dc = *dc_slot;
    if dc.is_null()
        || dc as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void
    {
        dc = xcalloc(::core::mem::size_of::<directory_contents>() as size_t)
            as *mut directory_contents;
        *dc = dc_key;
        hash_insert_at(
            &raw mut directory_contents,
            dc as *const ::core::ffi::c_void,
            dc_slot as *const ::core::ffi::c_void,
        );
    }
    (*dir).contents = dc;
    if (*dc).counter != command_count {
        if (*dc).counter != 0 {
            clear_directory_contents(dc);
        }
        (*dc).counter = command_count;
        loop {
            *__errno_location() = 0 as ::core::ffi::c_int;
            (*dc).dirstream = opendir(name);
            if !((*dc).dirstream.is_null() && *__errno_location() == EINTR) {
                break;
            }
        }
        if (*dc).dirstream.is_null() {
            (*dc).dirfiles.ht_vec = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
        } else {
            hash_init(
                &raw mut (*dc).dirfiles,
                DIRFILE_BUCKETS as ::core::ffi::c_ulong,
                Some(
                    dirfile_hash_1
                        as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
                ),
                Some(
                    dirfile_hash_2
                        as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
                ),
                Some(
                    dirfile_hash_cmp
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
            open_directories = open_directories.wrapping_add(1);
            if open_directories == MAX_OPEN_DIRECTORIES as ::core::ffi::c_uint {
                dir_contents_file_exists_p(dir, ::core::ptr::null::<::core::ffi::c_char>());
            }
        }
    }
    return dir;
}
unsafe extern "C" fn dir_contents_file_exists_p(
    mut dir: *mut directory,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut df: *mut dirfile = ::core::ptr::null_mut::<dirfile>();
    let mut d: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut dc: *mut directory_contents = (*dir).contents;
    if dc.is_null() || (*dc).dirfiles.ht_vec.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !filename.is_null() {
        let mut dirfile_key: dirfile = dirfile {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            length: 0,
            impossible: 0,
            type_0: 0,
        };
        if *filename as ::core::ffi::c_int == '\0' as i32 {
            return 1 as ::core::ffi::c_int;
        }
        dirfile_key.name = filename;
        dirfile_key.length = strlen(filename) as size_t;
        df = hash_find_item(
            &raw mut (*dc).dirfiles,
            &raw mut dirfile_key as *const ::core::ffi::c_void,
        ) as *mut dirfile;
        if !df.is_null() {
            return ((*df).impossible == 0) as ::core::ffi::c_int;
        }
    }
    if (*dc).dirstream.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        let mut len: size_t = 0;
        let mut dirfile_key_0: dirfile = dirfile {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            length: 0,
            impossible: 0,
            type_0: 0,
        };
        let mut dirfile_slot: *mut *mut dirfile = ::core::ptr::null_mut::<*mut dirfile>();
        loop {
            *__errno_location() = 0 as ::core::ffi::c_int;
            d = readdir((*dc).dirstream);
            if !(d.is_null() && *__errno_location() == EINTR) {
                break;
            }
        }
        if d.is_null() {
            if *__errno_location() != 0 {
                fatal(
                    ::core::ptr::null_mut::<floc>(),
                    (strlen((*dir).name) as size_t)
                        .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
                    b"readdir %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*dir).name,
                    strerror(*__errno_location()),
                );
            }
            break;
        } else {
            if !((*d).d_ino != 0 as __ino_t) {
                continue;
            }
            len = strlen(&raw mut (*d).d_name as *mut ::core::ffi::c_char) as size_t;
            dirfile_key_0.name = &raw mut (*d).d_name as *mut ::core::ffi::c_char;
            dirfile_key_0.length = len;
            dirfile_slot = hash_find_slot(
                &raw mut (*dc).dirfiles,
                &raw mut dirfile_key_0 as *const ::core::ffi::c_void,
            ) as *mut *mut dirfile;
            df = xmalloc(::core::mem::size_of::<dirfile>() as size_t) as *mut dirfile;
            (*df).name = strcache_add_len(&raw mut (*d).d_name as *mut ::core::ffi::c_char, len);
            (*df).type_0 = (*d).d_type;
            (*df).length = len;
            (*df).impossible = 0 as ::core::ffi::c_short;
            hash_insert_at(
                &raw mut (*dc).dirfiles,
                df as *const ::core::ffi::c_void,
                dirfile_slot as *const ::core::ffi::c_void,
            );
            if !filename.is_null()
                && (*(&raw mut (*d).d_name as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                    == *filename as ::core::ffi::c_int
                    && (*(&raw mut (*d).d_name as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                        == '\0' as i32
                        || strcmp(
                            (&raw mut (*d).d_name as *mut ::core::ffi::c_char)
                                .offset(1 as ::core::ffi::c_int as isize),
                            filename.offset(1 as ::core::ffi::c_int as isize),
                        ) == 0))
            {
                return 1 as ::core::ffi::c_int;
            }
        }
    }
    if d.is_null() {
        open_directories = open_directories.wrapping_sub(1);
        closedir((*dc).dirstream);
        (*dc).dirstream = ::core::ptr::null_mut::<DIR>();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dir_file_exists_p(
    mut dirname: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return dir_contents_file_exists_p(find_directory(dirname), filename);
}
#[no_mangle]
pub unsafe extern "C" fn file_exists_p(mut name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut dirend: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dirname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut slash: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if ar_name(name) != 0 {
        return (ar_member_date(name) != -(1 as ::core::ffi::c_int) as time_t)
            as ::core::ffi::c_int;
    }
    dirend = strrchr(name, '/' as i32);
    if dirend.is_null() {
        return dir_file_exists_p(b".\0" as *const u8 as *const ::core::ffi::c_char, name);
    }
    slash = dirend;
    if dirend == name {
        dirname = b"/\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (dirend.offset_from(name) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as ::core::ffi::c_ulong as usize,
        ));
        p = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        memcpy(
            p as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            dirend.offset_from(name) as ::core::ffi::c_long as size_t,
        );
        *p.offset(dirend.offset_from(name) as ::core::ffi::c_long as isize) =
            '\0' as i32 as ::core::ffi::c_char;
        dirname = p;
    }
    slash = slash.offset(1);
    return dir_file_exists_p(dirname, slash);
}
#[no_mangle]
pub unsafe extern "C" fn file_impossible(mut filename: *const ::core::ffi::c_char) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut dirend: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = filename;
    let mut dir: *mut directory = ::core::ptr::null_mut::<directory>();
    let mut new: *mut dirfile = ::core::ptr::null_mut::<dirfile>();
    dirend = strrchr(p, '/' as i32);
    if dirend.is_null() {
        dir = find_directory(b".\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        let mut dirname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut slash: *const ::core::ffi::c_char = dirend;
        if dirend == p {
            dirname = b"/\0" as *const u8 as *const ::core::ffi::c_char;
        } else {
            let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            alloca_allocations.push(::std::vec::from_elem(
                0,
                (dirend.offset_from(p) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                    as ::core::ffi::c_ulong as usize,
            ));
            cp = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                cp as *mut ::core::ffi::c_void,
                p as *const ::core::ffi::c_void,
                dirend.offset_from(p) as ::core::ffi::c_long as size_t,
            );
            *cp.offset(dirend.offset_from(p) as ::core::ffi::c_long as isize) =
                '\0' as i32 as ::core::ffi::c_char;
            dirname = cp;
        }
        dir = find_directory(dirname);
        p = slash.offset(1 as ::core::ffi::c_int as isize);
        filename = p;
    }
    if (*dir).contents.is_null() {
        (*dir).contents = xcalloc(::core::mem::size_of::<directory_contents>() as size_t)
            as *mut directory_contents;
    }
    if (*(*dir).contents).dirfiles.ht_vec.is_null() {
        hash_init(
            &raw mut (*(*dir).contents).dirfiles,
            DIRFILE_BUCKETS as ::core::ffi::c_ulong,
            Some(
                dirfile_hash_1
                    as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
            ),
            Some(
                dirfile_hash_2
                    as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
            ),
            Some(
                dirfile_hash_cmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    new = xmalloc(::core::mem::size_of::<dirfile>() as size_t) as *mut dirfile;
    (*new).length = strlen(filename) as size_t;
    (*new).name = strcache_add_len(filename, (*new).length);
    (*new).impossible = 1 as ::core::ffi::c_short;
    hash_insert(
        &raw mut (*(*dir).contents).dirfiles,
        new as *const ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn file_impossible_p(
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut dirend: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dir: *mut directory_contents = ::core::ptr::null_mut::<directory_contents>();
    let mut dirfile: *mut dirfile = ::core::ptr::null_mut::<dirfile>();
    let mut dirfile_key: dirfile = dirfile {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        length: 0,
        impossible: 0,
        type_0: 0,
    };
    dirend = strrchr(filename, '/' as i32);
    if dirend.is_null() {
        dir = (*find_directory(b".\0" as *const u8 as *const ::core::ffi::c_char)).contents;
    } else {
        let mut dirname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut slash: *const ::core::ffi::c_char = dirend;
        if dirend == filename {
            dirname = b"/\0" as *const u8 as *const ::core::ffi::c_char;
        } else {
            let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            alloca_allocations.push(::std::vec::from_elem(
                0,
                (dirend.offset_from(filename) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                    as ::core::ffi::c_ulong as usize,
            ));
            cp = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                cp as *mut ::core::ffi::c_void,
                filename as *const ::core::ffi::c_void,
                dirend.offset_from(filename) as ::core::ffi::c_long as size_t,
            );
            *cp.offset(dirend.offset_from(filename) as ::core::ffi::c_long as isize) =
                '\0' as i32 as ::core::ffi::c_char;
            dirname = cp;
        }
        dir = (*find_directory(dirname)).contents;
        filename = slash.offset(1 as ::core::ffi::c_int as isize);
    }
    if dir.is_null() || (*dir).dirfiles.ht_vec.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    dirfile_key.name = filename;
    dirfile_key.length = strlen(filename) as size_t;
    dirfile = hash_find_item(
        &raw mut (*dir).dirfiles,
        &raw mut dirfile_key as *const ::core::ffi::c_void,
    ) as *mut dirfile;
    if !dirfile.is_null() {
        return (*dirfile).impossible as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dir_name(
    mut dir: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    return (*find_directory(dir)).name;
}
#[no_mangle]
pub unsafe extern "C" fn print_dir_data_base() {
    let mut files: ::core::ffi::c_uint = 0;
    let mut impossible: ::core::ffi::c_uint = 0;
    let mut dir_slot: *mut *mut directory = ::core::ptr::null_mut::<*mut directory>();
    let mut dir_end: *mut *mut directory = ::core::ptr::null_mut::<*mut directory>();
    puts(b"\n# Directories\n\0" as *const u8 as *const ::core::ffi::c_char);
    impossible = 0 as ::core::ffi::c_uint;
    files = impossible;
    dir_slot = directories.ht_vec as *mut *mut directory;
    dir_end = dir_slot.offset(directories.ht_size as isize);
    while dir_slot < dir_end {
        let mut dir: *mut directory = *dir_slot;
        if !(dir.is_null()
            || dir as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
        {
            if (*dir).contents.is_null() {
                printf(
                    b"# %s: could not be stat'd.\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*dir).name,
                );
            } else if (*(*dir).contents).dirfiles.ht_vec.is_null() {
                printf(
                    b"# %s (device %ld, inode %ld): could not be opened.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*dir).name,
                    (*(*dir).contents).dev as ::core::ffi::c_long,
                    (*(*dir).contents).ino as ::core::ffi::c_long,
                );
            } else {
                let mut f: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                let mut im: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                let mut files_slot: *mut *mut dirfile = ::core::ptr::null_mut::<*mut dirfile>();
                let mut files_end: *mut *mut dirfile = ::core::ptr::null_mut::<*mut dirfile>();
                files_slot = (*(*dir).contents).dirfiles.ht_vec as *mut *mut dirfile;
                files_end = files_slot.offset((*(*dir).contents).dirfiles.ht_size as isize);
                while files_slot < files_end {
                    let mut df: *mut dirfile = *files_slot;
                    if !(df.is_null()
                        || df as *mut ::core::ffi::c_void
                            == hash_deleted_item as *mut ::core::ffi::c_void)
                    {
                        if (*df).impossible != 0 {
                            im = im.wrapping_add(1);
                        } else {
                            f = f.wrapping_add(1);
                        }
                    }
                    files_slot = files_slot.offset(1);
                }
                printf(
                    b"# %s (device %ld, inode %ld): \0" as *const u8 as *const ::core::ffi::c_char,
                    (*dir).name,
                    (*(*dir).contents).dev as ::core::ffi::c_long,
                    (*(*dir).contents).ino as ::core::ffi::c_long,
                );
                if f == 0 as ::core::ffi::c_uint {
                    fputs(b"No\0" as *const u8 as *const ::core::ffi::c_char, stdout);
                } else {
                    printf(b"%u\0" as *const u8 as *const ::core::ffi::c_char, f);
                }
                fputs(
                    b" files, \0" as *const u8 as *const ::core::ffi::c_char,
                    stdout,
                );
                if im == 0 as ::core::ffi::c_uint {
                    fputs(b"no\0" as *const u8 as *const ::core::ffi::c_char, stdout);
                } else {
                    printf(b"%u\0" as *const u8 as *const ::core::ffi::c_char, im);
                }
                fputs(
                    b" impossibilities\0" as *const u8 as *const ::core::ffi::c_char,
                    stdout,
                );
                if (*(*dir).contents).dirstream.is_null() {
                    puts(b".\0" as *const u8 as *const ::core::ffi::c_char);
                } else {
                    puts(b" so far.\0" as *const u8 as *const ::core::ffi::c_char);
                }
                files = files.wrapping_add(f);
                impossible = impossible.wrapping_add(im);
            }
        }
        dir_slot = dir_slot.offset(1);
    }
    fputs(b"\n# \0" as *const u8 as *const ::core::ffi::c_char, stdout);
    if files == 0 as ::core::ffi::c_uint {
        fputs(b"No\0" as *const u8 as *const ::core::ffi::c_char, stdout);
    } else {
        printf(b"%u\0" as *const u8 as *const ::core::ffi::c_char, files);
    }
    fputs(
        b" files, \0" as *const u8 as *const ::core::ffi::c_char,
        stdout,
    );
    if impossible == 0 as ::core::ffi::c_uint {
        fputs(b"no\0" as *const u8 as *const ::core::ffi::c_char, stdout);
    } else {
        printf(
            b"%u\0" as *const u8 as *const ::core::ffi::c_char,
            impossible,
        );
    }
    printf(
        b" impossibilities in %lu directories.\n\0" as *const u8 as *const ::core::ffi::c_char,
        directories.ht_fill,
    );
}
unsafe extern "C" fn open_dirstream(
    mut directory: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    let mut new: *mut dirstream = ::core::ptr::null_mut::<dirstream>();
    let mut dir: *mut directory = find_directory(directory);
    if (*dir).contents.is_null() || (*(*dir).contents).dirfiles.ht_vec.is_null() {
        return NULL;
    }
    dir_contents_file_exists_p(dir, ::core::ptr::null::<::core::ffi::c_char>());
    new = xmalloc(::core::mem::size_of::<dirstream>() as size_t) as *mut dirstream;
    (*new).contents = (*dir).contents;
    (*new).dirfile_slot = (*(*new).contents).dirfiles.ht_vec as *mut *mut dirfile;
    return new as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn read_dirstream(mut stream: *mut ::core::ffi::c_void) -> *mut dirent {
    static mut buf: *mut ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    static mut bufsz: size_t = 0;
    let ds: *mut dirstream = stream as *mut dirstream;
    let mut dc: *mut directory_contents = (*ds).contents;
    let mut dirfile_end: *mut *mut dirfile =
        ((*dc).dirfiles.ht_vec as *mut *mut dirfile).offset((*dc).dirfiles.ht_size as isize);
    while (*ds).dirfile_slot < dirfile_end {
        let fresh0 = (*ds).dirfile_slot;
        (*ds).dirfile_slot = (*ds).dirfile_slot.offset(1);
        let mut df: *mut dirfile = *fresh0;
        if !(df.is_null()
            || df as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
            && (*df).impossible == 0
        {
            let mut d: *mut dirent = ::core::ptr::null_mut::<dirent>();
            let mut len: size_t = (*df).length.wrapping_add(1 as size_t);
            let mut sz: size_t = (::core::mem::size_of::<dirent>() as size_t)
                .wrapping_sub(::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t)
                .wrapping_add(len);
            if sz > bufsz {
                bufsz = bufsz.wrapping_mul(2 as size_t);
                if sz > bufsz {
                    bufsz = sz;
                }
                buf = xrealloc(buf as *mut ::core::ffi::c_void, bufsz) as *mut ::core::ffi::c_char;
            }
            d = buf as *mut dirent;
            (*d).d_ino = 1 as __ino_t;
            (*d).d_type = (*df).type_0;
            memcpy(
                &raw mut (*d).d_name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                (*df).name as *const ::core::ffi::c_void,
                len as size_t,
            );
            return d;
        }
    }
    return ::core::ptr::null_mut::<dirent>();
}
#[no_mangle]
pub unsafe extern "C" fn dir_setup_glob(mut gl: *mut glob_t) {
    (*gl).gl_offs = 0 as __size_t;
    (*gl).gl_opendir = Some(
        open_dirstream
            as unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void,
    )
        as Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void>;
    (*gl).gl_readdir =
        Some(read_dirstream as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut dirent)
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut dirent>;
    (*gl).gl_closedir = Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    (*gl).gl_lstat = Some(
        lstat as unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int,
        >;
    (*gl).gl_stat = Some(
        stat as unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int,
        >;
}
#[no_mangle]
pub unsafe extern "C" fn hash_init_directories() {
    hash_init(
        &raw mut directories,
        DIRECTORY_BUCKETS as ::core::ffi::c_ulong,
        Some(
            directory_hash_1
                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            directory_hash_2
                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            directory_hash_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    hash_init(
        &raw mut directory_contents,
        DIRECTORY_BUCKETS as ::core::ffi::c_ulong,
        Some(
            directory_contents_hash_1
                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            directory_contents_hash_2
                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            directory_contents_hash_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
}
