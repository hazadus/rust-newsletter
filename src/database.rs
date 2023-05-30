//! Database related stuff.
use crate::configuration::DatabaseSettings;
use sqlx::{Connection, Executor, PgConnection, PgPool};

/// Configure and migrate database.
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create the database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(
            format!(
                r#"
                CREATE DATABASE "{}";
                "#,
                config.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}

/// Return true if database with `database_name` exists on the specified Postgres instance.
pub async fn database_exists(config: &DatabaseSettings) -> bool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres.");

    let databases = sqlx::query!(
        r#"
        SELECT COUNT(*) AS count FROM pg_database WHERE datname = $1;
        "#,
        config.database_name
    )
    .fetch_one(&mut connection)
    .await
    .expect("Failed to get database count from `pg_database`.");

    !matches!(databases.count, Some(0))
}

/// Check if database with `database_name` exists on the specified Postgres instance.
/// If not, create and migrate it.
pub async fn configure_db_if_not_exists(config: &DatabaseSettings) {
    let exists = database_exists(config).await;

    match exists {
        true => tracing::info!("Database exists."),
        false => {
            tracing::info!("Database DOES NOT exists, trying to create it.");
            configure_database(config).await;
        }
    }
}
