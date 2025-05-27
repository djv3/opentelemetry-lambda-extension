use lambda_extension::{tracing, Error};
use opentelemetry_lambda_extension::ShutdownReason;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel::<ShutdownReason>();
    Ok(())
}
