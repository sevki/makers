//! End-to-end: `plugins/compile-commands` produces a JSON Compilation
//! Database from a plain makefile.
//!
//! This is the plugin the interface was redesigned for, so these tests are
//! also the interface's acceptance criteria: per-target variable
//! precedence, order-only prerequisites excluded from inputs, link steps
//! told apart from compile steps, capability gating observable from
//! outside, and an artifact that is either complete or absent.
//!
//! Requires the component to be built:
//! `(cd plugins/compile-commands && cargo component build)`.

#![cfg(feature = "wasmtime")]

mod plugin_common;
use plugin_common::{assert_clean, component, run_make, workdir, Run};

const SOURCES: &[&str] = &["main.c", "util.c", "debug.c"];

fn run(extra: &[(&str, &str)]) -> Run {
    let plugin = component("compile-commands", "compile_commands");
    let dir = workdir("plugin_api.mk", SOURCES);
    let spec = format!("compdb={}", plugin.display());
    let mut env: Vec<(&str, &str)> = vec![("MAKERS_PLUGINS", &spec)];
    env.extend_from_slice(extra);
    run_make(&dir, &env)
}

/// The database matches what make would actually run, per target — the
/// property `bear` reconstructs by intercepting `exec` and `compiledb` gets
/// by re-parsing `make -n`, both after the fact.
#[test]
fn the_database_matches_the_commands_make_would_run() {
    let run = run(&[]);
    assert_clean(&run);
    let db = run.artifact("compile_commands.json");

    for (source, command) in [
        ("main.c", "cc -Wall -c -o main.o main.c"),
        ("util.c", "cc -Wall -c -o util.o util.c"),
    ] {
        assert!(
            db.contains(source),
            "{source} should be in the database:\n{db}"
        );
        assert!(
            db.contains(command),
            "the command should match make's own recipe expansion:\n{db}"
        );
        assert!(
            run.stdout.contains(command),
            "and it should be exactly what make printed:\n{}",
            run.stdout
        );
    }
}

/// `debug.o: CFLAGS := -Wall -O0 -g` must win over the global `CFLAGS`.
///
/// Reading the global value and calling it the target's flags is the classic
/// compile-database bug, and it is why `node.variable` does a target-scoped
/// lookup (per-target, then pattern-specific, then global) rather than
/// exposing the global set only.
#[test]
fn per_target_variables_beat_the_global_value() {
    let run = run(&[]);
    assert_clean(&run);
    let db = run.artifact("compile_commands.json");
    assert!(
        db.contains("cc -Wall -O0 -g -c -o debug.o debug.c"),
        "debug.o must carry its per-target CFLAGS:\n{db}"
    );
    assert!(
        !db.contains("cc -Wall -c -o debug.o"),
        "and not the global ones:\n{db}"
    );
}

/// `prog: $(OBJS) | build` has a `.c` prerequisite by way of nothing at all
/// and a recipe, but it is a link. Telling the two apart needs the
/// prerequisite list — which is what the interface hands the plugin and what
/// an `exec`-interception tool has to guess at.
#[test]
fn link_steps_are_not_mistaken_for_compiles() {
    let run = run(&[]);
    assert_clean(&run);
    let db = run.artifact("compile_commands.json");
    assert!(
        !db.contains("\"output\": \"prog\""),
        "the link step must not appear as a translation unit:\n{db}"
    );
    assert_eq!(
        db.matches("\"file\":").count(),
        3,
        "exactly the three compiles:\n{db}"
    );
}

/// An order-only prerequisite (`| build`) is an ordering constraint, not an
/// input, and must never appear as the `file` of an entry.
#[test]
fn order_only_prerequisites_are_not_inputs() {
    let run = run(&[]);
    assert_clean(&run);
    let db = run.artifact("compile_commands.json");
    assert!(
        !db.contains("\"file\": \"build\""),
        "an order-only prerequisite is not a translation unit:\n{db}"
    );
}

/// Withholding `read-recipes` is observable from outside: the host says so
/// once, and the plugin degrades to an empty database rather than probing or
/// crashing.
#[test]
fn withholding_read_recipes_is_reported_and_degrades() {
    let run = run(&[("MAKERS_PLUGIN_DENY", "compdb:read-recipes")]);
    assert_clean(&run);
    assert!(
        run.stderr.contains("withheld capabilities: read-recipes"),
        "the host reports what it withheld:\n{}",
        run.stderr
    );
    assert_eq!(
        run.artifact("compile_commands.json").trim(),
        "[]",
        "no recipes visible means no entries"
    );
}

/// Withholding `write-outputs` fails the plugin with a message naming the
/// capability, and — because its manifest is advisory — leaves the build
/// alone. Nothing partial is written.
#[test]
fn withholding_write_outputs_fails_the_plugin_but_not_the_build() {
    let run = run(&[("MAKERS_PLUGIN_DENY", "compdb:write-outputs")]);
    assert!(
        run.status.success(),
        "the build survives an advisory plugin"
    );
    assert!(
        run.stderr
            .contains("requires the `write-outputs` capability"),
        "the denial names the capability:\n{}",
        run.stderr
    );
    assert!(
        !run.produced("compile_commands.json"),
        "and nothing partial is left behind"
    );
}

/// The output path is the plugin's declared default unless the instance
/// overrides it — the separation that lets one compiled component serve
/// several projects without being forked.
#[test]
fn the_output_path_is_configurable_per_instance() {
    let plugin = component("compile-commands", "compile_commands");
    let dir = workdir("plugin_api.mk", SOURCES);
    let spec = format!("compdb={}", plugin.display());
    let run = run_make(
        &dir,
        &[
            ("MAKERS_PLUGINS", &spec),
            ("MAKERS_PLUGIN_ARGS", "compdb.out.database=build/db.json"),
        ],
    );
    assert_clean(&run);
    assert!(
        !run.produced("compile_commands.json"),
        "the default path is not used when overridden"
    );
    assert!(
        run.artifact("build/db.json").contains("main.c"),
        "and the parent directory is created"
    );
}

/// Two runs over an unchanged makefile produce byte-identical output. A
/// compile database that churns makes every editor re-index and every diff
/// noisy. Note that this plugin does *not* claim `deterministic` in its
/// manifest — it requests `expand-variables`, and the host refuses that
/// combination — so this is the property holding on its own rather than a
/// promise the host is enforcing.
#[test]
fn output_is_byte_stable_across_runs() {
    let plugin = component("compile-commands", "compile_commands");
    let dir = workdir("plugin_api.mk", SOURCES);
    let spec = format!("compdb={}", plugin.display());
    let first = run_make(&dir, &[("MAKERS_PLUGINS", &spec)]);
    assert_clean(&first);
    let a = first.artifact("compile_commands.json");
    let second = run_make(&dir, &[("MAKERS_PLUGINS", &spec)]);
    assert_clean(&second);
    assert_eq!(a, second.artifact("compile_commands.json"));
}

/// A recipe using a makefile *function* cannot be finished by target-scoped
/// substitution: `$(addprefix -l,$(LIBS))` is not a variable lookup. Without
/// `expand-variables` the plugin skips the entry and says why, rather than
/// writing a half-expanded command line — which would make clangd report
/// phantom errors across the whole translation unit.
#[test]
fn recipes_needing_the_expander_are_skipped_when_it_is_withheld() {
    let plugin = component("compile-commands", "compile_commands");
    let dir = workdir("plugin_expand.mk", &["lib.c"]);
    let spec = format!("compdb={}", plugin.display());
    let run = run_make(&dir, &[("MAKERS_PLUGINS", &spec)]);
    assert_clean(&run);
    assert!(
        run.stderr
            .contains("withheld capabilities: expand-variables"),
        "the host reports the capability that would have helped:\n{}",
        run.stderr
    );
    assert!(
        run.stderr.contains("left unexpanded references"),
        "and the plugin says what it skipped:\n{}",
        run.stderr
    );
    assert_eq!(run.artifact("compile_commands.json").trim(), "[]");
}

/// Granted `expand-variables`, the same recipe is finished by make's own
/// expander and comes out exactly as make would run it.
///
/// This is the capability doing real work rather than merely existing: the
/// plugin cannot reimplement `$(addprefix ...)`, and the host will not hand
/// out an expander that can also run `$(shell ...)` without being asked.
#[test]
fn granting_expand_variables_completes_the_command() {
    let plugin = component("compile-commands", "compile_commands");
    let dir = workdir("plugin_expand.mk", &["lib.c"]);
    let spec = format!("compdb={}", plugin.display());
    let run = run_make(
        &dir,
        &[
            ("MAKERS_PLUGINS", &spec),
            ("MAKERS_PLUGIN_ALLOW", "compdb:expand-variables"),
        ],
    );
    assert_clean(&run);
    assert!(
        !run.stderr.contains("withheld capabilities"),
        "nothing is withheld now:\n{}",
        run.stderr
    );
    let db = run.artifact("compile_commands.json");
    let command = "cc -Wall -c -o lib.o lib.c -lm -lpthread";
    assert!(db.contains(command), "expected the fully expanded command:\n{db}");
    assert!(
        run.stdout.contains(command),
        "and it should be exactly what make printed:\n{}",
        run.stdout
    );
}
