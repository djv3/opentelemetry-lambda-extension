use crate::{Result, messages::ScopedTelemetry};

mod json;
pub use json::JsonExporter;
pub trait Exporter: Send + Sync {
    /// Export telemetry data.
    fn export(&self, data: Vec<ScopedTelemetry>) -> Result<()>;

    /// Flush any buffered data.
    fn flush(&self) -> Result<()>;

    /// Shutdown the exporter, releasing any resources.
    fn shutdown(&self) -> Result<()>;
}
