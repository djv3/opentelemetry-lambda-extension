use lambda_extension::{service_fn, tracing, Error, Extension, SharedService};

mod events_extension;
use events_extension::events_processor;

mod telemetry_extension;
use telemetry_extension::telemetry_processor;

mod otel_exporter;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let endpoint = env::var("OTEL_EXPORTER_ENDPOINT").unwrap_or_else(|_| {
        tracing::warn!("OTEL_EXPORTER_ENDPOINT not set, using default");
        "http://localhost:4317".to_string()
    });

    let exporter = otel_exporter::create_exporter(endpoint)
        .await
        .expect("Failed to create exporter");

    Extension::new()
        .with_events_processor(service_fn(events_processor))
        .with_telemetry_processor(SharedService::new(service_fn(telemetry_processor)))
        .run()
        .await
}
