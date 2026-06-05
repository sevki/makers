use crate::file::{Dep, File};
use crate::floc::Floc;

use crate::misc::{make_rand, make_seed};
extern "C" {
    fn fatal(flocp: *const Floc, length: usize, fmt: *const ::core::ffi::c_char, ...) -> !;
    static mut not_parallel: ::core::ffi::c_int;
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    None,
    Random,
    Reverse,
    Identity,
}

type Shuffler = fn(&mut [*mut Dep]);

struct Config {
    mode: Mode,
    seed: u32,
    shuffler: Option<Shuffler>,
    label: String,
}

static mut CONFIG: Config = Config {
    mode: Mode::None,
    seed: 0,
    shuffler: None,
    label: String::new(),
};

fn config() -> &'static mut Config {
    // make's runtime state is single-threaded; matches the existing convention
    // for static mut globals throughout this codebase.
    unsafe { &mut CONFIG }
}

/// Returns the canonical label for the active shuffle mode (e.g. `"reverse"`,
/// or the seed as a decimal string for `random`), or `None` when shuffling is
/// disabled.
pub fn get_mode() -> Option<String> {
    let label = &config().label;
    if label.is_empty() {
        None
    } else {
        Some(label.clone())
    }
}

/// Configure shuffle behavior from a textual argument (typically the value of
/// the `--shuffle=` command-line flag). Aborts via `fatal` on a malformed
/// numeric seed, matching the original C behavior.
pub fn set_mode(arg: &str) {
    let cfg = config();
    if arg.eq_ignore_ascii_case("reverse") {
        cfg.mode = Mode::Reverse;
        cfg.shuffler = Some(reverse_shuffle);
        cfg.label = "reverse".to_string();
    } else if arg.eq_ignore_ascii_case("identity") {
        cfg.mode = Mode::Identity;
        cfg.shuffler = Some(identity_shuffle);
        cfg.label = "identity".to_string();
    } else if arg.eq_ignore_ascii_case("none") {
        cfg.mode = Mode::None;
        cfg.shuffler = None;
        cfg.label.clear();
    } else {
        let seed = if arg.eq_ignore_ascii_case("random") {
            unsafe { make_rand() }
        } else {
            match arg.parse::<u32>() {
                Ok(n) => n,
                Err(_) => fatal_invalid(arg),
            }
        };
        cfg.mode = Mode::Random;
        cfg.seed = seed;
        cfg.shuffler = Some(random_shuffle);
        cfg.label = seed.to_string();
    }
}

fn fatal_invalid(arg: &str) -> ! {
    let msg = format!("invalid shuffle mode: Invalid value: '{}'\0", arg);
    unsafe {
        fatal(
            ::core::ptr::null(),
            msg.len(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            msg.as_ptr() as *const ::core::ffi::c_char,
        );
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

fn reverse_shuffle(slice: &mut [*mut Dep]) {
    let len = slice.len();
    for i in 0..len / 2 {
        slice.swap(i, len - 1 - i);
    }
}

fn identity_shuffle(_: &mut [*mut Dep]) {}

/// Walk the deps linked list, shuffle the order, and write the new order back
/// via the `shuf` field on each node.
unsafe fn shuffle_deps(deps: *mut Dep) {
    let mut ndeps: usize = 0;
    let mut d = deps;
    while !d.is_null() {
        if (*d).wait_here() != 0 {
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

    if let Some(f) = config().shuffler {
        f(&mut deps_order);
    }

    d = deps;
    for dep in deps_order {
        (*d).shuf = dep;
        d = (*d).next;
    }
}

unsafe fn shuffle_file_deps_recursive(f: *mut File) {
    if f.is_null() || (*f).was_shuffled() != 0 {
        return;
    }
    (*f).set_was_shuffled(1);
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
    let cfg = config();
    if cfg.mode == Mode::None || not_parallel != 0 {
        return;
    }
    if cfg.mode == Mode::Random {
        make_seed(cfg.seed);
    }
    shuffle_deps(deps);
    let mut d = deps;
    while !d.is_null() {
        shuffle_file_deps_recursive((*d).file);
        d = (*d).next;
    }
}
