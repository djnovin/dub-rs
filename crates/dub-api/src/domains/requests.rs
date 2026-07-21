use serde::Serialize;

/// Request to create a domain
#[derive(Debug, Clone, Serialize)]
pub struct CreateDomainRequest {
    pub slug: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

/// Request to update a domain
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateDomainRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

/// Request to register a domain
#[derive(Debug, Clone, Serialize)]
pub struct RegisterDomainRequest {
    pub slug: String,
}

/// Request to check domain availability
#[derive(Debug, Clone, Serialize)]
pub struct CheckDomainAvailabilityRequest {
    pub slug: String,
}
