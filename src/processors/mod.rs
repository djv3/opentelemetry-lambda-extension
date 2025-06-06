use crate::Result;
use std::fmt::Debug;

pub trait Processor: Send + Sync + Debug {
    /// Process telemetry data.
    fn process(&self, data: Vec<u8>) -> Result<()>;
}
