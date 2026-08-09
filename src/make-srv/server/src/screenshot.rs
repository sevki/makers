use anyhow::{Context, bail};

use crate::config::Config;
use crate::job::{Job, Screenshot};

/// Runs the Playwright screenshot script for every route configured on the
/// job and returns the screenshots it produced.
pub async fn capture(config: &Config, job: &Job) -> anyhow::Result<Vec<Screenshot>> {
    let out_dir = config.artifacts_dir.join(job.id.to_string());
    tokio::fs::create_dir_all(&out_dir)
        .await
        .with_context(|| format!("creating artifacts dir {}", out_dir.display()))?;

    let status = tokio::process::Command::new("node")
        .arg(&config.screenshot_script)
        .arg("--base-url")
        .arg(&job.base_url)
        .arg("--routes")
        .arg(job.routes.join(","))
        .arg("--out-dir")
        .arg(&out_dir)
        .status()
        .await
        .with_context(|| {
            format!(
                "spawning screenshot script {}",
                config.screenshot_script.display()
            )
        })?;

    if !status.success() {
        bail!("screenshot script exited with {status}");
    }

    let mut screenshots = Vec::new();
    for route in &job.routes {
        let file_name = Job::route_file_name(route);
        let path = out_dir.join(&file_name);
        if tokio::fs::try_exists(&path).await.unwrap_or(false) {
            screenshots.push(Screenshot {
                route: route.clone(),
                file_name,
                raw_url: None,
            });
        } else {
            tracing::warn!(route, path = %path.display(), "screenshot script did not produce expected file");
        }
    }

    if screenshots.is_empty() {
        bail!("screenshot script produced no images for any of the requested routes");
    }

    Ok(screenshots)
}
