use chrono::Utc;
use dub_api::workspaces::Workspace;

#[test]
fn test_workspace_deserialization() {
    let json = r#"{
        "id": "ws_123",
        "name": "My Workspace",
        "slug": "my-workspace",
        "logo": "https://example.com/logo.png",
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-15T00:00:00Z"
    }"#;

    let workspace: Workspace = serde_json::from_str(json).unwrap();
    assert_eq!(workspace.id, "ws_123");
    assert_eq!(workspace.name, "My Workspace");
    assert_eq!(workspace.slug, "my-workspace");
    assert_eq!(
        workspace.logo.as_deref(),
        Some("https://example.com/logo.png")
    );
}

#[test]
fn test_workspace_deserialization_without_logo() {
    let json = r#"{
        "id": "ws_456",
        "name": "Test Workspace",
        "slug": "test-workspace",
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-15T00:00:00Z"
    }"#;

    let workspace: Workspace = serde_json::from_str(json).unwrap();
    assert_eq!(workspace.id, "ws_456");
    assert_eq!(workspace.name, "Test Workspace");
    assert_eq!(workspace.slug, "test-workspace");
    assert!(workspace.logo.is_none());
}

#[test]
fn test_workspace_serialization() {
    let workspace = Workspace {
        id: "ws_789".to_string(),
        name: "Serialization Test".to_string(),
        slug: "serialization-test".to_string(),
        logo: Some("https://example.com/test.png".to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let json = serde_json::to_string(&workspace).unwrap();
    assert!(json.contains("\"id\":\"ws_789\""));
    assert!(json.contains("\"name\":\"Serialization Test\""));
    assert!(json.contains("\"slug\":\"serialization-test\""));
    assert!(json.contains("\"createdAt\""));
    assert!(json.contains("\"updatedAt\""));
}

#[test]
fn test_workspace_serialization_without_logo() {
    let workspace = Workspace {
        id: "ws_999".to_string(),
        name: "No Logo".to_string(),
        slug: "no-logo".to_string(),
        logo: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let json = serde_json::to_string(&workspace).unwrap();
    // When logo is None, it should be skipped in serialization
    assert!(!json.contains("\"logo\""));
}

#[test]
fn test_workspace_roundtrip() {
    let original = Workspace {
        id: "ws_roundtrip".to_string(),
        name: "Roundtrip Test".to_string(),
        slug: "roundtrip-test".to_string(),
        logo: Some("https://example.com/roundtrip.png".to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Workspace = serde_json::from_str(&json).unwrap();

    assert_eq!(original.id, deserialized.id);
    assert_eq!(original.name, deserialized.name);
    assert_eq!(original.slug, deserialized.slug);
    assert_eq!(original.logo, deserialized.logo);
}

#[test]
fn test_workspace_field_names() {
    // Ensure camelCase conversion for created_at and updated_at
    let workspace = Workspace {
        id: "ws_fields".to_string(),
        name: "Field Test".to_string(),
        slug: "field-test".to_string(),
        logo: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let json = serde_json::to_string(&workspace).unwrap();
    assert!(json.contains("createdAt"));
    assert!(json.contains("updatedAt"));
    assert!(!json.contains("created_at"));
    assert!(!json.contains("updated_at"));
}

#[test]
fn test_workspace_empty_strings() {
    let json = r#"{
        "id": "",
        "name": "",
        "slug": "",
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-01T00:00:00Z"
    }"#;

    let workspace: Workspace = serde_json::from_str(json).unwrap();
    assert_eq!(workspace.id, "");
    assert_eq!(workspace.name, "");
    assert_eq!(workspace.slug, "");
}

#[test]
fn test_workspace_special_characters() {
    let json = r#"{
        "id": "ws_special",
        "name": "Test & Workspace <Special>",
        "slug": "test-workspace-special",
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-01T00:00:00Z"
    }"#;

    let workspace: Workspace = serde_json::from_str(json).unwrap();
    assert_eq!(workspace.name, "Test & Workspace <Special>");
}

#[test]
fn test_workspace_unicode() {
    let json = r#"{
        "id": "ws_unicode",
        "name": "测试工作区 🚀",
        "slug": "test-workspace",
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-01T00:00:00Z"
    }"#;

    let workspace: Workspace = serde_json::from_str(json).unwrap();
    assert_eq!(workspace.name, "测试工作区 🚀");
}

#[test]
fn test_workspace_long_strings() {
    let long_name = "A".repeat(1000);
    let workspace = Workspace {
        id: "ws_long".to_string(),
        name: long_name.clone(),
        slug: "long-slug".to_string(),
        logo: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let json = serde_json::to_string(&workspace).unwrap();
    let deserialized: Workspace = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, long_name);
}
