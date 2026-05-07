#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(raw_ref_op)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

/// Idiomatic-Rust replacement for the c2rust-translated `__assert_fail(...)`
/// call. Takes the assertion-text expression that c2rust emitted (a
/// `b"...\0" as *const u8 as *const c_char` chain), prints the message in
/// glibc's format using the Rust source location, and aborts.
#[macro_export]
macro_rules! make_assert_fail {
    ($text:expr) => {{
        let text = unsafe { ::core::ffi::CStr::from_ptr($text) };
        ::std::eprintln!(
            "{}:{}: Assertion `{}' failed.",
            ::core::file!(),
            ::core::line!(),
            text.to_string_lossy(),
        );
        ::std::process::abort();
    }};
}

pub mod ar;
pub mod arscan;
pub mod commands;
pub mod default;
pub mod dir;
pub mod expand;
pub mod stdio;
pub mod file;
pub mod floc;
pub mod function;
pub mod getopt;
pub mod getopt1;
pub mod guile;
pub mod hash;
pub mod implicit;
pub mod job;
pub mod load;
pub mod loadapi;
pub mod main;
pub mod misc;
pub mod output;
pub mod posixos;
pub mod read;
pub mod remake;
pub mod remote_stub;
pub mod rule;
pub mod shuffle;
pub mod signame;
pub mod strcache;
pub mod sys_stat;
pub mod variable;
pub mod version;
pub mod vpath;
pub mod warning;
