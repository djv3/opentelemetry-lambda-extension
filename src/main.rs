use lambda_extension::{service_fn, tracing, Extension, Error, SharedService};

mod telemetry_extension;
use telemetry_extension::telemetry_processor;

use tokio_util::sync::CancellationToken;

mod event_processor;
use event_processor::EventProcessor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let cancellation_token = CancellationToken::new();
    let event_processor = EventProcessor::new(cancellation_token.clone());

    Extension::new()
        .with_events_processor(event_processor)
        .with_telemetry_processor(SharedService::new(service_fn(telemetry_processor)))
        .run()
        .await
}
