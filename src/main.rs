use lambda_extension::tracing;
use opentelemetry_lambda_extension::{exporter::JsonExporter, PipelineBuilder, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init_default_subscriber();
    let ct = tokio_util::sync::CancellationToken::new();

    let json = JsonExporter;

    let (pipeline, _) = PipelineBuilder::new()
        .with_receivers(vec![])
        .with_processors(vec![])
        .with_exporters(vec![])
        .with_cancellation_token(ct.clone())
        .build();

    Ok(())
}
