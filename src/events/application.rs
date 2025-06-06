use super::ShutdownReason;

pub enum ApplicationEvent {
    Shutdown(ShutdownReason),
}
