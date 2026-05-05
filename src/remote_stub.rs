extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type __pid_t = ::core::ffi::c_int;
pub type pid_t = __pid_t;
pub const ECHILD: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[no_mangle]
pub static mut remote_description: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn remote_setup() {}
#[no_mangle]
pub unsafe extern "C" fn remote_cleanup() {}
#[no_mangle]
pub unsafe extern "C" fn start_remote_job_p(mut first_p: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn start_remote_job(
    mut argv: *mut *mut ::core::ffi::c_char,
    mut envp: *mut *mut ::core::ffi::c_char,
    mut stdin_fd: ::core::ffi::c_int,
    mut is_remote: *mut ::core::ffi::c_int,
    mut id_ptr: *mut pid_t,
    mut used_stdin: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn remote_status(
    mut exit_code_ptr: *mut ::core::ffi::c_int,
    mut signal_ptr: *mut ::core::ffi::c_int,
    mut coredump_ptr: *mut ::core::ffi::c_int,
    mut block: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *__errno_location() = ECHILD;
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn block_remote_children() {}
#[no_mangle]
pub unsafe extern "C" fn unblock_remote_children() {}
#[no_mangle]
pub unsafe extern "C" fn remote_kill(
    mut id: pid_t,
    mut sig: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return -(1 as ::core::ffi::c_int);
}
