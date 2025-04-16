use std::sync::Arc;

use opentelemetry_proto::tonic::collector::{
    logs::v1::logs_service_client::LogsServiceClient,
    metrics::v1::metrics_service_client::MetricsServiceClient,
    trace::v1::trace_service_client::TraceServiceClient,
};
use tokio::sync::Mutex;
use tonic::transport::Channel;
pub struct Exporter {
    pub logs_client: Arc<Mutex<LogsServiceClient<Channel>>>,
    pub metrics_client: Arc<Mutex<MetricsServiceClient<Channel>>>,
    pub trace_client: Arc<Mutex<TraceServiceClient<Channel>>>,
}

pub async fn create_exporter(
    endpoint: String,
) -> Result<Arc<Exporter>, Box<dyn std::error::Error>> {
    let (logs_client, metrics_client, trace_client) = tokio::try_join!(
        create_log_client(endpoint.clone()),
        create_metrics_client(endpoint.clone()),
        create_trace_client(endpoint.clone())
    )?;

    let exporter = Exporter {
        logs_client: Arc::new(Mutex::new(logs_client)),
        metrics_client: Arc::new(Mutex::new(metrics_client)),
        trace_client: Arc::new(Mutex::new(trace_client)),
    };

    Ok(Arc::new(exporter))
}

async fn create_log_client(
    addr: String,
) -> Result<LogsServiceClient<Channel>, tonic::transport::Error> {
    let channel = tonic::transport::Channel::from_shared(addr)
        .unwrap()
        .connect()
        .await?;

    Ok(LogsServiceClient::new(channel))
}

async fn create_metrics_client(
    addr: String,
) -> Result<MetricsServiceClient<Channel>, tonic::transport::Error> {
    let channel = tonic::transport::Channel::from_shared(addr)
        .unwrap()
        .connect()
        .await?;

    Ok(MetricsServiceClient::new(channel))
}

async fn create_trace_client(
    addr: String,
) -> Result<TraceServiceClient<Channel>, tonic::transport::Error> {
    let channel = tonic::transport::Channel::from_shared(addr)
        .unwrap()
        .connect()
        .await?;

    Ok(TraceServiceClient::new(channel))
}
