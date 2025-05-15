use opentelemetry::{metrics::Counter, KeyValue};

#[cfg(test)]
use mockall::{mock, predicate};

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

pub trait MetricRecorder {
    fn add(&self, value: u64, attributes: &[KeyValue]);
}

impl<T> MetricRecorder for Counter<T>
where
    T: From<u64> + Send + Sync,
{
    fn add(&self, value: u64, attributes: &[KeyValue]) {
        self.add(value.into(), attributes);
    }
}

#[cfg(test)]
mock! {
    pub MetricRecorderMock {}
    impl Clone for MetricRecorderMock {
        fn clone(&self) -> Self;
    }
    impl MetricRecorder for MetricRecorderMock {
        fn add(&self, value: u64, attributes: &[KeyValue]);
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

    pub fn emit_metric<T: MetricRecorder>(&self, counter: &T) {
        let reason = self.reason_string();
        counter.add(1, &[KeyValue::new("reason", reason)]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spindown_emits_correct_metric() {
        let mut mock_counter = MockMetricRecorderMock::new();

        mock_counter
            .expect_add()
            .with(
                predicate::eq(1),
                predicate::function(|attrs: &[KeyValue]| {
                    attrs.len() == 1
                        && attrs[0].key.as_str() == "reason"
                        && attrs[0].value.as_str() == "spindown"
                }),
            )
            .times(1)
            .return_const(());

        let reason = ShutdownReason::Spindown;
        reason.emit_metric(&mock_counter);
    }

    #[test]
    fn test_timeout_emits_correct_metric() {
        let mut mock_counter = MockMetricRecorderMock::new();

        mock_counter
            .expect_add()
            .with(
                predicate::eq(1),
                predicate::function(|attrs: &[KeyValue]| {
                    attrs.len() == 1
                        && attrs[0].key.as_str() == "reason"
                        && attrs[0].value.as_str() == "timeout"
                }),
            )
            .times(1)
            .return_const(());

        let reason = ShutdownReason::Timeout;
        reason.emit_metric(&mock_counter);
    }

    #[test]
    fn test_failure_emits_correct_metric() {
        let mut mock_counter = MockMetricRecorderMock::new();

        mock_counter
            .expect_add()
            .with(
                predicate::eq(1),
                predicate::function(|attrs: &[KeyValue]| {
                    attrs.len() == 1
                        && attrs[0].key.as_str() == "reason"
                        && attrs[0].value.as_str() == "failure"
                }),
            )
            .times(1)
            .return_const(());

        let reason = ShutdownReason::Failure;
        reason.emit_metric(&mock_counter);
    }

    #[test]
    fn test_other_emits_correct_metric() {
        let mut mock_counter = MockMetricRecorderMock::new();
        let custom_reason = "custom_reason";

        mock_counter
            .expect_add()
            .with(
                predicate::eq(1),
                predicate::function(move |attrs: &[KeyValue]| {
                    attrs.len() == 1
                        && attrs[0].key.as_str() == "reason"
                        && attrs[0].value.as_str() == custom_reason
                }),
            )
            .times(1)
            .return_const(());

        let reason = ShutdownReason::Other(custom_reason.to_string());
        reason.emit_metric(&mock_counter);
    }

    #[test]
    fn test_string_conversion() {
        assert!(matches!(
            ShutdownReason::from("SPINDOWN".to_string()),
            ShutdownReason::Spindown
        ));
        assert!(matches!(
            ShutdownReason::from("TIMEOUT".to_string()),
            ShutdownReason::Timeout
        ));
        assert!(matches!(
            ShutdownReason::from("FAILURE".to_string()),
            ShutdownReason::Failure
        ));

        let other = ShutdownReason::from("SOMETHING_ELSE".to_string());
        if let ShutdownReason::Other(reason) = other {
            assert_eq!(reason, "SOMETHING_ELSE");
        } else {
            panic!("Expected Other variant");
        }
    }
}
