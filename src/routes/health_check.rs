use actix_web::{HttpResponse, Responder};

/// This route is used to check whether the server is up and running, or not.
/// Return `200 OK`.
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
