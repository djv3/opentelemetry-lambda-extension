use lambda_extension::{tracing, Error, Extension, SharedService};
use opentelemetry::global;

use tokio_util::sync::CancellationToken;

mod event_processor;
use event_processor::EventProcessor;

mod telemetry_api_processor;
use telemetry_api_processor::TelemetryApiProcessor;

pub mod shutdown_reason;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let cancellation_token = CancellationToken::new();

    let meter = global::meter("lambda_extension");
    let shutdown_counter = meter
        .u64_counter("lambda_shutdown")
        .with_description("Counter tracking Lambda shutdown events")
        .build();

    let (telemetry_sender, _) = tokio::sync::mpsc::channel(100);

    let telemetry_api_processor = TelemetryApiProcessor::new(telemetry_sender);

    let event_processor = EventProcessor::new(cancellation_token.clone(), shutdown_counter);

    Extension::new()
        .with_events_processor(event_processor)
        .with_telemetry_processor(SharedService::new(telemetry_api_processor))
        .run()
        .await
}
