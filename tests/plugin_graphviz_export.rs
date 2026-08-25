//! End-to-end: `plugins/graphviz-export`, and with it the parts of the host
//! contract that only show up when two plugins run together.
//!
//! Requires both components to be built:
//! `(cd plugins/graphviz-export && cargo component build)` and
//! `(cd plugins/compile-commands && cargo component build)`.

#![cfg(feature = "wasmtime")]

mod plugin_common;
use plugin_common::{assert_clean, component, run_make, workdir, Run};

const SOURCES: &[&str] = &["main.c", "util.c", "debug.c"];

fn graph_only() -> Run {
    let plugin = component("graphviz-export", "graphviz_export");
    let dir = workdir("plugin_api.mk", SOURCES);
    let spec = format!("graph={}", plugin.display());
    run_make(&dir, &[("MAKERS_PLUGINS", &spec)])
}

/// The DOT describes the graph make actually resolved, not the makefile
/// text: `main.o -> main.c` exists only after `%.o: %.c` matched.
#[test]
fn the_dot_describes_the_resolved_graph() {
    let run = graph_only();
    assert_clean(&run);
    let dot = run.artifact("makers-graph.dot");
    assert!(dot.starts_with("digraph make {"), "{dot}");
    for edge in [
        "\"prog\" -> \"main.o\";",
        "\"main.o\" -> \"main.c\";",
        "\"debug.o\" -> \"debug.c\";",
    ] {
        assert!(dot.contains(edge), "missing {edge}:\n{dot}");
    }
    assert!(
        dot.contains("\"build\" [shape=box, style=dashed];"),
        "phony targets are styled from FileNode::phony, not from their name:\n{dot}"
    );
}

/// Order-only prerequisites are drawn as the ordering constraints they are.
/// The interface this replaces had no flags on a dependency edge at all, so
/// `| build` was indistinguishable from a real input — which is also the
/// bug that makes hand-rolled make caches invalidate on every `mkdir`.
#[test]
fn order_only_edges_are_distinguishable() {
    let run = graph_only();
    assert_clean(&run);
    let dot = run.artifact("makers-graph.dot");
    assert!(
        dot.contains("\"prog\" -> \"build\" [style=dotted, arrowhead=empty];"),
        "the order-only edge should be styled apart:\n{dot}"
    );
    assert!(
        dot.contains("\"prog\" -> \"main.o\";"),
        "and an ordinary edge should not be:\n{dot}"
    );
}

/// Two independently written plugins compose through a provider id.
///
/// `graphviz-export` does not depend on `compile-commands`, does not decode
/// its payload, and has no idea what C is: it annotates any node carrying
/// `makers:cc/compile-command`. This is the Bazel provider model, and it is
/// the thing the previous interface — whose hooks returned
/// `result<_, string>` — could not express at all.
#[test]
fn providers_carry_information_between_plugins() {
    let compdb = component("compile-commands", "compile_commands");
    let graph = component("graphviz-export", "graphviz_export");
    let dir = workdir("plugin_api.mk", SOURCES);
    let spec = format!("compdb={},graph={}", compdb.display(), graph.display());
    let run = run_make(&dir, &[("MAKERS_PLUGINS", &spec)]);
    assert_clean(&run);

    let dot = run.artifact("makers-graph.dot");
    for compiled in ["main.o", "util.o", "debug.o"] {
        assert!(
            dot.contains(&format!("\"{compiled}\" [shape=component")),
            "{compiled} carries a compile-command provider and should be annotated:\n{dot}"
        );
    }
    assert!(
        !dot.contains("\"prog\" [shape=component"),
        "the link step publishes no compile-command provider:\n{dot}"
    );
}

/// Providers flow forwards through the configured order, not backwards:
/// running the consumer first means there is nothing to consume. Ordering
/// being observable is the point — it is how an operator composes a pipeline.
#[test]
fn providers_only_reach_plugins_configured_after_the_producer() {
    let compdb = component("compile-commands", "compile_commands");
    let graph = component("graphviz-export", "graphviz_export");
    let dir = workdir("plugin_api.mk", SOURCES);
    let spec = format!("graph={},compdb={}", graph.display(), compdb.display());
    let run = run_make(&dir, &[("MAKERS_PLUGINS", &spec)]);
    assert_clean(&run);
    assert!(
        !run.artifact("makers-graph.dot").contains("shape=component"),
        "the consumer ran first, so it saw no providers"
    );
    assert!(
        run.artifact("compile_commands.json").contains("main.c"),
        "the producer still ran"
    );
}

/// Instance settings reach the plugin.
#[test]
fn settings_reach_the_plugin() {
    let plugin = component("graphviz-export", "graphviz_export");
    let dir = workdir("plugin_api.mk", SOURCES);
    let spec = format!("graph={}", plugin.display());
    let run = run_make(
        &dir,
        &[
            ("MAKERS_PLUGINS", &spec),
            ("MAKERS_PLUGIN_ARGS", "graph.rankdir=TB"),
        ],
    );
    assert_clean(&run);
    assert!(run.artifact("makers-graph.dot").contains("rankdir=TB;"));
}

/// The single-anonymous-plugin environment variable this interface grew out
/// of still loads a plugin, as the instance `default`.
#[test]
fn the_legacy_extension_variable_still_works() {
    let plugin = component("graphviz-export", "graphviz_export");
    let dir = workdir("plugin_api.mk", SOURCES);
    let run = run_make(&dir, &[("MAKERS_WASM_EXTENSION", plugin.to_str().unwrap())]);
    assert_clean(&run);
    assert!(run.artifact("makers-graph.dot").contains("digraph make {"));
}

/// Notes are quiet unless asked for, and `--plugin-verbose` reports what a
/// plugin was actually granted — the audit trail that makes a capability
/// system usable rather than merely present.
#[test]
fn verbose_mode_reports_the_granted_capabilities() {
    let plugin = component("graphviz-export", "graphviz_export");
    let dir = workdir("plugin_api.mk", SOURCES);
    let spec = format!("graph={}", plugin.display());
    let quiet = run_make(&dir, &[("MAKERS_PLUGINS", &spec)]);
    assert_clean(&quiet);
    assert!(
        !quiet.stderr.contains("capabilities:"),
        "quiet by default:\n{}",
        quiet.stderr
    );

    let loud = run_make(
        &dir,
        &[("MAKERS_PLUGINS", &spec), ("MAKERS_PLUGIN_VERBOSE", "1")],
    );
    assert_clean(&loud);
    assert!(
        loud.stderr.contains("capabilities: write-outputs"),
        "the grant is reported, and it is only what the plugin asked for:\n{}",
        loud.stderr
    );
}
