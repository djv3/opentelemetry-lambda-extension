use crate::{events::ScopedTelemetry, Result};
use std::fmt::Debug;
mod json;
pub use json::JsonExporter;

pub trait Exporter: Send + Sync + Debug {
    /// Export telemetry data.
    fn export(&self, data: Vec<ScopedTelemetry>) -> Result<()>;

    /// Flush any buffered data.
    fn flush(&self) -> Result<()>;

    /// Shutdown the exporter, releasing any resources.
    fn shutdown(&self) -> Result<()>;
}
