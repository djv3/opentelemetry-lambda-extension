use super::Exporter;
use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct JsonExporter;

impl Exporter for JsonExporter {
    fn export(&self, data: Vec<crate::events::ScopedTelemetry>) -> Result<()> {
        for item in data {
            let json = serde_json::to_string(&item).map_err(Error::SerializationError)?;
            println!("{}", json);
        }
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }

    fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
