#[allow(warnings)]
mod bindings;

use bindings::exports::makers::introspection::visitor::{Dep, File, Guest};

struct Component;

impl Guest for Component {
    fn visit_file(file: File) -> Result<(), String> {
        eprintln!(
            "visit {} (deps: {})",
            file.name,
            file.deps
                .iter()
                .map(|d| d.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
        Ok(())
    }

    fn visiting_child(parent: String, child: Dep) -> Result<(), String> {
        let parent = if parent.is_empty() { "<root>" } else { &parent };
        eprintln!("edge {parent} -> {}", child.name);
        Ok(())
    }

    fn visit_done() -> Result<(), String> {
        eprintln!("traversal done");
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
