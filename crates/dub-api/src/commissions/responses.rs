use serde::{Deserialize, Serialize};

/// Response from creating a commission
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateCommissionResponse {
    pub success: bool,
    pub message: String,
}
