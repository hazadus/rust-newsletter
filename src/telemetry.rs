use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

/// Compose multiple layers into a `tracing`'s subscriber.
pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    // We are falling back to printing all spans at info-level or above
    // if the `RUST_LOG` environment variable has not been set.
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name,
        // Output the formatted spans to `stdout`:
        std::io::stdout,
    );

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register subscriber as global default to process span data.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger.");
    // Specify that our `subscriber` should be used to process spans
    set_global_default(subscriber).expect("Failed to set subscriber.");
}