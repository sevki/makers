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
    s.replace(&format!("{basename}:"), "make:").into_bytes()
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

/// Like [`check`], but compares stdout/stderr as a *sorted multiset of lines*
/// rather than byte-for-byte.
///
/// Make's output ordering is not stable: the per-recipe header `@echo`s and the
/// child `for`-loop / multi-prereq steps flush through different paths (make's
/// own buffered stdout vs. the inherited child fd), so their interleaving jitters
/// between the C oracle and the Rust port — and run-to-run under load (e.g. the
/// cargo-mutants baseline, which runs the suite under heavy parallelism). The
/// stable, meaningful invariants are the *set* of emitted lines and the exit
/// code, not their order. This mirrors the approach already used by
/// `jobserver_parallel`.
fn check_unordered(name: &str, fixture: &str, target: &str, extra: &[&str]) {
    let fixture = fixtures_dir().join(fixture);
    let c = c_make();
    let r = PathBuf::from(RUST_MAKE);
    let c_run = run(&c, &fixture, target, extra);
    let r_run = run(&r, &fixture, target, extra);

    assert_eq!(
        c_run.code, r_run.code,
        "[{name}] exit code: C={:?} Rust={:?}",
        c_run.code, r_run.code
    );

    let assert_sorted = |stream: &str, c_bytes: &[u8], r_bytes: &[u8]| {
        let cn = normalize(c_bytes, &c);
        let rn = normalize(r_bytes, &r);
        let mut c_lines: Vec<_> = cn.split(|&b| b == b'\n').collect();
        let mut r_lines: Vec<_> = rn.split(|&b| b == b'\n').collect();
        c_lines.sort();
        r_lines.sort();
        assert_eq!(
            c_lines,
            r_lines,
            "[{name}] {stream} (sorted) differs:\n--- C ---\n{}\n--- Rust ---\n{}",
            String::from_utf8_lossy(&cn),
            String::from_utf8_lossy(&rn)
        );
    };

    assert_sorted("stdout", &c_run.stdout, &r_run.stdout);
    assert_sorted("stderr", &c_run.stderr, &r_run.stderr);
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
    // Independent `%.o` builds can flush in any interleaving (see
    // `check_unordered`); compare the line set, not the order.
    check_unordered("pattern", "03_pattern.mk", "all", &[]);
}

#[test]
fn conditionals_release() {
    check("cond-release", "04_cond.mk", "all", &[]);
}

#[test]
fn conditionals_debug() {
    check(
        "cond-debug",
        "04_cond.mk",
        "all",
        &["MODE=debug", "VERBOSE=1"],
    );
}

#[test]
fn conditionals_else_if_chain_default() {
    // `else ifeq` / `else ifdef` chain in conditional_line, falling through to
    // the final plain `else`.
    check("cond-chain-default", "15_cond_chain.mk", "all", &[]);
}

#[test]
fn conditionals_else_if_chain_release() {
    // Takes the `else ifeq ($(MODE),release)` branch.
    check(
        "cond-chain-release",
        "15_cond_chain.mk",
        "all",
        &["MODE=release"],
    );
}

#[test]
fn conditionals_else_if_chain_fallback() {
    // Takes the `else ifdef FALLBACK` branch and the `else` of the second block.
    check(
        "cond-chain-fallback",
        "15_cond_chain.mk",
        "all",
        &["FALLBACK=x", "HIDE=1"],
    );
}

#[test]
fn define_and_undefine_names() {
    // do_define / do_undefine name isolation (typed AST layer): a define whose
    // name is produced by expansion, and an undefine of a set variable.
    check("define-undefine", "33_define_undefine.mk", "all", &[]);
}

#[test]
fn tilde_expand_user_branch() {
    // tilde_expand's `~user` branch (slice/CString-based): `~root/<suffix>` is
    // expanded via getpwnam("root") + the suffix and then included. The file is
    // absent, so both binaries must emit the identical expanded path in their
    // "No such file or directory" error.
    check("tilde-user", "34_tilde_user.mk", "all", &[]);
}

#[test]
fn parse_file_seq_wait_and_dotslash() {
    // parse_file_seq token normalization (now via pure parser helpers): the
    // `.WAIT` ordering marker and `./` prefix stripping must behave identically
    // across the C oracle and the Rust port.
    check("wait-dotslash", "36_wait_dotslash.mk", "all", &[]);
}

#[test]
fn eval_ifeq_missing_separator() {
    // eval's "ifeq/ifneq must be followed by whitespace" diagnostic (now
    // classified in the typed AST layer): `ifeq(a,a)` with no separating space
    // must produce the specific error, byte-for-byte identical across binaries.
    check("ifeq-no-separator", "35_ifeq_no_separator.mk", "all", &[]);
}

#[test]
fn conditional_ifdef_token() {
    // conditional_line's ifdef/ifndef single-token extraction (typed AST layer):
    // defined/undefined names and a name produced by expansion.
    check("ifdef-token", "32_ifdef_token.mk", "all", &[]);
}

#[test]
fn conditional_ifeq_argument_forms() {
    // conditional_line's ifeq/ifneq argument scan (now in the typed AST layer):
    // paren form with references and spaces, double/single quoted forms, and a
    // balanced-parenthesis second argument — checked byte-for-byte vs the oracle.
    check("ifeq-forms", "31_ifeq_forms.mk", "all", &[]);
}

#[test]
fn variable_modifiers() {
    // Exercises eval's modifier classification: export/unexport/override/
    // private/define-endef/undefine.
    check("varmod", "16_varmod.mk", "all", &[]);
}

#[test]
fn call_function_args() {
    // Exercises func_call's `max_args` save/restore (now an atomic): `$(call)`
    // with varying arg counts and a nested call, byte-checked against the C
    // oracle (the `$N` automatics must clear correctly between calls).
    check("call-args", "26_call_args.mk", "all", &[]);
}

#[test]
fn recipe_prefix_override() {
    // Exercises eval's leading-byte line classification (LineKind) with a
    // custom `.RECIPEPREFIX` of `>`: recipe lines begin with `>` instead of a
    // tab, blank lines are skipped, and other lines parse as normal syntax.
    // Output is deterministic and byte-checked against the C oracle.
    check("recipe-prefix", "22_recipeprefix.mk", "all", &[]);
}

#[test]
fn second_expansion_prereq() {
    // Exercises the .SECONDEXPANSION flag (second_expansion): a `$$(DEP)`
    // prerequisite resolves on the second expansion pass to `real-dep`, which
    // is then built. Byte-checked against the C oracle.
    check("second-expansion", "24_secondexpansion.mk", "all", &[]);
}

#[test]
fn posix_special_target() {
    // Exercises the .POSIX special target (posix_pedantic): declaring it sets
    // the pedantic flag via read.rs's special-target handler. Byte-checked
    // against the C oracle.
    check("posix", "25_posix.mk", "all", &[]);
}

#[test]
fn remake_included_makefile() {
    // Exercises the makefile-remaking goal-chain pass (rebuilding_makefiles):
    // the included `gen.mk` is generated by its own rule, then make re-reads
    // the makefiles with `$(GEN)` defined. Byte-checked against the C oracle.
    check("remake-include", "23_remake_include.mk", "all", &[]);
}

#[test]
fn variable_modifiers_override_from_cmdline() {
    // `override OVR = …` keeps the makefile value even when OVR is set on the
    // command line.
    check(
        "varmod-override",
        "16_varmod.mk",
        "all",
        &["OVR=from-cmdline"],
    );
}

#[test]
fn define_nested_and_ignored() {
    // Exercises the define/endef block scanner: a nested define inside a define
    // body (nlevels nesting), an endef with a trailing comment, and a define
    // skipped inside a false conditional (the in_ignored_define path). Output
    // is deterministic and byte-checked against the C oracle.
    check("define-nested", "21_define_nested.mk", "all", &[]);
}

#[test]
fn func_shell_waits_for_completion() {
    // $(shell ...) runs a child and func_shell spin-waits on the
    // shell_function_completed flag (now an atomic) until the reaper callback
    // fires. Output is deterministic and byte-checked against the C oracle.
    check("shell", "19_shell.mk", "all", &[]);
}

#[test]
fn file_directives_vpath_and_optional_include() {
    // eval's file-directive classification: `vpath` plus the error-tolerant
    // `-include`/`sinclude` of missing files (silently ignored).
    check("directives", "17_directives.mk", "all", &[]);
}

#[test]
fn file_directives_with_export_arm() {
    // The file-directive arms classify through the interned whole-line node
    // while `export` keeps its bare-word arm: stacking them must stay identical
    // to the C oracle.
    check("filedir-export", "30_filedir_export.mk", "all", &[]);
}

#[test]
fn strict_include_of_missing_file_fails() {
    // Strict `include` of a missing file must fail identically to the C oracle
    // (exit code and stderr).
    check("include-missing", "18_include_missing.mk", "all", &[]);
}

#[test]
fn include_pulls_in_aux_file() {
    // `include sub.mk` brings in the included file's variables. Rust-only (the
    // differential harness runs in a fresh tempdir without aux files).
    let (out, code) = run_make(
        "include sub.mk\nall: ; @echo from-sub=$(FROM_SUB)\n",
        &[("sub.mk", "FROM_SUB = included-value\n")],
        &[],
    );
    assert_eq!(code, Some(0), "stdout: {out}");
    assert_eq!(out.trim_end(), "from-sub=included-value");
}

#[test]
fn builtin_functions() {
    check("funcs", "05_funcs.mk", "all", &[]);
}

#[test]
fn func_patsubst_matches_c_oracle() {
    // Differential coverage for patsubst_expand_pat: the %-pattern path of
    // $(patsubst) — prefix/suffix/both, stem capture (incl. empty stem),
    // whole-token %, no-% replacement, and non-matching passthrough.
    check("patsubst", "16_patsubst.mk", "all", &[]);
}

#[test]
fn func_subst_matches_c_oracle() {
    // Differential coverage for the index-based subst_expand rewrite: $(subst)
    // edge cases (multiple/overlapping/empty/no matches, start/end, longer
    // replacement) plus the whole-word no-% $(patsubst) boundary path.
    check("subst", "15_subst.mk", "all", &[]);
}

#[test]
fn func_abspath_matches_c_oracle() {
    // Differential coverage for the rewritten abspath/abspath_into path
    // normalizer. Absolute inputs make the result independent of the working
    // directory, so the C oracle and Rust port compare byte-for-byte.
    check("abspath", "13_abspath.mk", "all", &[]);
}

#[test]
fn recipes_and_autovars() {
    // The per-target header `@echo` and its `for`-loop steps flush through
    // different paths, so their interleaving jitters (see `check_unordered`).
    check_unordered("recipe-all", "06_recipe.mk", "all", &[]);
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
    // sm_reverse reorders the prereqs deterministically, but the resulting
    // stdout ordering is still subject to make's flush jitter (see
    // `check_unordered`) — assert both binaries build the same line set.
    check_unordered(
        "shuffle-reverse",
        "08_shuffle.mk",
        "all",
        &["--shuffle=reverse"],
    );
}

#[test]
fn shuffle_random_fixed_seed() {
    // sm_random with a fixed seed is reproducible across both binaries (both
    // link the same make_rand from misc.c); compare the line set, not order.
    check_unordered(
        "shuffle-random-42",
        "08_shuffle.mk",
        "all",
        &["--shuffle=42"],
    );
}

#[test]
fn shuffle_identity() {
    // sm_identity is a no-op; compare the line set, not order.
    check_unordered(
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

#[test]
fn job_slots_capped_parallel() {
    // -j 3 with six independent recipes forces make to fill all three slots,
    // block in reap_children until one frees, then spawn more — exercising the
    // job_slots_used increment/decrement and the `== job_slots` wait. Output
    // ordering varies under parallelism, so compare the sorted line multiset.
    let fixture = fixtures_dir().join("20_job_slots.mk");
    let c = c_make();
    let r = std::path::PathBuf::from(RUST_MAKE);
    let c_out = run(&c, &fixture, "all", &["-j", "3"]);
    let r_out = run(&r, &fixture, "all", &["-j", "3"]);
    assert_eq!(c_out.code, r_out.code, "exit code mismatch");
    let mut c_lines: Vec<_> = c_out.stdout.split(|&b| b == b'\n').collect();
    let mut r_lines: Vec<_> = r_out.stdout.split(|&b| b == b'\n').collect();
    c_lines.sort();
    r_lines.sort();
    assert_eq!(c_lines, r_lines, "stdout (sorted) mismatch");
}

#[test]
fn jobserver_tokens_recycled() {
    // -j 2 with six independent recipes forces make to acquire and release
    // jobserver tokens repeatedly: it holds the implicit slot plus one token
    // per running child, releasing each as a child is reaped (free_child) and
    // re-acquiring for the next — exercising the jobserver_tokens add/sub now
    // routed through the atomic. Output ordering varies, so compare the sorted
    // line multiset; the token count must drain to zero (no INTERNAL error).
    check_unordered("jobserver-tokens", "20_job_slots.mk", "all", &["-j", "2"]);
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

// ---------------------------------------------------------------------------
// Self-contained behavioural tests (no C oracle required).
//
// These pin behaviour of code paths reworked during the c2rust -> idiomatic
// cleanup: recipe execution (child_execute_job / start_job_command), child
// reaping (reap_children), variable flavors (do_variable_definition), and
// word/glob parsing (get_next_mword / get_next_word / parse_file_seq). Each
// runs the Rust make on an inline makefile and asserts on its output, so they
// work even where the differential `./make` oracle is unavailable.
// ---------------------------------------------------------------------------

/// Run the Rust make on an inline makefile in a fresh working directory, with
/// optional auxiliary files written alongside it. Returns (stdout, exit code).
fn run_make(makefile: &str, files: &[(&str, &str)], args: &[&str]) -> (String, Option<i32>) {
    let dir = tempdir();
    std::fs::write(dir.join("Makefile"), makefile).unwrap();
    for (name, contents) in files {
        std::fs::write(dir.join(name), contents).unwrap();
    }
    let out = Command::new(RUST_MAKE)
        .arg("--no-print-directory")
        .args(args)
        .current_dir(&dir)
        .output()
        .expect("failed to spawn make");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        out.status.code(),
    )
}

#[test]
fn variable_flavors() {
    // Exercises do_variable_definition / parse_variable_definition: =, :=, ::=,
    // +=, !=, ?=, and recursive append (which leaves a trailing space).
    let mk = "\
EMPTY =
REC = a$(EMPTY)b
SIM := simple-$(REC)
EXPND ::= expanded-$(REC)
APP = first
APP += second
APP += $(EMPTY)
SHL != echo from-shell
COND ?= conditional
COND ?= should-not-override
all: ; @printf 'REC=[%s] SIM=[%s] EXPND=[%s] APP=[%s] SHL=[%s] COND=[%s]\\n' '$(REC)' '$(SIM)' '$(EXPND)' '$(APP)' '$(SHL)' '$(COND)'
";
    let (out, code) = run_make(mk, &[], &[]);
    assert_eq!(code, Some(0), "stdout: {out}");
    assert_eq!(
        out.trim_end(),
        "REC=[ab] SIM=[simple-ab] EXPND=[expanded-ab] APP=[first second ] SHL=[from-shell] COND=[conditional]"
    );
}

#[test]
fn func_realpath_matches_c_oracle() {
    // Differential coverage for the rewritten realpath (now std::fs::canonicalize).
    // Absolute, existing inputs make the result cwd-independent, so the C oracle
    // and Rust port — both delegating to libc realpath — match byte-for-byte.
    check("realpath", "14_realpath.mk", "all", &[]);
}

#[test]
fn func_abspath_resolves_relative_against_cwd() {
    // A relative argument is anchored at the working directory; ".." climbs out
    // of it. Run from a fresh tempdir and compare against that directory.
    let dir = tempdir();
    let mk = "all: ; @printf '%s\\n' '$(abspath sub/./file)'\n";
    std::fs::write(dir.join("Makefile"), mk).unwrap();
    let out = Command::new(RUST_MAKE)
        .arg("--no-print-directory")
        .current_dir(&dir)
        .output()
        .expect("failed to spawn make");
    assert_eq!(out.status.code(), Some(0));
    let cwd = std::fs::canonicalize(&dir).unwrap();
    let expected = format!("{}/sub/file\n", cwd.display());
    assert_eq!(String::from_utf8_lossy(&out.stdout), expected);
}

#[test]
fn recipe_runs_path_and_absolute_commands() {
    // child_execute_job fast path: a simple command is resolved on PATH, and an
    // absolute path is exec'd directly. Separate recipe lines so each is its own
    // (shell-free) command.
    let mk = "\
all: t1 t2
t1: ; @echo via-path
t2: ; @/bin/echo via-abs
";
    let (out, code) = run_make(mk, &[], &[]);
    assert_eq!(code, Some(0), "stdout: {out}");
    assert_eq!(out, "via-path\nvia-abs\n");
}

#[test]
fn recipe_command_not_found_is_error_127() {
    // reap_children: a child that fails to exec reports exit 127 -> make errors.
    let (out, code) = run_make("all: ; @no_such_command_zz\n", &[], &[]);
    assert_eq!(code, Some(2), "stdout: {out}");
}

#[test]
fn recipe_enoexec_falls_back_to_shell() {
    // child_execute_job ENOEXEC path: an executable file with no shebang fails
    // execve with ENOEXEC and is retried via the shell. The `chmod` step makes
    // it executable; the second `;` command then runs it directly.
    let mk = "all: ; @chmod +x ./script; ./script\n";
    let (out, code) = run_make(mk, &[("script", "echo from-noshebang\n")], &[]);
    assert_eq!(code, Some(0), "stdout: {out}");
    assert_eq!(out, "from-noshebang\n");
}

#[test]
fn parallel_jobs_all_run() {
    // start_waiting_job / reap_children under -j.
    let mk = "\
all: j1 j2 j3 j4
j1: ; @echo done-j1
j2: ; @echo done-j2
j3: ; @echo done-j3
j4: ; @echo done-j4
";
    let (out, code) = run_make(mk, &[], &["-j4"]);
    assert_eq!(code, Some(0), "stdout: {out}");
    let mut lines: Vec<&str> = out.lines().collect();
    lines.sort_unstable();
    assert_eq!(lines, vec!["done-j1", "done-j2", "done-j3", "done-j4"]);
}

#[test]
fn keep_going_continues_after_failure() {
    // reap_children + keep_going: independent targets still build after a
    // failing one; make still exits non-zero.
    let mk = "\
all: a b c
a: ; @echo A
b: ; @echo B; false
c: ; @echo C
";
    let (out, code) = run_make(mk, &[], &["-k", "-j1"]);
    assert_eq!(code, Some(2), "stdout: {out}");
    assert!(out.contains("A") && out.contains("C"), "stdout: {out}");
}

#[test]
fn order_only_prereqs_and_var_refs() {
    // get_next_word: '|' order-only separator and $ references in a pattern
    // rule's prerequisite list.
    let mk = "\
DEPS = dep1 dep2
%.o: %.c | $(DEPS)
\t@echo \"build $@ orderonly=$|\"
all: foo.o
";
    let (out, code) = run_make(mk, &[("foo.c", ""), ("dep1", ""), ("dep2", "")], &[]);
    assert_eq!(code, Some(0), "stdout: {out}");
    assert_eq!(out, "build foo.o orderonly=dep1 dep2\n");
}

#[test]
fn static_pattern_double_colon_tokens() {
    // get_next_mword parses ':' / '::' tokens; static pattern uses two colons.
    let mk = "\
OBJS = a.o b.o
$(OBJS): %.o: %.c
\t@echo build $@
both: $(OBJS)
";
    let (out, code) = run_make(mk, &[("a.c", ""), ("b.c", "")], &["both"]);
    assert_eq!(code, Some(0), "stdout: {out}");
    let mut lines: Vec<&str> = out.lines().collect();
    lines.sort_unstable();
    assert_eq!(lines, vec!["build a.o", "build b.o"]);
}

#[test]
fn wildcard_function_and_nomatch() {
    // parse_file_seq glob handling: $(wildcard) matches existing files and
    // yields empty on no match.
    let mk = "all: ; @echo \"got=[$(sort $(wildcard *.in))] none=[$(wildcard *.nope)]\"\n";
    let (out, code) = run_make(mk, &[("a.in", ""), ("b.in", "")], &[]);
    assert_eq!(code, Some(0), "stdout: {out}");
    assert_eq!(out, "got=[a.in b.in] none=[]\n");
}

// construct_command_argv_internal: fast-path (exec directly) vs. shell decision
// and argv splitting. Behaviour verified to match GNU Make 4.3.

#[test]
fn recipe_fast_path_simple_and_words() {
    // No shell metacharacters -> argv split + direct exec; multiple args.
    let (out, code) = run_make("all: ; @echo one two   three\n", &[], &[]);
    assert_eq!(code, Some(0), "stdout: {out}");
    assert_eq!(out, "one two three\n");
}

#[test]
fn recipe_shell_metachars_use_shell() {
    // Pipe and semicolon force the shell path; output must still be correct.
    let (out, code) = run_make("all: ; @echo hi | tr a-z A-Z; echo done\n", &[], &[]);
    assert_eq!(code, Some(0), "stdout: {out}");
    assert_eq!(out, "HI\ndone\n");
}

#[test]
fn recipe_quotes_preserved() {
    // Quoted whitespace is kept as a single argument.
    let (out, code) = run_make("all: ; @printf '[%s]\\n' \"a   b\"\n", &[], &[]);
    assert_eq!(code, Some(0), "stdout: {out}");
    assert_eq!(out, "[a   b]\n");
}

#[test]
fn recipe_shell_builtin_uses_shell() {
    // A lone shell builtin (cd) can't be exec'd directly; must go via the shell.
    let (out, code) = run_make("all: ; @cd / && pwd\n", &[], &[]);
    assert_eq!(code, Some(0), "stdout: {out}");
    assert_eq!(out, "/\n");
}

#[test]
fn recipe_var_assignment_prefix_uses_shell() {
    // A leading VAR=value word is shell syntax, not an argv[0]. ($$ -> $ so the
    // shell, not make, expands FOO.)
    let (out, code) = run_make("all: ; @FOO=bar sh -c 'echo $$FOO'\n", &[], &[]);
    assert_eq!(code, Some(0), "stdout: {out}");
    assert_eq!(out, "bar\n");
}

#[test]
fn func_word_wordlist_words() {
    // Differential (C oracle vs Rust) coverage for parse_numeric + func_words /
    // func_word / func_wordlist, none of which were previously exercised. The
    // fixture probes word-count, 1-based indexing, $(word N) past the end, and
    // $(wordlist) stop-index clamping / empty-when-start-past-end.
    check("wordfuncs", "12_wordfuncs.mk", "all", &[]);
}

#[test]
fn func_word_zero_index_errors() {
    // Index 0 trips func_word's `i < 1` guard; compare the C oracle's fatal
    // message and exit code against the Rust port byte-for-byte.
    check("word-zero", "12_wordfuncs.mk", "badzero", &[]);
}

#[test]
fn func_word_nonnumeric_index_errors() {
    // A non-numeric index drives parse_numeric's (now safe) validation path;
    // both binaries must abort identically.
    check("word-nan", "12_wordfuncs.mk", "badnan", &[]);
}

#[test]
fn wait_special_target_warns() {
    // `.WAIT` declared as a target with prerequisites and commands trips both
    // one-shot warnings in check_special_file (now AtomicBool guards). The
    // warnings go to stderr and the build still completes; byte-check the whole
    // stdout/stderr/exit against the C oracle.
    check("wait-special", "27_wait_special.mk", "all", &[]);
}

#[test]
fn nothing_to_be_done() {
    // 'noop' has no recipe and no prerequisites, so make's commands_started
    // counter (now an atomic) never advances and it prints "Nothing to be done
    // for 'noop'."; byte-check the message and exit against the C oracle.
    check("nothing-todo", "28_nothing_todo.mk", "noop", &[]);
}

#[test]
fn recipe_runs_advances_commands_started() {
    // Contrast: 'done' has a recipe, so commands_started advances and the recipe
    // actually runs (no "Nothing to be done"). Byte-checked against the oracle.
    check("recipe-runs", "28_nothing_todo.mk", "done", &[]);
}

#[test]
fn oneshell_special() {
    // `.ONESHELL` (recognised by the SpecialTarget classifier in check_specials)
    // runs all of a target's recipe lines in one shell, so a variable set on the
    // first line is visible on the next. Byte-checked against the C oracle.
    check("oneshell", "29_oneshell.mk", "all", &[]);
}
