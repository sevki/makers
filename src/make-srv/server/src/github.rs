use anyhow::{Context, bail};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde_json::json;

use crate::config::Config;
use crate::job::{Job, Screenshot};

const USER_AGENT: &str = "make-srv-job-server";

/// Pushes the job's screenshots to `config.screenshot_branch` and posts them
/// as a single comment on the job's pull request. Returns the comment's URL.
pub async fn publish(
    http: &reqwest::Client,
    config: &Config,
    job: &Job,
    screenshots: &[Screenshot],
) -> anyhow::Result<String> {
    let token = config
        .github_token
        .as_deref()
        .context("GITHUB_TOKEN is required to publish screenshots")?;

    ensure_branch(http, token, &job.repo, &config.screenshot_branch).await?;

    let out_dir = config.artifacts_dir.join(job.id.to_string());
    let mut entries = Vec::with_capacity(screenshots.len());
    for shot in screenshots {
        let path = out_dir.join(&shot.file_name);
        let bytes = tokio::fs::read(&path)
            .await
            .with_context(|| format!("reading screenshot {}", path.display()))?;
        let repo_path = format!("screenshots/{}/{}", job.id, shot.file_name);
        put_file(
            http,
            token,
            &job.repo,
            &config.screenshot_branch,
            &repo_path,
            &bytes,
            &format!("screenshots: job {} ({})", job.id, shot.route),
        )
        .await?;
        let raw_url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}",
            job.repo, config.screenshot_branch, repo_path
        );
        entries.push((shot.route.clone(), raw_url));
    }

    let body = render_comment(job, &entries);
    post_comment(http, token, &job.repo, job.pr_number, &body).await
}

async fn ensure_branch(
    http: &reqwest::Client,
    token: &str,
    repo: &str,
    branch: &str,
) -> anyhow::Result<()> {
    let ref_url = format!("https://api.github.com/repos/{repo}/git/ref/heads/{branch}");
    let resp = gh_get(http, token, &ref_url).await?;
    if resp.status().is_success() {
        return Ok(());
    }
    if resp.status() != reqwest::StatusCode::NOT_FOUND {
        bail!("checking branch {branch} failed: {}", resp.status());
    }

    let repo_url = format!("https://api.github.com/repos/{repo}");
    let repo_info: serde_json::Value = gh_get(http, token, &repo_url)
        .await?
        .error_for_status()
        .context("fetching repo info")?
        .json()
        .await?;
    let default_branch = repo_info["default_branch"]
        .as_str()
        .context("repo info missing default_branch")?;

    let base_ref_url =
        format!("https://api.github.com/repos/{repo}/git/ref/heads/{default_branch}");
    let base_ref: serde_json::Value = gh_get(http, token, &base_ref_url)
        .await?
        .error_for_status()
        .context("fetching default branch ref")?
        .json()
        .await?;
    let sha = base_ref["object"]["sha"]
        .as_str()
        .context("default branch ref missing sha")?;

    let create_url = format!("https://api.github.com/repos/{repo}/git/refs");
    let resp = http
        .post(&create_url)
        .bearer_auth(token)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .json(&json!({ "ref": format!("refs/heads/{branch}"), "sha": sha }))
        .send()
        .await
        .context("creating screenshot branch")?;
    if !resp.status().is_success() {
        bail!(
            "failed to create branch {branch}: {} {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        );
    }
    Ok(())
}

async fn put_file(
    http: &reqwest::Client,
    token: &str,
    repo: &str,
    branch: &str,
    path: &str,
    bytes: &[u8],
    message: &str,
) -> anyhow::Result<()> {
    let url = format!("https://api.github.com/repos/{repo}/contents/{path}");
    let resp = http
        .put(&url)
        .bearer_auth(token)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .json(&json!({
            "message": message,
            "content": BASE64.encode(bytes),
            "branch": branch,
        }))
        .send()
        .await
        .with_context(|| format!("uploading {path}"))?;
    if !resp.status().is_success() {
        bail!(
            "failed to upload {path}: {} {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        );
    }
    Ok(())
}

async fn post_comment(
    http: &reqwest::Client,
    token: &str,
    repo: &str,
    pr_number: u64,
    body: &str,
) -> anyhow::Result<String> {
    let url = format!("https://api.github.com/repos/{repo}/issues/{pr_number}/comments");
    let resp = http
        .post(&url)
        .bearer_auth(token)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .json(&json!({ "body": body }))
        .send()
        .await
        .context("posting PR comment")?;
    if !resp.status().is_success() {
        bail!(
            "failed to post PR comment: {} {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        );
    }
    let comment: serde_json::Value = resp.json().await.context("parsing comment response")?;
    comment["html_url"]
        .as_str()
        .map(str::to_owned)
        .context("comment response missing html_url")
}

async fn gh_get(
    http: &reqwest::Client,
    token: &str,
    url: &str,
) -> anyhow::Result<reqwest::Response> {
    http.get(url)
        .bearer_auth(token)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .with_context(|| format!("GET {url}"))
}

fn render_comment(job: &Job, entries: &[(String, String)]) -> String {
    let mut body = format!(
        "### \u{1F4F8} UI screenshots for `{}`\n\nCommit `{}`\n\n",
        job.repo, job.head_sha
    );
    for (route, raw_url) in entries {
        body.push_str(&format!("**`{route}`**\n\n![{route}]({raw_url})\n\n"));
    }
    body.push_str(&format!("<sub>job `{}`</sub>\n", job.id));
    body
}
