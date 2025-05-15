use crate::shutdown_reason::{MetricRecorder, ShutdownReason};
use lambda_extension::{tracing, Error, LambdaEvent, NextEvent, Service};
use std::{future::Future, pin::Pin};
use tokio_util::sync::CancellationToken;

pub struct EventProcessor<T: MetricRecorder + Clone> {
    cancellation_token: CancellationToken,
    shutdown_counter: T,
}

impl<T: MetricRecorder + Clone> EventProcessor<T> {
    pub fn new(cancellation_token: CancellationToken, shutdown_counter: T) -> Self {
        Self {
            cancellation_token,
            shutdown_counter,
        }
    }
}

impl<T: MetricRecorder + Clone + Send + 'static> Service<LambdaEvent> for EventProcessor<T> {
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
        let token = self.cancellation_token.clone();
        let shutdown_counter = self.shutdown_counter.clone();

        Box::pin(async move {
            match event.next {
                NextEvent::Shutdown(e) => {
                    tracing::info!(event_type = "shutdown", ?e, "Cancelling all services");
                    let reason = ShutdownReason::from(e.shutdown_reason);
                    reason.emit_metric(&shutdown_counter);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shutdown_reason::MockMetricRecorderMock;
    use lambda_extension::{InvokeEvent, LambdaEvent, NextEvent, ShutdownEvent, Tracing};
    use mockall::predicate;
    use opentelemetry::KeyValue;

    #[tokio::test]
    async fn test_shutdown_emits_metric() {
        let mut mock_counter = MockMetricRecorderMock::new();

        mock_counter.expect_clone().return_once(move || {
            let mut clone = MockMetricRecorderMock::new();
            clone
                .expect_add()
                .with(
                    predicate::eq(1),
                    predicate::function(|attrs: &[KeyValue]| {
                        attrs.len() == 1
                            && attrs[0].key.as_str() == "reason"
                            && attrs[0].value.as_str() == "spindown"
                    }),
                )
                .times(1)
                .return_const(());
            clone
        });

        let token = CancellationToken::new();
        let mut processor = EventProcessor::new(token.clone(), mock_counter);

        let shutdown_event = LambdaEvent {
            next: NextEvent::Shutdown(ShutdownEvent {
                shutdown_reason: "SPINDOWN".to_string(),
                deadline_ms: 2000,
            }),
        };

        let result = processor.call(shutdown_event).await;

        assert!(result.is_ok());
        assert!(token.is_cancelled());
    }

    #[tokio::test]
    async fn test_invoke_does_not_emit_metric() {
        let mut mock_counter = MockMetricRecorderMock::new();

        mock_counter
            .expect_clone()
            .return_once(move || MockMetricRecorderMock::new());

        let token = CancellationToken::new();
        let mut processor = EventProcessor::new(token.clone(), mock_counter);

        let invoke_event = LambdaEvent {
            next: NextEvent::Invoke(InvokeEvent {
                request_id: "test-id".to_string(),
                deadline_ms: 1000,
                invoked_function_arn: "test-arn".to_string(),
                tracing: Tracing::default(),
            }),
        };

        let result = processor.call(invoke_event).await;

        assert!(result.is_ok());
        assert!(!token.is_cancelled());
    }
}
