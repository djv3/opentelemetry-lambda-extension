use super::TelemetryPipeline;
use crate::{exporter::Exporter, messages::ScopedTelemetry, Processor, Receiver, Result};
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
    receiver: Option<Box<dyn Receiver>>,
    receiver_channel: Option<mpsc::UnboundedReceiver<ScopedTelemetry>>,
    processors: Option<Vec<Box<dyn Processor>>>,
    exporter: Option<Box<dyn Exporter>>,
    failover_sender: Option<mpsc::UnboundedSender<ScopedTelemetry>>,
    cancellation_token: Option<CancellationToken>,
    _phantom: std::marker::PhantomData<(R, P, E, FC, CT)>,
}

impl PipelineBuilder<NoReceiver, NoProcessors, NoExporter, NoFailoverChannel, NoCancellationToken> {
    pub fn new() -> Self {
        Self {
            receiver: None,
            receiver_channel: None,
            processors: None,
            exporter: None,
            failover_sender: None,
            cancellation_token: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<P, E, FC, CT> PipelineBuilder<NoReceiver, P, E, FC, CT> {
    pub fn with_receiver<T: Receiver + 'static>(
        mut self,
        receiver: T,
    ) -> PipelineBuilder<HasReceiver, P, E, FC, CT> {
        PipelineBuilder {
            receiver: Some(Box::new(receiver)),
            receiver_channel: self.receiver_channel,
            processors: self.processors,
            exporter: self.exporter,
            failover_sender: self.failover_sender,
            cancellation_token: self.cancellation_token,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<R, E, FC, CT> PipelineBuilder<R, NoProcessors, E, FC, CT> {
    pub fn with_processors<F: Processor + 'static>(
        mut self,
        processors: Vec<F>,
    ) -> PipelineBuilder<R, HasProcessors, E, FC, CT> {
        let mut boxed_processors: Vec<Box<dyn Processor>> = Vec::new();
        for processor in processors {
            boxed_processors.push(Box::new(processor));
        }

        PipelineBuilder {
            receiver: self.receiver,
            receiver_channel: self.receiver_channel,
            processors: Some(boxed_processors),
            exporter: self.exporter,
            failover_sender: self.failover_sender,
            cancellation_token: self.cancellation_token,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<R, P, FC, CT> PipelineBuilder<R, P, NoExporter, FC, CT> {
    pub fn with_exporter<T: Exporter + 'static>(
        mut self,
        exporter: T,
    ) -> PipelineBuilder<R, P, HasExporter, FC, CT> {
        PipelineBuilder {
            receiver: self.receiver,
            receiver_channel: self.receiver_channel,
            processors: self.processors,
            exporter: Some(Box::new(exporter)),
            failover_sender: self.failover_sender,
            cancellation_token: self.cancellation_token,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<R, P, E, CT> PipelineBuilder<R, P, E, NoFailoverChannel, CT> {
    pub fn with_failover_channel(
        mut self,
        failover_sender: mpsc::UnboundedSender<ScopedTelemetry>,
    ) -> PipelineBuilder<R, P, E, HasFailoverChannel, CT> {
        PipelineBuilder {
            receiver: self.receiver,
            receiver_channel: self.receiver_channel,
            processors: self.processors,
            exporter: self.exporter,
            failover_sender: Some(failover_sender),
            cancellation_token: self.cancellation_token,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<R, P, E, FC> PipelineBuilder<R, P, E, FC, NoCancellationToken> {
    pub fn with_cancellation_token(
        mut self,
        cancellation_token: CancellationToken,
    ) -> PipelineBuilder<R, P, E, FC, HasCancellationToken> {
        PipelineBuilder {
            receiver: self.receiver,
            receiver_channel: self.receiver_channel,
            processors: self.processors,
            exporter: self.exporter,
            failover_sender: self.failover_sender,
            cancellation_token: Some(cancellation_token),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl TelemetryPipeline {
    pub fn run(self) -> Result<()> {
        Ok(())
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
                receiver: self.receiver,
                receiver_channel,
                processors: self
                    .processors
                    .expect("Processors should be set, this is a bug in the type system"),
                exporter: self
                    .exporter
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
