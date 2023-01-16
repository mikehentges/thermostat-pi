use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use opentelemetry::sdk::trace::RandomIdGenerator;
use opentelemetry::sdk::{trace, Resource};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            trace::config()
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "push-temp-local",
                )])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter().tonic().with_env(),
            // .http()
            // .with_endpoint("http://192.168.1.35:4318/v1/traces"), // .with_protocol(Protocol::HttpBinary),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("failed to initialize otel tracing pipeline");

    let opentelemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(opentelemetry_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
