use std::net::SocketAddr;
use std::sync::Arc;

use clap::Parser;
use make_srv::config::Config;
use make_srv::state::AppState;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("make_srv=info".parse()?),
        )
        .init();

    let config = Config::parse();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let ui_dist = config.ui_dist.clone();
    let max_concurrent_jobs = config.max_concurrent_jobs;

    let state: Arc<AppState> = AppState::new(config);
    let mut app = make_srv::api::router(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    if let Some(dist) = ui_dist {
        let index = dist.join("index.html");
        let serve_dir = ServeDir::new(&dist).not_found_service(ServeFile::new(index));
        app = app.fallback_service(serve_dir);
    }

    tracing::info!(%addr, max_concurrent_jobs, "make-srv listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
