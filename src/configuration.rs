use crate::net::server::AppState;
use crate::Result;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use config::{Config, Environment, File};
use reqwest::StatusCode;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use std::env;
use std::net::Ipv4Addr;
use tracing::{debug, instrument, trace};

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
    pub jwt: JwtSettings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub pool_size: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerSettings {
    pub axum_host: Ipv4Addr,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub axum_port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JwtSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub expires_in: u64,
}

/// The possible runtime environment for our application.
#[derive(Debug)]
pub enum AppEnvironment {
    Development,
    Production,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretString {
        SecretString::new(
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port,
                self.database_name
            )
            .into(),
        )
    }
}

impl AppEnvironment {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppEnvironment::Development => "dev",
            AppEnvironment::Production => "prod",
        }
    }
}

impl From<AppEnvironment> for String {
    fn from(env: AppEnvironment) -> Self {
        env.as_str().into()
    }
}

impl TryFrom<String> for AppEnvironment {
    type Error = crate::Error;

    fn try_from(s: String) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Development),
            "dev" => Ok(Self::Development),
            "development" => Ok(Self::Development),
            "prod" => Ok(Self::Production),
            "production" => Ok(Self::Production),
            other => Err(config::ConfigError::Message(format!(
                "Invalid environment: {}. Use either `local` or `production`.",
                other
            ))
            .into()),
        }
    }
}

/// Reads the application configuration from YAML files and deserializes it into the [`Settings`] struct.
///
/// This function uses the `config` crate to load configuration values from a base file named `application.yaml`
/// and an environment-specific file (e.g., `application.dev.yaml` for development or `application.prod.yaml`).
/// The environment is determined by the `APP_ENVIRONMENT` environment variable, which defaults to `development`
/// if not set. The loaded configuration values are then deserialized into the [`Settings`] struct using Serde.
///
/// # Returns
///
/// Returns a [`Result`] containing the [`Settings`] if deserialization succeeds,
/// or an error if the files cannot be read or the data is invalid.
///
/// # Errors
///
/// Returns an error if:
/// - The configuration files are missing or cannot be read.
/// - The YAML content cannot be deserialized into the [`Settings`] struct.
/// - The `APP_ENVIRONMENT` environment variable is invalid.
///
/// [`Settings`]: Settings
/// [`Result`]: Result
#[instrument(name = "settings")]
pub fn get_settings() -> Result<Settings> {
    let base_path = env::current_dir().expect("Failed to determine the current directory");
    let config_dir = base_path.join("config");

    // Detect the running environment.
    // Default to `development` if unspecified.
    let environment: AppEnvironment = env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    trace!("Running in environment: {:#?}", environment);

    let env_filename = format!("application.{}.yaml", environment.as_str());
    debug!("Loading settings from file: {:#?}", env_filename);

    // these overwrite the previous ones, so we can have stacking defaults with an ENV override
    let settings = Config::builder()
        .add_source(File::from(config_dir.join("application.yaml")))
        .add_source(File::from(config_dir.join(env_filename)))
        .add_source(
            Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    let settings = settings.try_deserialize::<Settings>()?;
    trace!(?settings);

    Ok(settings)
}

/// Loads environment variables and sets up logging configurations.
///
/// This function attempts to load environment variables from a `.env` file
/// using the `dotenvy` crate. If the `RUST_LOG` environment variable is set,
/// it ensures that logging information for Diesel queries is included.
/// If `RUST_LOG` is not set, it defaults to configure logging at the debug
/// for the `empire` application, for Diesel queries, and for axum HTTP requests.
///
/// # Returns
///
/// Returns a `Result` indicating success or an error if setting environment
/// variables fails.
pub fn load_env() -> Result<()> {
    dotenvy::dotenv().ok();
    match env::var("RUST_LOG").ok() {
        Some(v) => {
            if !v.contains("diesel") {
                env::set_var("RUST_LOG", format!("{},diesel=debug", v));
            }
            if !v.contains("tower_http") {
                env::set_var("RUST_LOG", format!("{},tower_http=debug", v));
            }
            if !v.contains("axum::rejection") {
                env::set_var("RUST_LOG", format!("{},axum::rejection=trace", v));
            }
        }
        None => env::set_var("RUST_LOG", "empire=debug,tower_http=debug,diesel=debug"),
    };

    Ok(())
}

impl<S> FromRequestParts<S> for Settings
where
    S: Send + Sync,
    Settings: FromRef<S>,
{
    type Rejection = (StatusCode, String);
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let settings = Settings::from_ref(state);
        Ok(settings)
    }
}

impl FromRef<AppState> for Settings {
    fn from_ref(state: &AppState) -> Self {
        state.settings.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_env() {
        load_env().unwrap();
    }

    #[test]
    fn test_get() {
        get_settings().unwrap();
    }
}
