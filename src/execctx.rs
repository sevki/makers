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
}

impl ExecContext {
    /// Build a context over the given immutable [`Config`].
    pub fn new(config: Config) -> Self {
        Self { config }
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
}
