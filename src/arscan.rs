use libc::{__errno_location, close, open, strcmp, strrchr};
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
    fn fatal(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn make_toui(
        _: *const ::core::ffi::c_char,
        _: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_uint;
    fn writebuf(_: ::core::ffi::c_int, _: *const ::core::ffi::c_void, _: size_t) -> ssize_t;
    fn readbuf(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_void, _: size_t) -> ssize_t;
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = isize;
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
pub type intmax_t = ::libc::intmax_t;
pub type uintmax_t = ::libc::uintmax_t;
use crate::floc::Floc;

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
unsafe extern "C" fn parse_int(
    mut ptr: *const ::core::ffi::c_char,
    len: size_t,
    base: ::core::ffi::c_int,
    mut max: uintmax_t,
    mut type_0: *const ::core::ffi::c_char,
    mut archive: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) -> uintmax_t {
    let ep: *const ::core::ffi::c_char = ptr.offset(len as isize);
    let maxchar: ::core::ffi::c_int = '0' as i32 + base - 1;
    let mut val: uintmax_t = 0 as uintmax_t;
    while ptr < ep && *ptr as ::core::ffi::c_int == ' ' as i32 {
        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
    }
    while ptr < ep && *ptr as ::core::ffi::c_int != ' ' as i32 {
        if (*ptr as ::core::ffi::c_int) < '0' as i32
            || *ptr as ::core::ffi::c_int > maxchar
            || (if ::core::mem::size_of::<uintmax_t>() as usize
                == ::core::mem::size_of::<::core::ffi::c_schar>() as usize
            {
                if !((0 as ::core::ffi::c_int as uintmax_t)
                    < -(1 as ::core::ffi::c_int) as uintmax_t)
                {
                    if (if base < 0 {
                        if val < 0 as uintmax_t {
                            if ((if 1 != 0 {
                                0
                            } else {
                                (if 1 != 0 {
                                    0
                                } else {
                                    127
                                }) + base
                            }) - 1)
                                < 0
                            {
                                (val < (127 / base) as uintmax_t)
                                    as ::core::ffi::c_int
                            } else {
                                ((if (if (if ((if 1 != 0 {
                                    0
                                } else {
                                    base
                                }) - 1)
                                    < 0
                                {
                                    !(((((if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + 1)
                                        << (::core::mem::size_of::<::core::ffi::c_int>()
                                            as usize)
                                            .wrapping_mul(8 as usize)
                                            .wrapping_sub(2 as usize))
                                        - 1)
                                        * 2
                                        + 1)
                                } else {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + 0
                                }) < 0
                                {
                                    (base
                                        < -(if ((if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) - 1)
                                            < 0
                                        {
                                            ((((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 1)
                                                << (::core::mem::size_of::<::core::ffi::c_int>()
                                                    as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                                - 1)
                                                * 2
                                                + 1
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) - 1
                                        }))
                                        as ::core::ffi::c_int
                                } else {
                                    ((0) < base) as ::core::ffi::c_int
                                }) != 0
                                {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + 127
                                        >> (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                            .wrapping_mul(CHAR_BIT as usize)
                                            .wrapping_sub(1 as usize)
                                } else {
                                    127 / -base
                                }) as uintmax_t
                                    <= (-(1 as ::core::ffi::c_int) as uintmax_t).wrapping_sub(val))
                                    as ::core::ffi::c_int
                            }
                        } else {
                            if (if (if ((if 1 != 0 {
                                0
                            } else {
                                (if 1 != 0 {
                                    0
                                } else {
                                    base
                                }) + (-(127 as ::core::ffi::c_int) - 1)
                            }) - 1)
                                < 0
                            {
                                !(((((if 1 != 0 {
                                    0
                                } else {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + (-(127 as ::core::ffi::c_int) - 1)
                                }) + 1)
                                    << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                        .wrapping_mul(8 as usize)
                                        .wrapping_sub(2 as usize))
                                    - 1)
                                    * 2
                                    + 1)
                            } else {
                                (if 1 != 0 {
                                    0
                                } else {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + (-(127 as ::core::ffi::c_int) - 1)
                                }) + 0
                            }) < 0
                            {
                                ((if 1 != 0 {
                                    0
                                } else {
                                    base
                                }) + (-(127 as ::core::ffi::c_int) - 1)
                                    < -(if ((if 1 != 0 {
                                        0
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + (-(127 as ::core::ffi::c_int)
                                            - 1)
                                    }) - 1)
                                        < 0
                                    {
                                        ((((if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + (-(127 as ::core::ffi::c_int)
                                                - 1)
                                        }) + 1)
                                            << (::core::mem::size_of::<::core::ffi::c_int>()
                                                as usize)
                                                .wrapping_mul(8 as usize)
                                                .wrapping_sub(2 as usize))
                                            - 1)
                                            * 2
                                            + 1
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + (-(127 as ::core::ffi::c_int)
                                                - 1)
                                        }) - 1
                                    })) as ::core::ffi::c_int
                            } else {
                                ((0)
                                    < (if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + (-(127 as ::core::ffi::c_int) - 1))
                                    as ::core::ffi::c_int
                            }) != 0
                                && base == -(1 as ::core::ffi::c_int)
                            {
                                if (if 1 != 0 {
                                    0 as uintmax_t
                                } else {
                                    val
                                })
                                .wrapping_sub(1 as uintmax_t)
                                    < 0 as uintmax_t
                                {
                                    ((0 as uintmax_t)
                                        < val.wrapping_add(
                                            (-(127 as ::core::ffi::c_int) - 1)
                                                as uintmax_t,
                                        )) as ::core::ffi::c_int
                                } else {
                                    ((0 as uintmax_t) < val
                                        && ((-(1 as ::core::ffi::c_int)
                                            - (-(127 as ::core::ffi::c_int)
                                                - 1))
                                            as uintmax_t)
                                            < val.wrapping_sub(1 as uintmax_t))
                                        as ::core::ffi::c_int
                                }
                            } else {
                                ((((-(127 as ::core::ffi::c_int) - 1) / base)
                                    as uintmax_t)
                                    < val) as ::core::ffi::c_int
                            }
                        }
                    } else {
                        if base == 0 {
                            0
                        } else {
                            if val < 0 as uintmax_t {
                                if (if (if (if 1 != 0 {
                                    0 as uintmax_t
                                } else {
                                    (if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        val
                                    })
                                    .wrapping_add(
                                        (-(127 as ::core::ffi::c_int) - 1)
                                            as uintmax_t,
                                    )
                                })
                                .wrapping_sub(1 as uintmax_t)
                                    < 0 as uintmax_t
                                {
                                    !((if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_add(
                                            (-(127 as ::core::ffi::c_int) - 1)
                                                as uintmax_t,
                                        )
                                    })
                                    .wrapping_add(1 as uintmax_t)
                                        << (::core::mem::size_of::<uintmax_t>() as usize)
                                            .wrapping_mul(8 as usize)
                                            .wrapping_sub(2 as usize))
                                    .wrapping_sub(1 as uintmax_t)
                                    .wrapping_mul(2 as uintmax_t)
                                    .wrapping_add(1 as uintmax_t)
                                } else {
                                    (if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_add(
                                            (-(127 as ::core::ffi::c_int) - 1)
                                                as uintmax_t,
                                        )
                                    })
                                    .wrapping_add(0 as uintmax_t)
                                }) < 0 as uintmax_t
                                {
                                    ((if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        val
                                    })
                                    .wrapping_add(
                                        (-(127 as ::core::ffi::c_int) - 1)
                                            as uintmax_t,
                                    ) < (if (if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_add(
                                            (-(127 as ::core::ffi::c_int) - 1)
                                                as uintmax_t,
                                        )
                                    })
                                    .wrapping_sub(1 as uintmax_t)
                                        < 0 as uintmax_t
                                    {
                                        ((if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(
                                                (-(127 as ::core::ffi::c_int)
                                                    - 1)
                                                    as uintmax_t,
                                            )
                                        })
                                        .wrapping_add(1 as uintmax_t)
                                            << (::core::mem::size_of::<uintmax_t>() as usize)
                                                .wrapping_mul(8 as usize)
                                                .wrapping_sub(2 as usize))
                                        .wrapping_sub(1 as uintmax_t)
                                        .wrapping_mul(2 as uintmax_t)
                                        .wrapping_add(1 as uintmax_t)
                                    } else {
                                        (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(
                                                (-(127 as ::core::ffi::c_int)
                                                    - 1)
                                                    as uintmax_t,
                                            )
                                        })
                                        .wrapping_sub(1 as uintmax_t)
                                    })
                                    .wrapping_neg())
                                        as ::core::ffi::c_int
                                } else {
                                    ((0 as uintmax_t)
                                        < (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_add(
                                            (-(127 as ::core::ffi::c_int) - 1)
                                                as uintmax_t,
                                        )) as ::core::ffi::c_int
                                }) != 0
                                    && val == -(1 as ::core::ffi::c_int) as uintmax_t
                                {
                                    if ((if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) - 1)
                                        < 0
                                    {
                                        ((0)
                                            < base
                                                + (-(127 as ::core::ffi::c_int)
                                                    - 1))
                                            as ::core::ffi::c_int
                                    } else {
                                        (-(1 as ::core::ffi::c_int)
                                            - (-(127 as ::core::ffi::c_int)
                                                - 1)
                                            < base - 1)
                                            as ::core::ffi::c_int
                                    }
                                } else {
                                    (((-(127 as ::core::ffi::c_int) - 1)
                                        as uintmax_t)
                                        .wrapping_div(val)
                                        < base as uintmax_t)
                                        as ::core::ffi::c_int
                                }
                            } else {
                                (((127 / base) as uintmax_t) < val)
                                    as ::core::ffi::c_int
                            }
                        }
                    }) != 0
                    {
                        val = (val as ::core::ffi::c_uint).wrapping_mul(base as ::core::ffi::c_uint)
                            as ::core::ffi::c_schar as uintmax_t;
                        1
                    } else {
                        val = (val as ::core::ffi::c_uint).wrapping_mul(base as ::core::ffi::c_uint)
                            as ::core::ffi::c_schar as uintmax_t;
                        0
                    }
                } else {
                    if (if base < 0 {
                        if val < 0 as uintmax_t {
                            if ((if 1 != 0 {
                                0
                            } else {
                                (if 1 != 0 {
                                    0
                                } else {
                                    127 * 2
                                        + 1
                                }) + base
                            }) - 1)
                                < 0
                            {
                                (val < ((127 * 2
                                    + 1)
                                    / base) as uintmax_t)
                                    as ::core::ffi::c_int
                            } else {
                                ((if (if (if ((if 1 != 0 {
                                    0
                                } else {
                                    base
                                }) - 1)
                                    < 0
                                {
                                    !(((((if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + 1)
                                        << (::core::mem::size_of::<::core::ffi::c_int>()
                                            as usize)
                                            .wrapping_mul(8 as usize)
                                            .wrapping_sub(2 as usize))
                                        - 1)
                                        * 2
                                        + 1)
                                } else {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + 0
                                }) < 0
                                {
                                    (base
                                        < -(if ((if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) - 1)
                                            < 0
                                        {
                                            ((((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 1)
                                                << (::core::mem::size_of::<::core::ffi::c_int>()
                                                    as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                                - 1)
                                                * 2
                                                + 1
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) - 1
                                        }))
                                        as ::core::ffi::c_int
                                } else {
                                    ((0) < base) as ::core::ffi::c_int
                                }) != 0
                                {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + (127 * 2
                                        + 1)
                                        >> (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                            .wrapping_mul(CHAR_BIT as usize)
                                            .wrapping_sub(1 as usize)
                                } else {
                                    (127 * 2
                                        + 1)
                                        / -base
                                }) as uintmax_t
                                    <= (-(1 as ::core::ffi::c_int) as uintmax_t).wrapping_sub(val))
                                    as ::core::ffi::c_int
                            }
                        } else {
                            if (if (if ((if 1 != 0 {
                                0
                            } else {
                                (if 1 != 0 {
                                    0
                                } else {
                                    base
                                }) + 0
                            }) - 1)
                                < 0
                            {
                                !(((((if 1 != 0 {
                                    0
                                } else {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + 0
                                }) + 1)
                                    << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                        .wrapping_mul(8 as usize)
                                        .wrapping_sub(2 as usize))
                                    - 1)
                                    * 2
                                    + 1)
                            } else {
                                (if 1 != 0 {
                                    0
                                } else {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + 0
                                }) + 0
                            }) < 0
                            {
                                (((if 1 != 0 {
                                    0
                                } else {
                                    base
                                }) + 0)
                                    < -(if ((if 1 != 0 {
                                        0
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + 0
                                    }) - 1)
                                        < 0
                                    {
                                        ((((if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 0
                                        }) + 1)
                                            << (::core::mem::size_of::<::core::ffi::c_int>()
                                                as usize)
                                                .wrapping_mul(8 as usize)
                                                .wrapping_sub(2 as usize))
                                            - 1)
                                            * 2
                                            + 1
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 0
                                        }) - 1
                                    })) as ::core::ffi::c_int
                            } else {
                                ((0)
                                    < (if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + 0)
                                    as ::core::ffi::c_int
                            }) != 0
                                && base == -(1 as ::core::ffi::c_int)
                            {
                                if (if 1 != 0 {
                                    0 as uintmax_t
                                } else {
                                    val
                                })
                                .wrapping_sub(1 as uintmax_t)
                                    < 0 as uintmax_t
                                {
                                    ((0 as uintmax_t) < val.wrapping_add(0 as uintmax_t))
                                        as ::core::ffi::c_int
                                } else {
                                    ((0 as uintmax_t) < val
                                        && ((-(1 as ::core::ffi::c_int) - 0)
                                            as uintmax_t)
                                            < val.wrapping_sub(1 as uintmax_t))
                                        as ::core::ffi::c_int
                                }
                            } else {
                                (((0 / base) as uintmax_t) < val)
                                    as ::core::ffi::c_int
                            }
                        }
                    } else {
                        if base == 0 {
                            0
                        } else {
                            if val < 0 as uintmax_t {
                                if (if (if (if 1 != 0 {
                                    0 as uintmax_t
                                } else {
                                    (if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        val
                                    })
                                    .wrapping_add(0 as uintmax_t)
                                })
                                .wrapping_sub(1 as uintmax_t)
                                    < 0 as uintmax_t
                                {
                                    !((if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_add(0 as uintmax_t)
                                    })
                                    .wrapping_add(1 as uintmax_t)
                                        << (::core::mem::size_of::<uintmax_t>() as usize)
                                            .wrapping_mul(8 as usize)
                                            .wrapping_sub(2 as usize))
                                    .wrapping_sub(1 as uintmax_t)
                                    .wrapping_mul(2 as uintmax_t)
                                    .wrapping_add(1 as uintmax_t)
                                } else {
                                    (if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_add(0 as uintmax_t)
                                    })
                                    .wrapping_add(0 as uintmax_t)
                                }) < 0 as uintmax_t
                                {
                                    ((if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        val
                                    })
                                    .wrapping_add(0 as uintmax_t)
                                        < (if (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(0 as uintmax_t)
                                        })
                                        .wrapping_sub(1 as uintmax_t)
                                            < 0 as uintmax_t
                                        {
                                            ((if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(0 as uintmax_t)
                                            })
                                            .wrapping_add(1 as uintmax_t)
                                                << (::core::mem::size_of::<uintmax_t>() as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                            .wrapping_sub(1 as uintmax_t)
                                            .wrapping_mul(2 as uintmax_t)
                                            .wrapping_add(1 as uintmax_t)
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(0 as uintmax_t)
                                            })
                                            .wrapping_sub(1 as uintmax_t)
                                        })
                                        .wrapping_neg())
                                        as ::core::ffi::c_int
                                } else {
                                    ((0 as uintmax_t)
                                        < (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_add(0 as uintmax_t))
                                        as ::core::ffi::c_int
                                }) != 0
                                    && val == -(1 as ::core::ffi::c_int) as uintmax_t
                                {
                                    if ((if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) - 1)
                                        < 0
                                    {
                                        ((0) < base + 0)
                                            as ::core::ffi::c_int
                                    } else {
                                        ((-(1 as ::core::ffi::c_int) - 0)
                                            < base - 1)
                                            as ::core::ffi::c_int
                                    }
                                } else {
                                    ((0 as uintmax_t).wrapping_div(val) < base as uintmax_t)
                                        as ::core::ffi::c_int
                                }
                            } else {
                                ((((127 * 2
                                    + 1)
                                    / base) as uintmax_t)
                                    < val) as ::core::ffi::c_int
                            }
                        }
                    }) != 0
                    {
                        val = (val as ::core::ffi::c_uint).wrapping_mul(base as ::core::ffi::c_uint)
                            as ::core::ffi::c_uchar as uintmax_t;
                        1
                    } else {
                        val = (val as ::core::ffi::c_uint).wrapping_mul(base as ::core::ffi::c_uint)
                            as ::core::ffi::c_uchar as uintmax_t;
                        0
                    }
                }
            } else {
                if ::core::mem::size_of::<uintmax_t>() as usize
                    == ::core::mem::size_of::<::core::ffi::c_short>() as usize
                {
                    if !((0 as ::core::ffi::c_int as uintmax_t)
                        < -(1 as ::core::ffi::c_int) as uintmax_t)
                    {
                        if (if base < 0 {
                            if val < 0 as uintmax_t {
                                if ((if 1 != 0 {
                                    0
                                } else {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        32767 as ::core::ffi::c_int
                                    }) + base
                                }) - 1)
                                    < 0
                                {
                                    (val < (32767 as ::core::ffi::c_int / base) as uintmax_t)
                                        as ::core::ffi::c_int
                                } else {
                                    ((if (if (if ((if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) - 1)
                                        < 0
                                    {
                                        !(((((if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + 1)
                                            << (::core::mem::size_of::<::core::ffi::c_int>()
                                                as usize)
                                                .wrapping_mul(8 as usize)
                                                .wrapping_sub(2 as usize))
                                            - 1)
                                            * 2
                                            + 1)
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + 0
                                    }) < 0
                                    {
                                        (base
                                            < -(if ((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) - 1)
                                                < 0
                                            {
                                                ((((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 1)
                                                    << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize)) - 1)
                                                    * 2 + 1
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) - 1
                                            }))
                                            as ::core::ffi::c_int
                                    } else {
                                        ((0) < base) as ::core::ffi::c_int
                                    }) != 0
                                    {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + 32767 as ::core::ffi::c_int
                                            >> (::core::mem::size_of::<::core::ffi::c_int>()
                                                as usize)
                                                .wrapping_mul(CHAR_BIT as usize)
                                                .wrapping_sub(1 as usize)
                                    } else {
                                        32767 as ::core::ffi::c_int / -base
                                    }) as uintmax_t
                                        <= (-(1 as ::core::ffi::c_int) as uintmax_t)
                                            .wrapping_sub(val))
                                        as ::core::ffi::c_int
                                }
                            } else {
                                if (if (if ((if 1 != 0 {
                                    0
                                } else {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + (-(32767 as ::core::ffi::c_int) - 1)
                                }) - 1)
                                    < 0
                                {
                                    !(((((if 1 != 0 {
                                        0
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + (-(32767 as ::core::ffi::c_int)
                                            - 1)
                                    }) + 1)
                                        << (::core::mem::size_of::<::core::ffi::c_int>()
                                            as usize)
                                            .wrapping_mul(8 as usize)
                                            .wrapping_sub(2 as usize))
                                        - 1)
                                        * 2
                                        + 1)
                                } else {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + (-(32767 as ::core::ffi::c_int)
                                            - 1)
                                    }) + 0
                                }) < 0
                                {
                                    ((if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + (-(32767 as ::core::ffi::c_int)
                                        - 1)
                                        < -(if ((if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + (-(32767 as ::core::ffi::c_int)
                                                - 1)
                                        }) - 1)
                                            < 0
                                        {
                                            ((((if 1 != 0 {
                                                0
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + (-(32767 as ::core::ffi::c_int)
                                                    - 1)
                                            }) + 1)
                                                << (::core::mem::size_of::<::core::ffi::c_int>()
                                                    as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                                - 1)
                                                * 2
                                                + 1
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + (-(32767 as ::core::ffi::c_int)
                                                    - 1)
                                            }) - 1
                                        }))
                                        as ::core::ffi::c_int
                                } else {
                                    ((0)
                                        < (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + (-(32767 as ::core::ffi::c_int)
                                            - 1))
                                        as ::core::ffi::c_int
                                }) != 0
                                    && base == -(1 as ::core::ffi::c_int)
                                {
                                    if (if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        val
                                    })
                                    .wrapping_sub(1 as uintmax_t)
                                        < 0 as uintmax_t
                                    {
                                        ((0 as uintmax_t)
                                            < val.wrapping_add(
                                                (-(32767 as ::core::ffi::c_int)
                                                    - 1)
                                                    as uintmax_t,
                                            ))
                                            as ::core::ffi::c_int
                                    } else {
                                        ((0 as uintmax_t) < val
                                            && ((-(1 as ::core::ffi::c_int)
                                                - (-(32767 as ::core::ffi::c_int)
                                                    - 1))
                                                as uintmax_t)
                                                < val.wrapping_sub(1 as uintmax_t))
                                            as ::core::ffi::c_int
                                    }
                                } else {
                                    ((((-(32767 as ::core::ffi::c_int) - 1)
                                        / base) as uintmax_t)
                                        < val)
                                        as ::core::ffi::c_int
                                }
                            }
                        } else {
                            if base == 0 {
                                0
                            } else {
                                if val < 0 as uintmax_t {
                                    if (if (if (if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_add(
                                            (-(32767 as ::core::ffi::c_int)
                                                - 1)
                                                as uintmax_t,
                                        )
                                    })
                                    .wrapping_sub(1 as uintmax_t)
                                        < 0 as uintmax_t
                                    {
                                        !((if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(
                                                (-(32767 as ::core::ffi::c_int)
                                                    - 1)
                                                    as uintmax_t,
                                            )
                                        })
                                        .wrapping_add(1 as uintmax_t)
                                            << (::core::mem::size_of::<uintmax_t>() as usize)
                                                .wrapping_mul(8 as usize)
                                                .wrapping_sub(2 as usize))
                                        .wrapping_sub(1 as uintmax_t)
                                        .wrapping_mul(2 as uintmax_t)
                                        .wrapping_add(1 as uintmax_t)
                                    } else {
                                        (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(
                                                (-(32767 as ::core::ffi::c_int)
                                                    - 1)
                                                    as uintmax_t,
                                            )
                                        })
                                        .wrapping_add(0 as uintmax_t)
                                    }) < 0 as uintmax_t
                                    {
                                        ((if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_add(
                                            (-(32767 as ::core::ffi::c_int)
                                                - 1)
                                                as uintmax_t,
                                        ) < (if (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(
                                                (-(32767 as ::core::ffi::c_int)
                                                    - 1)
                                                    as uintmax_t,
                                            )
                                        })
                                        .wrapping_sub(1 as uintmax_t)
                                            < 0 as uintmax_t
                                        {
                                            ((if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(
                                                    (-(32767 as ::core::ffi::c_int)
                                                        - 1)
                                                        as uintmax_t,
                                                )
                                            })
                                            .wrapping_add(1 as uintmax_t)
                                                << (::core::mem::size_of::<uintmax_t>() as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                            .wrapping_sub(1 as uintmax_t)
                                            .wrapping_mul(2 as uintmax_t)
                                            .wrapping_add(1 as uintmax_t)
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(
                                                    (-(32767 as ::core::ffi::c_int)
                                                        - 1)
                                                        as uintmax_t,
                                                )
                                            })
                                            .wrapping_sub(1 as uintmax_t)
                                        })
                                        .wrapping_neg())
                                            as ::core::ffi::c_int
                                    } else {
                                        ((0 as uintmax_t)
                                            < (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(
                                                (-(32767 as ::core::ffi::c_int)
                                                    - 1)
                                                    as uintmax_t,
                                            ))
                                            as ::core::ffi::c_int
                                    }) != 0
                                        && val == -(1 as ::core::ffi::c_int) as uintmax_t
                                    {
                                        if ((if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) - 1)
                                            < 0
                                        {
                                            ((0)
                                                < base
                                                    + (-(32767 as ::core::ffi::c_int)
                                                        - 1))
                                                as ::core::ffi::c_int
                                        } else {
                                            (-(1 as ::core::ffi::c_int)
                                                - (-(32767 as ::core::ffi::c_int)
                                                    - 1)
                                                < base - 1)
                                                as ::core::ffi::c_int
                                        }
                                    } else {
                                        (((-(32767 as ::core::ffi::c_int) - 1)
                                            as uintmax_t)
                                            .wrapping_div(val)
                                            < base as uintmax_t)
                                            as ::core::ffi::c_int
                                    }
                                } else {
                                    (((32767 as ::core::ffi::c_int / base) as uintmax_t) < val)
                                        as ::core::ffi::c_int
                                }
                            }
                        }) != 0
                        {
                            val = (val as ::core::ffi::c_uint)
                                .wrapping_mul(base as ::core::ffi::c_uint)
                                as ::core::ffi::c_short
                                as uintmax_t;
                            1
                        } else {
                            val = (val as ::core::ffi::c_uint)
                                .wrapping_mul(base as ::core::ffi::c_uint)
                                as ::core::ffi::c_short
                                as uintmax_t;
                            0
                        }
                    } else {
                        if (if base < 0 {
                            if val < 0 as uintmax_t {
                                if ((if 1 != 0 {
                                    0
                                } else {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        32767 as ::core::ffi::c_int * 2
                                            + 1
                                    }) + base
                                }) - 1)
                                    < 0
                                {
                                    (val < ((32767 as ::core::ffi::c_int * 2
                                        + 1)
                                        / base)
                                        as uintmax_t)
                                        as ::core::ffi::c_int
                                } else {
                                    ((if (if (if ((if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) - 1)
                                        < 0
                                    {
                                        !(((((if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + 1)
                                            << (::core::mem::size_of::<::core::ffi::c_int>()
                                                as usize)
                                                .wrapping_mul(8 as usize)
                                                .wrapping_sub(2 as usize))
                                            - 1)
                                            * 2
                                            + 1)
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + 0
                                    }) < 0
                                    {
                                        (base
                                            < -(if ((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) - 1)
                                                < 0
                                            {
                                                ((((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 1)
                                                    << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize)) - 1)
                                                    * 2 + 1
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) - 1
                                            }))
                                            as ::core::ffi::c_int
                                    } else {
                                        ((0) < base) as ::core::ffi::c_int
                                    }) != 0
                                    {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + (32767 as ::core::ffi::c_int * 2
                                            + 1)
                                            >> (::core::mem::size_of::<::core::ffi::c_int>()
                                                as usize)
                                                .wrapping_mul(CHAR_BIT as usize)
                                                .wrapping_sub(1 as usize)
                                    } else {
                                        (32767 as ::core::ffi::c_int * 2
                                            + 1)
                                            / -base
                                    }) as uintmax_t
                                        <= (-(1 as ::core::ffi::c_int) as uintmax_t)
                                            .wrapping_sub(val))
                                        as ::core::ffi::c_int
                                }
                            } else {
                                if (if (if ((if 1 != 0 {
                                    0
                                } else {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + 0
                                }) - 1)
                                    < 0
                                {
                                    !(((((if 1 != 0 {
                                        0
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + 0
                                    }) + 1)
                                        << (::core::mem::size_of::<::core::ffi::c_int>()
                                            as usize)
                                            .wrapping_mul(8 as usize)
                                            .wrapping_sub(2 as usize))
                                        - 1)
                                        * 2
                                        + 1)
                                } else {
                                    (if 1 != 0 {
                                        0
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + 0
                                    }) + 0
                                }) < 0
                                {
                                    (((if 1 != 0 {
                                        0
                                    } else {
                                        base
                                    }) + 0)
                                        < -(if ((if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 0
                                        }) - 1)
                                            < 0
                                        {
                                            ((((if 1 != 0 {
                                                0
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0
                                            }) + 1)
                                                << (::core::mem::size_of::<::core::ffi::c_int>()
                                                    as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                                - 1)
                                                * 2
                                                + 1
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0
                                            }) - 1
                                        }))
                                        as ::core::ffi::c_int
                                } else {
                                    ((0)
                                        < (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + 0)
                                        as ::core::ffi::c_int
                                }) != 0
                                    && base == -(1 as ::core::ffi::c_int)
                                {
                                    if (if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        val
                                    })
                                    .wrapping_sub(1 as uintmax_t)
                                        < 0 as uintmax_t
                                    {
                                        ((0 as uintmax_t) < val.wrapping_add(0 as uintmax_t))
                                            as ::core::ffi::c_int
                                    } else {
                                        ((0 as uintmax_t) < val
                                            && ((-(1 as ::core::ffi::c_int)
                                                - 0)
                                                as uintmax_t)
                                                < val.wrapping_sub(1 as uintmax_t))
                                            as ::core::ffi::c_int
                                    }
                                } else {
                                    (((0 / base) as uintmax_t) < val)
                                        as ::core::ffi::c_int
                                }
                            }
                        } else {
                            if base == 0 {
                                0
                            } else {
                                if val < 0 as uintmax_t {
                                    if (if (if (if 1 != 0 {
                                        0 as uintmax_t
                                    } else {
                                        (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_add(0 as uintmax_t)
                                    })
                                    .wrapping_sub(1 as uintmax_t)
                                        < 0 as uintmax_t
                                    {
                                        !((if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(0 as uintmax_t)
                                        })
                                        .wrapping_add(1 as uintmax_t)
                                            << (::core::mem::size_of::<uintmax_t>() as usize)
                                                .wrapping_mul(8 as usize)
                                                .wrapping_sub(2 as usize))
                                        .wrapping_sub(1 as uintmax_t)
                                        .wrapping_mul(2 as uintmax_t)
                                        .wrapping_add(1 as uintmax_t)
                                    } else {
                                        (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(0 as uintmax_t)
                                        })
                                        .wrapping_add(0 as uintmax_t)
                                    }) < 0 as uintmax_t
                                    {
                                        ((if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_add(0 as uintmax_t)
                                            < (if (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(0 as uintmax_t)
                                            })
                                            .wrapping_sub(1 as uintmax_t)
                                                < 0 as uintmax_t
                                            {
                                                ((if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(0 as uintmax_t)
                                                })
                                                .wrapping_add(1 as uintmax_t)
                                                    << (::core::mem::size_of::<uintmax_t>()
                                                        as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize))
                                                .wrapping_sub(1 as uintmax_t)
                                                .wrapping_mul(2 as uintmax_t)
                                                .wrapping_add(1 as uintmax_t)
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(0 as uintmax_t)
                                                })
                                                .wrapping_sub(1 as uintmax_t)
                                            })
                                            .wrapping_neg())
                                            as ::core::ffi::c_int
                                    } else {
                                        ((0 as uintmax_t)
                                            < (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(0 as uintmax_t))
                                            as ::core::ffi::c_int
                                    }) != 0
                                        && val == -(1 as ::core::ffi::c_int) as uintmax_t
                                    {
                                        if ((if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) - 1)
                                            < 0
                                        {
                                            ((0)
                                                < base + 0)
                                                as ::core::ffi::c_int
                                        } else {
                                            ((-(1 as ::core::ffi::c_int) - 0)
                                                < base - 1)
                                                as ::core::ffi::c_int
                                        }
                                    } else {
                                        ((0 as uintmax_t).wrapping_div(val) < base as uintmax_t)
                                            as ::core::ffi::c_int
                                    }
                                } else {
                                    ((((32767 as ::core::ffi::c_int * 2
                                        + 1)
                                        / base) as uintmax_t)
                                        < val)
                                        as ::core::ffi::c_int
                                }
                            }
                        }) != 0
                        {
                            val = (val as ::core::ffi::c_uint)
                                .wrapping_mul(base as ::core::ffi::c_uint)
                                as ::core::ffi::c_ushort
                                as uintmax_t;
                            1
                        } else {
                            val = (val as ::core::ffi::c_uint)
                                .wrapping_mul(base as ::core::ffi::c_uint)
                                as ::core::ffi::c_ushort
                                as uintmax_t;
                            0
                        }
                    }
                } else {
                    if ::core::mem::size_of::<uintmax_t>() as usize
                        == ::core::mem::size_of::<::core::ffi::c_int>() as usize
                    {
                        if (if 1 != 0 {
                            0 as uintmax_t
                        } else {
                            val
                        })
                        .wrapping_sub(1 as uintmax_t)
                            < 0 as uintmax_t
                        {
                            if (if base < 0 {
                                if val < 0 as uintmax_t {
                                    if ((if 1 != 0 {
                                        0
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            2147483647 as ::core::ffi::c_int
                                        }) + base
                                    }) - 1)
                                        < 0
                                    {
                                        (val < (2147483647 as ::core::ffi::c_int / base)
                                            as uintmax_t)
                                            as ::core::ffi::c_int
                                    } else {
                                        ((if (if (if ((if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) - 1)
                                            < 0
                                        {
                                            !(((((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 1)
                                                << (::core::mem::size_of::<::core::ffi::c_int>()
                                                    as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                                - 1)
                                                * 2
                                                + 1)
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 0
                                        }) < 0
                                        {
                                            (base
                                                < -(if ((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) - 1)
                                                    < 0
                                                {
                                                    ((((if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) + 1)
                                                        << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                            .wrapping_mul(8 as usize)
                                                            .wrapping_sub(2 as usize)) - 1)
                                                        * 2 + 1
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) - 1
                                                }))
                                                as ::core::ffi::c_int
                                        } else {
                                            ((0) < base) as ::core::ffi::c_int
                                        }) != 0
                                        {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 2147483647 as ::core::ffi::c_int
                                                >> (::core::mem::size_of::<::core::ffi::c_int>()
                                                    as usize)
                                                    .wrapping_mul(CHAR_BIT as usize)
                                                    .wrapping_sub(1 as usize)
                                        } else {
                                            2147483647 as ::core::ffi::c_int / -base
                                        }) as uintmax_t
                                            <= (-(1 as ::core::ffi::c_int) as uintmax_t)
                                                .wrapping_sub(val))
                                            as ::core::ffi::c_int
                                    }
                                } else {
                                    if (if (if ((if 1 != 0 {
                                        0
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + (-(2147483647 as ::core::ffi::c_int)
                                            - 1)
                                    }) - 1)
                                        < 0
                                    {
                                        !(((((if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + (-(2147483647 as ::core::ffi::c_int)
                                                - 1)
                                        }) + 1)
                                            << (::core::mem::size_of::<::core::ffi::c_int>()
                                                as usize)
                                                .wrapping_mul(8 as usize)
                                                .wrapping_sub(2 as usize))
                                            - 1)
                                            * 2
                                            + 1)
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + (-(2147483647 as ::core::ffi::c_int)
                                                - 1)
                                        }) + 0
                                    }) < 0
                                    {
                                        ((if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + (-(2147483647 as ::core::ffi::c_int)
                                            - 1)
                                            < -(if ((if 1 != 0 {
                                                0
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + (-(2147483647 as ::core::ffi::c_int)
                                                    - 1)
                                            }) - 1)
                                                < 0
                                            {
                                                ((((if 1 != 0 {
                                                    0
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    })
                                                        + (-(2147483647 as ::core::ffi::c_int)
                                                            - 1)
                                                }) + 1)
                                                    << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize)) - 1)
                                                    * 2 + 1
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) + (-(2147483647 as ::core::ffi::c_int)
                                                        - 1)
                                                }) - 1
                                            }))
                                            as ::core::ffi::c_int
                                    } else {
                                        ((0)
                                            < (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + (-(2147483647 as ::core::ffi::c_int)
                                                - 1))
                                            as ::core::ffi::c_int
                                    }) != 0
                                        && base == -(1 as ::core::ffi::c_int)
                                    {
                                        if (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_sub(1 as uintmax_t)
                                            < 0 as uintmax_t
                                        {
                                            ((0 as uintmax_t)
                                                < val.wrapping_add(
                                                    (-(2147483647 as ::core::ffi::c_int)
                                                        - 1)
                                                        as uintmax_t,
                                                ))
                                                as ::core::ffi::c_int
                                        } else {
                                            ((0 as uintmax_t) < val
                                                && ((-(1 as ::core::ffi::c_int)
                                                    - (-(2147483647 as ::core::ffi::c_int)
                                                        - 1))
                                                    as uintmax_t)
                                                    < val.wrapping_sub(1 as uintmax_t))
                                                as ::core::ffi::c_int
                                        }
                                    } else {
                                        ((((-(2147483647 as ::core::ffi::c_int)
                                            - 1)
                                            / base)
                                            as uintmax_t)
                                            < val)
                                            as ::core::ffi::c_int
                                    }
                                }
                            } else {
                                if base == 0 {
                                    0
                                } else {
                                    if val < 0 as uintmax_t {
                                        if (if (if (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(
                                                (-(2147483647 as ::core::ffi::c_int)
                                                    - 1)
                                                    as uintmax_t,
                                            )
                                        })
                                        .wrapping_sub(1 as uintmax_t)
                                            < 0 as uintmax_t
                                        {
                                            !((if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(
                                                    (-(2147483647 as ::core::ffi::c_int)
                                                        - 1)
                                                        as uintmax_t,
                                                )
                                            })
                                            .wrapping_add(1 as uintmax_t)
                                                << (::core::mem::size_of::<uintmax_t>() as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                            .wrapping_sub(1 as uintmax_t)
                                            .wrapping_mul(2 as uintmax_t)
                                            .wrapping_add(1 as uintmax_t)
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(
                                                    (-(2147483647 as ::core::ffi::c_int)
                                                        - 1)
                                                        as uintmax_t,
                                                )
                                            })
                                            .wrapping_add(0 as uintmax_t)
                                        }) < 0 as uintmax_t
                                        {
                                            ((if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(
                                                (-(2147483647 as ::core::ffi::c_int)
                                                    - 1)
                                                    as uintmax_t,
                                            ) < (if (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(
                                                    (-(2147483647 as ::core::ffi::c_int)
                                                        - 1)
                                                        as uintmax_t,
                                                )
                                            })
                                            .wrapping_sub(1 as uintmax_t)
                                                < 0 as uintmax_t
                                            {
                                                ((if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(
                                                        (-(2147483647 as ::core::ffi::c_int)
                                                            - 1)
                                                            as uintmax_t,
                                                    )
                                                })
                                                .wrapping_add(1 as uintmax_t)
                                                    << (::core::mem::size_of::<uintmax_t>()
                                                        as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize))
                                                .wrapping_sub(1 as uintmax_t)
                                                .wrapping_mul(2 as uintmax_t)
                                                .wrapping_add(1 as uintmax_t)
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(
                                                        (-(2147483647 as ::core::ffi::c_int)
                                                            - 1)
                                                            as uintmax_t,
                                                    )
                                                })
                                                .wrapping_sub(1 as uintmax_t)
                                            })
                                            .wrapping_neg())
                                                as ::core::ffi::c_int
                                        } else {
                                            ((0 as uintmax_t)
                                                < (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(
                                                    (-(2147483647 as ::core::ffi::c_int)
                                                        - 1)
                                                        as uintmax_t,
                                                ))
                                                as ::core::ffi::c_int
                                        }) != 0
                                            && val == -(1 as ::core::ffi::c_int) as uintmax_t
                                        {
                                            if ((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) - 1)
                                                < 0
                                            {
                                                ((0)
                                                    < base
                                                        + (-(2147483647 as ::core::ffi::c_int)
                                                            - 1))
                                                    as ::core::ffi::c_int
                                            } else {
                                                (-(1 as ::core::ffi::c_int)
                                                    - (-(2147483647 as ::core::ffi::c_int)
                                                        - 1)
                                                    < base - 1)
                                                    as ::core::ffi::c_int
                                            }
                                        } else {
                                            (((-(2147483647 as ::core::ffi::c_int)
                                                - 1)
                                                as uintmax_t)
                                                .wrapping_div(val)
                                                < base as uintmax_t)
                                                as ::core::ffi::c_int
                                        }
                                    } else {
                                        (((2147483647 as ::core::ffi::c_int / base) as uintmax_t)
                                            < val)
                                            as ::core::ffi::c_int
                                    }
                                }
                            }) != 0
                            {
                                val = (val as ::core::ffi::c_uint)
                                    .wrapping_mul(base as ::core::ffi::c_uint)
                                    as ::core::ffi::c_int
                                    as uintmax_t;
                                1
                            } else {
                                val = (val as ::core::ffi::c_uint)
                                    .wrapping_mul(base as ::core::ffi::c_uint)
                                    as ::core::ffi::c_int
                                    as uintmax_t;
                                0
                            }
                        } else {
                            if (if base < 0 {
                                if val < 0 as uintmax_t {
                                    if (if 1 != 0 {
                                        0
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            (2147483647 as ::core::ffi::c_int
                                                as ::core::ffi::c_uint)
                                                .wrapping_mul(2)
                                                .wrapping_add(1)
                                        })
                                        .wrapping_add(base as ::core::ffi::c_uint)
                                    })
                                    .wrapping_sub(1)
                                        < 0
                                    {
                                        (val < (2147483647 as ::core::ffi::c_int
                                            as ::core::ffi::c_uint)
                                            .wrapping_mul(2)
                                            .wrapping_add(1)
                                            .wrapping_div(base as ::core::ffi::c_uint)
                                            as uintmax_t)
                                            as ::core::ffi::c_int
                                    } else {
                                        ((if (if (if ((if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) - 1)
                                            < 0
                                        {
                                            !(((((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 1)
                                                << (::core::mem::size_of::<::core::ffi::c_int>()
                                                    as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                                - 1)
                                                * 2
                                                + 1)
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 0
                                        }) < 0
                                        {
                                            (base
                                                < -(if ((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) - 1)
                                                    < 0
                                                {
                                                    ((((if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) + 1)
                                                        << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                            .wrapping_mul(8 as usize)
                                                            .wrapping_sub(2 as usize)) - 1)
                                                        * 2 + 1
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) - 1
                                                }))
                                                as ::core::ffi::c_int
                                        } else {
                                            ((0) < base) as ::core::ffi::c_int
                                        }) != 0
                                        {
                                            ((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            })
                                                as ::core::ffi::c_uint)
                                                .wrapping_add(
                                                    (2147483647 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint)
                                                        .wrapping_mul(2)
                                                        .wrapping_add(1),
                                                )
                                                >> (::core::mem::size_of::<::core::ffi::c_int>()
                                                    as usize)
                                                    .wrapping_mul(CHAR_BIT as usize)
                                                    .wrapping_sub(1 as usize)
                                        } else {
                                            (2147483647 as ::core::ffi::c_int
                                                as ::core::ffi::c_uint)
                                                .wrapping_mul(2)
                                                .wrapping_add(1)
                                                .wrapping_div(-base as ::core::ffi::c_uint)
                                        }) as uintmax_t
                                            <= (-(1 as ::core::ffi::c_int) as uintmax_t)
                                                .wrapping_sub(val))
                                            as ::core::ffi::c_int
                                    }
                                } else {
                                    if (if (if ((if 1 != 0 {
                                        0
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + 0
                                    }) - 1)
                                        < 0
                                    {
                                        !(((((if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 0
                                        }) + 1)
                                            << (::core::mem::size_of::<::core::ffi::c_int>()
                                                as usize)
                                                .wrapping_mul(8 as usize)
                                                .wrapping_sub(2 as usize))
                                            - 1)
                                            * 2
                                            + 1)
                                    } else {
                                        (if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 0
                                        }) + 0
                                    }) < 0
                                    {
                                        (((if 1 != 0 {
                                            0
                                        } else {
                                            base
                                        }) + 0)
                                            < -(if ((if 1 != 0 {
                                                0
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0
                                            }) - 1)
                                                < 0
                                            {
                                                ((((if 1 != 0 {
                                                    0
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) + 0
                                                }) + 1)
                                                    << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize)) - 1)
                                                    * 2 + 1
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) + 0
                                                }) - 1
                                            }))
                                            as ::core::ffi::c_int
                                    } else {
                                        ((0)
                                            < (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 0)
                                            as ::core::ffi::c_int
                                    }) != 0
                                        && base == -(1 as ::core::ffi::c_int)
                                    {
                                        if (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            val
                                        })
                                        .wrapping_sub(1 as uintmax_t)
                                            < 0 as uintmax_t
                                        {
                                            ((0 as uintmax_t) < val.wrapping_add(0 as uintmax_t))
                                                as ::core::ffi::c_int
                                        } else {
                                            ((0 as uintmax_t) < val
                                                && ((-(1 as ::core::ffi::c_int)
                                                    - 0)
                                                    as uintmax_t)
                                                    < val.wrapping_sub(1 as uintmax_t))
                                                as ::core::ffi::c_int
                                        }
                                    } else {
                                        (((0 / base) as uintmax_t) < val)
                                            as ::core::ffi::c_int
                                    }
                                }
                            } else {
                                if base == 0 {
                                    0
                                } else {
                                    if val < 0 as uintmax_t {
                                        if (if (if (if 1 != 0 {
                                            0 as uintmax_t
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(0 as uintmax_t)
                                        })
                                        .wrapping_sub(1 as uintmax_t)
                                            < 0 as uintmax_t
                                        {
                                            !((if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(0 as uintmax_t)
                                            })
                                            .wrapping_add(1 as uintmax_t)
                                                << (::core::mem::size_of::<uintmax_t>() as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                            .wrapping_sub(1 as uintmax_t)
                                            .wrapping_mul(2 as uintmax_t)
                                            .wrapping_add(1 as uintmax_t)
                                        } else {
                                            (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(0 as uintmax_t)
                                            })
                                            .wrapping_add(0 as uintmax_t)
                                        }) < 0 as uintmax_t
                                        {
                                            ((if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_add(0 as uintmax_t)
                                                < (if (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(0 as uintmax_t)
                                                })
                                                .wrapping_sub(1 as uintmax_t)
                                                    < 0 as uintmax_t
                                                {
                                                    ((if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        (if 1 != 0 {
                                                            0 as uintmax_t
                                                        } else {
                                                            val
                                                        })
                                                        .wrapping_add(0 as uintmax_t)
                                                    })
                                                    .wrapping_add(1 as uintmax_t)
                                                        << (::core::mem::size_of::<uintmax_t>()
                                                            as usize)
                                                            .wrapping_mul(8 as usize)
                                                            .wrapping_sub(2 as usize))
                                                    .wrapping_sub(1 as uintmax_t)
                                                    .wrapping_mul(2 as uintmax_t)
                                                    .wrapping_add(1 as uintmax_t)
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        (if 1 != 0 {
                                                            0 as uintmax_t
                                                        } else {
                                                            val
                                                        })
                                                        .wrapping_add(0 as uintmax_t)
                                                    })
                                                    .wrapping_sub(1 as uintmax_t)
                                                })
                                                .wrapping_neg())
                                                as ::core::ffi::c_int
                                        } else {
                                            ((0 as uintmax_t)
                                                < (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(0 as uintmax_t))
                                                as ::core::ffi::c_int
                                        }) != 0
                                            && val == -(1 as ::core::ffi::c_int) as uintmax_t
                                        {
                                            if ((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) - 1)
                                                < 0
                                            {
                                                ((0)
                                                    < base + 0)
                                                    as ::core::ffi::c_int
                                            } else {
                                                ((-(1 as ::core::ffi::c_int)
                                                    - 0)
                                                    < base - 1)
                                                    as ::core::ffi::c_int
                                            }
                                        } else {
                                            ((0 as uintmax_t).wrapping_div(val) < base as uintmax_t)
                                                as ::core::ffi::c_int
                                        }
                                    } else {
                                        (((2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                            .wrapping_mul(2)
                                            .wrapping_add(1)
                                            .wrapping_div(base as ::core::ffi::c_uint)
                                            as uintmax_t)
                                            < val)
                                            as ::core::ffi::c_int
                                    }
                                }
                            }) != 0
                            {
                                val = (val as ::core::ffi::c_uint)
                                    .wrapping_mul(base as ::core::ffi::c_uint)
                                    as uintmax_t;
                                1
                            } else {
                                val = (val as ::core::ffi::c_uint)
                                    .wrapping_mul(base as ::core::ffi::c_uint)
                                    as uintmax_t;
                                0
                            }
                        }
                    } else {
                        if ::core::mem::size_of::<uintmax_t>() as usize
                            == ::core::mem::size_of::<::core::ffi::c_long>() as usize
                        {
                            if (if 1 != 0 {
                                0 as uintmax_t
                            } else {
                                val
                            })
                            .wrapping_sub(1 as uintmax_t)
                                < 0 as uintmax_t
                            {
                                if (if base < 0 {
                                    if val < 0 as uintmax_t {
                                        if ((if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                9223372036854775807 as ::core::ffi::c_long
                                            }) + base as ::core::ffi::c_long
                                        }) - 1)
                                            < 0
                                        {
                                            (val < (9223372036854775807 as ::core::ffi::c_long
                                                / base as ::core::ffi::c_long)
                                                as uintmax_t)
                                                as ::core::ffi::c_int
                                        } else {
                                            ((if (if (if ((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) - 1)
                                                < 0
                                            {
                                                !(((((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 1)
                                                    << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize)) - 1)
                                                    * 2 + 1)
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0
                                            }) < 0
                                            {
                                                (base
                                                    < -(if ((if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) - 1)
                                                        < 0
                                                    {
                                                        ((((if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        }) + 1)
                                                            << (::core::mem::size_of::<
                                                                ::core::ffi::c_int,
                                                            >(
                                                            )
                                                                as usize)
                                                                .wrapping_mul(8 as usize)
                                                                .wrapping_sub(2 as usize))
                                                            - 1)
                                                            * 2
                                                            + 1
                                                    } else {
                                                        (if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        }) - 1
                                                    }))
                                                    as ::core::ffi::c_int
                                            } else {
                                                ((0) < base)
                                                    as ::core::ffi::c_int
                                            }) != 0
                                            {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                })
                                                    as ::core::ffi::c_long
                                                    + 9223372036854775807 as ::core::ffi::c_long
                                                    >> (::core::mem::size_of::<::core::ffi::c_int>()
                                                        as usize)
                                                        .wrapping_mul(CHAR_BIT as usize)
                                                        .wrapping_sub(1 as usize)
                                            } else {
                                                9223372036854775807 as ::core::ffi::c_long
                                                    / -base as ::core::ffi::c_long
                                            })
                                                as uintmax_t
                                                <= (-(1 as ::core::ffi::c_int) as uintmax_t)
                                                    .wrapping_sub(val))
                                                as ::core::ffi::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            })
                                                as ::core::ffi::c_long
                                                + (-(9223372036854775807 as ::core::ffi::c_long)
                                                    - 1)
                                        }) - 1)
                                            < 0
                                        {
                                            !(((((if 1 != 0 {
                                                0
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                })
                                                    as ::core::ffi::c_long
                                                    + (-(9223372036854775807
                                                        as ::core::ffi::c_long)
                                                        - 1)
                                            }) + 1)
                                                << (::core::mem::size_of::<::core::ffi::c_long>()
                                                    as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                                - 1)
                                                * 2
                                                + 1)
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                })
                                                    as ::core::ffi::c_long
                                                    + (-(9223372036854775807
                                                        as ::core::ffi::c_long)
                                                        - 1)
                                            }) + 0
                                        }) < 0
                                        {
                                            ((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            })
                                                as ::core::ffi::c_long
                                                + (-(9223372036854775807 as ::core::ffi::c_long)
                                                    - 1)
                                                < -(if ((if 1 != 0 {
                                                    0
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    })
                                                        as ::core::ffi::c_long
                                                        + (-(9223372036854775807
                                                            as ::core::ffi::c_long)
                                                            - 1)
                                                }) - 1)
                                                    < 0
                                                {
                                                    ((((if 1 != 0 {
                                                        0
                                                    } else {
                                                        (if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        })
                                                            as ::core::ffi::c_long
                                                            + (-(9223372036854775807
                                                                as ::core::ffi::c_long)
                                                                - 1)
                                                    }) + 1)
                                                        << (::core::mem::size_of::<
                                                            ::core::ffi::c_long,
                                                        >(
                                                        )
                                                            as usize)
                                                            .wrapping_mul(8 as usize)
                                                            .wrapping_sub(2 as usize))
                                                        - 1)
                                                        * 2
                                                        + 1
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        (if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        })
                                                            as ::core::ffi::c_long
                                                            + (-(9223372036854775807
                                                                as ::core::ffi::c_long)
                                                                - 1)
                                                    }) - 1
                                                }))
                                                as ::core::ffi::c_int
                                        } else {
                                            ((0)
                                                < (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                })
                                                    as ::core::ffi::c_long
                                                    + (-(9223372036854775807
                                                        as ::core::ffi::c_long)
                                                        - 1))
                                                as ::core::ffi::c_int
                                        }) != 0
                                            && base == -(1 as ::core::ffi::c_int)
                                        {
                                            if (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_sub(1 as uintmax_t)
                                                < 0 as uintmax_t
                                            {
                                                ((0 as uintmax_t)
                                                    < val.wrapping_add(
                                                        (-(9223372036854775807
                                                            as ::core::ffi::c_long)
                                                            - 1)
                                                            as uintmax_t,
                                                    ))
                                                    as ::core::ffi::c_int
                                            } else {
                                                ((0 as uintmax_t) < val
                                                    && ((-(1 as ::core::ffi::c_int)
                                                        as ::core::ffi::c_long
                                                        - (-(9223372036854775807
                                                            as ::core::ffi::c_long)
                                                            - 1))
                                                        as uintmax_t)
                                                        < val.wrapping_sub(1 as uintmax_t))
                                                    as ::core::ffi::c_int
                                            }
                                        } else {
                                            ((((-(9223372036854775807 as ::core::ffi::c_long)
                                                - 1)
                                                / base as ::core::ffi::c_long)
                                                as uintmax_t)
                                                < val)
                                                as ::core::ffi::c_int
                                        }
                                    }
                                } else {
                                    if base == 0 {
                                        0
                                    } else {
                                        if val < 0 as uintmax_t {
                                            if (if (if (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(
                                                    (-(9223372036854775807 as ::core::ffi::c_long)
                                                        - 1)
                                                        as uintmax_t,
                                                )
                                            })
                                            .wrapping_sub(1 as uintmax_t)
                                                < 0 as uintmax_t
                                            {
                                                !((if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(
                                                        (-(9223372036854775807
                                                            as ::core::ffi::c_long)
                                                            - 1)
                                                            as uintmax_t,
                                                    )
                                                })
                                                .wrapping_add(1 as uintmax_t)
                                                    << (::core::mem::size_of::<uintmax_t>()
                                                        as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize))
                                                .wrapping_sub(1 as uintmax_t)
                                                .wrapping_mul(2 as uintmax_t)
                                                .wrapping_add(1 as uintmax_t)
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(
                                                        (-(9223372036854775807
                                                            as ::core::ffi::c_long)
                                                            - 1)
                                                            as uintmax_t,
                                                    )
                                                })
                                                .wrapping_add(0 as uintmax_t)
                                            }) < 0 as uintmax_t
                                            {
                                                ((if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(
                                                    (-(9223372036854775807 as ::core::ffi::c_long)
                                                        - 1)
                                                        as uintmax_t,
                                                ) < (if (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(
                                                        (-(9223372036854775807
                                                            as ::core::ffi::c_long)
                                                            - 1)
                                                            as uintmax_t,
                                                    )
                                                })
                                                .wrapping_sub(1 as uintmax_t)
                                                    < 0 as uintmax_t
                                                {
                                                    ((if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        (if 1 != 0 {
                                                            0 as uintmax_t
                                                        } else {
                                                            val
                                                        })
                                                        .wrapping_add(
                                                            (-(9223372036854775807
                                                                as ::core::ffi::c_long)
                                                                - 1)
                                                                as uintmax_t,
                                                        )
                                                    })
                                                    .wrapping_add(1 as uintmax_t)
                                                        << (::core::mem::size_of::<uintmax_t>()
                                                            as usize)
                                                            .wrapping_mul(8 as usize)
                                                            .wrapping_sub(2 as usize))
                                                    .wrapping_sub(1 as uintmax_t)
                                                    .wrapping_mul(2 as uintmax_t)
                                                    .wrapping_add(1 as uintmax_t)
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        (if 1 != 0 {
                                                            0 as uintmax_t
                                                        } else {
                                                            val
                                                        })
                                                        .wrapping_add(
                                                            (-(9223372036854775807
                                                                as ::core::ffi::c_long)
                                                                - 1)
                                                                as uintmax_t,
                                                        )
                                                    })
                                                    .wrapping_sub(1 as uintmax_t)
                                                })
                                                .wrapping_neg())
                                                    as ::core::ffi::c_int
                                            } else {
                                                ((0 as uintmax_t)
                                                    < (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(
                                                        (-(9223372036854775807
                                                            as ::core::ffi::c_long)
                                                            - 1)
                                                            as uintmax_t,
                                                    ))
                                                    as ::core::ffi::c_int
                                            }) != 0
                                                && val == -(1 as ::core::ffi::c_int) as uintmax_t
                                            {
                                                if ((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) - 1)
                                                    < 0
                                                {
                                                    ((0)
                                                        < base as ::core::ffi::c_long
                                                            + (-(9223372036854775807
                                                                as ::core::ffi::c_long)
                                                                - 1))
                                                        as ::core::ffi::c_int
                                                } else {
                                                    (-(1 as ::core::ffi::c_int)
                                                        as ::core::ffi::c_long
                                                        - (-(9223372036854775807
                                                            as ::core::ffi::c_long)
                                                            - 1)
                                                        < (base - 1)
                                                            as ::core::ffi::c_long)
                                                        as ::core::ffi::c_int
                                                }
                                            } else {
                                                (((-(9223372036854775807 as ::core::ffi::c_long)
                                                    - 1)
                                                    as uintmax_t)
                                                    .wrapping_div(val)
                                                    < base as uintmax_t)
                                                    as ::core::ffi::c_int
                                            }
                                        } else {
                                            (((9223372036854775807 as ::core::ffi::c_long
                                                / base as ::core::ffi::c_long)
                                                as uintmax_t)
                                                < val)
                                                as ::core::ffi::c_int
                                        }
                                    }
                                }) != 0
                                {
                                    val = (val as ::core::ffi::c_ulong)
                                        .wrapping_mul(base as ::core::ffi::c_ulong)
                                        as ::core::ffi::c_long
                                        as uintmax_t;
                                    1
                                } else {
                                    val = (val as ::core::ffi::c_ulong)
                                        .wrapping_mul(base as ::core::ffi::c_ulong)
                                        as ::core::ffi::c_long
                                        as uintmax_t;
                                    0
                                }
                            } else {
                                if (if base < 0 {
                                    if val < 0 as uintmax_t {
                                        if (if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                (9223372036854775807 as ::core::ffi::c_long
                                                    as ::core::ffi::c_ulong)
                                                    .wrapping_mul(2)
                                                    .wrapping_add(1)
                                            })
                                            .wrapping_add(base as ::core::ffi::c_ulong)
                                        })
                                        .wrapping_sub(1)
                                            < 0
                                        {
                                            (val < (9223372036854775807 as uintmax_t)
                                                .wrapping_mul(2 as uintmax_t)
                                                .wrapping_add(1 as uintmax_t)
                                                .wrapping_div(base as uintmax_t))
                                                as ::core::ffi::c_int
                                        } else {
                                            ((if (if (if ((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) - 1)
                                                < 0
                                            {
                                                !(((((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 1)
                                                    << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize)) - 1)
                                                    * 2 + 1)
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0
                                            }) < 0
                                            {
                                                (base
                                                    < -(if ((if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) - 1)
                                                        < 0
                                                    {
                                                        ((((if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        }) + 1)
                                                            << (::core::mem::size_of::<
                                                                ::core::ffi::c_int,
                                                            >(
                                                            )
                                                                as usize)
                                                                .wrapping_mul(8 as usize)
                                                                .wrapping_sub(2 as usize))
                                                            - 1)
                                                            * 2
                                                            + 1
                                                    } else {
                                                        (if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        }) - 1
                                                    }))
                                                    as ::core::ffi::c_int
                                            } else {
                                                ((0) < base)
                                                    as ::core::ffi::c_int
                                            }) != 0
                                            {
                                                ((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                })
                                                    as uintmax_t)
                                                    .wrapping_add(
                                                        (9223372036854775807 as uintmax_t)
                                                            .wrapping_mul(2 as uintmax_t)
                                                            .wrapping_add(1 as uintmax_t),
                                                    )
                                                    >> (::core::mem::size_of::<::core::ffi::c_int>()
                                                        as usize)
                                                        .wrapping_mul(CHAR_BIT as usize)
                                                        .wrapping_sub(1 as usize)
                                            } else {
                                                (9223372036854775807 as uintmax_t)
                                                    .wrapping_mul(2 as uintmax_t)
                                                    .wrapping_add(1 as uintmax_t)
                                                    .wrapping_div(-base as uintmax_t)
                                            }) <= (-(1 as ::core::ffi::c_int) as uintmax_t)
                                                .wrapping_sub(val))
                                                as ::core::ffi::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 0
                                        }) - 1)
                                            < 0
                                        {
                                            !(((((if 1 != 0 {
                                                0
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0
                                            }) + 1)
                                                << (::core::mem::size_of::<::core::ffi::c_int>()
                                                    as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                                - 1)
                                                * 2
                                                + 1)
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0
                                            }) + 0
                                        }) < 0
                                        {
                                            (((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 0)
                                                < -(if ((if 1 != 0 {
                                                    0
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) + 0
                                                }) - 1)
                                                    < 0
                                                {
                                                    ((((if 1 != 0 {
                                                        0
                                                    } else {
                                                        (if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        }) + 0
                                                    }) + 1)
                                                        << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                            .wrapping_mul(8 as usize)
                                                            .wrapping_sub(2 as usize)) - 1)
                                                        * 2 + 1
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        (if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        }) + 0
                                                    }) - 1
                                                }))
                                                as ::core::ffi::c_int
                                        } else {
                                            ((0)
                                                < (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0)
                                                as ::core::ffi::c_int
                                        }) != 0
                                            && base == -(1 as ::core::ffi::c_int)
                                        {
                                            if (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_sub(1 as uintmax_t)
                                                < 0 as uintmax_t
                                            {
                                                ((0 as uintmax_t)
                                                    < val.wrapping_add(0 as uintmax_t))
                                                    as ::core::ffi::c_int
                                            } else {
                                                ((0 as uintmax_t) < val
                                                    && ((-(1 as ::core::ffi::c_int)
                                                        - 0)
                                                        as uintmax_t)
                                                        < val.wrapping_sub(1 as uintmax_t))
                                                    as ::core::ffi::c_int
                                            }
                                        } else {
                                            (((0 / base) as uintmax_t) < val)
                                                as ::core::ffi::c_int
                                        }
                                    }
                                } else {
                                    if base == 0 {
                                        0
                                    } else {
                                        if val < 0 as uintmax_t {
                                            if (if (if (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(0 as uintmax_t)
                                            })
                                            .wrapping_sub(1 as uintmax_t)
                                                < 0 as uintmax_t
                                            {
                                                !((if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(0 as uintmax_t)
                                                })
                                                .wrapping_add(1 as uintmax_t)
                                                    << (::core::mem::size_of::<uintmax_t>()
                                                        as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize))
                                                .wrapping_sub(1 as uintmax_t)
                                                .wrapping_mul(2 as uintmax_t)
                                                .wrapping_add(1 as uintmax_t)
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(0 as uintmax_t)
                                                })
                                                .wrapping_add(0 as uintmax_t)
                                            }) < 0 as uintmax_t
                                            {
                                                ((if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(0 as uintmax_t)
                                                    < (if (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        (if 1 != 0 {
                                                            0 as uintmax_t
                                                        } else {
                                                            val
                                                        })
                                                        .wrapping_add(0 as uintmax_t)
                                                    })
                                                    .wrapping_sub(1 as uintmax_t)
                                                        < 0 as uintmax_t
                                                    {
                                                        ((if 1 != 0 {
                                                            0 as uintmax_t
                                                        } else {
                                                            (if 1 != 0 {
                                                                0 as uintmax_t
                                                            } else {
                                                                val
                                                            })
                                                            .wrapping_add(0 as uintmax_t)
                                                        })
                                                        .wrapping_add(1 as uintmax_t)
                                                            << (::core::mem::size_of::<uintmax_t>()
                                                                as usize)
                                                                .wrapping_mul(8 as usize)
                                                                .wrapping_sub(2 as usize))
                                                        .wrapping_sub(1 as uintmax_t)
                                                        .wrapping_mul(2 as uintmax_t)
                                                        .wrapping_add(1 as uintmax_t)
                                                    } else {
                                                        (if 1 != 0 {
                                                            0 as uintmax_t
                                                        } else {
                                                            (if 1 != 0 {
                                                                0 as uintmax_t
                                                            } else {
                                                                val
                                                            })
                                                            .wrapping_add(0 as uintmax_t)
                                                        })
                                                        .wrapping_sub(1 as uintmax_t)
                                                    })
                                                    .wrapping_neg())
                                                    as ::core::ffi::c_int
                                            } else {
                                                ((0 as uintmax_t)
                                                    < (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(0 as uintmax_t))
                                                    as ::core::ffi::c_int
                                            }) != 0
                                                && val == -(1 as ::core::ffi::c_int) as uintmax_t
                                            {
                                                if ((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) - 1)
                                                    < 0
                                                {
                                                    ((0)
                                                        < base + 0)
                                                        as ::core::ffi::c_int
                                                } else {
                                                    ((-(1 as ::core::ffi::c_int)
                                                        - 0)
                                                        < base - 1)
                                                        as ::core::ffi::c_int
                                                }
                                            } else {
                                                ((0 as uintmax_t).wrapping_div(val)
                                                    < base as uintmax_t)
                                                    as ::core::ffi::c_int
                                            }
                                        } else {
                                            ((9223372036854775807 as uintmax_t)
                                                .wrapping_mul(2 as uintmax_t)
                                                .wrapping_add(1 as uintmax_t)
                                                .wrapping_div(base as uintmax_t)
                                                < val)
                                                as ::core::ffi::c_int
                                        }
                                    }
                                }) != 0
                                {
                                    val = (val as ::core::ffi::c_ulong)
                                        .wrapping_mul(base as ::core::ffi::c_ulong)
                                        as uintmax_t;
                                    1
                                } else {
                                    val = (val as ::core::ffi::c_ulong)
                                        .wrapping_mul(base as ::core::ffi::c_ulong)
                                        as uintmax_t;
                                    0
                                }
                            }
                        } else {
                            if (if 1 != 0 {
                                0 as uintmax_t
                            } else {
                                val
                            })
                            .wrapping_sub(1 as uintmax_t)
                                < 0 as uintmax_t
                            {
                                if (if base < 0 {
                                    if val < 0 as uintmax_t {
                                        if ((if 1 != 0 {
                                            0 as ::core::ffi::c_longlong
                                        } else {
                                            (if 1 != 0 {
                                                0 as ::core::ffi::c_longlong
                                            } else {
                                                9223372036854775807 as ::core::ffi::c_longlong
                                            }) + base as ::core::ffi::c_longlong
                                        }) - 1 as ::core::ffi::c_longlong)
                                            < 0 as ::core::ffi::c_longlong
                                        {
                                            ((val as ::core::ffi::c_ulonglong)
                                                < (9223372036854775807 as ::core::ffi::c_longlong
                                                    / base as ::core::ffi::c_longlong)
                                                    as ::core::ffi::c_ulonglong)
                                                as ::core::ffi::c_int
                                        } else {
                                            ((if (if (if ((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) - 1)
                                                < 0
                                            {
                                                !(((((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 1)
                                                    << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize)) - 1)
                                                    * 2 + 1)
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0
                                            }) < 0
                                            {
                                                (base
                                                    < -(if ((if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) - 1)
                                                        < 0
                                                    {
                                                        ((((if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        }) + 1)
                                                            << (::core::mem::size_of::<
                                                                ::core::ffi::c_int,
                                                            >(
                                                            )
                                                                as usize)
                                                                .wrapping_mul(8 as usize)
                                                                .wrapping_sub(2 as usize))
                                                            - 1)
                                                            * 2
                                                            + 1
                                                    } else {
                                                        (if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        }) - 1
                                                    }))
                                                    as ::core::ffi::c_int
                                            } else {
                                                ((0) < base)
                                                    as ::core::ffi::c_int
                                            }) != 0
                                            {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                })
                                                    as ::core::ffi::c_longlong
                                                    + 9223372036854775807 as ::core::ffi::c_longlong
                                                    >> (::core::mem::size_of::<::core::ffi::c_int>()
                                                        as usize)
                                                        .wrapping_mul(CHAR_BIT as usize)
                                                        .wrapping_sub(1 as usize)
                                            } else {
                                                9223372036854775807 as ::core::ffi::c_longlong
                                                    / -base as ::core::ffi::c_longlong
                                            })
                                                as ::core::ffi::c_ulonglong
                                                <= (-(1 as ::core::ffi::c_int) as uintmax_t)
                                                    .wrapping_sub(val)
                                                    as ::core::ffi::c_ulonglong)
                                                as ::core::ffi::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 != 0 {
                                            0 as ::core::ffi::c_longlong
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            })
                                                as ::core::ffi::c_longlong
                                                + (-(9223372036854775807
                                                    as ::core::ffi::c_longlong)
                                                    - 1 as ::core::ffi::c_longlong)
                                        }) - 1 as ::core::ffi::c_longlong)
                                            < 0 as ::core::ffi::c_longlong
                                        {
                                            !(((((if 1 != 0 {
                                                0 as ::core::ffi::c_longlong
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) as ::core::ffi::c_longlong
                                                    + (-(9223372036854775807 as ::core::ffi::c_longlong)
                                                        - 1 as ::core::ffi::c_longlong)
                                            }) + 1 as ::core::ffi::c_longlong)
                                                << (::core::mem::size_of::<::core::ffi::c_longlong>()
                                                    as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize)) - 1 as ::core::ffi::c_longlong)
                                                * 2 as ::core::ffi::c_longlong
                                                + 1 as ::core::ffi::c_longlong)
                                        } else {
                                            (if 1 != 0 {
                                                0 as ::core::ffi::c_longlong
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                })
                                                    as ::core::ffi::c_longlong
                                                    + (-(9223372036854775807
                                                        as ::core::ffi::c_longlong)
                                                        - 1 as ::core::ffi::c_longlong)
                                            }) + 0 as ::core::ffi::c_longlong
                                        }) < 0 as ::core::ffi::c_longlong
                                        {
                                            ((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            })
                                                as ::core::ffi::c_longlong
                                                + (-(9223372036854775807
                                                    as ::core::ffi::c_longlong)
                                                    - 1 as ::core::ffi::c_longlong)
                                                < -(if ((if 1 != 0 {
                                                    0 as ::core::ffi::c_longlong
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    })
                                                        as ::core::ffi::c_longlong
                                                        + (-(9223372036854775807
                                                            as ::core::ffi::c_longlong)
                                                            - 1 as ::core::ffi::c_longlong)
                                                }) - 1 as ::core::ffi::c_longlong)
                                                    < 0 as ::core::ffi::c_longlong
                                                {
                                                    ((((if 1 != 0 {
                                                        0 as ::core::ffi::c_longlong
                                                    } else {
                                                        (if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        })
                                                            as ::core::ffi::c_longlong
                                                            + (-(9223372036854775807
                                                                as ::core::ffi::c_longlong)
                                                                - 1 as ::core::ffi::c_longlong)
                                                    }) + 1 as ::core::ffi::c_longlong)
                                                        << (::core::mem::size_of::<
                                                            ::core::ffi::c_longlong,
                                                        >(
                                                        )
                                                            as usize)
                                                            .wrapping_mul(8 as usize)
                                                            .wrapping_sub(2 as usize))
                                                        - 1 as ::core::ffi::c_longlong)
                                                        * 2 as ::core::ffi::c_longlong
                                                        + 1 as ::core::ffi::c_longlong
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as ::core::ffi::c_longlong
                                                    } else {
                                                        (if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        })
                                                            as ::core::ffi::c_longlong
                                                            + (-(9223372036854775807
                                                                as ::core::ffi::c_longlong)
                                                                - 1 as ::core::ffi::c_longlong)
                                                    }) - 1 as ::core::ffi::c_longlong
                                                }))
                                                as ::core::ffi::c_int
                                        } else {
                                            ((0 as ::core::ffi::c_longlong)
                                                < (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                })
                                                    as ::core::ffi::c_longlong
                                                    + (-(9223372036854775807
                                                        as ::core::ffi::c_longlong)
                                                        - 1 as ::core::ffi::c_longlong))
                                                as ::core::ffi::c_int
                                        }) != 0
                                            && base == -(1 as ::core::ffi::c_int)
                                        {
                                            if (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_sub(1 as uintmax_t)
                                                < 0 as uintmax_t
                                            {
                                                ((0 as ::core::ffi::c_ulonglong)
                                                    < (val as ::core::ffi::c_ulonglong)
                                                        .wrapping_add(
                                                            (-(9223372036854775807
                                                                as ::core::ffi::c_longlong)
                                                                - 1 as ::core::ffi::c_longlong)
                                                                as ::core::ffi::c_ulonglong,
                                                        ))
                                                    as ::core::ffi::c_int
                                            } else {
                                                ((0 as uintmax_t) < val
                                                    && ((-(1 as ::core::ffi::c_int)
                                                        as ::core::ffi::c_longlong
                                                        - (-(9223372036854775807
                                                            as ::core::ffi::c_longlong)
                                                            - 1 as ::core::ffi::c_longlong))
                                                        as ::core::ffi::c_ulonglong)
                                                        < val.wrapping_sub(1 as uintmax_t)
                                                            as ::core::ffi::c_ulonglong)
                                                    as ::core::ffi::c_int
                                            }
                                        } else {
                                            ((((-(9223372036854775807 as ::core::ffi::c_longlong)
                                                - 1 as ::core::ffi::c_longlong)
                                                / base as ::core::ffi::c_longlong)
                                                as ::core::ffi::c_ulonglong)
                                                < val as ::core::ffi::c_ulonglong)
                                                as ::core::ffi::c_int
                                        }
                                    }
                                } else {
                                    if base == 0 {
                                        0
                                    } else {
                                        if val < 0 as uintmax_t {
                                            if (if (if (if 1 != 0 {
                                                0 as ::core::ffi::c_ulonglong
                                            } else {
                                                ((if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                    as ::core::ffi::c_ulonglong)
                                                    .wrapping_add(
                                                        (-(9223372036854775807
                                                            as ::core::ffi::c_longlong)
                                                            - 1 as ::core::ffi::c_longlong)
                                                            as ::core::ffi::c_ulonglong,
                                                    )
                                            })
                                            .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                                                < 0 as ::core::ffi::c_ulonglong
                                            {
                                                !((if 1 != 0 {
                                                    0 as ::core::ffi::c_ulonglong
                                                } else {
                                                    ((if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                        as ::core::ffi::c_ulonglong)
                                                        .wrapping_add(
                                                            (-(9223372036854775807
                                                                as ::core::ffi::c_longlong)
                                                                - 1 as ::core::ffi::c_longlong)
                                                                as ::core::ffi::c_ulonglong,
                                                        )
                                                })
                                                .wrapping_add(1 as ::core::ffi::c_ulonglong)
                                                    << (::core::mem::size_of::<
                                                        ::core::ffi::c_ulonglong,
                                                    >(
                                                    )
                                                        as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize))
                                                .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                                                .wrapping_mul(2 as ::core::ffi::c_ulonglong)
                                                .wrapping_add(1 as ::core::ffi::c_ulonglong)
                                            } else {
                                                (if 1 != 0 {
                                                    0 as ::core::ffi::c_ulonglong
                                                } else {
                                                    ((if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                        as ::core::ffi::c_ulonglong)
                                                        .wrapping_add(
                                                            (-(9223372036854775807
                                                                as ::core::ffi::c_longlong)
                                                                - 1 as ::core::ffi::c_longlong)
                                                                as ::core::ffi::c_ulonglong,
                                                        )
                                                })
                                                .wrapping_add(0 as ::core::ffi::c_ulonglong)
                                            }) < 0 as ::core::ffi::c_ulonglong
                                            {
                                                (((if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                }) as ::core::ffi::c_ulonglong)
                                                    .wrapping_add(
                                                        (-(9223372036854775807 as ::core::ffi::c_longlong)
                                                            - 1 as ::core::ffi::c_longlong) as ::core::ffi::c_ulonglong,
                                                    )
                                                    < (if (if 1 != 0 {
                                                        0 as ::core::ffi::c_ulonglong
                                                    } else {
                                                        ((if 1 != 0 {
                                                            0 as uintmax_t
                                                        } else {
                                                            val
                                                        }) as ::core::ffi::c_ulonglong)
                                                            .wrapping_add(
                                                                (-(9223372036854775807 as ::core::ffi::c_longlong)
                                                                    - 1 as ::core::ffi::c_longlong) as ::core::ffi::c_ulonglong,
                                                            )
                                                    })
                                                        .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                                                        < 0 as ::core::ffi::c_ulonglong
                                                    {
                                                        ((if 1 != 0 {
                                                            0 as ::core::ffi::c_ulonglong
                                                        } else {
                                                            ((if 1 != 0 {
                                                                0 as uintmax_t
                                                            } else {
                                                                val
                                                            }) as ::core::ffi::c_ulonglong)
                                                                .wrapping_add(
                                                                    (-(9223372036854775807 as ::core::ffi::c_longlong)
                                                                        - 1 as ::core::ffi::c_longlong) as ::core::ffi::c_ulonglong,
                                                                )
                                                        })
                                                            .wrapping_add(1 as ::core::ffi::c_ulonglong)
                                                            << (::core::mem::size_of::<::core::ffi::c_ulonglong>()
                                                                as usize)
                                                                .wrapping_mul(8 as usize)
                                                                .wrapping_sub(2 as usize))
                                                            .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                                                            .wrapping_mul(2 as ::core::ffi::c_ulonglong)
                                                            .wrapping_add(1 as ::core::ffi::c_ulonglong)
                                                    } else {
                                                        (if 1 != 0 {
                                                            0 as ::core::ffi::c_ulonglong
                                                        } else {
                                                            ((if 1 != 0 {
                                                                0 as uintmax_t
                                                            } else {
                                                                val
                                                            }) as ::core::ffi::c_ulonglong)
                                                                .wrapping_add(
                                                                    (-(9223372036854775807 as ::core::ffi::c_longlong)
                                                                        - 1 as ::core::ffi::c_longlong) as ::core::ffi::c_ulonglong,
                                                                )
                                                        })
                                                            .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                                                    })
                                                        .wrapping_neg()) as ::core::ffi::c_int
                                            } else {
                                                ((0 as ::core::ffi::c_ulonglong)
                                                    < ((if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                        as ::core::ffi::c_ulonglong)
                                                        .wrapping_add(
                                                            (-(9223372036854775807
                                                                as ::core::ffi::c_longlong)
                                                                - 1 as ::core::ffi::c_longlong)
                                                                as ::core::ffi::c_ulonglong,
                                                        ))
                                                    as ::core::ffi::c_int
                                            }) != 0
                                                && val == -(1 as ::core::ffi::c_int) as uintmax_t
                                            {
                                                if ((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) - 1)
                                                    < 0
                                                {
                                                    ((0 as ::core::ffi::c_longlong)
                                                        < base as ::core::ffi::c_longlong
                                                            + (-(9223372036854775807
                                                                as ::core::ffi::c_longlong)
                                                                - 1 as ::core::ffi::c_longlong))
                                                        as ::core::ffi::c_int
                                                } else {
                                                    (-(1 as ::core::ffi::c_int)
                                                        as ::core::ffi::c_longlong
                                                        - (-(9223372036854775807
                                                            as ::core::ffi::c_longlong)
                                                            - 1 as ::core::ffi::c_longlong)
                                                        < (base - 1)
                                                            as ::core::ffi::c_longlong)
                                                        as ::core::ffi::c_int
                                                }
                                            } else {
                                                (((-(9223372036854775807
                                                    as ::core::ffi::c_longlong)
                                                    - 1 as ::core::ffi::c_longlong)
                                                    as ::core::ffi::c_ulonglong)
                                                    .wrapping_div(val as ::core::ffi::c_ulonglong)
                                                    < base as ::core::ffi::c_ulonglong)
                                                    as ::core::ffi::c_int
                                            }
                                        } else {
                                            (((9223372036854775807 as ::core::ffi::c_longlong
                                                / base as ::core::ffi::c_longlong)
                                                as ::core::ffi::c_ulonglong)
                                                < val as ::core::ffi::c_ulonglong)
                                                as ::core::ffi::c_int
                                        }
                                    }
                                }) != 0
                                {
                                    val = (val as ::core::ffi::c_ulonglong)
                                        .wrapping_mul(base as ::core::ffi::c_ulonglong)
                                        as ::core::ffi::c_longlong
                                        as uintmax_t;
                                    1
                                } else {
                                    val = (val as ::core::ffi::c_ulonglong)
                                        .wrapping_mul(base as ::core::ffi::c_ulonglong)
                                        as ::core::ffi::c_longlong
                                        as uintmax_t;
                                    0
                                }
                            } else {
                                if (if base < 0 {
                                    if val < 0 as uintmax_t {
                                        if (if 1 != 0 {
                                            0 as ::core::ffi::c_ulonglong
                                        } else {
                                            (if 1 != 0 {
                                                0 as ::core::ffi::c_ulonglong
                                            } else {
                                                (9223372036854775807 as ::core::ffi::c_ulonglong)
                                                    .wrapping_mul(2 as ::core::ffi::c_ulonglong)
                                                    .wrapping_add(1 as ::core::ffi::c_ulonglong)
                                            })
                                            .wrapping_add(base as ::core::ffi::c_ulonglong)
                                        })
                                        .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                                            < 0 as ::core::ffi::c_ulonglong
                                        {
                                            ((val as ::core::ffi::c_ulonglong)
                                                < (9223372036854775807 as ::core::ffi::c_ulonglong)
                                                    .wrapping_mul(2 as ::core::ffi::c_ulonglong)
                                                    .wrapping_add(1 as ::core::ffi::c_ulonglong)
                                                    .wrapping_div(base as ::core::ffi::c_ulonglong))
                                                as ::core::ffi::c_int
                                        } else {
                                            ((if (if (if ((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) - 1)
                                                < 0
                                            {
                                                !(((((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 1)
                                                    << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize)) - 1)
                                                    * 2 + 1)
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0
                                            }) < 0
                                            {
                                                (base
                                                    < -(if ((if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) - 1)
                                                        < 0
                                                    {
                                                        ((((if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        }) + 1)
                                                            << (::core::mem::size_of::<
                                                                ::core::ffi::c_int,
                                                            >(
                                                            )
                                                                as usize)
                                                                .wrapping_mul(8 as usize)
                                                                .wrapping_sub(2 as usize))
                                                            - 1)
                                                            * 2
                                                            + 1
                                                    } else {
                                                        (if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        }) - 1
                                                    }))
                                                    as ::core::ffi::c_int
                                            } else {
                                                ((0) < base)
                                                    as ::core::ffi::c_int
                                            }) != 0
                                            {
                                                ((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                })
                                                    as ::core::ffi::c_ulonglong)
                                                    .wrapping_add(
                                                        (9223372036854775807
                                                            as ::core::ffi::c_ulonglong)
                                                            .wrapping_mul(
                                                                2 as ::core::ffi::c_ulonglong,
                                                            )
                                                            .wrapping_add(
                                                                1 as ::core::ffi::c_ulonglong,
                                                            ),
                                                    )
                                                    >> (::core::mem::size_of::<::core::ffi::c_int>()
                                                        as usize)
                                                        .wrapping_mul(CHAR_BIT as usize)
                                                        .wrapping_sub(1 as usize)
                                            } else {
                                                (9223372036854775807 as ::core::ffi::c_ulonglong)
                                                    .wrapping_mul(2 as ::core::ffi::c_ulonglong)
                                                    .wrapping_add(1 as ::core::ffi::c_ulonglong)
                                                    .wrapping_div(-base as ::core::ffi::c_ulonglong)
                                            }) <= (-(1 as ::core::ffi::c_int) as uintmax_t)
                                                .wrapping_sub(val)
                                                as ::core::ffi::c_ulonglong)
                                                as ::core::ffi::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 != 0 {
                                            0
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 0
                                        }) - 1)
                                            < 0
                                        {
                                            !(((((if 1 != 0 {
                                                0
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0
                                            }) + 1)
                                                << (::core::mem::size_of::<::core::ffi::c_int>()
                                                    as usize)
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(2 as usize))
                                                - 1)
                                                * 2
                                                + 1)
                                        } else {
                                            (if 1 != 0 {
                                                0
                                            } else {
                                                (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0
                                            }) + 0
                                        }) < 0
                                        {
                                            (((if 1 != 0 {
                                                0
                                            } else {
                                                base
                                            }) + 0)
                                                < -(if ((if 1 != 0 {
                                                    0
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        base
                                                    }) + 0
                                                }) - 1)
                                                    < 0
                                                {
                                                    ((((if 1 != 0 {
                                                        0
                                                    } else {
                                                        (if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        }) + 0
                                                    }) + 1)
                                                        << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                            .wrapping_mul(8 as usize)
                                                            .wrapping_sub(2 as usize)) - 1)
                                                        * 2 + 1
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        (if 1 != 0 {
                                                            0
                                                        } else {
                                                            base
                                                        }) + 0
                                                    }) - 1
                                                }))
                                                as ::core::ffi::c_int
                                        } else {
                                            ((0)
                                                < (if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) + 0)
                                                as ::core::ffi::c_int
                                        }) != 0
                                            && base == -(1 as ::core::ffi::c_int)
                                        {
                                            if (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                val
                                            })
                                            .wrapping_sub(1 as uintmax_t)
                                                < 0 as uintmax_t
                                            {
                                                ((0 as uintmax_t)
                                                    < val.wrapping_add(0 as uintmax_t))
                                                    as ::core::ffi::c_int
                                            } else {
                                                ((0 as uintmax_t) < val
                                                    && ((-(1 as ::core::ffi::c_int)
                                                        - 0)
                                                        as uintmax_t)
                                                        < val.wrapping_sub(1 as uintmax_t))
                                                    as ::core::ffi::c_int
                                            }
                                        } else {
                                            (((0 / base) as uintmax_t) < val)
                                                as ::core::ffi::c_int
                                        }
                                    }
                                } else {
                                    if base == 0 {
                                        0
                                    } else {
                                        if val < 0 as uintmax_t {
                                            if (if (if (if 1 != 0 {
                                                0 as uintmax_t
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(0 as uintmax_t)
                                            })
                                            .wrapping_sub(1 as uintmax_t)
                                                < 0 as uintmax_t
                                            {
                                                !((if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(0 as uintmax_t)
                                                })
                                                .wrapping_add(1 as uintmax_t)
                                                    << (::core::mem::size_of::<uintmax_t>()
                                                        as usize)
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(2 as usize))
                                                .wrapping_sub(1 as uintmax_t)
                                                .wrapping_mul(2 as uintmax_t)
                                                .wrapping_add(1 as uintmax_t)
                                            } else {
                                                (if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(0 as uintmax_t)
                                                })
                                                .wrapping_add(0 as uintmax_t)
                                            }) < 0 as uintmax_t
                                            {
                                                ((if 1 != 0 {
                                                    0 as uintmax_t
                                                } else {
                                                    val
                                                })
                                                .wrapping_add(0 as uintmax_t)
                                                    < (if (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        (if 1 != 0 {
                                                            0 as uintmax_t
                                                        } else {
                                                            val
                                                        })
                                                        .wrapping_add(0 as uintmax_t)
                                                    })
                                                    .wrapping_sub(1 as uintmax_t)
                                                        < 0 as uintmax_t
                                                    {
                                                        ((if 1 != 0 {
                                                            0 as uintmax_t
                                                        } else {
                                                            (if 1 != 0 {
                                                                0 as uintmax_t
                                                            } else {
                                                                val
                                                            })
                                                            .wrapping_add(0 as uintmax_t)
                                                        })
                                                        .wrapping_add(1 as uintmax_t)
                                                            << (::core::mem::size_of::<uintmax_t>()
                                                                as usize)
                                                                .wrapping_mul(8 as usize)
                                                                .wrapping_sub(2 as usize))
                                                        .wrapping_sub(1 as uintmax_t)
                                                        .wrapping_mul(2 as uintmax_t)
                                                        .wrapping_add(1 as uintmax_t)
                                                    } else {
                                                        (if 1 != 0 {
                                                            0 as uintmax_t
                                                        } else {
                                                            (if 1 != 0 {
                                                                0 as uintmax_t
                                                            } else {
                                                                val
                                                            })
                                                            .wrapping_add(0 as uintmax_t)
                                                        })
                                                        .wrapping_sub(1 as uintmax_t)
                                                    })
                                                    .wrapping_neg())
                                                    as ::core::ffi::c_int
                                            } else {
                                                ((0 as uintmax_t)
                                                    < (if 1 != 0 {
                                                        0 as uintmax_t
                                                    } else {
                                                        val
                                                    })
                                                    .wrapping_add(0 as uintmax_t))
                                                    as ::core::ffi::c_int
                                            }) != 0
                                                && val == -(1 as ::core::ffi::c_int) as uintmax_t
                                            {
                                                if ((if 1 != 0 {
                                                    0
                                                } else {
                                                    base
                                                }) - 1)
                                                    < 0
                                                {
                                                    ((0)
                                                        < base + 0)
                                                        as ::core::ffi::c_int
                                                } else {
                                                    ((-(1 as ::core::ffi::c_int)
                                                        - 0)
                                                        < base - 1)
                                                        as ::core::ffi::c_int
                                                }
                                            } else {
                                                ((0 as uintmax_t).wrapping_div(val)
                                                    < base as uintmax_t)
                                                    as ::core::ffi::c_int
                                            }
                                        } else {
                                            ((9223372036854775807 as ::core::ffi::c_ulonglong)
                                                .wrapping_mul(2 as ::core::ffi::c_ulonglong)
                                                .wrapping_add(1 as ::core::ffi::c_ulonglong)
                                                .wrapping_div(base as ::core::ffi::c_ulonglong)
                                                < val as ::core::ffi::c_ulonglong)
                                                as ::core::ffi::c_int
                                        }
                                    }
                                }) != 0
                                {
                                    val = (val as ::core::ffi::c_ulonglong)
                                        .wrapping_mul(base as ::core::ffi::c_ulonglong)
                                        as uintmax_t;
                                    1
                                } else {
                                    val = (val as ::core::ffi::c_ulonglong)
                                        .wrapping_mul(base as ::core::ffi::c_ulonglong)
                                        as uintmax_t;
                                    0
                                }
                            }
                        }
                    }
                }
            }) != 0
            || {
                let (fresh0, fresh1) = val.overflowing_add((*ptr as ::core::ffi::c_int - '0' as i32) as u64);
                *&raw mut val = fresh0;
                fresh1 as ::core::ffi::c_int != 0
            }
            || val > max
        {
            fatal(
                ::core::ptr::null_mut::<Floc>(),
                (strlen(type_0) as size_t)
                    .wrapping_add(strlen(archive) as size_t)
                    .wrapping_add(strlen(name) as size_t),
                b"invalid %s for archive %s member %s\0" as *const u8 as *const ::core::ffi::c_char,
                type_0,
                archive,
                name,
            );
        }
        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
    }
    val
}
#[no_mangle]
pub unsafe extern "C" fn ar_scan(
    mut archive: *const ::core::ffi::c_char,
    mut function: ar_member_func_t,
    mut arg: *const ::core::ffi::c_void,
) -> intmax_t {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut current_block: u64;
    let mut namemap: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut namemap_size: ::core::ffi::c_uint = 0;
    let mut desc: ::core::ffi::c_int = open(archive, O_RDONLY, 0);
    if desc < 0 {
        return -(1 as ::core::ffi::c_int) as intmax_t;
    }
    let mut buf: [::core::ffi::c_char; 8] = [0; 8];
    let mut nread: ::core::ffi::c_int = 0;
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
            let mut nread_0: ssize_t = 0;
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
            let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut is_namemap: ::core::ffi::c_int = 0;
            let mut long_name: ::core::ffi::c_int = 0;
            let mut eltsize: ::core::ffi::c_long = 0;
            let mut eltmode: ::core::ffi::c_uint = 0;
            let mut eltdate: intmax_t = 0;
            let mut eltuid: ::core::ffi::c_int = 0;
            let mut eltgid: ::core::ffi::c_int = 0;
            let mut fnval: intmax_t = 0;
            let mut o: off_t = 0;
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
                let mut name_off: ::core::ffi::c_uint =
                    make_toui(name.offset(1 as ::core::ffi::c_int as isize), &raw mut err);
                let mut name_len: size_t = 0;
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
                let mut name_len_0: ::core::ffi::c_uint = make_toui(
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
            eltmode = parse_int(
                &raw mut member_header.ar_mode as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t,
                8,
                (if (0 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    < -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint
                {
                    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint
                } else {
                    ((1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << (::core::mem::size_of::<::core::ffi::c_uint>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(2 as usize))
                    .wrapping_sub(1)
                    .wrapping_mul(2)
                    .wrapping_add(1)
                }) as uintmax_t,
                b"mode\0" as *const u8 as *const ::core::ffi::c_char,
                archive,
                name,
            ) as ::core::ffi::c_uint;
            eltsize = parse_int(
                &raw mut member_header.ar_size as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t,
                10,
                (if (0 as ::core::ffi::c_int as ::core::ffi::c_long)
                    < -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
                {
                    -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
                } else {
                    (((1 as ::core::ffi::c_int as ::core::ffi::c_long)
                        << (::core::mem::size_of::<::core::ffi::c_long>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(2 as usize))
                        - 1)
                        * 2
                        + 1
                }) as uintmax_t,
                b"size\0" as *const u8 as *const ::core::ffi::c_char,
                archive,
                name,
            ) as ::core::ffi::c_long;
            eltdate = parse_int(
                &raw mut member_header.ar_date as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t,
                10,
                (if (0 as ::core::ffi::c_int as intmax_t) < -(1 as ::core::ffi::c_int) as intmax_t {
                    -(1 as ::core::ffi::c_int) as intmax_t
                } else {
                    (((1 as ::core::ffi::c_int as intmax_t)
                        << (::core::mem::size_of::<intmax_t>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(2 as usize))
                        - 1 as intmax_t)
                        * 2 as intmax_t
                        + 1 as intmax_t
                }) as uintmax_t,
                b"date\0" as *const u8 as *const ::core::ffi::c_char,
                archive,
                name,
            ) as intmax_t;
            eltuid = parse_int(
                &raw mut member_header.ar_uid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t,
                10,
                (if (0) < -(1 as ::core::ffi::c_int) {
                    -(1 as ::core::ffi::c_int)
                } else {
                    (((1)
                        << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(2 as usize))
                        - 1)
                        * 2
                        + 1
                }) as uintmax_t,
                b"uid\0" as *const u8 as *const ::core::ffi::c_char,
                archive,
                name,
            ) as ::core::ffi::c_int;
            eltgid = parse_int(
                &raw mut member_header.ar_gid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t,
                10,
                (if (0) < -(1 as ::core::ffi::c_int) {
                    -(1 as ::core::ffi::c_int)
                } else {
                    (((1)
                        << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(2 as usize))
                        - 1)
                        * 2
                        + 1
                }) as uintmax_t,
                b"gid\0" as *const u8 as *const ::core::ffi::c_char,
                archive,
                name,
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
                let mut clear: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut limit: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
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
                is_namemap = 0;
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
    mut mem: *const ::core::ffi::c_char,
    mut truncated: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
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
    mut mem: *const ::core::ffi::c_char,
    mut truncated: ::core::ffi::c_int,
    mut hdrpos: ::core::ffi::c_long,
    mut _datapos: ::core::ffi::c_long,
    mut _size: ::core::ffi::c_long,
    mut _date: intmax_t,
    mut _uid: ::core::ffi::c_int,
    mut _gid: ::core::ffi::c_int,
    mut _mode: ::core::ffi::c_uint,
    mut name: *const ::core::ffi::c_void,
) -> intmax_t {
    if ar_name_equal(name as *const ::core::ffi::c_char, mem, truncated) == 0 {
        return 0 as intmax_t;
    }
    hdrpos as intmax_t
}
#[no_mangle]
pub unsafe extern "C" fn ar_member_touch(
    mut arname: *const ::core::ffi::c_char,
    mut memname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pos: intmax_t = ar_scan(
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
    let mut opos: off_t = 0;
    let mut fd: ::core::ffi::c_int = 0;
    let mut ar_hdr: ar_hdr = ar_hdr {
        ar_name: [0; 16],
        ar_date: [0; 12],
        ar_uid: [0; 6],
        ar_gid: [0; 6],
        ar_mode: [0; 8],
        ar_size: [0; 10],
        ar_fmag: [0; 2],
    };
    let mut o: off_t = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut datelen: ::core::ffi::c_int = 0;
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
