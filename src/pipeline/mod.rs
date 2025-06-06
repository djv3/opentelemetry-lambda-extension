use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{events::ScopedTelemetry, exporter::Exporter, receivers::Receiver, Processor};
mod builder;
pub use builder::PipelineBuilder;
pub struct TelemetryPipeline {
    receivers: Vec<Box<dyn Receiver>>,
    receiver_channel: mpsc::UnboundedReceiver<ScopedTelemetry>,
    processors: Vec<Box<dyn Processor>>,
    exporters: Vec<Box<dyn Exporter>>,
    failover_sender: Option<mpsc::UnboundedSender<ScopedTelemetry>>,
    cancellation_token: CancellationToken,
}
