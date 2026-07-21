use serde::Serialize;

/// Parameters for listing events
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListEventsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,

    #[serde(rename = "linkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
