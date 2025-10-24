// Configuration module

use std::env;

pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://user:password@localhost/aether_vpn".to_string()),
            server_port: env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap_or(8080),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()),
        }
    }
}