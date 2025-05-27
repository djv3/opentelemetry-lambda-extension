use lambda_extension::tracing;
use opentelemetry_lambda_extension::{exporter::JsonExporter, PipelineBuilder, Result, Processor};

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init_default_subscriber();
    let ct = tokio_util::sync::CancellationToken::new();

    let json = JsonExporter;

    let cw_processors: Vec<Box<dyn Processor>> = vec![];

    let (cloudwatch, cw_sender) = PipelineBuilder::new()
        .with_cancellation_token(ct.clone())
        .with_exporter(json.clone())
        .with_processors(vec![])
        .build();

    let (pipeline, _) = PipelineBuilder::new()
        .with_processors(vec![])
        .with_exporter(json)
        .with_failover_channel(cw_sender)
        .with_cancellation_token(ct.clone())
        .build();

    Ok(())
}
