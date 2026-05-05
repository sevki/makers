
use libc::{free, sprintf, strcasecmp, strcpy};
use crate::file::{Dep, File};
extern "C" {
    pub type variable_set_list;
    pub type commands;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn fatal(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
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
pub type file = File;
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
pub type dep = Dep;
use crate::floc::Floc;

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
