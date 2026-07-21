use crate::error::{DubError, Result};

/// Configuration for the Dub SDK
#[derive(Debug, Clone)]
pub struct DubConfig {
    /// API token for authentication
    pub(crate) api_token: String,

    /// Base URL for the API
    pub(crate) base_url: String,

    /// Request timeout in seconds
    pub(crate) timeout: Option<u64>,
}

impl DubConfig {
    /// Create a new configuration with an API token
    pub fn new(api_token: impl Into<String>) -> Self {
        Self {
            api_token: api_token.into(),
            base_url: "https://api.dub.co".to_string(),
            timeout: Some(30),
        }
    }

    /// Set a custom base URL
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Set request timeout in seconds
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Disable request timeout
    pub fn without_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.api_token.is_empty() {
            return Err(DubError::Config("API token cannot be empty".to_string()));
        }

        url::Url::parse(&self.base_url)
            .map_err(|_| DubError::Config(format!("Invalid base URL: {}", self.base_url)))?;

        Ok(())
    }

    /// Get the API token
    pub fn api_token(&self) -> &str {
        &self.api_token
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the timeout
    pub fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}
