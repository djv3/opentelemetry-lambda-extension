use std::{future::Future, pin::Pin};

use lambda_extension::{Error, LambdaTelemetry, Service};
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct TelemetryApiProcessor {
    processor: Sender<LambdaTelemetry>,
}

impl TelemetryApiProcessor {
    pub fn new(processor: Sender<LambdaTelemetry>) -> Self {
        Self { processor }
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
        let processor = self.processor.clone();

        Box::pin(async move {
            for event in req {
                if let Err(e) = processor.send(event).await {
                    return Err(Error::from(format!(
                        "Failed to send telemetry event: {}",
                        e
                    )));
                }
            }
            Ok(())
        })
    }
}
