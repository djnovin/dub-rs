use dub_api::events::{Event, ListEventsParams};
use dub_core::{DubHandle, Result};

/// Events resource for tracking events
#[derive(Debug, Clone)]
pub struct Events {
    handle: DubHandle,
}

impl Events {
    pub(crate) fn new(handle: DubHandle) -> Self {
        Self { handle }
    }

    /// List all events
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dub::Dub;
    /// # async fn example() -> dub::Result<()> {
    /// let dub = Dub::new("token")?;
    /// let events = dub.events().list(None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: Option<ListEventsParams>) -> Result<Vec<Event>> {
        let client = self.handle.client();

        if let Some(params) = params {
            let mut path = "/events?".to_string();

            // Build query string
            let query =
                serde_json::to_string(&params).map_err(dub_core::DubError::Serialization)?;
            path.push_str(&query);

            client.get(&path).await
        } else {
            client.get("/events").await
        }
    }
}
