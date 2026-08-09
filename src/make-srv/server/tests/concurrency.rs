use std::time::Duration;

use axum::Router;
use axum::body::Body;
use http::{Request, StatusCode};
use http_body_util::BodyExt;
use make_srv::config::Config;
use make_srv::state::AppState;
use serde_json::{Value, json};
use tower::ServiceExt;

fn test_config(max_concurrent_jobs: usize) -> Config {
    Config {
        port: 0,
        max_concurrent_jobs,
        artifacts_dir: "artifacts".into(),
        github_token: None,
        screenshot_branch: "screenshot-artifacts".into(),
        screenshot_script: "unused".into(),
        ui_dist: None,
        dry_run: true,
    }
}

async fn body_json(response: axum::response::Response) -> Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

async fn submit_job(app: &Router, pr_number: u64) -> Value {
    let payload = json!({
        "repo": "octo/demo",
        "pr_number": pr_number,
        "head_sha": "deadbeef",
        "base_url": "http://localhost:4173",
        "routes": ["/"],
    });
    let request = Request::builder()
        .method("POST")
        .uri("/api/jobs")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    body_json(response).await
}

async fn list_jobs(app: &Router) -> Vec<Value> {
    let request = Request::builder()
        .uri("/api/jobs")
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    body_json(response).await.as_array().unwrap().clone()
}

/// Submitting several jobs at once must run at most `MAX_CONCURRENT_JOBS`
/// simultaneously, while still eventually completing all of them - proving
/// jobs are queued/concurrent rather than serialized or dropped.
#[tokio::test]
async fn bounds_concurrency_and_completes_every_job() {
    const MAX_CONCURRENT: usize = 2;
    const TOTAL_JOBS: u64 = 5;

    let state = AppState::new(test_config(MAX_CONCURRENT));
    let app = make_srv::api::router(state);

    for pr in 0..TOTAL_JOBS {
        submit_job(&app, pr).await;
    }

    // Give the scheduler a moment to start jobs, but not enough for a single
    // dry-run job (200ms) to finish.
    tokio::time::sleep(Duration::from_millis(50)).await;
    let jobs = list_jobs(&app).await;
    assert_eq!(jobs.len(), TOTAL_JOBS as usize);
    let running = jobs.iter().filter(|j| j["status"] == "running").count();
    assert!(
        running <= MAX_CONCURRENT,
        "expected at most {MAX_CONCURRENT} running jobs, saw {running}"
    );
    assert!(running >= 1, "expected at least one job to have started");

    // Enough time for all batches (ceil(5/2) * 200ms) to finish.
    tokio::time::sleep(Duration::from_millis(900)).await;
    let jobs = list_jobs(&app).await;
    for job in &jobs {
        assert_eq!(job["status"], "succeeded", "job did not succeed: {job}");
    }
}
