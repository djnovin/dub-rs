use serde::{Deserialize, Serialize};

/// Information about a partner in a payout
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartnerInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub image: String,

    #[serde(rename = "payoutsEnabledAt", skip_serializing_if = "Option::is_none")]
    pub payouts_enabled_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,

    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

/// Information about a user in a payout
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub image: String,
}

/// A payout record
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Payout {
    pub id: String,

    #[serde(rename = "invoiceId")]
    pub invoice_id: String,

    pub amount: f64,
    pub currency: String,

    /// Status: pending, processing, processed, sent, completed, failed, canceled
    pub status: String,

    #[serde(rename = "periodStart", skip_serializing_if = "Option::is_none")]
    pub period_start: Option<String>,

    #[serde(rename = "periodEnd", skip_serializing_if = "Option::is_none")]
    pub period_end: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "initiatedAt", skip_serializing_if = "Option::is_none")]
    pub initiated_at: Option<String>,

    #[serde(rename = "paidAt", skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,

    /// Mode: internal, external
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Method: connect, stablecoin, paypal, tremendous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    pub partner: PartnerInfo,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    #[serde(rename = "failureReason", skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,

    #[serde(rename = "traceId", skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}
