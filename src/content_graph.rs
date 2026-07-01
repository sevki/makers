//! A fully content-addressed dependency graph.
//!
//! `dep.rs` and `file.rs` already give every node in the dependency graph a
//! stable, content-hashed identity: `FileId <- FileNode`, `DepId <- DepNode`,
//! `GoalDepId <- GoalDepNode` (all blake3, via `id_wireformat!`). Until now
//! those `From` conversions were scaffolding with no caller — nothing stored
//! nodes keyed by the hash, and nothing resolved an edge's `Option<FileId>`
//! back to the node it names. [`ContentGraph`] is that caller: an
//! insert-by-content-hash store for all three node kinds, plus lookups that
//! walk goal -> target file -> prerequisite files purely through content-hash
//! keys.
//!
//! Content addressing gives two properties for free:
//! - **Dedup**: inserting a structurally identical node twice is a no-op —
//!   both inserts hash to the same key, so the second is dropped and the
//!   first insert's id is returned again.
//! - **Ripple**: a node's id is a pure function of its content, so editing a
//!   leaf and re-inserting the chain above it (each parent's `file`/`dep`
//!   link updated to the leaf's new id) produces a new id at every ancestor
//!   up to the goal, while any untouched sibling subtree keeps its original
//!   id and its original store entry.
//!
//! This module does not touch the live build graph (`ExecContext::filenodes`,
//! keyed by `FileNode::id()` — name-based, not content-based; see the
//! `FileId <- FileNode` doc comment in `file.rs` for why the two identities
//! are deliberately different). It is a standalone, additive structure.

use crate::dep::{DepId, DepNode, GoalDepId, GoalDepNode};
use crate::file::{FileId, FileNode};
use std::collections::HashMap;

/// A dependency graph keyed entirely by blake3 content hash: one
/// content-addressed store per node kind (file, dep edge, goal).
#[derive(Debug, Default)]
pub struct ContentGraph {
    files: HashMap<FileId, FileNode>,
    deps: HashMap<DepId, DepNode>,
    goals: HashMap<GoalDepId, GoalDepNode>,
}

impl ContentGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Content-address `node`: compute its `FileId` and store it under that
    /// key, returning the id. Re-inserting a structurally identical node is a
    /// no-op that returns the same id — the store dedups by content, not by
    /// insertion order.
    pub fn insert_file(&mut self, node: FileNode) -> FileId {
        let id = FileId::from(&node);
        self.files.entry(id).or_insert(node);
        id
    }

    /// Content-address a dep edge. See [`Self::insert_file`].
    pub fn insert_dep(&mut self, node: DepNode) -> DepId {
        let id = DepId::from(&node);
        self.deps.entry(id).or_insert(node);
        id
    }

    /// Content-address a goal. See [`Self::insert_file`].
    pub fn insert_goal(&mut self, node: GoalDepNode) -> GoalDepId {
        let id = GoalDepId::from(&node);
        self.goals.entry(id).or_insert(node);
        id
    }

    pub fn file(&self, id: FileId) -> Option<&FileNode> {
        self.files.get(&id)
    }

    pub fn dep(&self, id: DepId) -> Option<&DepNode> {
        self.deps.get(&id)
    }

    pub fn goal(&self, id: GoalDepId) -> Option<&GoalDepNode> {
        self.goals.get(&id)
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    pub fn dep_count(&self) -> usize {
        self.deps.len()
    }

    pub fn goal_count(&self) -> usize {
        self.goals.len()
    }

    /// Resolve a goal's target: follow `goal.dep.file` into the file store.
    /// `None` if the goal isn't in this graph, or its target link is absent
    /// or points at a `FileId` this graph never stored.
    pub fn goal_target_file(&self, goal: GoalDepId) -> Option<&FileNode> {
        let target = self.goal(goal)?.dep.file?;
        self.file(target)
    }

    /// Prerequisite files of a stored file node, resolved through each dep
    /// edge's `file` link. Prerequisites with no `file` link, or whose
    /// `FileId` was never inserted into this graph, are silently skipped —
    /// the same "absent means untracked" semantics as `goal_target_file`.
    pub fn prerequisites(&self, id: FileId) -> impl Iterator<Item = &FileNode> {
        self.file(id)
            .into_iter()
            .flat_map(|f| f.deps.iter())
            .filter_map(move |d| self.file(d.file?))
    }
}

#[cfg(test)]
mod tests {
    use super::ContentGraph;
    use crate::dep::{DepNode, GoalDepNode};
    use crate::file::{FileId, FileNode};

    /// Insert a source -> object -> program -> goal chain (mirroring
    /// `dep.rs`'s `goal_dep_id_tests::build_graph`, but through the graph's
    /// insert API instead of loose `FileId::from` calls) and return the
    /// graph plus the goal's id.
    fn build(graph: &mut ContentGraph, main_source_name: &str) -> crate::dep::GoalDepId {
        let main_c = graph.insert_file(FileNode::new(main_source_name.as_bytes().to_vec()));
        let util_c = graph.insert_file(FileNode::new(b"util.c".to_vec()));

        let mut main_o = FileNode::new(b"main.o".to_vec());
        main_o.deps.push(DepNode {
            name: main_source_name.to_string(),
            file: Some(main_c),
            is_explicit: true,
            ..Default::default()
        });
        let main_o = graph.insert_file(main_o);

        let mut util_o = FileNode::new(b"util.o".to_vec());
        util_o.deps.push(DepNode {
            name: "util.c".to_string(),
            file: Some(util_c),
            is_explicit: true,
            ..Default::default()
        });
        let util_o = graph.insert_file(util_o);

        let mut prog = FileNode::new(b"prog".to_vec());
        prog.deps.push(DepNode {
            name: "main.o".to_string(),
            file: Some(main_o),
            is_explicit: true,
            ..Default::default()
        });
        prog.deps.push(DepNode {
            name: "util.o".to_string(),
            file: Some(util_o),
            is_explicit: true,
            ..Default::default()
        });
        let prog = graph.insert_file(prog);

        graph.insert_goal(GoalDepNode {
            dep: DepNode {
                name: "prog".to_string(),
                file: Some(prog),
                is_explicit: true,
                ..Default::default()
            },
            ..Default::default()
        })
    }

    #[test]
    fn walks_goal_to_target_to_prerequisites() {
        let mut graph = ContentGraph::new();
        let goal = build(&mut graph, "main.c");

        assert_eq!(graph.file_count(), 5); // main.c, util.c, main.o, util.o, prog
        assert_eq!(graph.goal_count(), 1);

        let target = graph.goal_target_file(goal).expect("goal has a target");
        assert_eq!(target.name, b"prog");

        let mut prereq_names: Vec<&[u8]> = graph
            .prerequisites(FileId::from(target))
            .map(|f| f.name.as_slice())
            .collect();
        prereq_names.sort();
        assert_eq!(prereq_names, vec![&b"main.o"[..], &b"util.o"[..]]);

        let main_o = graph
            .prerequisites(FileId::from(target))
            .find(|f| f.name == b"main.o")
            .expect("main.o is a prerequisite of prog");
        let leaf_names: Vec<&[u8]> = graph
            .prerequisites(FileId::from(main_o))
            .map(|f| f.name.as_slice())
            .collect();
        assert_eq!(leaf_names, vec![&b"main.c"[..]]);
    }

    #[test]
    fn inserting_identical_content_twice_dedups() {
        let mut graph = ContentGraph::new();
        let a = graph.insert_file(FileNode::new(b"shared.h".to_vec()));
        let b = graph.insert_file(FileNode::new(b"shared.h".to_vec()));
        assert_eq!(a, b);
        assert_eq!(graph.file_count(), 1);
    }

    /// Editing a leaf source and re-inserting the whole chain produces a new
    /// id at every ancestor up to the goal (the ripple property), while the
    /// untouched chain's entries are still present under their original ids
    /// — content addressing never overwrites, it only adds.
    #[test]
    fn edits_ripple_from_leaf_to_goal_without_losing_history() {
        let mut graph = ContentGraph::new();
        let original_goal = build(&mut graph, "main.c");
        let tampered_goal = build(&mut graph, "main2.c");

        assert_ne!(original_goal, tampered_goal);
        // Both chains coexist: 5 original nodes + (main2.c, new main.o, new
        // prog) = 8. util.c/util.o are shared and not duplicated.
        assert_eq!(graph.file_count(), 8);
        assert_eq!(graph.goal_count(), 2);

        let original_target = graph.goal_target_file(original_goal).unwrap();
        let tampered_target = graph.goal_target_file(tampered_goal).unwrap();
        assert_ne!(FileId::from(original_target), FileId::from(tampered_target));
        assert_eq!(original_target.name, b"prog");
        assert_eq!(tampered_target.name, b"prog");

        // The shared util.o/util.c subtree resolves identically from either
        // program, proving the untouched branch wasn't duplicated or lost.
        let util_from_original = graph
            .prerequisites(FileId::from(original_target))
            .find(|f| f.name == b"util.o")
            .unwrap();
        let util_from_tampered = graph
            .prerequisites(FileId::from(tampered_target))
            .find(|f| f.name == b"util.o")
            .unwrap();
        assert_eq!(
            FileId::from(util_from_original),
            FileId::from(util_from_tampered)
        );
    }

    #[test]
    fn missing_ids_resolve_to_none() {
        let graph = ContentGraph::new();
        let phantom = FileId::from_bytes(b"never-inserted");
        assert!(graph.file(phantom).is_none());
        assert_eq!(graph.prerequisites(phantom).count(), 0);
    }
}
