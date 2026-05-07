use libc::{__errno_location, close, open, strcmp, strrchr};
pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
    __syscall_slong_t, __time_t, __uid_t, intmax_t, off_t, size_t, ssize_t, uintmax_t,
};
extern "C" {
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lseek(__fd: ::core::ffi::c_int, __offset: __off_t, __whence: ::core::ffi::c_int) -> __off_t;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn make_toui(
        _: *const ::core::ffi::c_char,
        _: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_uint;
    fn writebuf(_: ::core::ffi::c_int, _: *const ::core::ffi::c_void, _: size_t) -> ssize_t;
    fn readbuf(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_void, _: size_t) -> ssize_t;
}
pub use crate::sys_stat::timespec;
pub use crate::sys_stat::stat;

pub type ar_member_func_t = Option<
    unsafe extern "C" fn(
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
        ::core::ffi::c_long,
        ::core::ffi::c_long,
        ::core::ffi::c_long,
        intmax_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_uint,
        *const ::core::ffi::c_void,
    ) -> intmax_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ar_hdr {
    pub ar_name: [::core::ffi::c_char; 16],
    pub ar_date: [::core::ffi::c_char; 12],
    pub ar_uid: [::core::ffi::c_char; 6],
    pub ar_gid: [::core::ffi::c_char; 6],
    pub ar_mode: [::core::ffi::c_char; 8],
    pub ar_size: [::core::ffi::c_char; 10],
    pub ar_fmag: [::core::ffi::c_char; 2],
}
pub const EINTR: ::core::ffi::c_int = 4;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
pub const O_RDONLY: ::core::ffi::c_int = 0;
pub const ARMAG: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"!<arch>\n\0") };
pub const SARMAG: ::core::ffi::c_int = 8;
pub const ARFMAG: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"`\n\0") };
pub const AR_HDR_SIZE: usize = ::core::mem::size_of::<ar_hdr>();
/// Parse one fixed-width ASCII numeric field out of an `ar` archive header.
///
/// `field` is the raw bytes (space-padded ASCII), `base` is the numeric base
/// (8 for `ar_mode`, 10 for `ar_size`/`ar_date`/`ar_uid`/`ar_gid`), and `max`
/// is the largest accepted value. The remaining args supply context for the
/// fatal-error message on parse failure.
///
/// Aborts via `msg::fatal` on a malformed digit, an overflow, or a value
/// exceeding `max`. An all-spaces (or empty) field returns 0, matching the
/// original C behavior.
fn parse_int(field: &[u8], base: u32, max: u64, what: &str, archive: &str, name: &str) -> u64 {
    // Skip leading spaces, then take everything up to the next space or
    // end-of-field. Mirrors the C parser's two `while` loops.
    let start = field.iter().position(|&b| b != b' ').unwrap_or(field.len());
    let trailing = &field[start..];
    let end = trailing.iter().position(|&b| b == b' ').unwrap_or(trailing.len());
    let token = &trailing[..end];

    if token.is_empty() {
        return 0;
    }

    let max_char = b'0' + base as u8 - 1;
    let parsed = token
        .iter()
        .all(|&b| (b'0'..=max_char).contains(&b))
        .then(|| {
            // The digit-range check above guarantees ASCII, so this is valid UTF-8.
            ::core::str::from_utf8(token)
                .ok()
                .and_then(|s| u64::from_str_radix(s, base).ok())
        })
        .flatten();

    match parsed {
        Some(v) if v <= max => v,
        _ => crate::output::msg::fatal(
            None,
            &format!("invalid {what} for archive {archive} member {name}"),
        ),
    }
}
#[no_mangle]
pub unsafe extern "C" fn ar_scan(
    archive: *const ::core::ffi::c_char,
    function: ar_member_func_t,
    arg: *const ::core::ffi::c_void,
) -> intmax_t {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let current_block: u64;
    let mut namemap: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut namemap_size: ::core::ffi::c_uint = 0;
    let desc: ::core::ffi::c_int = open(archive, O_RDONLY, 0);
    if desc < 0 {
        return -(1 as ::core::ffi::c_int) as intmax_t;
    }
    let mut buf: [::core::ffi::c_char; 8] = [0; 8];
    let nread: ::core::ffi::c_int;
    nread = readbuf(
        desc,
        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        SARMAG as size_t,
    ) as ::core::ffi::c_int;
    if !(nread != SARMAG
        || memcmp(
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ARMAG.as_ptr() as *const ::core::ffi::c_void,
            SARMAG as size_t,
        ) != 0)
    {
        let mut member_offset: ::core::ffi::c_long = SARMAG as ::core::ffi::c_long;
        loop {
            let mut nread_0: ssize_t;
            let mut member_header: ar_hdr = ar_hdr {
                ar_name: [0; 16],
                ar_date: [0; 12],
                ar_uid: [0; 6],
                ar_gid: [0; 6],
                ar_mode: [0; 8],
                ar_size: [0; 10],
                ar_fmag: [0; 2],
            };
            let mut namebuf: [::core::ffi::c_char; 17] = [0; 17];
            let mut name: *mut ::core::ffi::c_char;
            let is_namemap: ::core::ffi::c_int;
            let mut long_name: ::core::ffi::c_int = 0;
            let eltsize: ::core::ffi::c_long;
            let eltmode: ::core::ffi::c_uint;
            let eltdate: intmax_t;
            let eltuid: ::core::ffi::c_int;
            let eltgid: ::core::ffi::c_int;
            let fnval: intmax_t;
            let mut o: off_t;
            memset(
                &raw mut member_header as *mut ::core::ffi::c_void,
                0,
                ::core::mem::size_of::<ar_hdr>() as size_t,
            );
            loop {
                o = lseek(desc, member_offset as __off_t, 0) as off_t;
                if !(o == -(1 as ::core::ffi::c_int) as off_t && *__errno_location() == EINTR) {
                    break;
                }
            }
            if o < 0 as off_t {
                current_block = 13383231232214443762;
                break;
            }
            nread_0 = readbuf(
                desc,
                &raw mut member_header as *mut ::core::ffi::c_void,
                AR_HDR_SIZE,
            );
            if nread_0 == 0 as ssize_t {
                current_block = 16203797167131938757;
                break;
            }
            if nread_0 as usize != AR_HDR_SIZE
                || memcmp(
                    &raw mut member_header.ar_fmag as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    ARFMAG.as_ptr() as *const ::core::ffi::c_void,
                    2,
                ) != 0
                    && 1 != 0
            {
                current_block = 13383231232214443762;
                break;
            }
            name = &raw mut namebuf as *mut ::core::ffi::c_char;
            memcpy(
                name as *mut ::core::ffi::c_void,
                &raw mut member_header.ar_name as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
            );
            let mut p: *mut ::core::ffi::c_char =
                name.offset(::core::mem::size_of::<[::core::ffi::c_char; 16]>() as usize as isize);
            loop {
                *p = 0;
                if !(p > name && {
                    p = p.offset(-(1 as ::core::ffi::c_int) as isize);
                    *p as ::core::ffi::c_int == ' ' as i32
                }) {
                    break;
                }
            }
            is_namemap = (strcmp(name, b"//\0" as *const u8 as *const ::core::ffi::c_char) == 0
                || strcmp(
                    name,
                    b"ARFILENAMES/\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0) as ::core::ffi::c_int;
            if *p as ::core::ffi::c_int == '/' as i32 {
                *p = 0;
            }
            if is_namemap == 0
                && (*name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
                    || *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32)
                && !namemap.is_null()
            {
                let mut err: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                let name_off: ::core::ffi::c_uint =
                    make_toui(name.offset(1 as ::core::ffi::c_int as isize), &raw mut err);
                let name_len: size_t;
                if !err.is_null() || name_off >= namemap_size {
                    current_block = 13383231232214443762;
                    break;
                }
                name = namemap.offset(name_off as isize);
                name_len = strlen(name) as size_t;
                if name_len < 1 {
                    current_block = 13383231232214443762;
                    break;
                }
                long_name = 1;
            } else if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32
                && *name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '1' as i32
                && *name.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '/' as i32
            {
                let mut err_0: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                let name_len_0: ::core::ffi::c_uint = make_toui(
                    name.offset(3 as ::core::ffi::c_int as isize),
                    &raw mut err_0,
                );
                if !err_0.is_null()
                    || name_len_0 == 0
                    || name_len_0
                        >= (if (4096 as ::core::ffi::c_int) < 2147483647 as ::core::ffi::c_int {
                            4096 as ::core::ffi::c_int
                        } else {
                            2147483647 as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint
                {
                    current_block = 13383231232214443762;
                    break;
                }
                alloca_allocations.push(::std::vec::from_elem(
                    0,
                    name_len_0.wrapping_add(1) as ::core::ffi::c_ulong
                        as usize,
                ));
                name =
                    alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
                nread_0 = readbuf(desc, name as *mut ::core::ffi::c_void, name_len_0 as size_t);
                if nread_0 < 0 as ssize_t || nread_0 as ::core::ffi::c_uint != name_len_0 {
                    current_block = 13383231232214443762;
                    break;
                }
                *name.offset(name_len_0 as isize) = 0;
                long_name = 1;
            }
            // SAFETY: `archive` and `name` are NUL-terminated C strings owned
            // by the caller / by the long-name buffer above; both outlive the
            // five `parse_int` calls below.
            let archive_str = ::core::ffi::CStr::from_ptr(archive).to_string_lossy();
            let name_str = ::core::ffi::CStr::from_ptr(name).to_string_lossy();
            // SAFETY: `member_header` is initialized above; reading the fixed
            // ASCII byte fields as `&[u8]` is valid (c_char and u8 share size
            // and alignment).
            let mode_field = ::core::slice::from_raw_parts(
                member_header.ar_mode.as_ptr() as *const u8,
                member_header.ar_mode.len(),
            );
            let size_field = ::core::slice::from_raw_parts(
                member_header.ar_size.as_ptr() as *const u8,
                member_header.ar_size.len(),
            );
            let date_field = ::core::slice::from_raw_parts(
                member_header.ar_date.as_ptr() as *const u8,
                member_header.ar_date.len(),
            );
            let uid_field = ::core::slice::from_raw_parts(
                member_header.ar_uid.as_ptr() as *const u8,
                member_header.ar_uid.len(),
            );
            let gid_field = ::core::slice::from_raw_parts(
                member_header.ar_gid.as_ptr() as *const u8,
                member_header.ar_gid.len(),
            );
            eltmode = parse_int(
                mode_field,
                8,
                ::core::ffi::c_uint::MAX as u64,
                "mode",
                &archive_str,
                &name_str,
            ) as ::core::ffi::c_uint;
            eltsize = parse_int(
                size_field,
                10,
                ::core::ffi::c_long::MAX as u64,
                "size",
                &archive_str,
                &name_str,
            ) as ::core::ffi::c_long;
            eltdate = parse_int(
                date_field,
                10,
                intmax_t::MAX as u64,
                "date",
                &archive_str,
                &name_str,
            ) as intmax_t;
            eltuid = parse_int(
                uid_field,
                10,
                ::core::ffi::c_int::MAX as u64,
                "uid",
                &archive_str,
                &name_str,
            ) as ::core::ffi::c_int;
            eltgid = parse_int(
                gid_field,
                10,
                ::core::ffi::c_int::MAX as u64,
                "gid",
                &archive_str,
                &name_str,
            ) as ::core::ffi::c_int;
            fnval = Some(function.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                desc,
                name,
                (long_name == 0) as ::core::ffi::c_int,
                member_offset,
                (member_offset as usize).wrapping_add(AR_HDR_SIZE) as ::core::ffi::c_long,
                eltsize,
                eltdate,
                eltuid,
                eltgid,
                eltmode,
                arg,
            );
            if fnval != 0 {
                close(desc);
                return fnval;
            }
            if is_namemap != 0 {
                let mut clear: *mut ::core::ffi::c_char;
                let limit: *mut ::core::ffi::c_char;
                if eltsize > INT_MAX as ::core::ffi::c_long {
                    current_block = 13383231232214443762;
                    break;
                }
                alloca_allocations.push(::std::vec::from_elem(
                    0,
                    (eltsize + 1) as ::core::ffi::c_ulong as usize,
                ));
                namemap =
                    alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
                nread_0 = readbuf(desc, namemap as *mut ::core::ffi::c_void, eltsize as size_t);
                if nread_0 != eltsize as ssize_t {
                    current_block = 13383231232214443762;
                    break;
                }
                namemap_size = eltsize as ::core::ffi::c_uint;
                limit = namemap.offset(eltsize as isize);
                clear = namemap;
                while clear < limit {
                    if *clear as ::core::ffi::c_int == '\n' as i32 {
                        *clear = 0;
                        if *clear.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == '/' as i32
                        {
                            *clear.offset(-(1 as ::core::ffi::c_int) as isize) = 0;
                        }
                    }
                    clear = clear.offset(1 as ::core::ffi::c_int as isize);
                }
                *limit = 0;
            }
            member_offset = (member_offset as ::core::ffi::c_ulong)
                .wrapping_add(AR_HDR_SIZE.wrapping_add(eltsize as usize) as ::core::ffi::c_ulong)
                as ::core::ffi::c_long as ::core::ffi::c_long;
            if member_offset % 2 != 0 {
                member_offset += 1;
            }
        }
        match current_block {
            13383231232214443762 => {}
            _ => {
                close(desc);
                return 0 as intmax_t;
            }
        }
    }
    close(desc);
    -(2 as ::core::ffi::c_int) as intmax_t
}
#[no_mangle]
pub unsafe extern "C" fn ar_name_equal(
    mut name: *const ::core::ffi::c_char,
    mem: *const ::core::ffi::c_char,
    truncated: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let p: *const ::core::ffi::c_char;
    if *name as ::core::ffi::c_int == *mem as ::core::ffi::c_int
        && (*name as ::core::ffi::c_int == 0
            || strcmp(name.offset(1 as ::core::ffi::c_int as isize), mem.offset(1 as ::core::ffi::c_int as isize), ) == 0)
    {
        return 1;
    }
    p = strrchr(name, '/' as i32);
    if !p.is_null() {
        name = p.offset(1 as ::core::ffi::c_int as isize);
    }
    if truncated != 0 {
        let mut _hdr: ar_hdr = ar_hdr {
            ar_name: [0; 16],
            ar_date: [0; 12],
            ar_uid: [0; 6],
            ar_gid: [0; 6],
            ar_mode: [0; 8],
            ar_size: [0; 10],
            ar_fmag: [0; 2],
        };
        return (strncmp(
            name,
            mem,
            (::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t)
                .wrapping_sub(1),
        ) == 0) as ::core::ffi::c_int;
    }
    (strcmp(name, mem) == 0) as ::core::ffi::c_int
}
unsafe extern "C" fn ar_member_pos(
    mut _desc: ::core::ffi::c_int,
    mem: *const ::core::ffi::c_char,
    truncated: ::core::ffi::c_int,
    hdrpos: ::core::ffi::c_long,
    mut _datapos: ::core::ffi::c_long,
    mut _size: ::core::ffi::c_long,
    mut _date: intmax_t,
    mut _uid: ::core::ffi::c_int,
    mut _gid: ::core::ffi::c_int,
    mut _mode: ::core::ffi::c_uint,
    name: *const ::core::ffi::c_void,
) -> intmax_t {
    if ar_name_equal(name as *const ::core::ffi::c_char, mem, truncated) == 0 {
        return 0 as intmax_t;
    }
    hdrpos as intmax_t
}
#[no_mangle]
pub unsafe extern "C" fn ar_member_touch(
    arname: *const ::core::ffi::c_char,
    memname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let pos: intmax_t = ar_scan(
        arname,
        Some(
            ar_member_pos
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_long,
                    ::core::ffi::c_long,
                    ::core::ffi::c_long,
                    intmax_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_uint,
                    *const ::core::ffi::c_void,
                ) -> intmax_t,
        ),
        memname as *const ::core::ffi::c_void,
    );
    let opos: off_t;
    let mut fd: ::core::ffi::c_int;
    let mut ar_hdr: ar_hdr = ar_hdr {
        ar_name: [0; 16],
        ar_date: [0; 12],
        ar_uid: [0; 6],
        ar_gid: [0; 6],
        ar_mode: [0; 8],
        ar_size: [0; 10],
        ar_fmag: [0; 2],
    };
    let mut o: off_t;
    let mut r: ::core::ffi::c_int;
    let datelen: ::core::ffi::c_int;
    let mut statbuf: stat = stat {
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
    if pos < 0 as intmax_t {
        return pos as ::core::ffi::c_int;
    }
    if pos == 0 {
        return 1;
    }
    opos = pos as off_t;
    loop {
        fd = open(
            arname,
            0o2 as ::core::ffi::c_int,
            0o666 as ::core::ffi::c_int,
        );
        if !(fd == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if fd < 0 {
        return -(3 as ::core::ffi::c_int);
    }
    loop {
        o = lseek(fd, opos as __off_t, 0) as off_t;
        if !(o == -(1 as ::core::ffi::c_int) as off_t && *__errno_location() == EINTR) {
            break;
        }
    }
    if !(o < 0 as off_t) {
        r = readbuf(fd, &raw mut ar_hdr as *mut ::core::ffi::c_void, AR_HDR_SIZE)
            as ::core::ffi::c_int;
        if !(r as usize != AR_HDR_SIZE) {
            loop {
                r = fstat(fd, &raw mut statbuf);
                if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                    break;
                }
            }
            if !(r < 0) {
                datelen = snprintf(
                    &raw mut ar_hdr.ar_date as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t,
                    b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
                    statbuf.st_mtim.tv_sec as intmax_t,
                );
                if 0 <= datelen
                    && datelen
                        < ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as ::core::ffi::c_int
                {
                    memset(
                        (&raw mut ar_hdr.ar_date as *mut ::core::ffi::c_char)
                            .offset(datelen as isize)
                            as *mut ::core::ffi::c_void,
                        ' ' as i32,
                        (::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t)
                            .wrapping_sub(datelen as size_t),
                    );
                    loop {
                        o = lseek(fd, opos as __off_t, 0) as off_t;
                        if !(o == -(1 as ::core::ffi::c_int) as off_t
                            && *__errno_location() == EINTR)
                        {
                            break;
                        }
                    }
                    if !(o < 0 as off_t) {
                        r = writebuf(
                            fd,
                            &raw mut ar_hdr as *const ::core::ffi::c_void,
                            AR_HDR_SIZE,
                        ) as ::core::ffi::c_int;
                        if !(r as usize != AR_HDR_SIZE) {
                            close(fd);
                            return 0;
                        }
                    }
                }
            }
        }
    }
    r = *__errno_location();
    close(fd);
    *__errno_location() = r;
    -(3 as ::core::ffi::c_int)
}
pub const __CHAR_BIT__: ::core::ffi::c_int = 8;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
