use serde::Serialize;

/// Request to create a tag
#[derive(Debug, Clone, Serialize)]
pub struct CreateTagRequest {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Request to update a tag
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateTagRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}
