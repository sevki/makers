#[allow(warnings)]
mod bindings;

use bindings::exports::makers::introspection::extension::Guest;
use bindings::makers::introspection::graph;

struct Component;

impl Guest for Component {
    fn run() -> Result<(), String> {
        let files = graph::list_files();
        eprintln!("introspection: {} target(s) in the graph", files.len());
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
