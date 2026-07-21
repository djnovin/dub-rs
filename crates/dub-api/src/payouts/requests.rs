use serde::{Deserialize, Serialize};

/// Parameters for listing payouts
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPayoutsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "partnerId", skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,

    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,

    #[serde(rename = "invoiceId", skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,

    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    /// Sort by field (default: amount)
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    /// Sort order (default: desc)
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}
