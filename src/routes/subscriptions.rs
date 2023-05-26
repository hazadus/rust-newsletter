use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Form data shape for `subscribe` endpoint.
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

/// Before calling `subscribe` actix-web invokes the `from_request` method for all subscribe’s
/// input arguments: in our case, `Form::from_request`;
///
/// `Form::from_request` tries to deserialise the body into `FormData` according to the rules of
/// URL-encoding leveraging `serde_urlencoded` and the `Deserialize` implementation of `FormData`,
/// automatically generated for us by `#[derive(serde::Deserialize)]`;
///
/// If `Form::from_request` fails, a `400 BAD REQUEST` is returned to the caller. If it succeeds,
/// `subscribe` is invoked and we return a `200 OK`.
///
/// `pool` is retrieved from application state.
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // We use UUID to correlate al log messages to the request.
    let request_id = Uuid::new_v4();
    tracing::info!(
        "request_id {} - Saving new subscriber in DB: name: {}, email: {}",
        request_id,
        form.name,
        form.email
    );

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved.",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
