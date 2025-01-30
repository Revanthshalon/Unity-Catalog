use config::{Config, File, FileFormat};
use database::DatabaseConfig;
use serde::Deserialize;

use crate::errors::UnityCatalogResult;

mod database;
mod server;

pub use self::server::ServerConfig;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct ApplicationConfig {
    pub server: ServerConfig,
    pub database: Option<DatabaseConfig>,
}

impl ApplicationConfig {
    pub fn load_config() -> UnityCatalogResult<Self> {
        let config = Config::builder()
            .add_source(File::new("./config/default.toml", FileFormat::Toml))
            .build()?;

        let config = config.try_deserialize()?;
        Ok(config)
    }
}
