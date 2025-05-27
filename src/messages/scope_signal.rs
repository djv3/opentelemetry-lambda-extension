use opentelemetry_proto::tonic::{
    logs::v1::ScopeLogs, metrics::v1::ScopeMetrics, trace::v1::ScopeSpans,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScopedTelemetry {
    Logs(ScopeLogs),
    Metrics(ScopeMetrics),
    Spans(ScopeSpans),
}
