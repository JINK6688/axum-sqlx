pub mod database;
pub mod env;
pub mod error;
pub mod jwt;
pub mod log_tracing;
pub mod profile;
pub mod server;

use std::path::PathBuf;

use anyhow::Result;
use config::ConfigError;
use database::DatabaseConfig;
use env::{get_env_source, get_profile};
use jwt::JwtConfig;
use once_cell::sync::Lazy;
use profile::Profile;
use serde::Deserialize;
use server::ServerConfig;
use tracing::info;

pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig::read().unwrap());

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub profile: Profile,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
}

impl AppConfig {
    pub fn read() -> Result<AppConfig, ConfigError> {
        let config_dir =
            get_root_dir().map_err(|e| ConfigError::Message(e.to_string()))?.join("setting");

        let env_source = get_env_source("APP");
        info!("config dir: {:#?}", config_dir);
        let profile = get_profile()?;
        let profile_filename = format!("{profile}.toml");
        info!("running in {:?} mode", profile);

        let config = config::Config::builder()
            .add_source(config::File::with_name(&format!(
                "{}/default.toml",
                config_dir.to_string_lossy()
            )))
            .add_source(config::File::with_name(&format!(
                "{}/{}",
                config_dir.to_string_lossy(),
                profile_filename
            )))
            .add_source(env_source)
            .build()?;
        config.try_deserialize()
    }
}

pub fn get_root_dir() -> Result<PathBuf, ConfigError> {
    let mut current = std::env::current_dir().map_err(|e| ConfigError::Message(e.to_string()))?;
    while current.parent().is_some() {
        if current.join("Cargo.lock").exists() {
            return Ok(current);
        }
        current = current.parent().unwrap().to_path_buf();
    }
    Err(ConfigError::Message("Cannot find Cargo.toml in any parent directory".into()))
}
