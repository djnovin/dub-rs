use dub_api::domains::{
    CheckDomainAvailabilityRequest, CreateDomainRequest, Domain, DomainAvailabilityResponse,
    RegisterDomainRequest, UpdateDomainRequest,
};
use dub_core::{DubHandle, Result};

/// Domains resource for managing custom domains
#[derive(Debug, Clone)]
pub struct Domains {
    handle: DubHandle,
}

impl Domains {
    pub(crate) fn new(handle: DubHandle) -> Self {
        Self { handle }
    }

    /// List all domains
    pub async fn list(&self) -> Result<Vec<Domain>> {
        let client = self.handle.client();
        client.get("/domains").await
    }

    /// Create a new domain
    pub async fn create(&self, request: CreateDomainRequest) -> Result<Domain> {
        let client = self.handle.client();
        client.post("/domains", &request).await
    }

    /// Update a domain
    pub async fn update(&self, slug: &str, request: UpdateDomainRequest) -> Result<Domain> {
        let client = self.handle.client();
        client.patch(&format!("/domains/{}", slug), &request).await
    }

    /// Delete a domain
    pub async fn delete(&self, slug: &str) -> Result<()> {
        let client = self.handle.client();
        client.delete(&format!("/domains/{}", slug)).await
    }

    /// Register a new domain
    pub async fn register(&self, request: RegisterDomainRequest) -> Result<Domain> {
        let client = self.handle.client();
        client.post("/domains/register", &request).await
    }

    /// Check if a domain is available for registration
    pub async fn check_availability(
        &self,
        request: CheckDomainAvailabilityRequest,
    ) -> Result<DomainAvailabilityResponse> {
        let client = self.handle.client();
        client.post("/domains/check-availability", &request).await
    }
}
