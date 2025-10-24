use thiserror::Error;

#[derive(Error, Debug)]
pub enum AvpnError {
    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("API request failed: {0}")]
    Api(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("WireGuard error: {0}")]
    WireGuard(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),
}