use super::types::Link;
use serde::{Deserialize, Serialize};

/// Response when creating or updating a link
pub type LinkResponse = Link;

/// Response when listing links
pub type ListLinksResponse = Vec<Link>;

/// Response for link count
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinkCountResponse {
    pub count: u64,
}

/// Response for bulk operations
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BulkResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}
