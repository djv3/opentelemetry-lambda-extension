use crate::Result;

pub trait Processor: Send + Sync {
    /// Process telemetry data.
    fn process(&self, data: Vec<u8>) -> Result<()>;
}
