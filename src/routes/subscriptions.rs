//!
//! Contains `/subscriptions` endpoint handlers.
//!
use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
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

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    /// Convert `FormData` to our domain-specific type `NewSubscriber`.
    // NB: If you provide a `TryFrom` implementation, your type automatically gets the corresponding
    // `TryInto` implementation, for free.
    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

/// Add new subscriber to database using validated `FormData`.
// Before calling `subscribe` actix-web invokes the `from_request` method for all subscribe’s
// input arguments: in our case, `Form::from_request`;
//
// `Form::from_request` tries to deserialise the body into `FormData` according to the rules of
// URL-encoding leveraging `serde_urlencoded` and the `Deserialize` implementation of `FormData`,
// automatically generated for us by `#[derive(serde::Deserialize)]`;
//
// If `Form::from_request` fails, a `400 BAD REQUEST` is returned to the caller. If it succeeds,
// `subscribe` is invoked and we return a `200 OK`.
//
// `pool` is retrieved from application state.
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_name = %form.name,
        subscriber_email =  %form.email
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_subscriber: NewSubscriber = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Insert subscriber row into database.
#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
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
