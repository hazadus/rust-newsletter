//! Contains tests for `/subscriptions` endpoint.
use crate::helpers::spawn_app;

/// Check that `/subscriptions` endpoint returns `200 OK` when valid form data was posted
/// and the data is properly saved in database.
#[tokio::test]
async fn subscribe_return_200_for_valid_form_and_data_properly_saved() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=hazadus&email=hazadus7%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription from database.");

    assert_eq!(saved.email, "hazadus7@gmail.com");
    assert_eq!(saved.name, "hazadus");
}

/// Check that `/subscriptions` endpoint returns `400 BAD REQUEST` when invalid data was posted.
#[tokio::test]
async fn subscribe_return_400_for_missing_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=hazadus", "missing the email"),
        ("email=hazadus%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional custom error message on test failure:
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

/// Ensure that empty fields and wrong emails are causing `400 BAD REQUEST` errors.
#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=hazadus7%40gmail.com", "empty name"),
        ("name=hazadus&email=", "empty email"),
        (
            "name=hazadus&email=definitely-not-an-email",
            "invalid email",
        ),
    ];
    for (body, description) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 BAD REQUEST when the payload was {}.",
            description
        );
    }
}
