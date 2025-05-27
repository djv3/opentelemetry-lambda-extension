use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Failed to run the AWS Lambda Extension Receiver")]
    ExtensionReceiver(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Failed to serialize JSON")]
    SerializationError(#[source] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
