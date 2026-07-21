use crate::config::DubConfig;
use crate::error::{DubError, Result};
use reqwest::{header, Client as ReqwestClient, Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

/// Core HTTP client for making API requests
#[derive(Debug, Clone)]
pub struct DubClient {
    client: ReqwestClient,
    config: DubConfig,
}

impl DubClient {
    /// Create a new client with the given configuration
    pub fn new(config: DubConfig) -> Result<Self> {
        config.validate()?;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", config.api_token))
                .map_err(|_| DubError::Auth("Invalid API token format".to_string()))?,
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let mut client_builder = ReqwestClient::builder()
            .default_headers(headers)
            .user_agent(format!("dub-rust-sdk/{}", env!("CARGO_PKG_VERSION")));

        if let Some(timeout) = config.timeout {
            client_builder = client_builder.timeout(Duration::from_secs(timeout));
        }

        let client = client_builder.build()?;

        Ok(Self { client, config })
    }

    /// Get the configuration
    pub fn config(&self) -> &DubConfig {
        &self.config
    }

    /// Build a request for the given method and path
    pub fn request(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let url = format!("{}{}", self.config.base_url, path);
        Ok(self.client.request(method, url))
    }

    /// Execute a request and parse the JSON response
    pub async fn execute<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T> {
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Execute a request without expecting a response body
    pub async fn execute_unit(&self, request: RequestBuilder) -> Result<()> {
        let response = request.send().await?;
        self.handle_response_unit(response).await
    }

    /// Handle a response and parse JSON
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            let body = response.json::<T>().await?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(DubError::Api {
                status: status_code,
                message: error_text,
            })
        }
    }

    /// Handle a response without parsing the body
    async fn handle_response_unit(&self, response: Response) -> Result<()> {
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(DubError::Api {
                status: status_code,
                message: error_text,
            })
        }
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let request = self.request(Method::GET, path)?;
        self.execute(request).await
    }

    /// Make a POST request with a JSON body
    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let request = self.request(Method::POST, path)?.json(body);
        self.execute(request).await
    }

    /// Make a PATCH request with a JSON body
    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let request = self.request(Method::PATCH, path)?.json(body);
        self.execute(request).await
    }

    /// Make a PUT request with a JSON body
    pub async fn put<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let request = self.request(Method::PUT, path)?.json(body);
        self.execute(request).await
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let request = self.request(Method::DELETE, path)?;
        self.execute_unit(request).await
    }

    /// Make a DELETE request with a JSON body
    pub async fn delete_with_body<B: Serialize>(&self, path: &str, body: &B) -> Result<()> {
        let request = self.request(Method::DELETE, path)?.json(body);
        self.execute_unit(request).await
    }
}
