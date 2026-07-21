use dub_api::track::{TrackLeadRequest, TrackResponse, TrackSaleRequest};
use dub_core::{DubHandle, Result};

/// Track resource for conversion tracking
#[derive(Debug, Clone)]
pub struct Track {
    handle: DubHandle,
}

impl Track {
    pub(crate) fn new(handle: DubHandle) -> Self {
        Self { handle }
    }

    /// Track a lead conversion
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub::TrackLeadRequest;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    ///
    /// let request = TrackLeadRequest {
    ///     click_id: "click_123".to_string(),
    ///     event_name: "Sign up".to_string(),
    ///     customer_id: Some("customer_456".to_string()),
    ///     customer_name: Some("John Doe".to_string()),
    ///     customer_email: Some("john@example.com".to_string()),
    ///     customer_avatar: None,
    ///     metadata: None,
    /// };
    ///
    /// let response = dub.track().lead(request).await?;
    /// println!("Tracked lead: {}", response.event_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn lead(&self, request: TrackLeadRequest) -> Result<TrackResponse> {
        let client = self.handle.client();
        client.post("/track/lead", &request).await
    }

    /// Track a sale conversion
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # use dub::TrackSaleRequest;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    ///
    /// let request = TrackSaleRequest {
    ///     click_id: "click_123".to_string(),
    ///     event_name: "Purchase".to_string(),
    ///     customer_id: Some("customer_456".to_string()),
    ///     amount: 9999.0, // Amount in cents ($99.99)
    ///     currency: Some("USD".to_string()),
    ///     metadata: None,
    ///     invoice_id: Some("inv_789".to_string()),
    ///     payment_processor: Some("stripe".to_string()),
    /// };
    ///
    /// let response = dub.track().sale(request).await?;
    /// println!("Tracked sale: {}", response.event_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn sale(&self, request: TrackSaleRequest) -> Result<TrackResponse> {
        let client = self.handle.client();
        client.post("/track/sale", &request).await
    }
}
