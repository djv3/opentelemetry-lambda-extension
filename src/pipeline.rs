use opentelemetry_proto::tonic::{
    collector::{
        logs::v1::ExportLogsServiceRequest, metrics::v1::ExportMetricsServiceRequest,
        trace::v1::ExportTraceServiceRequest,
    },
    logs::v1::ScopeLogs,
    metrics::v1::ScopeMetrics,
    resource::v1::Resource,
    trace::v1::ScopeSpans,
};
use tokio::sync::mpsc::Receiver;
use async_trait::async_trait;

#[async_trait]
pub trait Exporter {
    async fn export_logs(&self, request: ExportLogsServiceRequest);
    async fn export_metrics(&self, request: ExportMetricsServiceRequest);
    async fn export_spans(&self, request: ExportTraceServiceRequest);
}

pub struct Pipeline {
    pub resource: Option<Resource>,
    pub scope_logs_channel: Receiver<ScopeLogs>,
    pub scope_metrics_channel: Receiver<ScopeMetrics>,
    pub scope_spans_channel: Receiver<ScopeSpans>,
    pub logs_request_channel: Receiver<ExportLogsServiceRequest>,
    pub metrics_request_channel: Receiver<ExportMetricsServiceRequest>,
    pub spans_request_channel: Receiver<ExportTraceServiceRequest>,
    pub scope_logs: Vec<ScopeLogs>,
    pub scope_metrics: Vec<ScopeMetrics>,
    pub scope_spans: Vec<ScopeSpans>,
}

impl Pipeline {
    pub fn new(
        scope_logs_channel: Receiver<ScopeLogs>,
        scope_metrics_channel: Receiver<ScopeMetrics>,
        scope_spans_channel: Receiver<ScopeSpans>,
        logs_request_channel: Receiver<ExportLogsServiceRequest>,
        metrics_request_channel: Receiver<ExportMetricsServiceRequest>,
        spans_request_channel: Receiver<ExportTraceServiceRequest>,
    ) -> Self {
        Self {
            scope_logs_channel,
            scope_metrics_channel,
            scope_spans_channel,
            logs_request_channel,
            metrics_request_channel,
            spans_request_channel,
            scope_logs: Vec::new(),
            scope_metrics: Vec::new(),
            scope_spans: Vec::new(),
            resource: None,
        }
    }

    pub async fn run(&mut self) {
        while let Some(scope_log) = self.scope_logs_channel.recv().await {
            self.scope_logs.push(scope_log);
        }

        while let Some(scope_metric) = self.scope_metrics_channel.recv().await {
            self.scope_metrics.push(scope_metric);
        }

        while let Some(scope_span) = self.scope_spans_channel.recv().await {
            self.scope_spans.push(scope_span);
        }

        while let Some(lr) = self.logs_request_channel.recv().await {
            for rl in lr.resource_logs {
                if self.resource.is_none() && rl.resource.is_some() {
                    self.resource = rl.resource;
                }
                for sl in rl.scope_logs {
                    self.scope_logs.push(sl);
                }
            }
        }

        while let Some(mr) = self.metrics_request_channel.recv().await {
            for rm in mr.resource_metrics {
                if self.resource.is_none() && rm.resource.is_some() {
                    self.resource = rm.resource;
                }
                for sm in rm.scope_metrics {
                    self.scope_metrics.push(sm);
                }
            }
        }

        while let Some(sr) = self.spans_request_channel.recv().await {
            for rs in sr.resource_spans {
                if self.resource.is_none() && rs.resource.is_some() {
                    self.resource = rs.resource;
                }
                for ss in rs.scope_spans {
                    self.scope_spans.push(ss);
                }
            }
        }
    }
}
