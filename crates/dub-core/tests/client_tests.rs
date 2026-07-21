use dub_core::{DubClient, DubConfig, DubError};
use serde::{Deserialize, Serialize};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestResponse {
    message: String,
    count: u32,
}

#[derive(Debug, Serialize)]
struct TestRequest {
    name: String,
    value: i32,
}

#[tokio::test]
async fn test_client_new_with_valid_config() {
    let config = DubConfig::new("valid-token");
    let result = DubClient::new(config);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_new_with_empty_token() {
    let config = DubConfig::new("");
    let result = DubClient::new(config);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DubError::Config(_)));
}

#[tokio::test]
async fn test_client_new_with_invalid_base_url() {
    let config = DubConfig::new("token").with_base_url("not-a-url");
    let result = DubClient::new(config);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_client_get_success() {
    let mock_server = MockServer::start().await;

    let response_body = TestResponse {
        message: "success".to_string(),
        count: 42,
    };

    Mock::given(method("GET"))
        .and(path("/test"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result: Result<TestResponse, DubError> = client.get("/test").await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.message, "success");
    assert_eq!(response.count, 42);
}

#[tokio::test]
async fn test_client_get_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/not-found"))
        .respond_with(
            ResponseTemplate::new(404).set_body_string(r#"{"error": "Resource not found"}"#),
        )
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result: Result<TestResponse, DubError> = client.get("/not-found").await;
    assert!(result.is_err());

    match result.unwrap_err() {
        DubError::Api { status, message } => {
            assert_eq!(status, 404);
            assert!(message.contains("not found"));
        }
        _ => panic!("Expected Api error"),
    }
}

#[tokio::test]
async fn test_client_post_success() {
    let mock_server = MockServer::start().await;

    let request_body = TestRequest {
        name: "test".to_string(),
        value: 100,
    };

    let response_body = TestResponse {
        message: "created".to_string(),
        count: 1,
    };

    Mock::given(method("POST"))
        .and(path("/items"))
        .and(header("authorization", "Bearer test-token"))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result: Result<TestResponse, DubError> = client.post("/items", &request_body).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.message, "created");
}

#[tokio::test]
async fn test_client_post_bad_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/items"))
        .respond_with(
            ResponseTemplate::new(400).set_body_string(r#"{"error": "Invalid request body"}"#),
        )
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let request_body = TestRequest {
        name: "test".to_string(),
        value: -1,
    };

    let result: Result<TestResponse, DubError> = client.post("/items", &request_body).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        DubError::Api { status, message } => {
            assert_eq!(status, 400);
            assert!(message.contains("Invalid"));
        }
        _ => panic!("Expected Api error"),
    }
}

#[tokio::test]
async fn test_client_patch_success() {
    let mock_server = MockServer::start().await;

    let update_body = TestRequest {
        name: "updated".to_string(),
        value: 200,
    };

    let response_body = TestResponse {
        message: "updated".to_string(),
        count: 1,
    };

    Mock::given(method("PATCH"))
        .and(path("/items/123"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result: Result<TestResponse, DubError> = client.patch("/items/123", &update_body).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_put_success() {
    let mock_server = MockServer::start().await;

    let put_body = TestRequest {
        name: "replaced".to_string(),
        value: 300,
    };

    let response_body = TestResponse {
        message: "replaced".to_string(),
        count: 1,
    };

    Mock::given(method("PUT"))
        .and(path("/items/456"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result: Result<TestResponse, DubError> = client.put("/items/456", &put_body).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_delete_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/items/789"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result = client.delete("/items/789").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_delete_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/items/999"))
        .respond_with(ResponseTemplate::new(404).set_body_string(r#"{"error": "Item not found"}"#))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result = client.delete("/items/999").await;
    assert!(result.is_err());

    match result.unwrap_err() {
        DubError::Api { status, .. } => {
            assert_eq!(status, 404);
        }
        _ => panic!("Expected Api error"),
    }
}

#[tokio::test]
async fn test_client_unauthorized() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/secure"))
        .respond_with(ResponseTemplate::new(401).set_body_string(r#"{"error": "Unauthorized"}"#))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("invalid-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result: Result<TestResponse, DubError> = client.get("/secure").await;
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
async fn test_client_server_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/error"))
        .respond_with(
            ResponseTemplate::new(500).set_body_string(r#"{"error": "Internal server error"}"#),
        )
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result: Result<TestResponse, DubError> = client.get("/error").await;
    assert!(result.is_err());

    match result.unwrap_err() {
        DubError::Api { status, .. } => {
            assert_eq!(status, 500);
        }
        _ => panic!("Expected Api error"),
    }
}

#[tokio::test]
async fn test_client_timeout_configuration() {
    // Test that timeout is properly configured
    let config = DubConfig::new("test-token").with_timeout(5);
    let client = DubClient::new(config);
    assert!(client.is_ok());
    assert_eq!(client.unwrap().config().timeout(), Some(5));

    // Test without timeout
    let config = DubConfig::new("test-token").without_timeout();
    let client = DubClient::new(config);
    assert!(client.is_ok());
    assert_eq!(client.unwrap().config().timeout(), None);
}

#[tokio::test]
async fn test_client_authorization_header() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/check-auth"))
        .and(header("authorization", "Bearer my-secret-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&TestResponse {
            message: "authorized".to_string(),
            count: 1,
        }))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("my-secret-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result: Result<TestResponse, DubError> = client.get("/check-auth").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_content_type_header() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/test-headers"))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&TestResponse {
            message: "ok".to_string(),
            count: 1,
        }))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let request_body = TestRequest {
        name: "test".to_string(),
        value: 1,
    };

    let result: Result<TestResponse, DubError> = client.post("/test-headers", &request_body).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_json_deserialization_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/invalid-json"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not valid json"))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result: Result<TestResponse, DubError> = client.get("/invalid-json").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DubError::Request(_)));
}

#[tokio::test]
async fn test_client_custom_base_url() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/custom"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&TestResponse {
            message: "custom".to_string(),
            count: 99,
        }))
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result: Result<TestResponse, DubError> = client.get("/custom").await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.message, "custom");
}

#[tokio::test]
async fn test_client_rate_limit_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/rate-limited"))
        .respond_with(
            ResponseTemplate::new(429).set_body_string(r#"{"error": "Rate limit exceeded"}"#),
        )
        .mount(&mock_server)
        .await;

    let config = DubConfig::new("test-token").with_base_url(mock_server.uri());
    let client = DubClient::new(config).unwrap();

    let result: Result<TestResponse, DubError> = client.get("/rate-limited").await;
    assert!(result.is_err());

    match result.unwrap_err() {
        DubError::Api { status, message } => {
            assert_eq!(status, 429);
            assert!(message.contains("Rate limit"));
        }
        _ => panic!("Expected Api error"),
    }
}
