# makers-plugin

SDK for `make` wasm extensions (#633). Implement [`Plugin`] and register it
with [`export_plugin!`] — no WIT, no `wit-bindgen`, no `cargo-component`
generated bindings to touch directly.

```toml
# Cargo.toml
[package]
name = "my-plugin"
edition = "2021"

[dependencies]
makers-plugin = { path = "../../makers-plugin" }

[lib]
crate-type = ["cdylib"]
```

```rust
use makers_plugin::{export_plugin, Dep, File, Plugin};

struct MyPlugin;

impl Plugin for MyPlugin {
    fn visit_file(file: File) -> Result<(), String> {
        eprintln!("visiting {}", file.name);
        Ok(())
    }

    fn visiting_child(parent: String, child: Dep) -> Result<(), String> {
        Ok(())
    }

    fn visit_done() -> Result<(), String> {
        Ok(())
    }
}

export_plugin!(MyPlugin);
```

Build it as a component with `cargo component build` (needs the
`wasm32-wasip1` target and the `cargo-component` subcommand) — no `wit/`
directory of your own required; the WIT interface travels with this crate.
Run it against a real build with `MAKERS_WASM_EXTENSION=path/to/plugin.wasm
make ...` (needs `make` built with `--features wasmtime`).

See `plugins/graphviz-export` for a complete example.
