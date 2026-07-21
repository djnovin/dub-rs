use serde::{Deserialize, Serialize};

/// Customer information in a commission
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomerInfo {
    pub id: String,

    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    #[serde(rename = "stripeCustomerId", skip_serializing_if = "Option::is_none")]
    pub stripe_customer_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    pub sales: u32,

    #[serde(rename = "saleAmount")]
    pub sale_amount: f64,

    #[serde(rename = "firstSaleAt", skip_serializing_if = "Option::is_none")]
    pub first_sale_at: Option<String>,

    #[serde(
        rename = "subscriptionCanceledAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_canceled_at: Option<String>,
}

/// Partner information in a commission (re-exported from payouts)
pub use crate::payouts::PartnerInfo;

/// A commission record
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Commission {
    pub id: String,
    pub amount: f64,
    pub earnings: f64,
    pub currency: String,

    /// Status: pending, processed, paid, refunded, duplicate, fraud, canceled
    pub status: String,

    #[serde(rename = "invoiceId", skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub quantity: u32,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    pub partner: PartnerInfo,

    /// Type: click, lead, sale, referral, custom
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub commission_type: Option<String>,

    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerInfo>,
}
