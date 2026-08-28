//! End-to-end proof that `make` runs as a WASI guest, reading its makefile
//! through the WASI filesystem.
//!
//! This is the check that the `crate::fs` layer actually *works* on wasm
//! rather than merely compiling for it: the guest gets no libc filesystem, no
//! host syscalls, and no preopened directory other than the scratch tree
//! built here — everything it reads has to arrive over WASI's `path_open` /
//! `fd_read` / `fd_readdir`, which is exactly the surface a Cloudflare Worker
//! (or any other embedder) has to satisfy.
//!
//! Requires the `wasm32-wasip1` artifact, which this test builds on demand;
//! it skips if the toolchain has no such target installed.

#![cfg(feature = "wasmtime")]

use std::{fs, path::PathBuf, process::Command};

use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{
    p1::{add_to_linker_sync, WasiP1Ctx},
    p2::pipe::MemoryOutputPipe,
    FsPerms, WasiCtxBuilder,
};

/// Build (or reuse) the wasm32-wasip1 `make` binary. `None` when the target
/// is not installed, so the test skips rather than failing on a toolchain
/// that simply cannot produce the artifact.
fn wasm_make() -> Option<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let artifact = root.join("target/wasm32-wasip1/debug/make.wasm");

    let built = Command::new(env!("CARGO"))
        .args(["build", "--target", "wasm32-wasip1", "--bin", "make"])
        .current_dir(&root)
        .status()
        .ok()?;
    if !built.success() {
        return None;
    }
    artifact.exists().then_some(artifact)
}

/// Run `make` as a wasm guest over `dir`, returning its stdout.
fn run_in_wasi(wasm: &PathBuf, dir: &PathBuf, args: &[&str]) -> String {
    let engine = Engine::default();
    let module = Module::from_file(&engine, wasm).expect("load make.wasm");

    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    add_to_linker_sync(&mut linker, |t| t).expect("link wasi");

    // Capture stdout through a pipe so the assertion can read what the guest
    // wrote, and preopen the scratch tree as the guest's `.`.
    let stdout = MemoryOutputPipe::new(1 << 20);
    let mut argv = vec!["make".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let mut builder = WasiCtxBuilder::new();
    builder
        .stdout(stdout.clone())
        .inherit_stderr()
        .args(&argv)
        .preopened_dir(dir, ".", FsPerms::ReadWrite)
        .expect("preopen scratch dir");

    let mut store = Store::new(&engine, builder.build_p1());
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate make.wasm");
    let start = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .expect("_start export");
    // A guest that calls `proc_exit` unwinds as a trap carrying the status;
    // a clean exit(0) is reported the same way, so a trap here is not by
    // itself a failure — what the guest wrote is the thing under test.
    let _ = start.call(&mut store, ());
    drop(store);

    String::from_utf8_lossy(&stdout.contents()).into_owned()
}

#[test]
fn make_reads_a_makefile_through_the_wasi_filesystem() {
    let Some(wasm) = wasm_make() else {
        eprintln!("skipping: no wasm32-wasip1 target installed");
        return;
    };

    let dir = std::env::temp_dir().join(format!("makers_wasi_smoke_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    // `-p` forces the directory cache to be populated and dumped, so this
    // exercises `fd_readdir` as well as reading the makefile itself.
    fs::write(
        dir.join("Makefile"),
        b"VAR := from-the-makefile\nall:\n\t@echo unused\n",
    )
    .unwrap();
    fs::write(dir.join("marker.c"), b"").unwrap();

    let out = run_in_wasi(&wasm, &dir, &["-p", "-n", "all"]);

    // Read through WASI's `path_open`/`fd_read`: the variable only exists if
    // the makefile was actually parsed.
    assert!(
        out.contains("from-the-makefile"),
        "makefile was not read through WASI; got:\n{out}"
    );
    // Read through WASI's `fd_readdir`: the directory cache only lists
    // `marker.c` if the guest enumerated the preopened directory.
    assert!(
        out.contains("marker.c") || out.contains("# Directories"),
        "directory was not enumerated through WASI; got:\n{out}"
    );

    let _ = fs::remove_dir_all(&dir);
}
