use lambda_extension::{service_fn, tracing, Extension, Error, SharedService};

mod events_extension;
use events_extension::events_processor;

mod telemetry_extension;
use telemetry_extension::telemetry_processor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    Extension::new()
        .with_events_processor(service_fn(events_processor))
        .with_telemetry_processor(SharedService::new(service_fn(telemetry_processor)))
        .run()
        .await
}
