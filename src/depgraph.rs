//! The unified dependency graph — files, dep edges, and goals in one
//! traversable structure, with a [`salsa`]-backed incremental query layer.
//!
//! # Shape
//!
//! The graph follows the idiomatic node/edge split already established by
//! [`FileNode`]/[`DepNode`]/[`GoalDepNode`]:
//!
//! * **Nodes** are files (keyed by the same name-derived [`FileId`] the arena
//!   `ExecContext::filenodes` interns under), pattern rules (keyed by
//!   semantic [`RuleId`], with [`EdgeKind::DerivedBy`] provenance edges
//!   linking each matched target to the rule that produced it), plus one
//!   synthetic [`NodeId::Root`] standing for "the command line". Recipes are
//!   deliberately *payload*, not nodes: a recipe has no identity apart from
//!   its target(s), and the one structural fact it implies — several targets
//!   sharing one invocation — is the [`EdgeKind::AlsoMake`] edge.
//! * **Edges** are typed ([`EdgeKind`]): a prerequisite or `also_make` edge
//!   carries its [`DepId`] so the full [`DepNode`] payload (flags, stem,
//!   order-only, `.WAIT`) stays reachable; a goal edge from [`NodeId::Root`]
//!   carries its [`GoalDepId`]; `renamed`/`parent` inter-file links become
//!   payload-free edges. Edge *order is semantic* (it drives `$<`/`$^` and
//!   `.WAIT`), so adjacency is a `Vec`, never a set.
//!
//! Dep and goal payloads are content-addressed (interned by their BLAKE3
//! [`DepId`]/[`GoalDepId`]), so structurally identical edges share one stored
//! payload. File nodes are keyed by *name*-derived ids on purpose — identity
//! must survive content mutation (see the note on `id_wireformat!` in
//! `file.rs`) — which keeps every edge valid across rebuilds of the same
//! target.
//!
//! # Traversal and analysis
//!
//! [`DepGraph`] is a plain, lock-free snapshot: build it (via [`DepGraph::add_file`]
//! / [`DepGraph::add_goal`] or [`DepGraph::from_context`]) and query it.
//! Traversal state (visited sets, DFS colors) lives in the walkers, not on the
//! nodes — the graph-layer replacement for the legacy `considered`/`updating`
//! generation counters threaded through `File`. Analyses come in dependency
//! order ([`DepGraph::topo_order`]), reverse direction
//! ([`DepGraph::affected_by`], [`DepGraph::dependents`]), cycle reporting
//! ([`DepGraph::find_cycle`], the graph-level form of make's "Circular X <- Y
//! dependency dropped"), and rendering — Graphviz ([`DepGraph::to_dot`]) and
//! Mermaid ([`DepGraph::to_mermaid`], which GitHub renders natively in PRs;
//! `docs/depgraph-sample.md` is a test-maintained snapshot of it).
//!
//! # salsa integration
//!
//! [`DepGraphDb`] wraps the snapshot in the session salsa database type
//! ([`crate::makedb::MakeDb`], shared with the string interner and the
//! parser's AST nodes). The
//! whole [`DepGraph`] is a single `#[salsa::input]` and each analysis is a
//! `#[salsa::tracked]` query: results are memoized, re-queries are free, and
//! [`DepGraphDb::set_graph`] bumps the revision so downstream queries
//! re-validate (equal results backdate, so consumers of an unchanged answer
//! are not re-run). Parameterized queries ([`DepGraphDb::affected_by`]) rely
//! on salsa's automatic argument interning and memoize per file. This is
//! deliberately coarse-grained — one input for the whole graph; carving the
//! input into per-node salsa inputs (so an mtime touch only invalidates the
//! queries that read that node) is the planned next step once the legacy
//! `*mut File` graph is gone.

use std::collections::{BTreeMap, BTreeSet};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::dep::{DepId, DepNode, GoalDepId, GoalDepNode};
use crate::execctx::ExecContext;
use crate::file::{FileId, FileNode};
use crate::rule::{Rule, RuleId};

/// A node in the dependency graph: a file, or the synthetic root standing for
/// "the command line" (whose out-edges are the goals, in command-line order).
///
/// `Ord` gives analyses a deterministic iteration order over otherwise
/// unordered hash-map storage: `Root` first, then files by id bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeId {
    /// The synthetic entry point make was asked to build from.
    Root,
    /// A file/target node, keyed exactly as the `ExecContext::filenodes` arena
    /// keys it (name-derived `FileId`).
    File(FileId),
    /// A pattern (implicit) rule from the rule database, keyed by its
    /// semantic content hash (see the `ContentHash` impl in `rule.rs`).
    Rule(RuleId),
}

/// What a graph edge *is*. Prerequisite/also-make edges carry the [`DepId`] of
/// their interned [`DepNode`] payload — order-only (`ignore_mtime`), `.WAIT`
/// (`wait_here`), static-pattern stems and the rest stay one lookup away.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeKind {
    /// `Root -> target`: a goal from the command line (or an `include`),
    /// carrying its [`GoalDepNode`] payload.
    Goal(GoalDepId),
    /// `target -> prerequisite`: the target needs the prerequisite built
    /// first. The payload's `ignore_mtime` marks an order-only prerequisite.
    Prerequisite(DepId),
    /// `target -> sibling`: both are produced by the same recipe
    /// (`also_make`).
    AlsoMake(DepId),
    /// `old -> new`: the file was rekeyed by `rehash_file`.
    Renamed,
    /// `intermediate -> parent` in an implicit-rule chain.
    Parent,
    /// `file -> rule`: provenance — this target's deps/recipe came from
    /// matching that pattern rule (`pattern_search`).
    DerivedBy(RuleId),
    /// `rule -> pattern dep`: the rule-database view of a rule's own
    /// prerequisite patterns (e.g. `%.o: %.c` — target `%.c`).
    RulePrerequisite(DepId),
}

/// A directed edge to `to`. The source is the key the edge is stored under in
/// [`DepGraph::edges_from`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    pub to: NodeId,
    pub kind: EdgeKind,
}

/// A dependency cycle: `path[0]` is depended on (transitively) by each
/// following node, and the last node depends back on `path[0]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cycle {
    pub path: Vec<NodeId>,
}

impl Cycle {
    /// Render the cycle with file names, e.g. `a -> b -> c -> a` — the
    /// graph-level counterpart of make's "Circular b <- a dependency" message.
    pub fn describe(&self, graph: &DepGraph) -> String {
        let names: Vec<String> = self.path.iter().map(|&n| graph.display_name(n)).collect();
        format!("{} -> {}", names.join(" -> "), names[0])
    }
}

/// An immutable snapshot of the whole dependency graph. See the module docs
/// for the node/edge model; see [`DepGraphDb`] for the memoized query layer.
#[derive(Debug, Clone, Default)]
pub struct DepGraph {
    /// File nodes, keyed by their arena identity (`FileNode::id()`).
    files: FxHashMap<FileId, FileNode>,
    /// Interned dep-edge payloads (content-addressed, shared by identical
    /// edges).
    deps: FxHashMap<DepId, DepNode>,
    /// Interned goal payloads.
    goals: FxHashMap<GoalDepId, GoalDepNode>,
    /// Pattern-rule payloads, keyed by semantic content hash.
    rules: FxHashMap<RuleId, Rule>,
    /// Display names learned from dep edges for targets never added as file
    /// nodes (pattern prerequisites like `%.c`, not-yet-entered files) —
    /// diagnostics fallback only, never identity.
    names: FxHashMap<FileId, Vec<u8>>,
    /// Forward adjacency. Order within a `Vec` is prerequisite order and is
    /// semantic; do not sort.
    edges: FxHashMap<NodeId, Vec<Edge>>,
    /// Reverse adjacency: for each node, the sources of its incoming edges
    /// (one entry per edge occurrence).
    redges: FxHashMap<NodeId, Vec<NodeId>>,
}

impl DepGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Snapshot the live build graph: every file node in the arena plus the
    /// goal list. Arena iteration order is nondeterministic, so nodes are
    /// inserted sorted by name — two snapshots of the same state are
    /// structurally identical.
    pub fn from_context(ctx: &ExecContext, goals: &[GoalDepNode]) -> Self {
        let mut nodes: Vec<FileNode> = {
            let arena = ctx.filenodes.0.lock().unwrap_or_else(|e| e.into_inner());
            arena
                .values()
                .map(|node| node.lock().unwrap_or_else(|e| e.into_inner()).clone())
                .collect()
        };
        nodes.sort_by(|a, b| a.hname.cmp(&b.hname));

        let mut graph = DepGraph::new();
        for node in nodes {
            let matched_rule = node.matched_rule;
            let id = graph.add_file(node);
            // Build-time provenance recorded by `pattern_search`
            // (`FileNode::matched_rule`) becomes a `DerivedBy` edge.
            if let Some(rule) = matched_rule {
                graph.record_rule_match(id, rule);
            }
        }
        for goal in goals {
            graph.add_goal(goal.clone());
        }
        graph
    }

    // ------------------------------------------------------------------ //
    // Building                                                            //
    // ------------------------------------------------------------------ //

    /// Insert a file node and wire its edges (prerequisites, `also_make`,
    /// double-colon entries' prerequisites, `renamed`, `parent`) from the data
    /// already on the node. Re-adding a file replaces its payload and
    /// out-edges. Returns the node's arena identity.
    ///
    /// Edges may point at files not (yet) added; such targets still
    /// participate in traversals, they just have no [`FileNode`] payload.
    pub fn add_file(&mut self, node: FileNode) -> FileId {
        let id = node.id();
        let from = NodeId::File(id);
        self.remove_out_edges(from);
        self.edges.entry(from).or_default();

        for dep in &node.deps {
            self.wire_dep(from, dep, false);
        }
        for entry in &node.double_colon {
            for dep in &entry.deps {
                self.wire_dep(from, dep, false);
            }
        }
        for dep in &node.also_make {
            self.wire_dep(from, dep, true);
        }
        if let Some(renamed) = node.renamed {
            self.add_edge(
                from,
                Edge {
                    to: NodeId::File(renamed),
                    kind: EdgeKind::Renamed,
                },
            );
        }
        if let Some(parent) = node.parent {
            self.add_edge(
                from,
                Edge {
                    to: NodeId::File(parent),
                    kind: EdgeKind::Parent,
                },
            );
        }

        self.files.insert(id, node);
        id
    }

    /// Record a goal: interns the payload and wires a [`EdgeKind::Goal`] edge
    /// from [`NodeId::Root`] to the goal's target, preserving command-line
    /// order across calls.
    pub fn add_goal(&mut self, goal: GoalDepNode) -> GoalDepId {
        let id = GoalDepId::from(&goal);
        let target = Self::dep_target(&goal.dep);
        self.learn_name(target, &goal.dep);
        self.goals.insert(id, goal);
        self.add_edge(
            NodeId::Root,
            Edge {
                to: NodeId::File(target),
                kind: EdgeKind::Goal(id),
            },
        );
        id
    }

    /// Append one prerequisite edge from `from`, keeping the stored
    /// [`FileNode`] payload in sync when `from` has been added (the new dep is
    /// pushed onto its `deps` so payload and adjacency agree).
    pub fn add_dep(&mut self, from: FileId, dep: DepNode) -> DepId {
        if let Some(node) = self.files.get_mut(&from) {
            node.deps.push(dep.clone());
        }
        let target = Self::dep_target(&dep);
        let id = self.intern_dep(&dep);
        self.add_edge(
            NodeId::File(from),
            Edge {
                to: NodeId::File(target),
                kind: EdgeKind::Prerequisite(id),
            },
        );
        id
    }

    /// Insert a pattern rule and wire its [`EdgeKind::RulePrerequisite`]
    /// edges (targets are the rule's own dep patterns, e.g. `%.c`, resolved
    /// by name hash like any other dep). The rule's printable definition is
    /// computed eagerly so [`DepGraph::display_name`] can label the node
    /// without mutation. Re-adding a rule (same semantic content = same
    /// [`RuleId`]) replaces its payload and out-edges. Which *files* a rule
    /// produced is separate provenance — see
    /// [`DepGraph::record_rule_match`].
    pub fn add_rule(&mut self, mut rule: Rule) -> RuleId {
        let id = RuleId::from(&rule);
        let _ = rule.rule_defn();
        let from = NodeId::Rule(id);
        self.remove_out_edges(from);
        self.edges.entry(from).or_default();
        for dep in rule.deps.clone() {
            let target = Self::dep_target(&dep);
            self.learn_name(target, &dep);
            let dep_id = self.intern_dep(&dep);
            self.add_edge(
                from,
                Edge {
                    to: NodeId::File(target),
                    kind: EdgeKind::RulePrerequisite(dep_id),
                },
            );
        }
        self.rules.insert(id, rule);
        id
    }

    /// Record that `file`'s implicit-rule match resolved to `rule`
    /// (provenance from `pattern_search`): wires a [`EdgeKind::DerivedBy`]
    /// edge, so "which rule built this" and "which files did this rule
    /// build" are both plain adjacency queries.
    pub fn record_rule_match(&mut self, file: FileId, rule: RuleId) {
        self.add_edge(
            NodeId::File(file),
            Edge {
                to: NodeId::Rule(rule),
                kind: EdgeKind::DerivedBy(rule),
            },
        );
    }

    /// A dep edge's target file id: the resolved `dep.file` when the reader
    /// bound one, else derived from the dep's name — the same name-hash the
    /// arena would intern that file under, so the edge and a later
    /// `add_file` of the real node meet at the same [`FileId`].
    fn dep_target(dep: &DepNode) -> FileId {
        dep.file
            .unwrap_or_else(|| FileId::from_bytes(dep.name.as_bytes()))
    }

    /// Remember a dep-derived display name for a target that may never be
    /// added as a file node (see the `names` field).
    fn learn_name(&mut self, target: FileId, dep: &DepNode) {
        if !dep.name.is_empty() {
            self.names
                .entry(target)
                .or_insert_with(|| dep.name.as_bytes().to_vec());
        }
    }

    fn intern_dep(&mut self, dep: &DepNode) -> DepId {
        let id = DepId::from(dep);
        self.deps.entry(id).or_insert_with(|| dep.clone());
        id
    }

    fn wire_dep(&mut self, from: NodeId, dep: &DepNode, also_make: bool) {
        let target = Self::dep_target(dep);
        self.learn_name(target, dep);
        let id = self.intern_dep(dep);
        let kind = if also_make {
            EdgeKind::AlsoMake(id)
        } else {
            EdgeKind::Prerequisite(id)
        };
        self.add_edge(
            from,
            Edge {
                to: NodeId::File(target),
                kind,
            },
        );
    }

    fn add_edge(&mut self, from: NodeId, edge: Edge) {
        self.redges.entry(edge.to).or_default().push(from);
        self.edges.entry(from).or_default().push(edge);
    }

    fn remove_out_edges(&mut self, from: NodeId) {
        if let Some(old) = self.edges.remove(&from) {
            for edge in old {
                if let Some(sources) = self.redges.get_mut(&edge.to) {
                    if let Some(pos) = sources.iter().position(|&s| s == from) {
                        sources.remove(pos);
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------ //
    // Node and edge access                                                //
    // ------------------------------------------------------------------ //

    pub fn file(&self, id: FileId) -> Option<&FileNode> {
        self.files.get(&id)
    }

    pub fn dep(&self, id: DepId) -> Option<&DepNode> {
        self.deps.get(&id)
    }

    pub fn goal(&self, id: GoalDepId) -> Option<&GoalDepNode> {
        self.goals.get(&id)
    }

    pub fn rule(&self, id: RuleId) -> Option<&Rule> {
        self.rules.get(&id)
    }

    pub fn rules(&self) -> impl Iterator<Item = (RuleId, &Rule)> {
        self.rules.iter().map(|(&id, rule)| (id, rule))
    }

    /// The rule `f` was derived from, if provenance was recorded
    /// ([`DepGraph::record_rule_match`]).
    pub fn rule_for(&self, f: FileId) -> Option<RuleId> {
        self.edges_from(NodeId::File(f)).iter().find_map(|edge| {
            let EdgeKind::DerivedBy(id) = edge.kind else {
                return None;
            };
            Some(id)
        })
    }

    /// Every file recorded as derived from `rule`, deduplicated, first-seen
    /// order — "what did this pattern rule actually build".
    pub fn files_derived_by(&self, rule: RuleId) -> Vec<FileId> {
        let mut seen = FxHashSet::default();
        self.redges
            .get(&NodeId::Rule(rule))
            .map(Vec::as_slice)
            .unwrap_or(&[])
            .iter()
            .filter_map(|&source| match source {
                NodeId::File(id) if seen.insert(id) => Some(id),
                _ => None,
            })
            .collect()
    }

    pub fn files(&self) -> impl Iterator<Item = (FileId, &FileNode)> {
        self.files.iter().map(|(&id, node)| (id, node))
    }

    /// Goals in command-line order (the order of [`NodeId::Root`]'s
    /// out-edges).
    pub fn goals(&self) -> impl Iterator<Item = (GoalDepId, &GoalDepNode)> {
        self.edges_from(NodeId::Root).iter().filter_map(|edge| {
            let EdgeKind::Goal(id) = edge.kind else {
                return None;
            };
            self.goals.get(&id).map(|goal| (id, goal))
        })
    }

    /// Every node the graph knows of: added files, the root (once a goal
    /// exists), and any edge target never added as a file. Sorted (`BTreeSet`)
    /// so analyses over "all nodes" are deterministic.
    pub fn node_ids(&self) -> BTreeSet<NodeId> {
        let mut ids: BTreeSet<NodeId> = self.files.keys().map(|&id| NodeId::File(id)).collect();
        for (&from, edges) in &self.edges {
            ids.insert(from);
            for edge in edges {
                ids.insert(edge.to);
            }
        }
        ids
    }

    /// Out-edges of `n` in prerequisite order (empty for unknown nodes).
    pub fn edges_from(&self, n: NodeId) -> &[Edge] {
        self.edges.get(&n).map(Vec::as_slice).unwrap_or(&[])
    }

    /// This target's prerequisites, in order, with their [`DepNode`] payloads
    /// (order-only prerequisites included — filter on `dep.ignore_mtime`).
    pub fn prerequisites(&self, f: FileId) -> impl Iterator<Item = (FileId, &DepNode)> {
        self.edges_from(NodeId::File(f)).iter().filter_map(|edge| {
            let EdgeKind::Prerequisite(id) = edge.kind else {
                return None;
            };
            let NodeId::File(to) = edge.to else {
                return None;
            };
            self.deps.get(&id).map(|dep| (to, dep))
        })
    }

    /// The nodes with an edge *into* `f` — "who needs this file" (its direct
    /// dependents, plus [`NodeId::Root`] when `f` is a goal). Deduplicated,
    /// first-seen order.
    pub fn dependents(&self, f: FileId) -> Vec<NodeId> {
        let mut seen = FxHashSet::default();
        self.redges
            .get(&NodeId::File(f))
            .map(Vec::as_slice)
            .unwrap_or(&[])
            .iter()
            .copied()
            .filter(|&source| seen.insert(source))
            .collect()
    }

    /// Nodes with no incoming edges (always includes [`NodeId::Root`] when
    /// goals exist; otherwise the graph's unreferenced targets). Sorted.
    pub fn roots(&self) -> Vec<NodeId> {
        self.node_ids()
            .into_iter()
            .filter(|n| self.redges.get(n).is_none_or(Vec::is_empty))
            .collect()
    }

    /// Nodes with no outgoing edges — the pure sources of the build (files
    /// that are never targets). Sorted.
    pub fn leaves(&self) -> Vec<NodeId> {
        self.node_ids()
            .into_iter()
            .filter(|&n| self.edges_from(n).is_empty())
            .collect()
    }

    /// Human-readable node name for diagnostics/DOT/Mermaid: the file's name
    /// as written (falling back to a dep-learned name for targets never
    /// added, then to the id), `<root>` for the root, and the printable
    /// definition (`%.o: %.c`) for rules.
    pub fn display_name(&self, n: NodeId) -> String {
        match n {
            NodeId::Root => "<root>".to_string(),
            NodeId::File(id) => match self.files.get(&id) {
                Some(node) => String::from_utf8_lossy(&node.name).into_owned(),
                None => match self.names.get(&id) {
                    Some(name) => String::from_utf8_lossy(name).into_owned(),
                    None => format!("<file {id}>"),
                },
            },
            NodeId::Rule(id) => match self.rules.get(&id).and_then(|rule| rule.defn.as_ref()) {
                Some(defn) => String::from_utf8_lossy(defn).into_owned(),
                None => format!("<rule {id}>"),
            },
        }
    }

    // ------------------------------------------------------------------ //
    // Traversal                                                           //
    // ------------------------------------------------------------------ //

    /// Depth-first preorder over the forward edges, visiting each node once.
    /// The visited set lives in the iterator, not on the nodes.
    pub fn dfs(&self, roots: impl IntoIterator<Item = NodeId>) -> Dfs<'_> {
        let mut stack: Vec<NodeId> = roots.into_iter().collect();
        stack.reverse();
        Dfs {
            graph: self,
            stack,
            visited: FxHashSet::default(),
        }
    }

    /// Every node reachable from `roots` along forward edges (including the
    /// roots themselves).
    pub fn reachable(&self, roots: impl IntoIterator<Item = NodeId>) -> FxHashSet<NodeId> {
        self.dfs(roots).collect()
    }

    /// Every node in this build: reachable from the goals. The complement
    /// (over [`DepGraph::node_ids`]) is the set of defined-but-unused targets.
    pub fn reachable_from_goals(&self) -> FxHashSet<NodeId> {
        self.reachable([NodeId::Root])
    }

    /// Everything `f` transitively needs (prerequisites of prerequisites,
    /// …), excluding `f` itself.
    pub fn transitive_prerequisites(&self, f: FileId) -> FxHashSet<FileId> {
        let start = NodeId::File(f);
        self.dfs([start])
            .filter_map(|n| match n {
                NodeId::File(id) if n != start => Some(id),
                _ => None,
            })
            .collect()
    }

    /// Reverse reachability: the `changed` files plus every file that
    /// transitively depends on one of them — exactly the set a change
    /// invalidates (the query behind `-W`/watch-mode style analyses).
    pub fn affected_by(&self, changed: &[FileId]) -> FxHashSet<FileId> {
        let mut out = FxHashSet::default();
        let mut stack: Vec<NodeId> = changed.iter().map(|&f| NodeId::File(f)).collect();
        let mut visited: FxHashSet<NodeId> = stack.iter().copied().collect();
        while let Some(n) = stack.pop() {
            if let NodeId::File(id) = n {
                out.insert(id);
            }
            if let Some(sources) = self.redges.get(&n) {
                for &source in sources {
                    if visited.insert(source) {
                        stack.push(source);
                    }
                }
            }
        }
        out
    }

    /// A shortest forward path `from -> … -> to` (inclusive), or `None` if
    /// `to` is not reachable — "why does building X involve Y".
    pub fn path_between(&self, from: NodeId, to: NodeId) -> Option<Vec<NodeId>> {
        if from == to {
            return Some(vec![from]);
        }
        let mut parent: FxHashMap<NodeId, NodeId> = FxHashMap::default();
        let mut queue = std::collections::VecDeque::from([from]);
        let mut visited: FxHashSet<NodeId> = [from].into_iter().collect();
        while let Some(n) = queue.pop_front() {
            for edge in self.edges_from(n) {
                if !visited.insert(edge.to) {
                    continue;
                }
                parent.insert(edge.to, n);
                if edge.to == to {
                    let mut path = vec![to];
                    let mut cur = to;
                    while let Some(&p) = parent.get(&cur) {
                        path.push(p);
                        cur = p;
                    }
                    path.reverse();
                    return Some(path);
                }
                queue.push_back(edge.to);
            }
        }
        None
    }

    // ------------------------------------------------------------------ //
    // Analysis                                                            //
    // ------------------------------------------------------------------ //

    /// All nodes in dependency order — every prerequisite before every target
    /// that needs it (so it is a valid sequential build order, goals last).
    /// Deterministic. Fails with the offending [`Cycle`] if the graph has one.
    pub fn topo_order(&self) -> Result<Vec<NodeId>, Cycle> {
        self.dfs_postorder(self.node_ids())
    }

    /// [`DepGraph::topo_order`] restricted to what `roots` reach.
    pub fn topo_order_from(
        &self,
        roots: impl IntoIterator<Item = NodeId>,
    ) -> Result<Vec<NodeId>, Cycle> {
        self.dfs_postorder(roots)
    }

    /// The first dependency cycle found, if any.
    pub fn find_cycle(&self) -> Option<Cycle> {
        self.topo_order().err()
    }

    /// Iterative three-color DFS emitting postorder (= dependency order: a
    /// node is emitted only after everything it points at). A gray node
    /// re-entered while still on the active path is a back-edge; the path
    /// suffix from its first occurrence is the cycle.
    fn dfs_postorder(&self, roots: impl IntoIterator<Item = NodeId>) -> Result<Vec<NodeId>, Cycle> {
        enum Frame {
            Enter(NodeId),
            Exit(NodeId),
        }
        #[derive(PartialEq)]
        enum Color {
            Gray,
            Black,
        }

        let mut colors: FxHashMap<NodeId, Color> = FxHashMap::default();
        let mut path: Vec<NodeId> = Vec::new();
        let mut out: Vec<NodeId> = Vec::new();
        let mut stack: Vec<Frame> = Vec::new();

        for root in roots {
            stack.push(Frame::Enter(root));
            while let Some(frame) = stack.pop() {
                match frame {
                    Frame::Enter(n) => match colors.get(&n) {
                        Some(Color::Black) => {}
                        Some(Color::Gray) => {
                            let pos = path
                                .iter()
                                .position(|&p| p == n)
                                .expect("gray node must be on the active DFS path");
                            return Err(Cycle {
                                path: path[pos..].to_vec(),
                            });
                        }
                        None => {
                            colors.insert(n, Color::Gray);
                            path.push(n);
                            stack.push(Frame::Exit(n));
                            for edge in self.edges_from(n).iter().rev() {
                                stack.push(Frame::Enter(edge.to));
                            }
                        }
                    },
                    Frame::Exit(n) => {
                        colors.insert(n, Color::Black);
                        path.pop();
                        out.push(n);
                    }
                }
            }
        }
        Ok(out)
    }

    /// Graphviz DOT rendering of the whole graph, deterministic across runs.
    /// Goal edges are bold, order-only prerequisites dashed, `also_make`
    /// dotted, rename/parent links gray; phony targets get dashed borders and
    /// targets with a recipe are boxes.
    pub fn to_dot(&self) -> String {
        fn escape(name: &str) -> String {
            name.replace('\\', "\\\\").replace('"', "\\\"")
        }

        let ids: Vec<NodeId> = self.node_ids().into_iter().collect();
        let mut out = String::from("digraph deps {\n  rankdir=LR;\n");
        for &n in &ids {
            let name = escape(&self.display_name(n));
            let attrs = match n {
                NodeId::Root => " [shape=diamond]".to_string(),
                NodeId::File(id) => match self.files.get(&id) {
                    Some(node) => {
                        let shape = if node.recipe.is_some() {
                            "box"
                        } else {
                            "ellipse"
                        };
                        let style = if node.phony { ", style=dashed" } else { "" };
                        format!(" [shape={shape}{style}]")
                    }
                    None => " [shape=ellipse, color=gray]".to_string(),
                },
                NodeId::Rule(_) => " [shape=component, color=blue]".to_string(),
            };
            out.push_str(&format!("  \"{name}\"{attrs};\n"));
        }
        for &from in &ids {
            let from_name = escape(&self.display_name(from));
            for edge in self.edges_from(from) {
                let to_name = escape(&self.display_name(edge.to));
                let attrs = match edge.kind {
                    EdgeKind::Goal(_) => " [style=bold]",
                    EdgeKind::Prerequisite(id) => {
                        if self.deps.get(&id).is_some_and(|d| d.ignore_mtime) {
                            " [style=dashed, label=\"order-only\"]"
                        } else {
                            ""
                        }
                    }
                    EdgeKind::AlsoMake(_) => " [style=dotted, label=\"also\"]",
                    EdgeKind::Renamed => " [color=gray, label=\"renamed\"]",
                    EdgeKind::Parent => " [color=gray, label=\"parent\"]",
                    EdgeKind::DerivedBy(_) => " [color=blue, style=dashed, label=\"rule\"]",
                    EdgeKind::RulePrerequisite(_) => " [color=blue, style=dotted]",
                };
                out.push_str(&format!("  \"{from_name}\" -> \"{to_name}\"{attrs};\n"));
            }
        }
        out.push_str("}\n");
        out
    }

    /// Mermaid `flowchart` rendering of the whole graph, deterministic across
    /// runs. GitHub renders Mermaid natively in Markdown (PR descriptions,
    /// comments, and `.md` files), so this is the "paste the build graph into
    /// the PR" format. Same visual vocabulary as [`DepGraph::to_dot`]: goal
    /// edges thick, order-only/`also_make` dotted with labels, rule
    /// provenance dotted; the root is a rhombus, recipe-bearing targets are
    /// rectangles, plain files are stadiums, rules are subroutine boxes, and
    /// phony targets get a dashed class.
    pub fn to_mermaid(&self) -> String {
        // Mermaid quoted labels: only `"` is problematic; `#quot;` is the
        // documented escape.
        fn escape(name: &str) -> String {
            name.replace('"', "#quot;")
        }

        let ids: Vec<NodeId> = self.node_ids().into_iter().collect();
        let key = |n: NodeId| {
            let idx = ids.binary_search(&n).expect("node listed");
            format!("n{idx}")
        };

        let mut out = String::from("flowchart LR\n");
        let mut phony: Vec<String> = Vec::new();
        let mut rules: Vec<String> = Vec::new();
        for &n in &ids {
            let name = escape(&self.display_name(n));
            let shape = match n {
                NodeId::Root => format!("{{\"{name}\"}}"),
                NodeId::File(id) => match self.files.get(&id) {
                    Some(node) => {
                        if node.phony {
                            phony.push(key(n));
                        }
                        if node.recipe.is_some() {
                            format!("[\"{name}\"]")
                        } else {
                            format!("([\"{name}\"])")
                        }
                    }
                    None => format!("([\"{name}\"])"),
                },
                NodeId::Rule(_) => {
                    rules.push(key(n));
                    format!("[[\"{name}\"]]")
                }
            };
            out.push_str(&format!("  {}{shape}\n", key(n)));
        }
        for &from in &ids {
            for edge in self.edges_from(from) {
                let arrow = match edge.kind {
                    EdgeKind::Goal(_) => "==>".to_string(),
                    EdgeKind::Prerequisite(id) => {
                        if self.deps.get(&id).is_some_and(|d| d.ignore_mtime) {
                            "-.->|order-only|".to_string()
                        } else {
                            "-->".to_string()
                        }
                    }
                    EdgeKind::AlsoMake(_) => "-.->|also|".to_string(),
                    EdgeKind::Renamed => "-->|renamed|".to_string(),
                    EdgeKind::Parent => "-->|parent|".to_string(),
                    EdgeKind::DerivedBy(_) => "-.->|rule|".to_string(),
                    EdgeKind::RulePrerequisite(_) => "-.->".to_string(),
                };
                out.push_str(&format!("  {} {arrow} {}\n", key(from), key(edge.to)));
            }
        }
        if !phony.is_empty() {
            out.push_str("  classDef phony stroke-dasharray:5 5;\n");
            out.push_str(&format!("  class {} phony;\n", phony.join(",")));
        }
        if !rules.is_empty() {
            out.push_str("  classDef rule stroke:#36c,stroke-dasharray:3 3;\n");
            out.push_str(&format!("  class {} rule;\n", rules.join(",")));
        }
        out
    }

    /// [`DepGraph::to_mermaid`] wrapped in a fenced ```mermaid Markdown block,
    /// ready to drop into a PR description, comment, or committed `.md` file.
    pub fn to_mermaid_markdown(&self) -> String {
        format!("```mermaid\n{}```\n", self.to_mermaid())
    }
}

/// Debug hook for the real make binary: when the `MAKERS_DEPGRAPH`
/// environment variable is set, snapshot the live build graph — the whole
/// `filenodes` arena, the goal list, and the pattern-rule database — and
/// write it to that path. Called from `main_0` after makefiles are read,
/// deps snapped, and rules installed, right before goal shuffling/updating,
/// so the dump is the deterministic "what make knows before building" view.
///
/// The format follows the extension: `.dot` Graphviz, `.mmd` raw Mermaid,
/// anything else (canonically `.md`) a fenced Mermaid Markdown block that
/// GitHub renders.
///
/// Failures to write are reported on stderr but never fail the build: this
/// is a diagnostics tap, not a build step.
pub fn dump_graph_if_requested(ctx: &ExecContext, goals: &[GoalDepNode]) {
    dump_graph_env(ctx, goals, "MAKERS_DEPGRAPH");
}

/// The post-walk counterpart of [`dump_graph_if_requested`]: when
/// `MAKERS_DEPGRAPH_POST` is set, snapshot the graph again after
/// `update_goal_chain` finishes — the *resolved* view, where implicit-rule
/// matching has run, so pattern-derived prerequisites (`main.o -> main.c`),
/// stems, intermediate `parent` chains, and `DerivedBy` rule-provenance
/// edges (from `FileNode::matched_rule`) are all present. Same path/format
/// conventions; setting both variables yields a planned-vs-discovered pair.
pub fn dump_graph_post_if_requested(ctx: &ExecContext, goals: &[GoalDepNode]) {
    dump_graph_env(ctx, goals, "MAKERS_DEPGRAPH_POST");
}

/// Post-walk diagnostics hook for `--dump-bazel`: snapshot the resolved graph
/// after `update_goal_chain` and emit one `BUILD.bazel` per directory that
/// owns recipe-bearing targets.
pub fn dump_bazel_post_if_requested(ctx: &ExecContext, goals: &[GoalDepNode], enabled: bool) {
    if !enabled {
        return;
    }
    let mut graph = DepGraph::from_context(ctx, goals);
    crate::rule::with_pattern_rules(ctx, |rules| {
        for rule in rules {
            graph.add_rule(rule.clone());
        }
    });
    if let Err(err) = write_bazel_files(&graph) {
        eprintln!("make: cannot write BUILD.bazel files: {err}");
    }
}

fn dump_graph_env(ctx: &ExecContext, goals: &[GoalDepNode], var: &str) {
    let Some(path) = std::env::var_os(var) else {
        return;
    };
    let path = std::path::PathBuf::from(path);

    let mut graph = DepGraph::from_context(ctx, goals);
    crate::rule::with_pattern_rules(ctx, |rules| {
        for rule in rules {
            graph.add_rule(rule.clone());
        }
    });

    let out = match path.extension().and_then(|ext| ext.to_str()) {
        Some("dot") => graph.to_dot(),
        Some("mmd") => graph.to_mermaid(),
        _ => graph.to_mermaid_markdown(),
    };
    if let Err(err) = std::fs::write(&path, out) {
        eprintln!(
            "make: cannot write dependency graph to {}: {err}",
            path.display()
        );
    }
}

#[derive(Debug, Clone)]
struct BazelTarget {
    rule_name: String,
    output: String,
    srcs: Vec<String>,
    cmd: String,
}

fn write_bazel_files(graph: &DepGraph) -> std::io::Result<()> {
    let mut targets: Vec<(String, FileId)> = graph
        .files()
        .filter_map(|(id, node)| {
            if node.recipe.is_some() && !node.phony {
                Some((String::from_utf8_lossy(&node.name).into_owned(), id))
            } else {
                None
            }
        })
        .collect();
    targets.sort_by(|a, b| a.0.cmp(&b.0));

    let mut by_package: BTreeMap<String, Vec<BazelTarget>> = BTreeMap::new();
    let mut seen_rule_names: BTreeMap<String, std::collections::HashSet<String>> = BTreeMap::new();
    for (target_name, id) in targets {
        if target_name.is_empty() || target_name.contains('%') {
            continue;
        }
        let Some(node) = graph.file(id) else {
            continue;
        };
        let (pkg, output_name) = package_and_output(&target_name);
        if output_name.is_empty() {
            continue;
        }
        let mut rule_name = sanitize_rule_name(&output_name);
        let names = seen_rule_names.entry(pkg.clone()).or_default();
        if !names.insert(rule_name.clone()) {
            let base = rule_name.clone();
            let mut suffix = 2usize;
            while !names.insert(format!("{base}_{suffix}")) {
                suffix += 1;
            }
            rule_name = format!("{base}_{suffix}");
        }

        let mut srcs = Vec::new();
        let mut seen_srcs = std::collections::HashSet::new();
        for (dep_file, dep) in graph.prerequisites(id) {
            let dep_name = if dep.name.is_empty() {
                graph.display_name(NodeId::File(dep_file))
            } else {
                dep.name.clone()
            };
            if dep_name.is_empty() || dep_name.contains('%') {
                continue;
            }
            let Some(label) = dep_label_for_pkg(&dep_name, &pkg) else {
                continue;
            };
            if seen_srcs.insert(label.clone()) {
                srcs.push(label);
            }
        }
        srcs.sort_unstable();

        let lines: Vec<String> = match &node.recipe {
            Some(recipe) if !recipe.lines.is_empty() => recipe
                .lines
                .iter()
                .map(|line| String::from_utf8_lossy(&line.text).trim().to_string())
                .filter(|line| !line.is_empty())
                .collect(),
            Some(recipe) => String::from_utf8_lossy(&recipe.text)
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(ToString::to_string)
                .collect(),
            None => Vec::new(),
        };
        let cmd = if lines.is_empty() {
            "true".to_string()
        } else {
            lines.join(" && ")
        };
        by_package.entry(pkg).or_default().push(BazelTarget {
            rule_name,
            output: output_name,
            srcs,
            cmd,
        });
    }

    for (pkg, mut targets) in by_package {
        targets.sort_by(|a, b| a.output.cmp(&b.output));
        let mut out = String::from(
            "# Generated by `make --dump-bazel` after dependency resolution.\n\
             # This is a best-effort translation of make targets to Bazel genrules.\n\n",
        );
        for target in targets {
            out.push_str("genrule(\n");
            out.push_str(&format!(
                "    name = \"{}\",\n",
                bazel_escape(&target.rule_name)
            ));
            if target.srcs.is_empty() {
                out.push_str("    srcs = [],\n");
            } else {
                out.push_str("    srcs = [\n");
                for src in &target.srcs {
                    out.push_str(&format!("        \"{}\",\n", bazel_escape(src)));
                }
                out.push_str("    ],\n");
            }
            out.push_str(&format!(
                "    outs = [\"{}\"],\n",
                bazel_escape(&target.output)
            ));
            out.push_str(&format!("    cmd = \"{}\",\n", bazel_escape(&target.cmd)));
            out.push_str(")\n\n");
        }

        let path = if pkg.is_empty() {
            std::path::PathBuf::from("BUILD.bazel")
        } else {
            std::path::PathBuf::from(&pkg).join("BUILD.bazel")
        };
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(path, out)?;
    }

    Ok(())
}

fn package_and_output(path: &str) -> (String, String) {
    let path = path.replace('\\', "/");
    match path.rsplit_once('/') {
        Some((dir, file)) => (dir.to_string(), file.to_string()),
        None => (String::new(), path),
    }
}

fn dep_label_for_pkg(dep_name: &str, pkg: &str) -> Option<String> {
    let dep = dep_name.replace('\\', "/");
    if dep.is_empty() {
        return None;
    }
    let (dep_pkg, dep_out) = package_and_output(&dep);
    if dep_out.is_empty() {
        return None;
    }
    if dep_pkg == pkg {
        Some(dep_out)
    } else if dep_pkg.is_empty() {
        Some(format!("//:{dep_out}"))
    } else {
        Some(format!("//{dep_pkg}:{dep_out}"))
    }
}

fn sanitize_rule_name(name: &str) -> String {
    let mut out = String::new();
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        out.push_str("target");
    }
    if out.as_bytes()[0].is_ascii_digit() {
        out.insert(0, '_');
    }
    out
}

fn bazel_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Preorder DFS iterator over forward edges; see [`DepGraph::dfs`].
pub struct Dfs<'g> {
    graph: &'g DepGraph,
    stack: Vec<NodeId>,
    visited: FxHashSet<NodeId>,
}

impl Iterator for Dfs<'_> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        while let Some(n) = self.stack.pop() {
            if !self.visited.insert(n) {
                continue;
            }
            for edge in self.graph.edges_from(n).iter().rev() {
                if !self.visited.contains(&edge.to) {
                    self.stack.push(edge.to);
                }
            }
            return Some(n);
        }
        None
    }
}

// ---------------------------------------------------------------------- //
// salsa front-end                                                         //
// ---------------------------------------------------------------------- //

/// The whole graph snapshot as a single (coarse-grained, see module docs)
/// salsa input. Replacing it via its setter bumps the salsa revision and
/// invalidates every tracked query below.
#[salsa::input]
struct GraphInput {
    #[returns(ref)]
    graph: DepGraph,
}

#[salsa::tracked]
fn topo_order_query(db: &dyn salsa::Database, input: GraphInput) -> Result<Vec<NodeId>, Cycle> {
    input.graph(db).topo_order()
}

#[salsa::tracked]
fn find_cycle_query(db: &dyn salsa::Database, input: GraphInput) -> Option<Cycle> {
    input.graph(db).find_cycle()
}

/// Sorted so the memoized value is canonical (set iteration order would leak
/// hash-map nondeterminism into backdating comparisons).
#[salsa::tracked]
fn reachable_from_goals_query(db: &dyn salsa::Database, input: GraphInput) -> Vec<NodeId> {
    let mut nodes: Vec<NodeId> = input.graph(db).reachable_from_goals().into_iter().collect();
    nodes.sort_unstable();
    nodes
}

/// Parameterized query: salsa interns the `(input, changed)` argument tuple,
/// so the result is memoized per changed file.
#[salsa::tracked]
fn affected_by_query(db: &dyn salsa::Database, input: GraphInput, changed: FileId) -> Vec<FileId> {
    let mut files: Vec<FileId> = input
        .graph(db)
        .affected_by(&[changed])
        .into_iter()
        .collect();
    files.sort_unstable();
    files
}

#[salsa::tracked]
fn dot_query(db: &dyn salsa::Database, input: GraphInput) -> String {
    input.graph(db).to_dot()
}

#[salsa::tracked]
fn mermaid_query(db: &dyn salsa::Database, input: GraphInput) -> String {
    input.graph(db).to_mermaid()
}

/// A [`DepGraph`] snapshot plus its memoized analysis queries. Queries run at
/// most once per graph revision; [`DepGraphDb::set_graph`] starts the next
/// revision.
pub struct DepGraphDb {
    db: crate::makedb::MakeDb,
    input: GraphInput,
}

impl DepGraphDb {
    pub fn new(graph: DepGraph) -> Self {
        let db = crate::makedb::MakeDb::default();
        let input = GraphInput::new(&db, graph);
        DepGraphDb { db, input }
    }

    /// The current snapshot.
    pub fn graph(&self) -> &DepGraph {
        self.input.graph(&self.db)
    }

    /// Replace the snapshot, invalidating memoized queries. Queries whose
    /// recomputed result is unchanged backdate (their consumers stay valid).
    pub fn set_graph(&mut self, graph: DepGraph) {
        use salsa::Setter as _;
        self.input.set_graph(&mut self.db).to(graph);
    }

    /// How many tracked queries have actually executed (as opposed to being
    /// answered from cache) over this database's lifetime.
    pub fn executions(&self) -> u64 {
        self.db.executions()
    }

    pub fn topo_order(&self) -> Result<Vec<NodeId>, Cycle> {
        topo_order_query(&self.db, self.input).clone()
    }

    pub fn find_cycle(&self) -> Option<Cycle> {
        find_cycle_query(&self.db, self.input).clone()
    }

    pub fn reachable_from_goals(&self) -> Vec<NodeId> {
        reachable_from_goals_query(&self.db, self.input).to_vec()
    }

    /// Union of the per-file memoized reverse-reachability queries, sorted
    /// and deduplicated.
    pub fn affected_by(&self, changed: &[FileId]) -> Vec<FileId> {
        let mut out: Vec<FileId> = changed
            .iter()
            .flat_map(|&f| affected_by_query(&self.db, self.input, f).clone())
            .collect();
        out.sort_unstable();
        out.dedup();
        out
    }

    pub fn to_dot(&self) -> String {
        dot_query(&self.db, self.input).to_string()
    }

    pub fn to_mermaid(&self) -> String {
        mermaid_query(&self.db, self.input).to_string()
    }
}

impl std::ops::Deref for DepGraphDb {
    /// Everything not needing memoization (plain accessors, one-off walks)
    /// passes through to the snapshot.
    type Target = DepGraph;

    fn deref(&self) -> &DepGraph {
        self.graph()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `prog` links `main.o` + `util.o`, each compiled from its source — the
    /// same four-node shape as `dep.rs`'s goal-id tests, here with arena
    /// (name-derived) ids throughout. `main.o -> main.c` binds `dep.file`
    /// explicitly; `util.o -> util.c` leaves it `None` to exercise name-based
    /// target resolution. Returns the graph and the ids of interest.
    fn sample_graph() -> (DepGraph, [FileId; 5]) {
        let mut graph = DepGraph::new();

        let dep_on = |name: &str, file: Option<FileId>| DepNode {
            name: name.to_string(),
            file,
            is_explicit: true,
            ..Default::default()
        };

        let main_c = graph.add_file(FileNode::new(b"main.c".to_vec()));
        let util_c = graph.add_file(FileNode::new(b"util.c".to_vec()));

        let mut main_o_node = FileNode::new(b"main.o".to_vec());
        main_o_node.deps.push(dep_on("main.c", Some(main_c)));
        main_o_node.recipe = Some(Default::default());
        let main_o = graph.add_file(main_o_node);

        let mut util_o_node = FileNode::new(b"util.o".to_vec());
        util_o_node.deps.push(dep_on("util.c", None));
        util_o_node.recipe = Some(Default::default());
        let util_o = graph.add_file(util_o_node);

        let mut prog_node = FileNode::new(b"prog".to_vec());
        prog_node.deps.push(dep_on("main.o", Some(main_o)));
        prog_node.deps.push(dep_on("util.o", Some(util_o)));
        prog_node.recipe = Some(Default::default());
        let prog = graph.add_file(prog_node);

        graph.add_goal(GoalDepNode {
            dep: dep_on("prog", Some(prog)),
            ..Default::default()
        });

        (graph, [main_c, util_c, main_o, util_o, prog])
    }

    /// A three-node cycle `a -> b -> c -> a`.
    fn cyclic_graph() -> (DepGraph, [FileId; 3]) {
        let mut graph = DepGraph::new();
        let a = graph.add_file(FileNode::new(b"a".to_vec()));
        let b = graph.add_file(FileNode::new(b"b".to_vec()));
        let c = graph.add_file(FileNode::new(b"c".to_vec()));
        graph.add_dep(
            a,
            DepNode {
                name: "b".to_string(),
                file: Some(b),
                ..Default::default()
            },
        );
        graph.add_dep(
            b,
            DepNode {
                name: "c".to_string(),
                file: Some(c),
                ..Default::default()
            },
        );
        graph.add_dep(
            c,
            DepNode {
                name: "a".to_string(),
                file: Some(a),
                ..Default::default()
            },
        );
        (graph, [a, b, c])
    }

    /// Name-based target resolution: a dep with `file: None` lands on the
    /// same node a real `add_file` of that name creates.
    #[test]
    fn adjacency_and_payloads() {
        let (graph, [main_c, util_c, main_o, util_o, prog]) = sample_graph();

        let prereqs: Vec<FileId> = graph.prerequisites(prog).map(|(to, _)| to).collect();
        assert_eq!(prereqs, vec![main_o, util_o], "prerequisite order is kept");

        // util.o's dep had no resolved `file`; the name hash must land on
        // the interned util.c node.
        let util_prereqs: Vec<FileId> = graph.prerequisites(util_o).map(|(to, _)| to).collect();
        assert_eq!(util_prereqs, vec![util_c]);

        // Payloads are reachable through the edge.
        let (_, dep) = graph
            .prerequisites(main_o)
            .next()
            .expect("main.o has a dep");
        assert_eq!(dep.name, "main.c");
        assert!(dep.is_explicit);

        assert_eq!(graph.dependents(main_c), vec![NodeId::File(main_o)]);
        assert_eq!(graph.dependents(prog), vec![NodeId::Root]);

        let (_, goal) = graph.goals().next().expect("one goal");
        assert_eq!(goal.dep.name, "prog");

        assert_eq!(graph.display_name(NodeId::File(prog)), "prog");
        assert_eq!(graph.display_name(NodeId::Root), "<root>");
    }

    #[test]
    fn topo_order_puts_prerequisites_first() {
        let (graph, [main_c, _, main_o, _, prog]) = sample_graph();
        let order = graph.topo_order().expect("acyclic");

        let pos = |n: NodeId| order.iter().position(|&x| x == n).expect("in order");
        assert!(pos(NodeId::File(main_c)) < pos(NodeId::File(main_o)));
        assert!(pos(NodeId::File(main_o)) < pos(NodeId::File(prog)));
        assert!(
            pos(NodeId::File(prog)) < pos(NodeId::Root),
            "goals come last"
        );
        assert_eq!(order.len(), 6, "five files plus the root");

        // Deterministic across identically-built graphs.
        assert_eq!(order, sample_graph().0.topo_order().unwrap());
    }

    #[test]
    fn traversal_reachability_and_degrees() {
        let (mut graph, [main_c, util_c, main_o, util_o, prog]) = sample_graph();
        let orphan = graph.add_file(FileNode::new(b"orphan".to_vec()));

        let in_build = graph.reachable_from_goals();
        assert_eq!(in_build.len(), 6, "root plus the five build files");
        assert!(in_build.contains(&NodeId::File(util_c)));
        assert!(
            !in_build.contains(&NodeId::File(orphan)),
            "unused target is not part of the build"
        );

        assert_eq!(
            graph.transitive_prerequisites(prog),
            [main_o, util_o, main_c, util_c].into_iter().collect()
        );

        assert_eq!(
            graph.roots(),
            vec![NodeId::Root, NodeId::File(orphan)],
            "root and the orphan have no incoming edges"
        );
        let leaves = graph.leaves();
        assert!(leaves.contains(&NodeId::File(main_c)));
        assert!(leaves.contains(&NodeId::File(util_c)));
        assert!(!leaves.contains(&NodeId::File(prog)));

        // DFS from the root visits every build node exactly once, root first.
        let visited: Vec<NodeId> = graph.dfs([NodeId::Root]).collect();
        assert_eq!(visited[0], NodeId::Root);
        assert_eq!(visited.len(), 6);
    }

    #[test]
    fn affected_by_ripples_to_dependents() {
        let (graph, [main_c, util_c, main_o, util_o, prog]) = sample_graph();

        assert_eq!(
            graph.affected_by(&[main_c]),
            [main_c, main_o, prog].into_iter().collect(),
            "a source edit invalidates its object and the program, not the sibling"
        );
        assert_eq!(
            graph.affected_by(&[main_c, util_c]),
            [main_c, util_c, main_o, util_o, prog].into_iter().collect()
        );
        assert_eq!(
            graph.affected_by(&[prog]),
            [prog].into_iter().collect(),
            "nothing depends on the top-level program"
        );
    }

    #[test]
    fn path_between_explains_reachability() {
        let (graph, [main_c, util_c, main_o, _, prog]) = sample_graph();

        assert_eq!(
            graph.path_between(NodeId::Root, NodeId::File(main_c)),
            Some(vec![
                NodeId::Root,
                NodeId::File(prog),
                NodeId::File(main_o),
                NodeId::File(main_c),
            ])
        );
        assert_eq!(
            graph.path_between(NodeId::File(main_c), NodeId::File(util_c)),
            None,
            "sources do not reach each other"
        );
        assert_eq!(
            graph.path_between(NodeId::File(prog), NodeId::File(prog)),
            Some(vec![NodeId::File(prog)])
        );
    }

    #[test]
    fn cycles_are_detected_and_described() {
        let (graph, [a, b, c]) = cyclic_graph();

        let cycle = graph.find_cycle().expect("a -> b -> c -> a");
        assert_eq!(cycle.path.len(), 3);
        for id in [a, b, c] {
            assert!(cycle.path.contains(&NodeId::File(id)));
        }
        // The description closes the loop: first node repeated at the end.
        let described = cycle.describe(&graph);
        assert_eq!(described.matches("->").count(), 3);
        assert!(graph.topo_order().is_err());

        // Self-loop: the smallest cycle.
        let mut graph = DepGraph::new();
        let f = graph.add_file(FileNode::new(b"self".to_vec()));
        graph.add_dep(
            f,
            DepNode {
                name: "self".to_string(),
                file: Some(f),
                ..Default::default()
            },
        );
        let cycle = graph.find_cycle().expect("self-loop");
        assert_eq!(cycle.path, vec![NodeId::File(f)]);

        let (graph, _) = sample_graph();
        assert_eq!(graph.find_cycle(), None);
    }

    #[test]
    fn re_adding_a_file_replaces_its_edges() {
        let (mut graph, [main_c, util_c, _, _, _]) = sample_graph();

        // Rewrite main.o to depend on util.c instead of main.c.
        let mut node = FileNode::new(b"main.o".to_vec());
        node.deps.push(DepNode {
            name: "util.c".to_string(),
            file: Some(util_c),
            ..Default::default()
        });
        let main_o = graph.add_file(node);

        let prereqs: Vec<FileId> = graph.prerequisites(main_o).map(|(to, _)| to).collect();
        assert_eq!(prereqs, vec![util_c]);
        assert_eq!(
            graph.dependents(main_c),
            Vec::<NodeId>::new(),
            "the stale reverse edge is gone"
        );
    }

    /// The `%.o: %.c` pattern rule as an owned [`Rule`].
    fn object_rule() -> Rule {
        Rule {
            targets: vec![b"%.o".to_vec()],
            suffixes: vec![1],
            lens: vec![3],
            deps: vec![DepNode {
                name: "%.c".to_string(),
                ..Default::default()
            }],
            cmds: Some(Default::default()),
            defn: None,
            num: 1,
            terminal: false,
            in_use: false,
        }
    }

    /// [`sample_graph`] extended to exercise every node/edge flavor: a phony
    /// order-only prerequisite, an `also_make` sibling, and the `%.o: %.c`
    /// pattern rule with provenance recorded for both objects. This is the
    /// graph rendered into `docs/depgraph-sample.md`.
    fn showcase_graph() -> DepGraph {
        let (mut graph, [_, _, main_o, util_o, prog]) = sample_graph();

        let mut outdir = FileNode::new(b"outdir".to_vec());
        outdir.phony = true;
        let outdir = graph.add_file(outdir);
        graph.add_dep(
            prog,
            DepNode {
                name: "outdir".to_string(),
                file: Some(outdir),
                ignore_mtime: true,
                ..Default::default()
            },
        );

        let mut util_d = FileNode::new(b"util.d".to_vec());
        util_d.also_make.push(DepNode {
            name: "util.o".to_string(),
            file: Some(util_o),
            ..Default::default()
        });
        graph.add_file(util_d);

        let rule = graph.add_rule(object_rule());
        graph.record_rule_match(main_o, rule);
        graph.record_rule_match(util_o, rule);

        graph
    }

    #[test]
    fn rule_nodes_carry_provenance() {
        let (mut graph, [main_c, _, main_o, util_o, _]) = sample_graph();
        let rule = graph.add_rule(object_rule());
        graph.record_rule_match(main_o, rule);
        graph.record_rule_match(util_o, rule);

        // The rule id is semantic: matching scratch and the lazily-computed
        // printable definition don't change identity.
        let mut recomputed = object_rule();
        recomputed.in_use = true;
        let _ = recomputed.rule_defn();
        assert_eq!(rule, RuleId::from(&recomputed));

        assert_eq!(graph.display_name(NodeId::Rule(rule)), "%.o: %.c");
        assert_eq!(graph.rule_for(main_o), Some(rule));
        assert_eq!(graph.rule_for(main_c), None, "sources match no rule");
        assert_eq!(graph.files_derived_by(rule), vec![main_o, util_o]);

        // The rule's own pattern prerequisite is a (name-learned) phantom
        // node reachable from the rule.
        let pattern_c = FileId::from_bytes(b"%.c");
        assert_eq!(graph.display_name(NodeId::File(pattern_c)), "%.c");
        assert_eq!(
            graph.edges_from(NodeId::Rule(rule)),
            &[Edge {
                to: NodeId::File(pattern_c),
                kind: EdgeKind::RulePrerequisite(DepId::from(&object_rule().deps[0])),
            }]
        );

        // Provenance edges make used rules goal-reachable; an unused rule
        // stays outside the build slice.
        assert!(graph.reachable_from_goals().contains(&NodeId::Rule(rule)));
        let unused = graph.add_rule(Rule {
            targets: vec![b"%.tab.c".to_vec()],
            suffixes: vec![1],
            lens: vec![7],
            deps: vec![DepNode {
                name: "%.y".to_string(),
                ..Default::default()
            }],
            cmds: None,
            defn: None,
            num: 1,
            terminal: false,
            in_use: false,
        });
        assert!(!graph.reachable_from_goals().contains(&NodeId::Rule(unused)));

        // Rules take part in topo order: a file comes after the rule that
        // derives it (the rule is a build-order prerequisite of its product).
        let order = graph.topo_order().expect("acyclic");
        let pos = |n: NodeId| order.iter().position(|&x| x == n).unwrap();
        assert!(pos(NodeId::Rule(rule)) < pos(NodeId::File(main_o)));
    }

    #[test]
    fn mermaid_renders_every_edge_flavor() {
        let graph = showcase_graph();
        let mermaid = graph.to_mermaid();

        assert!(mermaid.starts_with("flowchart LR\n"));
        assert!(mermaid.contains("{\"<root>\"}"), "root rhombus");
        assert!(mermaid.contains("[\"prog\"]"), "recipe target rectangle");
        assert!(mermaid.contains("([\"main.c\"])"), "source stadium");
        assert!(mermaid.contains("[[\"%.o: %.c\"]]"), "rule subroutine box");
        assert!(mermaid.contains(" ==> "), "goal edge is thick");
        assert!(mermaid.contains(" -.->|order-only| "));
        assert!(mermaid.contains(" -.->|also| "));
        assert!(mermaid.contains(" -.->|rule| "));
        assert!(mermaid.contains("classDef phony"));
        assert!(mermaid.contains("classDef rule"));
        assert_eq!(mermaid, showcase_graph().to_mermaid(), "deterministic");

        let markdown = graph.to_mermaid_markdown();
        assert!(markdown.starts_with("```mermaid\nflowchart LR\n"));
        assert!(markdown.ends_with("```\n"));
    }

    /// The committed sample visualization. The graph rendering is
    /// deterministic, so the doc is a snapshot: this test regenerates it and
    /// fails if `docs/depgraph-sample.md` is stale. Refresh with
    /// `UPDATE_SNAPSHOTS=1 cargo test --lib depgraph`. The regenerated copy
    /// is also written to `target/depgraph-sample.md` as a build artifact
    /// (e.g. for CI to attach or post on a PR).
    #[test]
    fn mermaid_snapshot_doc_is_current() {
        let doc = format!(
            "# Dependency graph — sample visualization\n\
             \n\
             <!-- Generated by the `depgraph::tests::mermaid_snapshot_doc_is_current` test. -->\n\
             <!-- Regenerate with: UPDATE_SNAPSHOTS=1 cargo test --lib depgraph -->\n\
             \n\
             The showcase graph from `src/depgraph.rs`: `prog` linked from two\n\
             objects, each derived from its source by the `%.o: %.c` pattern rule\n\
             (dotted `rule` edges are provenance), with a phony order-only\n\
             `outdir` prerequisite and a `util.d` sibling produced by the same\n\
             recipe as `util.o` (`also` edge).\n\
             \n\
             {}",
            showcase_graph().to_mermaid_markdown()
        );

        let snapshot = concat!(env!("CARGO_MANIFEST_DIR"), "/docs/depgraph-sample.md");
        // Best-effort artifact for CI; the assertion below is the real check.
        let _ = std::fs::write(
            concat!(env!("CARGO_MANIFEST_DIR"), "/target/depgraph-sample.md"),
            &doc,
        );
        if std::env::var_os("UPDATE_SNAPSHOTS").is_some() {
            std::fs::write(snapshot, &doc).expect("write snapshot");
            return;
        }
        let committed = std::fs::read_to_string(snapshot)
            .expect("docs/depgraph-sample.md exists (UPDATE_SNAPSHOTS=1 to create)");
        assert_eq!(
            committed, doc,
            "docs/depgraph-sample.md is stale; regenerate with UPDATE_SNAPSHOTS=1"
        );
    }

    #[test]
    fn dot_output_is_deterministic_and_styled() {
        let (mut graph, [_, _, _, util_o, prog]) = sample_graph();
        // Give prog an order-only prerequisite on a phony `outdir` target.
        let mut outdir = FileNode::new(b"outdir".to_vec());
        outdir.phony = true;
        let outdir = graph.add_file(outdir);
        graph.add_dep(
            prog,
            DepNode {
                name: "outdir".to_string(),
                file: Some(outdir),
                ignore_mtime: true,
                ..Default::default()
            },
        );
        // And an also_make sibling relationship from util.o.
        let mut util_d = FileNode::new(b"util.d".to_vec());
        util_d.also_make.push(DepNode {
            name: "util.o".to_string(),
            file: Some(util_o),
            ..Default::default()
        });
        graph.add_file(util_d);

        let dot = graph.to_dot();
        assert!(dot.starts_with("digraph deps {"));
        assert!(dot.contains("\"prog\" -> \"main.o\";"));
        assert!(dot.contains("\"<root>\" -> \"prog\" [style=bold];"));
        assert!(dot.contains("\"prog\" -> \"outdir\" [style=dashed, label=\"order-only\"];"));
        assert!(dot.contains("\"util.d\" -> \"util.o\" [style=dotted, label=\"also\"];"));
        assert!(dot.contains("\"outdir\" [shape=ellipse, style=dashed];"));
        assert!(dot.contains("\"prog\" [shape=box];"));
        assert_eq!(dot, graph.to_dot(), "rendering is stable");
    }

    #[test]
    fn from_context_snapshots_the_arena() {
        let ctx = ExecContext::default();
        let mut node = FileNode::new(b"snap.o".to_vec());
        // Simulate `pattern_search` having committed a rule match: the
        // snapshot must surface it as a `DerivedBy` provenance edge.
        let rule_id = RuleId::from(&object_rule());
        node.matched_rule = Some(rule_id);
        let id = node.id();
        ctx.filenodes
            .0
            .lock()
            .unwrap()
            .insert(id, std::sync::Arc::new(std::sync::Mutex::new(node)));

        let goal = GoalDepNode {
            dep: DepNode {
                name: "snap.o".to_string(),
                file: Some(id),
                ..Default::default()
            },
            ..Default::default()
        };
        let graph = DepGraph::from_context(&ctx, &[goal]);

        assert!(graph.file(id).is_some());
        assert_eq!(graph.goals().count(), 1);
        assert!(graph.reachable_from_goals().contains(&NodeId::File(id)));
        assert_eq!(
            graph.rule_for(id),
            Some(rule_id),
            "provenance survives the snapshot"
        );
        assert_eq!(graph.files_derived_by(rule_id), vec![id]);
    }

    // ------------------------------------------------------------------ //
    // salsa layer                                                         //
    // ------------------------------------------------------------------ //

    #[test]
    fn queries_are_memoized_until_the_graph_changes() {
        let (graph, [main_c, _, _, _, _]) = sample_graph();
        let mut db = DepGraphDb::new(graph);

        let order = db.topo_order().expect("acyclic");
        let after_first = db.executions();
        assert!(after_first >= 1);

        // Re-querying answers from the memo — no new executions.
        assert_eq!(db.topo_order().unwrap(), order);
        assert_eq!(db.reachable_from_goals(), db.reachable_from_goals());
        let baseline = db.executions();
        db.topo_order().unwrap();
        assert_eq!(db.executions(), baseline, "memoized result, no re-run");

        // Growing the graph invalidates: the query re-executes and sees the
        // new node.
        let mut grown = db.graph().clone();
        let extra = grown.add_file(FileNode::new(b"extra.h".to_vec()));
        grown.add_dep(
            main_c,
            DepNode {
                name: "extra.h".to_string(),
                file: Some(extra),
                ..Default::default()
            },
        );
        db.set_graph(grown);
        let order2 = db.topo_order().expect("still acyclic");
        assert!(db.executions() > baseline, "revision bump re-ran the query");
        assert_eq!(order2.len(), order.len() + 1);
        assert!(order2.contains(&NodeId::File(extra)));
    }

    #[test]
    fn affected_by_is_memoized_per_file() {
        let (graph, [main_c, util_c, main_o, util_o, prog]) = sample_graph();
        let db = DepGraphDb::new(graph);

        let mut expected = vec![main_c, main_o, prog];
        expected.sort_unstable();
        assert_eq!(db.affected_by(&[main_c]), expected);

        let first = db.executions();
        db.affected_by(&[main_c]);
        assert_eq!(db.executions(), first, "same file: memoized");

        db.affected_by(&[util_c]);
        assert!(db.executions() > first, "new file argument: new memo");

        // Multi-file unions come out sorted and deduplicated.
        let mut all = vec![main_c, util_c, main_o, util_o, prog];
        all.sort_unstable();
        assert_eq!(db.affected_by(&[main_c, util_c]), all);
    }

    #[test]
    fn cycle_and_dot_flow_through_salsa() {
        let (graph, _) = cyclic_graph();
        let db = DepGraphDb::new(graph);
        assert!(db.find_cycle().is_some());
        assert!(db.topo_order().is_err());

        let (graph, _) = sample_graph();
        let mut db = DepGraphDb::new(graph);
        assert_eq!(db.find_cycle(), None);
        let dot = db.to_dot();
        assert!(dot.contains("\"<root>\" -> \"prog\" [style=bold];"));
        // Deref passthrough exposes the plain snapshot API.
        assert_eq!(db.display_name(NodeId::Root), "<root>");

        // A cyclic replacement flips the memoized answers.
        let (cyclic, _) = cyclic_graph();
        db.set_graph(cyclic);
        assert!(db.find_cycle().is_some());
    }
}
