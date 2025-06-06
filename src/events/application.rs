use super::ShutdownReason;

#[derive(Debug, Clone, PartialEq)]
pub enum ApplicationEvent {
    Shutdown(ShutdownReason),
    Sentinel,
}
