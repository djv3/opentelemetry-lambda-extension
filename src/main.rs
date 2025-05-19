use lambda_extension::{tracing, Error, Extension, SharedService};
use opentelemetry::global;

use opentelemetry_proto::tonic::{
    collector::{
        logs::v1::ExportLogsServiceRequest, metrics::v1::ExportMetricsServiceRequest,
        trace::v1::ExportTraceServiceRequest,
    },
    logs::v1::ScopeLogs,
    metrics::v1::ScopeMetrics,
    trace::v1::ScopeSpans,
};
use tokio_util::sync::CancellationToken;

mod event_processor;
use event_processor::EventProcessor;

mod telemetry_api_processor;
use telemetry_api_processor::TelemetryApiProcessor;

pub mod shutdown_reason;

mod http_receiver;
mod pipeline;
use pipeline::Pipeline;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let cancellation_token = CancellationToken::new();

    let meter = global::meter("lambda_extension");
    let shutdown_counter = meter
        .u64_counter("lambda_shutdown")
        .with_description("Counter tracking Lambda shutdown events")
        .build();

    let (scope_logs_sender, scope_logs_receiver) = tokio::sync::mpsc::channel::<ScopeLogs>(100);
    let (scope_metrics_sender, scope_metrics_receiver) =
        tokio::sync::mpsc::channel::<ScopeMetrics>(100);
    let (scope_spans_sender, scope_spans_receiver) = tokio::sync::mpsc::channel::<ScopeSpans>(100);
    let (logs_request_sender, logs_request_receiver) =
        tokio::sync::mpsc::channel::<ExportLogsServiceRequest>(100);
    let (metrics_request_sender, metrics_request_receiver) =
        tokio::sync::mpsc::channel::<ExportMetricsServiceRequest>(100);
    let (spans_request_sender, spans_request_receiver) =
        tokio::sync::mpsc::channel::<ExportTraceServiceRequest>(100);

    Pipeline::new(
        scope_logs_receiver,
        scope_metrics_receiver,
        scope_spans_receiver,
        logs_request_receiver,
        metrics_request_receiver,
        spans_request_receiver,
    )
    .run()
    .await;

    let http_receiver = http_receiver::create_receiver(
        cancellation_token.clone(),
        logs_request_sender,
        metrics_request_sender,
        spans_request_sender,
    )
    .await;

    let telemetry_api_processor =
        TelemetryApiProcessor::new(scope_logs_sender, scope_metrics_sender, scope_spans_sender);

    let event_processor = EventProcessor::new(cancellation_token.clone(), shutdown_counter);

    Extension::new()
        .with_events_processor(event_processor)
        .with_telemetry_processor(SharedService::new(telemetry_api_processor))
        .run()
        .await
}
