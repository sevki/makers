//! Nested node sets — the host side of `makers:plugin/graph.node-set`.
//!
//! The problem this solves is the one Bazel's `depset` solves. A plugin that
//! accumulates information up the dependency graph (transitive include
//! paths, link lines, licence sets — the shape of essentially every
//! non-trivial aspect) writes, at every node, "my answer is my own
//! contribution plus my children's". If "plus" copies, the total work is
//! quadratic in graph size: each node's list is rebuilt in full at every one
//! of its dependents.
//!
//! So [`union`](Sets::union) does not copy. It appends one node to an arena
//! and returns its index; the members are enumerated only when someone asks
//! for them, once, with the result memoised. `transitive` is lazier still —
//! it names a closure the host has not walked yet, so a plugin that unions
//! ten transitive closures and then filters them pays for one traversal at
//! the end rather than ten up front.

use rustc_hash::{FxHashMap, FxHashSet};
use std::sync::Arc;

use crate::depgraph::{DepGraph, EdgeKind, NodeId};

/// Index of a set in [`Sets`]. Small and `Copy`, so the WIT resource that
/// wraps it stays a plain integer.
pub type SetId = u32;

/// One node of the set DAG.
enum SetNode {
    /// Exactly these nodes, in this order, before deduplication.
    Leaf(Vec<NodeId>),
    /// The union of two sets; left members come first.
    Union(SetId, SetId),
    /// Everything reachable from this node over prerequisite/also-make
    /// edges, excluding the node itself. Never walked until flattened.
    Transitive(NodeId),
}

/// The arena of live sets for one plugin instance.
#[derive(Default)]
pub struct Sets {
    arena: Vec<SetNode>,
    /// Flattened members, memoised per set. `Arc` so a repeated `to-list`
    /// (or a `contains` after a `to-list`) is free.
    flat: FxHashMap<SetId, Arc<Vec<NodeId>>>,
}

impl Sets {
    /// A set with exactly these members.
    pub fn leaf(&mut self, nodes: Vec<NodeId>) -> SetId {
        self.push(SetNode::Leaf(nodes))
    }

    /// The transitive closure of `node`'s prerequisites, unwalked.
    pub fn transitive(&mut self, node: NodeId) -> SetId {
        self.push(SetNode::Transitive(node))
    }

    /// `a ∪ b`, in constant time. Neither operand is traversed.
    pub fn union(&mut self, a: SetId, b: SetId) -> SetId {
        self.push(SetNode::Union(a, b))
    }

    fn push(&mut self, node: SetNode) -> SetId {
        let id = self.arena.len() as SetId;
        self.arena.push(node);
        id
    }

    /// Cheap emptiness test: never flattens, and stops at the first member
    /// it can prove exists.
    pub fn is_empty(&self, id: SetId, graph: &DepGraph) -> bool {
        if let Some(flat) = self.flat.get(&id) {
            return flat.is_empty();
        }
        let mut stack = vec![id];
        while let Some(cur) = stack.pop() {
            match self.arena.get(cur as usize) {
                Some(SetNode::Leaf(nodes)) if !nodes.is_empty() => return false,
                Some(SetNode::Leaf(_)) | None => {}
                Some(SetNode::Union(a, b)) => {
                    stack.push(*a);
                    stack.push(*b);
                }
                // A closure is non-empty iff the node has at least one
                // outgoing dependency edge — no need to walk it.
                Some(SetNode::Transitive(n)) => {
                    if child_edges(graph, *n).next().is_some() {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Members, deduplicated, in the documented order: within a union the
    /// left operand's members come first, and a node appears at the position
    /// of its *first* occurrence. Backs the WIT `node-set.to-list`; named
    /// `flatten` here because it takes `&mut self` to memoise, which a
    /// `to_*` conversion is not supposed to do.
    ///
    /// Order is part of the contract rather than an implementation detail
    /// because plugins turn these lists into files — a link line, a compile
    /// database, a Ninja build — and an unspecified order makes every one of
    /// those artifacts differ run to run for no reason.
    pub fn flatten(&mut self, id: SetId, graph: &DepGraph) -> Arc<Vec<NodeId>> {
        if let Some(flat) = self.flat.get(&id) {
            return Arc::clone(flat);
        }
        let mut seen = FxHashSet::default();
        let mut out = Vec::new();
        self.collect(id, graph, &mut seen, &mut out);
        let flat = Arc::new(out);
        self.flat.insert(id, Arc::clone(&flat));
        flat
    }

    pub fn contains(&mut self, id: SetId, needle: NodeId, graph: &DepGraph) -> bool {
        self.flatten(id, graph).contains(&needle)
    }

    /// Iterative left-to-right walk of the set DAG. Recursion would be
    /// bounded by union depth, which a plugin controls; a build with a
    /// 100k-deep accumulation should not blow the host's stack.
    fn collect(
        &self,
        id: SetId,
        graph: &DepGraph,
        seen: &mut FxHashSet<NodeId>,
        out: &mut Vec<NodeId>,
    ) {
        let mut stack = vec![id];
        while let Some(cur) = stack.pop() {
            match self.arena.get(cur as usize) {
                None => {}
                Some(SetNode::Leaf(nodes)) => {
                    for &n in nodes {
                        if seen.insert(n) {
                            out.push(n);
                        }
                    }
                }
                // Pushed right-then-left so the left operand pops first.
                Some(SetNode::Union(a, b)) => {
                    stack.push(*b);
                    stack.push(*a);
                }
                Some(SetNode::Transitive(root)) => {
                    closure_into(graph, *root, seen, out);
                }
            }
        }
    }
}

/// Dependency edges a plugin can see: prerequisites, grouped-target
/// siblings, and — from the synthetic root — the goals. Rule-provenance and
/// rename bookkeeping edges are host-internal and never part of a set.
pub fn child_edges(graph: &DepGraph, from: NodeId) -> impl Iterator<Item = NodeId> + '_ {
    graph
        .edges_from(from)
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                EdgeKind::Goal(_) | EdgeKind::Prerequisite(_) | EdgeKind::AlsoMake(_)
            )
        })
        .map(|e| e.to)
}

/// Breadth-order closure from `root`, excluding `root` itself, appending to
/// `out` in first-seen order. Uses the caller's `seen` set so that a union
/// of overlapping closures visits each node once in total.
fn closure_into(
    graph: &DepGraph,
    root: NodeId,
    seen: &mut FxHashSet<NodeId>,
    out: &mut Vec<NodeId>,
) {
    let mut queue: std::collections::VecDeque<NodeId> = child_edges(graph, root).collect();
    let mut queued: FxHashSet<NodeId> = queue.iter().copied().collect();
    while let Some(n) = queue.pop_front() {
        if seen.insert(n) {
            out.push(n);
        }
        for child in child_edges(graph, n) {
            if queued.insert(child) {
                queue.push_back(child);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::{FileId, FileNode};

    fn file(name: &str) -> NodeId {
        NodeId::File(FileId::from_bytes(name.as_bytes()))
    }

    /// A union is a constant-time link, and flattening it puts the left
    /// operand first with duplicates collapsing onto their first occurrence.
    #[test]
    fn union_is_ordered_and_deduplicated() {
        let graph = DepGraph::new();
        let mut sets = Sets::default();
        let a = sets.leaf(vec![file("a"), file("b")]);
        let b = sets.leaf(vec![file("b"), file("c")]);
        let u = sets.union(a, b);
        assert_eq!(
            *sets.flatten(u, &graph),
            vec![file("a"), file("b"), file("c")]
        );
    }

    /// Flattening is memoised: the second call returns the same allocation,
    /// which is what keeps `contains` from being quadratic when a plugin
    /// probes a set once per node.
    #[test]
    fn flattening_is_memoised() {
        let graph = DepGraph::new();
        let mut sets = Sets::default();
        let a = sets.leaf(vec![file("a")]);
        let first = sets.flatten(a, &graph);
        let second = sets.flatten(a, &graph);
        assert!(Arc::ptr_eq(&first, &second));
    }

    /// A deep chain of unions flattens without recursing — the guard against
    /// a plugin that accumulates one union per graph node blowing the host's
    /// stack.
    #[test]
    fn deep_union_chains_do_not_recurse() {
        let graph = DepGraph::new();
        let mut sets = Sets::default();
        let mut acc = sets.leaf(vec![file("n0")]);
        for i in 1..50_000 {
            let leaf = sets.leaf(vec![file(&format!("n{i}"))]);
            acc = sets.union(acc, leaf);
        }
        assert_eq!(sets.flatten(acc, &graph).len(), 50_000);
    }

    /// `is-empty` answers from the set structure without materialising it.
    #[test]
    fn emptiness_does_not_flatten() {
        let graph = DepGraph::new();
        let mut sets = Sets::default();
        let empty = sets.leaf(vec![]);
        let also_empty = sets.union(empty, empty);
        assert!(sets.is_empty(also_empty, &graph));
        let one = sets.leaf(vec![file("a")]);
        let non_empty = sets.union(also_empty, one);
        assert!(!sets.is_empty(non_empty, &graph));
        assert!(sets.flat.is_empty(), "is-empty must not populate the memo");
    }

    /// A transitive closure is a lazy member of the set DAG: building it
    /// costs nothing, and flattening walks the real graph.
    #[test]
    fn transitive_closures_are_lazy_and_complete() {
        let mut graph = DepGraph::new();
        let prog = graph.add_file(FileNode::new(b"prog".to_vec()));
        let obj = graph.add_file(FileNode::new(b"main.o".to_vec()));
        let src = graph.add_file(FileNode::new(b"main.c".to_vec()));
        graph.add_dep(
            prog,
            crate::dep::DepNode {
                name: "main.o".to_string(),
                file: Some(obj),
                ..Default::default()
            },
        );
        graph.add_dep(
            obj,
            crate::dep::DepNode {
                name: "main.c".to_string(),
                file: Some(src),
                ..Default::default()
            },
        );

        let mut sets = Sets::default();
        let closure = sets.transitive(NodeId::File(prog));
        assert!(!sets.is_empty(closure, &graph));
        assert_eq!(
            *sets.flatten(closure, &graph),
            vec![NodeId::File(obj), NodeId::File(src)]
        );
    }
}
