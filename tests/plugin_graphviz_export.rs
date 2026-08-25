//! End-to-end test for the `graphviz-export` plugin (`plugins/graphviz-export`):
//! run the real `make` binary on a real makefile with `MAKERS_WASM_EXTENSION`
//! pointing at the compiled plugin, and verify the DOT graph it emits over
//! stderr matches the actual resolved build graph.
//!
//! Only compiled under `--features wasmtime`, matching the feature the
//! `MAKERS_WASM_EXTENSION` hook itself is gated behind (`src/wasm_ext.rs`).
//! Requires the plugin to already be built. `plugins/graphviz-export` is its
//! own standalone workspace (see its `Cargo.toml`), so build it from within
//! that directory rather than via `--manifest-path` -- the latter would
//! silently write to `plugins/graphviz-export/target/` instead of the
//! repo-root `target/` this test looks in (see `.cargo/config.toml` there):
//! `(cd plugins/graphviz-export && cargo component build)`.

#![cfg(feature = "wasmtime")]

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

const RUST_MAKE: &str = env!("CARGO_BIN_EXE_make");

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn plugin_component() -> PathBuf {
    manifest_dir().join("target/wasm32-wasip1/debug/graphviz_export.wasm")
}

fn tempdir() -> PathBuf {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let dir = std::env::temp_dir().join(format!(
        "makers-plugin-graphviz-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).expect("create tempdir");
    dir
}

#[test]
fn real_makefile_is_exported_as_a_dot_graph() {
    let component = plugin_component();
    assert!(
        component.exists(),
        "build the plugin first: (cd plugins/graphviz-export && cargo component build) \
         (looked for {})",
        component.display()
    );

    let fixture = manifest_dir().join("tests/fixtures/depgraph.mk");
    let workdir = tempdir();
    std::fs::copy(&fixture, workdir.join("Makefile")).expect("copy fixture");
    std::fs::write(workdir.join("main.c"), "").expect("write main.c");
    std::fs::write(workdir.join("util.c"), "").expect("write util.c");
    std::fs::write(workdir.join("gen.y"), "").expect("write gen.y");

    let out = Command::new(RUST_MAKE)
        .args(["--no-print-directory", "-r", "-n", "-f", "Makefile"])
        .env("MAKERS_WASM_EXTENSION", &component)
        .env_remove("MAKEFLAGS")
        .env_remove("GNUMAKEFLAGS")
        .env_remove("MAKEFILES")
        .current_dir(&workdir)
        .output()
        .expect("spawn make");

    assert!(
        out.status.success(),
        "make -rn on the fixture should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        !stderr.contains("wasm extension failed"),
        "extension should run cleanly: {stderr}"
    );
    assert!(
        stderr.contains("digraph make {"),
        "emits a DOT graph: {stderr}"
    );
    // The order-only prerequisite `outdir` is `.PHONY`-declared in the
    // fixture — proof the plugin read `FileNode::phony`, not just names.
    assert!(
        stderr.contains("\"outdir\" [shape=box, style=dashed];"),
        "phony targets get the dashed-box style: {stderr}"
    );
    // The root->goal edge, resolved to the goal's real name (see
    // tests/wasm_ext.rs for why this is non-trivial: `DepNode::name` is
    // empty for goals).
    assert!(
        stderr.contains("\"<root>\" -> \"prog\";"),
        "goal edge from the synthetic root: {stderr}"
    );
    // An implicit-rule-resolved edge, not just literal makefile text
    // (`main.o -> main.c` only exists after `%.o: %.c` matches).
    assert!(
        stderr.contains("\"main.o\" -> \"main.c\";"),
        "implicit-rule-resolved edge: {stderr}"
    );
}
