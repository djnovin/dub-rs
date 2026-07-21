use dub::{CreateLinkRequest, Dub, DubError, LinkBuilder, ListLinksParams, UpdateLinkRequest};
use dub_core::DubConfig;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ========== Links Integration Tests ==========

#[tokio::test]
async fn test_links_create_success() {
    let mock_server = MockServer::start().await;

    let link_response = json!({
        "id": "link_123",
        "domain": "dub.sh",
        "key": "test",
        "url": "https://example.com",
        "archived": false,
        "clicks": 0,
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-01T00:00:00Z",
        "projectId": "proj_123"
    });

    Mock::given(method("POST"))
        .and(path("/links"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&link_response))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let request = CreateLinkRequest {
        url: "https://example.com".to_string(),
        ..Default::default()
    };

    let result = dub.links().create(request).await;
    assert!(result.is_ok());

    let link = result.unwrap();
    assert_eq!(link.id, "link_123");
    assert_eq!(link.domain, "dub.sh");
    assert_eq!(link.url, "https://example.com");
}

#[tokio::test]
async fn test_links_create_with_builder() {
    let mock_server = MockServer::start().await;

    let link_response = json!({
        "id": "link_builder",
        "domain": "custom.dub",
        "key": "my-link",
        "url": "https://example.com",
        "title": "Test Link",
        "description": "A test",
        "tags": ["test", "demo"],
        "archived": false,
        "clicks": 0,
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-01T00:00:00Z",
        "projectId": "proj_123"
    });

    Mock::given(method("POST"))
        .and(path("/links"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&link_response))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = LinkBuilder::new("https://example.com")
        .domain("custom.dub")
        .key("my-link")
        .title("Test Link")
        .description("A test")
        .tags(vec!["test", "demo"])
        .create(&dub)
        .await;

    assert!(result.is_ok());
    let link = result.unwrap();
    assert_eq!(link.key, "my-link");
    assert_eq!(link.title.as_deref(), Some("Test Link"));
}

#[tokio::test]
async fn test_links_list_success() {
    let mock_server = MockServer::start().await;

    let links_response = json!([
        {
            "id": "link_1",
            "domain": "dub.sh",
            "key": "link1",
            "url": "https://example1.com",
            "archived": false,
            "clicks": 10,
            "createdAt": "2024-01-01T00:00:00Z",
            "updatedAt": "2024-01-01T00:00:00Z",
            "projectId": "proj_123"
        },
        {
            "id": "link_2",
            "domain": "dub.sh",
            "key": "link2",
            "url": "https://example2.com",
            "archived": false,
            "clicks": 20,
            "createdAt": "2024-01-02T00:00:00Z",
            "updatedAt": "2024-01-02T00:00:00Z",
            "projectId": "proj_123"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/links"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&links_response))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = dub.links().list(None).await;
    assert!(result.is_ok());

    let links = result.unwrap();
    assert_eq!(links.len(), 2);
    assert_eq!(links[0].id, "link_1");
    assert_eq!(links[1].id, "link_2");
}

#[tokio::test]
async fn test_links_list_with_params() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/links"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let params = ListLinksParams {
        domain: Some("dub.sh".to_string()),
        page: Some(1),
        limit: Some(50),
        ..Default::default()
    };

    let result = dub.links().list(Some(params)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_links_get_success() {
    let mock_server = MockServer::start().await;

    let link_response = json!({
        "id": "link_get",
        "domain": "dub.sh",
        "key": "get-test",
        "url": "https://example.com",
        "archived": false,
        "clicks": 100,
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-01T00:00:00Z",
        "projectId": "proj_123"
    });

    Mock::given(method("GET"))
        .and(path("/links/link_get"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&link_response))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = dub.links().get("link_get").await;
    assert!(result.is_ok());

    let link = result.unwrap();
    assert_eq!(link.id, "link_get");
    assert_eq!(link.clicks, 100);
}

#[tokio::test]
async fn test_links_get_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/links/nonexistent"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({"error": "Link not found"})))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = dub.links().get("nonexistent").await;
    assert!(result.is_err());

    match result.unwrap_err() {
        DubError::Api { status, .. } => assert_eq!(status, 404),
        _ => panic!("Expected Api error"),
    }
}

#[tokio::test]
async fn test_links_update_success() {
    let mock_server = MockServer::start().await;

    let updated_link = json!({
        "id": "link_update",
        "domain": "dub.sh",
        "key": "updated",
        "url": "https://updated.com",
        "title": "Updated Title",
        "archived": false,
        "clicks": 50,
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-15T00:00:00Z",
        "projectId": "proj_123"
    });

    Mock::given(method("PATCH"))
        .and(path("/links/link_update"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&updated_link))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let update_request = UpdateLinkRequest {
        title: Some("Updated Title".to_string()),
        url: Some("https://updated.com".to_string()),
        ..Default::default()
    };

    let result = dub.links().update("link_update", update_request).await;
    assert!(result.is_ok());

    let link = result.unwrap();
    assert_eq!(link.title.as_deref(), Some("Updated Title"));
}

#[tokio::test]
async fn test_links_delete_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/links/link_delete"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = dub.links().delete("link_delete").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_links_delete_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/links/missing"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({"error": "Link not found"})))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = dub.links().delete("missing").await;
    assert!(result.is_err());
}

// ========== Analytics Integration Tests ==========

#[tokio::test]
async fn test_analytics_get_success() {
    let mock_server = MockServer::start().await;

    let analytics_response = json!({
        "clicks": 150,
        "data": [
            {"clicks": 50, "date": "2024-01-01"},
            {"clicks": 100, "date": "2024-01-02"}
        ]
    });

    Mock::given(method("GET"))
        .and(path("/analytics/links/link_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&analytics_response))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = dub.analytics().get_link("link_123").await;
    assert!(result.is_ok());

    let summary = result.unwrap();
    assert_eq!(summary.clicks, 150);
    assert_eq!(summary.data.len(), 2);
}

#[tokio::test]
async fn test_analytics_empty_data() {
    let mock_server = MockServer::start().await;

    let analytics_response = json!({
        "clicks": 0,
        "data": []
    });

    Mock::given(method("GET"))
        .and(path("/analytics/links/empty_link"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&analytics_response))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = dub.analytics().get_link("empty_link").await;
    assert!(result.is_ok());

    let summary = result.unwrap();
    assert_eq!(summary.clicks, 0);
    assert!(summary.data.is_empty());
}

#[tokio::test]
async fn test_analytics_workspace_level() {
    let mock_server = MockServer::start().await;

    let analytics_response = json!({
        "clicks": 1000,
        "data": [
            {"clicks": 300, "date": "2024-01-01"},
            {"clicks": 400, "date": "2024-01-02"},
            {"clicks": 300, "date": "2024-01-03"}
        ]
    });

    Mock::given(method("GET"))
        .and(path("/analytics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&analytics_response))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = dub.analytics().get_workspace().await;
    assert!(result.is_ok());

    let summary = result.unwrap();
    assert_eq!(summary.clicks, 1000);
    assert_eq!(summary.data.len(), 3);
}

// ========== Workspaces Integration Tests ==========

#[tokio::test]
async fn test_workspaces_list_success() {
    let mock_server = MockServer::start().await;

    let workspaces_response = json!([
        {
            "id": "ws_1",
            "name": "Workspace 1",
            "slug": "workspace-1",
            "logo": "https://example.com/logo1.png",
            "createdAt": "2024-01-01T00:00:00Z",
            "updatedAt": "2024-01-01T00:00:00Z"
        },
        {
            "id": "ws_2",
            "name": "Workspace 2",
            "slug": "workspace-2",
            "createdAt": "2024-01-02T00:00:00Z",
            "updatedAt": "2024-01-02T00:00:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/workspaces"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&workspaces_response))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = dub.workspaces().list().await;
    assert!(result.is_ok());

    let workspaces = result.unwrap();
    assert_eq!(workspaces.len(), 2);
    assert_eq!(workspaces[0].id, "ws_1");
    assert_eq!(workspaces[1].id, "ws_2");
}

#[tokio::test]
async fn test_workspaces_get_success() {
    let mock_server = MockServer::start().await;

    let workspace_response = json!({
        "id": "ws_get",
        "name": "Get Workspace",
        "slug": "get-workspace",
        "logo": "https://example.com/logo.png",
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-01T00:00:00Z"
    });

    Mock::given(method("GET"))
        .and(path("/workspaces/ws_get"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&workspace_response))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = dub.workspaces().get("ws_get").await;
    assert!(result.is_ok());

    let workspace = result.unwrap();
    assert_eq!(workspace.id, "ws_get");
    assert_eq!(workspace.name, "Get Workspace");
}

// ========== Error Handling Tests ==========

#[tokio::test]
async fn test_unauthorized_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/links"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({"error": "Unauthorized"})))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("invalid-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = dub.links().list(None).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        DubError::Api { status, message } => {
            assert_eq!(status, 401);
            assert!(message.contains("Unauthorized"));
        }
        _ => panic!("Expected Api error"),
    }
}

#[tokio::test]
async fn test_rate_limit_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/links"))
        .respond_with(
            ResponseTemplate::new(429).set_body_json(json!({"error": "Rate limit exceeded"})),
        )
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let request = CreateLinkRequest {
        url: "https://example.com".to_string(),
        ..Default::default()
    };

    let result = dub.links().create(request).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        DubError::Api { status, .. } => assert_eq!(status, 429),
        _ => panic!("Expected Api error"),
    }
}

#[tokio::test]
async fn test_server_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/links"))
        .respond_with(
            ResponseTemplate::new(500).set_body_json(json!({"error": "Internal server error"})),
        )
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let result = dub.links().list(None).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        DubError::Api { status, .. } => assert_eq!(status, 500),
        _ => panic!("Expected Api error"),
    }
}

#[tokio::test]
async fn test_bad_request_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/links"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({"error": "Invalid URL"})))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    let request = CreateLinkRequest {
        url: "not-a-valid-url".to_string(),
        ..Default::default()
    };

    let result = dub.links().create(request).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        DubError::Api { status, message } => {
            assert_eq!(status, 400);
            assert!(message.contains("Invalid"));
        }
        _ => panic!("Expected Api error"),
    }
}

// ========== Concurrent Requests Tests ==========

#[tokio::test]
async fn test_concurrent_requests() {
    let mock_server = MockServer::start().await;

    let link_response = json!({
        "id": "link_concurrent",
        "domain": "dub.sh",
        "key": "concurrent",
        "url": "https://example.com",
        "archived": false,
        "clicks": 0,
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-01T00:00:00Z",
        "projectId": "proj_123"
    });

    Mock::given(method("GET"))
        .and(path("/links/link_concurrent"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&link_response))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let dub = Dub::with_config(config).unwrap();

    // Make multiple concurrent requests
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let dub_clone = dub.clone();
            tokio::spawn(async move { dub_clone.links().get("link_concurrent").await })
        })
        .collect();

    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}
