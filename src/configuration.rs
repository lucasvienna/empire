use crate::Result;
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
