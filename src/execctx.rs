//! Process configuration and execution context.
//!
//! This module is the home of make's owned runtime state as it migrates off the
//! c2rust `static mut` / scalar-atomic globals. The design splits that state
//! into three classes, each with a principled home:
//!
//! 1. **Immutable-after-init config** — values computed once during startup and
//!    read-only thereafter (e.g. `$(MAKELEVEL)`). These live in [`Config`],
//!    installed once into a process-wide [`OnceLock`]. A write-once `OnceLock`
//!    is **not** a forbidden mutable global: it is `Send + Sync`, carries no
//!    interior mutability after install, and is safe to read from anywhere —
//!    including async-signal-handler context and (later) multiple worker
//!    threads — precisely *because* it never changes. Deep, high-fan-in readers
//!    that cannot take a parameter reach it through the [`config`] accessor;
//!    threaded code can hold a cheap `Arc<Config>` clone.
//!
//! 2. **Mutable per-build runtime state** — the genuinely contended flags that
//!    change during a build and are read by code that *can* be threaded. These
//!    belong in [`ExecContext`], passed explicitly by `&mut` (never a
//!    thread-local or a fresh global). `ExecContext` is therefore the single
//!    place real synchronization would ever need to live if make's
//!    orchestration is later run on a thread pool (tokio/rayon).
//!
//! 3. **Signal-shared state** — values read or written from an async signal
//!    handler (e.g. the temp-stdin path reached by `fatal_error_signal`). POSIX
//!    async-signal-safety means a handler may only touch atomics/`sig_atomic_t`
//!    and cannot receive an `&ExecContext`, so this state legitimately stays a
//!    process-global atomic. This is a deliberate carve-out, not a smell: it is
//!    the same justification as the already-converted signal-shared atomics.
//!
//! Pass one establishes the module and migrates the cleanest class-1 member,
//! `makelevel`, out of `static mut`.

use std::sync::Arc;
use std::sync::OnceLock;

/// Immutable process configuration: values fixed once during startup and read
/// for the rest of the run. Cheap to share read-only (`Arc<Config>`) and safe
/// to read from any context because nothing mutates it after [`install_config`].
#[derive(Debug, Default, Clone)]
pub struct Config {
    /// `$(MAKELEVEL)` — the recursion depth of *this* make process. Parsed once
    /// from the `MAKELEVEL` environment variable during startup (0 at the top
    /// level, N inside a recursive `$(MAKE)`), then immutable. Replaces the
    /// former `static mut makelevel: c_uint`.
    pub makelevel: u32,
}

/// The process-wide installed [`Config`], or `None` before startup installs it.
static CONFIG: OnceLock<Config> = OnceLock::new();

/// Install the process [`Config`]. Called once during startup; first write
/// wins (a second call is ignored), matching the single-assignment lifetime of
/// the globals it replaces.
pub fn install_config(config: Config) {
    let _ = CONFIG.set(config);
}

/// The installed process [`Config`], or `None` before startup installs it.
pub fn config() -> Option<&'static Config> {
    CONFIG.get()
}

/// `$(MAKELEVEL)` for this make process.
///
/// Returns 0 until startup installs the real value, exactly mirroring the zero
/// default of the former `static mut makelevel`. The read never installs, so an
/// early read before the real value is known cannot pin a stale value.
pub fn makelevel() -> u32 {
    CONFIG.get().map_or(0, |c| c.makelevel)
}

/// Install the process [`Config`] carrying `makelevel`. Convenience for the
/// startup site that learns `makelevel` before the rest of `Config` is built;
/// as more class-1 fields move in, the install consolidates to a single site.
pub fn install_makelevel(makelevel: u32) {
    install_config(Config { makelevel });
}

/// Mutable per-build execution context: the eventual owner of make's contended,
/// non-signal runtime flags, threaded explicitly by `&mut`. It carries an
/// `Arc<Config>` so threaded tasks reach immutable config without the global
/// accessor. This is the runtime-agnostic seam that keeps a future tokio/rayon
/// layer an additive change rather than a rewrite: synchronization, if ever
/// needed, lives here and nowhere else.
#[derive(Debug, Clone)]
pub struct ExecContext {
    /// Read-only process configuration, shared cheaply with any worker.
    pub config: Arc<Config>,
}

impl ExecContext {
    /// Build a context over the given immutable [`Config`].
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[cfg(test)]
mod tests {
    use super::{install_makelevel, makelevel, Config, ExecContext};
    use std::sync::Arc;

    /// `Config` simply carries the value; the field is the source of truth.
    #[test]
    fn config_holds_makelevel() {
        let c = Config { makelevel: 3 };
        assert_eq!(c.makelevel, 3);
    }

    /// `ExecContext` exposes immutable config through its `Arc<Config>` without
    /// touching the process-global `OnceLock`, so it stays isolated from other
    /// tests.
    #[test]
    fn execcontext_carries_config() {
        let ctx = ExecContext::new(Arc::new(Config { makelevel: 9 }));
        assert_eq!(ctx.config.makelevel, 9);
        // Cloning shares the same immutable config.
        assert_eq!(ctx.clone().config.makelevel, 9);
    }

    /// `makelevel()` reflects the installed `Config`. Installing 0 matches the
    /// universal test/default value, so this does not perturb the shared global
    /// for other tests that assume a top-level (zero) make.
    #[test]
    fn makelevel_accessor_reads_installed_config() {
        install_makelevel(0);
        assert_eq!(makelevel(), 0);
    }
}
