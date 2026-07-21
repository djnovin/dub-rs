use dub_api::customers::{Customer, ListCustomersParams, UpdateCustomerRequest};
use dub_core::{DubHandle, Result};

/// Customers resource for managing customers
#[derive(Debug, Clone)]
pub struct Customers {
    handle: DubHandle,
}

impl Customers {
    pub(crate) fn new(handle: DubHandle) -> Self {
        Self { handle }
    }

    /// List all customers
    pub async fn list(&self, params: Option<ListCustomersParams>) -> Result<Vec<Customer>> {
        let client = self.handle.client();

        if let Some(params) = params {
            let mut path = "/customers?".to_string();

            // Build query string
            let query =
                serde_json::to_string(&params).map_err(dub_core::DubError::Serialization)?;
            path.push_str(&query);

            client.get(&path).await
        } else {
            client.get("/customers").await
        }
    }

    /// Get a customer by ID
    pub async fn get(&self, id: &str) -> Result<Customer> {
        let client = self.handle.client();
        client.get(&format!("/customers/{}", id)).await
    }

    /// Update a customer
    pub async fn update(&self, id: &str, request: UpdateCustomerRequest) -> Result<Customer> {
        let client = self.handle.client();
        client.patch(&format!("/customers/{}", id), &request).await
    }

    /// Delete a customer
    pub async fn delete(&self, id: &str) -> Result<()> {
        let client = self.handle.client();
        client.delete(&format!("/customers/{}", id)).await
    }
}
