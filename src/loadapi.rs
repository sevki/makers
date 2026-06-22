use libc::free;

pub use crate::ffi_types::{size_t, uintmax_t};
use crate::file::{Dep, File, VariableSet, VariableSetList};
use crate::misc::xmalloc;
extern "C" {
    pub type commands;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gmk_floc {
    pub filenm: *const ::core::ffi::c_char,
    pub lineno: ::core::ffi::c_ulong,
}
pub type gmk_func_ptr = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        ::core::ffi::c_uint,
        *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char,
>;
use crate::expand::{
    allocated_expand_string_for_file, install_variable_buffer, restore_variable_buffer,
};
use crate::floc::Floc;
use crate::function::define_new_function;
use crate::read::{eval_buffer, reading_file};

pub type file = File;
pub type cmd_state = ::core::ffi::c_uint;
pub const cs_finished: cmd_state = 3;
pub const cs_running: cmd_state = 2;
pub const cs_deps_running: cmd_state = 1;
pub const cs_not_started: cmd_state = 0;
pub type update_status = ::core::ffi::c_uint;
pub type update_status_0 = u32;
pub const us_failed: update_status_0 = 3;
pub const us_question: update_status_0 = 2;
pub const us_none: update_status_0 = 1;
pub const us_success: update_status_0 = 0;
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
pub type hash_table = crate::hash::hash_table;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
pub type dep = Dep;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
#[no_mangle]
pub unsafe extern "C" fn gmk_alloc(len: ::core::ffi::c_uint) -> *mut ::core::ffi::c_char {
    xmalloc(len as size_t) as *mut ::core::ffi::c_char
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
#[no_mangle]
pub unsafe extern "C" fn gmk_free(s: *mut ::core::ffi::c_char) {
    free(s as *mut ::core::ffi::c_void);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
#[no_mangle]
pub unsafe extern "C" fn gmk_eval(buffer: *const ::core::ffi::c_char, gfloc: *const gmk_floc) {
    let mut pbuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut plen: size_t = 0;
    let mut fl: Floc = Floc {
        filenm: ::core::ptr::null::<::core::ffi::c_char>(),
        lineno: 0,
        offset: 0,
    };
    let flp: *mut Floc;
    if !gfloc.is_null() {
        fl.filenm = (*gfloc).filenm;
        fl.lineno = (*gfloc).lineno;
        fl.offset = 0;
        flp = &raw mut fl;
    } else {
        flp = ::core::ptr::null_mut::<Floc>();
    }
    install_variable_buffer(&raw mut pbuf, &raw mut plen);
    let mut eval_input = ::std::ffi::CStr::from_ptr(buffer)
        .to_bytes_with_nul()
        .to_vec();
    // This is a `gmk_*` plugin-ABI entry point invoked from loaded C objects;
    // its C-ABI signature cannot carry the owned `ExecContext`, and there is
    // deliberately no global to read it from. The only use of `ctx` in the
    // callees is the `make[N]:` message prefix, which is cosmetic here, so we
    // hand them a default (top-level) context.
    let ctx = crate::execctx::ExecContext::default();
    eval_buffer(
        &ctx,
        eval_input.as_mut_ptr() as *mut ::core::ffi::c_char,
        flp,
    );
    restore_variable_buffer(pbuf, plen);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
#[no_mangle]
pub unsafe extern "C" fn gmk_expand(ref_0: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    // `gmk_*` plugin-ABI entry point: its C-ABI signature cannot carry the
    // owned `ExecContext` and there is deliberately no global for it. Hand the
    // callee a default (top-level) context (only the cosmetic `make[N]:` prefix
    // depends on it).
    let ctx = crate::execctx::ExecContext::default();
    allocated_expand_string_for_file(&ctx, ref_0, ::core::ptr::null_mut::<file>())
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
#[no_mangle]
pub unsafe extern "C" fn gmk_add_function(
    name: *const ::core::ffi::c_char,
    func: gmk_func_ptr,
    min: ::core::ffi::c_uint,
    max: ::core::ffi::c_uint,
    flags: ::core::ffi::c_uint,
) {
    // `gmk_*` plugin-ABI entry point: its C-ABI signature cannot carry the
    // owned `ExecContext` and there is deliberately no global for it. Hand the
    // callee a default (top-level) context (only the cosmetic `make[N]:` prefix
    // depends on it).
    let ctx = crate::execctx::ExecContext::default();
    define_new_function(&ctx, reading_file, name, min, max, flags, func);
}
