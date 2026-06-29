//! Idiomatic dependency-graph edge types.
//!
//! [`DepNode`] (keyed by [`DepId`]) is the idiomatic replacement for the c2rust
//! `Dep`/`GoalDep` records — no raw pointers, no `c_char`, no `#[repr(C)]`. The
//! legacy C-ABI structs still live in `file.rs` alongside the other c2rust FFI
//! types until the `*mut`-to-handle swap deletes them; this module is the
//! pointer-free home the dependency graph is migrating to.

use crate::content_hash::ContentHash;
use crate::file::{FileId, HASH_SIZE};

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
