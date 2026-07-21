use std::sync::Arc;

use crate::client::DubClient;

/// A handle to the Dub client that can be cheaply cloned and shared
///
/// This is the main entry point for API resources. Each resource
/// gets its own handle that shares the underlying HTTP client.
#[derive(Debug, Clone)]
pub struct DubHandle {
    client: Arc<DubClient>,
}

impl DubHandle {
    /// Create a new handle from a client
    pub fn new(client: DubClient) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    /// Get a reference to the underlying client
    pub fn client(&self) -> &DubClient {
        &self.client
    }
}

impl From<DubClient> for DubHandle {
    fn from(client: DubClient) -> Self {
        Self::new(client)
    }
}
