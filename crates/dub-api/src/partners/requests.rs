use serde::Serialize;

/// Request to create a new partner
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreatePartnerRequest {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "commissionRate", skip_serializing_if = "Option::is_none")]
    pub commission_rate: Option<f64>,
}

/// Parameters for listing partners
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListPartnersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Request to create a partner link
#[derive(Debug, Clone, Serialize)]
pub struct CreatePartnerLinkRequest {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// Request to upsert a partner link
#[derive(Debug, Clone, Serialize)]
pub struct UpsertPartnerLinkRequest {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
