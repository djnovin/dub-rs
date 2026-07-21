use serde::Serialize;

/// Request to create a new link
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateLinkRequest {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// Request to update an existing link
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateLinkRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// Parameters for listing links
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListLinksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Request for upsert link operation
#[derive(Debug, Clone, Serialize)]
pub struct UpsertLinkRequest {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// Request for bulk delete operation
#[derive(Debug, Clone, Serialize)]
pub struct BulkDeleteLinksRequest {
    #[serde(rename = "linkIds")]
    pub link_ids: Vec<String>,
}

/// Request for bulk update operation
#[derive(Debug, Clone, Serialize)]
pub struct BulkUpdateLinksRequest {
    #[serde(rename = "linkIds")]
    pub link_ids: Vec<String>,

    pub data: UpdateLinkRequest,
}
