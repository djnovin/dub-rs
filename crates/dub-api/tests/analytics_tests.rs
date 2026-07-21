use dub_api::analytics::{AnalyticsDataPoint, AnalyticsSummary};

#[test]
fn test_analytics_data_point_new() {
    let data_point = AnalyticsDataPoint {
        clicks: 100,
        date: "2024-01-15".to_string(),
    };

    assert_eq!(data_point.clicks, 100);
    assert_eq!(data_point.date, "2024-01-15");
}

#[test]
fn test_analytics_data_point_serialization() {
    let data_point = AnalyticsDataPoint {
        clicks: 42,
        date: "2024-01-01".to_string(),
    };

    let json = serde_json::to_string(&data_point).unwrap();
    assert!(json.contains("\"clicks\":42"));
    assert!(json.contains("\"date\":\"2024-01-01\""));
}

#[test]
fn test_analytics_data_point_deserialization() {
    let json = r#"{"clicks":150,"date":"2024-02-15"}"#;
    let data_point: AnalyticsDataPoint = serde_json::from_str(json).unwrap();

    assert_eq!(data_point.clicks, 150);
    assert_eq!(data_point.date, "2024-02-15");
}

#[test]
fn test_analytics_summary_new() {
    let data = vec![
        AnalyticsDataPoint {
            clicks: 10,
            date: "2024-01-01".to_string(),
        },
        AnalyticsDataPoint {
            clicks: 20,
            date: "2024-01-02".to_string(),
        },
    ];

    let summary = AnalyticsSummary { clicks: 30, data };

    assert_eq!(summary.clicks, 30);
    assert_eq!(summary.data.len(), 2);
}

#[test]
fn test_analytics_summary_serialization() {
    let summary = AnalyticsSummary {
        clicks: 100,
        data: vec![AnalyticsDataPoint {
            clicks: 50,
            date: "2024-01-01".to_string(),
        }],
    };

    let json = serde_json::to_string(&summary).unwrap();
    assert!(json.contains("\"clicks\":100"));
    assert!(json.contains("\"data\""));
}

#[test]
fn test_analytics_summary_deserialization() {
    let json = r#"{
        "clicks": 250,
        "data": [
            {"clicks": 100, "date": "2024-01-01"},
            {"clicks": 150, "date": "2024-01-02"}
        ]
    }"#;

    let summary: AnalyticsSummary = serde_json::from_str(json).unwrap();
    assert_eq!(summary.clicks, 250);
    assert_eq!(summary.data.len(), 2);
    assert_eq!(summary.data[0].clicks, 100);
    assert_eq!(summary.data[1].clicks, 150);
}

#[test]
fn test_analytics_summary_empty_data() {
    let summary = AnalyticsSummary {
        clicks: 0,
        data: vec![],
    };

    assert_eq!(summary.clicks, 0);
    assert!(summary.data.is_empty());

    let json = serde_json::to_string(&summary).unwrap();
    let deserialized: AnalyticsSummary = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.clicks, 0);
    assert!(deserialized.data.is_empty());
}

#[test]
fn test_analytics_data_point_zero_clicks() {
    let data_point = AnalyticsDataPoint {
        clicks: 0,
        date: "2024-01-01".to_string(),
    };

    let json = serde_json::to_string(&data_point).unwrap();
    let deserialized: AnalyticsDataPoint = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.clicks, 0);
}

#[test]
fn test_analytics_summary_large_numbers() {
    let summary = AnalyticsSummary {
        clicks: u64::MAX,
        data: vec![AnalyticsDataPoint {
            clicks: u64::MAX,
            date: "2024-01-01".to_string(),
        }],
    };

    let json = serde_json::to_string(&summary).unwrap();
    let deserialized: AnalyticsSummary = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.clicks, u64::MAX);
}

#[test]
fn test_analytics_date_formats() {
    // Test various date string formats
    let formats = vec![
        "2024-01-01",
        "2024-12-31",
        "2024-02-29", // leap year
        "2024-01-01T00:00:00Z",
    ];

    for date_str in formats {
        let data_point = AnalyticsDataPoint {
            clicks: 1,
            date: date_str.to_string(),
        };

        let json = serde_json::to_string(&data_point).unwrap();
        let deserialized: AnalyticsDataPoint = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.date, date_str);
    }
}

#[test]
fn test_analytics_summary_multiple_data_points() {
    let data = (0..10)
        .map(|i| AnalyticsDataPoint {
            clicks: i * 10,
            date: format!("2024-01-{:02}", i + 1),
        })
        .collect::<Vec<_>>();

    let total_clicks: u64 = data.iter().map(|d| d.clicks).sum();
    let summary = AnalyticsSummary {
        clicks: total_clicks,
        data,
    };

    assert_eq!(summary.data.len(), 10);
    assert_eq!(summary.clicks, 450); // 0+10+20+...+90

    let json = serde_json::to_string(&summary).unwrap();
    let deserialized: AnalyticsSummary = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.data.len(), 10);
    assert_eq!(deserialized.clicks, 450);
}
