//! Contains `run()` function used to create HTTP `Server` instance.
use crate::email_client::EmailClient;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

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
