mod aws_lambda_extension;
mod otlp_http;

use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Receiver: Send + Sync {
    async fn start(&self) -> Result<()>;
}
