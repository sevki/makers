use std::path::PathBuf;

use clap::Parser;

/// Configuration for the make-srv job server.
#[derive(Debug, Clone, Parser)]
pub struct Config {
    /// Port to listen on.
    #[arg(long, env = "PORT", default_value_t = 8787)]
    pub port: u16,

    /// Maximum number of screenshot jobs that may run at the same time.
    #[arg(long, env = "MAX_CONCURRENT_JOBS", default_value_t = 4)]
    pub max_concurrent_jobs: usize,

    /// Directory where per-job screenshot artifacts are written.
    #[arg(long, env = "ARTIFACTS_DIR", default_value = "artifacts")]
    pub artifacts_dir: PathBuf,

    /// GitHub token used to push screenshots and post PR comments. If unset,
    /// jobs still run and capture screenshots but skip publishing to GitHub.
    #[arg(long, env = "GITHUB_TOKEN")]
    pub github_token: Option<String>,

    /// Branch that screenshot artifacts are committed to.
    #[arg(
        long,
        env = "SCREENSHOT_BRANCH",
        default_value = "screenshot-artifacts"
    )]
    pub screenshot_branch: String,

    /// Path to the Node/Playwright script used to capture screenshots.
    #[arg(
        long,
        env = "SCREENSHOT_SCRIPT",
        default_value = "ui/scripts/screenshot.mjs"
    )]
    pub screenshot_script: PathBuf,

    /// Optional path to a built UI (dist/) to serve alongside the API.
    #[arg(long, env = "UI_DIST_DIR")]
    pub ui_dist: Option<PathBuf>,

    /// Skip screenshot capture and GitHub publishing; used for tests and
    /// local development of the dashboard.
    #[arg(long, env = "DRY_RUN", default_value_t = false)]
    pub dry_run: bool,
}
