use lambda_extension::{Error, LambdaTelemetry, LambdaTelemetryRecord};

/// Process telemetry from the Lambda extension API.
///
/// This function is called when the Lambda extension receives telemetry from the Lambda API.
pub(crate) async fn telemetry_processor(events: Vec<LambdaTelemetry>) -> Result<(), Error> {
    for event in events {
        match event.record {
            LambdaTelemetryRecord::PlatformRuntimeDone {
                request_id,
                status,
                error_type,
                metrics,
                spans,
                tracing,
            } => todo!("trigger exporting here initially"),
            _ignore_other => {}
        }
    }
    Ok(())
}
