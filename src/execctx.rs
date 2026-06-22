//! Process configuration and execution context.
//!
//! This is the owned root of make's runtime state. The c2rust translation kept
//! that state in process-global `static mut`s; we are replacing them with state
//! owned by `main` and threaded explicitly down the call graph by reference, so
//! functions stay pure (no ambient globals) and become safe to run on multiple
//! threads later. Think of `main` as the root of a tree and the context as
//! passed depth-first into every node that needs it — never reached through a
//! global or thread-local.
//!
//! [`Config`] holds values fixed once during startup (read-only thereafter);
//! [`ExecContext`] owns it plus the mutable per-build runtime state. Readers
//! take `&ExecContext` (or `&mut` when they update it); there is no global
//! accessor and no singleton.

/// Immutable process configuration: values fixed once during startup and read
/// for the rest of the run.
#[derive(Debug, Default, Clone)]
pub struct Config {
    /// `$(MAKELEVEL)` — the recursion depth of *this* make process. Parsed once
    /// from the `MAKELEVEL` environment variable during startup (0 at the top
    /// level, N inside a recursive `$(MAKE)`), then immutable.
    pub makelevel: u32,
}

/// The owned execution context, created in `main` and threaded by reference
/// into the call graph. Holds the immutable [`Config`] plus (as the migration
/// proceeds) the mutable runtime state that used to live in `static mut`s.
#[derive(Debug, Default, Clone)]
pub struct ExecContext {
    /// Read-only process configuration.
    pub config: Config,

    /// `f_mtime`'s future-timestamp cache: the most recently sampled "adjusted
    /// now" (`file_timestamp_now` plus the timestamp resolution slack). When a
    /// file's mtime is at or before this value it is known not to be in the
    /// future without re-reading the system clock, so the warning check only
    /// re-samples the clock when a file's mtime is past the cache. Per-run
    /// mutable state — interior mutability keeps readers on `&ExecContext`.
    pub mtime_adjusted_now: ::core::cell::Cell<crate::ffi_types::uintmax_t>,
    /// `load_too_high`'s per-second job-weighting cache: the wall-clock second
    /// of the previous load sample. When a new second begins, the running-job
    /// estimate folds in the jobs counted during the second just elapsed (see
    /// [`Self::load_prev_weight`]) and resets the per-second counter. Per-run
    /// mutable state — interior mutability keeps readers on `&ExecContext`.
    pub load_sample_second: ::core::cell::Cell<crate::ffi_types::time_t>,

    /// `load_too_high`'s per-second job-weighting cache: the job weight carried
    /// from the immediately preceding second (`LOAD_WEIGHT_B * jobs`), or `0`
    /// when more than one second has elapsed. Added to the current second's
    /// weight to estimate system load between real `getloadavg` samples.
    pub load_prev_weight: ::core::cell::Cell<::core::ffi::c_double>,
}

impl ExecContext {
    /// Build a context over the given immutable [`Config`]. Mutable per-run
    /// caches start at their zero defaults.
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ..Self::default()
        }
    }

    /// `$(MAKELEVEL)` for this make process.
    pub fn makelevel(&self) -> u32 {
        self.config.makelevel
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, ExecContext};

    #[test]
    fn context_exposes_makelevel() {
        let ctx = ExecContext::new(Config { makelevel: 3 });
        assert_eq!(ctx.makelevel(), 3);
        // Cloning yields an independent copy of the owned state.
        assert_eq!(ctx.clone().makelevel(), 3);
    }

    #[test]
    fn default_makelevel_is_zero() {
        assert_eq!(ExecContext::default().makelevel(), 0);
    }

    #[test]
    fn load_sample_cache_starts_zeroed() {
        let ctx = ExecContext::new(Config { makelevel: 0 });
        assert_eq!(ctx.load_sample_second.get(), 0);
        assert_eq!(ctx.load_prev_weight.get(), 0.0);
        // `..Self::default()` in `new` must not skip the cache fields.
        assert_eq!(ExecContext::default().load_sample_second.get(), 0);
    }
}
