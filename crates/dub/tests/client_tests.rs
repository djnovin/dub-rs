use dub::{Dub, DubError};
use dub_core::DubConfig;

#[test]
fn test_dub_new_with_token() {
    let result = Dub::new("test-token");
    assert!(result.is_ok());
}

#[test]
fn test_dub_new_with_empty_token() {
    let result = Dub::new("");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DubError::Config(_)));
}

#[test]
fn test_dub_with_config() {
    let config = DubConfig::new("test-token")
        .with_base_url("https://custom.api.com")
        .with_timeout(60);

    let result = Dub::with_config(config);
    assert!(result.is_ok());
}

#[test]
fn test_dub_with_invalid_config() {
    let config = DubConfig::new("test-token").with_base_url("not-a-valid-url");

    let result = Dub::with_config(config);
    assert!(result.is_err());
}

#[test]
fn test_dub_links_resource() {
    let dub = Dub::new("test-token").unwrap();
    let links = dub.links();

    // Just verify we can create the resource
    // The actual API calls will be tested in integration tests
    drop(links);
}

#[test]
fn test_dub_analytics_resource() {
    let dub = Dub::new("test-token").unwrap();
    let analytics = dub.analytics();

    drop(analytics);
}

#[test]
fn test_dub_workspaces_resource() {
    let dub = Dub::new("test-token").unwrap();
    let workspaces = dub.workspaces();

    drop(workspaces);
}

#[test]
fn test_dub_multiple_resource_instances() {
    let dub = Dub::new("test-token").unwrap();

    let links1 = dub.links();
    let links2 = dub.links();
    let analytics = dub.analytics();
    let workspaces = dub.workspaces();

    // All resources should be independently usable
    drop(links1);
    drop(links2);
    drop(analytics);
    drop(workspaces);
}

#[test]
fn test_dub_clone() {
    let dub1 = Dub::new("test-token").unwrap();
    let dub2 = dub1.clone();

    // Both should be usable
    let _links1 = dub1.links();
    let _links2 = dub2.links();
}

#[test]
fn test_dub_send_sync() {
    // Verify Dub implements Send and Sync for use across threads
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<Dub>();
    assert_sync::<Dub>();
}

#[test]
fn test_dub_debug() {
    let dub = Dub::new("test-token").unwrap();
    let debug_str = format!("{:?}", dub);

    assert!(debug_str.contains("Dub"));
}

#[test]
fn test_dub_with_custom_timeout() {
    let config = DubConfig::new("test-token").with_timeout(120);
    let dub = Dub::with_config(config).unwrap();

    drop(dub);
}

#[test]
fn test_dub_without_timeout() {
    let config = DubConfig::new("test-token").without_timeout();
    let dub = Dub::with_config(config).unwrap();

    drop(dub);
}

#[test]
fn test_dub_with_custom_base_url() {
    let config = DubConfig::new("test-token").with_base_url("https://staging.api.dub.co");
    let dub = Dub::with_config(config).unwrap();

    drop(dub);
}

#[test]
fn test_dub_config_builder_pattern() {
    let config = DubConfig::new("test-token")
        .with_base_url("https://api.example.com")
        .with_timeout(90);

    let dub = Dub::with_config(config).unwrap();

    drop(dub);
}

#[tokio::test]
async fn test_dub_resource_lifetime() {
    let dub = Dub::new("test-token").unwrap();

    // Resources should outlive their creation scope
    let links = {
        let temp_dub = dub.clone();
        temp_dub.links()
    };

    drop(links);
}

#[test]
fn test_dub_with_long_token() {
    let long_token = "a".repeat(1000);
    let result = Dub::new(&long_token);
    assert!(result.is_ok());
}

#[test]
fn test_dub_with_special_chars_in_token() {
    let token = "token-with-special.chars_123";
    let result = Dub::new(token);
    assert!(result.is_ok());
}

#[test]
fn test_dub_error_types() {
    // Test that we can match on different error types
    let result = Dub::new("");

    match result {
        Ok(_) => panic!("Should have failed"),
        Err(DubError::Config(msg)) => {
            assert!(msg.contains("token"));
        }
        Err(_) => panic!("Wrong error type"),
    }
}

#[test]
fn test_dub_multiple_instances() {
    let dub1 = Dub::new("token1").unwrap();
    let dub2 = Dub::new("token2").unwrap();
    let dub3 = Dub::new("token3").unwrap();

    // All should be independently usable
    drop(dub1);
    drop(dub2);
    drop(dub3);
}

#[test]
fn test_dub_config_validation_before_client_creation() {
    // Empty token should fail validation
    let config = DubConfig::new("");
    let result = Dub::with_config(config);
    assert!(result.is_err());

    // Invalid URL should fail validation
    let config = DubConfig::new("token").with_base_url("invalid");
    let result = Dub::with_config(config);
    assert!(result.is_err());
}

#[test]
fn test_dub_token_ownership() {
    let token = String::from("owned-token");
    let result = Dub::new(token);
    assert!(result.is_ok());
}

#[test]
fn test_dub_token_str_slice() {
    let token: &str = "slice-token";
    let result = Dub::new(token);
    assert!(result.is_ok());
}
