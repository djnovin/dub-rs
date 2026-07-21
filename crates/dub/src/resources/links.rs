use dub_api::links::{
    BulkDeleteLinksRequest, BulkUpdateLinksRequest, CreateLinkRequest, Link, LinkCountResponse,
    ListLinksParams, UpdateLinkRequest, UpsertLinkRequest,
};
use dub_core::{DubHandle, Result};

/// Links resource for managing short links
#[derive(Debug, Clone)]
pub struct Links {
    handle: DubHandle,
}

impl Links {
    pub(crate) fn new(handle: DubHandle) -> Self {
        Self { handle }
    }

    /// List all links
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    /// let links = dub.links().list(None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: Option<ListLinksParams>) -> Result<Vec<Link>> {
        let client = self.handle.client();

        if let Some(params) = params {
            let mut path = "/links?".to_string();

            // Build query string
            let query =
                serde_json::to_string(&params).map_err(dub_core::DubError::Serialization)?;
            path.push_str(&query);

            client.get(&path).await
        } else {
            client.get("/links").await
        }
    }

    /// Get a specific link by ID
    pub async fn get(&self, link_id: &str) -> Result<Link> {
        let client = self.handle.client();
        client.get(&format!("/links/{}", link_id)).await
    }

    /// Create a new link
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub::CreateLinkRequest;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    ///
    /// let request = CreateLinkRequest {
    ///     url: "https://example.com".to_string(),
    ///     domain: Some("dub.sh".to_string()),
    ///     key: Some("my-link".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let link = dub.links().create(request).await?;
    /// println!("Created link: {}/{}", link.domain, link.key);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, request: CreateLinkRequest) -> Result<Link> {
        let client = self.handle.client();
        client.post("/links", &request).await
    }

    /// Update an existing link
    pub async fn update(&self, link_id: &str, request: UpdateLinkRequest) -> Result<Link> {
        let client = self.handle.client();
        client.patch(&format!("/links/{}", link_id), &request).await
    }

    /// Delete a link
    pub async fn delete(&self, link_id: &str) -> Result<()> {
        let client = self.handle.client();
        client.delete(&format!("/links/{}", link_id)).await
    }

    /// Upsert a link (create or update)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub::UpsertLinkRequest;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    ///
    /// let request = UpsertLinkRequest {
    ///     url: "https://example.com".to_string(),
    ///     domain: Some("dub.sh".to_string()),
    ///     key: Some("my-link".to_string()),
    ///     title: None,
    ///     description: None,
    ///     image: None,
    ///     ios: None,
    ///     android: None,
    ///     tags: None,
    ///     comments: None,
    /// };
    ///
    /// let link = dub.links().upsert(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upsert(&self, request: UpsertLinkRequest) -> Result<Link> {
        let client = self.handle.client();
        client.put("/links/upsert", &request).await
    }

    /// Count links matching the given parameters
    pub async fn count(&self, params: Option<ListLinksParams>) -> Result<u64> {
        let client = self.handle.client();

        let response: LinkCountResponse = if let Some(params) = params {
            let mut path = "/links/count?".to_string();
            let query =
                serde_json::to_string(&params).map_err(dub_core::DubError::Serialization)?;
            path.push_str(&query);
            client.get(&path).await?
        } else {
            client.get("/links/count").await?
        };

        Ok(response.count)
    }

    /// Bulk create links
    pub async fn bulk_create(&self, requests: Vec<CreateLinkRequest>) -> Result<Vec<Link>> {
        let client = self.handle.client();
        client.post("/links/bulk", &requests).await
    }

    /// Bulk update links
    pub async fn bulk_update(&self, request: BulkUpdateLinksRequest) -> Result<Vec<Link>> {
        let client = self.handle.client();
        client.patch("/links/bulk", &request).await
    }

    /// Bulk delete links
    pub async fn bulk_delete(&self, request: BulkDeleteLinksRequest) -> Result<()> {
        let client = self.handle.client();
        client.delete_with_body("/links/bulk", &request).await
    }
}
