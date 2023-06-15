use newsletter::configuration::get_configuration;
use newsletter::startup::Application;
use newsletter::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newspaper".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read config file
    let configuration = get_configuration().expect("Failed to read config file.");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
