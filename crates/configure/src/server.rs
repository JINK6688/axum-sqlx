use std::net::{AddrParseError, SocketAddr};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn get_addr(&self) -> Result<String, AddrParseError> {
        Ok(format!("{}:{}", self.host, self.port))
    }

    pub fn get_http_addr(&self) -> Result<String, AddrParseError> {
        Ok(format!("http://{}:{}", self.host, self.port))
    }

    //get socket addr
    pub fn get_socket_addr(&self) -> Result<SocketAddr, AddrParseError> {
        self.get_addr()?.parse()
    }
}
