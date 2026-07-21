use serde::{Deserialize, Serialize};

/// Parameters for listing bounty submissions
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBountySubmissionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    #[serde(rename = "partnerId", skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,

    /// Sort by field (default: completedAt)
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    /// Sort order (default: asc)
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// Request to approve a bounty submission
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApproveBountyRequest {
    // Empty body or any required fields can be added here
}

/// Request to reject a bounty submission
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RejectBountyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}
