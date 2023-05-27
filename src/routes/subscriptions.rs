//?
//? Contains `/subscriptions` endpoint handlers.
//?
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
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_name = %form.name,
        subscriber_email =  %form.email
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Insert subscriber row into database.
#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
