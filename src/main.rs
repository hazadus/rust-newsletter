use newsletter::configuration::get_configuration;
use newsletter::database::configure_db_if_not_exists;
use newsletter::startup::build;
use newsletter::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newspaper".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read config file
    let configuration = get_configuration().expect("Failed to read config file.");

    configure_db_if_not_exists(&configuration.database).await;

    let server = build(configuration).await?;
    server.await?;

    Ok(())
}
