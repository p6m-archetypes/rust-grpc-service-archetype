use crate::readiness::ReadinessState;
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

pub fn routes(readiness: ReadinessState) -> Router {
    Router::new()
        .route("/health/readiness", get(readiness_check))
        .route("/health/liveness", get(liveness_check))
        .route("/metrics", get(metrics_handler))
        .with_state(readiness)
}

async fn readiness_check(State(readiness): State<ReadinessState>) -> impl IntoResponse {
    if readiness.is_ready().await {
        (StatusCode::OK, axum::Json(serde_json::json!({"status": "ok"})))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, axum::Json(serde_json::json!({"status": "not_ready"})))
    }
}

async fn liveness_check() -> impl IntoResponse {
    (StatusCode::OK, axum::Json(serde_json::json!({"status": "ok"})))
}

/// Prometheus metrics endpoint on the management port.
/// Returns metrics collected by the `metrics` crate via `metrics-exporter-prometheus`.
async fn metrics_handler() -> impl IntoResponse {
    // TODO: wire up metrics-exporter-prometheus handle and return rendered text.
    // For now returns an empty valid Prometheus response so Kubernetes scraping succeeds.
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        "# Prometheus metrics\n",
    )
}

pub async fn serve(listener: TcpListener, readiness: ReadinessState) -> anyhow::Result<()> {
    let addr = listener.local_addr()?;
    tracing::info!("Management server on http://{addr}");
    axum::serve(listener, routes(readiness)).await?;
    Ok(())
}
