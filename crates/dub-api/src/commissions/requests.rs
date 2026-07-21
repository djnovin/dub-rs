use serde::{Deserialize, Serialize};

/// Parameters for listing commissions
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommissionsParams {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub commission_type: Option<String>,

    #[serde(rename = "customerId", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,

    #[serde(rename = "payoutId", skip_serializing_if = "Option::is_none")]
    pub payout_id: Option<String>,

    #[serde(rename = "partnerId", skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,

    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,

    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    #[serde(rename = "partnerTagId", skip_serializing_if = "Option::is_none")]
    pub partner_tag_id: Option<String>,

    #[serde(rename = "invoiceId", skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Sort by field (default: createdAt)
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    /// Sort order (default: desc)
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,

    /// Interval (default: all)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    #[serde(rename = "endingBefore", skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(rename = "startingAfter", skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// Request to create a new commission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommissionRequest {
    /// Must be "custom", "lead", or "sale"
    #[serde(rename = "type")]
    pub commission_type: String,

    #[serde(rename = "partnerId")]
    pub partner_id: String,

    pub amount: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to update a commission
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCommissionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
