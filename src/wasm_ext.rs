//! `wasmtime`-backed component host for the push-based introspection
//! `visitor` interface (#633/#636/#644). Follows the same env-var-triggered
//! diagnostics-tap convention as `depgraph::dump_graph_if_requested`:
//! `MAKERS_WASM_EXTENSION`, when set to a `.wasm` component path, is run
//! against the fully-resolved build graph right after it would otherwise be
//! dumped. Never fails the build — errors are reported on stderr only, same
//! as the depgraph tap.
//!
//! Unlike the original MVP (which let the guest pull a full snapshot via
//! `list-files`), the host now drives the traversal itself — depth-first
//! from the goals, following `Prerequisite`/`AlsoMake` edges — and calls
//! into the guest's `visitor` export at each step. Only the host's real
//! traversal order can answer "what order" meaningfully ($<`/`$^`/`.WAIT`
//! order); a guest-driven pull put that decision in the wrong place.

use crate::dep::DepNode;
use crate::depgraph::{DepGraph, Edge, EdgeKind, NodeId};
use crate::execctx::ExecContext;
use crate::file::FileNode;

wasmtime::component::bindgen!({
    path: "wit/introspection/wit/world.wit",
    world: "introspection-extension",
});

use makers::introspection::graph::{Dep, File, Host as GraphHost};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

/// The graph snapshot handed to the guest, plus the WASI state the
/// `cargo-component` "reactor" template's guest imports need (stdio, clocks,
/// filesystem preopens — unused by introspection today, but part of the
/// component's baseline imports regardless). Built once before instantiation
/// so the `wasmtime::component::Linker` callbacks never need to reach back
/// into `ExecContext`'s locks while the guest is running.
struct GraphSnapshot {
    graph: DepGraph,
    wasi: WasiCtx,
    table: ResourceTable,
}

impl WasiView for GraphSnapshot {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

fn to_wit_dep(d: &DepNode) -> Dep {
    Dep {
        name: d.name.clone(),
        stem: d.stem.clone(),
        is_explicit: d.is_explicit,
    }
}

fn to_wit_file(f: &FileNode) -> File {
    File {
        name: String::from_utf8_lossy(&f.name).into_owned(),
        stem: f.stem.clone(),
        deps: f.deps.iter().map(to_wit_dep).collect(),
        also_make: f.also_make.iter().map(to_wit_dep).collect(),
        is_target: f.is_target,
        precious: f.precious,
        phony: f.phony,
    }
}

/// The `dep` payload an edge represents, if it is a "child" edge for this
/// interface's purposes. Only `Goal`/`Prerequisite`/`AlsoMake` edges carry a
/// `DepNode`-shaped payload — `Renamed`/`Parent`/`DerivedBy`/
/// `RulePrerequisite` are host-internal bookkeeping edges the visitor MVP
/// does not surface (rule provenance is a separate concern from the
/// read-only dependency view `FileNode::deps`/`also_make` already gave).
fn edge_dep(graph: &DepGraph, edge: &Edge) -> Option<Dep> {
    match edge.kind {
        // A goal's `DepNode::name` is always empty (see
        // `entry::goaldep_for_file`) — identity travels through `dep.file`
        // instead, so the name shown here has to come from the edge target
        // (`display_name`), not the payload.
        EdgeKind::Goal(id) => {
            let dep = to_wit_dep(&graph.goal(id)?.dep);
            Some(Dep {
                name: graph.display_name(edge.to),
                ..dep
            })
        }
        EdgeKind::Prerequisite(id) | EdgeKind::AlsoMake(id) => graph.dep(id).map(to_wit_dep),
        EdgeKind::Renamed | EdgeKind::Parent | EdgeKind::DerivedBy(_) => None,
        EdgeKind::RulePrerequisite(_) => None,
    }
}

impl GraphHost for GraphSnapshot {
    fn find_file(&mut self, name: String) -> Option<File> {
        self.graph
            .file(crate::file::FileId::from_bytes(name.as_bytes()))
            .map(to_wit_file)
    }

    fn get_variable(&mut self, name: String) -> Option<String> {
        // SAFETY: `name_c` is a live, NUL-terminated buffer for the whole
        // call; `lookup_variable`'s raw pointer/length contract is met.
        unsafe {
            let name_c = std::ffi::CString::new(name).ok()?;
            let len = name_c.as_bytes().len() as crate::ffi_types::size_t;
            let v = crate::variable::lookup_variable(current_ctx(), name_c.as_ptr(), len).ok()?;
            if v.is_null() || (*v).value.is_null() {
                return None;
            }
            Some(
                std::ffi::CStr::from_ptr((*v).value)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }
}

// The `ExecContext` the running extension call is snapshotting. Set for the
// duration of `run_extension_if_requested` only — `get_variable` needs a
// context handle but `wasmtime::component::bindgen!`'s generated `Host`
// trait carries no lifetime parameter to thread one through directly.
thread_local! {
    static CTX_PTR: std::cell::Cell<*const ExecContext> = const { std::cell::Cell::new(std::ptr::null()) };
}

fn current_ctx() -> &'static ExecContext {
    // SAFETY: only non-null for the duration of the `with_context` scope
    // below, which outlives every `get_variable` call the guest can make
    // (the guest runs synchronously, single-threaded, within that scope).
    unsafe { CTX_PTR.get().as_ref() }.expect("wasm_ext: no ExecContext installed")
}

fn with_context<R>(ctx: &ExecContext, f: impl FnOnce() -> R) -> R {
    CTX_PTR.set(ctx as *const ExecContext);
    let r = f();
    CTX_PTR.set(std::ptr::null());
    r
}

/// Debug hook for the real make binary: when `MAKERS_WASM_EXTENSION` names a
/// `.wasm` component, load it and drive a depth-first traversal of the
/// resolved build graph through its `visitor` export. Called from `main_0`
/// at the same point as `depgraph::dump_graph_post_if_requested` — after
/// makefiles are read, deps snapped, and the goal chain resolved.
pub fn run_extension_if_requested(ctx: &ExecContext, goals: &[crate::dep::GoalDepNode]) {
    let Some(path) = std::env::var_os("MAKERS_WASM_EXTENSION") else {
        return;
    };
    if let Err(err) = with_context(ctx, || run_extension(ctx, goals, path.as_ref())) {
        eprintln!("make: wasm extension failed: {err:#}");
    }
}

fn run_extension(
    ctx: &ExecContext,
    goals: &[crate::dep::GoalDepNode],
    wasm_path: &std::path::Path,
) -> anyhow::Result<()> {
    let graph = DepGraph::from_context(ctx, goals);

    let mut config = wasmtime::Config::new();
    config.wasm_component_model(true);
    let engine = wasmtime::Engine::new(&config)?;
    let component = wasmtime::component::Component::from_file(&engine, wasm_path)?;

    let mut linker = wasmtime::component::Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    makers::introspection::graph::add_to_linker::<
        GraphSnapshot,
        wasmtime::component::HasSelf<GraphSnapshot>,
    >(&mut linker, |s| s)?;

    let mut store = wasmtime::Store::new(
        &engine,
        GraphSnapshot {
            graph,
            wasi: WasiCtxBuilder::new().inherit_stderr().build(),
            table: ResourceTable::new(),
        },
    );
    let instance = IntrospectionExtension::instantiate(&mut store, &component, &linker)?;
    let visitor = instance.makers_introspection_visitor();

    // Depth-first from the synthetic root, following only Goal/Prerequisite/
    // AlsoMake edges (see `edge_dep`). Each node is visited once even if
    // reachable by multiple paths.
    let mut visited: rustc_hash::FxHashSet<NodeId> = Default::default();
    let mut stack: Vec<NodeId> = vec![NodeId::Root];
    visited.insert(NodeId::Root);
    while let Some(node) = stack.pop() {
        let parent_name = match node {
            NodeId::Root => String::new(),
            _ => store.data().graph.display_name(node),
        };
        // Collect (edge target, dep payload) before mutating `stack`/`store`,
        // since `store.data()` borrows the snapshot immutably.
        let children: Vec<(NodeId, Option<Dep>)> = store
            .data()
            .graph
            .edges_from(node)
            .iter()
            .map(|edge: &Edge| (edge.to, edge_dep(&store.data().graph, edge)))
            .collect();

        for (child, dep) in children {
            let Some(dep) = dep else { continue };
            visitor
                .call_visiting_child(&mut store, &parent_name, &dep)?
                .map_err(|e| anyhow::anyhow!(e))?;
            if !visited.insert(child) {
                continue;
            }
            if let NodeId::File(id) = child {
                if let Some(file) = store.data().graph.file(id).map(to_wit_file) {
                    visitor
                        .call_visit_file(&mut store, &file)?
                        .map_err(|e| anyhow::anyhow!(e))?;
                }
            }
            stack.push(child);
        }
    }

    visitor
        .call_visit_done(&mut store)?
        .map_err(|e| anyhow::anyhow!(e))
}
