use actix_web::{web, HttpResponse};

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
pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("Name: {}, email: {}", form.name, form.email);

    HttpResponse::Ok().finish()
}
