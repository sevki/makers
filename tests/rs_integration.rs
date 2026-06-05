//! Differential tests: run each fixture through the c2rust-translated `make`
//! and the original C `make`, then compare stdout/stderr/exit-code byte-for-byte.
//!
//! The C binary is the in-tree `./make` produced by `make MAKE_CFLAGS="-Wall"`.
//! The Rust binary is what cargo just built (located via env!("CARGO_BIN_EXE_make")).

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const RUST_MAKE: &str = env!("CARGO_BIN_EXE_make");

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn c_make() -> PathBuf {
    let p = manifest_dir().join("make");
    assert!(
        p.exists(),
        "C oracle binary not found at {p:?}. Run `make MAKE_CFLAGS=\"-Wall\"` in the project root first."
    );
    p
}

fn fixtures_dir() -> PathBuf {
    manifest_dir().join("tests/fixtures")
}

#[derive(Debug)]
struct Run {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    code: Option<i32>,
}

impl From<Output> for Run {
    fn from(o: Output) -> Self {
        Run {
            stdout: o.stdout,
            stderr: o.stderr,
            code: o.status.code(),
        }
    }
}

fn run(make_bin: &Path, fixture: &Path, target: &str, extra: &[&str]) -> Run {
    // Each invocation gets a tempdir cwd so fixtures that touch files don't
    // collide between runs. We pass the fixture by absolute path.
    let workdir = tempdir();
    let out = Command::new(make_bin)
        .arg("--no-print-directory")
        .arg("-f")
        .arg(fixture)
        .args(extra)
        .arg(target)
        .current_dir(&workdir)
        .output()
        .expect("failed to spawn make");
    out.into()
}

fn tempdir() -> PathBuf {
    // std::env::temp_dir() + a unique name. We keep it simple — each test gets
    // a new dir per invocation; cleanup is best-effort.
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("rsmake-{nanos}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Normalize make output: replace the binary's own path in error messages so
/// stderr from `./make` and `target/debug/make` matches. Make prefixes errors
/// with the program's basename — both should be "make", but if argv[0] differs
/// we strip it.
fn normalize(bytes: &[u8], make_path: &Path) -> Vec<u8> {
    let s = String::from_utf8_lossy(bytes);
    let basename = make_path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    s.replace(&format!("{basename}:"), "make:")
        .into_bytes()
}

fn assert_diff(name: &str, c: &Run, r: &Run, c_bin: &Path, r_bin: &Path) {
    let cn = normalize(&c.stdout, c_bin);
    let rn = normalize(&r.stdout, r_bin);
    let cnerr = normalize(&c.stderr, c_bin);
    let rnerr = normalize(&r.stderr, r_bin);

    let mut issues = Vec::new();
    if c.code != r.code {
        issues.push(format!("exit code: C={:?} Rust={:?}", c.code, r.code));
    }
    if cn != rn {
        issues.push(format!(
            "stdout differs:\n--- C ---\n{}\n--- Rust ---\n{}",
            String::from_utf8_lossy(&cn),
            String::from_utf8_lossy(&rn)
        ));
    }
    if cnerr != rnerr {
        issues.push(format!(
            "stderr differs:\n--- C ---\n{}\n--- Rust ---\n{}",
            String::from_utf8_lossy(&cnerr),
            String::from_utf8_lossy(&rnerr)
        ));
    }
    assert!(
        issues.is_empty(),
        "[{name}] divergence between C and Rust make:\n{}",
        issues.join("\n\n")
    );
}

fn check(name: &str, fixture: &str, target: &str, extra: &[&str]) {
    let fixture = fixtures_dir().join(fixture);
    let c = c_make();
    let r = PathBuf::from(RUST_MAKE);
    let c_run = run(&c, &fixture, target, extra);
    let r_run = run(&r, &fixture, target, extra);
    assert_diff(name, &c_run, &r_run, &c, &r);
}

#[test]
fn basic() {
    check("basic", "01_basic.mk", "all", &[]);
}

#[test]
fn variable_expansion() {
    check("vars", "02_vars.mk", "all", &["FROMCMD=cli"]);
}

#[test]
fn pattern_rules() {
    check("pattern", "03_pattern.mk", "all", &[]);
}

#[test]
fn conditionals_release() {
    check("cond-release", "04_cond.mk", "all", &[]);
}

#[test]
fn conditionals_debug() {
    check("cond-debug", "04_cond.mk", "all", &["MODE=debug", "VERBOSE=1"]);
}

#[test]
fn builtin_functions() {
    check("funcs", "05_funcs.mk", "all", &[]);
}

#[test]
fn recipes_and_autovars() {
    check("recipe-all", "06_recipe.mk", "all", &[]);
}

#[test]
fn line_continuation() {
    check("recipe-lines", "06_recipe.mk", "lines", &[]);
}

#[test]
fn recipe_failure() {
    check("error", "07_error.mk", "all", &[]);
}

#[test]
fn shuffle_reverse() {
    // sm_reverse is deterministic — six prereqs become z,e,d,g,b,a.
    check("shuffle-reverse", "08_shuffle.mk", "all", &["--shuffle=reverse"]);
}

#[test]
fn shuffle_random_fixed_seed() {
    // sm_random with a fixed seed is reproducible across runs and across
    // both binaries (both link the same make_rand from misc.c).
    check(
        "shuffle-random-42",
        "08_shuffle.mk",
        "all",
        &["--shuffle=42"],
    );
}

#[test]
fn shuffle_identity() {
    // sm_identity is a no-op — order should equal the unshuffled output.
    check(
        "shuffle-identity",
        "08_shuffle.mk",
        "all",
        &["--shuffle=identity"],
    );
}

#[test]
fn warn_undefined_var() {
    check(
        "warn-undefined-var",
        "09_warn.mk",
        "all",
        &["--warn=undefined-var"],
    );
}

#[test]
fn warn_global_action() {
    check(
        "warn-error",
        "09_warn.mk",
        "all",
        &["--warn=error,undefined-var"],
    );
}

#[test]
fn jobserver_serial() {
    // -j 1 still spins up the jobserver but with one slot — exercises
    // jobserver_setup, jobserver_acquire/release without parallelism.
    check("jobs-1", "10_jobs.mk", "all", &["-j", "1"]);
}

#[test]
fn jobserver_parallel() {
    // -j 4 with four independent recipes. Output ordering can vary, so we
    // sort each goal's stdout before diffing — tweak the harness inline
    // here rather than complicating the generic check().
    let fixture = fixtures_dir().join("10_jobs.mk");
    let c = c_make();
    let r = std::path::PathBuf::from(RUST_MAKE);
    let c_out = run(&c, &fixture, "all", &["-j", "4"]);
    let r_out = run(&r, &fixture, "all", &["-j", "4"]);
    assert_eq!(c_out.code, r_out.code, "exit code mismatch");
    let mut c_lines: Vec<_> = c_out.stdout.split(|&b| b == b'\n').collect();
    let mut r_lines: Vec<_> = r_out.stdout.split(|&b| b == b'\n').collect();
    c_lines.sort();
    r_lines.sort();
    assert_eq!(c_lines, r_lines, "stdout (sorted) mismatch");
}

/// Pins a subtle, easily-misread GNU make behaviour: a static pattern rule's
/// *first* target becomes the default goal, exactly like any other explicit
/// rule (pattern rules, by contrast, never set the default goal). With
///
///     OBJS = x1.o x2.o x3.o
///     $(OBJS): %.o: %.c
///     all: $(OBJS)
///
/// a bare `make` builds only `x1.o` (default goal == x1.o), while `make all`
/// builds all three. Verified identical against GNU Make 4.3. This guards the
/// port against regressing to "static-pattern targets don't set the default
/// goal" (which would wrongly make `all` the default).
///
/// Self-contained: asserts the behaviour directly, so it needs no C oracle.
fn run_static_pattern(extra: &[&str]) -> String {
    let workdir = tempdir();
    for stem in ["x1", "x2", "x3"] {
        std::fs::write(workdir.join(format!("{stem}.c")), b"").unwrap();
    }
    let fixture = fixtures_dir().join("11_static_pattern.mk");
    let out = Command::new(RUST_MAKE)
        .arg("--no-print-directory")
        .arg("-f")
        .arg(&fixture)
        .args(extra)
        .current_dir(&workdir)
        .output()
        .expect("failed to spawn make");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn static_pattern_sets_default_goal() {
    // Bare `make`: the default goal is the first static-pattern target only.
    let default_out = run_static_pattern(&[]);
    assert!(
        default_out.contains("build x1.o"),
        "default goal should build x1.o:\n{default_out}"
    );
    assert!(
        !default_out.contains("build x2.o") && !default_out.contains("build x3.o"),
        "default goal should build ONLY x1.o (GNU make gotcha), got:\n{default_out}"
    );
}

#[test]
fn static_pattern_explicit_all_builds_every_target() {
    // `make all`: every static-pattern target is built.
    let all_out = run_static_pattern(&["all"]);
    for obj in ["x1.o", "x2.o", "x3.o"] {
        assert!(
            all_out.contains(&format!("build {obj}")),
            "`make all` should build {obj}:\n{all_out}"
        );
    }
}

#[test]
fn warn_unknown_warning_is_error() {
    // unknown warning names trigger fatal() — exit code != 0; the diff harness
    // checks that both binaries fail the same way.
    check(
        "warn-unknown",
        "09_warn.mk",
        "all",
        &["--warn=no-such-warning"],
    );
}
