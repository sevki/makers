//! Proof-of-concept dependency graph using arena indices instead of raw
//! pointers — the target representation for migrating make's pointer-based
//! `File`/`Dep` graph (currently `*mut File` / `*mut Dep` everywhere) to safe,
//! idiomatic Rust.
//!
//! Design notes — what a rustc/cargo author reaches for, using only `std`:
//!
//!   * Files live in a `Vec` arena and everything refers to them by a small
//!     `Copy` [`FileId`] index instead of `*mut File`. This is the same
//!     "typed index into an arena" pattern rustc uses for its IR nodes and
//!     cargo uses for package ids — no extra crates.
//!   * make's graph has cycles (`a: b` and `b: a`) and heavy sharing (one
//!     file is a prerequisite of many). Indices model both trivially: there
//!     is no ownership to thread, so no `Rc`/`Weak`, no `RefCell`, no
//!     reference cycles to leak, and no `unsafe`.
//!   * Targets are interned by name in a `HashMap`, mirroring make's global
//!     file hash table (`enter_file` / `lookup_file`).
//!
//! The intent is a faithful-enough model to be a template for the real
//! migration, not a drop-in replacement yet.

use std::collections::HashMap;

/// Index of a target file in a [`Graph`]. Cheap, `Copy`, and comparable —
/// the safe replacement for a `*mut File`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FileId(u32);

impl FileId {
    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Per-edge prerequisite flags (mirrors make's per-`dep` bitfield).
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct DepFlags {
    /// `|`-style order-only prerequisite.
    pub order_only: bool,
    /// Do not use this prerequisite's mtime when deciding to rebuild.
    pub ignore_mtime: bool,
}

/// A prerequisite of a target: an edge to another file in the arena.
#[derive(Clone, Copy, Debug)]
pub struct Dep {
    /// The prerequisite file (replaces `Dep::file: *mut File`).
    pub target: FileId,
    /// Explicit (written in a rule) vs. inferred by an implicit rule.
    pub explicit: bool,
    pub flags: DepFlags,
}

/// A target node (the graph-relevant slice of make's `struct file`).
#[derive(Debug)]
pub struct FileNode {
    name: Box<str>,
    /// Ordinary prerequisites in declaration order (replaces `File::deps`,
    /// a `*mut Dep` linked list).
    deps: Vec<Dep>,
    /// Additional `::` rule entries for the same target name (replaces the
    /// `File::double_colon` pointer chain).
    double_colon: Vec<FileId>,
    is_target: bool,
    phony: bool,
}

impl FileNode {
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[inline]
    pub fn deps(&self) -> &[Dep] {
        &self.deps
    }
    #[inline]
    pub fn double_colon(&self) -> &[FileId] {
        &self.double_colon
    }
    #[inline]
    pub fn is_target(&self) -> bool {
        self.is_target
    }
    #[inline]
    pub fn is_phony(&self) -> bool {
        self.phony
    }
}

/// The dependency graph: an arena of files interned by name.
#[derive(Debug, Default)]
pub struct Graph {
    files: Vec<FileNode>,
    by_name: HashMap<Box<str>, FileId>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of distinct target nodes (including extra `::` entries).
    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    fn push_node(&mut self, name: Box<str>, is_target: bool) -> FileId {
        let id = FileId(self.files.len() as u32);
        self.files.push(FileNode {
            name,
            deps: Vec::new(),
            double_colon: Vec::new(),
            is_target,
            phony: false,
        });
        id
    }

    /// Look up `name`, creating an empty target node if absent (make's
    /// `enter_file`). The returned id is stable for the life of the graph.
    pub fn enter(&mut self, name: &str) -> FileId {
        if let Some(&id) = self.by_name.get(name) {
            return id;
        }
        let boxed: Box<str> = name.into();
        let id = self.push_node(boxed.clone(), false);
        self.by_name.insert(boxed, id);
        id
    }

    /// Look up an existing target without creating one (make's `lookup_file`).
    pub fn lookup(&self, name: &str) -> Option<FileId> {
        self.by_name.get(name).copied()
    }

    #[inline]
    pub fn file(&self, id: FileId) -> &FileNode {
        &self.files[id.index()]
    }

    #[inline]
    pub fn file_mut(&mut self, id: FileId) -> &mut FileNode {
        &mut self.files[id.index()]
    }

    pub fn set_phony(&mut self, id: FileId, phony: bool) {
        self.files[id.index()].phony = phony;
    }

    /// Mark a name as a real rule target (`File::is_target`).
    pub fn mark_target(&mut self, id: FileId) {
        self.files[id.index()].is_target = true;
    }

    /// Add `dep` as a prerequisite of `target`. Mirrors make's de-dup: the
    /// same explicit prerequisite is not recorded twice.
    pub fn add_dep(&mut self, target: FileId, dep: Dep) {
        let node = &mut self.files[target.index()];
        if dep.explicit
            && node
                .deps
                .iter()
                .any(|d| d.target == dep.target && d.explicit)
        {
            return;
        }
        node.deps.push(dep);
    }

    /// Convenience: add an explicit prerequisite by name, interning it, and
    /// return its id.
    pub fn add_prereq_by_name(&mut self, target: FileId, prereq_name: &str) -> FileId {
        let prereq = self.enter(prereq_name);
        self.add_dep(
            target,
            Dep {
                target: prereq,
                explicit: true,
                flags: DepFlags::default(),
            },
        );
        prereq
    }

    /// Record an additional `::` entry under the same target name and return
    /// its id (each entry is its own node, chained under the head).
    pub fn add_double_colon(&mut self, name: &str) -> FileId {
        let head = self.enter(name);
        let entry_name = self.files[head.index()].name.clone();
        let entry = self.push_node(entry_name, true);
        self.files[head.index()].double_colon.push(entry);
        entry
    }

    /// Visit `start` and all its transitive prerequisites depth-first,
    /// calling `visit` once per reachable file. Cycles terminate safely
    /// because each file is visited at most once.
    pub fn visit_deps(&self, start: FileId, mut visit: impl FnMut(FileId)) {
        let mut seen = vec![false; self.files.len()];
        let mut stack = vec![start];
        while let Some(id) = stack.pop() {
            if std::mem::replace(&mut seen[id.index()], true) {
                continue;
            }
            visit(id);
            for dep in &self.files[id.index()].deps {
                stack.push(dep.target);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter_is_idempotent_like_the_file_hash() {
        let mut g = Graph::new();
        let a1 = g.enter("foo.o");
        let a2 = g.enter("foo.o");
        assert_eq!(a1, a2);
        assert_eq!(g.len(), 1);
        assert_eq!(g.file(a1).name(), "foo.o");
    }

    #[test]
    fn lookup_does_not_create() {
        let mut g = Graph::new();
        assert_eq!(g.lookup("x"), None);
        let id = g.enter("x");
        assert_eq!(g.lookup("x"), Some(id));
        assert_eq!(g.len(), 1);
    }

    #[test]
    fn prerequisites_are_edges_to_interned_files() {
        let mut g = Graph::new();
        let prog = g.enter("prog");
        g.add_prereq_by_name(prog, "a.o");
        g.add_prereq_by_name(prog, "b.o");
        let names: Vec<&str> = g
            .file(prog)
            .deps()
            .iter()
            .map(|d| g.file(d.target).name())
            .collect();
        assert_eq!(names, ["a.o", "b.o"]);
    }

    #[test]
    fn shared_prerequisite_is_one_node() {
        // Two targets depending on the same header: one file, two edges.
        let mut g = Graph::new();
        let a = g.enter("a.o");
        let b = g.enter("b.o");
        let h1 = g.add_prereq_by_name(a, "h.h");
        let h2 = g.add_prereq_by_name(b, "h.h");
        assert_eq!(h1, h2);
        assert_eq!(g.len(), 3); // a.o, b.o, h.h
    }

    #[test]
    fn duplicate_explicit_prereqs_are_ignored() {
        let mut g = Graph::new();
        let t = g.enter("t");
        g.add_prereq_by_name(t, "dep");
        g.add_prereq_by_name(t, "dep");
        assert_eq!(g.file(t).deps().len(), 1);
    }

    #[test]
    fn cycles_are_representable_and_traversal_terminates() {
        // `a: b` and `b: a` — a circular dependency. With raw pointers this
        // is a delicate aliased structure; with indices it is just two edges,
        // and the visited set makes traversal terminate.
        let mut g = Graph::new();
        let a = g.enter("a");
        let b = g.enter("b");
        g.add_prereq_by_name(a, "b");
        g.add_prereq_by_name(b, "a");
        let mut visited = Vec::new();
        g.visit_deps(a, |id| visited.push(id));
        visited.sort_by_key(|id| id.index());
        assert_eq!(visited, [a, b]);
    }

    #[test]
    fn double_colon_entries_chain_under_one_name() {
        let mut g = Graph::new();
        let e1 = g.add_double_colon("clean");
        let e2 = g.add_double_colon("clean");
        let head = g.lookup("clean").unwrap();
        assert_eq!(g.file(head).double_colon(), [e1, e2]);
        assert_ne!(e1, e2);
    }

    #[test]
    fn phony_and_target_flags_round_trip() {
        let mut g = Graph::new();
        let id = g.enter("all");
        assert!(!g.file(id).is_phony());
        assert!(!g.file(id).is_target());
        g.set_phony(id, true);
        g.mark_target(id);
        assert!(g.file(id).is_phony());
        assert!(g.file(id).is_target());
    }
}
