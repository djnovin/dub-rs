use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A short link
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Link {
    pub id: String,
    pub domain: String,
    pub key: String,
    pub url: String,

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

    pub archived: bool,
    pub clicks: u64,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,

    #[serde(rename = "projectId")]
    pub project_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// Link metadata for shortened display
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinkMeta {
    pub domain: String,
    pub key: String,
    pub url: String,
}
