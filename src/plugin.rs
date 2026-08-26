//! Host for `makers:plugin@1.0.0` — the build-plugin system.
//!
//! See `docs/plugin-api.md` for the design and the reasoning behind it, and
//! `wit/makers-plugin/` for the interface itself. This module is the part
//! that turns a configured list of `.wasm` components into a mediated,
//! ordered pass over the resolved build graph.
//!
//! # Shape of a run
//!
//! For each configured instance, in configuration order:
//!
//! 1. **Describe.** The component is instantiated in a store holding *no*
//!    authority at all and asked for its manifest. Nothing else is callable
//!    yet — the manifest is a claim made before any grant exists.
//! 2. **Grant.** The host intersects what the plugin asked for with what the
//!    operator allowed, reports the difference, and rejects the combinations
//!    that are incoherent (`deterministic` plus a clock; `fatal` without
//!    `fail-build`).
//! 3. **Instantiate.** A second store is built carrying exactly that
//!    authority — including which WASI preopens exist at all — and the
//!    component is instantiated against it.
//! 4. **Walk.** `start`, then `analyze` once per reachable node in
//!    dependency order, then `finish`.
//!
//! Providers published in step 4 outlive the instance and are handed to the
//! next one, which is what lets two independently written plugins compose.

pub mod host;
mod nodeset;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::dep::GoalDepNode;
use crate::depgraph::{DepGraph, NodeId};
use crate::execctx::ExecContext;
use host::{AnalyzerPlugin, Caps, FailurePolicy, OutputSlot, Phase, PluginInfo, PluginStore};

/// Default fuel budget for one plugin instance.
///
/// Metering is not optional theatre: a plugin is third-party code running
/// inside the build tool, and "someone's aspect hung CI" is a routine Bazel
/// failure mode with no equivalent lever — Starlark has no metering, so a
/// pathological `.bzl` simply stalls analysis. Fuel gives the host a
/// deterministic bound that does not depend on machine speed, so a plugin
/// that loops forever fails the same way on a laptop and on a CI runner.
/// Roughly a second of dense guest work on current hardware; raise it with
/// `--plugin-arg <instance>:fuel=<n>`.
const DEFAULT_FUEL: u64 = 20_000_000_000;

/// Default linear-memory ceiling for one plugin instance.
const DEFAULT_MEMORY_BYTES: usize = 512 * 1024 * 1024;

/// One configured plugin instance.
struct InstanceSpec {
    /// The name it was configured under, e.g. `compdb`.
    name: String,
    path: PathBuf,
    settings: BTreeMap<String, String>,
    allowed: Caps,
}

// ─── Configuration ───────────────────────────────────────────────────────

/// Read the configured instances from the environment.
///
/// Environment variables are the surface for this slice; the makefile-level
/// (`.PLUGIN:`) and command-line (`--plugin`) surfaces are specified in
/// `docs/plugin-api.md` and land separately, because adding options to
/// make's own getopt table is a change with its own blast radius and does
/// not belong in the same diff as the interface.
///
/// * `MAKERS_PLUGINS=name=path[,name=path...]`
/// * `MAKERS_PLUGIN_ARGS=name.key=value[;name.key=value...]`
/// * `MAKERS_PLUGIN_ALLOW=name:cap[,cap...][;name:cap...]` — `*` for all
///   instances, `all` for all capabilities.
/// * `MAKERS_WASM_EXTENSION=path` — the single-anonymous-plugin debug tap
///   this interface grew out of, kept working as the instance `default`.
fn configured_instances() -> Vec<InstanceSpec> {
    fn var(name: &str) -> Option<String> {
        std::env::var_os(name).map(|v| v.to_string_lossy().into_owned())
    }
    parse_instances(
        var("MAKERS_PLUGINS").as_deref(),
        var("MAKERS_WASM_EXTENSION").as_deref(),
        var("MAKERS_PLUGIN_ARGS").as_deref(),
        var("MAKERS_PLUGIN_ALLOW").as_deref(),
        var("MAKERS_PLUGIN_DENY").as_deref(),
    )
}

/// The pure half of [`configured_instances`], so the parsing rules are
/// testable without mutating the process environment (which no test can do
/// safely while other tests run).
fn parse_instances(
    plugins: Option<&str>,
    legacy_extension: Option<&str>,
    args: Option<&str>,
    allow: Option<&str>,
    deny: Option<&str>,
) -> Vec<InstanceSpec> {
    let mut specs: Vec<InstanceSpec> = Vec::new();

    for entry in plugins.unwrap_or_default().split(',') {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let (name, path) = match entry.split_once('=') {
            Some((n, p)) => (n.trim().to_string(), PathBuf::from(p.trim())),
            None => {
                // A bare path still names an instance — after its file stem,
                // so `--plugin-arg` has something to address.
                let path = PathBuf::from(entry);
                let name = path
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "plugin".to_string());
                (name, path)
            }
        };
        specs.push(InstanceSpec {
            name,
            path,
            settings: BTreeMap::new(),
            allowed: Caps::DEFAULT_GRANT,
        });
    }

    if let Some(path) = legacy_extension.filter(|p| !p.is_empty()) {
        specs.push(InstanceSpec {
            name: "default".to_string(),
            path: PathBuf::from(path),
            settings: BTreeMap::new(),
            allowed: Caps::DEFAULT_GRANT,
        });
    }

    for entry in args.unwrap_or_default().split(';') {
        let Some((lhs, value)) = entry.trim().split_once('=') else {
            continue;
        };
        // Split at the *first* dot: the instance name is one segment, and
        // keys are dotted (`out.database`).
        let Some((instance, key)) = lhs.trim().split_once('.') else {
            continue;
        };
        for spec in specs.iter_mut().filter(|s| s.name == instance) {
            spec.settings
                .insert(key.to_string(), value.trim().to_string());
        }
    }

    apply_capability_policy(&mut specs, "MAKERS_PLUGIN_ALLOW", allow, true);
    apply_capability_policy(&mut specs, "MAKERS_PLUGIN_DENY", deny, false);
    specs
}

/// Apply one `instance:cap,cap;instance:cap` policy list, adding to or
/// subtracting from each instance's allowed set.
fn apply_capability_policy(
    specs: &mut [InstanceSpec],
    var: &str,
    policy: Option<&str>,
    grant: bool,
) {
    for entry in policy.unwrap_or_default().split(';') {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let (target, caps) = match entry.split_once(':') {
            Some((t, c)) => (t.trim(), c),
            None => ("*", entry),
        };
        let mut mask = Caps::empty();
        for name in caps.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            match Caps::parse(name) {
                Some(c) => mask |= c,
                None => eprintln!("make: {var}: unknown capability `{name}`"),
            }
        }
        for spec in specs
            .iter_mut()
            .filter(|s| target == "*" || s.name == target)
        {
            if grant {
                spec.allowed |= mask;
            } else {
                spec.allowed &= !mask;
            }
        }
    }
}

// ─── Entry point ─────────────────────────────────────────────────────────

/// Run every configured plugin against the resolved build graph.
///
/// Returns `true` if a plugin whose manifest declares
/// [`FailurePolicy::Fatal`] — and which was granted `fail-build` — reported
/// an error, so the caller can set make's exit status. Anything else is
/// reported and survived: a plugin is an observer, and an observer that can
/// break the build by accident is worse than no observer.
pub fn run_plugins_if_requested(ctx: &ExecContext, goals: &[GoalDepNode]) -> bool {
    let specs = configured_instances();
    if specs.is_empty() {
        return false;
    }

    // `--shuffle` deliberately perturbs order to smoke out missing
    // prerequisites, and this port applies that reordering to
    // `FileNode::deps` and the goal list in place — by the time the pass
    // runs, `shuffle_goals_recursive` has already been through them and the
    // makefile's own order is gone. The `graph` interface promises makefile
    // order, so say plainly that this run cannot keep that promise rather
    // than let a plugin write a compile database or a build description in
    // scheduler order and look reproducible.
    if crate::shuffle::reorders_the_graph(ctx) {
        eprintln!(
            "make: plugin analysis: --shuffle reordered the graph; \
             `graph.goals`, `node.dep-edges` and analysis order are in shuffled \
             order, not makefile order"
        );
    }

    let graph = DepGraph::from_context(ctx, goals);
    let session = SessionFacts::collect(ctx, goals);
    let mut providers: FxHashMap<NodeId, Vec<host::Provider>> = FxHashMap::default();
    let mut fatal = false;

    with_context(ctx, || {
        for spec in &specs {
            match run_instance(spec, &graph, &session, &mut providers) {
                Ok(instance_fatal) => fatal |= instance_fatal,
                Err(err) => {
                    eprintln!("make: {}: plugin failed: {err:#}", spec.name);
                }
            }
        }
    });
    fatal
}

/// The invocation facts every instance sees, gathered once.
struct SessionFacts {
    working_dir: String,
    makefiles: Vec<String>,
    goal_names: Vec<String>,
    dry_run: bool,
    job_slots: u32,
    verbose: bool,
}

impl SessionFacts {
    fn collect(ctx: &ExecContext, goals: &[GoalDepNode]) -> Self {
        SessionFacts {
            working_dir: std::env::current_dir()
                .map(|d| d.display().to_string())
                .unwrap_or_default(),
            // `MAKEFILE_LIST` is make's own record of what it read, kept
            // correct through `include`, `-f` and remade makefiles — far
            // more reliable than re-deriving the list here.
            makefiles: lookup_global_raw("MAKEFILE_LIST")
                .unwrap_or_default()
                .split_whitespace()
                .map(str::to_string)
                .collect(),
            goal_names: goals
                .iter()
                .map(|g| {
                    g.dep
                        .file
                        .map(|f| f.to_string())
                        .unwrap_or_else(|| g.dep.name.clone())
                })
                .collect(),
            dry_run: crate::entry::opt_just_print(ctx),
            job_slots: crate::entry::opt_job_slots(ctx) as u32,
            verbose: std::env::var_os("MAKERS_PLUGIN_VERBOSE").is_some(),
        }
    }
}

// ─── One instance ────────────────────────────────────────────────────────

fn run_instance(
    spec: &InstanceSpec,
    graph: &DepGraph,
    session: &SessionFacts,
    providers: &mut FxHashMap<NodeId, Vec<host::Provider>>,
) -> anyhow::Result<bool> {
    let mut config = wasmtime::Config::new();
    config.wasm_component_model(true);
    config.consume_fuel(true);
    let engine = wasmtime::Engine::new(&config)?;
    let component = wasmtime::component::Component::from_file(&engine, &spec.path)?;

    // Step 1: the manifest, read with no authority granted.
    let info = describe(&engine, &component, spec)?;

    // Step 2: grant, and reject incoherent manifests.
    let requested = info
        .capabilities
        .iter()
        .fold(Caps::empty(), |acc, c| acc | Caps::from_wit(*c));
    if let Err(reason) = coherent(&info, requested) {
        anyhow::bail!("{} v{}: {reason}", info.name, info.version);
    }
    if !info.phases.contains(&Phase::Analyze) {
        eprintln!(
            "make: {}: plugin `{}` declares no analysis-phase hooks; skipped",
            spec.name, info.name
        );
        return Ok(false);
    }
    let granted = requested & spec.allowed;
    let withheld = requested & !granted;
    if !withheld.is_empty() {
        eprintln!(
            "make: {}: withheld capabilities: {} (grant with MAKERS_PLUGIN_ALLOW={}:{})",
            spec.name,
            withheld.names().join(", "),
            spec.name,
            withheld.names().join(",")
        );
    }

    // Step 3: the real store, carrying exactly the granted authority.
    let digest = input_digest(graph, &spec.settings);
    let outputs = resolve_outputs(&info, spec, &session.working_dir);
    let mut store = wasmtime::Store::new(
        &engine,
        PluginStore {
            graph: graph.clone(),
            sets: Default::default(),
            providers: std::mem::take(providers),
            instance: spec.name.clone(),
            settings: spec.settings.clone(),
            granted,
            outputs,
            digest,
            working_dir: session.working_dir.clone(),
            makefiles: session.makefiles.clone(),
            goal_names: session.goal_names.clone(),
            dry_run: session.dry_run,
            job_slots: session.job_slots,
            verbose: session.verbose,
            notes: 0,
            warnings: 0,
            errors: 0,
            table: Default::default(),
            wasi: wasi_ctx(granted, &session.working_dir),
            limits: wasmtime::StoreLimitsBuilder::new()
                .memory_size(memory_limit(&spec.settings))
                .build(),
        },
    );
    store.limiter(|s| &mut s.limits);
    store.set_fuel(fuel_budget(&spec.settings))?;

    let mut linker = wasmtime::component::Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    AnalyzerPlugin::add_to_linker::<PluginStore, wasmtime::component::HasSelf<PluginStore>>(
        &mut linker,
        |s| s,
    )?;
    let instance = AnalyzerPlugin::instantiate(&mut store, &component, &linker)?;

    // Step 4: the walk.
    let outcome = analyze_all(&mut store, &instance, graph);

    let data = store.into_data();
    *providers = data.providers;
    let (notes, warnings, errors) = (data.notes, data.warnings, data.errors);
    let unpublished: Vec<&str> = data
        .outputs
        .iter()
        .filter(|o| !o.published)
        .map(|o| o.logical.as_str())
        .collect();

    if let Err(err) = outcome {
        eprintln!("make: {}: {err:#}", spec.name);
        return Ok(matches!(info.failure_policy, FailurePolicy::Fatal)
            && granted.contains(Caps::FAIL_BUILD));
    }
    if session.verbose {
        eprintln!(
            "make: {}: {} note(s), {} warning(s), {} error(s); \
             capabilities: {}; unpublished outputs: {}",
            spec.name,
            notes,
            warnings,
            errors,
            if granted.is_empty() {
                "none".to_string()
            } else {
                granted.names().join(", ")
            },
            if unpublished.is_empty() {
                "none".to_string()
            } else {
                unpublished.join(", ")
            },
        );
    }
    Ok(errors > 0
        && matches!(info.failure_policy, FailurePolicy::Fatal)
        && granted.contains(Caps::FAIL_BUILD))
}

/// Instantiate in a zero-authority store purely to read the manifest.
///
/// The second instantiation is not waste: compilation is cached in the
/// `Component`, so this costs one extra instantiation, and it buys the
/// property that no capability-bearing store ever exists before the host has
/// seen and validated what the plugin asked for. A manifest baked into a
/// custom section at build time would be cheaper and would describe the
/// source rather than the shipped artefact.
fn describe(
    engine: &wasmtime::Engine,
    component: &wasmtime::component::Component,
    spec: &InstanceSpec,
) -> anyhow::Result<PluginInfo> {
    let mut store = wasmtime::Store::new(
        engine,
        PluginStore {
            graph: DepGraph::new(),
            sets: Default::default(),
            providers: FxHashMap::default(),
            instance: spec.name.clone(),
            settings: BTreeMap::new(),
            granted: Caps::empty(),
            outputs: Vec::new(),
            digest: String::new(),
            working_dir: String::new(),
            makefiles: Vec::new(),
            goal_names: Vec::new(),
            dry_run: false,
            job_slots: 0,
            verbose: false,
            notes: 0,
            warnings: 0,
            errors: 0,
            table: Default::default(),
            wasi: wasi_ctx(Caps::empty(), ""),
            limits: wasmtime::StoreLimitsBuilder::new()
                .memory_size(DEFAULT_MEMORY_BYTES)
                .build(),
        },
    );
    store.limiter(|s| &mut s.limits);
    store.set_fuel(DEFAULT_FUEL)?;

    let mut linker = wasmtime::component::Linker::new(engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    AnalyzerPlugin::add_to_linker::<PluginStore, wasmtime::component::HasSelf<PluginStore>>(
        &mut linker,
        |s| s,
    )?;
    let instance = AnalyzerPlugin::instantiate(&mut store, component, &linker)?;
    Ok(instance.makers_plugin_plugin().call_describe(&mut store)?)
}

/// Manifest claims that contradict each other, rejected before the plugin
/// gets a store with anything in it.
fn coherent(info: &PluginInfo, requested: Caps) -> Result<(), String> {
    if info.name.trim().is_empty() {
        return Err("manifest has an empty name".to_string());
    }
    if matches!(info.failure_policy, FailurePolicy::Fatal) && !requested.contains(Caps::FAIL_BUILD)
    {
        return Err(
            "declares failure-policy `fatal` without requesting the `fail-build` capability"
                .to_string(),
        );
    }
    // `expand-variables` belongs in this set for a less obvious reason than
    // the other two: expansion can run `$(shell ...)`, whose output is not in
    // the digest. A plugin that needs make's own expander therefore cannot
    // promise determinism — which is the whole capability argument in
    // miniature, and the reason the two are declared separately rather than
    // inferred from each other.
    const NONDETERMINISTIC: Caps = Caps::WALL_CLOCK
        .union(Caps::READ_ENVIRONMENT)
        .union(Caps::EXPAND_VARIABLES);
    if info.deterministic && requested.intersects(NONDETERMINISTIC) {
        return Err(format!(
            "declares `deterministic` while requesting {} — each is a way to depend on state \
             `session.input-digest` does not cover",
            (requested & NONDETERMINISTIC).names().join(" and ")
        ));
    }
    let mut seen = FxHashSet::default();
    for out in &info.outputs {
        if out.logical_name.trim().is_empty() {
            return Err("declares an output with an empty logical name".to_string());
        }
        if !seen.insert(out.logical_name.as_str()) {
            return Err(format!("declares output `{}` twice", out.logical_name));
        }
        if let Err(why) = confined(&out.default_path) {
            return Err(format!(
                "declares output `{}` at `{}`, which {why}",
                out.logical_name, out.default_path
            ));
        }
    }
    if !info.outputs.is_empty() && !requested.contains(Caps::WRITE_OUTPUTS) {
        return Err(
            "declares outputs without requesting the `write-outputs` capability".to_string(),
        );
    }
    Ok(())
}

/// A path a *manifest* may name for an output.
///
/// The WIT calls `default-path` "relative to make's working directory"; this
/// is what makes that a rule rather than a hope. The component is untrusted
/// code — that is the entire premise of running it in a sandbox with no
/// preopens — but the host writes its outputs, with the invoking user's
/// privileges. Without this check a manifest saying `/etc/cron.d/plugin` or
/// `../../.ssh/authorized_keys` gets exactly that file written, and since
/// `write-outputs` is part of `DEFAULT_GRANT`, *running* the plugin at all
/// would be enough. Mediating the write is worth nothing if the destination
/// comes from the same place as the bytes.
///
/// The check is deliberately syntactic and deliberately not applied to
/// `--plugin-arg <instance>:out.<logical>=<path>`: an operator typing a path
/// on the command line is exercising authority they already have, and
/// confining *them* to the working directory would only stop the legitimate
/// case (`out.database=/tmp/cc.json`). The asymmetry is the point — the
/// manifest proposes a default, the operator decides.
fn confined(path: &str) -> Result<(), &'static str> {
    use std::path::Component;
    if path.trim().is_empty() {
        return Err("is empty");
    }
    for component in Path::new(path).components() {
        match component {
            Component::Prefix(_) | Component::RootDir => {
                return Err("must be relative to make's working directory, not absolute");
            }
            // Rejected without normalising first: `a/../b` is harmless but
            // indistinguishable here from `a/../../b` without resolving
            // symlinks, and a manifest has no reason to need either.
            Component::ParentDir => {
                return Err("must not escape make's working directory with `..`");
            }
            Component::CurDir | Component::Normal(_) => {}
        }
    }
    Ok(())
}

/// Turn declared outputs into real paths, honouring
/// `--plugin-arg <instance>:out.<logical>=<path>`.
fn resolve_outputs(info: &PluginInfo, spec: &InstanceSpec, cwd: &str) -> Vec<OutputSlot> {
    info.outputs
        .iter()
        .map(|decl| {
            // `decl.default_path` has been through `confined` already;
            // `configured` deliberately has not — see that function.
            let configured = spec.settings.get(&format!("out.{}", decl.logical_name));
            let raw = configured.map(String::as_str).unwrap_or(&decl.default_path);
            let path = PathBuf::from(raw);
            OutputSlot {
                logical: decl.logical_name.clone(),
                path: if path.is_absolute() {
                    path
                } else {
                    PathBuf::from(cwd).join(path)
                },
                buf: Vec::new(),
                open: false,
                published: false,
            }
        })
        .collect()
}

/// `start`, one `analyze` per reachable node in dependency order, `finish`.
fn analyze_all(
    store: &mut wasmtime::Store<PluginStore>,
    instance: &AnalyzerPlugin,
    graph: &DepGraph,
) -> anyhow::Result<()> {
    let analyzer = instance.makers_plugin_analyzer();
    analyzer
        .call_start(&mut *store)?
        .map_err(|e| anyhow::anyhow!("start: {}", e.message))?;

    let (order, cycle) = analysis_order(graph);
    if let Some(cycle) = cycle {
        // make itself drops circular prerequisites and carries on; the
        // plugin pass matches that rather than inventing a stricter rule,
        // but says so, because a plugin's output over a graph with a dropped
        // edge is incomplete in a way worth knowing about.
        eprintln!("make: plugin analysis: dependency cycle dropped: {cycle}");
    }

    for node in order {
        let owned = store.data_mut().table.push(host::NodeHandle(node))?;
        let borrow = wasmtime::component::Resource::new_borrow(owned.rep());
        let published = analyzer.call_analyze(&mut *store, borrow)?;
        store.data_mut().table.delete(owned)?;

        let published = published
            .map_err(|e| anyhow::anyhow!("analyze({}): {}", graph.display_name(node), e.message))?;
        for provider in published {
            if let Err(reason) = valid_provider_id(&provider.id) {
                anyhow::bail!(
                    "analyze({}): provider `{}`: {reason}",
                    graph.display_name(node),
                    provider.id
                );
            }
            store
                .data_mut()
                .providers
                .entry(node)
                .or_default()
                .push(provider);
        }
    }

    analyzer
        .call_finish(&mut *store)?
        .map_err(|e| anyhow::anyhow!("finish: {}", e.message))?;
    Ok(())
}

/// Provider ids must be namespaced, so two unrelated plugins cannot collide
/// on `"info"` — the same reason WIT package names are namespaced and the
/// same failure Bazel avoids by making providers object identities rather
/// than strings.
fn valid_provider_id(id: &str) -> Result<(), &'static str> {
    match id.split_once(':') {
        Some((org, rest)) if !org.is_empty() && rest.contains('/') => Ok(()),
        _ => Err("ids must be namespaced as `<org>:<package>/<name>`"),
    }
}

// ─── Traversal order ─────────────────────────────────────────────────────

/// Reachable nodes in dependency order — every prerequisite before the
/// target that needs it, siblings in makefile edge order — plus a
/// description of the first cycle found, if any.
///
/// This is deliberately *not* `DepGraph::topo_order_from`: that walks every
/// edge kind, so it would drag rule-provenance nodes and their pattern
/// prerequisites (`%.c`) into a walk that is supposed to be over the files
/// this build actually touches. Here only goal, prerequisite and also-make
/// edges are followed.
///
/// A cycle is dropped rather than propagated, matching make's own
/// "Circular X <- Y dependency dropped" behaviour: a plugin pass that
/// refused to run on a graph make itself builds happily would be useless on
/// exactly the makefiles that most need inspecting.
pub(crate) fn analysis_order(graph: &DepGraph) -> (Vec<NodeId>, Option<String>) {
    enum Frame {
        Enter(NodeId),
        Exit(NodeId),
    }
    #[derive(PartialEq, Clone, Copy)]
    enum Color {
        Gray,
        Black,
    }

    let mut colors: FxHashMap<NodeId, Color> = FxHashMap::default();
    let mut path: Vec<NodeId> = Vec::new();
    let mut out: Vec<NodeId> = Vec::new();
    let mut cycle: Option<String> = None;
    let mut stack = vec![Frame::Enter(NodeId::Root)];

    while let Some(frame) = stack.pop() {
        match frame {
            Frame::Enter(n) => match colors.get(&n) {
                Some(Color::Black) => {}
                Some(Color::Gray) => {
                    if cycle.is_none() {
                        let from = path.iter().position(|&p| p == n).unwrap_or(0);
                        let names: Vec<String> = path[from..]
                            .iter()
                            .chain(std::iter::once(&n))
                            .map(|&p| graph.display_name(p))
                            .collect();
                        cycle = Some(names.join(" -> "));
                    }
                }
                None => {
                    colors.insert(n, Color::Gray);
                    path.push(n);
                    stack.push(Frame::Exit(n));
                    // Reversed so children are *entered* in edge order,
                    // which is `$^` order and therefore the order every
                    // artifact a plugin writes should follow.
                    let children: Vec<NodeId> = nodeset::child_edges(graph, n).collect();
                    for child in children.into_iter().rev() {
                        stack.push(Frame::Enter(child));
                    }
                }
            },
            Frame::Exit(n) => {
                colors.insert(n, Color::Black);
                path.pop();
                // The synthetic root is not a target; nothing can be
                // analysed about it.
                if n != NodeId::Root {
                    out.push(n);
                }
            }
        }
    }
    (out, cycle)
}

// ─── Input digest ────────────────────────────────────────────────────────

/// BLAKE3 over everything the analysis phase can observe about the graph,
/// plus this instance's settings.
///
/// This is what makes a plugin's output cacheable without requiring the
/// plugin to be pure in the Starlark sense. The digest covers node identity,
/// dependency structure and edge flags, recipe text, and per-target
/// variables — i.e. every input reachable through `makers:plugin/graph`.
/// It deliberately does not cover file *contents* or mtimes: a plugin that
/// needs those must request `read-file-content`, and a plugin that requests
/// it should not claim `deterministic`.
///
/// The goal list is hashed as well, in order, and it has to be: the analysis
/// walk starts from the goals, so `make lib` and `make tests` over one
/// unchanged makefile hand the plugin different `graph.goals`, different
/// `session.goal-names`, and a different set of reachable nodes. Hashing
/// only the file arena would give those two runs the same digest, and a
/// cache keyed on it would serve the artifact built for the wrong goals.
fn input_digest(graph: &DepGraph, settings: &BTreeMap<String, String>) -> String {
    use crate::content_hash::ContentHash as _;

    struct Blake3(blake3::Hasher);
    impl digest::Update for Blake3 {
        fn update(&mut self, data: &[u8]) {
            self.0.update(data);
        }
    }

    let mut hasher = Blake3(blake3::Hasher::new());
    for (_, goal) in graph.goals() {
        hasher.0.update(goal.dep.name.as_bytes());
        hasher.0.update(b"\0");
    }
    hasher.0.update(b"--goals--");
    let mut ids: Vec<_> = graph.files().map(|(id, _)| id).collect();
    ids.sort();
    for id in ids {
        hasher.0.update(&id.0);
        let Some(node) = graph.file(id) else { continue };
        hasher.0.update(&node.name);
        node.deps.hash(&mut hasher);
        node.also_make.hash(&mut hasher);
        node.recipe.hash(&mut hasher);
        node.variables.hash(&mut hasher);
        hasher.0.update(&[
            node.phony as u8,
            node.precious as u8,
            node.is_target as u8,
            node.intermediate as u8,
            node.secondary as u8,
        ]);
    }
    for (k, v) in settings {
        hasher.0.update(k.as_bytes());
        hasher.0.update(b"=");
        hasher.0.update(v.as_bytes());
        hasher.0.update(b"\0");
    }
    hasher.0.finalize().to_hex().to_string()
}

// ─── WASI ────────────────────────────────────────────────────────────────

/// Build the WASI context a granted capability set implies.
///
/// The default is deliberately barren: no preopens, no inherited stdio, no
/// environment, no arguments. A component built by `cargo component` imports
/// `wasi:filesystem` and `wasi:cli` whether it uses them or not, so those
/// imports must be *satisfiable* — but satisfying them with an empty context
/// means `std::fs::File::open` inside the guest fails with "no such file"
/// rather than reading the user's source tree. `read-file-content` swaps
/// that for a single read-only preopen of make's working directory: the
/// capability is a scoped handle, not an ambient right.
fn wasi_ctx(granted: Caps, working_dir: &str) -> wasmtime_wasi::WasiCtx {
    let mut builder = wasmtime_wasi::WasiCtxBuilder::new();
    // Guest stderr is inherited so that a panic message from inside a plugin
    // is not silently swallowed; everything a plugin means to *say* should
    // go through `diagnostics.emit`, which is attributed and counted.
    builder.inherit_stderr();
    if granted.contains(Caps::READ_FILE_CONTENT) && !working_dir.is_empty() {
        let _ = builder.preopened_dir(working_dir, ".", wasmtime_wasi::FsPerms::ReadOnly);
    }
    if !granted.contains(Caps::WALL_CLOCK) {
        builder.wall_clock(FrozenClock);
        builder.monotonic_clock(FrozenClock);
    }
    builder.build()
}

/// The clock an ungranted plugin gets: stopped at the epoch.
///
/// A capability that is only checked at the interface it names is not a
/// capability, because `wasi:clocks` is imported by every component
/// `cargo component` produces whether or not the plugin's own code mentions
/// time. Withholding `wall-clock` therefore has to reach the WASI clock
/// itself, not just a `makers:plugin` function — otherwise
/// `std::time::SystemTime::now()` sails straight past the check and a plugin
/// that claims `deterministic` can still emit a timestamp.
///
/// Stopped rather than absent because a missing clock makes Rust's `std`
/// panic on paths a plugin never asked for; a constant one keeps those paths
/// alive and merely makes time unobservable.
struct FrozenClock;

impl wasmtime_wasi::HostWallClock for FrozenClock {
    fn resolution(&self) -> std::time::Duration {
        std::time::Duration::from_secs(1)
    }
    fn now(&self) -> std::time::Duration {
        std::time::Duration::ZERO
    }
}

impl wasmtime_wasi::HostMonotonicClock for FrozenClock {
    fn resolution(&self) -> u64 {
        1
    }
    fn now(&self) -> u64 {
        0
    }
}

fn fuel_budget(settings: &BTreeMap<String, String>) -> u64 {
    settings
        .get("fuel")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_FUEL)
}

fn memory_limit(settings: &BTreeMap<String, String>) -> usize {
    settings
        .get("memory-bytes")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_MEMORY_BYTES)
}

// ─── Reaching `ExecContext` from a host callback ─────────────────────────

// The `ExecContext` the running plugin pass is reading. Installed for the
// duration of `run_plugins_if_requested` only: the generated host traits
// carry no lifetime parameter to thread a borrow through, so this follows
// the same borrow-channel convention `loadapi.rs` already uses for the
// `gmk_*` entry points.
thread_local! {
    static CTX_PTR: std::cell::Cell<*const ExecContext> =
        const { std::cell::Cell::new(std::ptr::null()) };
}

fn with_context<R>(ctx: &ExecContext, f: impl FnOnce() -> R) -> R {
    let previous = CTX_PTR.replace(ctx as *const ExecContext);
    let r = f();
    CTX_PTR.set(previous);
    r
}

/// The installed context pointer, or null outside a plugin pass.
///
/// Returned raw rather than as a reference: fabricating a `&'static
/// ExecContext` here would be a lifetime the borrow checker cannot police,
/// so the dereference stays inside the two `unsafe` blocks that already need
/// one for their C-ABI calls, under one `SAFETY` comment each.
fn ctx_ptr() -> *const ExecContext {
    CTX_PTR.get()
}

/// Raw expanded value of a global variable, or `None` if undefined.
fn lookup_global_raw(name: &str) -> Option<String> {
    let ctx = ctx_ptr();
    if ctx.is_null() {
        return None;
    }
    let name_c = std::ffi::CString::new(name).ok()?;
    let len = name_c.as_bytes().len() as crate::ffi_types::size_t;
    // SAFETY: `CTX_PTR` is non-null only inside `with_context`, which outlives
    // every host callback a guest can make (guests run synchronously within
    // that scope), so `ctx` points at a live `ExecContext`. `name_c` is a live
    // NUL-terminated buffer for the whole call, meeting `lookup_variable`'s
    // pointer/length contract, and the returned `variable` is owned by the
    // global set and outlives the read.
    unsafe {
        let v = crate::variable::lookup_variable(&*ctx, name_c.as_ptr(), len).ok()?;
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

/// A global variable as the WIT interface describes it.
pub(crate) fn lookup_global(name: &str) -> Option<host::Variable> {
    let value = lookup_global_raw(name)?;
    Some(host::Variable {
        name: name.to_string(),
        value,
        // The legacy `variable` record's flavor/origin fields are private to
        // `variable.rs`; the graph-side `TargetVariable` carries them and is
        // what `node.variable` reports. Global lookups report the value and
        // leave provenance unknown rather than guessing it.
        flavor: host::VarFlavor::Recursive,
        origin: host::VarOrigin::File,
        defined_at: None,
        exported: false,
        private: false,
    })
}

/// Expand makefile text through make's own expander, in the global scope.
pub(crate) fn expand_global(text: &str) -> Result<String, String> {
    let ctx = ctx_ptr();
    if ctx.is_null() {
        return Err("no make context available".to_string());
    }
    let input = std::ffi::CString::new(text).map_err(|_| "text contains a NUL byte".to_string())?;
    // SAFETY: `ctx` is live for the same reason as in `lookup_global_raw`;
    // `input` outlives the call; the null `file` pointer is the documented
    // "global scope" argument, exactly as `gmk_expand` passes it; and the
    // returned buffer is a `malloc`ed NUL-terminated string this call owns.
    unsafe {
        let expanded = crate::expand::allocated_expand_string_for_file(
            &*ctx,
            input.as_ptr(),
            std::ptr::null_mut::<crate::file::File>(),
        )
        .map_err(|_| "expansion failed".to_string())?;
        if expanded.is_null() {
            return Ok(String::new());
        }
        let out = std::ffi::CStr::from_ptr(expanded)
            .to_string_lossy()
            .into_owned();
        libc::free(expanded as *mut libc::c_void);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::FileNode;

    fn spec_named(specs: &[InstanceSpec], name: &str) -> InstanceSpec {
        let s = specs
            .iter()
            .find(|s| s.name == name)
            .unwrap_or_else(|| panic!("no instance named {name}"));
        InstanceSpec {
            name: s.name.clone(),
            path: s.path.clone(),
            settings: s.settings.clone(),
            allowed: s.allowed,
        }
    }

    /// A three-node chain plus a second prerequisite, so that both
    /// dependency order and sibling order are observable.
    fn chain_graph() -> DepGraph {
        let mut graph = DepGraph::new();
        let prog = graph.add_file(FileNode::new(b"prog".to_vec()));
        let main_o = graph.add_file(FileNode::new(b"main.o".to_vec()));
        let util_o = graph.add_file(FileNode::new(b"util.o".to_vec()));
        let main_c = graph.add_file(FileNode::new(b"main.c".to_vec()));
        for (from, name, to) in [
            (prog, "main.o", main_o),
            (prog, "util.o", util_o),
            (main_o, "main.c", main_c),
        ] {
            graph.add_dep(
                from,
                crate::dep::DepNode {
                    name: name.to_string(),
                    file: Some(to),
                    ..Default::default()
                },
            );
        }
        graph.add_goal(crate::dep::GoalDepNode {
            dep: crate::dep::DepNode {
                file: Some(prog),
                ..Default::default()
            },
            ..Default::default()
        });
        graph
    }

    fn names(graph: &DepGraph, order: &[NodeId]) -> Vec<String> {
        order.iter().map(|&n| graph.display_name(n)).collect()
    }

    /// Every prerequisite is analysed before the target that needs it, and
    /// siblings keep the makefile's own edge order — the two properties the
    /// aspect model depends on (a hook can only read `dep.providers()` if
    /// the dep already ran, and artifacts are only reproducible if sibling
    /// order is fixed).
    #[test]
    fn analysis_order_is_dependency_order_with_stable_siblings() {
        let graph = chain_graph();
        let (order, cycle) = analysis_order(&graph);
        assert!(cycle.is_none());
        assert_eq!(
            names(&graph, &order),
            ["main.c", "main.o", "util.o", "prog"]
        );
    }

    /// The synthetic root is not a target and is never handed to a plugin.
    #[test]
    fn analysis_order_excludes_the_root() {
        let graph = chain_graph();
        let (order, _) = analysis_order(&graph);
        assert!(!order.contains(&NodeId::Root));
    }

    /// A cycle is described and dropped rather than propagated, matching
    /// make's own "Circular X <- Y dependency dropped" — a plugin pass that
    /// refused to run on such a graph would be useless on exactly the
    /// makefiles most worth inspecting.
    #[test]
    fn analysis_order_drops_cycles_and_reports_them() {
        let mut graph = DepGraph::new();
        let a = graph.add_file(FileNode::new(b"a".to_vec()));
        let b = graph.add_file(FileNode::new(b"b".to_vec()));
        graph.add_dep(
            a,
            crate::dep::DepNode {
                name: "b".to_string(),
                file: Some(b),
                ..Default::default()
            },
        );
        graph.add_dep(
            b,
            crate::dep::DepNode {
                name: "a".to_string(),
                file: Some(a),
                ..Default::default()
            },
        );
        graph.add_goal(crate::dep::GoalDepNode {
            dep: crate::dep::DepNode {
                file: Some(a),
                ..Default::default()
            },
            ..Default::default()
        });
        let (order, cycle) = analysis_order(&graph);
        assert!(cycle.is_some_and(|c| c.contains("a") && c.contains("b")));
        // Both nodes still reach the plugin exactly once.
        assert_eq!(order.len(), 2);
    }

    /// The digest is stable across two builds of the same graph and moves
    /// when the graph does — the whole basis of the `deterministic` promise.
    #[test]
    fn input_digest_is_stable_and_structure_sensitive() {
        let settings = BTreeMap::new();
        let base = input_digest(&chain_graph(), &settings);
        assert_eq!(base, input_digest(&chain_graph(), &settings));

        let mut changed = chain_graph();
        let extra = changed.add_file(FileNode::new(b"extra.h".to_vec()));
        let main_o = crate::file::FileId::from_bytes(b"main.o");
        changed.add_dep(
            main_o,
            crate::dep::DepNode {
                name: "extra.h".to_string(),
                file: Some(extra),
                ..Default::default()
            },
        );
        assert_ne!(base, input_digest(&changed, &settings));
    }

    /// The digest has to move when the requested goals move. `make prog`
    /// and `make main.o` read one unchanged makefile into one identical file
    /// arena, but the analysis walk starts from the goals, so the plugin
    /// sees a different `graph.goals`, a different `session.goal-names` and
    /// a different set of analysed nodes. A digest blind to that would let a
    /// cache hand back the artifact built for the other invocation.
    #[test]
    fn input_digest_covers_the_requested_goals() {
        let settings = BTreeMap::new();
        let prog = input_digest(&chain_graph(), &settings);

        let mut other = chain_graph();
        other.add_goal(crate::dep::GoalDepNode {
            dep: crate::dep::DepNode {
                name: "main.o".to_string(),
                file: Some(crate::file::FileId::from_bytes(b"main.o")),
                ..Default::default()
            },
            ..Default::default()
        });
        assert_ne!(
            prog,
            input_digest(&other, &settings),
            "a second goal must change the digest"
        );
    }

    /// Settings are part of the digest: the same graph analysed with a
    /// different configuration is a different question and must not hit a
    /// cache entry for the first one.
    #[test]
    fn input_digest_covers_instance_settings() {
        let graph = chain_graph();
        let mut settings = BTreeMap::new();
        let base = input_digest(&graph, &settings);
        settings.insert("out.database".to_string(), "elsewhere.json".to_string());
        assert_ne!(base, input_digest(&graph, &settings));
    }

    /// Unnamespaced provider ids are rejected, so two unrelated plugins
    /// cannot collide on `"info"`.
    #[test]
    fn provider_ids_must_be_namespaced() {
        assert!(valid_provider_id("makers:cc/compile-command").is_ok());
        assert!(valid_provider_id("info").is_err());
        assert!(valid_provider_id("makers:cc").is_err());
        assert!(valid_provider_id(":cc/x").is_err());
    }

    fn manifest(caps: &[host::Capability]) -> PluginInfo {
        PluginInfo {
            name: "p".to_string(),
            version: "0".to_string(),
            description: String::new(),
            phases: vec![Phase::Analyze],
            capabilities: caps.to_vec(),
            outputs: Vec::new(),
            deterministic: false,
            failure_policy: FailurePolicy::Advisory,
        }
    }

    fn caps_of(info: &PluginInfo) -> Caps {
        info.capabilities
            .iter()
            .fold(Caps::empty(), |acc, c| acc | Caps::from_wit(*c))
    }

    /// A plugin that declares `fatal` without asking for `fail-build` is
    /// refused rather than silently downgraded: a policy gate that stops
    /// gating without saying so is worse than one that refuses to start.
    #[test]
    fn fatal_without_fail_build_is_refused() {
        let mut info = manifest(&[]);
        info.failure_policy = FailurePolicy::Fatal;
        assert!(coherent(&info, caps_of(&info)).is_err());

        let mut ok = manifest(&[host::Capability::FailBuild]);
        ok.failure_policy = FailurePolicy::Fatal;
        assert!(coherent(&ok, caps_of(&ok)).is_ok());
    }

    /// `deterministic` is checked, not merely trusted: a clock or the
    /// environment are both ways to depend on state the input digest does
    /// not cover.
    #[test]
    fn deterministic_rejects_the_nondeterministic_capabilities() {
        for cap in [
            host::Capability::WallClock,
            host::Capability::ReadEnvironment,
        ] {
            let mut info = manifest(&[cap]);
            info.deterministic = true;
            assert!(
                coherent(&info, caps_of(&info)).is_err(),
                "{cap:?} should be incompatible with `deterministic`"
            );
        }
    }

    /// Declaring an output without asking for the capability to write one
    /// would fail later, at `artifacts.open`, after the plugin had already
    /// done its work. Catching it at load turns a confusing runtime error
    /// into a load-time one.
    #[test]
    fn declared_outputs_require_write_outputs() {
        let decl = host::OutputDecl {
            logical_name: "database".to_string(),
            default_path: "db.json".to_string(),
            description: String::new(),
        };
        let mut info = manifest(&[]);
        info.outputs = vec![decl.clone()];
        assert!(coherent(&info, caps_of(&info)).is_err());

        let mut ok = manifest(&[host::Capability::WriteOutputs]);
        ok.outputs = vec![decl.clone(), decl];
        assert!(
            coherent(&ok, caps_of(&ok)).is_err(),
            "a duplicated logical name is ambiguous and must be refused"
        );
    }

    /// A manifest may not name a path outside make's working directory. The
    /// host does the writing, so an unchecked `default-path` turns "load
    /// this plugin" into "overwrite this file" — and `write-outputs` is in
    /// `DEFAULT_GRANT`, so loading it is all an attacker would need.
    #[test]
    fn manifest_output_paths_are_confined_to_the_working_directory() {
        for bad in [
            "/etc/cron.d/pwn",
            "../../.ssh/authorized_keys",
            "build/../../escape",
            "   ",
        ] {
            assert!(
                confined(bad).is_err(),
                "`{bad}` must be refused as an output path"
            );
        }
        for good in ["compile_commands.json", "build/graph.dot", "./out/db.json"] {
            assert!(confined(good).is_ok(), "`{good}` must be accepted");
        }
    }

    /// The refusal happens at manifest validation, so the plugin never runs
    /// — the path is not silently rewritten into the working directory,
    /// which would leave the operator with an artifact where they did not
    /// ask for one and no indication why.
    #[test]
    fn an_escaping_output_path_refuses_the_manifest() {
        let mut info = manifest(&[host::Capability::WriteOutputs]);
        info.outputs = vec![host::OutputDecl {
            logical_name: "db".to_string(),
            default_path: "../../etc/shadow".to_string(),
            description: String::new(),
        }];
        let err = coherent(&info, caps_of(&info)).expect_err("must be refused");
        assert!(err.contains("escape"), "unexpected message: {err}");
    }

    /// Instances are named, settings are addressed by instance, and dotted
    /// keys survive the split.
    #[test]
    fn instances_take_names_paths_and_settings() {
        let specs = parse_instances(
            Some("compdb=./cc.wasm, graph=./g.wasm"),
            None,
            Some("compdb.out.database=build/db.json;graph.rankdir=TB;absent.k=v"),
            None,
            None,
        );
        assert_eq!(specs.len(), 2);
        let compdb = spec_named(&specs, "compdb");
        assert_eq!(compdb.path, PathBuf::from("./cc.wasm"));
        assert_eq!(
            compdb.settings.get("out.database").map(String::as_str),
            Some("build/db.json")
        );
        assert_eq!(
            spec_named(&specs, "graph").settings.get("rankdir"),
            Some(&"TB".to_string())
        );
    }

    /// A bare path is still an addressable instance, named after its stem.
    #[test]
    fn a_bare_path_is_named_after_its_stem() {
        let specs = parse_instances(Some("./tools/lint.wasm"), None, None, None, None);
        assert_eq!(specs[0].name, "lint");
    }

    /// The single-anonymous-plugin tap this interface grew out of keeps
    /// working, as the instance `default`.
    #[test]
    fn the_legacy_extension_variable_becomes_an_instance() {
        let specs = parse_instances(None, Some("./old.wasm"), None, None, None);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].name, "default");
        assert_eq!(specs[0].allowed, Caps::DEFAULT_GRANT);
    }

    /// Grants and denials are per instance, `*` reaches all of them, and
    /// denial is applied after grant so it always wins.
    #[test]
    fn capability_policy_is_per_instance_and_deny_wins() {
        let specs = parse_instances(
            Some("a=./a.wasm,b=./b.wasm"),
            None,
            None,
            Some("a:expand-variables,read-file-content;*:wall-clock"),
            Some("a:read-recipes"),
        );
        let a = spec_named(&specs, "a");
        let b = spec_named(&specs, "b");
        assert!(a.allowed.contains(Caps::EXPAND_VARIABLES));
        assert!(a.allowed.contains(Caps::READ_FILE_CONTENT));
        assert!(!a.allowed.contains(Caps::READ_RECIPES), "deny must win");
        assert!(a.allowed.contains(Caps::WALL_CLOCK), "`*` reaches a");
        assert!(b.allowed.contains(Caps::WALL_CLOCK), "`*` reaches b");
        assert!(!b.allowed.contains(Caps::EXPAND_VARIABLES));
        assert!(b.allowed.contains(Caps::READ_RECIPES), "b was not denied");
    }

    /// Nothing configured means nothing runs — the plugin pass must be
    /// invisible to every build that does not ask for it.
    #[test]
    fn no_configuration_means_no_instances() {
        assert!(parse_instances(None, None, None, None, None).is_empty());
        assert!(parse_instances(Some(""), Some(""), Some(""), None, None).is_empty());
    }

    /// The default grant is the read-only view of what make already read,
    /// plus a declared output. Everything that reaches further is opt-in.
    #[test]
    fn the_default_grant_withholds_the_reaching_capabilities() {
        let d = Caps::DEFAULT_GRANT;
        assert!(d.contains(Caps::READ_RECIPES | Caps::READ_VARIABLES | Caps::WRITE_OUTPUTS));
        for withheld in [
            Caps::EXPAND_VARIABLES,
            Caps::READ_ENVIRONMENT,
            Caps::READ_FILE_CONTENT,
            Caps::WALL_CLOCK,
            Caps::FAIL_BUILD,
        ] {
            assert!(!d.intersects(withheld), "{withheld:?} must be opt-in");
        }
    }
}
