use super::TelemetryPipeline;
use crate::{events::ScopedTelemetry, exporter::Exporter, Processor, Receiver, Result};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub struct NoReceiver;
pub struct HasReceiver;
pub struct NoProcessors;
pub struct HasProcessors;
pub struct NoExporter;
pub struct HasExporter;
pub struct NoFailoverChannel;
pub struct HasFailoverChannel;
pub struct NoCancellationToken;
pub struct HasCancellationToken;

pub struct PipelineBuilder<R, P, E, FC, CT> {
    receivers: Option<Vec<Box<dyn Receiver>>>,
    processors: Option<Vec<Box<dyn Processor>>>,
    exporters: Option<Vec<Box<dyn Exporter>>>,
    failover_sender: Option<mpsc::UnboundedSender<ScopedTelemetry>>,
    cancellation_token: Option<CancellationToken>,
    _phantom: std::marker::PhantomData<(R, P, E, FC, CT)>,
}

impl PipelineBuilder<NoReceiver, NoProcessors, NoExporter, NoFailoverChannel, NoCancellationToken> {
    pub fn new() -> Self {
        Self {
            receivers: None,
            processors: None,
            exporters: None,
            failover_sender: None,
            cancellation_token: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<P, E, FC, CT> PipelineBuilder<NoReceiver, P, E, FC, CT> {
    pub fn with_receivers(
        self,
        receivers: Vec<Box<dyn Receiver>>,
    ) -> PipelineBuilder<HasReceiver, P, E, FC, CT> {
        PipelineBuilder {
            receivers: Some(receivers),
            processors: self.processors,
            exporters: self.exporters,
            failover_sender: self.failover_sender,
            cancellation_token: self.cancellation_token,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<R, E, FC, CT> PipelineBuilder<R, NoProcessors, E, FC, CT> {
    pub fn with_processors(
        self,
        processors: Vec<Box<dyn Processor>>,
    ) -> PipelineBuilder<R, HasProcessors, E, FC, CT> {
        PipelineBuilder {
            receivers: self.receivers,
            processors: Some(processors),
            exporters: self.exporters,
            failover_sender: self.failover_sender,
            cancellation_token: self.cancellation_token,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<R, P, FC, CT> PipelineBuilder<R, P, NoExporter, FC, CT> {
    pub fn with_exporters(
        self,
        exporters: Vec<Box<dyn Exporter>>,
    ) -> PipelineBuilder<R, P, HasExporter, FC, CT> {
        PipelineBuilder {
            receivers: self.receivers,
            processors: self.processors,
            exporters: Some(exporters),
            failover_sender: self.failover_sender,
            cancellation_token: self.cancellation_token,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<R, P, E, CT> PipelineBuilder<R, P, E, NoFailoverChannel, CT> {
    pub fn with_failover_channel(
        self,
        failover_sender: mpsc::UnboundedSender<ScopedTelemetry>,
    ) -> PipelineBuilder<R, P, E, HasFailoverChannel, CT> {
        PipelineBuilder {
            receivers: self.receivers,
            processors: self.processors,
            exporters: self.exporters,
            failover_sender: Some(failover_sender),
            cancellation_token: self.cancellation_token,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<R, P, E, FC> PipelineBuilder<R, P, E, FC, NoCancellationToken> {
    pub fn with_cancellation_token(
        self,
        cancellation_token: CancellationToken,
    ) -> PipelineBuilder<R, P, E, FC, HasCancellationToken> {
        PipelineBuilder {
            receivers: self.receivers,
            processors: self.processors,
            exporters: self.exporters,
            failover_sender: self.failover_sender,
            cancellation_token: Some(cancellation_token),
            _phantom: std::marker::PhantomData,
        }
    }
}

// Build method - only available when all required components are set
// Required: HasReceiverChannel, HasProcessors, HasExporter, HasCancellationToken
// Optional: R (receiver), FC (failover channel)
impl<R, FC> PipelineBuilder<R, HasProcessors, HasExporter, FC, HasCancellationToken> {
    pub fn build(self) -> (TelemetryPipeline, mpsc::UnboundedSender<ScopedTelemetry>) {
        let (sender_channel, receiver_channel) = mpsc::unbounded_channel::<ScopedTelemetry>();
        (
            TelemetryPipeline {
                receivers: self
                    .receivers
                    .expect("Receivers should be set, this is a bug in the type system"),
                receiver_channel,
                processors: self
                    .processors
                    .expect("Processors should be set, this is a bug in the type system"),
                exporters: self
                    .exporters
                    .expect("Exporter should be set, this is a bug in the type system"),
                failover_sender: self.failover_sender,
                cancellation_token: self
                    .cancellation_token
                    .expect("Cancellation token should be set, this is a bug in the type system"),
            },
            sender_channel,
        )
    }
}
