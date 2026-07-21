use thiserror::Error;

/// Core error types for the Dub SDK
#[derive(Debug, Error)]
pub enum DubError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    /// Failed to serialize/deserialize data
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Invalid configuration
    #[error("Configuration error: {0}")]
    Config(String),

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),

    /// Authentication failed
    #[error("Authentication error: {0}")]
    Auth(String),
}

pub type Result<T> = std::result::Result<T, DubError>;
