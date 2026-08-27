use crate::{dep::DepNode, file::FileId};

use crate::{entry::not_parallel, fatal};

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum Mode {
    #[default]
    None,
    Random,
    Reverse,
    Identity,
}

/// `--shuffle` mode/seed plus the xorshift PRNG state it drives, the former
/// shuffle.rs `static CONFIG: OnceLock<Mutex<Config>>` and misc.rs
/// `static MK_STATE: AtomicU32`. Genuinely per-run configuration (each build
/// may pass a different `--shuffle=` value), so it lives on `ExecContext`
/// (see [`crate::execctx::ExecContext::shuffle`]) instead of a process-wide
/// singleton shared across sessions.
#[derive(Copy, Clone, Default, Debug)]
pub struct ShuffleState {
    mode: Mode,
    seed: u32,
    prng: u32,
}

fn config(ctx: &crate::execctx::ExecContext) -> ShuffleState {
    ctx.shuffle.get()
}

/// Returns the canonical label for the active shuffle mode (e.g. `"reverse"`,
/// or the seed as a decimal string for `random`), or `None` when shuffling is
/// disabled.
pub fn get_mode(ctx: &crate::execctx::ExecContext) -> Option<String> {
    let cfg = config(ctx);
    match cfg.mode {
        Mode::None => None,
        Mode::Random => Some(cfg.seed.to_string()),
        Mode::Reverse => Some("reverse".to_string()),
        Mode::Identity => Some("identity".to_string()),
    }
}

/// Whether this run's shuffling will actually reorder the graph.
///
/// Distinct from [`get_mode`], which reports what was *configured*:
/// `--shuffle=identity` is a mode but not a reorder, and any mode is a no-op
/// under `.NOTPARALLEL`. Callers that read dep or goal order out of the
/// graph after [`shuffle_goals_recursive`] has run — the `makers:plugin`
/// analysis pass is the one that does — need the narrower question, because
/// this port applies the reordering to `FileNode::deps` in place. The C
/// implementation kept the original `->next` chain alongside a separate
/// `->shuf` link, so there the makefile order remained recoverable; here it
/// does not survive.
pub fn reorders_the_graph(ctx: &crate::execctx::ExecContext) -> bool {
    matches!(config(ctx).mode, Mode::Random | Mode::Reverse) && !not_parallel(ctx)
}

/// Configure shuffle behavior from a textual argument (typically the value of
/// the `--shuffle=` command-line flag). Aborts via `fatal` on a malformed
/// numeric seed, matching the original C behavior.
pub fn set_mode(ctx: &crate::execctx::ExecContext, arg: &str) {
    let mut cfg = ctx.shuffle.get();
    if arg.eq_ignore_ascii_case("reverse") {
        cfg.mode = Mode::Reverse;
    } else if arg.eq_ignore_ascii_case("identity") {
        cfg.mode = Mode::Identity;
    } else if arg.eq_ignore_ascii_case("none") {
        cfg.mode = Mode::None;
    } else {
        let seed = if arg.eq_ignore_ascii_case("random") {
            let s = make_rand(ctx);
            // make_rand just wrote the advanced PRNG state to ctx.shuffle;
            // re-fetch so the `cfg` snapshot taken above doesn't clobber it
            // with the pre-advance value when set below.
            cfg = ctx.shuffle.get();
            s
        } else {
            match arg.parse::<u32>() {
                Ok(n) => n,
                Err(_) => fatal!(ctx, None, "invalid shuffle mode: Invalid value: '{arg}'"),
            }
        };
        cfg.mode = Mode::Random;
        cfg.seed = seed;
    }
    ctx.shuffle.set(cfg);
}

/// Seed the xorshift PRNG used by `--shuffle`.
fn make_seed(ctx: &crate::execctx::ExecContext, seed: u32) {
    let mut cfg = ctx.shuffle.get();
    cfg.prng = seed;
    ctx.shuffle.set(cfg);
}

/// Combine a timestamp and PID into an initial PRNG seed. Pulled out of
/// `make_rand` so the XOR mixing can be pinned by a test with fixed inputs
/// (the real call site reads the live clock/PID, which can't be pinned).
fn initial_seed(time: libc::time_t, pid: libc::time_t) -> u32 {
    ((time ^ pid) as u32).wrapping_add(1)
}

/// Return the next value from the xorshift PRNG, self-seeding from the time
/// and PID on first use.
fn make_rand(ctx: &crate::execctx::ExecContext) -> u32 {
    let mut cfg = ctx.shuffle.get();
    let mut next = if cfg.prng == 0 {
        unsafe {
            initial_seed(
                libc::time(::core::ptr::null_mut()),
                crate::misc::make_pid() as libc::time_t,
            )
        }
    } else {
        cfg.prng
    };
    next ^= next << 13;
    next ^= next >> 17;
    next ^= next << 5;
    cfg.prng = next;
    ctx.shuffle.set(cfg);
    next
}

fn random_shuffle<T>(ctx: &crate::execctx::ExecContext, slice: &mut [T]) {
    let len = slice.len();
    if len <= 1 {
        return;
    }
    for i in (1..len).rev() {
        let j = (make_rand(ctx) as usize) % (i + 1);
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
fn shuffle_deps(ctx: &crate::execctx::ExecContext, deps: &mut [DepNode]) {
    if deps.is_empty() || deps.iter().any(|d| d.wait_here) {
        return;
    }
    match config(ctx).mode {
        Mode::None => {}
        Mode::Random => random_shuffle(ctx, deps),
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
        shuffle_deps(ctx, &mut guard.deps);
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
        let cfg = config(ctx);
        (cfg.mode, cfg.seed)
    };
    if mode == Mode::None || not_parallel(ctx) {
        return;
    }
    if mode == Mode::Random {
        make_seed(ctx, seed);
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
        let cfg = config(ctx);
        (cfg.mode, cfg.seed)
    };
    if mode == Mode::None || not_parallel(ctx) {
        return;
    }
    if mode == Mode::Random {
        make_seed(ctx, seed);
    }
    // A `wait_here` marker on any goal disables shuffling for the list.
    if !goals.iter().any(|g| g.dep.wait_here) {
        match mode {
            Mode::None => {}
            Mode::Random => random_shuffle(ctx, goals),
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

    #[test]
    fn set_mode_reverse_is_reported_by_get_mode() {
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "none");
        set_mode(&ctx, "reverse");
        assert_eq!(get_mode(&ctx).as_deref(), Some("reverse"));
    }

    #[test]
    fn set_mode_identity_is_reported_by_get_mode() {
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "none");
        set_mode(&ctx, "identity");
        assert_eq!(get_mode(&ctx).as_deref(), Some("identity"));
    }

    #[test]
    fn set_mode_none_clears_mode() {
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "reverse");
        set_mode(&ctx, "none");
        assert_eq!(get_mode(&ctx), None);
    }

    #[test]
    fn set_mode_numeric_seed_is_reported_by_get_mode() {
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "none");
        set_mode(&ctx, "1234");
        assert_eq!(get_mode(&ctx).as_deref(), Some("1234"));
    }

    #[test]
    fn set_mode_random_produces_active_mode_label() {
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "none");
        set_mode(&ctx, "random");
        assert!(get_mode(&ctx).is_some());
    }

    /// Regression test for a lost-update bug: `set_mode`'s `cfg` snapshot was
    /// taken before `make_rand` advanced the PRNG, so the final `set` clobbered
    /// that advance and every `--shuffle=random` call reused the same seed.
    #[test]
    fn set_mode_random_advances_prng_across_calls() {
        let ctx = crate::execctx::ExecContext::default();
        let mut cfg = ctx.shuffle.get();
        cfg.prng = 12345;
        ctx.shuffle.set(cfg);

        set_mode(&ctx, "random");
        let first_seed = ctx.shuffle.get().seed;
        set_mode(&ctx, "random");
        let second_seed = ctx.shuffle.get().seed;

        assert_ne!(
            first_seed, second_seed,
            "each --shuffle=random call must advance the PRNG, not reuse the same seed"
        );
    }

    /// Pins the xorshift math to known values from a known seed so mutating
    /// any operator in `make_rand` (the shifts, the XORs, or the self-seed
    /// check) is caught, not just masked by reproducibility-only assertions.
    #[test]
    fn make_rand_applies_xorshift_from_known_seed() {
        let ctx = crate::execctx::ExecContext::default();
        let mut cfg = ctx.shuffle.get();
        cfg.prng = 12345;
        ctx.shuffle.set(cfg);

        assert_eq!(make_rand(&ctx), 3336926330);
        assert_eq!(make_rand(&ctx), 1697253807);
        assert_eq!(make_rand(&ctx), 2816511904);
    }

    /// Pins the exact XOR mixing of time/PID into the self-seed, so
    /// replacing `^` with `|` or `&` (which still yields *some* seed, just
    /// the wrong one) is caught.
    #[test]
    fn initial_seed_mixes_time_and_pid_with_xor() {
        assert_eq!(initial_seed(0x1234_5678, 0x0000_00ff), 0x1234_5688);
    }

    /// Pins `random_shuffle`'s exact permutation for a known seed, so
    /// replacing it with a no-op (or corrupting the `i + 1` modulus) is
    /// caught even though it doesn't change the *set* of elements.
    #[test]
    fn random_shuffle_produces_expected_permutation_for_known_seed() {
        let ctx = crate::execctx::ExecContext::default();
        let mut cfg = ctx.shuffle.get();
        cfg.prng = 999;
        ctx.shuffle.set(cfg);

        let mut items = vec!["a", "b", "c", "d", "e"];
        random_shuffle(&ctx, &mut items);
        assert_eq!(items, vec!["e", "b", "d", "c", "a"]);
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
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "reverse");
        let mut deps = vec![
            dep_named("a"),
            dep_named("b"),
            dep_named("c"),
            dep_named("d"),
        ];
        shuffle_deps(&ctx, &mut deps);
        assert_eq!(dep_names(&deps), vec!["d", "c", "b", "a"]);
        set_mode(&ctx, "none");
    }

    /// A `wait_here` marker anywhere in a *non-empty* list must disable shuffling
    /// for that list (guards the `is_empty() || any(wait_here)` short-circuit).
    #[test]
    fn shuffle_deps_wait_here_marker_disables_shuffle() {
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "reverse");
        let mut b = dep_named("b");
        b.wait_here = true;
        let mut deps = vec![dep_named("a"), b, dep_named("c")];
        shuffle_deps(&ctx, &mut deps);
        // Order is preserved because the wait marker disables shuffling.
        assert_eq!(dep_names(&deps), vec!["a", "b", "c"]);
        set_mode(&ctx, "none");
    }

    /// `shuffle_deps_recursive` must reorder the target's deps *and* descend into
    /// each prerequisite file, reordering its deps too.
    #[test]
    fn shuffle_deps_recursive_reorders_target_and_children() {
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
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "reverse");

        let mut goals = vec![goal_named("a"), goal_named("b"), goal_named("c")];
        shuffle_goals_recursive(&ctx, &mut goals);
        let names: Vec<String> = goals.iter().map(|g| g.dep.name.clone()).collect();
        assert_eq!(names, vec!["c", "b", "a"]);
        set_mode(&ctx, "none");
    }

    /// `random` mode must re-seed on every `shuffle_goals_recursive` entry
    /// (mirroring `shuffle_random_reseeds_for_reproducible_order` for the
    /// goal-list entry point), so two independent goal lists with the same
    /// contents shuffle to the same order.
    #[test]
    fn shuffle_goals_recursive_random_reseeds_for_reproducible_order() {
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "424242");

        let names = ["a", "b", "c", "d", "e", "f", "g", "h"];
        let mut goals1: Vec<_> = names.iter().map(|n| goal_named(n)).collect();
        let mut goals2: Vec<_> = names.iter().map(|n| goal_named(n)).collect();

        shuffle_goals_recursive(&ctx, &mut goals1);
        shuffle_goals_recursive(&ctx, &mut goals2);

        let order1: Vec<String> = goals1.iter().map(|g| g.dep.name.clone()).collect();
        let order2: Vec<String> = goals2.iter().map(|g| g.dep.name.clone()).collect();
        assert_eq!(
            order1, order2,
            "re-seeding must make the two goal-list shuffles reproduce the same order"
        );
        set_mode(&ctx, "none");
    }

    /// `not_parallel(ctx)` must short-circuit `shuffle_goals_recursive` on its
    /// own, independent of `mode`: with a mode that would otherwise reorder
    /// (`reverse`), a `.NOTPARALLEL` run must still leave the goal list
    /// untouched. Guards the `mode == Mode::None || not_parallel(ctx)` check
    /// staying `||` rather than collapsing to `&&` (which would only return
    /// early when *both* conditions hold).
    #[test]
    fn shuffle_goals_recursive_not_parallel_is_a_noop_even_with_reorder_mode() {
        let ctx = crate::execctx::ExecContext::default();
        set_mode(&ctx, "reverse");
        crate::entry::set_not_parallel(&ctx);

        let mut goals = vec![goal_named("a"), goal_named("b"), goal_named("c")];
        shuffle_goals_recursive(&ctx, &mut goals);
        let names: Vec<String> = goals.iter().map(|g| g.dep.name.clone()).collect();
        assert_eq!(
            names,
            vec!["a", "b", "c"],
            "not_parallel must short-circuit even though mode requests a reorder"
        );
        set_mode(&ctx, "none");
    }

    /// `reorders_the_graph` answers the question a consumer of graph order
    /// has to ask: not "was `--shuffle` passed" but "did anything move".
    #[test]
    fn reorders_the_graph_tracks_effective_reordering() {
        let ctx = crate::execctx::ExecContext::default();
        assert!(!reorders_the_graph(&ctx), "no mode set");

        set_mode(&ctx, "identity");
        assert!(
            !reorders_the_graph(&ctx),
            "identity is a mode but never moves anything"
        );

        set_mode(&ctx, "reverse");
        assert!(reorders_the_graph(&ctx));

        crate::entry::set_not_parallel(&ctx);
        assert!(
            !reorders_the_graph(&ctx),
            "not_parallel short-circuits the shuffle, so order is intact"
        );
        set_mode(&ctx, "none");
    }
}
