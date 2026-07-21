use serde::Serialize;

/// Request to track a lead conversion
#[derive(Debug, Clone, Serialize)]
pub struct TrackLeadRequest {
    #[serde(rename = "clickId")]
    pub click_id: String,

    #[serde(rename = "eventName")]
    pub event_name: String,

    #[serde(rename = "customerId", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,

    #[serde(rename = "customerName", skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,

    #[serde(rename = "customerEmail", skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,

    #[serde(rename = "customerAvatar", skip_serializing_if = "Option::is_none")]
    pub customer_avatar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Request to track a sale conversion
#[derive(Debug, Clone, Serialize)]
pub struct TrackSaleRequest {
    #[serde(rename = "clickId")]
    pub click_id: String,

    #[serde(rename = "eventName")]
    pub event_name: String,

    #[serde(rename = "customerId", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,

    /// Amount in cents
    pub amount: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    #[serde(rename = "invoiceId", skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,

    #[serde(rename = "paymentProcessor", skip_serializing_if = "Option::is_none")]
    pub payment_processor: Option<String>,
}
