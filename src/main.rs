use newsletter::configuration::get_configuration;
use newsletter::database::configure_db_if_not_exists;
use newsletter::startup::run;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newspaper".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read config file
    let configuration = get_configuration().expect("Failed to read config file.");

    configure_db_if_not_exists(&configuration.database).await;
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind address.");
    run(listener, connection_pool)?.await
}
