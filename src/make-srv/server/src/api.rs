use std::convert::Infallible;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::response::sse::{Event, Sse};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_util::StreamExt;
use reqwest::StatusCode;
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

use crate::job::NewJobRequest;
use crate::state::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/jobs", post(create_job).get(list_jobs))
        .route("/api/jobs/{id}", get(get_job))
        .route("/api/events", get(events))
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn create_job(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NewJobRequest>,
) -> impl IntoResponse {
    if req.repo.split('/').count() != 2 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "repo must be in the form owner/name" })),
        )
            .into_response();
    }
    if req.routes.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "routes must not be empty" })),
        )
            .into_response();
    }
    let job = state.enqueue(req).await;
    (StatusCode::CREATED, Json(job)).into_response()
}

async fn list_jobs(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(state.list_jobs().await)
}

async fn get_job(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match state.get_job(id).await {
        Some(job) => Json(job).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn events(
    State(state): State<Arc<AppState>>,
) -> Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.events.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|event| async move {
        let event = event.ok()?;
        let data = serde_json::to_string(&event.job).ok()?;
        Some(Ok(Event::default().event("job").data(data)))
    });
    Sse::new(stream)
}
