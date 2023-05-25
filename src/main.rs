use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read config file
    let configuration = get_configuration().expect("Failed to read config file.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind address.");
    run(listener)?.await
}
