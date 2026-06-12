use libc::__errno_location;

pub use crate::ffi_types::pid_t;

pub const ECHILD: ::core::ffi::c_int = 10;
pub static mut remote_description: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;

pub trait RemoteBackend {
    fn setup(&self) {}
    fn cleanup(&self) {}
    fn can_start_job(&self, first: bool) -> bool;
    fn start_job(
        &self,
        argv: *mut *mut ::core::ffi::c_char,
        envp: *mut *mut ::core::ffi::c_char,
        stdin_fd: ::core::ffi::c_int,
        is_remote: *mut ::core::ffi::c_int,
        id_ptr: *mut pid_t,
        used_stdin: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn status(
        &self,
        exit_code: *mut ::core::ffi::c_int,
        signal: *mut ::core::ffi::c_int,
        coredump: *mut ::core::ffi::c_int,
        block: bool,
    ) -> ::core::ffi::c_int;
    fn block_children(&self) {}
    fn unblock_children(&self) {}
    fn kill(&self, id: pid_t, sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
        _stdin_fd: ::core::ffi::c_int,
        _is_remote: *mut ::core::ffi::c_int,
        _id_ptr: *mut pid_t,
        _used_stdin: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        -1
    }

    fn status(
        &self,
        _exit_code: *mut ::core::ffi::c_int,
        _signal: *mut ::core::ffi::c_int,
        _coredump: *mut ::core::ffi::c_int,
        _block: bool,
    ) -> ::core::ffi::c_int {
        unsafe {
            *__errno_location() = ECHILD;
        }
        -1
    }

    fn kill(&self, _id: pid_t, _sig: ::core::ffi::c_int) -> ::core::ffi::c_int {
        -1
    }
}

static REMOTE: StubRemote = StubRemote;

pub fn remote_setup() {
    REMOTE.setup();
}

pub fn remote_cleanup() {
    REMOTE.cleanup();
}

pub fn start_remote_job_p(first_p: ::core::ffi::c_int) -> ::core::ffi::c_int {
    REMOTE.can_start_job(first_p != 0) as ::core::ffi::c_int
}

pub fn start_remote_job(
    argv: *mut *mut ::core::ffi::c_char,
    envp: *mut *mut ::core::ffi::c_char,
    stdin_fd: ::core::ffi::c_int,
    is_remote: *mut ::core::ffi::c_int,
    id_ptr: *mut pid_t,
    used_stdin: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    REMOTE.start_job(argv, envp, stdin_fd, is_remote, id_ptr, used_stdin)
}

pub fn remote_status(
    exit_code: *mut ::core::ffi::c_int,
    signal: *mut ::core::ffi::c_int,
    coredump: *mut ::core::ffi::c_int,
    block: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    REMOTE.status(exit_code, signal, coredump, block != 0)
}

pub fn block_remote_children() {
    REMOTE.block_children();
}

pub fn unblock_remote_children() {
    REMOTE.unblock_children();
}

pub fn remote_kill(id: pid_t, sig: ::core::ffi::c_int) -> ::core::ffi::c_int {
    REMOTE.kill(id, sig)
}
