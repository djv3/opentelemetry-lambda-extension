use lambda_extension::{tracing, Error, LambdaEvent, NextEvent, Service};
use std::{future::Future, pin::Pin};
use tokio_util::sync::CancellationToken;

pub struct EventProcessor {
    cancellation_token: CancellationToken,
}

impl EventProcessor {
    pub fn new(cancellation_token: CancellationToken) -> Self {
        Self { cancellation_token }
    }
}

impl Service<LambdaEvent> for EventProcessor {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    type Response = ();

    fn poll_ready(
        &mut self,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        core::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, event: LambdaEvent) -> Self::Future {
        let token = self.cancellation_token.clone();

        Box::pin(async move {
            match event.next {
                NextEvent::Shutdown(e) => {
                    tracing::info!(event_type = "shutdown", ?e, "Cancelling all services");
                    token.cancel();
                }
                NextEvent::Invoke(e) => {
                    tracing::info!(event_type = "invoke", ?e, "Function invoked");
                }
            }
            Ok(())
        })
    }
}
