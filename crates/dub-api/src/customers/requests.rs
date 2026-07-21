use serde::Serialize;

/// Request to update a customer
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateCustomerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// Parameters for listing customers
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListCustomersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
