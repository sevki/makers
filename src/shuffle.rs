use ::c2rust_bitfields;
use libc::{free, sprintf, strcasecmp, strcpy};
extern "C" {
    pub type variable_set_list;
    pub type commands;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn fatal(flocp: *const floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn make_toui(
        _: *const ::core::ffi::c_char,
        _: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_uint;
    fn make_seed(_: ::core::ffi::c_uint);
    fn make_rand() -> ::core::ffi::c_uint;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    static mut not_parallel: ::core::ffi::c_int;
}
pub type size_t = usize;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct dep {
    pub next: *mut dep,
    pub name: *const ::core::ffi::c_char,
    pub file: *mut file,
    pub shuf: *mut dep,
    pub stem: *const ::core::ffi::c_char,
    #[bitfield(name = "flags", ty = "::core::ffi::c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(
        name = "need_2nd_expansion",
        ty = "::core::ffi::c_uint",
        bits = "11..=11"
    )]
    #[bitfield(
        name = "ignore_automatic_vars",
        ty = "::core::ffi::c_uint",
        bits = "12..=12"
    )]
    #[bitfield(name = "is_explicit", ty = "::core::ffi::c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "::core::ffi::c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here:
        [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
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
pub struct C2RustUnnamed {
    pub mode: shuffle_mode,
    pub seed: ::core::ffi::c_uint,
    pub shuffler: Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, size_t) -> ()>,
    pub strval: [::core::ffi::c_char; 23],
}
pub type shuffle_mode = ::core::ffi::c_uint;
pub const sm_identity: shuffle_mode = 3;
pub const sm_reverse: shuffle_mode = 2;
pub const sm_random: shuffle_mode = 1;
pub const sm_none: shuffle_mode = 0;

static mut config: C2RustUnnamed = C2RustUnnamed {
    mode: sm_none,
    seed: 0,
    shuffler: None,
    strval: [0; 23],
};

#[no_mangle]
pub unsafe extern "C" fn shuffle_get_mode() -> *const ::core::ffi::c_char {
    if config.strval[0] == 0 {
        ::core::ptr::null()
    } else {
        &raw const config.strval as *const ::core::ffi::c_char
    }
}

// Pre-1.77 nightly: no `c"..."` literals, so build C strings as byte slices
// with explicit NUL and reinterpret the pointer.
const fn cstr(b: &[u8]) -> *const ::core::ffi::c_char {
    b.as_ptr() as *const ::core::ffi::c_char
}

#[no_mangle]
pub unsafe extern "C" fn shuffle_set_mode(cmdarg: *const ::core::ffi::c_char) {
    let strval_ptr = &raw mut config.strval as *mut ::core::ffi::c_char;

    if strcasecmp(cmdarg, cstr(b"reverse\0")) == 0 {
        config.mode = sm_reverse;
        config.shuffler = Some(reverse_shuffle_array);
        strcpy(strval_ptr, cstr(b"reverse\0"));
    } else if strcasecmp(cmdarg, cstr(b"identity\0")) == 0 {
        config.mode = sm_identity;
        config.shuffler = Some(identity_shuffle_array);
        strcpy(strval_ptr, cstr(b"identity\0"));
    } else if strcasecmp(cmdarg, cstr(b"none\0")) == 0 {
        config.mode = sm_none;
        config.shuffler = None;
        config.strval[0] = 0;
    } else {
        if strcasecmp(cmdarg, cstr(b"random\0")) == 0 {
            config.seed = make_rand();
        } else {
            let mut err: *const ::core::ffi::c_char = ::core::ptr::null();
            config.seed = make_toui(cmdarg, &raw mut err);
            if !err.is_null() {
                fatal(
                    ::core::ptr::null(),
                    strlen(err) + strlen(cmdarg),
                    cstr(b"invalid shuffle mode: %s: '%s'\0"),
                    err,
                    cmdarg,
                );
            }
        }
        config.mode = sm_random;
        config.shuffler = Some(random_shuffle_array);
        sprintf(strval_ptr, cstr(b"%u\0"), config.seed);
    }
}

/// Fisher-Yates shuffle. The `extern "C"` signature is fixed because this is
/// stored as a function pointer in `config.shuffler`; the body operates on a
/// safe slice view to drop the pointer arithmetic.
unsafe extern "C" fn random_shuffle_array(a: *mut *mut ::core::ffi::c_void, len: size_t) {
    if len <= 1 {
        return;
    }
    let slice = ::core::slice::from_raw_parts_mut(a, len);
    for i in (1..len).rev() {
        let j = (make_rand() as size_t) % (i + 1);
        if i != j {
            slice.swap(i, j);
        }
    }
}

unsafe extern "C" fn reverse_shuffle_array(a: *mut *mut ::core::ffi::c_void, len: size_t) {
    let slice = ::core::slice::from_raw_parts_mut(a, len);
    for i in 0..len / 2 {
        slice.swap(i, len - 1 - i);
    }
}

unsafe extern "C" fn identity_shuffle_array(_a: *mut *mut ::core::ffi::c_void, _len: size_t) {}

/// Walk the deps linked list, shuffle the order, and write the new order back
/// via the `shuf` field on each node.
unsafe fn shuffle_deps(deps: *mut dep) {
    // Count deps; bail out if any has wait_here (those preserve order).
    let mut ndeps: size_t = 0;
    let mut d = deps;
    while !d.is_null() {
        if (*d).wait_here() != 0 {
            return;
        }
        ndeps += 1;
        d = (*d).next;
    }
    if ndeps == 0 {
        return;
    }

    // Pack pointers into a contiguous array, shuffle, then write back.
    let da = xmalloc(::core::mem::size_of::<*mut dep>() * ndeps) as *mut *mut ::core::ffi::c_void;
    let slots = ::core::slice::from_raw_parts_mut(da, ndeps);

    d = deps;
    for slot in slots.iter_mut() {
        *slot = d as *mut ::core::ffi::c_void;
        d = (*d).next;
    }

    config.shuffler.expect("non-null function pointer")(da, ndeps);

    d = deps;
    for slot in slots.iter() {
        (*d).shuf = *slot as *mut dep;
        d = (*d).next;
    }
    free(da as *mut ::core::ffi::c_void);
}

unsafe fn shuffle_file_deps_recursive(f: *mut file) {
    if f.is_null() || (*f).was_shuffled() != 0 {
        return;
    }
    (*f).set_was_shuffled(1);
    shuffle_deps((*f).deps);
    let mut d = (*f).deps;
    while !d.is_null() {
        shuffle_file_deps_recursive((*d).file);
        d = (*d).next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn shuffle_deps_recursive(deps: *mut dep) {
    if config.mode == sm_none || not_parallel != 0 {
        return;
    }
    if config.mode == sm_random {
        make_seed(config.seed);
    }
    shuffle_deps(deps);
    let mut d = deps;
    while !d.is_null() {
        shuffle_file_deps_recursive((*d).file);
        d = (*d).next;
    }
}
