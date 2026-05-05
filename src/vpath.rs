use libc::{__errno_location, free, printf, puts, strcmp, strrchr};
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type dep;
    pub type commands;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xrealloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn find_percent(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn dir_file_exists_p(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn dir_name(_: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn strcache_add(str: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn strcache_add_len(str: *const ::core::ffi::c_char, len: size_t)
        -> *const ::core::ffi::c_char;
    static mut stopchar_map: [::core::ffi::c_ushort; 0];
    fn lookup_file(name: *const ::core::ffi::c_char) -> *mut file;
    fn file_timestamp_cons(
        _: *const ::core::ffi::c_char,
        _: time_t,
        _: ::core::ffi::c_long,
    ) -> uintmax_t;
    fn expand_variable_buf(
        buf: *mut ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
        length: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn pattern_matches(
        pattern: *const ::core::ffi::c_char,
        percent: *const ::core::ffi::c_char,
        str: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
pub type uintmax_t = ::libc::uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const ::core::ffi::c_char,
    pub hname: *const ::core::ffi::c_char,
    pub vpath: *const ::core::ffi::c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const ::core::ffi::c_char,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: ::core::ffi::c_uint,
    pub command_flags: ::core::ffi::c_int,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(
        name = "low_resolution_time",
        ty = "::core::ffi::c_uint",
        bits = "8..=8"
    )]
    #[bitfield(name = "tried_implicit", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "::core::ffi::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "::core::ffi::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "::core::ffi::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "::core::ffi::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "::core::ffi::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "::core::ffi::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "::core::ffi::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "::core::ffi::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "::core::ffi::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "::core::ffi::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "::core::ffi::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "::core::ffi::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "::core::ffi::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "::core::ffi::c_uint", bits = "24..=24")]
    #[bitfield(name = "suffix", ty = "::core::ffi::c_uint", bits = "25..=25")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped_suffix:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type cmd_state = ::core::ffi::c_uint;
pub const cs_finished: cmd_state = 3;
pub const cs_running: cmd_state = 2;
pub const cs_deps_running: cmd_state = 1;
pub const cs_not_started: cmd_state = 0;
pub type update_status = ::core::ffi::c_uint;
pub const us_failed: update_status = 3;
pub const us_question: update_status = 2;
pub const us_none: update_status = 1;
pub const us_success: update_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set_list {
    pub next: *mut variable_set_list,
    pub set: *mut variable_set,
    pub next_is_parent: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set {
    pub table: hash_table,
}
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
pub struct vpath {
    pub next: *mut vpath,
    pub pattern: *const ::core::ffi::c_char,
    pub percent: *const ::core::ffi::c_char,
    pub patlen: size_t,
    pub searchpath: *mut *const ::core::ffi::c_char,
    pub maxlen: size_t,
}
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const UNKNOWN_MTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OLD_MTIME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut vpaths: *mut vpath = ::core::ptr::null::<vpath>() as *mut vpath;
static mut general_vpath: *mut vpath = ::core::ptr::null::<vpath>() as *mut vpath;
static mut gpaths: *mut vpath = ::core::ptr::null::<vpath>() as *mut vpath;
#[no_mangle]
pub unsafe extern "C" fn build_vpath_lists() {
    let mut new: *mut vpath = ::core::ptr::null_mut::<vpath>();
    let mut old: *mut vpath = ::core::ptr::null_mut::<vpath>();
    let mut nexto: *mut vpath = ::core::ptr::null_mut::<vpath>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    old = vpaths;
    while !old.is_null() {
        nexto = (*old).next;
        (*old).next = new;
        new = old;
        old = nexto;
    }
    vpaths = new;
    p = expand_variable_buf(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        b"VPATH\0" as *const u8 as *const ::core::ffi::c_char,
        5 as size_t,
    );
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        != 0 as ::core::ffi::c_int
    {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int != '\0' as i32 {
        let mut save_vpaths: *mut vpath = vpaths;
        let mut gp: [::core::ffi::c_char; 2] =
            ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"%\0");
        vpaths = ::core::ptr::null_mut::<vpath>();
        construct_vpath_list(&raw mut gp as *mut ::core::ffi::c_char, p);
        general_vpath = vpaths;
        vpaths = save_vpaths;
    }
    p = expand_variable_buf(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        b"GPATH\0" as *const u8 as *const ::core::ffi::c_char,
        5 as size_t,
    );
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        != 0 as ::core::ffi::c_int
    {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int != '\0' as i32 {
        let mut save_vpaths_0: *mut vpath = vpaths;
        let mut gp_0: [::core::ffi::c_char; 2] =
            ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"%\0");
        vpaths = ::core::ptr::null_mut::<vpath>();
        construct_vpath_list(&raw mut gp_0 as *mut ::core::ffi::c_char, p);
        gpaths = vpaths;
        vpaths = save_vpaths_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn construct_vpath_list(
    mut pattern: *mut ::core::ffi::c_char,
    mut dirpath: *mut ::core::ffi::c_char,
) {
    let mut elem: ::core::ffi::c_uint = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut vpath: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut maxvpath: size_t = 0;
    let mut maxelem: ::core::ffi::c_uint = 0;
    let mut percent: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !pattern.is_null() {
        percent = find_percent(pattern);
    }
    if dirpath.is_null() {
        let mut path: *mut vpath = ::core::ptr::null_mut::<vpath>();
        let mut lastpath: *mut vpath = ::core::ptr::null_mut::<vpath>();
        lastpath = ::core::ptr::null_mut::<vpath>();
        path = vpaths;
        while !path.is_null() {
            let mut next: *mut vpath = (*path).next;
            if pattern.is_null()
                || (percent.is_null() && (*path).percent.is_null()
                    || percent.offset_from(pattern) as ::core::ffi::c_long
                        == (*path).percent.offset_from((*path).pattern) as ::core::ffi::c_long)
                    && (*pattern as ::core::ffi::c_int == *(*path).pattern as ::core::ffi::c_int
                        && (*pattern as ::core::ffi::c_int == '\0' as i32
                            || strcmp(
                                pattern.offset(1 as ::core::ffi::c_int as isize),
                                (*path).pattern.offset(1 as ::core::ffi::c_int as isize),
                            ) == 0))
            {
                if lastpath.is_null() {
                    vpaths = (*path).next;
                } else {
                    (*lastpath).next = next;
                }
                free((*path).searchpath as *mut ::core::ffi::c_void);
                free(path as *mut ::core::ffi::c_void);
            } else {
                lastpath = path;
            }
            path = next;
        }
        return;
    }
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*dirpath as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x40 as ::core::ffi::c_int)
        != 0 as ::core::ffi::c_int
    {
        dirpath = dirpath.offset(1);
    }
    maxelem = 2 as ::core::ffi::c_uint;
    p = dirpath;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        let fresh0 = p;
        p = p.offset(1);
        if *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*fresh0 as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x40 as ::core::ffi::c_int)
            != 0 as ::core::ffi::c_int
        {
            maxelem = maxelem.wrapping_add(1);
        }
    }
    vpath = xmalloc(
        (maxelem as size_t)
            .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
    ) as *mut *const ::core::ffi::c_char;
    maxvpath = 0 as size_t;
    elem = 0 as ::core::ffi::c_uint;
    p = dirpath;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        let mut v: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut len: size_t = 0;
        v = p;
        while *p as ::core::ffi::c_int != '\0' as i32
            && *p as ::core::ffi::c_int != PATH_SEPARATOR_CHAR
            && !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                & 0x2 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int)
        {
            p = p.offset(1);
        }
        len = p.offset_from(v) as ::core::ffi::c_long as size_t;
        if len > 1 as size_t
            && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == '/' as i32
        {
            len = len.wrapping_sub(1);
        }
        if len > 1 as size_t || *v as ::core::ffi::c_int != '.' as i32 {
            let fresh1 = elem;
            elem = elem.wrapping_add(1);
            let ref mut fresh2 = *vpath.offset(fresh1 as isize);
            *fresh2 = dir_name(strcache_add_len(v, len));
            if len > maxvpath {
                maxvpath = len;
            }
        }
        while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x40 as ::core::ffi::c_int)
            != 0 as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
    }
    if elem > 0 as ::core::ffi::c_uint {
        let mut path_0: *mut vpath = ::core::ptr::null_mut::<vpath>();
        if elem < maxelem.wrapping_sub(1 as ::core::ffi::c_uint) {
            vpath = xrealloc(
                vpath as *mut ::core::ffi::c_void,
                (elem.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
                    .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
            ) as *mut *const ::core::ffi::c_char;
        }
        let ref mut fresh3 = *vpath.offset(elem as isize);
        *fresh3 = ::core::ptr::null::<::core::ffi::c_char>();
        path_0 = xmalloc(::core::mem::size_of::<vpath>() as size_t) as *mut vpath;
        (*path_0).searchpath = vpath;
        (*path_0).maxlen = maxvpath;
        (*path_0).next = vpaths;
        vpaths = path_0;
        (*path_0).pattern = strcache_add(pattern);
        (*path_0).patlen = strlen(pattern) as size_t;
        (*path_0).percent = if !percent.is_null() {
            (*path_0)
                .pattern
                .offset(percent.offset_from(pattern) as ::core::ffi::c_long as isize)
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
    } else {
        free(vpath as *mut ::core::ffi::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gpath_search(
    mut file: *const ::core::ffi::c_char,
    mut len: size_t,
) -> ::core::ffi::c_int {
    if !gpaths.is_null() && len <= (*gpaths).maxlen {
        let mut gp: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        gp = (*gpaths).searchpath;
        while !(*gp).is_null() {
            if strncmp(*gp, file, len as size_t) == 0 as ::core::ffi::c_int
                && *(*gp).offset(len as isize) as ::core::ffi::c_int == '\0' as i32
            {
                return 1 as ::core::ffi::c_int;
            }
            gp = gp.offset(1);
        }
    }
    0 as ::core::ffi::c_int
}
unsafe extern "C" fn selective_vpath_search(
    mut path: *mut vpath,
    mut file: *const ::core::ffi::c_char,
    mut mtime_ptr: *mut uintmax_t,
    mut path_index: *mut ::core::ffi::c_uint,
) -> *const ::core::ffi::c_char {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut not_target: ::core::ffi::c_int = 0;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut filename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut vpath: *mut *const ::core::ffi::c_char = (*path).searchpath;
    let mut maxvpath: size_t = (*path).maxlen;
    let mut i: ::core::ffi::c_uint = 0;
    let mut flen: size_t = 0;
    let mut name_dplen: size_t = 0;
    let mut exists: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut f: *mut file = lookup_file(file);
    not_target = (f.is_null() || (*f).is_target() == 0) as ::core::ffi::c_int;
    flen = strlen(file) as size_t;
    n = strrchr(file, '/' as i32);
    name_dplen = (if !n.is_null() {
        n.offset_from(file) as ::core::ffi::c_long
    } else {
        0 as ::core::ffi::c_long
    }) as size_t;
    filename = if name_dplen > 0 as size_t {
        n.offset(1 as ::core::ffi::c_int as isize)
    } else {
        file
    };
    if name_dplen > 0 as size_t {
        flen = flen.wrapping_sub(name_dplen.wrapping_add(1 as size_t));
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        maxvpath
            .wrapping_add(1 as size_t)
            .wrapping_add(name_dplen)
            .wrapping_add(1 as size_t)
            .wrapping_add(flen)
            .wrapping_add(1 as size_t) as usize,
    ));
    name = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    let mut current_block_45: u64;
    i = 0 as ::core::ffi::c_uint;
    while !(*vpath.offset(i as isize)).is_null() {
        let mut exists_in_cache: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p: *mut ::core::ffi::c_char = name;
        let mut vlen: size_t = strlen(*vpath.offset(i as isize)) as size_t;
        p = mempcpy(
            p as *mut ::core::ffi::c_void,
            *vpath.offset(i as isize) as *const ::core::ffi::c_void,
            vlen as size_t,
        ) as *mut ::core::ffi::c_char;
        if name_dplen > 0 as size_t {
            let fresh4 = p;
            p = p.offset(1);
            *fresh4 = '/' as i32 as ::core::ffi::c_char;
            p = mempcpy(
                p as *mut ::core::ffi::c_void,
                file as *const ::core::ffi::c_void,
                name_dplen as size_t,
            ) as *mut ::core::ffi::c_char;
        }
        if p != name
            && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != '/' as i32
        {
            *p = '/' as i32 as ::core::ffi::c_char;
            memcpy(
                p.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                filename as *const ::core::ffi::c_void,
                (flen as size_t).wrapping_add(1 as size_t),
            );
        } else {
            memcpy(
                p as *mut ::core::ffi::c_void,
                filename as *const ::core::ffi::c_void,
                (flen as size_t).wrapping_add(1 as size_t),
            );
        }
        let mut f_0: *mut file = lookup_file(name);
        if !f_0.is_null() {
            exists = (not_target != 0 || (*f_0).is_target() as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int;
            if exists != 0
                && !mtime_ptr.is_null()
                && ((*f_0).last_mtime == OLD_MTIME as uintmax_t
                    || (*f_0).last_mtime
                        == (!(0 as ::core::ffi::c_int as uintmax_t)).wrapping_sub(
                            if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                                0 as ::core::ffi::c_int as uintmax_t
                            } else {
                                !(0 as ::core::ffi::c_int as uintmax_t)
                                    << (::core::mem::size_of::<uintmax_t>() as usize)
                                        .wrapping_mul(CHAR_BIT as usize)
                                        .wrapping_sub(1 as usize)
                            },
                        ))
            {
                *mtime_ptr = (*f_0).last_mtime;
                mtime_ptr = ::core::ptr::null_mut::<uintmax_t>();
            }
        }
        if exists == 0 {
            *p = '\0' as i32 as ::core::ffi::c_char;
            exists = dir_file_exists_p(name, filename);
            exists_in_cache = exists;
        }
        if exists != 0 {
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
            *p = '/' as i32 as ::core::ffi::c_char;
            if exists_in_cache != 0 {
                let mut e: ::core::ffi::c_int = 0;
                loop {
                    e = stat(name, &raw mut st);
                    if !(e == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                        break;
                    }
                }
                if e != 0 as ::core::ffi::c_int {
                    exists = 0 as ::core::ffi::c_int;
                    current_block_45 = 2868539653012386629;
                } else {
                    if !mtime_ptr.is_null() {
                        *mtime_ptr = file_timestamp_cons(
                            name,
                            st.st_mtim.tv_sec as time_t,
                            st.st_mtim.tv_nsec as ::core::ffi::c_long,
                        );
                        mtime_ptr = ::core::ptr::null_mut::<uintmax_t>();
                    }
                    current_block_45 = 7427571413727699167;
                }
            } else {
                current_block_45 = 7427571413727699167;
            }
            match current_block_45 {
                2868539653012386629 => {}
                _ => {
                    if !mtime_ptr.is_null() {
                        *mtime_ptr = UNKNOWN_MTIME as uintmax_t;
                    }
                    if !path_index.is_null() {
                        *path_index = i;
                    }
                    return strcache_add_len(
                        name,
                        (p.offset(1 as ::core::ffi::c_int as isize).offset_from(name)
                            as ::core::ffi::c_long as size_t)
                            .wrapping_add(flen),
                    );
                }
            }
        }
        i = i.wrapping_add(1);
    }
    ::core::ptr::null::<::core::ffi::c_char>()
}
#[no_mangle]
pub unsafe extern "C" fn vpath_search(
    mut file: *const ::core::ffi::c_char,
    mut mtime_ptr: *mut uintmax_t,
    mut vpath_index: *mut ::core::ffi::c_uint,
    mut path_index: *mut ::core::ffi::c_uint,
) -> *const ::core::ffi::c_char {
    let mut v: *mut vpath = ::core::ptr::null_mut::<vpath>();
    if *file.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32
        || vpaths.is_null() && general_vpath.is_null()
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if !vpath_index.is_null() {
        *vpath_index = 0 as ::core::ffi::c_uint;
        *path_index = 0 as ::core::ffi::c_uint;
    }
    v = vpaths;
    while !v.is_null() {
        if pattern_matches((*v).pattern, (*v).percent, file) != 0 {
            let mut p: *const ::core::ffi::c_char =
                selective_vpath_search(v, file, mtime_ptr, path_index);
            if !p.is_null() {
                return p;
            }
        }
        if !vpath_index.is_null() {
            *vpath_index = (*vpath_index).wrapping_add(1);
        }
        v = (*v).next;
    }
    if !general_vpath.is_null() {
        let mut p_0: *const ::core::ffi::c_char =
            selective_vpath_search(general_vpath, file, mtime_ptr, path_index);
        if !p_0.is_null() {
            return p_0;
        }
    }
    ::core::ptr::null::<::core::ffi::c_char>()
}
#[no_mangle]
pub unsafe extern "C" fn print_vpath_data_base() {
    let mut nvpaths: ::core::ffi::c_uint = 0;
    let mut v: *mut vpath = ::core::ptr::null_mut::<vpath>();
    puts(b"\n# VPATH Search Paths\n\0" as *const u8 as *const ::core::ffi::c_char);
    nvpaths = 0 as ::core::ffi::c_uint;
    v = vpaths;
    while !v.is_null() {
        let mut i: ::core::ffi::c_uint = 0;
        nvpaths = nvpaths.wrapping_add(1);
        printf(
            b"vpath %s \0" as *const u8 as *const ::core::ffi::c_char,
            (*v).pattern,
        );
        i = 0 as ::core::ffi::c_uint;
        while !(*(*v).searchpath.offset(i as isize)).is_null() {
            printf(
                b"%s%c\0" as *const u8 as *const ::core::ffi::c_char,
                *(*v).searchpath.offset(i as isize),
                if (*(*v)
                    .searchpath
                    .offset(i.wrapping_add(1 as ::core::ffi::c_uint) as isize))
                .is_null()
                {
                    '\n' as i32
                } else {
                    PATH_SEPARATOR_CHAR
                },
            );
            i = i.wrapping_add(1);
        }
        v = (*v).next;
    }
    if vpaths.is_null() {
        puts(b"# No 'vpath' search paths.\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        printf(
            b"\n# %u 'vpath' search paths.\n\0" as *const u8 as *const ::core::ffi::c_char,
            nvpaths,
        );
    }
    if general_vpath.is_null() {
        puts(
            b"\n# No general ('VPATH' variable) search path.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        let mut path: *mut *const ::core::ffi::c_char = (*general_vpath).searchpath;
        let mut i_0: ::core::ffi::c_uint = 0;
        fputs(
            b"\n# General ('VPATH' variable) search path:\n# \0" as *const u8
                as *const ::core::ffi::c_char,
            stdout,
        );
        i_0 = 0 as ::core::ffi::c_uint;
        while !(*path.offset(i_0 as isize)).is_null() {
            printf(
                b"%s%c\0" as *const u8 as *const ::core::ffi::c_char,
                *path.offset(i_0 as isize),
                if (*path.offset(i_0.wrapping_add(1 as ::core::ffi::c_uint) as isize)).is_null() {
                    '\n' as i32
                } else {
                    PATH_SEPARATOR_CHAR
                },
            );
            i_0 = i_0.wrapping_add(1);
        }
    };
}
pub const __CHAR_BIT__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PATH_SEPARATOR_CHAR: ::core::ffi::c_int = ':' as i32;
