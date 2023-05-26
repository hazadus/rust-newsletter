use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer (because we want it to be available for all workers).
    let db_pool = web::Data::new(db_pool);
    // `HttpServer::new` does not take `App` as argument - it wants a closure that returns an `App` struct.
    // This is to support actix-web’s runtime model: actix-web will spin up a worker process for each
    // available core on your machine.
    //
    // Each worker runs its own copy of the application built by `HttpServer` calling the very same closure
    // that `HttpServer::new` takes as argument.
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
