//! Includes all integration tests.
use newsletter::configuration::get_configuration;
use newsletter::database::configure_database;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::PgPool;
use std::net::TcpListener;
use uuid::Uuid;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "newsletter_test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

/// Launch the application in background.
/// Bind TCP listener to random port.
/// Create new database with random name to isolate test runs.
/// Return `TestApp` including server address and database connection pool.
async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read config file.");
    // Create new database on each test run to isolate tests.
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = newsletter::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address.");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

/// Check that `/health_check` endpoint return `200 OK` with zero-length content.
#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to send request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

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

/// Empty fields and wrong email are passing for now.
#[tokio::test]
async fn subscribe_returns_a_200_when_fields_are_present_but_empty() {
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
            200,
            response.status().as_u16(),
            "The API did not return a 200 OK when the payload was {}.",
            description
        );
    }
}
