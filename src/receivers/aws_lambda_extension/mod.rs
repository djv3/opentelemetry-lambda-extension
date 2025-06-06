use crate::{
    events::{ApplicationEvent, ScopedTelemetry},
    Error, Result,
};
use event::EventProcessor;
use lambda_extension::{tracing, Extension, SharedService};
use telemetry_api::TelemetryApiProcessor;
use tokio_util::sync::CancellationToken;

use super::Receiver;
use async_trait::async_trait;
use tokio::sync::mpsc;

mod event;
mod telemetry_api;

pub struct AwsLambdaExtensionReceiver {
    cancellation_token: CancellationToken,
    application_channel: mpsc::UnboundedSender<ApplicationEvent>,
    pipeline_channel: mpsc::UnboundedSender<ScopedTelemetry>,
}

async fn run_lambda_extension(ep: EventProcessor, tp: TelemetryApiProcessor) -> Result<()> {
    Extension::new()
        .with_events_processor(ep)
        .with_telemetry_processor(SharedService::new(tp))
        .run()
        .await
        .map_err(Error::ExtensionReceiver)
}

#[async_trait]
impl Receiver for AwsLambdaExtensionReceiver {
    async fn start(&self) -> Result<()> {
        let ep = EventProcessor::new(self.application_channel.clone());
        let tp = TelemetryApiProcessor::new(self.pipeline_channel.clone());

        tokio::select! {
            _ = run_lambda_extension(ep, tp) => {
                tracing::error!("The extension receiver finished before the component was shutdown properly");
                Ok(())
            }
            _ = self.cancellation_token.cancelled() => {
                tracing::info!("AwsLambdaExtensionReceiver was shutdown");
                Ok(())
            }
        }
    }

    async fn stop(&self) -> Result<()> {
        self.cancellation_token.cancel();
        self.cancellation_token.cancelled().await;
        Ok(())
    }
}
