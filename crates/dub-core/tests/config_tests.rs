use dub_core::{DubConfig, DubError};

#[test]
fn test_config_new() {
    let config = DubConfig::new("test-token");
    assert_eq!(config.api_token(), "test-token");
    assert_eq!(config.base_url(), "https://api.dub.co");
    assert_eq!(config.timeout(), Some(30));
}

#[test]
fn test_config_with_base_url() {
    let config = DubConfig::new("test-token").with_base_url("https://custom.api.com");
    assert_eq!(config.base_url(), "https://custom.api.com");
}

#[test]
fn test_config_with_timeout() {
    let config = DubConfig::new("test-token").with_timeout(60);
    assert_eq!(config.timeout(), Some(60));
}

#[test]
fn test_config_without_timeout() {
    let config = DubConfig::new("test-token").without_timeout();
    assert_eq!(config.timeout(), None);
}

#[test]
fn test_config_validate_empty_token() {
    let config = DubConfig::new("");
    let result = config.validate();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DubError::Config(_)));
}

#[test]
fn test_config_validate_invalid_url() {
    let config = DubConfig::new("token").with_base_url("not-a-valid-url");
    let result = config.validate();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DubError::Config(_)));
}

#[test]
fn test_config_validate_success() {
    let config = DubConfig::new("valid-token");
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_builder_pattern() {
    let config = DubConfig::new("token")
        .with_base_url("https://api.example.com")
        .with_timeout(120);

    assert_eq!(config.api_token(), "token");
    assert_eq!(config.base_url(), "https://api.example.com");
    assert_eq!(config.timeout(), Some(120));
}
