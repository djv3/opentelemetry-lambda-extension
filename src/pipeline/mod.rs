use crate::receivers::Receiver;

struct TelemetryPipeline {
    receivers: Vec<Box<dyn Receiver>>,
}
