use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A partner in the affiliate/partner program
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Partner {
    pub id: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "commissionRate", skip_serializing_if = "Option::is_none")]
    pub commission_rate: Option<f64>,

    pub status: String, // "active", "inactive", "banned"

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

/// A partner sale record
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartnerSale {
    pub id: String,
    pub amount: f64,
    pub currency: String,

    #[serde(rename = "customerId", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}
