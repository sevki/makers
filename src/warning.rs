extern "C" {
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
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn error(flocp: *const floc, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    static mut stopchar_map: [::core::ffi::c_ushort; 0];
    fn variable_buffer_output(
        ptr: *mut ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
        length: size_t,
    ) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type uintmax_t = ::libc::uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const ::core::ffi::c_char,
    pub lineno: ::core::ffi::c_ulong,
    pub offset: ::core::ffi::c_ulong,
}
pub type warning_type = ::core::ffi::c_uint;
pub const wt_max: warning_type = 4;
pub const wt_undefined_var: warning_type = 3;
pub const wt_invalid_var: warning_type = 2;
pub const wt_invalid_ref: warning_type = 1;
pub const wt_circular_dep: warning_type = 0;
pub type warning_action = ::core::ffi::c_uint;
pub const w_error: warning_action = 3;
pub const w_warn: warning_action = 2;
pub const w_ignore: warning_action = 1;
pub const w_unset: warning_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct warning_data {
    pub global: warning_action,
    pub actions: [warning_action; 4],
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INTSTR_LENGTH: usize = (53 as usize)
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22 as usize)
    .wrapping_add(3 as usize);
#[no_mangle]
pub static mut warnings: [warning_action; 4] = [w_unset; 4];
static mut warn_default: warning_data = warning_data {
    global: w_unset,
    actions: [w_unset; 4],
};
static mut warn_variable: warning_data = warning_data {
    global: w_unset,
    actions: [w_unset; 4],
};
static mut warn_flag: warning_data = warning_data {
    global: w_unset,
    actions: [w_unset; 4],
};
static mut w_action_map: [*const ::core::ffi::c_char; 4] = [
    ::core::ptr::null::<::core::ffi::c_char>(),
    b"ignore\0" as *const u8 as *const ::core::ffi::c_char,
    b"warn\0" as *const u8 as *const ::core::ffi::c_char,
    b"error\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut w_name_map: [*const ::core::ffi::c_char; 4] = [
    b"circular-dep\0" as *const u8 as *const ::core::ffi::c_char,
    b"invalid-ref\0" as *const u8 as *const ::core::ffi::c_char,
    b"invalid-var\0" as *const u8 as *const ::core::ffi::c_char,
    b"undefined-var\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn set_warnings() {
    let mut wt: warning_type = wt_circular_dep;
    while (wt as ::core::ffi::c_uint) < wt_max as ::core::ffi::c_int as ::core::ffi::c_uint {
        warnings[wt as usize] = (if warn_flag.actions[wt as usize] as ::core::ffi::c_uint
            != w_unset as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            warn_flag.actions[wt as usize] as ::core::ffi::c_uint
        } else if warn_flag.global as ::core::ffi::c_uint
            != w_unset as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            warn_flag.global as ::core::ffi::c_uint
        } else if warn_variable.actions[wt as usize] as ::core::ffi::c_uint
            != w_unset as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            warn_variable.actions[wt as usize] as ::core::ffi::c_uint
        } else if warn_variable.global as ::core::ffi::c_uint
            != w_unset as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            warn_variable.global as ::core::ffi::c_uint
        } else {
            warn_default.actions[wt as usize] as ::core::ffi::c_uint
        }) as warning_action;
        wt += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn warn_init() {
    memset(
        &raw mut warn_default as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<warning_data>() as size_t,
    );
    memset(
        &raw mut warn_variable as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<warning_data>() as size_t,
    );
    memset(
        &raw mut warn_flag as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<warning_data>() as size_t,
    );
    warn_default.global = w_warn;
    warn_default.actions[wt_circular_dep as ::core::ffi::c_int as usize] = w_warn;
    warn_default.actions[wt_invalid_ref as ::core::ffi::c_int as usize] = w_warn;
    warn_default.actions[wt_invalid_var as ::core::ffi::c_int as usize] = w_warn;
    warn_default.actions[wt_undefined_var as ::core::ffi::c_int as usize] = w_ignore;
    set_warnings();
}
#[no_mangle]
pub unsafe extern "C" fn init_data(mut data: *mut warning_data) {
    (*data).global = w_unset;
    let mut wt: warning_type = wt_circular_dep;
    while (wt as ::core::ffi::c_uint) < wt_max as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*data).actions[wt as usize] = w_unset;
        wt += 1;
    }
}
unsafe extern "C" fn decode_warn_action(
    mut action: *const ::core::ffi::c_char,
    mut length: size_t,
) -> warning_action {
    let mut st: warning_action = w_ignore;
    while st as ::core::ffi::c_uint <= w_error as ::core::ffi::c_int as ::core::ffi::c_uint {
        let mut len: size_t = strlen(w_action_map[st as usize]) as size_t;
        if length == len
            && strncasecmp(action, w_action_map[st as usize], length as size_t)
                == 0 as ::core::ffi::c_int
        {
            return st;
        }
        st += 1;
    }
    return w_unset;
}
unsafe extern "C" fn decode_warn_name(
    mut name: *const ::core::ffi::c_char,
    mut length: size_t,
) -> warning_type {
    let mut wt: warning_type = wt_circular_dep;
    while (wt as ::core::ffi::c_uint) < wt_max as ::core::ffi::c_int as ::core::ffi::c_uint {
        let mut len: size_t = strlen(w_name_map[wt as usize]) as size_t;
        if length == len
            && strncasecmp(name, w_name_map[wt as usize], length as size_t)
                == 0 as ::core::ffi::c_int
        {
            return wt;
        }
        wt += 1;
    }
    return wt_max;
}
#[no_mangle]
pub unsafe extern "C" fn decode_warn_actions(
    mut value: *const ::core::ffi::c_char,
    mut flocp: *const floc,
) {
    let mut data: *mut warning_data = &raw mut warn_flag;
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*value as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        != 0 as ::core::ffi::c_int
    {
        value = value.offset(1);
    }
    if !flocp.is_null() {
        data = &raw mut warn_variable;
        if *value as ::core::ffi::c_int == '\0' as i32 {
            init_data(data);
        }
    }
    while *value as ::core::ffi::c_int != '\0' as i32 {
        let mut action: warning_action = w_unset;
        let mut ep: *const ::core::ffi::c_char = value;
        while !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*ep as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x400 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
            != 0 as ::core::ffi::c_int)
        {
            ep = ep.offset(1);
        }
        action = decode_warn_action(
            value,
            ep.offset_from(value) as ::core::ffi::c_long as size_t,
        );
        if action as ::core::ffi::c_uint != w_unset as ::core::ffi::c_int as ::core::ffi::c_uint {
            (*data).global = action;
        } else {
            let mut type_0: warning_type = wt_circular_dep;
            let mut cp: *const ::core::ffi::c_char = memchr(
                value as *const ::core::ffi::c_void,
                ':' as i32,
                ep.offset_from(value) as ::core::ffi::c_long as size_t,
            ) as *const ::core::ffi::c_char;
            let mut wl: ::core::ffi::c_int = 0;
            let mut al: ::core::ffi::c_int = 0;
            if cp.is_null() {
                cp = ep;
            }
            wl = cp.offset_from(value) as ::core::ffi::c_long as ::core::ffi::c_int;
            type_0 = decode_warn_name(value, wl as size_t);
            if cp == ep {
                action = w_warn;
            } else {
                cp = cp.offset(1);
                al = ep.offset_from(cp) as ::core::ffi::c_long as ::core::ffi::c_int;
                action = decode_warn_action(cp, al as size_t);
            }
            if type_0 as ::core::ffi::c_uint == wt_max as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if flocp.is_null() {
                    fatal(
                        ::core::ptr::null_mut::<floc>(),
                        INTSTR_LENGTH.wrapping_add(strlen(value) as size_t),
                        b"unknown warning '%.*s'\0" as *const u8 as *const ::core::ffi::c_char,
                        wl,
                        value,
                    );
                }
                error(
                    flocp,
                    INTSTR_LENGTH.wrapping_add(strlen(value) as size_t),
                    b"unknown warning '%.*s': ignored\0" as *const u8 as *const ::core::ffi::c_char,
                    wl,
                    value,
                );
            } else if action as ::core::ffi::c_uint
                == w_unset as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if flocp.is_null() {
                    fatal(
                        ::core::ptr::null_mut::<floc>(),
                        INTSTR_LENGTH.wrapping_add(strlen(cp) as size_t),
                        b"unknown warning action '%.*s'\0" as *const u8
                            as *const ::core::ffi::c_char,
                        al,
                        cp,
                    );
                }
                error(
                    flocp,
                    INTSTR_LENGTH.wrapping_add(strlen(cp) as size_t),
                    b"unknown warning action '%.*s': ignored\0" as *const u8
                        as *const ::core::ffi::c_char,
                    al,
                    cp,
                );
            } else {
                (*data).actions[type_0 as usize] = action;
            }
        }
        value = ep;
        while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*value as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x400 as ::core::ffi::c_int)
            != 0 as ::core::ffi::c_int
        {
            value = value.offset(1);
        }
    }
    set_warnings();
}
#[no_mangle]
pub unsafe extern "C" fn encode_warn_flag(
    mut fp: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut wt: warning_type = wt_circular_dep;
    let mut sp: ::core::ffi::c_char = '=' as i32 as ::core::ffi::c_char;
    wt = wt_circular_dep;
    while (wt as ::core::ffi::c_uint) < wt_max as ::core::ffi::c_int as ::core::ffi::c_uint {
        if warn_flag.actions[wt as usize] as ::core::ffi::c_uint
            != w_unset as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        wt += 1;
    }
    if wt as ::core::ffi::c_uint == wt_max as ::core::ffi::c_int as ::core::ffi::c_uint
        && warn_flag.global as ::core::ffi::c_uint
            == w_unset as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return fp;
    }
    fp = variable_buffer_output(
        fp,
        b" --warn\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t).wrapping_sub(1 as size_t),
    );
    if wt as ::core::ffi::c_uint == wt_max as ::core::ffi::c_int as ::core::ffi::c_uint
        && warn_flag.global as ::core::ffi::c_uint
            == w_warn as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return fp;
    }
    if warn_flag.global as ::core::ffi::c_uint
        > w_unset as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fp = variable_buffer_output(fp, &raw mut sp, 1 as size_t);
        sp = ',' as i32 as ::core::ffi::c_char;
        fp = variable_buffer_output(
            fp,
            w_action_map[warn_flag.global as usize],
            strlen(w_action_map[warn_flag.global as usize]) as size_t,
        );
    }
    if wt as ::core::ffi::c_uint != wt_max as ::core::ffi::c_int as ::core::ffi::c_uint {
        wt = wt_circular_dep;
        while (wt as ::core::ffi::c_uint) < wt_max as ::core::ffi::c_int as ::core::ffi::c_uint {
            let mut act: warning_action = warn_flag.actions[wt as usize];
            if act as ::core::ffi::c_uint > w_unset as ::core::ffi::c_int as ::core::ffi::c_uint {
                fp = variable_buffer_output(fp, &raw mut sp, 1 as size_t);
                sp = ',' as i32 as ::core::ffi::c_char;
                fp = variable_buffer_output(
                    fp,
                    w_name_map[wt as usize],
                    strlen(w_name_map[wt as usize]) as size_t,
                );
                if act as ::core::ffi::c_uint != w_warn as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    fp = variable_buffer_output(
                        variable_buffer_output(
                            fp,
                            b":\0" as *const u8 as *const ::core::ffi::c_char,
                            1 as size_t,
                        ),
                        w_action_map[act as usize],
                        strlen(w_action_map[act as usize]) as size_t,
                    );
                }
            }
            wt += 1;
        }
    }
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn warn_get_vardata(mut data: *mut warning_data) {
    memcpy(
        data as *mut ::core::ffi::c_void,
        &raw mut warn_variable as *const ::core::ffi::c_void,
        ::core::mem::size_of::<warning_data>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn warn_set_vardata(mut data: *const warning_data) {
    memcpy(
        &raw mut warn_variable as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        ::core::mem::size_of::<warning_data>() as size_t,
    );
    set_warnings();
}
