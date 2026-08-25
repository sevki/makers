//! Structured run results for `main_0` — Phase B of the library-ification
//! plan (#432): a library returns errors, it does not exit the process.
//!
//! `main_0` reports how the run ended as `Result<BuildReport, BuildError>`;
//! the `bin/make.rs` shim is the single place that maps that result onto a
//! process exit code (`std::process::exit`). The variants mirror make's
//! canonical exit statuses: `MAKE_SUCCESS` (0), `MAKE_TROUBLE` (1, "some
//! target is not up to date" under `-q`), and `MAKE_FAILURE` (2, a target
//! failed to build or a fatal error was raised).
//!
//! Fatal errors (`fatal()`/`pfatal_with_name`/`out_of_memory`) surface here
//! as [`BuildError::Failure`]. Call sites that already return a `Result` use
//! the non-diverging `_err` twins and propagate; the handful that cannot yet
//! (signal handlers, `decode_switches`) run the shared end-of-run cleanup and
//! bridge through `output::exit_on_err`, which is the last stop before the
//! shim's single exit.

use core::fmt;

use crate::entry::{MAKE_SUCCESS, MAKE_TROUBLE};

/// Success report for a completed run: every goal was brought up to date (or
/// was already current). Carries no detail yet; later #432 subtasks grow it
/// as the update walk starts returning structured results instead of writing
/// through the context.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BuildReport;

impl BuildReport {
    /// The process exit code this report maps to: `MAKE_SUCCESS` (0).
    pub fn exit_code(self) -> i32 {
        MAKE_SUCCESS
    }
}

/// How a run failed, mirroring make's non-zero exit statuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildError {
    /// `-q`/`--question` found a target that is not up to date —
    /// `MAKE_TROUBLE` (1).
    Trouble,
    /// A target failed to build, or a fatal error ended the run —
    /// `MAKE_FAILURE` (2).
    Failure,
}

impl BuildError {
    /// The process exit code this error maps to.
    pub fn exit_code(self) -> i32 {
        match self {
            BuildError::Trouble => MAKE_TROUBLE,
            BuildError::Failure => crate::entry::MAKE_FAILURE,
        }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::Trouble => f.write_str("some target is not up to date"),
            BuildError::Failure => f.write_str("failed to remake a target"),
        }
    }
}

impl std::error::Error for BuildError {}

/// Map one of make's canonical exit statuses (`MAKE_SUCCESS`/`MAKE_TROUBLE`/
/// `MAKE_FAILURE`) onto the structured run result. Callers only pass those
/// three; any other non-zero status is treated as a failure.
pub fn result_from_status(status: i32) -> Result<BuildReport, BuildError> {
    match status {
        MAKE_SUCCESS => Ok(BuildReport),
        MAKE_TROUBLE => Err(BuildError::Trouble),
        _ => Err(BuildError::Failure),
    }
}

/// The process exit code for a finished run, however it ended — the inverse
/// of [`result_from_status`], used by the `bin/make.rs` shim's single
/// `std::process::exit` call.
pub fn exit_code(result: Result<BuildReport, BuildError>) -> i32 {
    match result {
        Ok(report) => report.exit_code(),
        Err(err) => err.exit_code(),
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::entry::MAKE_FAILURE};

    #[test]
    fn exit_codes_mirror_the_canonical_statuses() {
        assert_eq!(BuildReport.exit_code(), MAKE_SUCCESS);
        assert_eq!(BuildError::Trouble.exit_code(), MAKE_TROUBLE);
        assert_eq!(BuildError::Failure.exit_code(), MAKE_FAILURE);
    }

    #[test]
    fn result_from_status_round_trips_each_status() {
        assert_eq!(result_from_status(MAKE_SUCCESS), Ok(BuildReport));
        assert_eq!(result_from_status(MAKE_TROUBLE), Err(BuildError::Trouble));
        assert_eq!(result_from_status(MAKE_FAILURE), Err(BuildError::Failure));
    }

    #[test]
    fn exit_code_inverts_result_from_status() {
        for status in [MAKE_SUCCESS, MAKE_TROUBLE, MAKE_FAILURE] {
            assert_eq!(exit_code(result_from_status(status)), status);
        }
    }

    #[test]
    fn errors_render_a_human_message() {
        assert_eq!(
            BuildError::Trouble.to_string(),
            "some target is not up to date"
        );
        assert_eq!(BuildError::Failure.to_string(), "failed to remake a target");
    }
}
