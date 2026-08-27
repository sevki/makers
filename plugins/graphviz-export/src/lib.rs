//! Graphviz export of the resolved build graph.
//!
//! Rebuilt on `makers:plugin@1.0.0`. Three things changed from the version
//! that ran against the old push-based `visitor` interface, and each is the
//! interface change earning its keep:
//!
//! * It writes a **declared artifact** instead of stderr, so the DOT can be
//!   piped into `dot -Tsvg` without also catching make's diagnostics, and
//!   the file appears atomically.
//! * It reads edges from `node.dep-edges()`, so order-only prerequisites
//!   (`|`) are drawn as the ordering constraints they are rather than as
//!   ordinary dependencies — the old `dep` record had no flags at all.
//! * It **consumes another plugin's providers**: any node carrying
//!   `makers:cc/compile-command` is drawn as a compile step. This plugin has
//!   no idea how that provider is produced and does not depend on the crate
//!   that produces it; run `compile-commands` before it and the graph gains
//!   compile annotations, run it alone and the graph is still correct.

use makers_plugin::prelude::*;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::io::Write as _;

/// The provider a C/C++ compile-database plugin publishes. Consumed by id
/// alone — the payload is not decoded here, only its presence is used.
const COMPILE_COMMAND: &str = "makers:cc/compile-command";

struct GraphvizExport;

#[derive(Default)]
struct Accumulated {
    /// `(from, to, order_only)`, sorted so the DOT is byte-stable.
    edges: BTreeSet<(String, String, bool)>,
    phony: BTreeSet<String>,
    compiled: BTreeSet<String>,
    missing: BTreeSet<String>,
}

thread_local! {
    static STATE: RefCell<Accumulated> = RefCell::new(Accumulated::default());
}

fn dot_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

impl Analyzer for GraphvizExport {
    fn describe() -> PluginInfo {
        Manifest::new("graphviz-export", env!("CARGO_PKG_VERSION"))
            .description("Graphviz DOT export of the resolved build graph")
            .capability(Capability::WriteOutputs)
            // The DOT is a pure function of the graph and this instance's
            // settings, both of which `session.input-digest` covers.
            .deterministic()
            .output(
                "graph",
                "makers-graph.dot",
                "the build graph in Graphviz DOT format",
            )
            .build()
    }

    fn analyze(target: &Node) -> Result<Vec<Provider>, Error> {
        let name = target.name();
        if target.phony() {
            STATE.with(|s| s.borrow_mut().phony.insert(name.clone()));
        }
        // A prerequisite that is neither a target nor an existing file is
        // the single most common real makefile bug; the graph is the natural
        // place to see it.
        if !target.is_target() && target.mtime().is_none() && !target.phony() {
            STATE.with(|s| s.borrow_mut().missing.insert(name.clone()));
        }
        if target.provider(COMPILE_COMMAND).is_some() {
            STATE.with(|s| s.borrow_mut().compiled.insert(name.clone()));
        }
        for edge in target.dep_edges() {
            let order_only = edge.flags.contains(DepFlags::ORDER_ONLY);
            let child = edge.target.name();
            STATE.with(|s| {
                s.borrow_mut()
                    .edges
                    .insert((name.clone(), child, order_only))
            });
        }
        Ok(Vec::new())
    }

    fn finish() -> Result<(), Error> {
        let rankdir = makers_plugin::session::setting("rankdir").unwrap_or("LR".to_string());
        let mut out = makers_plugin::open_output("graph")?;
        let write = |out: &mut makers_plugin::Output, s: &str| -> Result<(), Error> {
            out.write_all(s.as_bytes())
                .map_err(|e| makers_plugin::fail(e.to_string()))
        };

        write(&mut out, "digraph make {\n")?;
        write(&mut out, &format!("    rankdir={rankdir};\n"))?;
        STATE.with(|s| -> Result<(), Error> {
            let state = s.borrow();
            for name in &state.phony {
                write(
                    &mut out,
                    &format!("    \"{}\" [shape=box, style=dashed];\n", dot_escape(name)),
                )?;
            }
            for name in &state.compiled {
                write(
                    &mut out,
                    &format!(
                        "    \"{}\" [shape=component, style=filled, fillcolor=\"#dbeafe\"];\n",
                        dot_escape(name)
                    ),
                )?;
            }
            for name in &state.missing {
                write(
                    &mut out,
                    &format!(
                        "    \"{}\" [shape=ellipse, color=\"#b91c1c\", fontcolor=\"#b91c1c\"];\n",
                        dot_escape(name)
                    ),
                )?;
            }
            for (from, to, order_only) in &state.edges {
                let style = if *order_only {
                    " [style=dotted, arrowhead=empty]"
                } else {
                    ""
                };
                write(
                    &mut out,
                    &format!(
                        "    \"{}\" -> \"{}\"{};\n",
                        dot_escape(from),
                        dot_escape(to),
                        style
                    ),
                )?;
            }
            Ok(())
        })?;
        write(&mut out, "}\n")?;
        out.finish()?;

        if let Some(path) = makers_plugin::output_path("graph") {
            makers_plugin::note(&format!("wrote {path}"));
        }
        Ok(())
    }
}

export_plugin!(GraphvizExport);
