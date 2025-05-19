use std::{future::Future, pin::Pin};

use lambda_extension::{Error, LambdaTelemetry, LambdaTelemetryRecord, Service};
use lambda_extension::{
    InitPhase, InitReportMetrics, InitType, ReportMetrics, RuntimeDoneMetrics, Span, Status,
    TraceContext,
};
use opentelemetry_proto::tonic::{
    logs::v1::ScopeLogs, metrics::v1::ScopeMetrics, trace::v1::ScopeSpans,
};
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct TelemetryApiProcessor {
    logs_consumer: Sender<ScopeLogs>,
    metrics_consumer: Sender<ScopeMetrics>,
    traces_consumer: Sender<ScopeSpans>,
}

impl TelemetryApiProcessor {
    pub fn new(
        logs_consumer: Sender<ScopeLogs>,
        metrics_consumer: Sender<ScopeMetrics>,
        traces_consumer: Sender<ScopeSpans>,
    ) -> Self {
        Self {
            logs_consumer,
            metrics_consumer,
            traces_consumer,
        }
    }

    async fn handle_init_report(
        &self,
        initialization_type: InitType,
        phase: InitPhase,
        metrics: InitReportMetrics,
        spans: Vec<Span>,
    ) -> () {
        todo!("oops");
    }

    async fn handle_runtime_done(
        &self,
        request_id: String,
        status: Status,
        error_type: Option<String>,
        metrics: Option<RuntimeDoneMetrics>,
        spans: Vec<Span>,
        tracing: Option<TraceContext>,
    ) -> () {
        todo!("oops");
    }

    async fn handle_platform_report(
        &self,
        request_id: String,
        status: Status,
        error_type: Option<String>,
        metrics: ReportMetrics,
        spans: Vec<Span>,
        tracing: Option<TraceContext>,
    ) -> () {
        todo!("oops");
    }
}

impl Service<Vec<LambdaTelemetry>> for TelemetryApiProcessor {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<(), self::Error>> + Send + Sync>>;
    type Response = ();

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        core::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Vec<LambdaTelemetry>) -> Self::Future {
        let this = self.clone();

        Box::pin(async move {
            for event in req {
                match event.record {
                    LambdaTelemetryRecord::PlatformInitReport {
                        initialization_type,
                        phase,
                        metrics,
                        spans,
                    } => {
                        this.handle_init_report(initialization_type, phase, metrics, spans)
                            .await
                    }
                    LambdaTelemetryRecord::PlatformRuntimeDone {
                        request_id,
                        status,
                        error_type,
                        metrics,
                        spans,
                        tracing,
                    } => {
                        this.handle_runtime_done(
                            request_id, status, error_type, metrics, spans, tracing,
                        )
                        .await
                    }
                    LambdaTelemetryRecord::PlatformReport {
                        request_id,
                        status,
                        error_type,
                        metrics,
                        spans,
                        tracing,
                    } => {
                        this.handle_platform_report(
                            request_id, status, error_type, metrics, spans, tracing,
                        )
                        .await
                    }
                    _ => (),
                }
            }
            Ok(())
        })
    }
}
