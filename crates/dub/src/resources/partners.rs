use dub_api::analytics::AnalyticsSummary;
use dub_api::links::Link;
use dub_api::partners::{
    CreatePartnerLinkRequest, CreatePartnerRequest, ListPartnersParams, Partner,
    UpsertPartnerLinkRequest,
};
use dub_core::{DubHandle, Result};

/// Partners resource for managing affiliate/partner program
#[derive(Debug, Clone)]
pub struct Partners {
    handle: DubHandle,
}

impl Partners {
    pub(crate) fn new(handle: DubHandle) -> Self {
        Self { handle }
    }

    /// List all partners
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    /// let partners = dub.partners().list(None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: Option<ListPartnersParams>) -> Result<Vec<Partner>> {
        let client = self.handle.client();

        if let Some(params) = params {
            // Build query string from params
            let mut query_parts = Vec::new();

            if let Some(page) = params.page {
                query_parts.push(format!("page={}", page));
            }
            if let Some(limit) = params.limit {
                query_parts.push(format!("limit={}", limit));
            }
            if let Some(status) = params.status {
                query_parts.push(format!("status={}", status));
            }

            if query_parts.is_empty() {
                client.get("/partners").await
            } else {
                let query = query_parts.join("&");
                client.get(&format!("/partners?{}", query)).await
            }
        } else {
            client.get("/partners").await
        }
    }

    /// Create a new partner
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub_api::partners::CreatePartnerRequest;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    /// let request = CreatePartnerRequest {
    ///     name: "Acme Corp".to_string(),
    ///     email: Some("partner@acme.com".to_string()),
    ///     ..Default::default()
    /// };
    /// let partner = dub.partners().create(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, request: CreatePartnerRequest) -> Result<Partner> {
        let client = self.handle.client();
        client.post("/partners", &request).await
    }

    /// Ban a partner
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    /// let partner = dub.partners().ban("partner_id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ban(&self, id: &str) -> Result<Partner> {
        let client = self.handle.client();
        let empty: Option<()> = None;
        client.post(&format!("/partners/{}/ban", id), &empty).await
    }

    /// Deactivate a partner
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    /// let partner = dub.partners().deactivate("partner_id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn deactivate(&self, id: &str) -> Result<Partner> {
        let client = self.handle.client();
        let empty: Option<()> = None;
        client
            .post(&format!("/partners/{}/deactivate", id), &empty)
            .await
    }

    /// Create a link for a partner
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub_api::partners::CreatePartnerLinkRequest;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    /// let request = CreatePartnerLinkRequest {
    ///     url: "https://example.com".to_string(),
    ///     domain: None,
    ///     key: None,
    /// };
    /// let link = dub.partners().create_link("partner_id", request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_link(&self, id: &str, request: CreatePartnerLinkRequest) -> Result<Link> {
        let client = self.handle.client();
        client
            .post(&format!("/partners/{}/links", id), &request)
            .await
    }

    /// Upsert a link for a partner
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub_api::partners::UpsertPartnerLinkRequest;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    /// let request = UpsertPartnerLinkRequest {
    ///     url: "https://example.com".to_string(),
    ///     domain: None,
    ///     key: None,
    /// };
    /// let link = dub.partners().upsert_link("partner_id", request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upsert_link(&self, id: &str, request: UpsertPartnerLinkRequest) -> Result<Link> {
        let client = self.handle.client();
        client
            .put(&format!("/partners/{}/links", id), &request)
            .await
    }

    /// Get links for a partner
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    /// let links = dub.partners().get_links("partner_id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_links(&self, id: &str) -> Result<Vec<Link>> {
        let client = self.handle.client();
        client.get(&format!("/partners/{}/links", id)).await
    }

    /// Get analytics for a partner
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    /// let analytics = dub.partners().get_analytics("partner_id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_analytics(&self, id: &str) -> Result<AnalyticsSummary> {
        let client = self.handle.client();
        client.get(&format!("/partners/{}/analytics", id)).await
    }
}
