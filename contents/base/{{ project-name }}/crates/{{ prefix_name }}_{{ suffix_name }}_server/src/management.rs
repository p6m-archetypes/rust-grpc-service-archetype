use crate::readiness::ReadinessState;
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use tokio::net::TcpListener;

/// Management routes — health probes and Prometheus metrics on the management port.
///
/// Installs the process-global Prometheus recorder (call once, at startup).
pub fn routes(readiness: ReadinessState) -> Router {
    let metrics_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install Prometheus metrics recorder");

    // Seed a build-info family so /metrics is meaningful from the first scrape.
    metrics::gauge!(
        "{{ prefix_name }}_{{ suffix_name }}_build_info",
        "version" => env!("CARGO_PKG_VERSION")
    )
    .set(1.0);

    Router::new()
        .route("/health/readiness", get(readiness_check))
        .route("/health/liveness", get(liveness_check))
        .route(
            "/metrics",
            get(move || {
                let handle = metrics_handle.clone();
                async move { metrics_handler(handle).await }
            }),
        )
        .with_state(readiness)
}

async fn readiness_check(State(readiness): State<ReadinessState>) -> impl IntoResponse {
    if readiness.is_ready().await {
        (StatusCode::OK, axum::Json(serde_json::json!({"status": "ok"})))
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            axum::Json(serde_json::json!({"status": "not_ready"})),
        )
    }
}

async fn liveness_check() -> impl IntoResponse {
    (StatusCode::OK, axum::Json(serde_json::json!({"status": "ok"})))
}

/// Prometheus metrics endpoint: renders everything the installed recorder has collected.
async fn metrics_handler(handle: PrometheusHandle) -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        handle.render(),
    )
}

pub async fn serve(listener: TcpListener, readiness: ReadinessState) -> anyhow::Result<()> {
    let addr = listener.local_addr()?;
    tracing::info!("Management server on http://{addr}");
    axum::serve(listener, routes(readiness)).await?;
    Ok(())
}
