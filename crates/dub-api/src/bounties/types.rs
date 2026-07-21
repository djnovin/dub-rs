use serde::{Deserialize, Serialize};

/// File information in a bounty submission
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileInfo {
    pub url: String,

    #[serde(rename = "fileName")]
    pub file_name: String,

    pub size: u64,
}

/// A bounty submission record
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BountySubmission {
    pub id: String,

    #[serde(rename = "bountyId")]
    pub bounty_id: String,

    #[serde(rename = "partnerId")]
    pub partner_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileInfo>>,

    /// Status: draft, submitted, approved, rejected
    pub status: String,

    #[serde(rename = "performanceCount", skip_serializing_if = "Option::is_none")]
    pub performance_count: Option<u32>,

    #[serde(rename = "socialMetricCount", skip_serializing_if = "Option::is_none")]
    pub social_metric_count: Option<i64>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "completedAt", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,

    #[serde(rename = "reviewedAt", skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,

    #[serde(rename = "rejectionReason", skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<String>,

    #[serde(rename = "rejectionNote", skip_serializing_if = "Option::is_none")]
    pub rejection_note: Option<String>,

    #[serde(rename = "periodNumber")]
    pub period_number: u64,

    #[serde(
        rename = "socialMetricsLastSyncedAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub social_metrics_last_synced_at: Option<String>,
}
