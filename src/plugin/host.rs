//! The `wasmtime` side of `makers:plugin@1.0.0`: store state, resource
//! tables, and the host implementations of every imported interface.
//!
//! The store data type ([`PluginStore`]) is built once per plugin *instance*
//! and holds three separable things: a read-only snapshot of the build graph
//! (so no host callback ever has to reach back into `ExecContext`'s locks
//! while the guest is running), the instance's granted authority, and the
//! side effects it has accumulated — providers published, diagnostics
//! emitted, artifacts opened.

use std::collections::BTreeMap;
use std::path::PathBuf;

use rustc_hash::FxHashMap;
use wasmtime::component::Resource;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView};

use crate::depgraph::{DepGraph, Edge, EdgeKind, NodeId};
use crate::plugin::nodeset::{child_edges, SetId, Sets};

wasmtime::component::bindgen!({
    path: "wit/makers-plugin",
    world: "analyzer-plugin",
    imports: { default: trappable },
    with: {
        "makers:plugin/graph.node": NodeHandle,
        "makers:plugin/graph.node-set": SetHandle,
        "makers:plugin/artifacts.output": OutputHandle,
    },
});

pub use self::makers::plugin::graph::{
    DepEdge, DepFlags, Host as GraphHost, HostNode, HostNodeSet, NodeKind, Recipe, RecipeLine,
    VarFlavor, VarOrigin, Variable,
};
pub use self::makers::plugin::types::{
    Diagnostic, Error as WitError, Location, Provider, Severity,
};

pub use self::exports::makers::plugin::plugin::{
    Capability, FailurePolicy, OutputDecl, Phase, PluginInfo,
};

/// A graph node, as the guest holds it. Only the id: everything else is a
/// call back into the snapshot, which is the whole point of the handle
/// design (see `wit/makers-plugin/graph.wit`).
pub struct NodeHandle(pub NodeId);

/// A nested set, as the guest holds it — an index into [`PluginStore::sets`].
pub struct SetHandle(pub SetId);

/// An open declared output, as the guest holds it.
pub struct OutputHandle(pub usize);

// ─── Capabilities ────────────────────────────────────────────────────────

bitflags::bitflags! {
    /// The authority granted to one plugin instance.
    ///
    /// A capability withheld is not an error the guest sees as a denial: a
    /// gated accessor returns `none` and a gated action returns an `error`
    /// naming the missing capability. A plugin that degrades gracefully when
    /// it cannot read recipes is a better plugin than one that probes.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Caps: u16 {
        const READ_RECIPES     = 1 << 0;
        const READ_VARIABLES   = 1 << 1;
        const EXPAND_VARIABLES = 1 << 2;
        const READ_ENVIRONMENT = 1 << 3;
        const READ_FILE_CONTENT = 1 << 4;
        const WALL_CLOCK       = 1 << 5;
        const WRITE_OUTPUTS    = 1 << 6;
        const FAIL_BUILD       = 1 << 7;
    }
}

impl Caps {
    /// What a plugin gets without the operator saying anything.
    ///
    /// The line is drawn at "does this reach outside the makefiles make has
    /// already read on the user's behalf". Structure, variables and recipes
    /// are all things `make -p` prints on request, so a plugin reading them
    /// learns nothing the invoking user could not already see. The
    /// environment, the filesystem, the clock, make's own expander (which
    /// runs `$(shell ...)`), and the ability to fail the build all cross that
    /// line and are opt-in per instance.
    pub const DEFAULT_GRANT: Caps = Caps::READ_RECIPES
        .union(Caps::READ_VARIABLES)
        .union(Caps::WRITE_OUTPUTS);

    pub fn from_wit(c: Capability) -> Caps {
        match c {
            Capability::ReadRecipes => Caps::READ_RECIPES,
            Capability::ReadVariables => Caps::READ_VARIABLES,
            Capability::ExpandVariables => Caps::EXPAND_VARIABLES,
            Capability::ReadEnvironment => Caps::READ_ENVIRONMENT,
            Capability::ReadFileContent => Caps::READ_FILE_CONTENT,
            Capability::WallClock => Caps::WALL_CLOCK,
            Capability::WriteOutputs => Caps::WRITE_OUTPUTS,
            Capability::FailBuild => Caps::FAIL_BUILD,
        }
    }

    /// Parse one capability name as it is written in `MAKERS_PLUGIN_ALLOW`.
    pub fn parse(name: &str) -> Option<Caps> {
        Some(match name {
            "read-recipes" => Caps::READ_RECIPES,
            "read-variables" => Caps::READ_VARIABLES,
            "expand-variables" => Caps::EXPAND_VARIABLES,
            "read-environment" => Caps::READ_ENVIRONMENT,
            "read-file-content" => Caps::READ_FILE_CONTENT,
            "wall-clock" => Caps::WALL_CLOCK,
            "write-outputs" => Caps::WRITE_OUTPUTS,
            "fail-build" => Caps::FAIL_BUILD,
            "all" => Caps::all(),
            _ => return None,
        })
    }

    /// Names, for diagnostics. Stable order so messages are reproducible.
    pub fn names(self) -> Vec<&'static str> {
        [
            (Caps::READ_RECIPES, "read-recipes"),
            (Caps::READ_VARIABLES, "read-variables"),
            (Caps::EXPAND_VARIABLES, "expand-variables"),
            (Caps::READ_ENVIRONMENT, "read-environment"),
            (Caps::READ_FILE_CONTENT, "read-file-content"),
            (Caps::WALL_CLOCK, "wall-clock"),
            (Caps::WRITE_OUTPUTS, "write-outputs"),
            (Caps::FAIL_BUILD, "fail-build"),
        ]
        .into_iter()
        .filter(|(c, _)| self.contains(*c))
        .map(|(_, n)| n)
        .collect()
    }
}

// ─── Outputs ─────────────────────────────────────────────────────────────

/// One declared output, resolved to a real path.
pub struct OutputSlot {
    pub logical: String,
    pub path: PathBuf,
    /// Buffered bytes. Nothing reaches the filesystem before `finish`, so a
    /// plugin that traps mid-write leaves the previous artifact intact.
    pub buf: Vec<u8>,
    pub open: bool,
    pub published: bool,
}

// ─── Store state ─────────────────────────────────────────────────────────

/// Everything one plugin instance can see and everything it has done.
pub struct PluginStore {
    pub graph: DepGraph,
    pub sets: Sets,
    /// Providers published on nodes, shared across every instance in this
    /// run — this is what lets a later plugin consume an earlier one's
    /// output without the two knowing about each other.
    pub providers: FxHashMap<NodeId, Vec<Provider>>,

    pub instance: String,
    pub settings: BTreeMap<String, String>,
    pub granted: Caps,
    pub outputs: Vec<OutputSlot>,
    pub digest: String,
    pub working_dir: String,
    pub makefiles: Vec<String>,
    pub goal_names: Vec<String>,
    pub dry_run: bool,
    pub job_slots: u32,
    pub verbose: bool,

    /// Counts by severity, for the run summary and the failure policy.
    pub notes: usize,
    pub warnings: usize,
    pub errors: usize,

    pub table: ResourceTable,
    pub wasi: WasiCtx,
    pub limits: wasmtime::StoreLimits,
}

impl WasiView for PluginStore {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

impl PluginStore {
    fn node_of(&mut self, r: &Resource<NodeHandle>) -> wasmtime::Result<NodeId> {
        Ok(self.table.get(r)?.0)
    }

    fn set_of(&mut self, r: &Resource<SetHandle>) -> wasmtime::Result<SetId> {
        Ok(self.table.get(r)?.0)
    }

    fn push_node(&mut self, id: NodeId) -> wasmtime::Result<Resource<NodeHandle>> {
        Ok(self.table.push(NodeHandle(id))?)
    }

    fn push_set(&mut self, id: SetId) -> wasmtime::Result<Resource<SetHandle>> {
        Ok(self.table.push(SetHandle(id))?)
    }

    /// The `FileNode` behind a handle, if the handle names a file at all.
    fn file_node(&self, id: NodeId) -> Option<&crate::file::FileNode> {
        match id {
            NodeId::File(f) => self.graph.file(f),
            _ => None,
        }
    }

    /// Report a diagnostic through make's stderr, prefixed with the instance
    /// name so several plugins in one run stay distinguishable.
    pub fn report(&mut self, d: &Diagnostic) {
        match d.severity {
            Severity::Note => self.notes += 1,
            Severity::Warning => self.warnings += 1,
            Severity::Error => self.errors += 1,
        }
        if matches!(d.severity, Severity::Note) && !self.verbose {
            return;
        }
        let level = match d.severity {
            Severity::Note => "note",
            Severity::Warning => "warning",
            Severity::Error => "error",
        };
        match &d.location {
            Some(loc) if loc.line > 0 => {
                eprintln!(
                    "make: {}:{}: {}: {}: {}",
                    loc.file, loc.line, self.instance, level, d.message
                );
            }
            Some(loc) => eprintln!(
                "make: {}: {}: {}: {}",
                loc.file, self.instance, level, d.message
            ),
            None => eprintln!("make: {}: {}: {}", self.instance, level, d.message),
        }
    }
}

fn denied(cap: &str, what: &str) -> WitError {
    WitError {
        message: format!("{what} requires the `{cap}` capability, which was not granted"),
        location: None,
    }
}

fn location_of(file: Option<&Vec<u8>>, line: u64) -> Option<Location> {
    file.map(|f| Location {
        file: String::from_utf8_lossy(f).into_owned(),
        line,
        column: 0,
    })
}

// ─── graph ───────────────────────────────────────────────────────────────

impl GraphHost for PluginStore {
    fn root(&mut self) -> wasmtime::Result<Resource<NodeHandle>> {
        self.push_node(NodeId::Root)
    }

    fn goals(&mut self) -> wasmtime::Result<Vec<Resource<NodeHandle>>> {
        let goals: Vec<NodeId> = self
            .graph
            .edges_from(NodeId::Root)
            .iter()
            .filter(|e| matches!(e.kind, EdgeKind::Goal(_)))
            .map(|e| e.to)
            .collect();
        goals.into_iter().map(|g| self.push_node(g)).collect()
    }

    fn find(&mut self, name: String) -> wasmtime::Result<Option<Resource<NodeHandle>>> {
        let id = crate::file::FileId::from_bytes(name.as_bytes());
        match self.graph.file(id) {
            Some(_) => self.push_node(NodeId::File(id)).map(Some),
            None => Ok(None),
        }
    }

    fn reachable(&mut self) -> wasmtime::Result<Resource<SetHandle>> {
        let set = self.sets.transitive(NodeId::Root);
        self.push_set(set)
    }

    fn topological_order(
        &mut self,
    ) -> wasmtime::Result<Result<Vec<Resource<NodeHandle>>, WitError>> {
        let (order, cycle) = crate::plugin::analysis_order(&self.graph);
        if let Some(cycle) = cycle {
            return Ok(Err(WitError {
                message: format!("dependency cycle: {cycle}"),
                location: None,
            }));
        }
        let handles: wasmtime::Result<Vec<_>> =
            order.into_iter().map(|n| self.push_node(n)).collect();
        Ok(Ok(handles?))
    }
}

impl HostNode for PluginStore {
    fn id(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<String> {
        Ok(match self.node_of(&this)? {
            NodeId::Root => "<root>".to_string(),
            NodeId::File(f) => f.to_string(),
            NodeId::Rule(r) => r.to_string(),
        })
    }

    fn kind(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<NodeKind> {
        Ok(match self.node_of(&this)? {
            NodeId::Root => NodeKind::Root,
            NodeId::File(_) => NodeKind::File,
            NodeId::Rule(_) => NodeKind::Rule,
        })
    }

    fn name(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<String> {
        let id = self.node_of(&this)?;
        Ok(self.graph.display_name(id))
    }

    fn stem(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<Option<String>> {
        let id = self.node_of(&this)?;
        Ok(self.file_node(id).and_then(|f| f.stem.clone()))
    }

    fn deps(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<Resource<SetHandle>> {
        let id = self.node_of(&this)?;
        let members: Vec<NodeId> = child_edges(&self.graph, id).collect();
        let set = self.sets.leaf(members);
        self.push_set(set)
    }

    fn dep_edges(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<Vec<DepEdge>> {
        let id = self.node_of(&this)?;
        let raw: Vec<Edge> = self.graph.edges_from(id).to_vec();
        let mut out = Vec::with_capacity(raw.len());
        for edge in raw {
            let Some((name, stem, flags)) = self.edge_payload(&edge) else {
                continue;
            };
            out.push(DepEdge {
                target: self.push_node(edge.to)?,
                name,
                stem,
                flags,
            });
        }
        Ok(out)
    }

    fn also_make(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<Resource<SetHandle>> {
        let id = self.node_of(&this)?;
        let members: Vec<NodeId> = self
            .graph
            .edges_from(id)
            .iter()
            .filter(|e| matches!(e.kind, EdgeKind::AlsoMake(_)))
            .map(|e| e.to)
            .collect();
        let set = self.sets.leaf(members);
        self.push_set(set)
    }

    fn dependents(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<Resource<SetHandle>> {
        let id = self.node_of(&this)?;
        let members: Vec<NodeId> = match id {
            NodeId::File(f) => self
                .graph
                .dependents(f)
                .into_iter()
                // Rule nodes are provenance, not users: a rule "depending
                // on" `%.c` would otherwise show up as a dependent of every
                // C file in the build.
                .filter(|n| !matches!(n, NodeId::Rule(_)))
                .collect(),
            _ => Vec::new(),
        };
        let set = self.sets.leaf(members);
        self.push_set(set)
    }

    fn transitive_deps(
        &mut self,
        this: Resource<NodeHandle>,
    ) -> wasmtime::Result<Resource<SetHandle>> {
        let id = self.node_of(&this)?;
        let set = self.sets.transitive(id);
        self.push_set(set)
    }

    fn recipe(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<Option<Recipe>> {
        if !self.granted.contains(Caps::READ_RECIPES) {
            return Ok(None);
        }
        let id = self.node_of(&this)?;
        Ok(self
            .file_node(id)
            .and_then(|f| f.recipe.as_ref())
            .map(|r| Recipe {
                defined_at: location_of(r.defined_in.as_ref(), r.defined_lineno),
                lines: r
                    .lines
                    .iter()
                    .map(|line| RecipeLine {
                        text: String::from_utf8_lossy(&line.text).into_owned(),
                        silent: line.flags.contains(crate::recipe::RecipeLineFlags::SILENT),
                        ignore_errors: line.flags.contains(crate::recipe::RecipeLineFlags::NOERROR),
                        recursive: line.flags.contains(crate::recipe::RecipeLineFlags::RECURSE),
                    })
                    .collect(),
            }))
    }

    fn matched_rule(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<Option<String>> {
        let id = self.node_of(&this)?;
        let rule = match id {
            NodeId::File(f) => self.graph.rule_for(f),
            _ => None,
        };
        Ok(rule.map(|r| self.graph.display_name(NodeId::Rule(r))))
    }

    fn defined_at(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<Option<Location>> {
        let id = self.node_of(&this)?;
        Ok(self
            .file_node(id)
            .and_then(|f| f.recipe.as_ref())
            .and_then(|r| location_of(r.defined_in.as_ref(), r.defined_lineno)))
    }

    fn mtime(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<Option<u64>> {
        let id = self.node_of(&this)?;
        Ok(self.file_node(id).map(|f| f.last_mtime).filter(|&m| m != 0))
    }

    fn phony(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<bool> {
        let id = self.node_of(&this)?;
        Ok(self.file_node(id).is_some_and(|f| f.phony))
    }

    fn precious(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<bool> {
        let id = self.node_of(&this)?;
        Ok(self.file_node(id).is_some_and(|f| f.precious))
    }

    fn intermediate(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<bool> {
        let id = self.node_of(&this)?;
        Ok(self.file_node(id).is_some_and(|f| f.intermediate))
    }

    fn secondary(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<bool> {
        let id = self.node_of(&this)?;
        Ok(self.file_node(id).is_some_and(|f| f.secondary))
    }

    fn is_target(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<bool> {
        let id = self.node_of(&this)?;
        Ok(self.file_node(id).is_some_and(|f| f.is_target))
    }

    fn builtin(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<bool> {
        let id = self.node_of(&this)?;
        Ok(self.file_node(id).is_some_and(|f| f.builtin))
    }

    fn double_colon(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<bool> {
        let id = self.node_of(&this)?;
        Ok(self.file_node(id).is_some_and(|f| f.is_double_colon))
    }

    fn variable(
        &mut self,
        this: Resource<NodeHandle>,
        name: String,
    ) -> wasmtime::Result<Option<Variable>> {
        if !self.granted.contains(Caps::READ_VARIABLES) {
            return Ok(None);
        }
        let id = self.node_of(&this)?;
        // Target-scoped precedence, the same order make itself searches:
        // the target's own definitions, then pattern-specific ones, then
        // the global set. Reading the global value and calling it the
        // target's is the classic per-target-flags bug.
        if let Some(node) = self.file_node(id) {
            let hit = node
                .variables
                .iter()
                .chain(node.pat_variables.iter())
                .find(|v| v.name == name.as_bytes());
            if let Some(v) = hit {
                return Ok(Some(target_variable(v)));
            }
        }
        Ok(crate::plugin::lookup_global(&name))
    }

    fn own_variables(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<Vec<Variable>> {
        if !self.granted.contains(Caps::READ_VARIABLES) {
            return Ok(Vec::new());
        }
        let id = self.node_of(&this)?;
        Ok(self
            .file_node(id)
            .map(|f| f.variables.iter().map(target_variable).collect())
            .unwrap_or_default())
    }

    fn providers(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<Vec<Provider>> {
        let id = self.node_of(&this)?;
        Ok(self.providers.get(&id).cloned().unwrap_or_default())
    }

    fn provider(
        &mut self,
        this: Resource<NodeHandle>,
        id: String,
    ) -> wasmtime::Result<Option<Provider>> {
        let node = self.node_of(&this)?;
        Ok(self
            .providers
            .get(&node)
            .and_then(|ps| ps.iter().find(|p| p.id == id).cloned()))
    }

    fn drop(&mut self, this: Resource<NodeHandle>) -> wasmtime::Result<()> {
        self.table.delete(this)?;
        Ok(())
    }
}

impl PluginStore {
    /// The as-written name, stem and flags an edge carries, or `None` for
    /// the host-internal bookkeeping edges (rename, parent, rule provenance)
    /// that are not part of the dependency view.
    fn edge_payload(&self, edge: &Edge) -> Option<(String, Option<String>, DepFlags)> {
        let dep = match edge.kind {
            // A goal's own `DepNode::name` is always empty (see
            // `entry::goaldep_for_file`); identity travels through
            // `dep.file`, so the name has to come from the edge target.
            EdgeKind::Goal(id) => {
                let goal = self.graph.goal(id)?;
                return Some((
                    self.graph.display_name(edge.to),
                    goal.dep.stem.clone(),
                    dep_flags(&goal.dep),
                ));
            }
            EdgeKind::Prerequisite(id) | EdgeKind::AlsoMake(id) => self.graph.dep(id)?,
            EdgeKind::Renamed
            | EdgeKind::Parent
            | EdgeKind::DerivedBy(_)
            | EdgeKind::RulePrerequisite(_) => return None,
        };
        Some((dep.name.clone(), dep.stem.clone(), dep_flags(dep)))
    }
}

fn dep_flags(dep: &crate::dep::DepNode) -> DepFlags {
    let mut f = DepFlags::empty();
    if dep.ignore_mtime {
        f |= DepFlags::ORDER_ONLY;
    }
    if dep.wait_here {
        f |= DepFlags::WAIT;
    }
    if dep.is_explicit {
        f |= DepFlags::EXPLICIT;
    }
    if dep.static_pattern {
        f |= DepFlags::STATIC_PATTERN;
    }
    if dep.needs_second_expansion {
        f |= DepFlags::SECOND_EXPANSION;
    }
    f
}

fn target_variable(v: &crate::target_var::TargetVariable) -> Variable {
    Variable {
        name: String::from_utf8_lossy(&v.name).into_owned(),
        value: String::from_utf8_lossy(&v.value).into_owned(),
        flavor: match v.flavor {
            crate::file::VarFlavor::Simple => VarFlavor::Simple,
            crate::file::VarFlavor::Recursive => VarFlavor::Recursive,
            _ => VarFlavor::Undefined,
        },
        origin: match v.origin {
            crate::file::VarOrigin::Default => VarOrigin::Default,
            crate::file::VarOrigin::Environment => VarOrigin::Environment,
            crate::file::VarOrigin::EnvOverride => VarOrigin::EnvOverride,
            crate::file::VarOrigin::File => VarOrigin::File,
            crate::file::VarOrigin::Command => VarOrigin::CommandLine,
            crate::file::VarOrigin::Override => VarOrigin::Override,
            crate::file::VarOrigin::Automatic => VarOrigin::Automatic,
            _ => VarOrigin::Default,
        },
        defined_at: location_of(v.defined_in.as_ref(), v.defined_lineno),
        exported: v.exportable,
        private: v.private_var,
    }
}

// ─── node-set ────────────────────────────────────────────────────────────

impl HostNodeSet for PluginStore {
    fn new(&mut self, nodes: Vec<Resource<NodeHandle>>) -> wasmtime::Result<Resource<SetHandle>> {
        let mut members = Vec::with_capacity(nodes.len());
        for n in nodes {
            members.push(self.node_of(&n)?);
            self.table.delete(n)?;
        }
        let set = self.sets.leaf(members);
        self.push_set(set)
    }

    fn union(
        &mut self,
        this: Resource<SetHandle>,
        other: Resource<SetHandle>,
    ) -> wasmtime::Result<Resource<SetHandle>> {
        let a = self.set_of(&this)?;
        let b = self.set_of(&other)?;
        let u = self.sets.union(a, b);
        self.push_set(u)
    }

    fn is_empty(&mut self, this: Resource<SetHandle>) -> wasmtime::Result<bool> {
        let id = self.set_of(&this)?;
        Ok(self.sets.is_empty(id, &self.graph))
    }

    fn contains(
        &mut self,
        this: Resource<SetHandle>,
        n: Resource<NodeHandle>,
    ) -> wasmtime::Result<bool> {
        let id = self.set_of(&this)?;
        let needle = self.node_of(&n)?;
        Ok(self.sets.contains(id, needle, &self.graph))
    }

    fn to_list(
        &mut self,
        this: Resource<SetHandle>,
    ) -> wasmtime::Result<Vec<Resource<NodeHandle>>> {
        let id = self.set_of(&this)?;
        let members = self.sets.flatten(id, &self.graph);
        members.iter().map(|&n| self.push_node(n)).collect()
    }

    fn drop(&mut self, this: Resource<SetHandle>) -> wasmtime::Result<()> {
        self.table.delete(this)?;
        Ok(())
    }
}

// ─── vars ────────────────────────────────────────────────────────────────

impl self::makers::plugin::vars::Host for PluginStore {
    fn get(&mut self, name: String) -> wasmtime::Result<Option<Variable>> {
        if !self.granted.contains(Caps::READ_VARIABLES) {
            return Ok(None);
        }
        Ok(crate::plugin::lookup_global(&name))
    }

    fn expand(&mut self, text: String) -> wasmtime::Result<Result<String, WitError>> {
        if !self.granted.contains(Caps::EXPAND_VARIABLES) {
            return Ok(Err(denied("expand-variables", "vars.expand")));
        }
        Ok(
            crate::plugin::expand_global(&text).map_err(|message| WitError {
                message,
                location: None,
            }),
        )
    }
}

// ─── session ─────────────────────────────────────────────────────────────

impl self::makers::plugin::session::Host for PluginStore {
    fn instance_name(&mut self) -> wasmtime::Result<String> {
        Ok(self.instance.clone())
    }

    fn setting(&mut self, key: String) -> wasmtime::Result<Option<String>> {
        Ok(self.settings.get(&key).cloned())
    }

    fn settings(&mut self) -> wasmtime::Result<Vec<(String, String)>> {
        Ok(self
            .settings
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect())
    }

    fn working_directory(&mut self) -> wasmtime::Result<String> {
        Ok(self.working_dir.clone())
    }

    fn makefiles(&mut self) -> wasmtime::Result<Vec<String>> {
        Ok(self.makefiles.clone())
    }

    fn goal_names(&mut self) -> wasmtime::Result<Vec<String>> {
        Ok(self.goal_names.clone())
    }

    fn dry_run(&mut self) -> wasmtime::Result<bool> {
        Ok(self.dry_run)
    }

    fn job_slots(&mut self) -> wasmtime::Result<u32> {
        Ok(self.job_slots)
    }

    fn input_digest(&mut self) -> wasmtime::Result<String> {
        Ok(self.digest.clone())
    }

    fn env(&mut self, name: String) -> wasmtime::Result<Option<String>> {
        if !self.granted.contains(Caps::READ_ENVIRONMENT) {
            return Ok(None);
        }
        Ok(std::env::var(name).ok())
    }
}

// ─── artifacts ───────────────────────────────────────────────────────────

impl self::makers::plugin::artifacts::Host for PluginStore {
    fn open(
        &mut self,
        logical_name: String,
    ) -> wasmtime::Result<Result<Resource<OutputHandle>, WitError>> {
        if !self.granted.contains(Caps::WRITE_OUTPUTS) {
            return Ok(Err(denied("write-outputs", "artifacts.open")));
        }
        let Some(idx) = self.outputs.iter().position(|o| o.logical == logical_name) else {
            return Ok(Err(WitError {
                message: format!(
                    "`{logical_name}` is not a declared output of this plugin (declared: {})",
                    self.outputs
                        .iter()
                        .map(|o| o.logical.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                location: None,
            }));
        };
        if self.outputs[idx].open || self.outputs[idx].published {
            return Ok(Err(WitError {
                message: format!("output `{logical_name}` has already been opened"),
                location: None,
            }));
        }
        self.outputs[idx].open = true;
        Ok(Ok(self.table.push(OutputHandle(idx))?))
    }

    fn path_of(&mut self, logical_name: String) -> wasmtime::Result<Option<String>> {
        Ok(self
            .outputs
            .iter()
            .find(|o| o.logical == logical_name)
            .map(|o| o.path.display().to_string()))
    }
}

impl self::makers::plugin::artifacts::HostOutput for PluginStore {
    fn write(
        &mut self,
        this: Resource<OutputHandle>,
        bytes: Vec<u8>,
    ) -> wasmtime::Result<Result<(), WitError>> {
        let idx = self.table.get(&this)?.0;
        if !self.outputs[idx].open {
            return Ok(Err(WitError {
                message: "write after finish".to_string(),
                location: None,
            }));
        }
        self.outputs[idx].buf.extend_from_slice(&bytes);
        Ok(Ok(()))
    }

    fn finish(&mut self, this: Resource<OutputHandle>) -> wasmtime::Result<Result<(), WitError>> {
        let idx = self.table.get(&this)?.0;
        if !self.outputs[idx].open {
            return Ok(Err(WitError {
                message: "output already finished".to_string(),
                location: None,
            }));
        }
        self.outputs[idx].open = false;
        let slot = &self.outputs[idx];
        Ok(match publish(&slot.path, &slot.buf) {
            Ok(()) => {
                self.outputs[idx].published = true;
                Ok(())
            }
            Err(e) => Err(WitError {
                message: e,
                location: None,
            }),
        })
    }

    fn drop(&mut self, this: Resource<OutputHandle>) -> wasmtime::Result<()> {
        let idx = self.table.get(&this)?.0;
        // An abandoned stream publishes nothing: a half-written
        // compile_commands.json breaks every editor that reads it, so the
        // previous one is left in place instead.
        self.outputs[idx].open = false;
        self.table.delete(this)?;
        Ok(())
    }
}

/// Write `bytes` to `path` atomically: a sibling temporary file, then a
/// rename. Readers of the artifact never observe a partial write, and a
/// crashed plugin cannot corrupt the previous one.
fn publish(path: &std::path::Path, bytes: &[u8]) -> Result<(), String> {
    if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    let tmp = path.with_extension(format!(
        "{}.tmp{}",
        path.extension()
            .map(|e| e.to_string_lossy())
            .unwrap_or_default(),
        std::process::id()
    ));
    std::fs::write(&tmp, bytes).map_err(|e| format!("{}: {e}", tmp.display()))?;
    std::fs::rename(&tmp, path).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        format!("{}: {e}", path.display())
    })
}

// ─── diagnostics ─────────────────────────────────────────────────────────

impl self::makers::plugin::diagnostics::Host for PluginStore {
    fn emit(&mut self, d: Diagnostic) -> wasmtime::Result<()> {
        self.report(&d);
        Ok(())
    }
}

impl self::makers::plugin::types::Host for PluginStore {}
