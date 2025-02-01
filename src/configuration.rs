use crate::Result;
use std::env;
use tracing::debug;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub pool_size: Option<u32>,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ServerSettings {
    pub rest_port: u16,
}

/// Reads the application configuration from a YAML file and deserializes it into the [`Settings`] struct.
///
/// This function uses the `config` crate to load configuration values from a file named `configuration.yaml`
/// and expects the file to be in YAML format. The loaded configuration values are then deserialized into
/// the [`Settings`] struct using Serde.
///
/// # Returns
///
/// Returns a [`Result`] containing the [`Settings`] if deserialization succeeds,
/// or an error if the file cannot be read or the data is invalid.
///
/// # Errors
///
/// Returns an error if:
/// - The configuration file is missing or cannot be read.
/// - The YAML content cannot be deserialized into the [`Settings`] struct.
///
/// [`Settings`]: Settings
/// [`Result`]: Result
pub fn get_configuration() -> Result<Settings> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    let settings = settings.try_deserialize::<Settings>()?;
    debug!("Loaded configuration from file. {:#?}", settings);

    Ok(settings)
}

/// Loads environment variables and sets up logging configurations.
///
/// This function attempts to load environment variables from a `.env` file
/// using the `dotenvy` crate. If the `RUST_LOG` environment variable is set,
/// it ensures that logging information for Diesel queries is included.
/// If `RUST_LOG` is not set, it defaults to configure logging at the debug
/// for the `empire` application and for Diesel queries.
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
        }
        None => env::set_var("RUST_LOG", "empire=debug,diesel=debug"),
    };

    Ok(())
}
