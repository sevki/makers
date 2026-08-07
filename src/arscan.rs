pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
    __syscall_slong_t, __time_t, __uid_t, intmax_t, off_t, size_t, ssize_t, uintmax_t,
};
use crate::misc::{make_toui, readbuf, writebuf};
use libc::{__errno_location, close, open, strcmp};
extern "C" {
    fn fstat(__fd: i32, __buf: *mut stat) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> i32;
    fn memset(__s: *mut ::core::ffi::c_void, __c: i32, __n: size_t) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> i32;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub use crate::sys_stat::stat;
pub use crate::sys_stat::timespec;

pub type ar_member_func_t = Option<
    unsafe fn(
        i32,
        *const ::core::ffi::c_char,
        i32,
        ::core::ffi::c_long,
        ::core::ffi::c_long,
        ::core::ffi::c_long,
        intmax_t,
        i32,
        i32,
        ::core::ffi::c_uint,
        *const ::core::ffi::c_void,
    ) -> intmax_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArHdr {
    pub ar_name: [::core::ffi::c_char; 16],
    pub ar_date: [::core::ffi::c_char; 12],
    pub ar_uid: [::core::ffi::c_char; 6],
    pub ar_gid: [::core::ffi::c_char; 6],
    pub ar_mode: [::core::ffi::c_char; 8],
    pub ar_size: [::core::ffi::c_char; 10],
    pub ar_fmag: [::core::ffi::c_char; 2],
}
pub const EINTR: i32 = 4;
pub const INT_MAX: i32 = __INT_MAX__;
pub const CHAR_BIT: i32 = __CHAR_BIT__;
pub const O_RDONLY: i32 = 0;
pub const ARMAG: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"!<arch>\n\0") };
pub const SARMAG: i32 = 8;
pub const ARFMAG: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"`\n\0") };
pub const AR_HDR_SIZE: usize = ::core::mem::size_of::<ArHdr>();
/// Parse one fixed-width ASCII numeric field out of an `ar` archive header.
///
/// `field` is the raw bytes (space-padded ASCII), `base` is the numeric base
/// (8 for `ar_mode`, 10 for `ar_size`/`ar_date`/`ar_uid`/`ar_gid`), and `max`
/// is the largest accepted value.
///
/// This is a pure parser: it returns `None` on a malformed digit, an overflow,
/// or a value exceeding `max`, leaving error *reporting* to the caller (which
/// owns the `ExecContext` and the archive/member names). An all-spaces (or
/// empty) field returns `Some(0)`, matching the original C behavior.
fn parse_int(field: &[u8], base: u32, max: u64) -> Option<u64> {
    // Skip leading spaces, then take everything up to the next space or
    // end-of-field. Mirrors the C parser's two `while` loops.
    let start = field.iter().position(|&b| b != b' ').unwrap_or(field.len());
    let trailing = &field[start..];
    let end = trailing
        .iter()
        .position(|&b| b == b' ')
        .unwrap_or(trailing.len());
    let token = &trailing[..end];

    if token.is_empty() {
        return Some(0);
    }

    let max_char = b'0' + base as u8 - 1;
    let value = token
        .iter()
        .all(|&b| (b'0'..=max_char).contains(&b))
        .then(|| {
            // The digit-range check above guarantees ASCII, so this is valid UTF-8.
            ::core::str::from_utf8(token)
                .ok()
                .and_then(|s| u64::from_str_radix(s, base).ok())
        })
        .flatten()?;

    (value <= max).then_some(value)
}
/// Right-trims an `ar_name` archive-header field down to its significant
/// bytes: everything up to (but not including) the run of trailing ASCII
/// spaces. An all-spaces field trims to an empty slice.
///
/// `ar_name` is raw on-disk archive data, not platform `char` text, so this
/// takes plain bytes rather than `c_char` — the `c_char`/`u8` conversion
/// happens once, at the boundary where the header is read off disk.
fn trim_ar_name(field: &[u8; 16]) -> &[u8] {
    match field.iter().rposition(|&b| b != b' ') {
        Some(last) => &field[..=last],
        None => &field[..0],
    }
}

/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_scan(
    ctx: &crate::execctx::ExecContext,
    archive: *const ::core::ffi::c_char,
    function: ar_member_func_t,
    arg: *const ::core::ffi::c_void,
) -> intmax_t {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut namemap: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut namemap_size: ::core::ffi::c_uint = 0;
    let desc: i32 = open(archive, O_RDONLY, 0);
    if desc < 0 {
        return -1_i32 as intmax_t;
    }
    let mut buf: [::core::ffi::c_char; 8] = [0; 8];
    let nread: i32;
    nread = readbuf(
        desc,
        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        SARMAG as size_t,
    ) as i32;
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
            let mut member_header: ArHdr = ArHdr {
                ar_name: [0; 16],
                ar_date: [0; 12],
                ar_uid: [0; 6],
                ar_gid: [0; 6],
                ar_mode: [0; 8],
                ar_size: [0; 10],
                ar_fmag: [0; 2],
            };
            let mut namebuf: [u8; 17] = [0; 17];
            let mut name: *mut ::core::ffi::c_char;
            let is_namemap: i32;
            let mut long_name: i32 = 0;
            let fnval: intmax_t;
            let mut o: off_t;
            memset(
                &raw mut member_header as *mut ::core::ffi::c_void,
                0,
                ::core::mem::size_of::<ArHdr>() as size_t,
            );
            loop {
                o = lseek(desc, member_offset as __off_t, 0) as off_t;
                if !(o == -1_i32 as off_t && *__errno_location() == EINTR) {
                    break;
                }
            }
            if o < 0 as off_t {
                break;
            }
            nread_0 = readbuf(
                desc,
                &raw mut member_header as *mut ::core::ffi::c_void,
                AR_HDR_SIZE,
            );
            if nread_0 == 0 as ssize_t {
                close(desc);
                return 0 as intmax_t;
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
                break;
            }
            name = &raw mut namebuf as *mut u8 as *mut ::core::ffi::c_char;
            let ar_name_bytes: [u8; 16] = member_header.ar_name.map(|c| c as u8);
            let trimmed = trim_ar_name(&ar_name_bytes);
            namebuf[..trimmed.len()].copy_from_slice(trimmed);
            namebuf[trimmed.len()..].fill(0);
            let p: *mut ::core::ffi::c_char = name.add(trimmed.len().saturating_sub(1));
            is_namemap = (strcmp(name, b"//\0" as *const u8 as *const ::core::ffi::c_char) == 0
                || strcmp(
                    name,
                    b"ARFILENAMES/\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0) as i32;
            if *p as i32 == '/' as i32 {
                *p = 0;
            }
            if is_namemap == 0
                && (*name.offset(0_i32 as isize) as i32 == ' ' as i32
                    || *name.offset(0_i32 as isize) as i32 == '/' as i32)
                && !namemap.is_null()
            {
                let Ok(name_off) =
                    make_toui(::core::ffi::CStr::from_ptr(name.offset(1_i32 as isize)))
                else {
                    break;
                };
                if name_off >= namemap_size {
                    break;
                }
                name = namemap.offset(name_off as isize);
                let name_len: size_t = strlen(name) as size_t;
                if name_len < 1 {
                    break;
                }
                long_name = 1;
            } else if *name.offset(0_i32 as isize) as i32 == '#' as i32
                && *name.offset(1_i32 as isize) as i32 == '1' as i32
                && *name.offset(2_i32 as isize) as i32 == '/' as i32
            {
                let name_len_0 =
                    make_toui(::core::ffi::CStr::from_ptr(name.offset(3_i32 as isize)))
                        .unwrap_or(0);
                if name_len_0 == 0
                    || name_len_0
                        >= (if 4096_i32 < 2147483647_i32 {
                            4096_i32
                        } else {
                            2147483647_i32
                        }) as ::core::ffi::c_uint
                {
                    break;
                }
                alloca_allocations.push(::std::vec::from_elem(
                    0,
                    name_len_0.wrapping_add(1) as ::core::ffi::c_ulong as usize,
                ));
                name =
                    alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
                nread_0 = readbuf(desc, name as *mut ::core::ffi::c_void, name_len_0 as size_t);
                if nread_0 < 0 as ssize_t || nread_0 as ::core::ffi::c_uint != name_len_0 {
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
            // `parse_int` is pure; reporting the fatal lives here, at the
            // impure boundary that owns `ctx` and the archive/member names.
            let parse_field = |field: &[u8], base: u32, max: u64, what: &str| -> u64 {
                parse_int(field, base, max).unwrap_or_else(|| {
                    crate::output::msg::fatal(
                        ctx,
                        None,
                        &format!("invalid {what} for archive {archive_str} member {name_str}"),
                    )
                })
            };
            let eltmode = parse_field(mode_field, 8, ::core::ffi::c_uint::MAX as u64, "mode")
                as ::core::ffi::c_uint;
            let eltsize = parse_field(size_field, 10, ::core::ffi::c_long::MAX as u64, "size")
                as ::core::ffi::c_long;
            let eltdate = parse_field(date_field, 10, intmax_t::MAX as u64, "date") as intmax_t;
            let eltuid = parse_field(uid_field, 10, i32::MAX as u64, "uid") as i32;
            let eltgid = parse_field(gid_field, 10, i32::MAX as u64, "gid") as i32;
            fnval = Some(function.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                desc,
                name,
                (long_name == 0) as i32,
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
                    break;
                }
                namemap_size = eltsize as ::core::ffi::c_uint;
                limit = namemap.offset(eltsize as isize);
                clear = namemap;
                while clear < limit {
                    if *clear as i32 == '\n' as i32 {
                        *clear = 0;
                        if *clear.offset(-1_i32 as isize) as i32 == '/' as i32 {
                            *clear.offset(-1_i32 as isize) = 0;
                        }
                    }
                    clear = clear.offset(1_i32 as isize);
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
    }
    close(desc);
    -2_i32 as intmax_t
}
/// Does archive member `name` refer to header member `mem`? Thin `&CStr`
/// wrapper over [`ar_name_equal_bytes`] (which holds the actual comparison).
pub fn ar_name_equal(
    name: &::core::ffi::CStr,
    mem: &::core::ffi::CStr,
    truncated: bool,
) -> bool {
    ar_name_equal_bytes(name.to_bytes(), mem.to_bytes(), truncated)
}

/// Number of significant `ar_name` bytes a truncated (System V/GNU short)
/// archive member name is compared on: `sizeof ar_hdr.ar_name - 1`.
const AR_NAME_CMP_LEN: usize = 15;

/// Does archive member `name` refer to header member `mem`? Pure mirror of
/// make's `ar_name_equal`: try a full match first, then retry on `name`'s
/// basename, comparing only the first [`AR_NAME_CMP_LEN`] bytes when the
/// archive format truncates member names.
fn ar_name_equal_bytes(name: &[u8], mem: &[u8], truncated: bool) -> bool {
    if name == mem {
        return true;
    }
    // An archive member name has no directory part: retry on the basename.
    let name = match name.iter().rposition(|&c| c == b'/') {
        Some(i) => &name[i + 1..],
        None => name,
    };
    if truncated {
        strncmp_eq(name, mem, AR_NAME_CMP_LEN)
    } else {
        name == mem
    }
}

/// Equivalent of `strncmp(a, b, n) == 0` for NUL-terminated strings supplied as
/// their NUL-free byte slices: the end of a slice acts as the terminating NUL.
fn strncmp_eq(a: &[u8], b: &[u8], n: usize) -> bool {
    for i in 0..n {
        let ca = a.get(i).copied().unwrap_or(0);
        let cb = b.get(i).copied().unwrap_or(0);
        if ca != cb {
            return false;
        }
        if ca == 0 {
            break;
        }
    }
    true
}
// The argument list is the fixed ar_scan callback protocol.
#[allow(clippy::too_many_arguments)]
unsafe fn ar_member_pos(
    mut _desc: i32,
    mem: *const ::core::ffi::c_char,
    truncated: i32,
    hdrpos: ::core::ffi::c_long,
    mut _datapos: ::core::ffi::c_long,
    mut _size: ::core::ffi::c_long,
    mut _date: intmax_t,
    mut _uid: i32,
    mut _gid: i32,
    mut _mode: ::core::ffi::c_uint,
    name: *const ::core::ffi::c_void,
) -> intmax_t {
    if !ar_name_equal(
        ::core::ffi::CStr::from_ptr(name as *const ::core::ffi::c_char),
        ::core::ffi::CStr::from_ptr(mem),
        truncated != 0,
    ) {
        return 0 as intmax_t;
    }
    hdrpos as intmax_t
}
/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_member_touch(
    ctx: &crate::execctx::ExecContext,
    arname: *const ::core::ffi::c_char,
    memname: *const ::core::ffi::c_char,
) -> i32 {
    let pos: intmax_t = ar_scan(
        ctx,
        arname,
        Some(ar_member_pos),
        memname as *const ::core::ffi::c_void,
    );
    let opos: off_t;
    let mut fd: i32;
    let mut ar_hdr: ArHdr = ArHdr {
        ar_name: [0; 16],
        ar_date: [0; 12],
        ar_uid: [0; 6],
        ar_gid: [0; 6],
        ar_mode: [0; 8],
        ar_size: [0; 10],
        ar_fmag: [0; 2],
    };
    let mut o: off_t;
    let mut r: i32;
    let datelen: i32;
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
        return pos as i32;
    }
    if pos == 0 {
        return 1;
    }
    opos = pos as off_t;
    loop {
        fd = open(arname, 0o2_i32, 0o666_i32);
        if !(fd == -1_i32 && *__errno_location() == EINTR) {
            break;
        }
    }
    if fd < 0 {
        return -3_i32;
    }
    loop {
        o = lseek(fd, opos as __off_t, 0) as off_t;
        if !(o == -1_i32 as off_t && *__errno_location() == EINTR) {
            break;
        }
    }
    if !(o < 0 as off_t) {
        r = readbuf(fd, &raw mut ar_hdr as *mut ::core::ffi::c_void, AR_HDR_SIZE) as i32;
        if !(r as usize != AR_HDR_SIZE) {
            loop {
                r = fstat(fd, &raw mut statbuf);
                if !(r == -1_i32 && *__errno_location() == EINTR) {
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
                    && datelen < ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as i32
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
                        if !(o == -1_i32 as off_t && *__errno_location() == EINTR) {
                            break;
                        }
                    }
                    if !(o < 0 as off_t) {
                        r = writebuf(
                            fd,
                            &raw mut ar_hdr as *const ::core::ffi::c_void,
                            AR_HDR_SIZE,
                        ) as i32;
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
    -3_i32
}
pub const __CHAR_BIT__: i32 = 8;
pub const __INT_MAX__: i32 = 2147483647_i32;

#[cfg(test)]
mod ar_name_equal_tests {
    use super::ar_name_equal_bytes;

    #[test]
    fn exact_match() {
        assert!(ar_name_equal_bytes(b"foo.o", b"foo.o", false));
        assert!(ar_name_equal_bytes(b"", b"", false));
    }

    #[test]
    fn basename_is_compared_when_name_has_directory() {
        // The header member name has no directory part, so `name`'s leading
        // directory is stripped before comparing.
        assert!(ar_name_equal_bytes(b"dir/foo.o", b"foo.o", false));
        assert!(ar_name_equal_bytes(b"a/b/c/foo.o", b"foo.o", false));
        assert!(!ar_name_equal_bytes(b"dir/foo.o", b"bar.o", false));
    }

    #[test]
    fn untruncated_requires_full_equality() {
        // 16-char basenames differ only past byte 15; without truncation they
        // are distinct.
        assert!(!ar_name_equal_bytes(
            b"abcdefghijklmno1",
            b"abcdefghijklmno2",
            false,
        ));
    }

    #[test]
    fn truncated_compares_only_first_15_bytes() {
        // Same first 15 bytes, differing 16th: equal under truncation.
        assert!(ar_name_equal_bytes(
            b"abcdefghijklmno1",
            b"abcdefghijklmno2",
            true,
        ));
        // A difference within the first 15 bytes is still caught.
        assert!(!ar_name_equal_bytes(
            b"abcdefghijklmnX",
            b"abcdefghijklmnY",
            true
        ));
        // Shorter-but-equal names match under truncation too.
        assert!(ar_name_equal_bytes(b"short.o", b"short.o", true));
        assert!(!ar_name_equal_bytes(b"short.o", b"shorter.o", true));
    }
}

#[cfg(test)]
mod ar_name_equal_unsafe_oracle {
    //! `ar_name_equal` was a `pub unsafe fn` over `*const c_char`/`i32`; this
    //! keeps the verbatim pre-conversion implementation as a differential
    //! oracle and asserts the safe `&CStr`/`bool` version agrees (AGENTS
    //! rule 3).
    use super::{ar_name_equal, ar_name_equal_bytes};
    use ::core::ffi::{c_char, CStr};

    /// Verbatim pre-conversion implementation.
    unsafe fn oracle(name: *const c_char, mem: *const c_char, truncated: i32) -> i32 {
        let name = CStr::from_ptr(name).to_bytes();
        let mem = CStr::from_ptr(mem).to_bytes();
        ar_name_equal_bytes(name, mem, truncated != 0) as i32
    }

    /// Drive both implementations and assert identical verdicts.
    fn check(name: &CStr, mem: &CStr, truncated: bool) {
        let safe = ar_name_equal(name, mem, truncated);
        // SAFETY: both are valid NUL-terminated C strings.
        let oracle_res = unsafe { oracle(name.as_ptr(), mem.as_ptr(), truncated as i32) };
        assert_eq!(
            safe as i32, oracle_res,
            "name={name:?} mem={mem:?} truncated={truncated}"
        );
    }

    #[test]
    fn differential() {
        check(c"foo.o", c"foo.o", false);
        check(c"dir/foo.o", c"foo.o", false);
        check(c"foo.o", c"bar.o", false);
        check(c"abcdefghijklmno1", c"abcdefghijklmno2", true);
        check(c"abcdefghijklmno1", c"abcdefghijklmno2", false);
        check(c"short.o", c"shorter.o", true);
    }
}

#[cfg(test)]
mod trim_ar_name_tests {
    use super::trim_ar_name;

    #[test]
    fn full_16_bytes_no_trailing_space() {
        assert_eq!(trim_ar_name(b"abcdefghijklmnop"), b"abcdefghijklmnop");
    }

    #[test]
    fn trailing_spaces_trimmed() {
        assert_eq!(trim_ar_name(b"foo.o           "), b"foo.o");
    }

    #[test]
    fn all_spaces_trim_to_empty() {
        assert_eq!(trim_ar_name(b"                "), b"");
    }

    #[test]
    fn trailing_slash_terminator_survives_trim() {
        // The `/` short-name terminator is not a space, so it is kept; the
        // caller strips it separately once it knows this is a short name.
        assert_eq!(trim_ar_name(b"short.o/        "), b"short.o/");
    }
}

#[cfg(test)]
mod trim_ar_name_unsafe_oracle {
    //! The original c2rust `ar_scan` trimmed `ar_name` in place with a raw
    //! pointer walk (`p = p.offset(-1); *p == ' '`) starting one byte past
    //! the 16-byte field, leaving `p` pointing at the last significant byte
    //! (not one past it — the loop always decrements before its first
    //! check, and an all-spaces field walks `p` down to the field start).
    //! This keeps that walk verbatim as a differential oracle and asserts
    //! the safe, slice-based `trim_ar_name` agrees (AGENTS rule 3).
    use super::trim_ar_name;

    /// Verbatim pre-conversion pointer walk. `buf` holds the 16-byte field
    /// followed by one scratch byte (mirroring the original's 17-byte
    /// `namebuf`). Returns the raw index `p` lands on.
    unsafe fn oracle(buf: &mut [u8; 17]) -> usize {
        let name = buf.as_mut_ptr();
        let mut p = name.offset(16);
        loop {
            *p = 0;
            if !(p > name && {
                p = p.offset(-1);
                *p == b' '
            }) {
                break;
            }
        }
        p.offset_from(name) as usize
    }

    fn check(field: [u8; 16]) {
        let safe = trim_ar_name(&field);
        let field_is_all_spaces = field.iter().all(|&b| b == b' ');

        let mut buf: [u8; 17] = [0; 17];
        buf[..16].copy_from_slice(&field);
        // SAFETY: `buf` is a valid, fully-initialized 17-element array.
        let last = unsafe { oracle(&mut buf) };
        // `p` always lands on the index of the last significant byte,
        // *unless* the whole field is spaces, in which case it also lands
        // on index 0 — but there it means "nothing kept" (see module doc).
        let oracle_last = (!field_is_all_spaces || last != 0).then_some(last);

        match oracle_last {
            Some(last) => assert_eq!(safe, &buf[..=last], "field={field:?}"),
            None => assert_eq!(safe, b"", "field={field:?}"),
        }
    }

    #[test]
    fn differential() {
        check(*b"abcdefghijklmnop");
        check(*b"foo.o           ");
        check(*b"                ");
        check(*b"short.o/        ");
        check(*b"a               ");
        check(*b"          spaced");
    }
}
