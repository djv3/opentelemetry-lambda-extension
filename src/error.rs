use opentelemetry_proto::tonic::resource::v1::Resource;
use thiserror::Error as ThisError;
use tokio::sync::SetError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Failed to run the AWS Lambda Extension Receiver")]
    ExtensionReceiver(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Failed to serialize JSON")]
    SerializationError(#[source] serde_json::Error),
    #[error("Failed to set resource")]
    SetResourceError(#[from] SetError<Resource>),
    #[error("Failed to get resource")]
    GetResourceError,
}

pub type Result<T> = std::result::Result<T, Error>;
