//! SDK for `make` wasm extensions: wraps the `makers:introspection` WIT
//! interface so a plugin author writes plain Rust — implement [`Plugin`],
//! call [`export_plugin!`] — without touching WIT, `wit-bindgen`, or
//! `cargo-component`'s generated bindings directly.

wit_bindgen::generate!({
    path: "../wit/introspection/wit/world.wit",
    world: "introspection-extension",
    pub_export_macro: true,
    default_bindings_module: "$crate",
});

/// A build-graph node, mirroring the host's `FileNode`. See the WIT record
/// docs in `wit/introspection/wit/world.wit` for field-by-field provenance.
pub use exports::makers::introspection::visitor::File;

/// A dependency edge, mirroring the host's `DepNode`.
pub use exports::makers::introspection::visitor::Dep;

/// Implement this on your plugin's type and register it with
/// [`export_plugin!`]. The host calls these hooks while it walks the
/// resolved build graph depth-first from the goals (#644) — strictly
/// observational, no hook can skip a node or influence traversal order.
pub use exports::makers::introspection::visitor::Guest as Plugin;

/// Auxiliary graph lookups, callable from inside any [`Plugin`] hook.
pub mod graph {
    pub use crate::makers::introspection::graph::{find_file, get_variable};
}

/// Register `$ty` as this component's plugin implementation. Call once, at
/// crate root:
///
/// ```ignore
/// struct MyPlugin;
/// impl makers_plugin::Plugin for MyPlugin { /* ... */ }
/// makers_plugin::export_plugin!(MyPlugin);
/// ```
#[macro_export]
macro_rules! export_plugin {
    ($ty:ident) => {
        $crate::export!($ty with_types_in $crate);
    };
}
