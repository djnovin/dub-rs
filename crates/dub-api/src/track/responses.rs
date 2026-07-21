use serde::{Deserialize, Serialize};

/// Response from tracking a conversion event
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackResponse {
    #[serde(rename = "clickId")]
    pub click_id: String,

    #[serde(rename = "eventName")]
    pub event_name: String,

    #[serde(rename = "customerId")]
    pub customer_id: Option<String>,
}
