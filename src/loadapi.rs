pub use crate::file::{CommandState, UpdateStatus};
use libc::free;

pub use crate::ffi_types::{size_t, uintmax_t};
use crate::file::{file, VariableSet, VariableSetList};
use crate::misc::xmalloc;
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

pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
pub type hash_table = crate::hash::hash_table;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
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
    // `gmk_eval` defines targets/rules that must land in the live build's file
    // table — not a throwaway context. This C-ABI entry point can't carry the
    // owned `ExecContext`, so reach `main_0`'s through the `CTX_PTR` borrow
    // channel (installed for all of `main_0`, which is on the stack whenever a
    // loaded plugin runs).
    crate::make_main::with_exec_context(|ctx| unsafe {
        eval_buffer(
            ctx,
            eval_input.as_mut_ptr() as *mut ::core::ffi::c_char,
            flp,
        );
    });
    restore_variable_buffer(pbuf, plen);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
#[no_mangle]
pub unsafe extern "C" fn gmk_expand(ref_0: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    // `gmk_*` plugin-ABI entry point: its C-ABI signature cannot carry the
    // owned `ExecContext`, so reach `main_0`'s live one through the `CTX_PTR`
    // borrow channel (installed whenever a loaded plugin runs) — the expansion
    // may emit output whose `make[N]:`/directory-trace prefixes read
    // `program`/`starting_directory` off the context. Fall back to a default
    // context only when none is installed (bare unit tests).
    match crate::make_main::try_with_exec_context(|ctx| unsafe {
        allocated_expand_string_for_file(ctx, ref_0, ::core::ptr::null_mut::<file>())
    }) {
        Some(expansion) => expansion,
        None => {
            let ctx = crate::execctx::ExecContext::default();
            allocated_expand_string_for_file(&ctx, ref_0, ::core::ptr::null_mut::<file>())
        }
    }
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
    // owned `ExecContext`, so reach `main_0`'s live one through the `CTX_PTR`
    // borrow channel, falling back to a default context only when none is
    // installed (bare unit tests).
    let defined = crate::make_main::try_with_exec_context(|ctx| unsafe {
        define_new_function(ctx, reading_file, name, min, max, flags, func);
    });
    if defined.is_none() {
        let ctx = crate::execctx::ExecContext::default();
        define_new_function(&ctx, reading_file, name, min, max, flags, func);
    }
}

#[cfg(test)]
mod gmk_expand_tests {
    //! `gmk_expand` reaches `main_0`'s live context through the `CTX_PTR`
    //! channel and falls back to a throwaway default context when none is
    //! installed; both arms must expand to the same bytes (#461 review).

    unsafe fn expand_to_string(input: &::core::ffi::CStr) -> String {
        let p = super::gmk_expand(input.as_ptr());
        assert!(!p.is_null());
        let s = ::core::ffi::CStr::from_ptr(p).to_string_lossy().into_owned();
        super::gmk_free(p);
        s
    }

    #[test]
    fn expands_literal_without_installed_context() {
        let _buf_g = crate::expand::VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        crate::make_main::install_default_options_for_test();
        // No context installed on this test thread: the fallback arm runs.
        let s = unsafe { expand_to_string(c"plugin-literal") };
        assert_eq!(s, "plugin-literal");
    }

    #[test]
    fn expands_literal_with_installed_context() {
        let _buf_g = crate::expand::VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        crate::make_main::install_default_options_for_test();
        crate::make_main::install_default_exec_context_for_test();
        let s = unsafe { expand_to_string(c"plugin-live-ctx") };
        assert_eq!(s, "plugin-live-ctx");
    }
}
