use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{events::ScopedTelemetry, exporter::Exporter, receivers::Receiver, Processor, Result};
mod builder;
pub use builder::PipelineBuilder;

#[derive(Debug)]
pub struct Pipeline {
    receivers: Vec<Box<dyn Receiver>>,
    receiver_channel: mpsc::UnboundedReceiver<ScopedTelemetry>,
    processors: Vec<Box<dyn Processor>>,
    exporters: Vec<Box<dyn Exporter>>,
    failover_sender: Option<mpsc::UnboundedSender<ScopedTelemetry>>,
    cancellation_token: CancellationToken,
}

impl Pipeline {
    pub async fn start(&self) -> Result<()> {
        todo!("Implement the start logic for the pipeline");
    }
}
