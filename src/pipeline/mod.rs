use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{exporter::Exporter, messages::ScopedTelemetry, receivers::Receiver, Processor};
mod builder;
pub use builder::PipelineBuilder;
pub struct TelemetryPipeline {
    receiver: Option<Box<dyn Receiver>>,
    receiver_channel: mpsc::UnboundedReceiver<ScopedTelemetry>,
    processors: Vec<Box<dyn Processor>>,
    exporter: Box<dyn Exporter>,
    failover_sender: Option<mpsc::UnboundedSender<ScopedTelemetry>>,
    cancellation_token: CancellationToken,
}
