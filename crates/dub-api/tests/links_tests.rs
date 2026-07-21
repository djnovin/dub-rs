use chrono::Utc;
use dub_api::links::{CreateLinkRequest, Link, ListLinksParams, UpdateLinkRequest};

#[test]
fn test_create_link_request_default() {
    let request = CreateLinkRequest {
        url: "https://example.com".to_string(),
        ..Default::default()
    };

    assert_eq!(request.url, "https://example.com");
    assert!(request.domain.is_none());
    assert!(request.key.is_none());
    assert!(request.title.is_none());
}

#[test]
fn test_create_link_request_full() {
    let request = CreateLinkRequest {
        url: "https://example.com".to_string(),
        domain: Some("dub.sh".to_string()),
        key: Some("test".to_string()),
        title: Some("Test Link".to_string()),
        description: Some("A test link".to_string()),
        tags: Some(vec!["test".to_string(), "demo".to_string()]),
        ..Default::default()
    };

    assert_eq!(request.url, "https://example.com");
    assert_eq!(request.domain.as_deref(), Some("dub.sh"));
    assert_eq!(request.key.as_deref(), Some("test"));
    assert_eq!(request.title.as_deref(), Some("Test Link"));
    assert_eq!(request.tags.as_ref().unwrap().len(), 2);
}

#[test]
fn test_update_link_request_partial() {
    let request = UpdateLinkRequest {
        title: Some("Updated Title".to_string()),
        archived: Some(true),
        ..Default::default()
    };

    assert!(request.url.is_none());
    assert_eq!(request.title.as_deref(), Some("Updated Title"));
    assert_eq!(request.archived, Some(true));
}

#[test]
fn test_link_request_serialization() {
    let request = CreateLinkRequest {
        url: "https://example.com".to_string(),
        domain: Some("dub.sh".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("example.com"));
    assert!(json.contains("dub.sh"));
}

#[test]
fn test_create_link_request_with_all_fields() {
    let request = CreateLinkRequest {
        url: "https://example.com".to_string(),
        domain: Some("dub.sh".to_string()),
        key: Some("my-key".to_string()),
        title: Some("My Title".to_string()),
        description: Some("My Description".to_string()),
        image: Some("https://example.com/image.png".to_string()),
        ios: Some("https://apps.apple.com/app".to_string()),
        android: Some("https://play.google.com/app".to_string()),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        comments: Some("Test comments".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("example.com"));
    assert!(json.contains("dub.sh"));
    assert!(json.contains("my-key"));
    assert!(json.contains("My Title"));
    assert!(json.contains("My Description"));
    assert!(json.contains("tag1"));
    assert!(json.contains("tag2"));
}

#[test]
fn test_create_link_request_skip_none_fields() {
    let request = CreateLinkRequest {
        url: "https://example.com".to_string(),
        domain: Some("dub.sh".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&request).unwrap();
    // Fields with None should not be serialized
    assert!(!json.contains("\"key\":"));
    assert!(!json.contains("\"title\":"));
    assert!(!json.contains("\"description\":"));
}

#[test]
fn test_update_link_request_all_fields() {
    let request = UpdateLinkRequest {
        url: Some("https://updated.com".to_string()),
        title: Some("Updated Title".to_string()),
        description: Some("Updated Description".to_string()),
        image: Some("https://updated.com/image.png".to_string()),
        ios: Some("https://apps.apple.com/updated".to_string()),
        android: Some("https://play.google.com/updated".to_string()),
        archived: Some(false),
        tags: Some(vec!["updated".to_string()]),
        comments: Some("Updated comments".to_string()),
    };

    assert!(request.url.is_some());
    assert_eq!(request.archived, Some(false));
    assert_eq!(request.tags.as_ref().unwrap().len(), 1);
}

#[test]
fn test_update_link_request_archive() {
    let request = UpdateLinkRequest {
        archived: Some(true),
        ..Default::default()
    };

    assert_eq!(request.archived, Some(true));
    assert!(request.url.is_none());
}

#[test]
fn test_list_links_params_default() {
    let params = ListLinksParams::default();

    assert!(params.domain.is_none());
    assert!(params.search.is_none());
    assert!(params.tags.is_none());
    assert!(params.archived.is_none());
    assert!(params.page.is_none());
    assert!(params.limit.is_none());
}

#[test]
fn test_list_links_params_with_filters() {
    let params = ListLinksParams {
        domain: Some("dub.sh".to_string()),
        search: Some("example".to_string()),
        tags: Some(vec!["production".to_string()]),
        archived: Some(false),
        page: Some(1),
        limit: Some(50),
    };

    assert_eq!(params.domain.as_deref(), Some("dub.sh"));
    assert_eq!(params.search.as_deref(), Some("example"));
    assert_eq!(params.page, Some(1));
    assert_eq!(params.limit, Some(50));
}

#[test]
fn test_list_links_params_serialization() {
    let params = ListLinksParams {
        domain: Some("dub.sh".to_string()),
        page: Some(2),
        limit: Some(100),
        ..Default::default()
    };

    let json = serde_json::to_string(&params).unwrap();
    assert!(json.contains("dub.sh"));
    assert!(json.contains("\"page\":2"));
    assert!(json.contains("\"limit\":100"));
}

#[test]
fn test_link_deserialization() {
    let json = r#"{
        "id": "link_123",
        "domain": "dub.sh",
        "key": "example",
        "url": "https://example.com",
        "title": "Example Site",
        "description": "An example website",
        "image": "https://example.com/og.png",
        "tags": ["test", "demo"],
        "archived": false,
        "clicks": 100,
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-15T00:00:00Z",
        "projectId": "proj_456",
        "comments": "Test link"
    }"#;

    let link: Link = serde_json::from_str(json).unwrap();
    assert_eq!(link.id, "link_123");
    assert_eq!(link.domain, "dub.sh");
    assert_eq!(link.key, "example");
    assert_eq!(link.url, "https://example.com");
    assert_eq!(link.title.as_deref(), Some("Example Site"));
    assert_eq!(link.clicks, 100);
    assert!(!link.archived);
    assert_eq!(link.tags.as_ref().unwrap().len(), 2);
}

#[test]
fn test_link_minimal_deserialization() {
    let json = r#"{
        "id": "link_min",
        "domain": "dub.sh",
        "key": "min",
        "url": "https://minimal.com",
        "archived": false,
        "clicks": 0,
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-01T00:00:00Z",
        "projectId": "proj_min"
    }"#;

    let link: Link = serde_json::from_str(json).unwrap();
    assert_eq!(link.id, "link_min");
    assert!(link.title.is_none());
    assert!(link.description.is_none());
    assert!(link.tags.is_none());
    assert_eq!(link.clicks, 0);
}

#[test]
fn test_link_serialization() {
    let link = Link {
        id: "link_ser".to_string(),
        domain: "dub.sh".to_string(),
        key: "serialize".to_string(),
        url: "https://serialize.com".to_string(),
        title: Some("Serialize Test".to_string()),
        description: None,
        image: None,
        ios: None,
        android: None,
        tags: Some(vec!["test".to_string()]),
        archived: false,
        clicks: 50,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        project_id: "proj_ser".to_string(),
        comments: None,
    };

    let json = serde_json::to_string(&link).unwrap();
    assert!(json.contains("\"id\":\"link_ser\""));
    assert!(json.contains("\"domain\":\"dub.sh\""));
    assert!(json.contains("\"clicks\":50"));
    assert!(json.contains("createdAt"));
    assert!(json.contains("updatedAt"));
    assert!(json.contains("projectId"));
}

#[test]
fn test_link_roundtrip() {
    let original = Link {
        id: "link_roundtrip".to_string(),
        domain: "example.com".to_string(),
        key: "test".to_string(),
        url: "https://target.com".to_string(),
        title: Some("Title".to_string()),
        description: Some("Desc".to_string()),
        image: None,
        ios: None,
        android: None,
        tags: Some(vec!["tag1".to_string()]),
        archived: false,
        clicks: 999,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        project_id: "proj_rt".to_string(),
        comments: Some("Comments".to_string()),
    };

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Link = serde_json::from_str(&json).unwrap();

    assert_eq!(original.id, deserialized.id);
    assert_eq!(original.domain, deserialized.domain);
    assert_eq!(original.key, deserialized.key);
    assert_eq!(original.clicks, deserialized.clicks);
}

#[test]
fn test_create_link_empty_tags() {
    let request = CreateLinkRequest {
        url: "https://example.com".to_string(),
        tags: Some(vec![]),
        ..Default::default()
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"tags\":[]"));
}

#[test]
fn test_update_link_clear_fields() {
    // An update request with Some(None) would require different modeling,
    // but we can test updating to new values
    let request = UpdateLinkRequest {
        url: Some("https://new-url.com".to_string()),
        title: Some("New Title".to_string()),
        ..Default::default()
    };

    assert!(request.url.is_some());
    assert!(request.description.is_none());
}
