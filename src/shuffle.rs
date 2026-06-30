use crate::dep::DepNode;
use crate::file::FileId;
use std::sync::{Mutex, OnceLock};

use crate::fatal;
use crate::make_main::not_parallel;
use crate::misc::{make_rand, make_seed};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    None,
    Random,
    Reverse,
    Identity,
}

struct Config {
    mode: Mode,
    seed: u32,
}

impl Config {
    fn new() -> Self {
        Self {
            mode: Mode::None,
            seed: 0,
        }
    }
}

static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

fn config() -> std::sync::MutexGuard<'static, Config> {
    CONFIG
        .get_or_init(|| Mutex::new(Config::new()))
        .lock()
        .unwrap_or_else(|err| err.into_inner())
}

/// Returns the canonical label for the active shuffle mode (e.g. `"reverse"`,
/// or the seed as a decimal string for `random`), or `None` when shuffling is
/// disabled.
pub fn get_mode() -> Option<String> {
    let cfg = config();
    match cfg.mode {
        Mode::None => None,
        Mode::Random => Some(cfg.seed.to_string()),
        Mode::Reverse => Some("reverse".to_string()),
        Mode::Identity => Some("identity".to_string()),
    }
}

/// Configure shuffle behavior from a textual argument (typically the value of
/// the `--shuffle=` command-line flag). Aborts via `fatal` on a malformed
/// numeric seed, matching the original C behavior.
pub fn set_mode(ctx: &crate::execctx::ExecContext, arg: &str) {
    let mut cfg = config();
    if arg.eq_ignore_ascii_case("reverse") {
        cfg.mode = Mode::Reverse;
    } else if arg.eq_ignore_ascii_case("identity") {
        cfg.mode = Mode::Identity;
    } else if arg.eq_ignore_ascii_case("none") {
        cfg.mode = Mode::None;
    } else {
        let seed = if arg.eq_ignore_ascii_case("random") {
            unsafe { make_rand() }
        } else {
            match arg.parse::<u32>() {
                Ok(n) => n,
                Err(_) => fatal!(ctx, None, "invalid shuffle mode: Invalid value: '{arg}'"),
            }
        };
        cfg.mode = Mode::Random;
        cfg.seed = seed;
    }
}

fn random_shuffle<T>(slice: &mut [T]) {
    let len = slice.len();
    if len <= 1 {
        return;
    }
    for i in (1..len).rev() {
        let j = (unsafe { make_rand() } as usize) % (i + 1);
        if i != j {
            slice.swap(i, j);
        }
    }
}

fn reverse_shuffle<T>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len / 2 {
        slice.swap(i, len - 1 - i);
    }
}

fn identity_shuffle<T>(_: &mut [T]) {}

/// Reorder a `Vec<DepNode>` per the active shuffle mode. A `wait_here` marker
/// anywhere in the list disables shuffling for that list (matching the C code,
/// which leaves `->shuf` null so the original `->next` order is kept).
///
/// Unlike the C version (which preserved `->next` and recorded the reordering
/// in a separate `->shuf` link), the idiomatic updater iterates the `deps`
/// vector directly, so the reorder is applied to the vector in place — the same
/// observable build order.
fn shuffle_deps(deps: &mut [DepNode]) {
    if deps.is_empty() || deps.iter().any(|d| d.wait_here) {
        return;
    }
    match config().mode {
        Mode::None => {}
        Mode::Random => random_shuffle(deps),
        Mode::Reverse => reverse_shuffle(deps),
        Mode::Identity => identity_shuffle(deps),
    }
}

/// Recursively shuffle a file's deps and the deps of each prerequisite file,
/// guarded by `was_shuffled` so each file is processed once.
fn shuffle_file_deps_recursive(ctx: &crate::execctx::ExecContext, f: FileId) {
    let children: Vec<FileId> = {
        let Some(node) = ctx.filenodes.get(f) else {
            return;
        };
        let mut guard = node.lock().expect("file node poisoned");
        if guard.was_shuffled {
            return;
        }
        guard.was_shuffled = true;
        shuffle_deps(&mut guard.deps);
        guard.deps.iter().filter_map(|d| d.file).collect()
    };
    for child in children {
        shuffle_file_deps_recursive(ctx, child);
    }
}

/// Shuffle the deps of `file` and recursively shuffle each prerequisite's deps.
/// Safe to call when shuffling is disabled (no-op).
pub fn shuffle_deps_recursive(ctx: &crate::execctx::ExecContext, file: FileId) {
    let (mode, seed) = {
        let cfg = config();
        (cfg.mode, cfg.seed)
    };
    if mode == Mode::None || not_parallel() {
        return;
    }
    if mode == Mode::Random {
        make_seed(seed);
    }
    shuffle_file_deps_recursive(ctx, file);
}

/// Shuffle the goal list (`Vec<GoalDepNode>`) and recursively shuffle each
/// goal's target file's deps. The C entry point shuffled `goals` (a `GoalDep`
/// chain) the same way it shuffled any dep list; here the goal vector is
/// reordered in place and each goal's file is descended into.
pub fn shuffle_goals_recursive(
    ctx: &crate::execctx::ExecContext,
    goals: &mut [crate::dep::GoalDepNode],
) {
    let (mode, seed) = {
        let cfg = config();
        (cfg.mode, cfg.seed)
    };
    if mode == Mode::None || not_parallel() {
        return;
    }
    if mode == Mode::Random {
        make_seed(seed);
    }
    // A `wait_here` marker on any goal disables shuffling for the list.
    if !goals.iter().any(|g| g.dep.wait_here) {
        match mode {
            Mode::None => {}
            Mode::Random => random_shuffle(goals),
            Mode::Reverse => reverse_shuffle(goals),
            Mode::Identity => identity_shuffle(goals),
        }
    }
    let files: Vec<FileId> = goals.iter().filter_map(|g| g.dep.file).collect();
    for f in files {
        shuffle_file_deps_recursive(ctx, f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    static MODE_TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn lock_mode_tests() -> std::sync::MutexGuard<'static, ()> {
        MODE_TEST_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("mode test lock must not be poisoned")
    }

    #[test]
    fn set_mode_reverse_is_reported_by_get_mode() {
        let _guard = lock_mode_tests();
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "none");
        set_mode(&ctx, "reverse");
        assert_eq!(get_mode().as_deref(), Some("reverse"));
    }

    #[test]
    fn set_mode_identity_is_reported_by_get_mode() {
        let _guard = lock_mode_tests();
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "none");
        set_mode(&ctx, "identity");
        assert_eq!(get_mode().as_deref(), Some("identity"));
    }

    #[test]
    fn set_mode_none_clears_mode() {
        let _guard = lock_mode_tests();
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "reverse");
        set_mode(&ctx, "none");
        assert_eq!(get_mode(), None);
    }

    #[test]
    fn set_mode_numeric_seed_is_reported_by_get_mode() {
        let _guard = lock_mode_tests();
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "none");
        set_mode(&ctx, "1234");
        assert_eq!(get_mode().as_deref(), Some("1234"));
    }

    #[test]
    fn set_mode_random_produces_active_mode_label() {
        let _guard = lock_mode_tests();
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "none");
        set_mode(&ctx, "random");
        assert!(get_mode().is_some());
    }

    // --- behavioral tests for the dep/goal reordering machinery ---

    fn dep_named(name: &str) -> DepNode {
        DepNode {
            name: name.to_string(),
            file: None,
            shuf: None,
            stem: None,
            flags: crate::dep::DepFlags::empty(),
            changed: false,
            ignore_mtime: false,
            static_pattern: false,
            needs_second_expansion: false,
            ignore_automatic_vars: false,
            is_explicit: false,
            wait_here: false,
        }
    }

    fn goal_named(name: &str) -> crate::dep::GoalDepNode {
        crate::dep::GoalDepNode {
            dep: dep_named(name),
            error: 0,
            defined_in: None,
            lineno: 0,
            offset: 0,
        }
    }

    fn dep_names(deps: &[DepNode]) -> Vec<String> {
        deps.iter().map(|d| d.name.clone()).collect()
    }

    fn file_dep_names(ctx: &crate::execctx::ExecContext, f: FileId) -> Vec<String> {
        let node = ctx.filenodes.get(f).expect("file node present");
        let guard = node.lock().expect("file node poisoned");
        dep_names(&guard.deps)
    }

    /// `reverse` mode must actually reverse a dep list in place.
    #[test]
    fn shuffle_deps_reverse_reorders_in_place() {
        let _guard = lock_mode_tests();
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "reverse");
        let mut deps = vec![dep_named("a"), dep_named("b"), dep_named("c"), dep_named("d")];
        shuffle_deps(&mut deps);
        assert_eq!(dep_names(&deps), vec!["d", "c", "b", "a"]);
        set_mode(&ctx, "none");
    }

    /// A `wait_here` marker anywhere in a *non-empty* list must disable shuffling
    /// for that list (guards the `is_empty() || any(wait_here)` short-circuit).
    #[test]
    fn shuffle_deps_wait_here_marker_disables_shuffle() {
        let _guard = lock_mode_tests();
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "reverse");
        let mut b = dep_named("b");
        b.wait_here = true;
        let mut deps = vec![dep_named("a"), b, dep_named("c")];
        shuffle_deps(&mut deps);
        // Order is preserved because the wait marker disables shuffling.
        assert_eq!(dep_names(&deps), vec!["a", "b", "c"]);
        set_mode(&ctx, "none");
    }

    /// `shuffle_deps_recursive` must reorder the target's deps *and* descend into
    /// each prerequisite file, reordering its deps too.
    #[test]
    fn shuffle_deps_recursive_reorders_target_and_children() {
        let _guard = lock_mode_tests();
        crate::make_main::install_default_options_for_test();
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "reverse");

        let p = crate::file::enter_file(&ctx, b"shuf_P");
        let c1 = crate::file::enter_file(&ctx, b"shuf_C1");
        let c2 = crate::file::enter_file(&ctx, b"shuf_C2");
        {
            let node = ctx.filenodes.get(p).expect("parent present");
            let mut g = node.lock().expect("file node poisoned");
            let mut d1 = dep_named("shuf_C1");
            d1.file = Some(c1);
            let mut d2 = dep_named("shuf_C2");
            d2.file = Some(c2);
            g.deps = vec![d1, d2, dep_named("shuf_Z")];
        }
        {
            let node = ctx.filenodes.get(c1).expect("child present");
            let mut g = node.lock().expect("file node poisoned");
            g.deps = vec![dep_named("g1"), dep_named("g2")];
        }

        shuffle_deps_recursive(&ctx, p);

        assert_eq!(
            file_dep_names(&ctx, p),
            vec!["shuf_Z", "shuf_C2", "shuf_C1"],
            "parent deps must be reversed"
        );
        assert_eq!(
            file_dep_names(&ctx, c1),
            vec!["g2", "g1"],
            "child deps must be reversed via recursion"
        );
        set_mode(&ctx, "none");
    }

    /// With shuffling disabled (`Mode::None`), `shuffle_deps_recursive` must
    /// return before touching anything — in particular it must not mark the file
    /// `was_shuffled` (guards the `mode == None || not_parallel()` short-circuit).
    #[test]
    fn shuffle_deps_recursive_none_mode_is_a_noop() {
        let _guard = lock_mode_tests();
        crate::make_main::install_default_options_for_test();
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "none");

        let f = crate::file::enter_file(&ctx, b"shuf_none");
        {
            let node = ctx.filenodes.get(f).expect("file present");
            let mut g = node.lock().expect("file node poisoned");
            g.deps = vec![dep_named("x"), dep_named("y")];
        }
        shuffle_deps_recursive(&ctx, f);

        let node = ctx.filenodes.get(f).expect("file present");
        let was = node.lock().expect("file node poisoned").was_shuffled;
        assert!(!was, "None mode must not descend / mark was_shuffled");
    }

    /// `random` mode re-seeds the RNG on every entry, so two independent files
    /// carrying the same dep list shuffle to the *same* order. The seeding is
    /// what makes runs reproducible; without it the second shuffle would diverge.
    #[test]
    fn shuffle_random_reseeds_for_reproducible_order() {
        let _guard = lock_mode_tests();
        crate::make_main::install_default_options_for_test();
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "424242");

        let names = ["a", "b", "c", "d", "e", "f", "g", "h"];
        let p1 = crate::file::enter_file(&ctx, b"shuf_R1");
        let p2 = crate::file::enter_file(&ctx, b"shuf_R2");
        for &p in &[p1, p2] {
            let node = ctx.filenodes.get(p).expect("file present");
            let mut g = node.lock().expect("file node poisoned");
            g.deps = names.iter().map(|n| dep_named(n)).collect();
        }

        shuffle_deps_recursive(&ctx, p1);
        shuffle_deps_recursive(&ctx, p2);

        assert_eq!(
            file_dep_names(&ctx, p1),
            file_dep_names(&ctx, p2),
            "re-seeding must make the two shuffles reproduce the same order"
        );
        set_mode(&ctx, "none");
    }

    /// The goal-list entry point must reorder the goals (`reverse` here) when no
    /// `wait_here` marker is present.
    #[test]
    fn shuffle_goals_recursive_reverse_reorders() {
        let _guard = lock_mode_tests();
        crate::make_main::install_default_options_for_test();
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "reverse");

        let mut goals = vec![goal_named("a"), goal_named("b"), goal_named("c")];
        shuffle_goals_recursive(&ctx, &mut goals);
        let names: Vec<String> = goals.iter().map(|g| g.dep.name.clone()).collect();
        assert_eq!(names, vec!["c", "b", "a"]);
        set_mode(&ctx, "none");
    }
}
