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
        unsafe { make_seed(seed) };
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
        unsafe { make_seed(seed) };
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
}
