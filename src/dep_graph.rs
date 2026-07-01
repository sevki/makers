//! A dependency graph keyed by content hash, with first-class edges.
//!
//! `dep.rs` and `file.rs` already give every node in the dependency graph a
//! stable, content-hashed identity: `FileId <- FileNode`, `DepId <- DepNode`,
//! `GoalDepId <- GoalDepNode` (all blake3, via `id_wireformat!`). Until now
//! those `From` conversions were scaffolding with no caller — nothing stored
//! nodes keyed by the hash, and nothing resolved an edge's `Option<FileId>`
//! back to the node it names. [`DepGraph`] is that caller.
//!
//! A node's `Option<FileId>` field is a pointer, not a graph: on its own it
//! only supports a forward walk from a node you already have in hand, one
//! hop at a time, and can't answer "what depends on this file" at all. This
//! module builds explicit [`Edge`]s (`from`/`to` node ids plus the `DepId` of
//! the `DepNode` that names the edge) and indexes them both ways
//! (`edges_from`/`edges_to`), so both traversal directions — prerequisites
//! and dependents — are real graph queries instead of one-way pointer
//! chasing.
//!
//! Content addressing gives two more properties for free:
//! - **Dedup**: inserting a structurally identical node twice is a no-op —
//!   both inserts hash to the same key, so the second is dropped (and its
//!   edges are not re-recorded) and the first insert's id is returned again.
//! - **Ripple**: a node's id is a pure function of its content, so editing a
//!   leaf and re-inserting the chain above it (each parent's `file`/`dep`
//!   link updated to the leaf's new id) produces a new id, and new edges, at
//!   every ancestor up to the goal, while any untouched sibling subtree keeps
//!   its original id and its original edges.
//!
//! This module does not touch the live build graph (`ExecContext::filenodes`,
//! keyed by `FileNode::id()` — name-based, not content-based; see the
//! `FileId <- FileNode` doc comment in `file.rs` for why the two identities
//! are deliberately different). It is a standalone, additive structure.

use crate::dep::{DepId, DepNode, GoalDepId, GoalDepNode};
use crate::file::{FileId, FileNode};
use std::collections::HashMap;

/// A directed dependency edge: `from` depends on `to`. `dep` is the content
/// hash of the `DepNode` edge data (name, flags, ...) that produced this
/// edge — the same id the dep store uses, so `graph.dep(edge.dep)` looks up
/// the edge's full metadata (name, `static_pattern`, `is_explicit`, ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub from: FileId,
    pub dep: DepId,
    pub to: FileId,
}

/// A dependency graph keyed entirely by blake3 content hash: one
/// content-addressed store per node kind (file, dep edge, goal), plus an
/// explicit, bidirectionally-indexed edge set built from each file's
/// prerequisite links as it's inserted.
#[derive(Debug, Default)]
pub struct DepGraph {
    files: HashMap<FileId, FileNode>,
    deps: HashMap<DepId, DepNode>,
    goals: HashMap<GoalDepId, GoalDepNode>,
    out_edges: HashMap<FileId, Vec<Edge>>,
    in_edges: HashMap<FileId, Vec<Edge>>,
}

impl DepGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Content-address `node` and store it under its `FileId`. For every
    /// prerequisite in `node.deps` that names a target (`DepNode.file`),
    /// record an [`Edge`] from this node to that target, indexed under both
    /// endpoints. Re-inserting a structurally identical node is a no-op —
    /// the store dedups by content, not by insertion order, and a no-op
    /// insert does not re-record edges.
    pub fn insert_file(&mut self, node: FileNode) -> FileId {
        let id = FileId::from(&node);
        if !self.files.contains_key(&id) {
            for dep in &node.deps {
                let dep_id = self.insert_dep(dep.clone());
                if let Some(to) = dep.file {
                    let edge = Edge {
                        from: id,
                        dep: dep_id,
                        to,
                    };
                    self.out_edges.entry(id).or_default().push(edge);
                    self.in_edges.entry(to).or_default().push(edge);
                }
            }
            self.files.insert(id, node);
        }
        id
    }

    /// Content-address a dep edge's data. See [`Self::insert_file`]; this
    /// alone does not create an [`Edge`] — `insert_file` does that for the
    /// prerequisites it owns.
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

    /// Outgoing edges: the prerequisites `id` depends on. Empty if `id` was
    /// never inserted, or was inserted with no prerequisites.
    pub fn edges_from(&self, id: FileId) -> &[Edge] {
        self.out_edges.get(&id).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Incoming edges: the files that name `id` as a prerequisite — the
    /// reverse direction a bare `Option<FileId>` link can't answer at all.
    /// Empty if nothing in the graph depends on `id`.
    pub fn edges_to(&self, id: FileId) -> &[Edge] {
        self.in_edges.get(&id).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Resolve a goal's target: follow `goal.dep.file` into the file store.
    /// `None` if the goal isn't in this graph, or its target link is absent
    /// or points at a `FileId` this graph never stored.
    pub fn goal_target_file(&self, goal: GoalDepId) -> Option<&FileNode> {
        let target = self.goal(goal)?.dep.file?;
        self.file(target)
    }

    /// Prerequisite nodes of `id`, resolved through its outgoing edges.
    pub fn prerequisites(&self, id: FileId) -> impl Iterator<Item = &FileNode> {
        self.edges_from(id)
            .iter()
            .filter_map(move |e| self.file(e.to))
    }

    /// Dependent nodes of `id`: every stored node whose prerequisites
    /// include `id`, resolved through its incoming edges.
    pub fn dependents(&self, id: FileId) -> impl Iterator<Item = &FileNode> {
        self.edges_to(id)
            .iter()
            .filter_map(move |e| self.file(e.from))
    }
}

#[cfg(test)]
mod tests {
    use super::DepGraph;
    use crate::dep::{DepNode, GoalDepNode};
    use crate::file::{FileId, FileNode};

    /// Insert a source -> object -> program -> goal chain (mirroring
    /// `dep.rs`'s `goal_dep_id_tests::build_graph`, but through the graph's
    /// insert API instead of loose `FileId::from` calls) and return the
    /// graph plus the goal's id.
    fn build(graph: &mut DepGraph, main_source_name: &str) -> crate::dep::GoalDepId {
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
        let mut graph = DepGraph::new();
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

    /// The reverse direction a bare `Option<FileId>` pointer can't answer:
    /// given a leaf, who depends on it? `edges_to`/`dependents` walk the
    /// explicit incoming-edge index, not the forward `deps` links.
    #[test]
    fn walks_dependents_in_reverse() {
        let mut graph = DepGraph::new();
        let goal = build(&mut graph, "main.c");
        let target = graph.goal_target_file(goal).unwrap();
        let prog_id = FileId::from(target);

        // `build` inserts the leaf as an unmodified `FileNode::new(...)`, so
        // its content hash (not the name-based `FileId::from_bytes`, which
        // keys the graph's separate live-identity path — see the
        // `FileId <- FileNode` doc comment in file.rs) is reproducible here.
        let main_c_id = FileId::from(&FileNode::new(b"main.c".to_vec()));
        let main_o = graph
            .dependents(main_c_id)
            .next()
            .expect("main.o depends on main.c");
        assert_eq!(main_o.name, b"main.o");
        assert_eq!(graph.dependents(main_c_id).count(), 1);

        let mut prog_dependents: Vec<&[u8]> = graph
            .dependents(FileId::from(main_o))
            .map(|f| f.name.as_slice())
            .collect();
        assert_eq!(prog_dependents, vec![&b"prog"[..]]);
        prog_dependents.clear();

        // Nothing depends on the root: `prog` has an empty in-edge set.
        assert_eq!(graph.dependents(prog_id).count(), 0);

        // Edge metadata round-trips through the dep store.
        let edge = graph.edges_to(main_c_id)[0];
        assert_eq!(edge.to, main_c_id);
        assert_eq!(graph.dep(edge.dep).unwrap().name, "main.c");
    }

    #[test]
    fn inserting_identical_content_twice_dedups_nodes_and_edges() {
        let mut graph = DepGraph::new();
        let leaf = graph.insert_file(FileNode::new(b"shared.h".to_vec()));

        let mut a = FileNode::new(b"a.o".to_vec());
        a.deps.push(DepNode {
            name: "shared.h".to_string(),
            file: Some(leaf),
            ..Default::default()
        });
        let id_a = graph.insert_file(a.clone());
        let id_a_again = graph.insert_file(a);
        assert_eq!(id_a, id_a_again);
        assert_eq!(graph.file_count(), 2); // shared.h, a.o
                                           // The second, no-op insert must not duplicate the edge.
        assert_eq!(graph.edges_from(id_a).len(), 1);
        assert_eq!(graph.edges_to(leaf).len(), 1);
    }

    /// Editing a leaf source and re-inserting the whole chain produces a new
    /// id and new edges at every ancestor up to the goal (the ripple
    /// property), while the untouched chain's entries and edges are still
    /// present under their original ids — content addressing never
    /// overwrites, it only adds.
    #[test]
    fn edits_ripple_from_leaf_to_goal_without_losing_history() {
        let mut graph = DepGraph::new();
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
        // program, proving the untouched branch wasn't duplicated or lost —
        // and both programs show up as its dependent.
        let util_from_original = graph
            .prerequisites(FileId::from(original_target))
            .find(|f| f.name == b"util.o")
            .unwrap();
        let util_from_tampered = graph
            .prerequisites(FileId::from(tampered_target))
            .find(|f| f.name == b"util.o")
            .unwrap();
        let util_o_id = FileId::from(util_from_original);
        assert_eq!(util_o_id, FileId::from(util_from_tampered));
        assert_eq!(graph.dependents(util_o_id).count(), 2);
    }

    #[test]
    fn missing_ids_resolve_to_none() {
        let graph = DepGraph::new();
        let phantom = FileId::from_bytes(b"never-inserted");
        assert!(graph.file(phantom).is_none());
        assert_eq!(graph.prerequisites(phantom).count(), 0);
        assert_eq!(graph.dependents(phantom).count(), 0);
        assert!(graph.edges_from(phantom).is_empty());
        assert!(graph.edges_to(phantom).is_empty());
    }
}
