//! A dependency graph keyed by content hash, with first-class edges.
//!
//! `dep.rs` and `file.rs` already give every node in the dependency graph a
//! stable, content-hashed identity: `FileId <- FileNode`, `DepId <- DepNode`,
//! `GoalDepId <- GoalDepNode` (all blake3, via `id_wireformat!`). Until now
//! those `From` conversions were scaffolding with no caller — nothing stored
//! nodes keyed by the hash, and nothing resolved an edge's `Option<FileId>`
//! back to the node it names. [`DepGraph`] is that caller.
//!
//! [`NodeId`] unifies the three id kinds into one type so `edges` can point
//! from any node to any other, the same way a linked list's node holds its
//! neighbors' ids rather than a separate side table: a `File` points at the
//! `Dep` node for each of its prerequisites, a `Dep` points at the `File` it
//! targets (if named), and a `GoalDep` points at the `Dep` it wraps.
//! `edges_to` (a reverse scan — `edges` is only indexed forward) is what
//! makes [`DepGraph::dependents`] a real query instead of one-way pointer
//! chasing, and — because it returns every match, not just the first — two
//! different files sharing a bit-for-bit identical prerequisite edge are
//! both still found (content-addressing collapses the edge itself, not its
//! owners).
//!
//! Content addressing gives two more properties for free:
//! - **Dedup**: adding a structurally identical node twice is a no-op —
//!   both calls hash to the same key, so the second is dropped (and its
//!   edges are not re-recorded) and the first call's id is returned again.
//! - **Ripple**: a node's id is a pure function of its content, so editing a
//!   leaf and re-adding the chain above it (each parent's `file`/`dep` link
//!   updated to the leaf's new id) produces a new id, and new edges, at
//!   every ancestor up to the goal, while any untouched sibling subtree keeps
//!   its original id and its original edges.
//!
//! This module does not touch the live build graph (`ExecContext::filenodes`,
//! keyed by `FileNode::id()` — name-based, not content-based; see the
//! `FileId <- FileNode` doc comment in `file.rs` for why the two identities
//! are deliberately different). It *can*, however, safely ingest real
//! `FileNode`s produced by that graph: every `add_file` also indexes the
//! node under its name-derived `FileNode::id()` in `name_index`, so a live
//! `DepNode.file` (always name-derived, never content-hash) still resolves
//! to the right entry here even though this graph's own ids are content
//! hashes. `resolve` is the lookup that tries both.

use hashbrown::HashMap;

use crate::dep::{DepId, DepNode, GoalDepId, GoalDepNode};
use crate::file::{FileId, FileNode};

/// A single graph node identity, unifying `FileId`/`DepId`/`GoalDepId` so
/// [`DepGraph::edges`] can connect any node to any other by id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeId {
    File(FileId),
    Dep(DepId),
    GoalDep(GoalDepId),
}

/// A dependency graph keyed entirely by blake3 content hash, with real
/// adjacency: `edges` maps every node to the nodes it points directly at,
/// so both prerequisites (forward, one hop) and dependents (reverse, via
/// `edges_to`) are real graph queries.
#[derive(Debug, Clone, Default)]
pub struct DepGraph {
    pub files: HashMap<FileId, FileNode>,
    pub deps: HashMap<DepId, DepNode>,
    pub goal_deps: HashMap<GoalDepId, GoalDepNode>,
    pub edges: HashMap<NodeId, Vec<NodeId>>,
    /// `FileNode::id()` (name-derived, the live build graph's own identity)
    /// to the content-hash id currently stored for whichever node occupies
    /// that name. Populated on every `add_file`; consulted by `resolve` when
    /// an id isn't already a content-hash key in `files`.
    name_index: HashMap<FileId, FileId>,
}

impl DepGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Resolve `id` to a stored file: first as a content-hash key, then
    /// (via `name_index`) as a live-graph, name-derived key. This is what
    /// lets every lookup below accept either kind of id transparently.
    fn resolve(&self, id: FileId) -> Option<&FileNode> {
        self.files
            .get(&id)
            .or_else(|| self.files.get(self.name_index.get(&id)?))
    }

    /// True if `a` and `b` resolve to the same stored file — so a
    /// content-hash id and its live-graph, name-derived alias compare equal.
    fn same_file(&self, a: FileId, b: FileId) -> bool {
        match (self.resolve(a), self.resolve(b)) {
            (Some(x), Some(y)) => FileId::from(x) == FileId::from(y),
            _ => false,
        }
    }

    /// Content-address `node` under its `FileId`, add a `Dep` node for each
    /// prerequisite (wired to the file it targets, if named), and record the
    /// file's own edges to those `Dep` nodes. Re-adding a structurally
    /// identical file is a no-op — the store dedups by content, not by
    /// insertion order, and a no-op add does not re-record edges or the name
    /// index.
    pub fn add_file(&mut self, node: FileNode) -> FileId {
        let id = FileId::from(&node);
        if self.files.contains_key(&id) {
            return id;
        }
        // Index under the live-graph identity too, so a real `DepNode.file`
        // (name-derived, not content-hash) still resolves via `resolve`.
        self.name_index.insert(node.id(), id);
        let edges = node
            .deps
            .iter()
            .map(|dep| NodeId::Dep(self.add_dep(dep.clone())))
            .collect();
        self.edges.insert(NodeId::File(id), edges);
        self.files.insert(id, node);
        id
    }

    /// Content-address a dep edge and wire it to the file it names, if any.
    /// See [`Self::add_file`].
    pub fn add_dep(&mut self, node: DepNode) -> DepId {
        let id = DepId::from(&node);
        if self.deps.contains_key(&id) {
            return id;
        }
        let edges = node.file.map(NodeId::File).into_iter().collect();
        self.edges.insert(NodeId::Dep(id), edges);
        self.deps.insert(id, node);
        id
    }

    /// Content-address a goal and wire it to the `Dep` node it wraps. See
    /// [`Self::add_file`].
    pub fn add_goal_dep(&mut self, node: GoalDepNode) -> GoalDepId {
        let id = GoalDepId::from(&node);
        if self.goal_deps.contains_key(&id) {
            return id;
        }
        let dep_id = self.add_dep(node.dep.clone());
        self.edges
            .insert(NodeId::GoalDep(id), vec![NodeId::Dep(dep_id)]);
        self.goal_deps.insert(id, node);
        id
    }

    /// The nodes `id` points directly at. Empty if `id` was never added.
    pub fn edges_from(&self, id: NodeId) -> &[NodeId] {
        self.edges.get(&id).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Every node with a recorded edge to `id` — a reverse scan, since
    /// `edges` is only indexed forward.
    pub fn edges_to(&self, id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.edges
            .iter()
            .filter(move |(_, targets)| targets.contains(&id))
            .map(|(&from, _)| from)
    }

    pub fn file(&self, id: FileId) -> Option<&FileNode> {
        self.resolve(id)
    }

    pub fn dep(&self, id: DepId) -> Option<&DepNode> {
        self.deps.get(&id)
    }

    pub fn goal_dep(&self, id: GoalDepId) -> Option<&GoalDepNode> {
        self.goal_deps.get(&id)
    }

    /// A goal's target file: `GoalDep -> Dep -> File`, two hops through
    /// `edges`. `None` if the goal isn't in this graph, its `Dep` names no
    /// target, or the target was never added (under either identity).
    pub fn goal_target_file(&self, goal: GoalDepId) -> Option<&FileNode> {
        let &NodeId::Dep(dep_id) = self.edges_from(NodeId::GoalDep(goal)).first()? else {
            return None;
        };
        let &NodeId::File(target) = self.edges_from(NodeId::Dep(dep_id)).first()? else {
            return None;
        };
        self.resolve(target)
    }

    /// Prerequisite files of `id`: `File -> Dep -> File`, two hops.
    pub fn prerequisites(&self, id: FileId) -> impl Iterator<Item = &FileNode> {
        self.edges_from(NodeId::File(id))
            .iter()
            .filter_map(move |dep_node| {
                let &NodeId::Dep(dep_id) = dep_node else {
                    return None;
                };
                let &NodeId::File(target) = self.edges_from(NodeId::Dep(dep_id)).first()? else {
                    return None;
                };
                self.resolve(target)
            })
    }

    /// Dependent files of `id`: every file with a `Dep` edge that resolves
    /// to `id` — the reverse direction a bare `Option<FileId>` link can't
    /// answer at all. Both hops go through `edges_to`, which returns every
    /// match rather than just the first, so files that share a bit-for-bit
    /// identical prerequisite edge (content-addressing collapses the edge
    /// itself, not its owners) are all found, not just one.
    pub fn dependents(&self, id: FileId) -> impl Iterator<Item = &FileNode> + '_ {
        self.edges
            .iter()
            .filter(move |(from, targets)| {
                matches!(from, NodeId::Dep(_))
                    && targets
                        .iter()
                        .any(|t| matches!(t, NodeId::File(f) if self.same_file(*f, id)))
            })
            .flat_map(move |(&dep_node, _)| {
                self.edges_to(dep_node)
                    .filter_map(move |owner| match owner {
                        NodeId::File(file_id) => self.resolve(file_id),
                        _ => None,
                    })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{DepGraph, NodeId};
    use crate::dep::{DepNode, GoalDepNode};
    use crate::file::{FileId, FileNode};

    /// Add a source -> object -> program -> goal chain (mirroring `dep.rs`'s
    /// `goal_dep_id_tests::build_graph`, but through the graph's own API
    /// instead of loose `FileId::from` calls) and return the graph plus the
    /// goal's id.
    fn build(graph: &mut DepGraph, main_source_name: &str) -> crate::dep::GoalDepId {
        let main_c = graph.add_file(FileNode::new(main_source_name.as_bytes().to_vec()));
        let util_c = graph.add_file(FileNode::new(b"util.c".to_vec()));

        let mut main_o = FileNode::new(b"main.o".to_vec());
        main_o.deps.push(DepNode {
            name: main_source_name.to_string(),
            file: Some(main_c),
            is_explicit: true,
            ..Default::default()
        });
        let main_o = graph.add_file(main_o);

        let mut util_o = FileNode::new(b"util.o".to_vec());
        util_o.deps.push(DepNode {
            name: "util.c".to_string(),
            file: Some(util_c),
            is_explicit: true,
            ..Default::default()
        });
        let util_o = graph.add_file(util_o);

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
        let prog = graph.add_file(prog);

        graph.add_goal_dep(GoalDepNode {
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

        assert_eq!(graph.files.len(), 5); // main.c, util.c, main.o, util.o, prog
        assert_eq!(graph.goal_deps.len(), 1);

        let target = graph.goal_target_file(goal).expect("goal has a target");
        assert_eq!(target.name, b"prog");

        // `prog` has two prerequisites (main.o, util.o): each file's edges
        // are a Vec<NodeId>, so both survive as distinct entries.
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
    /// given a leaf, who depends on it? `dependents` walks `edges` in
    /// reverse via `edges_to`, not the forward `File -> Dep -> File` links.
    #[test]
    fn walks_dependents_in_reverse() {
        let mut graph = DepGraph::new();
        let goal = build(&mut graph, "main.c");
        let target = graph.goal_target_file(goal).unwrap();
        let prog_id = FileId::from(target);

        // `build` adds the leaf as an unmodified `FileNode::new(...)`, so
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

        let prog_dependents: Vec<&[u8]> = graph
            .dependents(FileId::from(main_o))
            .map(|f| f.name.as_slice())
            .collect();
        assert_eq!(prog_dependents, vec![&b"prog"[..]]);

        // Nothing depends on the root: `prog` has no incoming edges.
        assert_eq!(graph.dependents(prog_id).count(), 0);

        // Edge metadata round-trips through the dep store: the `Dep` node
        // targeting `main.c` is `main_o`'s own prerequisite edge.
        let NodeId::Dep(dep_id) = graph
            .edges_to(NodeId::File(main_c_id))
            .find(|n| matches!(n, NodeId::Dep(_)))
            .expect("a Dep node targets main.c")
        else {
            unreachable!()
        };
        assert_eq!(graph.dep(dep_id).unwrap().name, "main.c");
    }

    /// A real build-graph `DepNode.file` is always name-derived
    /// (`FileId::from_bytes(hname)`), never this graph's content-hash id.
    /// `resolve` (via `name_index`) must still find the target — the
    /// interop gap a pure content-hash-only lookup would silently miss.
    #[test]
    fn resolves_name_derived_live_graph_targets() {
        let mut graph = DepGraph::new();
        let leaf = FileNode::new(b"live.c".to_vec());
        let live_id = leaf.id(); // name-derived, NOT FileId::from(&leaf)
        graph.add_file(leaf);

        let mut obj = FileNode::new(b"live.o".to_vec());
        obj.deps.push(DepNode {
            name: "live.c".to_string(),
            file: Some(live_id),
            is_explicit: true,
            ..Default::default()
        });
        let obj_id = graph.add_file(obj);

        let resolved = graph
            .prerequisites(obj_id)
            .next()
            .expect("name-derived target resolves through name_index");
        assert_eq!(resolved.name, b"live.c");
    }

    #[test]
    fn adding_identical_content_twice_dedups_nodes_and_edges() {
        let mut graph = DepGraph::new();
        let leaf = graph.add_file(FileNode::new(b"shared.h".to_vec()));

        let mut a = FileNode::new(b"a.o".to_vec());
        a.deps.push(DepNode {
            name: "shared.h".to_string(),
            file: Some(leaf),
            ..Default::default()
        });
        let id_a = graph.add_file(a.clone());
        let id_a_again = graph.add_file(a);
        assert_eq!(id_a, id_a_again);
        assert_eq!(graph.files.len(), 2); // shared.h, a.o
                                          // The second, no-op add must not duplicate the edge.
        assert_eq!(graph.prerequisites(id_a).count(), 1);
        assert_eq!(graph.dependents(leaf).count(), 1);
    }

    /// Editing a leaf source and re-adding the whole chain produces a new id
    /// and new edges at every ancestor up to the goal (the ripple property),
    /// while the untouched chain's entries and edges are still present under
    /// their original ids — content addressing never overwrites, it only
    /// adds. The shared `util.o`/`util.c` subtree is reachable, as a
    /// dependent, from *both* programs: because `dependents` scans for every
    /// matching edge rather than stopping at the first, files sharing a
    /// bit-for-bit identical prerequisite aren't undercounted.
    #[test]
    fn edits_ripple_from_leaf_to_goal_without_losing_history() {
        let mut graph = DepGraph::new();
        let original_goal = build(&mut graph, "main.c");
        let tampered_goal = build(&mut graph, "main2.c");

        assert_ne!(original_goal, tampered_goal);
        // Both chains coexist: 5 original nodes + (main2.c, new main.o, new
        // prog) = 8. util.c/util.o are shared and not duplicated.
        assert_eq!(graph.files.len(), 8);
        assert_eq!(graph.goal_deps.len(), 2);

        let original_target = graph.goal_target_file(original_goal).unwrap();
        let tampered_target = graph.goal_target_file(tampered_goal).unwrap();
        assert_ne!(FileId::from(original_target), FileId::from(tampered_target));
        assert_eq!(original_target.name, b"prog");
        assert_eq!(tampered_target.name, b"prog");

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
        let phantom = FileId::from_bytes(b"never-added");
        assert!(graph.file(phantom).is_none());
        assert_eq!(graph.prerequisites(phantom).count(), 0);
        assert_eq!(graph.dependents(phantom).count(), 0);
        assert!(graph.edges_from(NodeId::File(phantom)).is_empty());
        assert_eq!(graph.edges_to(NodeId::File(phantom)).count(), 0);
    }
}
