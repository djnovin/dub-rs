use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A domain
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Domain {
    pub id: String,
    pub slug: String,
    pub verified: bool,
    pub primary: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
