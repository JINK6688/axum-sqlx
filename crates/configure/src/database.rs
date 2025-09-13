use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

impl DatabaseConfig {
    pub fn get_url(&self) -> String {
        Self::create_url(&self.username, &self.password, &self.host, self.port, &self.database_name)
    }

    pub fn create_url(
        username: &str,
        password: &str,
        host: &str,
        post: u16,
        database_name: &str,
    ) -> String {
        format!("postgres://{username}:{password}@{host}:{post}/{database_name}")
    }
}
