# makers-plugin

Rust SDK for [`makers`](https://github.com/sevki/makers) build plugins.

A plugin is a WebAssembly component that `make` loads, grants a declared set
of capabilities, and runs against the resolved build graph. This crate wraps
the `makers:plugin@1.0.0` WIT interface so a plugin author writes plain Rust:
implement one trait, call one macro. No WIT, no `wit-bindgen`, no generated
bindings to touch.

See [`docs/plugin-api.md`](../docs/plugin-api.md) for the interface design
and the reasoning behind it, and [`wit/makers-plugin/`](../wit/makers-plugin)
for the contract itself.

## A plugin, end to end

```toml
# Cargo.toml
[package]
name = "my-plugin"
version = "0.1.0"
edition = "2021"

[dependencies]
makers-plugin = { path = "../../makers-plugin" }

[lib]
crate-type = ["cdylib"]

[workspace]
```

```rust
use makers_plugin::prelude::*;
use std::io::Write as _;

struct CountTargets;

impl Analyzer for CountTargets {
    fn describe() -> PluginInfo {
        Manifest::new("count-targets", env!("CARGO_PKG_VERSION"))
            .description("counts the targets in a build")
            .capability(Capability::WriteOutputs)
            .deterministic()
            .output("report", "targets.txt", "one line per target")
            .build()
    }

    // Called once per node reachable from the goals, prerequisites first.
    fn analyze(target: &Node) -> Result<Vec<Provider>, Error> {
        if target.is_target() {
            makers_plugin::note(&format!("{}", target.name()));
        }
        Ok(vec![])
    }

    fn finish() -> Result<(), Error> {
        let mut out = makers_plugin::open_output("report")?;
        writeln!(out, "done").map_err(|e| makers_plugin::fail(e.to_string()))?;
        out.finish()
    }
}

export_plugin!(CountTargets);
```

Build and run it:

```sh
cargo component build                 # needs wasm32-wasip1 + cargo-component
MAKERS_PLUGINS="count=target/wasm32-wasip1/debug/my_plugin.wasm" make
```

(`make` must be built with `--features wasmtime`.)

## What the host gives you

| module | what it is |
|---|---|
| [`Node`] | a graph node — name, flags, recipe, target-scoped variables, edges. Every accessor is a call, so you pay only for what you read. |
| [`NodeSet`] | a nested set with O(1) `union` and explicit `to_list` — Bazel's `depset`, so accumulating up the graph is not quadratic. |
| [`Provider`] | a namespaced, opaque value published on a node. Visible to that node's dependents and to later plugins: this is how two plugins compose. |
| [`session`] | this invocation and this instance's settings, plus [`session::input_digest`]. |
| [`vars`] | global variable lookup, and (capability-gated) make's own expander. |
| [`open_output`] | a declared output file, buffered and published atomically. |
| [`note`]/[`warn`]/[`error`] | diagnostics, optionally with a makefile [`Location`]. |

## Capabilities

Everything a plugin may do is declared in [`Manifest`] and granted by the
host. The defaults are the least-privilege ones — analysis phase only, no
capabilities, no outputs, advisory failures — so every authority a plugin
holds is a visible line in its own source. `read-recipes`,
`read-variables` and `write-outputs` are granted by default; anything that
reaches outside the makefiles make already read (`read-file-content`,
`read-environment`, `wall-clock`, `expand-variables`) and `fail-build` are
opt-in per instance:

```sh
MAKERS_PLUGIN_ALLOW="count:read-file-content"
```

## Configuration

```sh
MAKERS_PLUGINS="name=path[,name=path…]"
MAKERS_PLUGIN_ARGS="name.key=value[;name.key=value…]"   # `name.out.<logical>=<path>` retargets an output
MAKERS_PLUGIN_ALLOW="name:cap[,cap…][;name:cap…]"       # `*` for every instance, `all` for every capability
MAKERS_PLUGIN_DENY="name:cap[,cap…]"
MAKERS_PLUGIN_VERBOSE=1
```

Plugins run in configuration order, sharing one provider map.

## Examples

* [`plugins/compile-commands`](../plugins/compile-commands) — produces
  `compile_commands.json` for clangd, from the graph rather than by
  intercepting `exec`.
* [`plugins/graphviz-export`](../plugins/graphviz-export) — Graphviz DOT of
  the resolved graph, annotating any node that carries another plugin's
  `makers:cc/compile-command` provider.
