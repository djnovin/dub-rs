use serde::{Deserialize, Serialize};

/// Common pagination parameters
#[derive(Debug, Clone, Serialize, Default)]
pub struct PaginationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Paginated response wrapper
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

/// Pagination metadata
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginationMeta {
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

/// Common error response from the API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiError {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
