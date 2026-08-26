use makers_plugin::{export_plugin, Dep, File, Plugin};
use std::cell::RefCell;
use std::collections::BTreeSet;

/// Graphviz DOT export of the resolved build graph, driven by the host's
/// traversal callbacks (#644): every edge the host follows becomes a
/// `parent -> child` line, every visited phony target gets a shape
/// override. Emitted to stderr on `visit_done` — MVP has no dedicated
/// output channel back to the host (see docs/wasm-extension-system.md).
struct GraphvizExport;

thread_local! {
    static EDGES: RefCell<BTreeSet<(String, String)>> = RefCell::new(BTreeSet::new());
    static PHONY: RefCell<BTreeSet<String>> = RefCell::new(BTreeSet::new());
}

fn dot_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

impl Plugin for GraphvizExport {
    fn visit_file(file: File) -> Result<(), String> {
        if file.phony {
            PHONY.with(|p| p.borrow_mut().insert(file.name.clone()));
        }
        Ok(())
    }

    fn visiting_child(parent: String, child: Dep) -> Result<(), String> {
        let parent = if parent.is_empty() {
            "<root>".to_string()
        } else {
            parent
        };
        EDGES.with(|e| e.borrow_mut().insert((parent, child.name)));
        Ok(())
    }

    fn visit_done() -> Result<(), String> {
        let mut out = String::from("digraph make {\n    rankdir=LR;\n");
        PHONY.with(|p| {
            for name in p.borrow().iter() {
                out.push_str(&format!(
                    "    \"{0}\" [shape=box, style=dashed];\n",
                    dot_escape(name)
                ));
            }
        });
        EDGES.with(|e| {
            for (parent, child) in e.borrow().iter() {
                out.push_str(&format!(
                    "    \"{}\" -> \"{}\";\n",
                    dot_escape(parent),
                    dot_escape(child)
                ));
            }
        });
        out.push_str("}\n");
        eprint!("{out}");
        Ok(())
    }
}

export_plugin!(GraphvizExport);
