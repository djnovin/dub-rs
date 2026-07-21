use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// An event
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: String,
    pub event: String,

    #[serde(rename = "linkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,

    #[serde(rename = "clickId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<String>,

    pub timestamp: DateTime<Utc>,
}
