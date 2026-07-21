use dub_core::DubError;

#[test]
fn test_error_display() {
    let error = DubError::Config("test error".to_string());
    assert_eq!(format!("{}", error), "Configuration error: test error");

    let error = DubError::Api {
        status: 404,
        message: "Not found".to_string(),
    };
    assert_eq!(format!("{}", error), "API error: 404 - Not found");

    let error = DubError::Auth("Invalid token".to_string());
    assert_eq!(format!("{}", error), "Authentication error: Invalid token");
}

#[test]
fn test_error_debug() {
    let error = DubError::Config("test".to_string());
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("Config"));
}
