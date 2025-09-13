use std::str::FromStr;

use super::profile::Profile;

pub fn get_env_source(prefix: &str) -> config::Environment {
    config::Environment::with_prefix(prefix).try_parsing(true).prefix_separator("__").separator("_")
}

pub fn get_profile() -> Result<Profile, config::ConfigError> {
    dotenvy::dotenv().ok();
    std::env::var("ENVIRONMENT")
        .map(|env| Profile::from_str(&env).map_err(|e| config::ConfigError::Message(e.to_string())))
        .unwrap_or_else(|_e| Ok(Profile::Development))
}
