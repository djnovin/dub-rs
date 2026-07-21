use dub_api::analytics::AnalyticsSummary;
use dub_core::{DubHandle, Result};

/// Analytics resource for retrieving link analytics
#[derive(Debug, Clone)]
pub struct Analytics {
    handle: DubHandle,
}

impl Analytics {
    pub(crate) fn new(handle: DubHandle) -> Self {
        Self { handle }
    }

    /// Get analytics for a specific link
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    /// let analytics = dub.analytics().get_link("link-id").await?;
    /// println!("Total clicks: {}", analytics.clicks);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_link(&self, link_id: &str) -> Result<AnalyticsSummary> {
        let client = self.handle.client();
        client.get(&format!("/analytics/links/{}", link_id)).await
    }

    /// Get workspace-level analytics
    pub async fn get_workspace(&self) -> Result<AnalyticsSummary> {
        let client = self.handle.client();
        client.get("/analytics").await
    }
}
