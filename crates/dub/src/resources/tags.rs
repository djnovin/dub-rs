use dub_api::tags::{CreateTagRequest, Tag, UpdateTagRequest};
use dub_core::{DubHandle, Result};

/// Tags resource for managing tags
#[derive(Debug, Clone)]
pub struct Tags {
    handle: DubHandle,
}

impl Tags {
    pub(crate) fn new(handle: DubHandle) -> Self {
        Self { handle }
    }

    /// List all tags
    pub async fn list(&self) -> Result<Vec<Tag>> {
        let client = self.handle.client();
        client.get("/tags").await
    }

    /// Create a new tag
    pub async fn create(&self, request: CreateTagRequest) -> Result<Tag> {
        let client = self.handle.client();
        client.post("/tags", &request).await
    }

    /// Update a tag
    pub async fn update(&self, id: &str, request: UpdateTagRequest) -> Result<Tag> {
        let client = self.handle.client();
        client.patch(&format!("/tags/{}", id), &request).await
    }
}
