#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![feature(extern_types)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
extern crate self as make_sys;

pub mod ar;
pub mod arscan;
pub mod build_result;
pub mod commands;
pub mod content_hash;
pub mod default;
pub mod dep;
pub mod depgraph;
pub mod dir;
pub mod execctx;
pub mod expand;
pub mod ffi_types;
pub mod file;
pub mod findprog;
pub mod floc;
pub mod function;
pub mod getopt;
pub mod getopt1;
pub mod guile;
pub mod hash;
pub mod id_wireformat;
pub mod implicit;
pub mod job;
pub mod load;
pub mod loadapi;
#[path = "main.rs"]
pub mod make_main;
pub mod makedb;
pub mod misc;
pub mod output;
pub mod parser;
pub mod posixos;
pub mod read;
pub mod recipe;
pub mod remake;
pub mod remote_stub;
pub mod rule;
pub mod shuffle;
pub mod signame;
pub mod stdio;
pub mod strcache;
pub mod sys_stat;
pub mod target_var;
pub mod tenant;
pub mod variable;
pub mod version;
pub mod vpath;
pub mod warning;
