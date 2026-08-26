//! End-to-end wasm introspection extension: run the real `make` binary on a
//! real makefile with `MAKERS_WASM_EXTENSION` pointing at the compiled
//! `wit/introspection` guest, and verify its push-based `visitor` callbacks
//! (`visiting-child`/`visit-file`/`visit-done`, #644) see the actual
//! resolved build graph (including implicit-rule matches) over stderr.
//!
//! Only compiled under `--features wasmtime`, matching the feature the
//! `MAKERS_WASM_EXTENSION` hook itself is gated behind (`src/wasm_ext.rs`).
//! Requires the guest component to already be built:
//! `cargo component build --manifest-path wit/introspection/Cargo.toml`.

#![cfg(feature = "wasmtime")]

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

const RUST_MAKE: &str = env!("CARGO_BIN_EXE_make");

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn introspection_component() -> PathBuf {
    manifest_dir().join("target/wasm32-wasip1/debug/introspection.wasm")
}

fn tempdir() -> PathBuf {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let dir = std::env::temp_dir().join(format!(
        "makers-wasm-ext-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).expect("create tempdir");
    dir
}

#[test]
fn real_makefile_is_introspected_by_the_wasm_guest() {
    let component = introspection_component();
    assert!(
        component.exists(),
        "build the guest first: cargo component build --manifest-path wit/introspection/Cargo.toml \
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
    // The root->goal edge, resolved to the goal's real name (`DepNode::name`
    // is empty for goals — see `entry::goaldep_for_file` — so this proves
    // the host resolved identity through `dep.file` rather than forwarding
    // the empty string).
    assert!(
        stderr.contains("edge <root> -> prog"),
        "goal edge from the synthetic root: {stderr}"
    );
    // The explicit target and its real, implicit-rule-resolved prerequisite —
    // proof the guest saw the host's *resolved* graph, not just the literal
    // makefile text (`main.o: main.c` only exists after `%.o: %.c` matches).
    assert!(
        stderr.contains("visit prog (deps: main.o, util.o, gen.tab.c, outdir)"),
        "explicit target with its listed deps (incl. the order-only prereq): {stderr}"
    );
    assert!(
        stderr.contains("visit main.o (deps: main.c)"),
        "implicit-rule-resolved dep, not just literal text: {stderr}"
    );
    assert!(
        stderr.trim_end().ends_with("traversal done"),
        "visit-done fires once traversal completes: {stderr}"
    );
}
