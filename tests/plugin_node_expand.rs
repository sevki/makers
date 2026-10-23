//! End-to-end: `node.expand` and `node.recipe-environment`, through
//! `plugins/expand-probe`.
//!
//! Requires the component to be built:
//! `(cd plugins/expand-probe && cargo component build)`.

#![cfg(feature = "wasmtime")]

mod plugin_common;

use plugin_common::{assert_clean, component, run_make, workdir};

fn probe(env_extra: &[(&str, &str)]) -> plugin_common::Run {
    let plugin = component("expand-probe", "expand_probe");
    let dir = workdir("plugin_node_expand.mk", &["in.c", "extra.h"]);
    let spec = format!("probe={}", plugin.display());
    let mut env: Vec<(&str, &str)> = vec![("MAKERS_PLUGINS", &spec)];
    env.extend_from_slice(env_extra);
    run_make(&dir, &env)
}

const ALLOW: (&str, &str) = ("MAKERS_PLUGIN_ALLOW", "probe:expand-variables");

/// A command built by a make function from the automatics comes out as the
/// command make would run, which neither `vars.expand` (no automatics) nor
/// substituting before the function runs can produce.
#[test]
fn expands_a_recipe_in_the_targets_scope_with_automatics_set() {
    let run = probe(&[ALLOW]);
    assert_clean(&run);
    let report = run.artifact("expand.txt");

    assert!(
        report.contains("expand\tout.o\t$(call mk,$(@F))\techo built out.o from in.c and in.c extra.h\n"),
        "`$@`, `$<` and `$^` are for out.o:\n{report}"
    );
}

/// The per-target definition is what the target sees, not the global one.
#[test]
fn a_per_target_variable_wins_over_the_global_one() {
    let run = probe(&[ALLOW]);
    assert_clean(&run);
    let report = run.artifact("expand.txt");

    assert!(
        report.contains("echo target other.o"),
        "`other.o: WHO := target` beats the global `WHO`:\n{report}"
    );
}

/// What make adds to the environment is reported; what the process already
/// had is not, so a plugin is not handed credentials it did not ask for.
#[test]
fn the_recipe_environment_is_what_make_added_and_nothing_inherited() {
    let run = probe(&[ALLOW, ("PROBE_SECRET", "hunter2")]);
    assert_clean(&run);
    let report = run.artifact("expand.txt");

    assert!(
        report.contains("env\tKBUILD_FLAG\tfrom-make\n"),
        "an exported makefile variable is in the recipe environment:\n{report}"
    );
    assert!(
        !report.contains("PROBE_SECRET") && !report.contains("hunter2"),
        "the inherited environment must not reach the plugin:\n{report}"
    );
    assert!(
        !report.contains("env\tMAKEFLAGS"),
        "make's own bookkeeping is not the build's environment:\n{report}"
    );
}

/// Both calls run the expander, which can run `$(shell ...)`, so both need
/// the grant that `vars.expand` needs and report a refusal rather than
/// answering.
#[test]
fn both_calls_need_expand_variables() {
    let run = probe(&[]);
    assert_clean(&run);
    let report = run.artifact("expand.txt");

    assert!(
        report.contains("ERROR") && report.contains("expand-variables"),
        "withheld, `node.expand` says which capability it needed:\n{report}"
    );
    assert!(
        report.contains("env\tERROR"),
        "and so does `node.recipe-environment`:\n{report}"
    );
}
