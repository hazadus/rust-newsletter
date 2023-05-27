//?
//? Contains `/subscriptions` endpoint handlers.
//?
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
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
    let request_span = tracing::info_span!(
        "Adding new subscriber.",
        %request_id,
        subscriber_name = %form.name,
        subscriber_email =  %form.email
    );

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database.");
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
    .execute(pool.as_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
