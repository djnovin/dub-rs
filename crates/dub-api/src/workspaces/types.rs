use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A workspace/project
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub slug: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
