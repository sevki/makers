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
//! * `vars.expand()` — optional, and the reason this plugin does not declare
//!   `deterministic`. Target-scoped substitution handles `$(CC) $(CFLAGS)`
//!   without any extra authority; a recipe using a *function*
//!   (`$(addprefix -l,$(LIBS))`) needs make's real expander, which can also
//!   run `$(shell ...)` and so cannot be part of a determinism promise. The
//!   plugin works either way and says which mode it is in.
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

/// Characters that mean a `$(...)` span is a function call or a
/// substitution reference rather than a plain variable name.
fn is_plain_name(inner: &str) -> bool {
    !inner.is_empty()
        && !inner.chars().any(|c| {
            c.is_whitespace() || matches!(c, ',' | '$' | '(' | ')' | '{' | '}' | ':' | '=')
        })
}

/// Every plain variable reference in `text`, descending into function calls:
/// `$(addprefix -l,$(LIBS))` yields `LIBS`.
///
/// Iterative rather than recursive because `text` is makefile-supplied and
/// nesting depth is not bounded by anything this plugin controls; in a
/// component a stack overflow is a trap that loses the entire run.
fn referenced_variables(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut work = vec![text];
    while let Some(cur) = work.pop() {
        let bytes = cur.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] != b'$' {
                i += 1;
                continue;
            }
            let (open, close) = match bytes.get(i + 1) {
                Some(b'(') => (b'(', b')'),
                Some(b'{') => (b'{', b'}'),
                // `$$` is a literal dollar and `$X` is an automatic, which
                // `expand_recipe_line` has already substituted. Neither can
                // introduce a target-scoped reference here.
                _ => {
                    i += 1;
                    continue;
                }
            };
            let mut depth = 1;
            let mut j = i + 2;
            while j < bytes.len() && depth > 0 {
                if bytes[j] == open {
                    depth += 1;
                } else if bytes[j] == close {
                    depth -= 1;
                }
                j += 1;
            }
            if depth != 0 {
                break;
            }
            // `$`, `(`, `)`, `{` and `}` are ASCII, so these indices are
            // always on character boundaries.
            let inner = &cur[i + 2..j - 1];
            if is_plain_name(inner) {
                out.push(inner.to_string());
            } else {
                work.push(inner);
            }
            i = j;
        }
    }
    out
}

/// The first reference in `text` that make's *global* expander would resolve
/// differently from `target`.
///
/// This is the seam between the two expanders. `expand_recipe_line`
/// substitutes what it can in the target's scope and leaves balanced
/// function calls alone, so what reaches `vars::expand` is a call whose
/// arguments have never been looked at — and `vars::expand` runs in global
/// scope by design (there is deliberately no `expand-for`; see
/// `wit/makers-plugin/vars.wit`). For `debug.o: LIBS := debug` and a recipe
/// saying `$(addprefix -l,$(LIBS))`, that hands back the *global* `LIBS`,
/// and the result looks fully expanded, so nothing downstream can tell it is
/// wrong. A compile database that is confidently wrong about a flag is worse
/// than one missing an entry: clangd reports errors against source that
/// compiles.
fn globally_misexpanded(target: &Node, text: &str) -> Option<String> {
    referenced_variables(text).into_iter().find(|name| {
        let scoped = target.variable(name).map(|v| v.value);
        scoped.is_some() && scoped != makers_plugin::vars::get(name).map(|v| v.value)
    })
}

struct CompileCommands;

thread_local! {
    static ENTRIES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    static UNRESOLVED: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    static TARGET_SCOPED: RefCell<Vec<(String, String)>> = const { RefCell::new(Vec::new()) };
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
            // Optional: without it, recipes using makefile *functions* are
            // skipped rather than emitted half-expanded. Requested here so
            // that the host's withheld-capability report doubles as the
            // discovery path for turning it on.
            .capability(Capability::ExpandVariables)
            .capability(Capability::WriteOutputs)
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
        // What is left after target-scoped substitution is a makefile
        // function or an undefined variable. Finish it with make's own
        // expander if this instance was granted one; otherwise skip the
        // entry, because a half-expanded command line makes clangd report
        // phantom errors for the whole translation unit.
        let command = if command.contains('$') {
            // Refuse before asking the global expander a question whose
            // answer would be silently wrong for this target.
            if let Some(name) = globally_misexpanded(target, command) {
                TARGET_SCOPED.with(|t| t.borrow_mut().push((target.name(), name)));
                return Ok(Vec::new());
            }
            match makers_plugin::vars::expand(command) {
                Ok(expanded) if !expanded.contains('$') => expanded,
                _ => {
                    UNRESOLVED.with(|u| u.borrow_mut().push(target.name()));
                    return Ok(Vec::new());
                }
            }
        } else {
            command.clone()
        };

        let entry = format!(
            "  {{\n    \"directory\": \"{}\",\n    \"file\": \"{}\",\n    \
             \"output\": \"{}\",\n    \"command\": \"{}\"\n  }}",
            json_escape(&makers_plugin::session::working_directory()),
            json_escape(source),
            json_escape(&target.name()),
            json_escape(&command)
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
        TARGET_SCOPED.with(|t| {
            let scoped = t.borrow();
            if let Some((target, name)) = scoped.first() {
                makers_plugin::warn(&format!(
                    "{} recipe(s) were skipped because a function argument reads a \
                     target-specific variable make's global expander would resolve \
                     differently (first: `{name}` in {target})",
                    scoped.len()
                ));
            }
        });
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
