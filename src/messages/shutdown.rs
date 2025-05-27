pub enum ShutdownReason {
    Spindown,
    Timeout,
    Failure,
    Other(String),
}

impl From<String> for ShutdownReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "SPINDOWN" => ShutdownReason::Spindown,
            "TIMEOUT" => ShutdownReason::Timeout,
            "FAILURE" => ShutdownReason::Failure,
            r => ShutdownReason::Other(r.to_string()),
        }
    }
}

impl ShutdownReason {
    fn reason_string(&self) -> String {
        match self {
            ShutdownReason::Spindown => "spindown".to_string(),
            ShutdownReason::Timeout => "timeout".to_string(),
            ShutdownReason::Failure => "failure".to_string(),
            ShutdownReason::Other(reason) => reason.clone(),
        }
    }
}
