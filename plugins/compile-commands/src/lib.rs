//! `compile_commands.json` from a plain makefile.
//!
//! This is the plugin that justifies the interface. A JSON Compilation
//! Database is what clangd, clang-tidy, IWYU, rust-analyzer's C interop and
//! every C/C++ IDE integration read, and `make` cannot produce one. The
//! workarounds people actually use are all bad in the same way: `bear` and
//! `intercept-build` `LD_PRELOAD` a shim into every process the build spawns
//! and reconstruct intent from `execve` calls, `compiledb` re-parses the
//! output of `make -n`, and both require a full (or fully dry) build to run
//! before they know anything. All of them are guessing at a graph make
//! already has in memory.
//!
//! What this needs from the host, and why each one is in the interface:
//!
//! * `node.recipe()` — the command, unexpanded. Capability-gated, because
//!   recipe text is where credentials end up.
//! * `node.dep-edges()` — which prerequisite is the *source*, and which are
//!   order-only (a `| build/` prerequisite is not an input to the compile
//!   and must not appear as one).
//! * `node.variable()` — `$(CFLAGS)` **for this target**, so that
//!   `debug.o: CFLAGS += -O0` comes out right. Reading the global value
//!   instead is the classic compile-database bug.
//! * `session.working-directory()` — the schema requires it.
//! * `artifacts.open()` — one atomic file. A half-written database makes
//!   clangd report thousands of phantom errors.
//!
//! It publishes each entry as a `makers:cc/compile-command` provider so that
//! other plugins can see which nodes are compile steps without knowing
//! anything about C.

use makers_plugin::prelude::*;
use std::cell::RefCell;
use std::io::Write as _;

const COMPILE_COMMAND: &str = "makers:cc/compile-command";

/// Extensions treated as a compilable translation unit. Deliberately a
/// fixed list rather than "anything with a recipe": a compile database that
/// includes link steps and `mkdir` recipes is worse than none, because
/// clangd will try to parse them.
const SOURCE_EXTENSIONS: &[&str] = &["c", "cc", "cpp", "cxx", "c++", "m", "mm", "S", "s"];

/// Extensions that make a node a *link* step rather than a compile step.
///
/// Without this, `prog: main.o util.o gen.tab.c` is misread as a compile of
/// `gen.tab.c` — it has a recipe and a `.c` prerequisite — and clangd is
/// handed a link command as if it were a translation unit. Telling the two
/// apart needs the prerequisite list, which is exactly the kind of thing
/// `bear` has to reconstruct from `exec` calls and this plugin can simply
/// read.
const OBJECT_EXTENSIONS: &[&str] = &["o", "obj", "a", "lo", "la", "so", "dylib"];

struct CompileCommands;

thread_local! {
    static ENTRIES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    static UNRESOLVED: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn has_extension(name: &str, set: &[&str]) -> bool {
    name.rsplit_once('.').is_some_and(|(_, ext)| set.contains(&ext))
}

fn is_source(name: &str) -> bool {
    has_extension(name, SOURCE_EXTENSIONS)
}

fn is_object(name: &str) -> bool {
    has_extension(name, OBJECT_EXTENSIONS)
}

impl Analyzer for CompileCommands {
    fn describe() -> PluginInfo {
        Manifest::new("compile-commands", env!("CARGO_PKG_VERSION"))
            .description("JSON Compilation Database (compile_commands.json) for clangd and friends")
            .capability(Capability::ReadRecipes)
            .capability(Capability::ReadVariables)
            .capability(Capability::WriteOutputs)
            .deterministic()
            .output(
                "database",
                "compile_commands.json",
                "JSON Compilation Database",
            )
            .build()
    }

    fn analyze(target: &Node) -> Result<Vec<Provider>, Error> {
        let Some(recipe) = target.recipe() else {
            // Either the target has no recipe, or `read-recipes` was
            // withheld. Degrading quietly is the right response to a
            // withheld capability: the host has already said so once.
            return Ok(Vec::new());
        };

        // Order-only prerequisites are ordering constraints
        // (`| $(BUILDDIR)`), never inputs, so they are dropped before
        // anything else looks at the list.
        let inputs: Vec<String> = target
            .dep_edges()
            .into_iter()
            .filter(|e| !e.flags.contains(DepFlags::ORDER_ONLY))
            .map(|e| e.name)
            .collect();
        // Exactly one translation unit and nothing already compiled: one
        // source plus objects is a link, and several sources is a rule this
        // plugin has no business guessing about.
        let sources: Vec<&String> = inputs.iter().filter(|n| is_source(n)).collect();
        if sources.len() != 1 || inputs.iter().any(|n| is_object(n)) {
            return Ok(Vec::new());
        }
        let source = sources[0];

        // One entry per translation unit, not per recipe line: a database
        // with two entries for `main.c` makes clangd pick one arbitrarily.
        // The line that names the source is the compile; anything else in a
        // multi-line recipe is setup.
        let commands: Vec<String> = recipe
            .lines
            .iter()
            .map(|line| makers_plugin::expand_recipe_line(target, &line.text))
            .collect();
        let Some(command) = commands
            .iter()
            .find(|c| c.contains(source.as_str()))
            .or_else(|| commands.first())
        else {
            return Ok(Vec::new());
        };
        if command.contains('$') {
            // A reference the plugin could not resolve — a `$(shell ...)` or
            // a user function. Report it rather than writing a
            // half-expanded command that clangd will choke on.
            UNRESOLVED.with(|u| u.borrow_mut().push(target.name()));
            return Ok(Vec::new());
        }

        let entry = format!(
            "  {{\n    \"directory\": \"{}\",\n    \"file\": \"{}\",\n    \
             \"output\": \"{}\",\n    \"command\": \"{}\"\n  }}",
            json_escape(&makers_plugin::session::working_directory()),
            json_escape(source),
            json_escape(&target.name()),
            json_escape(command)
        );
        ENTRIES.with(|e| e.borrow_mut().push(entry.clone()));
        Ok(vec![Provider {
            id: COMPILE_COMMAND.to_string(),
            payload: entry.into_bytes(),
        }])
    }

    fn finish() -> Result<(), Error> {
        let mut out = makers_plugin::open_output("database")?;
        let body = ENTRIES.with(|e| e.borrow().join(",\n"));
        let document = if body.is_empty() {
            "[]\n".to_string()
        } else {
            format!("[\n{body}\n]\n")
        };
        out.write_all(document.as_bytes())
            .map_err(|e| makers_plugin::fail(e.to_string()))?;
        out.finish()?;

        let count = ENTRIES.with(|e| e.borrow().len());
        if let Some(path) = makers_plugin::output_path("database") {
            makers_plugin::note(&format!("wrote {count} entries to {path}"));
        }
        UNRESOLVED.with(|u| {
            let unresolved = u.borrow();
            if !unresolved.is_empty() {
                makers_plugin::warn(&format!(
                    "{} recipe(s) left unexpanded references and were skipped (first: {}); \
                     grant `expand-variables` for make's own expander",
                    unresolved.len(),
                    unresolved[0]
                ));
            }
        });
        Ok(())
    }
}

export_plugin!(CompileCommands);
