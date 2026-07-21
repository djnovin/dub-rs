use serde::{Deserialize, Serialize};

/// Analytics data point
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnalyticsDataPoint {
    pub clicks: u64,
    pub date: String,
}

/// Analytics summary
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnalyticsSummary {
    pub clicks: u64,
    pub data: Vec<AnalyticsDataPoint>,
}
