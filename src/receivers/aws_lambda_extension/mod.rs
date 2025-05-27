use crate::{
    messages::{ApplicationEvent, ScopedTelemetry},
    Error, Result,
};
use event::EventProcessor;
use lambda_extension::{Extension, SharedService};
use telemetry_api::TelemetryApiProcessor;

use super::Receiver;
use async_trait::async_trait;
use tokio::sync::mpsc;

mod event;
mod telemetry_api;

pub struct AwsLambdaExtensionReceiver {
    application_channel: mpsc::UnboundedSender<ApplicationEvent>,
    pipeline_channel: mpsc::UnboundedSender<ScopedTelemetry>,
}

#[async_trait]
impl Receiver for AwsLambdaExtensionReceiver {
    async fn start(&self) -> Result<()> {
        let ep = EventProcessor::new(self.application_channel.clone());
        let tp = TelemetryApiProcessor::new(self.pipeline_channel.clone());

        Extension::new()
            .with_events_processor(ep)
            .with_telemetry_processor(SharedService::new(tp))
            .run()
            .await
            .map_err(Error::ExtensionReceiver)?;

        Ok(())
    }
}
