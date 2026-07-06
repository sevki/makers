use libc::__errno_location;

pub use crate::ffi_types::pid_t;

pub const ECHILD: i32 = 10;

pub trait RemoteBackend {
    fn setup(&self) {}
    fn cleanup(&self) {}
    /// The backend's description string, printed by `-v`. Callers convert
    /// to a raw pointer only at the FFI boundary (e.g. as a
    /// `printf`/`fprintf` vararg).
    fn description(&self) -> Option<&'static ::core::ffi::CStr> {
        None
    }
    fn can_start_job(&self, first: bool) -> bool;
    fn start_job(
        &self,
        argv: *mut *mut ::core::ffi::c_char,
        envp: *mut *mut ::core::ffi::c_char,
        stdin_fd: i32,
        is_remote: *mut i32,
        id_ptr: *mut pid_t,
        used_stdin: *mut i32,
    ) -> i32;
    fn status(&self, exit_code: *mut i32, signal: *mut i32, coredump: *mut i32, block: bool)
        -> i32;
    fn block_children(&self) {}
    fn unblock_children(&self) {}
    fn kill(&self, id: pid_t, sig: i32) -> i32;
}

pub struct StubRemote;

impl RemoteBackend for StubRemote {
    fn can_start_job(&self, _first: bool) -> bool {
        false
    }

    fn start_job(
        &self,
        _argv: *mut *mut ::core::ffi::c_char,
        _envp: *mut *mut ::core::ffi::c_char,
        _stdin_fd: i32,
        _is_remote: *mut i32,
        _id_ptr: *mut pid_t,
        _used_stdin: *mut i32,
    ) -> i32 {
        -1
    }

    fn status(
        &self,
        _exit_code: *mut i32,
        _signal: *mut i32,
        _coredump: *mut i32,
        _block: bool,
    ) -> i32 {
        unsafe {
            *__errno_location() = ECHILD;
        }
        -1
    }

    fn kill(&self, _id: pid_t, _sig: i32) -> i32 {
        -1
    }
}
