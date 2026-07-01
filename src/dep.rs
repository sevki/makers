//! Idiomatic dependency-graph edge types.
//!
//! [`DepNode`] (keyed by [`DepId`]) is the idiomatic replacement for the c2rust
//! [`Dep`]/[`GoalDep`] records — no raw pointers, no `c_char`, no `#[repr(C)]`.
//! Both live here now: the legacy `#[repr(C)]` structs at the bottom of the
//! module and the pointer-free forms the dependency graph is migrating to at the
//! top. `file.rs` re-exports `Dep`/`GoalDep` for compatibility until the
//! `*mut`-to-handle swap deletes them.

use crate::content_hash::ContentHash;
use crate::file::{File, FileId, HASH_SIZE};
use crate::floc::Floc;

/// Idiomatic Rust dep edge for the new dependency graph layer.
/// Replaces `Dep` once all FFI bodies have been migrated.
#[derive(Debug, Clone, ContentHash)]
pub struct DepNode {
    pub name: String,
    pub file: Option<FileId>,
    pub shuf: Option<DepId>,
    pub stem: Option<String>,
    pub flags: DepFlags,
    pub changed: bool,
    pub ignore_mtime: bool,
    pub static_pattern: bool,
    pub needs_second_expansion: bool,
    pub ignore_automatic_vars: bool,
    pub is_explicit: bool,
    pub wait_here: bool,
}

// Stable identity for a dep edge: content-hash of the full (immutable) `DepNode`.
crate::id_wireformat!(DepId[HASH_SIZE] <- DepNode);

bitflags::bitflags! {
    /// Goal-dep resolution flags — the idiomatic form of the c2rust
    /// `Dep`/`GoalDep` `flags: c_uint` field. Bit values match the `RM_*`
    /// constants (`main.rs`/`read.rs`) so the two representations round-trip:
    /// the field only ever carries these makefile-reading goal flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct DepFlags: u32 {
        /// `RM_NO_DEFAULT_GOAL` — this goal must not become the default goal.
        const NO_DEFAULT_GOAL = 1 << 0;
        /// `RM_INCLUDED` — the goal came from an `include`d makefile.
        const INCLUDED = 1 << 1;
        /// `RM_DONTCARE` — a failure to remake this goal is not fatal.
        const DONTCARE = 1 << 2;
        /// `RM_NO_TILDE` — do not expand a leading `~` in the goal name.
        const NO_TILDE = 1 << 3;
    }
}

/// A goal: a top-level target make was asked to build — the idiomatic,
/// pointer-free replacement for the c2rust `GoalDep`. A goal is a [`DepNode`]
/// edge from "the command line" (or an `include` directive) to a target, plus
/// error/location bookkeeping. The former `*mut File`/`*const c_char` fields are
/// gone: the target is `dep.file: Option<FileId>` and the name is owned, and the
/// source location is carried inline (the former `floc`, sans `c_char`).
#[derive(Debug, Clone)]
pub struct GoalDepNode {
    /// The dependency edge itself (name, target `FileId`, flags, …).
    pub dep: DepNode,
    /// `errno` captured when the goal's makefile could not be read (the former
    /// `GoalDep::error`); `0` when there was no error.
    pub error: i32,
    /// Source makefile the goal was read from (raw bytes; `None` if synthetic —
    /// the former null `floc.filenm`).
    pub defined_in: Option<Vec<u8>>,
    /// 1-based line number of the goal's definition (`floc.lineno`).
    pub lineno: u64,
    /// Byte offset within the line (`floc.offset`).
    pub offset: u64,
}

/// Legacy c2rust dependency-edge record: a raw-pointer linked list. The
/// idiomatic, pointer-free replacement is [`DepNode`]; this `#[repr(C)]` struct
/// stays only until the `*mut`-to-handle swap removes the last `*mut Dep` site.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dep {
    pub next: *mut Dep,
    pub name: *const ::core::ffi::c_char,
    pub file: *mut File,
    pub shuf: *mut Dep,
    pub stem: *const ::core::ffi::c_char,
    pub flags: ::core::ffi::c_uint,
    pub changed: bool,
    pub ignore_mtime: bool,
    pub staticpattern: bool,
    pub need_2nd_expansion: bool,
    pub ignore_automatic_vars: bool,
    pub is_explicit: bool,
    pub wait_here: bool,
}
impl Default for Dep {
    fn default() -> Self {
        Dep {
            next: ::core::ptr::null_mut(),
            name: ::core::ptr::null(),
            file: ::core::ptr::null_mut(),
            shuf: ::core::ptr::null_mut(),
            stem: ::core::ptr::null(),
            flags: 0,
            changed: false,
            ignore_mtime: false,
            staticpattern: false,
            need_2nd_expansion: false,
            ignore_automatic_vars: false,
            is_explicit: false,
            wait_here: false,
        }
    }
}

/// A goal: a top-level target make was asked to build, with error/location
/// tracking. Legacy c2rust C-ABI record (mirrors `Dep` plus bookkeeping); kept
/// until the `*mut`-to-handle swap removes the last `*mut GoalDep` site. The
/// pointer-free replacement is [`GoalDepNode`].
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GoalDep {
    pub next: *mut GoalDep,
    pub name: *const ::core::ffi::c_char,
    pub file: *mut File,
    pub shuf: *mut GoalDep,
    pub stem: *const ::core::ffi::c_char,
    pub flags: ::core::ffi::c_uint,
    pub changed: bool,
    pub ignore_mtime: bool,
    pub staticpattern: bool,
    pub need_2nd_expansion: bool,
    pub ignore_automatic_vars: bool,
    pub is_explicit: bool,
    pub wait_here: bool,
    pub error: ::core::ffi::c_int,
    pub floc: Floc,
}
impl Default for GoalDep {
    fn default() -> Self {
        GoalDep {
            next: ::core::ptr::null_mut(),
            name: ::core::ptr::null(),
            file: ::core::ptr::null_mut(),
            shuf: ::core::ptr::null_mut(),
            stem: ::core::ptr::null(),
            flags: 0,
            changed: false,
            ignore_mtime: false,
            staticpattern: false,
            need_2nd_expansion: false,
            ignore_automatic_vars: false,
            is_explicit: false,
            wait_here: false,
            error: 0,
            floc: Floc {
                filenm: ::core::ptr::null(),
                lineno: 0,
                offset: 0,
            },
        }
    }
}
