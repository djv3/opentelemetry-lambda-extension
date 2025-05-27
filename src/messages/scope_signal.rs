use opentelemetry_proto::tonic::{
    logs::v1::ScopeLogs, metrics::v1::ScopeMetrics, trace::v1::ScopeSpans,
};

pub enum ScopedTelemetry {
    Logs(ScopeLogs),
    Metrics(ScopeMetrics),
    Spans(ScopeSpans),
}
