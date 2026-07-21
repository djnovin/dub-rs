use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A tag
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
