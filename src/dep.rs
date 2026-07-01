//! Idiomatic dependency-graph edge types.
//!
//! Both representations of a dependency edge live here. At the top are the
//! idiomatic, pointer-free forms the dependency graph is migrating to —
//! [`DepNode`] (keyed by [`DepId`]) and [`GoalDepNode`]: owned `String`s and
//! `FileId` handles, no raw pointers, no `c_char`, no `#[repr(C)]`. At the
//! bottom are the legacy c2rust [`Dep`]/[`GoalDep`] records they replace — the
//! raw-pointer `#[repr(C)]` structs (`*mut`, `c_char`), which `file.rs`
//! re-exports for compatibility until the `*mut`-to-handle swap deletes them.

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

/// Build a fresh, empty dependency edge — the idiomatic replacement for the
/// c2rust `alloc_dep` (which `xcalloc`'d a zeroed `Dep`). All flags clear, no
/// resolved target, empty name; callers fill in what they need.
#[inline]
pub(crate) fn alloc_dep() -> DepNode {
    DepNode {
        name: String::new(),
        file: None,
        shuf: None,
        stem: None,
        flags: DepFlags::empty(),
        changed: false,
        ignore_mtime: false,
        static_pattern: false,
        needs_second_expansion: false,
        ignore_automatic_vars: false,
        is_explicit: false,
        wait_here: false,
    }
}

/// Build a fresh, empty goal — the idiomatic replacement for the c2rust
/// `alloc_goaldep` (a zeroed `GoalDep`): an empty [`DepNode`] edge plus zeroed
/// error/location bookkeeping.
#[inline]
pub(crate) fn alloc_goaldep() -> GoalDepNode {
    GoalDepNode {
        dep: alloc_dep(),
        error: 0,
        defined_in: None,
        lineno: 0,
        offset: 0,
    }
}

/// Copy a single dependency edge. The idiomatic [`DepNode`] owns its fields
/// (the `name: String` is cloned, the linked-target `file: Option<FileId>` is a
/// `Copy` handle), so a value clone is the whole copy — there is no `next` link
/// to clear and no second-expansion name aliasing to break.
pub fn copy_dep(d: &DepNode) -> DepNode {
    d.clone()
}

/// Copy a whole prerequisite list as an owned `Vec<DepNode>` clone — the
/// pointer-free replacement for following and duplicating a `*mut Dep` chain.
pub fn copy_dep_chain(d: &[DepNode]) -> Vec<DepNode> {
    d.to_vec()
}

/// Copy a single goal edge as an owned [`GoalDepNode`] value clone.
pub fn copy_goaldep(d: &GoalDepNode) -> GoalDepNode {
    d.clone()
}

/// Copy a whole goal list as an owned `Vec<GoalDepNode>` clone — the
/// pointer-free replacement for duplicating a `*mut GoalDep` chain.
pub fn copy_goal_chain(d: &[GoalDepNode]) -> Vec<GoalDepNode> {
    d.to_vec()
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
