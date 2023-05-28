use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::{ConnectOptions, Connection, Executor, PgConnection, PgPool};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

impl DatabaseSettings {
    /// Return Postgres connection options without DB name.
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            // Try an encrypted connection, fallback to unencrypted if it fails
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    /// Return Postgres connection options, including DB name.
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        // Lower `sqlx` logs from `INFO` to `TRACE` level.
        options.log_statements(tracing::log::LevelFilter::Trace);
        options
    }
}

/// Load application configuration from config file and deserialize it.
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to detect the current directory.");
    let configuration_directory = base_path.join("configuration");

    // Read the default configuration file
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    // Detect the running environment.
    // Default to `local` if not specified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    // Layer on the environment-specific values:
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    settings.try_into()
}

/// Represents the possible runtime environment for the application
pub enum Environment {
    Local,
    Production,
}

/// Conversion to string
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

/// Create enum from string
impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "`{}` is not supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

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

    match databases.count {
        Some(0) => false,
        _ => true,
    }
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
