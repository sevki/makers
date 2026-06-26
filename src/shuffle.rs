use crate::file::{Dep, File};
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

fn random_shuffle(slice: &mut [*mut Dep]) {
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

fn reverse_shuffle<T>(slice: &mut [*mut T]) {
    let len = slice.len();
    for i in 0..len / 2 {
        slice.swap(i, len - 1 - i);
    }
}

fn identity_shuffle<T>(_: &mut [*mut T]) {}

/// Walk the deps linked list, shuffle the order, and write the new order back
/// via the `shuf` field on each node.
unsafe fn shuffle_deps(deps: *mut Dep) {
    let mut ndeps: usize = 0;
    let mut d = deps;
    while !d.is_null() {
        if (*d).wait_here {
            return;
        }
        ndeps += 1;
        d = (*d).next;
    }
    if ndeps == 0 {
        return;
    }

    let mut deps_order = Vec::with_capacity(ndeps);

    d = deps;
    for _ in 0..ndeps {
        deps_order.push(d);
        d = (*d).next;
    }

    match config().mode {
        Mode::None => {}
        Mode::Random => random_shuffle(&mut deps_order),
        Mode::Reverse => reverse_shuffle(&mut deps_order),
        Mode::Identity => identity_shuffle(&mut deps_order),
    }

    d = deps;
    for dep in deps_order {
        (*d).shuf = dep;
        d = (*d).next;
    }
}

unsafe fn shuffle_file_deps_recursive(f: *mut File) {
    if f.is_null() || (*f).was_shuffled {
        return;
    }
    (*f).was_shuffled = true;
    shuffle_deps((*f).deps);
    let mut d = (*f).deps;
    while !d.is_null() {
        shuffle_file_deps_recursive((*d).file);
        d = (*d).next;
    }
}

/// Shuffle the order of `deps` and recursively shuffle each file's deps. Safe
/// to call when shuffling is disabled (no-op).
///
/// # Safety
/// `deps` must be a valid (possibly null) head of a properly-linked `Dep`
/// chain, and the chain's `File` pointers must be valid.
pub unsafe fn shuffle_deps_recursive(deps: *mut Dep) {
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
    shuffle_deps(deps);
    let mut d = deps;
    while !d.is_null() {
        shuffle_file_deps_recursive((*d).file);
        d = (*d).next;
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
