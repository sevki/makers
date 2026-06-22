extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
}

pub use crate::ffi_types::{__off64_t, __off_t, size_t};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _io_read_ptr: *mut ::core::ffi::c_char,
    pub _io_read_end: *mut ::core::ffi::c_char,
    pub _io_read_base: *mut ::core::ffi::c_char,
    pub _io_write_base: *mut ::core::ffi::c_char,
    pub _io_write_ptr: *mut ::core::ffi::c_char,
    pub _io_write_end: *mut ::core::ffi::c_char,
    pub _io_buf_base: *mut ::core::ffi::c_char,
    pub _io_buf_end: *mut ::core::ffi::c_char,
    pub _io_save_base: *mut ::core::ffi::c_char,
    pub _io_backup_base: *mut ::core::ffi::c_char,
    pub _io_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [::core::ffi::c_char; 20],
}

pub type FILE = _IO_FILE;
