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
/// Replaces `Dep` once all FFI bodies have been migrated. `DepNode::default()`
/// is a fresh, empty edge (all flags clear, no target, empty name) — the
/// idiomatic replacement for the c2rust `alloc_dep`.
#[derive(Debug, Clone, Default, ContentHash)]
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

#[cfg(test)]
mod dep_id_tests {
    use super::{DepFlags, DepId, DepNode};

    /// Two structurally-identical dep edges hash identically; edges differing
    /// in any field — name, flags, or a bool marker — hash differently, since
    /// the whole struct is content-hashed. (`DepId` predates this session's
    /// `FileId`/`GoalDepId` work but had no test coverage of its own.)
    #[test]
    fn hashes_whole_struct_deterministically() {
        let a = DepNode {
            name: "foo.o".to_string(),
            ..Default::default()
        };
        let a_again = DepNode {
            name: "foo.o".to_string(),
            ..Default::default()
        };
        let b = DepNode {
            name: "bar.o".to_string(),
            ..Default::default()
        };
        assert_eq!(DepId::from(&a), DepId::from(&a_again));
        assert_ne!(DepId::from(&a), DepId::from(&b));

        let mut c = a.clone();
        c.flags = DepFlags::DONTCARE;
        assert_ne!(DepId::from(&a), DepId::from(&c));

        let mut d = a.clone();
        d.is_explicit = true;
        assert_ne!(DepId::from(&a), DepId::from(&d));
    }
}

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
/// `GoalDepNode::default()` is a fresh, empty goal (an empty [`DepNode`] plus
/// zeroed error/location bookkeeping) — the idiomatic replacement for the
/// c2rust `alloc_goaldep`.
#[derive(Debug, Clone, Default, ContentHash)]
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

// Stable identity for a goal: content-hash of the full (immutable) `GoalDepNode`.
// Mirrors `DepId <- DepNode` above and `FileId <- FileNode` (file.rs), extending
// blake3 content-hash identity across the dependency graph's node/edge/goal
// triad. Like those two, this is scaffolding for an in-progress migration —
// no caller derives a `GoalDepId` yet.
crate::id_wireformat!(GoalDepId[HASH_SIZE] <- GoalDepNode);

#[cfg(test)]
mod goal_dep_id_tests {
    use super::{DepNode, GoalDepId, GoalDepNode};
    use crate::file::{FileId, FileNode};

    /// Build a small but non-trivial dependency graph — a program linked from
    /// two object files, each compiled from a source file — using real
    /// `FileNode`s, wiring the `DepNode` edges with the actual content-hash
    /// `FileId::from(&node)` of each prerequisite (not placeholder literals
    /// or the name-only `.id()`, which by design stays fixed across content
    /// changes and so wouldn't let a leaf edit ripple upward). Returns the
    /// top-level goal for `prog`.
    fn build_graph(main_source_name: &str) -> GoalDepNode {
        let main_c = FileNode::new(main_source_name.as_bytes().to_vec());
        let util_c = FileNode::new(b"util.c".to_vec());

        let mut main_o = FileNode::new(b"main.o".to_vec());
        main_o.deps.push(DepNode {
            name: main_source_name.to_string(),
            file: Some(FileId::from(&main_c)),
            is_explicit: true,
            ..Default::default()
        });

        let mut util_o = FileNode::new(b"util.o".to_vec());
        util_o.deps.push(DepNode {
            name: "util.c".to_string(),
            file: Some(FileId::from(&util_c)),
            is_explicit: true,
            ..Default::default()
        });

        let mut prog = FileNode::new(b"prog".to_vec());
        prog.deps.push(DepNode {
            name: "main.o".to_string(),
            file: Some(FileId::from(&main_o)),
            is_explicit: true,
            ..Default::default()
        });
        prog.deps.push(DepNode {
            name: "util.o".to_string(),
            file: Some(FileId::from(&util_o)),
            is_explicit: true,
            ..Default::default()
        });

        GoalDepNode {
            dep: DepNode {
                name: "prog".to_string(),
                file: Some(FileId::from(&prog)),
                is_explicit: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// End-to-end: hashing a realistic multi-node graph (source -> object ->
    /// program -> goal, four real `FileNode`s deep, linked by real computed
    /// `FileId` content hashes) is deterministic across an independent
    /// rebuild, and a change to a single leaf source ripples through every
    /// ancestor's content hash up to the goal.
    #[test]
    fn hashes_a_realistic_multi_node_graph_end_to_end() {
        let goal = build_graph("main.c");
        let rebuilt = build_graph("main.c");
        let tampered = build_graph("main2.c");

        assert_eq!(GoalDepId::from(&goal), GoalDepId::from(&rebuilt));
        assert_ne!(GoalDepId::from(&goal), GoalDepId::from(&tampered));
        assert_ne!(goal.dep.file, tampered.dep.file);
    }

    /// Two structurally-identical goals hash identically; goals differing in
    /// the underlying dep edge, or in the goal-only bookkeeping fields
    /// (error/location), hash differently.
    #[test]
    fn hashes_whole_struct_deterministically() {
        let goal = |name: &str| GoalDepNode {
            dep: DepNode {
                name: name.to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let a = goal("all");
        let a_again = goal("all");
        let b = goal("clean");
        assert_eq!(GoalDepId::from(&a), GoalDepId::from(&a_again));
        assert_ne!(GoalDepId::from(&a), GoalDepId::from(&b));

        // Same dep edge, different goal-only bookkeeping (error/location) —
        // still distinct, since the whole struct is hashed.
        let mut c = goal("all");
        let before = GoalDepId::from(&c);
        c.error = 2;
        let after = GoalDepId::from(&c);
        assert_ne!(before, after);

        let mut d = goal("all");
        d.lineno = 7;
        assert_ne!(GoalDepId::from(&a), GoalDepId::from(&d));
    }
}

/// A simple chain of names — the base name-chain node whose `next`/`name`
/// prefix [`Dep`] and [`GoalDep`] share (see the `SeqNode` trait in `file.rs`).
/// Legacy c2rust raw-pointer type; the makefile reader now builds owned `Vec`s
/// instead of threading these chains, so it survives only for the remaining
/// pointer-punning sites (e.g. `ar_glob`, the `free_ns*` helpers). `#[repr(C)]`
/// like its `Dep`/`GoalDep` siblings: those layout-punning casts rely on `next`
/// sitting at offset 0. `file.rs` re-exports it (and the `nameseq` alias).
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameSeq {
    pub next: *mut NameSeq,
    pub name: *const ::core::ffi::c_char,
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
