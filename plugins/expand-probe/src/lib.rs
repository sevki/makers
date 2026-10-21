//! Test plugin for `node.expand` and `node.recipe-environment`.
//!
//! For every target with a recipe it writes, one per line, the target, the
//! raw recipe line and what the host expands it to in the target's scope; and
//! once, the environment make would give that recipe. It exists so that the
//! host side of those two calls is exercised from outside, as a guest sees it.

use makers_plugin::prelude::*;
use std::cell::RefCell;
use std::io::Write as _;

struct ExpandProbe;

thread_local! {
    static REPORT: RefCell<String> = const { RefCell::new(String::new()) };
}

impl Analyzer for ExpandProbe {
    fn describe() -> PluginInfo {
        Manifest::new("expand-probe", env!("CARGO_PKG_VERSION"))
            .description("reports what node.expand and node.recipe-environment return")
            .capability(Capability::ReadRecipes)
            .capability(Capability::ExpandVariables)
            .capability(Capability::WriteOutputs)
            .output("report", "expand.txt", "one line per expanded recipe line")
            .build()
    }

    fn analyze(target: &Node) -> Result<Vec<Provider>, Error> {
        let Some(recipe) = target.recipe() else {
            return Ok(Vec::new());
        };
        let name = target.name();
        for line in &recipe.lines {
            let expanded = match target.expand(&line.text) {
                Ok(text) => text,
                Err(e) => format!("ERROR {}", e.message),
            };
            REPORT.with(|r| {
                r.borrow_mut()
                    .push_str(&format!("expand\t{name}\t{}\t{expanded}\n", line.text))
            });
        }
        if name == "out.o" {
            let env = match target.recipe_environment() {
                Ok(env) => env,
                Err(e) => vec![("ERROR".to_string(), e.message)],
            };
            for (k, v) in env {
                REPORT.with(|r| r.borrow_mut().push_str(&format!("env\t{k}\t{v}\n")));
            }
        }
        Ok(Vec::new())
    }

    fn finish() -> Result<(), Error> {
        let text = REPORT.with(|r| std::mem::take(&mut *r.borrow_mut()));
        let mut out = makers_plugin::open_output("report")?;
        out.write_all(text.as_bytes())
            .map_err(|e| makers_plugin::fail(e.to_string()))?;
        out.finish()
    }
}

export_plugin!(ExpandProbe);
