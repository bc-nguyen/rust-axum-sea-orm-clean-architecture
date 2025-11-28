use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_stdout as otel_stdout;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn register() {
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(otel_stdout::SpanExporter::default())
        .build();

    let tracer = provider.tracer(format!("{}=debug", env!("CARGO_CRATE_NAME")));

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(telemetry)
        .init();
}
