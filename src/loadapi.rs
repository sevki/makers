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
use crate::read::eval_buffer;

pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
pub type HashTable = crate::hash::HashTable;
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
        // `gmk_floc::lineno` is `c_ulong`, which is 32 bits on some targets
        // (e.g. wasm32) and 64 bits on others (e.g. x86_64) -- this
        // conversion is a no-op on the latter but required on the former.
        #[allow(clippy::useless_conversion)]
        {
            fl.lineno = u64::from((*gfloc).lineno);
        }
        fl.offset = 0;
        flp = &raw mut fl;
    } else {
        flp = ::core::ptr::null_mut::<Floc>();
    }
    let mut eval_input = ::std::ffi::CStr::from_ptr(buffer)
        .to_bytes_with_nul()
        .to_vec();
    // `gmk_eval` defines targets/rules that must land in the live build's file
    // table — not a throwaway context. This C-ABI entry point can't carry the
    // owned `ExecContext`, so reach `main_0`'s through the `CTX_PTR` borrow
    // channel (installed for all of `main_0`, which is on the stack whenever a
    // loaded plugin runs). The variable-buffer save/restore now needs that
    // same `ctx`, so it moves inside the closure alongside `eval_buffer`.
    crate::make_main::with_exec_context(|ctx| unsafe {
        install_variable_buffer(ctx, &raw mut pbuf, &raw mut plen);
        // `gmk_eval` is a C-ABI entry point called from a loaded plugin: there
        // is no Rust frame between here and the plugin to carry a `Result`, so
        // a failed eval bridges through `exit_on_err` (#432 Phase B, #442).
        // The variable buffer is restored first so the bridge does not leave
        // it swapped out.
        let evaluated = eval_buffer(
            ctx,
            eval_input.as_mut_ptr() as *mut ::core::ffi::c_char,
            flp,
        );
        restore_variable_buffer(ctx, pbuf, plen);
        if let Err(e) = evaluated {
            crate::output::exit_on_err(e);
        }
    });
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
#[no_mangle]
pub unsafe extern "C" fn gmk_expand(ref_0: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    with_live_or_default_ctx(|ctx| unsafe {
        // A permanent bridge, not a slice boundary: `gmk_expand` is a
        // `#[no_mangle] extern "C"` plugin entry point whose signature is fixed
        // by the `gnumake.h` ABI, so a rejected expansion has nowhere to
        // propagate and must end the process here (#432 Phase B, #442).
        allocated_expand_string_for_file(ctx, ref_0, ::core::ptr::null_mut::<file>())
            .unwrap_or_else(|e| crate::output::exit_on_err(e))
    })
}
/// Run `f` against `main_0`'s live context, reached through the `CTX_PTR`
/// borrow channel (installed whenever a loaded plugin runs). The `gmk_*`
/// plugin-ABI entry points cannot carry the owned `ExecContext` in their C
/// signatures, and what they run may emit output whose `make[N]:`/
/// directory-trace prefixes read `program`/`starting_directory` off the
/// context. Fall back to a throwaway default context only when none is
/// installed (bare unit tests).
fn with_live_or_default_ctx<R>(f: impl Fn(&crate::execctx::ExecContext) -> R) -> R {
    match crate::make_main::try_with_exec_context(&f) {
        Some(result) => result,
        None => f(&crate::execctx::ExecContext::default()),
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
    with_live_or_default_ctx(|ctx| unsafe {
        // `gmk_add_function` is a C-ABI entry point called from a loaded
        // plugin: there is no Rust frame between here and the plugin to carry
        // a `Result`, so a rejected function definition bridges through
        // `exit_on_err` rather than propagating (#432 Phase B, #442). The five
        // name/arity validations inside `define_new_function` now hand their
        // diagnostic back as a value; this is the one place it still exits.
        if let Err(e) =
            define_new_function(ctx, ctx.reading_file.0.get(), name, min, max, flags, func)
        {
            crate::output::exit_on_err(e);
        }
    });
}

#[cfg(test)]
mod gmk_expand_tests {
    //! `gmk_expand` reaches `main_0`'s live context through the `CTX_PTR`
    //! channel and falls back to a throwaway default context when none is
    //! installed; both arms must expand to the same bytes (#461 review).

    unsafe fn expand_to_string(input: &::core::ffi::CStr) -> String {
        let p = super::gmk_expand(input.as_ptr());
        assert!(!p.is_null());
        let s = ::core::ffi::CStr::from_ptr(p)
            .to_string_lossy()
            .into_owned();
        super::gmk_free(p);
        s
    }

    #[test]
    fn expands_literal_without_installed_context() {
        let _buf_g = crate::expand::VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let _ctx = crate::make_main::install_default_exec_context_for_test();
        // No context installed on this test thread: the fallback arm runs.
        let s = unsafe { expand_to_string(c"plugin-literal") };
        assert_eq!(s, "plugin-literal");
    }

    #[test]
    fn expands_literal_with_installed_context() {
        let _buf_g = crate::expand::VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let _ctx = crate::make_main::install_default_exec_context_for_test();
        let _ctx = crate::make_main::install_default_exec_context_for_test();
        let s = unsafe { expand_to_string(c"plugin-live-ctx") };
        assert_eq!(s, "plugin-live-ctx");
    }
}
