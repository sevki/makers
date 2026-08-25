//! End-to-end dependency-graph dump: run the real `make` binary on a real
//! makefile with `MAKERS_DEPGRAPH` / `MAKERS_DEPGRAPH_POST` set and verify
//! the Mermaid graphs it writes — the pre-walk snapshot ("what make plans")
//! and the post-walk snapshot ("what make discovered": implicit-rule
//! resolution and `DerivedBy` provenance).
//!
//! The dumped graphs double as the committed sample in
//! `docs/depgraph-makefile.md`; regenerate it with
//! `UPDATE_SNAPSHOTS=1 cargo test --test depgraph_dump`.

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

const RUST_MAKE: &str = env!("CARGO_BIN_EXE_make");

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// A fresh scratch directory per call (unique per pid + counter), so runs
/// never collide; contents are left behind for post-mortem inspection.
fn tempdir() -> PathBuf {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let dir = std::env::temp_dir().join(format!(
        "makers-depgraph-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).expect("create tempdir");
    dir
}

struct Dump {
    /// Pre-walk snapshot (`MAKERS_DEPGRAPH`).
    pre: String,
    /// Post-walk snapshot (`MAKERS_DEPGRAPH_POST`).
    post: String,
    status: std::process::ExitStatus,
    workdir: PathBuf,
}

/// Run the just-built `make` on the depgraph fixture in a scratch dir with
/// both dump variables pointing at `dump_name` / `post-<dump_name>`, and
/// return the dumped graphs. `-r` keeps builtin rules out of the graph,
/// `-n` keeps the run side-effect-free (but still walks the whole graph, so
/// implicit rules resolve); MAKEFLAGS-style env is scrubbed so the snapshots
/// are hermetic.
fn dump_graph(dump_name: &str) -> Dump {
    dump_graph_with_args(dump_name, &[])
}

fn dump_graph_with_args(dump_name: &str, extra_args: &[&str]) -> Dump {
    let fixture = manifest_dir().join("tests/fixtures/depgraph.mk");
    let workdir = tempdir();
    std::fs::copy(&fixture, workdir.join("Makefile")).expect("copy fixture");
    // The pattern rules' sources exist, so `-n` walks the whole graph and
    // exits 0 (commands are printed, never run).
    std::fs::write(workdir.join("main.c"), "").expect("write main.c");
    std::fs::write(workdir.join("util.c"), "").expect("write util.c");
    std::fs::write(workdir.join("gen.y"), "").expect("write gen.y");

    let pre = workdir.join(dump_name);
    let post = workdir.join(format!("post-{dump_name}"));
    let mut cmd = Command::new(RUST_MAKE);
    cmd.args(["--no-print-directory", "-r", "-n", "-f", "Makefile"])
        .args(extra_args)
        .env("MAKERS_DEPGRAPH", &pre)
        .env("MAKERS_DEPGRAPH_POST", &post)
        .env_remove("MAKEFLAGS")
        .env_remove("GNUMAKEFLAGS")
        .env_remove("MAKEFILES")
        .current_dir(&workdir);
    let status = cmd.status().expect("spawn make");
    let read = |p: &PathBuf| {
        std::fs::read_to_string(p)
            .unwrap_or_else(|err| panic!("dump not written to {}: {err}", p.display()))
    };
    Dump {
        pre: read(&pre),
        post: read(&post),
        status,
        workdir,
    }
}

#[test]
fn real_makefile_dumps_a_mermaid_graph() {
    let dump = dump_graph("graph.md");
    let graph = &dump.pre;
    assert!(
        dump.status.success(),
        "make -rn on the fixture should succeed"
    );

    assert!(
        graph.starts_with("```mermaid\nflowchart LR\n"),
        "markdown-fenced mermaid"
    );
    assert!(
        graph.contains("[\"prog\"]"),
        "goal target with a recipe: {graph}"
    );
    assert!(
        graph.contains("[[\"%.o: %.c\"]]"),
        "user pattern rule from the rule db: {graph}"
    );
    assert!(graph.contains(" ==> "), "goal edge from <root>: {graph}");
    assert!(
        graph.contains("-.->|order-only|"),
        "`| outdir` prerequisite: {graph}"
    );
    assert!(
        graph.contains("classDef phony"),
        "outdir is marked .PHONY: {graph}"
    );
    // Prerequisites named but never defined as targets resolve by name hash
    // and keep their dep-learned labels.
    assert!(graph.contains("([\"main.o\"])"), "{graph}");
    assert!(graph.contains("([\"util.o\"])"), "{graph}");
    // Pre-walk, implicit matching has not run: no sources, no provenance.
    assert!(
        !graph.contains("([\"main.c\"])"),
        "pre-walk has no resolved sources: {graph}"
    );
    assert!(
        !graph.contains("-.->|rule|"),
        "pre-walk has no provenance: {graph}"
    );

    // The same run must be byte-for-byte reproducible.
    let again = dump_graph("graph.md");
    assert_eq!(graph, &again.pre, "dump is deterministic across runs");
}

#[test]
fn post_walk_dump_shows_resolved_graph_with_provenance() {
    let dump = dump_graph("graph.md");
    let post = &dump.post;

    // The update walk ran pattern matching: sources are now real nodes...
    assert!(post.contains("([\"main.c\"])"), "resolved source: {post}");
    assert!(post.contains("([\"util.c\"])"), "resolved source: {post}");
    assert!(post.contains("([\"gen.y\"])"), "resolved source: {post}");
    // ...the derived objects are targets with recipes...
    assert!(post.contains("[\"main.o\"]"), "derived target: {post}");
    assert!(post.contains("[\"util.o\"]"), "derived target: {post}");
    // ...and every derived output carries a DerivedBy provenance edge —
    // including gen.tab.h, the multi-target rule's peer output that only the
    // also_make loop creates (never requested directly).
    assert!(post.contains("gen.tab.h"), "peer output present: {post}");
    assert_eq!(
        post.matches("-.->|rule|").count(),
        4,
        "main.o, util.o via `%.o: %.c`; gen.tab.c and its peer gen.tab.h \
         via `%.tab.c %.tab.h: %.y`: {post}"
    );
    assert!(post.contains("[[\"%.o: %.c\"]]"), "{post}");
    assert!(post.contains("[[\"%.tab.c %.tab.h: %.y\"]]"), "{post}");

    // Post-walk dump is deterministic too.
    let again = dump_graph("graph.md");
    assert_eq!(post, &again.post, "post dump is deterministic across runs");
}

#[test]
fn dump_format_follows_extension() {
    let dump = dump_graph("graph.mmd");
    assert!(
        dump.pre.starts_with("flowchart LR\n"),
        "raw mermaid: {}",
        dump.pre
    );
    assert!(
        dump.post.starts_with("flowchart LR\n"),
        "raw mermaid: {}",
        dump.post
    );

    let dot = dump_graph("graph.dot");
    assert!(
        dot.pre.starts_with("digraph deps {"),
        "graphviz: {}",
        dot.pre
    );
    assert!(dot.pre.contains("\"prog\""), "{}", dot.pre);
}

#[test]
fn dump_bazel_generates_build_file() {
    let dump = dump_graph_with_args("graph.md", &["--dump-bazel"]);
    assert!(
        dump.status.success(),
        "make run with --dump-bazel should succeed"
    );
    let build = std::fs::read_to_string(dump.workdir.join("BUILD.bazel"))
        .expect("BUILD.bazel should be generated");
    assert!(
        build.contains("Generated by `make --dump-bazel`"),
        "header should explain provenance: {build}"
    );
    assert!(
        build.contains("name = \"prog\""),
        "prog target present: {build}"
    );
    assert!(
        build.contains("outs = [\"prog\"]"),
        "prog output present: {build}"
    );
    assert!(
        build.contains("\"main.o\"") && build.contains("\"util.o\""),
        "prog deps represented as srcs: {build}"
    );
    // Phony targets (here: `outdir`, order-only prereq of `prog`) must NOT
    // appear as `srcs` labels — they have no file output and would produce
    // unresolvable Bazel labels (e.g. `//:FORCE` visibility errors).
    assert!(
        !build.contains("\"outdir\""),
        "phony dep `outdir` must not appear as a srcs label: {build}"
    );
}

#[test]
fn dump_bazel_preserves_normal_recipe_execution() {
    let workdir = tempdir();
    std::fs::write(
        workdir.join("Makefile"),
        "result: input\n\tprintf '%s\\n' built > result\n",
    )
    .expect("write Makefile");
    std::fs::write(workdir.join("input"), "source\n").expect("write prerequisite");

    let output = Command::new(RUST_MAKE)
        .args(["--no-print-directory", "-r", "--dump-bazel", "result"])
        .env_remove("MAKEFLAGS")
        .env_remove("GNUMAKEFLAGS")
        .env_remove("MAKEFILES")
        .current_dir(&workdir)
        .output()
        .expect("spawn make");

    assert!(
        output.status.success(),
        "make should execute the recipe successfully: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(workdir.join("result")).expect("recipe output"),
        "built\n"
    );
    let build = std::fs::read_to_string(workdir.join("BUILD.bazel"))
        .expect("BUILD.bazel should be generated after execution");
    assert!(build.contains("name = \"result\""), "{build}");
    assert!(build.contains("\"input\""), "{build}");
}

/// Committed visualization of the fixture's dumps. Fails when stale;
/// regenerate with `UPDATE_SNAPSHOTS=1 cargo test --test depgraph_dump`.
#[test]
fn makefile_snapshot_doc_is_current() {
    let dump = dump_graph("graph.md");
    let fixture = std::fs::read_to_string(manifest_dir().join("tests/fixtures/depgraph.mk"))
        .expect("fixture readable");

    let doc = format!(
        "# Dependency graph of a real makefile\n\
         \n\
         <!-- Generated by tests/depgraph_dump.rs (makefile_snapshot_doc_is_current). -->\n\
         <!-- Regenerate with: UPDATE_SNAPSHOTS=1 cargo test --test depgraph_dump -->\n\
         \n\
         Dumped by the real `make` binary via\n\
         `MAKERS_DEPGRAPH=graph.md MAKERS_DEPGRAPH_POST=post.md make -rn`\n\
         from this makefile (`tests/fixtures/depgraph.mk`):\n\
         \n\
         ```make\n{fixture}```\n\
         \n\
         ## Before the update walk\n\
         \n\
         What make knows after reading makefiles — plain prerequisites and the\n\
         rule database; implicit-rule matching has not run yet:\n\
         \n\
         {pre}\n\
         ## After the update walk\n\
         \n\
         The resolved graph: pattern matching derived `main.o`/`util.o` from\n\
         their sources, and each object carries a `rule` provenance edge to the\n\
         `%.o: %.c` rule that built it:\n\
         \n\
         {post}",
        pre = dump.pre,
        post = dump.post,
    );

    let snapshot = manifest_dir().join("docs/depgraph-makefile.md");
    if std::env::var_os("UPDATE_SNAPSHOTS").is_some() {
        std::fs::write(&snapshot, &doc).expect("write snapshot");
        return;
    }
    let committed = std::fs::read_to_string(&snapshot)
        .expect("docs/depgraph-makefile.md exists (UPDATE_SNAPSHOTS=1 to create)");
    assert_eq!(
        committed, doc,
        "docs/depgraph-makefile.md is stale; regenerate with UPDATE_SNAPSHOTS=1"
    );
}
