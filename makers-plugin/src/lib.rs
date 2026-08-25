//! Rust SDK for `makers` build plugins.
//!
//! A plugin author implements one trait and calls one macro. Nothing in a
//! plugin crate mentions WIT, `wit-bindgen`, or generated bindings:
//!
//! ```ignore
//! use makers_plugin::{prelude::*, Analyzer, Manifest, Node, Provider};
//!
//! struct Mine;
//!
//! impl Analyzer for Mine {
//!     fn describe() -> PluginInfo {
//!         Manifest::new("mine", "0.1.0")
//!             .description("counts targets")
//!             .build()
//!     }
//!     fn analyze(target: &Node) -> Result<Vec<Provider>, Error> {
//!         makers_plugin::note(&format!("saw {}", target.name()));
//!         Ok(vec![])
//!     }
//! }
//!
//! makers_plugin::export_plugin!(Mine);
//! ```
//!
//! See `wit/makers-plugin/` for the interface this wraps, and
//! `docs/plugin-api.md` for why it is shaped the way it is.

wit_bindgen::generate!({
    path: "../wit/makers-plugin",
    world: "analyzer-plugin",
    pub_export_macro: true,
    default_bindings_module: "$crate",
});

// ─── Host interfaces, re-exported under short names ──────────────────────

/// The resolved build graph: [`Node`], [`NodeSet`], and the entry points
/// ([`graph::root`], [`graph::goals`], [`graph::find`], [`graph::reachable`],
/// [`graph::topological_order`]).
pub use crate::makers::plugin::graph;
/// Facts about this make invocation and this instance's settings.
pub use crate::makers::plugin::session;
/// Global variable lookup and (capability-gated) makefile expansion.
pub use crate::makers::plugin::vars;

pub use crate::makers::plugin::graph::{
    DepEdge, DepFlags, Node, NodeKind, NodeSet, Recipe, RecipeLine, VarFlavor, VarOrigin, Variable,
};
pub use crate::makers::plugin::types::{Diagnostic, Error, Location, Provider, Severity};

pub use crate::exports::makers::plugin::plugin::{
    Capability, FailurePolicy, OutputDecl, Phase, PluginInfo,
};

/// Everything a typical plugin wants in scope.
pub mod prelude {
    pub use crate::{
        Analyzer, Automatics, Capability, DepEdge, DepFlags, Error, FailurePolicy, Location,
        Manifest, Node, NodeKind, NodeSet, Output, Phase, PluginInfo, Provider, Recipe, VarOrigin,
        Variable, error, error_at, export_plugin, note, note_at, warn, warn_at,
    };
}

// ─── Diagnostics ─────────────────────────────────────────────────────────

fn emit(severity: Severity, message: &str, location: Option<Location>) {
    crate::makers::plugin::diagnostics::emit(&Diagnostic {
        severity,
        message: message.to_string(),
        location,
    });
}

/// Report something informational. Shown under `--plugin-verbose`.
pub fn note(message: &str) {
    emit(Severity::Note, message, None);
}

/// Report a problem that does not stop this plugin.
pub fn warn(message: &str) {
    emit(Severity::Warning, message, None);
}

/// Report a problem. Fails the build only if this plugin declared
/// [`FailurePolicy::Fatal`] *and* was granted [`Capability::FailBuild`];
/// otherwise it is reported and the build continues.
pub fn error(message: &str) {
    emit(Severity::Error, message, None);
}

/// [`note`] with a makefile location attached.
pub fn note_at(location: Location, message: &str) {
    emit(Severity::Note, message, Some(location));
}

/// [`warn`] with a makefile location attached.
pub fn warn_at(location: Location, message: &str) {
    emit(Severity::Warning, message, Some(location));
}

/// [`error`] with a makefile location attached.
pub fn error_at(location: Location, message: &str) {
    emit(Severity::Error, message, Some(location));
}

/// Build an [`Error`] to return from a hook, aborting this plugin.
pub fn fail(message: impl Into<String>) -> Error {
    Error {
        message: message.into(),
        location: None,
    }
}

// ─── Artifacts ───────────────────────────────────────────────────────────

/// A declared output file, opened with [`open_output`].
///
/// Implements [`std::io::Write`], so `write!`/`writeln!` and
/// `serde_json::to_writer` work directly. Nothing is published until
/// [`Output::finish`] is called: an `Output` dropped early — including one
/// dropped by a panic or a trap — leaves any previous file untouched.
pub struct Output(crate::makers::plugin::artifacts::Output);

impl Output {
    /// Publish the artifact atomically.
    pub fn finish(self) -> Result<(), Error> {
        self.0.finish()
    }
}

impl std::io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0
            .write(buf)
            .map(|()| buf.len())
            .map_err(|e| std::io::Error::other(e.message))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Open one of the outputs this plugin declared in its manifest.
pub fn open_output(logical_name: &str) -> Result<Output, Error> {
    crate::makers::plugin::artifacts::open(logical_name).map(Output)
}

/// The absolute path a declared output will be written to.
pub fn output_path(logical_name: &str) -> Option<String> {
    crate::makers::plugin::artifacts::path_of(logical_name)
}

// ─── Manifest ────────────────────────────────────────────────────────────

/// Builder for the [`PluginInfo`] a plugin returns from
/// [`Analyzer::describe`].
///
/// Defaults are the least-privilege ones: analysis phase only, no
/// capabilities, no outputs, not deterministic, advisory failures. Every
/// authority a plugin holds is therefore visible as an explicit line in its
/// own source.
pub struct Manifest(PluginInfo);

impl Manifest {
    /// Start from the least-privilege defaults.
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Manifest(PluginInfo {
            name: name.into(),
            version: version.into(),
            description: String::new(),
            phases: vec![Phase::Analyze],
            capabilities: Vec::new(),
            outputs: Vec::new(),
            deterministic: false,
            failure_policy: FailurePolicy::Advisory,
        })
    }

    /// One line, shown by `--plugin-list` and in load errors.
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.0.description = d.into();
        self
    }

    /// Request an authority. The host grants the intersection of what is
    /// requested here and what it was configured to allow.
    pub fn capability(mut self, c: Capability) -> Self {
        self.0.capabilities.push(c);
        self
    }

    /// Declare an output file. `default_path` is relative to make's working
    /// directory and may be overridden with
    /// `--plugin-arg <instance>:out.<logical_name>=<path>`.
    pub fn output(
        mut self,
        logical_name: impl Into<String>,
        default_path: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.0.outputs.push(OutputDecl {
            logical_name: logical_name.into(),
            default_path: default_path.into(),
            description: description.into(),
        });
        self
    }

    /// Promise that the declared outputs are a pure function of what this
    /// plugin reads through the host interfaces, letting the host skip the
    /// run entirely when [`session::input_digest`] is unchanged.
    ///
    /// Rejected at load time in combination with [`Capability::WallClock`],
    /// [`Capability::ReadEnvironment`] or [`Capability::ExpandVariables`],
    /// each of which is a way to depend on something the digest does not
    /// cover — the last because expansion can run `$(shell ...)`, so a
    /// plugin that needs make's own expander has to give this up.
    pub fn deterministic(mut self) -> Self {
        self.0.deterministic = true;
        self
    }

    /// Make this plugin's failures fail the build. Requires
    /// [`Capability::FailBuild`], which this adds for you.
    pub fn fatal(mut self) -> Self {
        self.0.failure_policy = FailurePolicy::Fatal;
        if !self.0.capabilities.contains(&Capability::FailBuild) {
            self.0.capabilities.push(Capability::FailBuild);
        }
        self
    }

    pub fn build(self) -> PluginInfo {
        self.0
    }
}

// ─── The trait a plugin implements ───────────────────────────────────────

/// An analysis-phase plugin: an aspect over the resolved build graph.
///
/// [`Analyzer::analyze`] is called once per node reachable from the goals,
/// in dependency order — every prerequisite of `target` has already been
/// analysed, so `target.deps()` carries their published [`Provider`]s. That
/// ordering is what makes bottom-up accumulation (link lines, transitive
/// include paths, licence sets) expressible at all.
pub trait Analyzer {
    /// Identity, phases, capabilities and outputs. Called before the plugin
    /// is granted anything, in a store with no capabilities at all — so it
    /// must not call any other host function.
    fn describe() -> PluginInfo;

    /// Called once before the walk.
    fn start() -> Result<(), Error> {
        Ok(())
    }

    /// Called once per reachable node, prerequisites first. Returned
    /// providers are published on `target`.
    fn analyze(target: &Node) -> Result<Vec<Provider>, Error>;

    /// Called once after the walk. Write accumulated artifacts here.
    fn finish() -> Result<(), Error> {
        Ok(())
    }
}

/// Register `$ty` as this component's plugin. Call once, at crate root.
///
/// `$ty` must be captured as an `ident`, not a `ty`: a `ty` fragment is
/// opaque once parsed and the generated `export!` macro re-matches it as an
/// `ident`, which fails with an error pointing at this crate rather than at
/// the caller.
#[macro_export]
macro_rules! export_plugin {
    ($ty:ident) => {
        const _: () = {
            impl $crate::exports::makers::plugin::plugin::Guest for $ty {
                fn describe() -> $crate::PluginInfo {
                    <$ty as $crate::Analyzer>::describe()
                }
            }

            impl $crate::exports::makers::plugin::analyzer::Guest for $ty {
                fn start() -> ::core::result::Result<(), $crate::Error> {
                    <$ty as $crate::Analyzer>::start()
                }

                fn analyze(
                    target: &$crate::Node,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<$crate::Provider>,
                    $crate::Error,
                > {
                    <$ty as $crate::Analyzer>::analyze(target)
                }

                fn finish() -> ::core::result::Result<(), $crate::Error> {
                    <$ty as $crate::Analyzer>::finish()
                }
            }
        };

        $crate::export!($ty with_types_in $crate);
    };
}

// ─── Automatic variables and lightweight expansion ───────────────────────

/// The automatic variables for one node.
///
/// These are not fetched from the host: `$@` *is* the node's name and `$^`
/// *is* its prerequisite list, so a plugin already holds everything needed
/// to substitute them. That is why the interface has no `expand-for`
/// — see the note in `wit/makers-plugin/vars.wit`.
pub struct Automatics {
    /// `$@` — the target.
    pub target: String,
    /// `$<` — the first prerequisite, empty if there are none.
    pub first_dep: String,
    /// `$^` — all prerequisites, deduplicated, in makefile order, with
    /// order-only prerequisites excluded exactly as make excludes them.
    pub deps: Vec<String>,
    /// `$|` — the order-only prerequisites.
    pub order_only_deps: Vec<String>,
    /// `$*` — the implicit-rule stem.
    pub stem: String,
}

/// Collect a node's automatic variables.
pub fn automatics(node: &Node) -> Automatics {
    let mut deps = Vec::new();
    let mut order_only = Vec::new();
    for edge in node.dep_edges() {
        if edge.flags.contains(DepFlags::ORDER_ONLY) {
            if !order_only.contains(&edge.name) {
                order_only.push(edge.name);
            }
        } else if !deps.contains(&edge.name) {
            deps.push(edge.name);
        }
    }
    Automatics {
        target: node.name(),
        first_dep: deps.first().cloned().unwrap_or_default(),
        deps,
        order_only_deps: order_only,
        stem: node.stem().unwrap_or_default(),
    }
}

impl Automatics {
    /// The value of one automatic variable, or `None` if `name` is not one.
    pub fn get(&self, name: &str) -> Option<String> {
        Some(match name {
            "@" => self.target.clone(),
            "<" => self.first_dep.clone(),
            "^" => self.deps.join(" "),
            "|" => self.order_only_deps.join(" "),
            "*" => self.stem.clone(),
            _ => return None,
        })
    }
}

/// Substitute `$(NAME)`, `${NAME}` and single-character `$X` references
/// using `lookup`, leaving anything `lookup` does not resolve untouched.
///
/// This is deliberately *not* make's expander: it does not call functions,
/// does not handle `$(patsubst ...)`, and does not re-expand results more
/// than `depth` times. A plugin that needs real expansion asks for the
/// `expand-variables` capability and calls [`vars::expand`] — which is
/// gated precisely because it can run `$(shell ...)` in make's own process.
///
/// Most plugins do not need that. Resolving `$@`, `$<` and a handful of
/// `$(CFLAGS)`-shaped references against [`Node::variable`] — which already
/// applies per-target and pattern-specific precedence — covers the ordinary
/// compile-database and build-description cases without handing a plugin the
/// ability to execute commands.
pub fn expand_with(text: &str, depth: usize, lookup: impl Fn(&str) -> Option<String>) -> String {
    let mut current = text.to_string();
    for _ in 0..depth.max(1) {
        let next = expand_once(&current, &lookup);
        if next == current {
            break;
        }
        current = next;
    }
    current
}

fn expand_once(text: &str, lookup: &impl Fn(&str) -> Option<String>) -> String {
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'$' {
            let start = i;
            while i < bytes.len() && bytes[i] != b'$' {
                i += 1;
            }
            out.push_str(&text[start..i]);
            continue;
        }
        // `$$` is a literal dollar.
        if bytes.get(i + 1) == Some(&b'$') {
            out.push('$');
            i += 2;
            continue;
        }
        let (open, close) = match bytes.get(i + 1) {
            Some(b'(') => (b'(', b')'),
            Some(b'{') => (b'{', b'}'),
            Some(_) => {
                // `$X`: a one-character reference, which in practice is
                // always an automatic.
                let name = &text[i + 1..i + 2];
                match lookup(name) {
                    Some(v) => out.push_str(&v),
                    None => out.push_str(&text[i..i + 2]),
                }
                i += 2;
                continue;
            }
            None => {
                out.push('$');
                i += 1;
                continue;
            }
        };
        // Balanced scan, so `$(foo $(bar))` is left alone as one unit when
        // `foo ...` does not resolve.
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
            out.push_str(&text[i..]);
            break;
        }
        let name = &text[i + 2..j - 1];
        match lookup(name) {
            Some(v) => out.push_str(&v),
            None => out.push_str(&text[i..j]),
        }
        i = j;
    }
    out
}

/// Expand one recipe line for `node` the way a build-description generator
/// wants it: automatics from the node, everything else from the node's own
/// target-scoped variable lookup.
///
/// References the host cannot resolve — function calls, undefined variables
/// — are left in place, so a caller can detect them and report rather than
/// silently emitting a half-expanded command.
pub fn expand_recipe_line(node: &Node, line: &str) -> String {
    let auto = automatics(node);
    expand_with(line, 8, |name| {
        auto.get(name)
            .or_else(|| node.variable(name).map(|v| v.value))
    })
}
