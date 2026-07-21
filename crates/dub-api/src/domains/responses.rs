use super::types::Domain;
use serde::{Deserialize, Serialize};

/// Response when creating or updating a domain
pub type DomainResponse = Domain;

/// Response when listing domains
pub type ListDomainsResponse = Vec<Domain>;

/// Response for domain availability check
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainAvailabilityResponse {
    pub available: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
}
