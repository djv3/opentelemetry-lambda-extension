use std::{future::Future, pin::Pin};

use lambda_extension::{Error, LambdaTelemetry, Service};
use tokio::sync::mpsc;

use crate::events::ScopedTelemetry;

#[derive(Clone)]
pub struct TelemetryApiProcessor {
    pipeline_channel: mpsc::UnboundedSender<ScopedTelemetry>,
}

impl TelemetryApiProcessor {
    pub fn new(pipeline_channel: mpsc::UnboundedSender<ScopedTelemetry>) -> Self {
        Self { pipeline_channel }
    }
}

impl Service<Vec<LambdaTelemetry>> for TelemetryApiProcessor {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<(), self::Error>> + Send + Sync>>;
    type Response = ();

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        core::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Vec<LambdaTelemetry>) -> Self::Future {
        let pipeline_channel = self.pipeline_channel.clone();

        Box::pin(async move {
            for event in req {
                todo!("Process telemetry event: {:?}", event);
            }
            Ok(())
        })
    }
}
