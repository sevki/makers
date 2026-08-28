use make_sys::{build_result, entry::main_0};

fn main() {
    // The single process-exit point (Phase B, #432): the library reports how
    // the run ended; only this shim turns that into an exit status.
    std::process::exit({
        #[cfg(unix)]
        use std::os::unix::ffi::OsStrExt;
        #[cfg(target_os = "wasi")]
        use std::os::wasi::ffi::OsStrExt;
        let mut args_strings: Vec<Vec<u8>> = ::std::env::args_os()
            .map(|arg| {
                ::std::ffi::CString::new(arg.as_bytes())
                    .expect("Failed to convert argument into CString.")
                    .into_bytes_with_nul()
            })
            .collect();
        let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
            .iter_mut()
            .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
            .chain(::core::iter::once(::core::ptr::null_mut()))
            .collect();
        // Own the env strings like the args above so they are reclaimed when this
        // function returns — `main_0` copies what it keeps (`xstrndup`/`xstrdup`
        // in `define_variable_in_set`), and libc `environ` never aliases these.
        let mut vars_strings: Vec<Vec<u8>> = ::std::env::vars()
            .map(|(var_name, var_value)| {
                ::std::ffi::CString::new(format!("{}={}", var_name, var_value))
                    .expect("Failed to convert environment variable into CString.")
                    .into_bytes_with_nul()
            })
            .collect();
        let mut vars: Vec<*mut ::core::ffi::c_char> = vars_strings
            .iter_mut()
            .map(|var| var.as_mut_ptr() as *mut ::core::ffi::c_char)
            .chain(::core::iter::once(::core::ptr::null_mut()))
            .collect();
        let result = unsafe {
            main_0(
                (args_ptrs.len() - 1) as i32,
                args_ptrs.as_mut_ptr(),
                vars.as_mut_ptr(),
            )
        };
        build_result::exit_code(result)
    });
}
