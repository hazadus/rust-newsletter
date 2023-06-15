//! Contains `build()` and `run()` functions used to create HTTP `Server` instance.
use crate::configuration::{DatabaseSettings, Settings};
use crate::database::configure_db_if_not_exists;
use crate::email_client::EmailClient;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Type to hold the newly built server and its port
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    /// Configure database, create DB connection pool, create the server.
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        configure_db_if_not_exists(&configuration.database).await;

        let connection_pool = get_connection_pool(&configuration.database);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address.");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, email_client)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

/// Helper function to create DB connection pool.
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .connect_lazy_with(configuration.with_db())
}

/// Initialize and return HTTP `Server` instance, with `TracingLogger`, routes, database
/// connection pool and email client attached to it.
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // Wrap the connection pool and email client in smarts pointers (because we want them to be available for all workers).
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    // `HttpServer::new` does not take `App` as argument - it wants a closure that returns an `App` struct.
    // This is to support actix-web’s runtime model: actix-web will spin up a worker process for each
    // available core on your machine.
    //
    // Each worker runs its own copy of the application built by `HttpServer` calling the very same closure
    // that `HttpServer::new` takes as argument.
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection pool as part of the application state
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
