use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use newsletter::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newspaper".into(), "info".into());
    init_subscriber(subscriber);

    // Panic if we can't read config file
    let configuration = get_configuration().expect("Failed to read config file.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind address.");
    run(listener, connection_pool)?.await
}
