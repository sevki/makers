use libc::{__errno_location};
pub type __pid_t = ::core::ffi::c_int;
pub type pid_t = __pid_t;
pub const ECHILD: ::core::ffi::c_int = 10;
#[no_mangle]
pub static mut remote_description: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn remote_setup() {}
#[no_mangle]
pub unsafe extern "C" fn remote_cleanup() {}
#[no_mangle]
pub unsafe extern "C" fn start_remote_job_p(mut _first_p: ::core::ffi::c_int) -> ::core::ffi::c_int {
    0
}
#[no_mangle]
pub unsafe extern "C" fn start_remote_job(
    mut _argv: *mut *mut ::core::ffi::c_char,
    mut _envp: *mut *mut ::core::ffi::c_char,
    mut _stdin_fd: ::core::ffi::c_int,
    mut _is_remote: *mut ::core::ffi::c_int,
    mut _id_ptr: *mut pid_t,
    mut _used_stdin: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    -(1 as ::core::ffi::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn remote_status(
    mut _exit_code_ptr: *mut ::core::ffi::c_int,
    mut _signal_ptr: *mut ::core::ffi::c_int,
    mut _coredump_ptr: *mut ::core::ffi::c_int,
    mut _block: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *__errno_location() = ECHILD;
    -(1 as ::core::ffi::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn block_remote_children() {}
#[no_mangle]
pub unsafe extern "C" fn unblock_remote_children() {}
#[no_mangle]
pub unsafe extern "C" fn remote_kill(
    mut _id: pid_t,
    mut _sig: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    -(1 as ::core::ffi::c_int)
}
