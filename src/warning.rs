extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    fn error(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn fatal(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    static mut stopchar_map: [::core::ffi::c_ushort; 0];
    fn variable_buffer_output(
        ptr: *mut ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
        length: size_t,
    ) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type uintmax_t = ::libc::uintmax_t;
use crate::floc::Floc;

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
pub const INTSTR_LENGTH: usize = 53 * ::core::mem::size_of::<uintmax_t>() / 22 + 3;

const fn cstr(b: &[u8]) -> *const ::core::ffi::c_char {
    b.as_ptr() as *const ::core::ffi::c_char
}

// stopchar_map (defined in misc.c) bit-flags per byte:
// 0x002 = blank, 0x400 = comma, 0x001 = end-of-line, 0x004 = end-of-name.
const STOPMAP_BLANK: ::core::ffi::c_int = 0x2;
const STOPMAP_NAME_END: ::core::ffi::c_int = 0x4;
const STOPMAP_COMMA: ::core::ffi::c_int = 0x400;
const STOPMAP_END: ::core::ffi::c_int = 0x1;

unsafe fn stopmap(c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    *(&raw const stopchar_map as *const ::core::ffi::c_ushort)
        .offset(c as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
}

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
    ::core::ptr::null(),
    cstr(b"ignore\0"),
    cstr(b"warn\0"),
    cstr(b"error\0"),
];
static mut w_name_map: [*const ::core::ffi::c_char; 4] = [
    cstr(b"circular-dep\0"),
    cstr(b"invalid-ref\0"),
    cstr(b"invalid-var\0"),
    cstr(b"undefined-var\0"),
];

/// Resolve the active per-warning action by walking the precedence chain:
/// per-flag → flag-global → per-variable → variable-global → default.
unsafe fn set_warnings() {
    for wt in 0..wt_max as usize {
        warnings[wt] = if warn_flag.actions[wt] != w_unset {
            warn_flag.actions[wt]
        } else if warn_flag.global != w_unset {
            warn_flag.global
        } else if warn_variable.actions[wt] != w_unset {
            warn_variable.actions[wt]
        } else if warn_variable.global != w_unset {
            warn_variable.global
        } else {
            warn_default.actions[wt]
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn warn_init() {
    let zeroed = warning_data {
        global: w_unset,
        actions: [w_unset; 4],
    };
    warn_default = zeroed;
    warn_variable = zeroed;
    warn_flag = zeroed;

    warn_default.global = w_warn;
    warn_default.actions[wt_circular_dep as usize] = w_warn;
    warn_default.actions[wt_invalid_ref as usize] = w_warn;
    warn_default.actions[wt_invalid_var as usize] = w_warn;
    warn_default.actions[wt_undefined_var as usize] = w_ignore;
    set_warnings();
}

unsafe fn init_data(data: *mut warning_data) {
    (*data).global = w_unset;
    for wt in 0..wt_max as usize {
        (*data).actions[wt] = w_unset;
    }
}

/// Match `action[..length]` against the action name table; return w_unset on miss.
unsafe fn decode_warn_action(
    action: *const ::core::ffi::c_char,
    length: size_t,
) -> warning_action {
    for st in (w_ignore as usize)..=(w_error as usize) {
        let candidate = w_action_map[st];
        if length == strlen(candidate) && strncasecmp(action, candidate, length) == 0 {
            return st as warning_action;
        }
    }
    w_unset
}

/// Match `name[..length]` against the warning-name table; return wt_max on miss.
unsafe fn decode_warn_name(
    name: *const ::core::ffi::c_char,
    length: size_t,
) -> warning_type {
    for wt in 0..wt_max as usize {
        let candidate = w_name_map[wt];
        if length == strlen(candidate) && strncasecmp(name, candidate, length) == 0 {
            return wt as warning_type;
        }
    }
    wt_max
}

#[no_mangle]
pub unsafe extern "C" fn decode_warn_actions(
    mut value: *const ::core::ffi::c_char,
    flocp: *const Floc,
) {
    let mut data: *mut warning_data = &raw mut warn_flag;

    // Skip leading blanks/name-stop characters.
    while stopmap(*value) & (STOPMAP_BLANK | STOPMAP_NAME_END) != 0 {
        value = value.offset(1 as ::core::ffi::c_int as isize);
    }
    if !flocp.is_null() {
        data = &raw mut warn_variable;
        if *value == 0 {
            init_data(data);
        }
    }

    while *value != 0 {
        let mut action: warning_action;
        let mut ep: *const ::core::ffi::c_char = value;
        // Walk to the next blank/comma/end.
        while stopmap(*ep) & (STOPMAP_BLANK | STOPMAP_COMMA | STOPMAP_END) == 0 {
            ep = ep.offset(1 as ::core::ffi::c_int as isize);
        }
        let span = ep.offset_from(value) as size_t;
        action = decode_warn_action(value, span);
        if action != w_unset {
            (*data).global = action;
        } else {
            // The token is `name` or `name:action`; split on ':'.
            let mut cp = memchr(value as *const ::core::ffi::c_void, ':' as i32, span)
                as *const ::core::ffi::c_char;
            if cp.is_null() {
                cp = ep;
            }
            let wl = cp.offset_from(value) as ::core::ffi::c_int;
            let type_0 = decode_warn_name(value, wl as size_t);
            let al;
            if cp == ep {
                action = w_warn;
                al = 0;
            } else {
                cp = cp.offset(1 as ::core::ffi::c_int as isize);
                al = ep.offset_from(cp) as ::core::ffi::c_int;
                action = decode_warn_action(cp, al as size_t);
            }
            if type_0 == wt_max {
                if flocp.is_null() {
                    fatal(
                        ::core::ptr::null(),
                        INTSTR_LENGTH + strlen(value),
                        cstr(b"unknown warning '%.*s'\0"),
                        wl,
                        value,
                    );
                }
                error(
                    flocp,
                    INTSTR_LENGTH + strlen(value),
                    cstr(b"unknown warning '%.*s': ignored\0"),
                    wl,
                    value,
                );
            } else if action == w_unset {
                if flocp.is_null() {
                    fatal(
                        ::core::ptr::null(),
                        INTSTR_LENGTH + strlen(cp),
                        cstr(b"unknown warning action '%.*s'\0"),
                        al,
                        cp,
                    );
                }
                error(
                    flocp,
                    INTSTR_LENGTH + strlen(cp),
                    cstr(b"unknown warning action '%.*s': ignored\0"),
                    al,
                    cp,
                );
            } else {
                (*data).actions[type_0 as usize] = action;
            }
        }
        value = ep;
        // Skip the separator(s).
        while stopmap(*value) & (STOPMAP_BLANK | STOPMAP_COMMA) != 0 {
            value = value.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    set_warnings();
}

#[no_mangle]
pub unsafe extern "C" fn encode_warn_flag(
    mut fp: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    // Find the first per-warning override, if any.
    let mut wt: usize = 0;
    while wt < wt_max as usize && warn_flag.actions[wt] == w_unset {
        wt += 1;
    }
    if wt == wt_max as usize && warn_flag.global == w_unset {
        return fp;
    }

    fp = variable_buffer_output(fp, cstr(b" --warn\0"), b" --warn".len());

    if wt == wt_max as usize && warn_flag.global == w_warn {
        // Plain `--warn` is enough; default global is w_warn.
        return fp;
    }

    let mut sp: ::core::ffi::c_char = b'=' as ::core::ffi::c_char;

    if warn_flag.global > w_unset {
        fp = variable_buffer_output(fp, &raw const sp, 1);
        sp = b',' as ::core::ffi::c_char;
        let m = w_action_map[warn_flag.global as usize];
        fp = variable_buffer_output(fp, m, strlen(m));
    }

    if wt != wt_max as usize {
        for wt in 0..wt_max as usize {
            let act = warn_flag.actions[wt];
            if act > w_unset {
                fp = variable_buffer_output(fp, &raw const sp, 1);
                sp = b',' as ::core::ffi::c_char;
                let name = w_name_map[wt];
                fp = variable_buffer_output(fp, name, strlen(name));
                if act != w_warn {
                    let action_name = w_action_map[act as usize];
                    fp = variable_buffer_output(fp, cstr(b":\0"), 1);
                    fp = variable_buffer_output(fp, action_name, strlen(action_name));
                }
            }
        }
    }
    fp
}

#[no_mangle]
pub unsafe extern "C" fn warn_get_vardata(data: *mut warning_data) {
    memcpy(
        data as *mut ::core::ffi::c_void,
        &raw const warn_variable as *const ::core::ffi::c_void,
        ::core::mem::size_of::<warning_data>(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn warn_set_vardata(data: *const warning_data) {
    memcpy(
        &raw mut warn_variable as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        ::core::mem::size_of::<warning_data>(),
    );
    set_warnings();
}
