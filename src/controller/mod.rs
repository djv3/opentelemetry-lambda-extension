use crate::{events::ApplicationEvent, Error, Result};
use lambda_extension::tracing;
use opentelemetry_proto::tonic::resource::v1::Resource;
use tokio::sync::{mpsc::UnboundedReceiver, OnceCell};
use tokio_util::sync::CancellationToken;

pub struct ApplicationController {
    resource: OnceCell<Resource>,
    application_event_channel: UnboundedReceiver<ApplicationEvent>,
    cancellation_token: CancellationToken,
}

impl ApplicationController {
    pub fn set_resource(&self, resource: Resource) -> Result<()> {
        match self.resource.set(resource.clone()) {
            Ok(_) => {
                tracing::info!("Resource set successfully");
                Ok(())
            }
            Err(e) => {
                tracing::debug!(resource = ?resource, "Failed to set resource");
                Err(Error::SetResourceError(e))
            }
        }
    }

    pub fn get_resource(&self) -> Result<Resource> {
        self.resource
            .get()
            .cloned()
            .ok_or_else(|| Error::GetResourceError)
    }

    pub fn new(application_event_channel: UnboundedReceiver<ApplicationEvent>) -> Self {
        Self {
            resource: OnceCell::new(),
            application_event_channel,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        tracing::debug!("Starting ApplicationController");

        loop {
            match self.application_event_channel.recv().await {
                Some(event) => {
                    tracing::debug!(event = ?event, "Received application event");
                    match event {
                        ApplicationEvent::Shutdown(shutdown_reason) => {
                            tracing::info!(reason = ?shutdown_reason, "Received shutdown event");
                            self.cancellation_token.cancel();
                            self.cancellation_token.cancelled().await;
                            tracing::info!("ApplicationController shutdown complete, all pipelines stopped successfully");
                            break;
                        }
                        event => {
                            todo!("Handle application event: {:?}", event)
                        }
                    }
                }
                None => {
                    tracing::warn!("Application event channel closed");
                    break;
                }
            }
        }

        Ok(())
    }
}
