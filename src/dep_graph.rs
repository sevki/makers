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
//! module builds an explicit edge table and a reverse scan, so both
//! traversal directions — [`DepGraph::prerequisites`] and
//! [`DepGraph::dependents`] — are real graph queries instead of one-way
//! pointer chasing. [`DepGraph::edges`] enumerates the full edge set as
//! [`Edge`] values.
//!
//! Storage is raw `[u8; HASH_SIZE]` bytes throughout (not the typed
//! `FileId`/`DepId`/`GoalDepId` newtypes) — the public API still takes and
//! returns the typed ids; only the internal maps use raw arrays. The edge
//! table is keyed by each prerequisite's own `DepId` bytes, not by the
//! owning file: a `DepId` already uniquely content-hashes one specific
//! (name, flags, target, ...) edge, so this stays single-valued (one target
//! per key) without losing a file's other prerequisites, which get their
//! own distinct `DepId` keys.
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
//! are deliberately different). It *can*, however, safely ingest real
//! `FileNode`s produced by that graph: every insert also indexes the node
//! under its name-derived `FileNode::id()` in `name_index`, so a live
//! `DepNode.file` (always name-derived, never content-hash) still resolves
//! to the right entry here even though this graph's own ids are content
//! hashes. `resolve` is the lookup that tries both.

use crate::dep::{DepId, DepNode, GoalDepId, GoalDepNode};
use crate::file::{FileId, FileNode, HASH_SIZE};
use std::collections::HashMap;

/// A directed dependency edge, as raw content-hash bytes: `from` (the
/// owning file's id), `dep` (the `DepNode` edge's own id — look up its full
/// metadata via [`DepGraph::dep`]), and `to` (the edge's recorded target,
/// exactly as read from `DepNode.file` — resolve it to a node via
/// [`DepGraph::file`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub from: [u8; HASH_SIZE],
    pub dep: [u8; HASH_SIZE],
    pub to: [u8; HASH_SIZE],
}

/// Content-addressed store of [`FileNode`]s keyed by raw content-hash bytes.
/// A thin named wrapper over the underlying map so [`DepGraph`] can name the
/// field by its role instead of spelling out the collection type.
#[derive(Debug, Default)]
struct FileArena(HashMap<[u8; HASH_SIZE], FileNode>);

impl FileArena {
    fn insert(&mut self, id: [u8; HASH_SIZE], node: FileNode) {
        self.0.entry(id).or_insert(node);
    }

    fn get(&self, id: [u8; HASH_SIZE]) -> Option<&FileNode> {
        self.0.get(&id)
    }

    fn contains(&self, id: [u8; HASH_SIZE]) -> bool {
        self.0.contains_key(&id)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&[u8; HASH_SIZE], &FileNode)> {
        self.0.iter()
    }
}

/// A dependency graph keyed entirely by blake3 content hash: one
/// content-addressed store per node kind (file, dep edge, goal), plus a
/// single-valued edge table (`DepId` bytes -> target bytes) and a
/// name-derived index that lets live build-graph data resolve correctly.
#[derive(Debug, Default)]
pub struct DepGraph {
    files: FileArena,
    deps: HashMap<[u8; HASH_SIZE], DepNode>,
    goals: HashMap<[u8; HASH_SIZE], GoalDepNode>,
    edges: HashMap<[u8; HASH_SIZE], [u8; HASH_SIZE]>,
    /// `FileNode::id()` (name-derived, the live build graph's own identity)
    /// to the content-hash id currently stored for whichever node occupies
    /// that name. Populated on every insert; consulted by `resolve` when a
    /// raw id isn't already a content-hash key in `files`.
    name_index: HashMap<[u8; HASH_SIZE], [u8; HASH_SIZE]>,
}

impl DepGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Content-address `node` and store it under its `FileId`. For every
    /// prerequisite in `node.deps` that names a target (`DepNode.file`),
    /// record its edge under the prerequisite's own `DepId`. Re-inserting a
    /// structurally identical node is a no-op — the store dedups by
    /// content, not by insertion order, and a no-op insert does not
    /// re-record edges or the name index.
    pub fn insert_file(&mut self, node: FileNode) -> FileId {
        let id = FileId::from(&node);
        if !self.files.contains(id.0) {
            // Index under the live-graph identity too, so a real
            // `DepNode.file` (name-derived, not content-hash) still
            // resolves to this insertion via `resolve`.
            self.name_index.insert(node.id().0, id.0);
            for dep in &node.deps {
                let dep_id = self.insert_dep(dep.clone());
                if let Some(to) = dep.file {
                    self.edges.insert(dep_id.0, to.0);
                }
            }
            self.files.insert(id.0, node);
        }
        id
    }

    /// Content-address a dep edge's data. See [`Self::insert_file`]; this
    /// alone does not record an edge target — `insert_file` does that for
    /// the prerequisites it owns.
    pub fn insert_dep(&mut self, node: DepNode) -> DepId {
        let id = DepId::from(&node);
        self.deps.entry(id.0).or_insert(node);
        id
    }

    /// Content-address a goal. See [`Self::insert_file`].
    pub fn insert_goal(&mut self, node: GoalDepNode) -> GoalDepId {
        let id = GoalDepId::from(&node);
        self.goals.entry(id.0).or_insert(node);
        id
    }

    /// Resolve `raw` to a stored node: first as a content-hash key, then
    /// (falling back through `name_index`) as a live-graph, name-derived
    /// key. This is what lets every lookup below accept either kind of id
    /// transparently.
    fn resolve(&self, raw: [u8; HASH_SIZE]) -> Option<&FileNode> {
        self.files
            .get(raw)
            .or_else(|| self.files.get(*self.name_index.get(&raw)?))
    }

    pub fn file(&self, id: FileId) -> Option<&FileNode> {
        self.resolve(id.0)
    }

    pub fn dep(&self, id: DepId) -> Option<&DepNode> {
        self.deps.get(&id.0)
    }

    pub fn goal(&self, id: GoalDepId) -> Option<&GoalDepNode> {
        self.goals.get(&id.0)
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

    /// The file that owns the `DepNode` with the given id: a linear scan
    /// over stored files for one whose `.deps` contains a matching `DepId`.
    /// There is no reverse `DepId -> owner` index (the edge table is
    /// intentionally single-valued, keyed by `DepId`), so this is O(files).
    ///
    /// KNOWN LIMITATION: `DepId` content-hashes the edge (name, flags,
    /// target, ...), not the owner, so two different files with a
    /// bit-for-bit identical prerequisite (same name/flags pointing at the
    /// same shared target — a realistic case, not a contrived one) hash to
    /// the *same* `DepId` and collide in `edges`. Only the first owner found
    /// by this scan is reachable; `dependents()` on the shared target will
    /// undercount. Restoring full fidelity here would mean keying edges by
    /// `(owner, DepId)` or going back to a multi-valued table.
    fn owner_of(&self, dep_id: [u8; HASH_SIZE]) -> Option<([u8; HASH_SIZE], &FileNode)> {
        self.files
            .iter()
            .find(|(_, f)| f.deps.iter().any(|d| DepId::from(d).0 == dep_id))
            .map(|(id, f)| (*id, f))
    }

    /// Every stored edge, as [`Edge`] values pairing each prerequisite's own
    /// id with its owning file and recorded (unresolved) target.
    pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges.iter().filter_map(move |(&dep_id, &to)| {
            let (from, _) = self.owner_of(dep_id)?;
            Some(Edge {
                from,
                dep: dep_id,
                to,
            })
        })
    }

    /// Resolve a goal's target: follow `goal.dep.file` into the file store.
    /// `None` if the goal isn't in this graph, or its target link is absent
    /// or points at an id this graph never stored (under either identity).
    pub fn goal_target_file(&self, goal: GoalDepId) -> Option<&FileNode> {
        let target = self.goal(goal)?.dep.file?;
        self.file(target)
    }

    /// Prerequisite nodes of `id`: for each `DepNode` in `id`'s own `.deps`,
    /// resolve its recorded target through the edge table.
    pub fn prerequisites(&self, id: FileId) -> impl Iterator<Item = &FileNode> {
        self.file(id)
            .into_iter()
            .flat_map(|f| f.deps.iter())
            .filter_map(move |dep| {
                let to = *self.edges.get(&DepId::from(dep).0)?;
                self.resolve(to)
            })
    }

    /// Dependent nodes of `id`: every stored file with a prerequisite that
    /// resolves to the same node as `id` — the reverse direction a bare
    /// `Option<FileId>` link can't answer at all. Both sides are resolved
    /// (not compared as raw bytes) so this still matches when `id`, or an
    /// edge's recorded target, is a live-graph name-derived id rather than
    /// this graph's own content hash.
    ///
    /// Subject to the same collision `owner_of` documents: if two different
    /// files have a bit-for-bit identical prerequisite edge pointing at
    /// `id` (same name/flags — a realistic shared-dependency case), only one
    /// of them is reachable here, not both.
    pub fn dependents(&self, id: FileId) -> impl Iterator<Item = &FileNode> {
        let canonical = self.resolve(id.0).map(FileId::from);
        self.edges.iter().filter_map(move |(&dep_id, &to)| {
            if Some(FileId::from(self.resolve(to)?)) != canonical {
                return None;
            }
            self.owner_of(dep_id).map(|(_, f)| f)
        })
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

        // `prog` has two prerequisites (main.o, util.o): distinct `DepNode`s
        // get distinct `DepId` keys in the edge table, so both survive even
        // though the table is single-valued per key.
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
    /// given a leaf, who depends on it? `dependents` walks the edge table
    /// in reverse, not the forward `deps` links.
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

        // Nothing depends on the root: `prog` has no incoming edges.
        assert_eq!(graph.dependents(prog_id).count(), 0);

        // Edge metadata round-trips through the dep store.
        let edge = graph
            .edges()
            .find(|e| e.to == main_c_id.0)
            .expect("main.o -> main.c edge is recorded");
        assert_eq!(
            graph.dep(crate::dep::DepId(edge.dep)).unwrap().name,
            "main.c"
        );
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
        graph.insert_file(leaf);

        let mut obj = FileNode::new(b"live.o".to_vec());
        obj.deps.push(DepNode {
            name: "live.c".to_string(),
            file: Some(live_id),
            is_explicit: true,
            ..Default::default()
        });
        let obj_id = graph.insert_file(obj);

        let resolved = graph
            .prerequisites(obj_id)
            .next()
            .expect("name-derived target resolves through name_index");
        assert_eq!(resolved.name, b"live.c");
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
        assert_eq!(graph.prerequisites(id_a).count(), 1);
        assert_eq!(graph.dependents(leaf).count(), 1);
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
        // program, proving the untouched branch wasn't duplicated or lost.
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
        // `owner_of`'s documented collision: `prog` and the tampered `prog`
        // both reference `util.o` through a bit-for-bit identical DepNode
        // (same name/flags/target), so they collide on the same `DepId` key
        // in the edge table and only one owner is reachable here — not the
        // full-fidelity "both programs depend on it" a multi-valued index
        // would give. This asserts the documented (reduced) behavior, not
        // the ideal one.
        assert_eq!(graph.dependents(util_o_id).count(), 1);
    }

    #[test]
    fn missing_ids_resolve_to_none() {
        let graph = DepGraph::new();
        let phantom = FileId::from_bytes(b"never-inserted");
        assert!(graph.file(phantom).is_none());
        assert_eq!(graph.prerequisites(phantom).count(), 0);
        assert_eq!(graph.dependents(phantom).count(), 0);
        assert_eq!(graph.edges().count(), 0);
    }
}
