use crate::{messages::ApplicationEvent, ShutdownReason};
use lambda_extension::{tracing, Error, LambdaEvent, NextEvent, Service};
use std::{future::Future, pin::Pin};
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct EventProcessor {
    application_channel: mpsc::UnboundedSender<ApplicationEvent>,
}

impl EventProcessor {
    pub fn new(application_channel: mpsc::UnboundedSender<ApplicationEvent>) -> Self {
        Self {
            application_channel,
        }
    }
}

impl Service<LambdaEvent> for EventProcessor {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    type Response = ();

    fn poll_ready(
        &mut self,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        core::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, event: LambdaEvent) -> Self::Future {
        let sender = self.application_channel.clone();

        Box::pin(async move {
            match event.next {
                NextEvent::Shutdown(e) => {
                    tracing::info!(event_type = "shutdown", ?e, "Cancelling all services");
                    let reason = ShutdownReason::from(e.shutdown_reason);
                    match sender.send(ApplicationEvent::Shutdown(reason)) {
                        Ok(_) => tracing::info!("Shutdown event sent to application channel"),
                        Err(e) => tracing::error!("Failed to send shutdown event: {}", e),
                    }
                }
                _ => (),
            }
            Ok(())
        })
    }
}
