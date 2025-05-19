use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use lambda_extension::tracing;
use opentelemetry_proto::tonic::collector::{
    logs::v1::ExportLogsServiceRequest, metrics::v1::ExportMetricsServiceRequest,
    trace::v1::ExportTraceServiceRequest,
};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct HttpReceiver {
    logs_consumer: Sender<ExportLogsServiceRequest>,
    metrics_consumer: Sender<ExportMetricsServiceRequest>,
    traces_consumer: Sender<ExportTraceServiceRequest>,
}

impl HttpReceiver {
    pub fn new(
        logs_consumer: Sender<ExportLogsServiceRequest>,
        metrics_consumer: Sender<ExportMetricsServiceRequest>,
        traces_consumer: Sender<ExportTraceServiceRequest>,
    ) -> Self {
        Self {
            logs_consumer,
            metrics_consumer,
            traces_consumer,
        }
    }
}

pub async fn create_receiver(
    ct: CancellationToken,
    logs_consumer: Sender<ExportLogsServiceRequest>,
    metrics_consumer: Sender<ExportMetricsServiceRequest>,
    traces_consumer: Sender<ExportTraceServiceRequest>,
) {
    let receiver = HttpReceiver::new(logs_consumer, metrics_consumer, traces_consumer);

    let app = axum::Router::new()
        .route("/v1/logs", axum::routing::post(handle_logs))
        .route("/v1/metrics", axum::routing::post(handle_metrics))
        .route("/v1/traces", axum::routing::post(handle_traces))
        .with_state(receiver);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4318").await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(async move { ct.cancelled().await })
        .await
        .unwrap();
}

#[axum::debug_handler]
async fn handle_logs(
    State(receiver): State<HttpReceiver>,
    Json(request): Json<ExportLogsServiceRequest>,
) -> impl IntoResponse {
    if let Err(e) = receiver.logs_consumer.send(request).await {
        tracing::error!("Failed to send logs request: {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::ACCEPTED
}

#[axum::debug_handler]
async fn handle_metrics(
    State(receiver): State<HttpReceiver>,
    Json(request): Json<ExportMetricsServiceRequest>,
) -> impl IntoResponse {
    if let Err(e) = receiver.metrics_consumer.send(request).await {
        tracing::error!("Failed to send metrics request: {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::ACCEPTED
}

#[axum::debug_handler]
async fn handle_traces(
    State(receiver): State<HttpReceiver>,
    Json(request): Json<ExportTraceServiceRequest>,
) -> impl IntoResponse {
    if let Err(e) = receiver.traces_consumer.send(request).await {
        tracing::error!("Failed to send trace request: {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::ACCEPTED
}
