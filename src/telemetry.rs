use crate::configuration::Settings;
use tracing::Subscriber;
use tracing_log::LogTracer;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber(
    configuration: &Settings,
) -> impl Subscriber + Send + Sync + for<'span> LookupSpan<'span> {
    let default_logging_level = if configuration.debug { "info" } else { "warn" };
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_logging_level));
    Registry::default().with(env_filter)
}

pub fn init_tracing(subscriber: impl Subscriber + Send + Sync + for<'span> LookupSpan<'span>) {
    LogTracer::init().expect("Failed to init LogTracer");
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set the default tracing subscriber");
}
