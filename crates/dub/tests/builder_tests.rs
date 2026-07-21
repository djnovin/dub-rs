use dub::LinkBuilder;

#[test]
fn test_link_builder_new() {
    let builder = LinkBuilder::new("https://example.com");
    let request = builder.build();

    assert_eq!(request.url, "https://example.com");
    assert!(request.domain.is_none());
    assert!(request.key.is_none());
}

#[test]
fn test_link_builder_with_domain() {
    let builder = LinkBuilder::new("https://example.com").domain("dub.sh");
    let request = builder.build();

    assert_eq!(request.url, "https://example.com");
    assert_eq!(request.domain.as_deref(), Some("dub.sh"));
}

#[test]
fn test_link_builder_with_key() {
    let builder = LinkBuilder::new("https://example.com").key("my-link");
    let request = builder.build();

    assert_eq!(request.key.as_deref(), Some("my-link"));
}

#[test]
fn test_link_builder_with_title() {
    let builder = LinkBuilder::new("https://example.com").title("Example Site");
    let request = builder.build();

    assert_eq!(request.title.as_deref(), Some("Example Site"));
}

#[test]
fn test_link_builder_with_description() {
    let builder = LinkBuilder::new("https://example.com").description("A test link");
    let request = builder.build();

    assert_eq!(request.description.as_deref(), Some("A test link"));
}

#[test]
fn test_link_builder_with_image() {
    let builder = LinkBuilder::new("https://example.com").image("https://example.com/og.png");
    let request = builder.build();

    assert_eq!(request.image.as_deref(), Some("https://example.com/og.png"));
}

#[test]
fn test_link_builder_with_ios() {
    let builder = LinkBuilder::new("https://example.com").ios("https://apps.apple.com/app");
    let request = builder.build();

    assert_eq!(request.ios.as_deref(), Some("https://apps.apple.com/app"));
}

#[test]
fn test_link_builder_with_android() {
    let builder = LinkBuilder::new("https://example.com").android("https://play.google.com/app");
    let request = builder.build();

    assert_eq!(
        request.android.as_deref(),
        Some("https://play.google.com/app")
    );
}

#[test]
fn test_link_builder_with_tags_vec() {
    let tags = vec!["tag1", "tag2", "tag3"];
    let builder = LinkBuilder::new("https://example.com").tags(tags);
    let request = builder.build();

    assert_eq!(request.tags.as_ref().unwrap().len(), 3);
    assert_eq!(request.tags.as_ref().unwrap()[0], "tag1");
    assert_eq!(request.tags.as_ref().unwrap()[1], "tag2");
    assert_eq!(request.tags.as_ref().unwrap()[2], "tag3");
}

#[test]
fn test_link_builder_with_single_tag() {
    let builder = LinkBuilder::new("https://example.com").tag("single-tag");
    let request = builder.build();

    assert_eq!(request.tags.as_ref().unwrap().len(), 1);
    assert_eq!(request.tags.as_ref().unwrap()[0], "single-tag");
}

#[test]
fn test_link_builder_with_multiple_single_tags() {
    let builder = LinkBuilder::new("https://example.com")
        .tag("tag1")
        .tag("tag2")
        .tag("tag3");
    let request = builder.build();

    assert_eq!(request.tags.as_ref().unwrap().len(), 3);
    assert!(request.tags.as_ref().unwrap().contains(&"tag1".to_string()));
    assert!(request.tags.as_ref().unwrap().contains(&"tag2".to_string()));
    assert!(request.tags.as_ref().unwrap().contains(&"tag3".to_string()));
}

#[test]
fn test_link_builder_tags_then_tag() {
    // Test that tag() can be used after tags() to add more
    let builder = LinkBuilder::new("https://example.com")
        .tags(vec!["tag1", "tag2"])
        .tag("tag3");
    let request = builder.build();

    assert_eq!(request.tags.as_ref().unwrap().len(), 3);
}

#[test]
fn test_link_builder_with_comments() {
    let builder = LinkBuilder::new("https://example.com").comments("Internal notes");
    let request = builder.build();

    assert_eq!(request.comments.as_deref(), Some("Internal notes"));
}

#[test]
fn test_link_builder_chaining() {
    let builder = LinkBuilder::new("https://example.com")
        .domain("dub.sh")
        .key("test-link")
        .title("Test Link")
        .description("A test description")
        .image("https://example.com/image.png")
        .tags(vec!["test", "demo"])
        .comments("Test comments");

    let request = builder.build();

    assert_eq!(request.url, "https://example.com");
    assert_eq!(request.domain.as_deref(), Some("dub.sh"));
    assert_eq!(request.key.as_deref(), Some("test-link"));
    assert_eq!(request.title.as_deref(), Some("Test Link"));
    assert_eq!(request.description.as_deref(), Some("A test description"));
    assert_eq!(
        request.image.as_deref(),
        Some("https://example.com/image.png")
    );
    assert_eq!(request.tags.as_ref().unwrap().len(), 2);
    assert_eq!(request.comments.as_deref(), Some("Test comments"));
}

#[test]
fn test_link_builder_all_fields() {
    let builder = LinkBuilder::new("https://example.com")
        .domain("custom.domain")
        .key("custom-key")
        .title("Complete Link")
        .description("Full description")
        .image("https://example.com/full.png")
        .ios("https://apps.apple.com/full")
        .android("https://play.google.com/full")
        .tags(vec!["tag1", "tag2", "tag3"])
        .comments("All fields set");

    let request = builder.build();

    assert_eq!(request.url, "https://example.com");
    assert!(request.domain.is_some());
    assert!(request.key.is_some());
    assert!(request.title.is_some());
    assert!(request.description.is_some());
    assert!(request.image.is_some());
    assert!(request.ios.is_some());
    assert!(request.android.is_some());
    assert!(request.tags.is_some());
    assert!(request.comments.is_some());
}

#[test]
fn test_link_builder_minimal() {
    let builder = LinkBuilder::new("https://minimal.com");
    let request = builder.build();

    assert_eq!(request.url, "https://minimal.com");
    assert!(request.domain.is_none());
    assert!(request.key.is_none());
    assert!(request.title.is_none());
    assert!(request.description.is_none());
    assert!(request.image.is_none());
    assert!(request.ios.is_none());
    assert!(request.android.is_none());
    assert!(request.tags.is_none());
    assert!(request.comments.is_none());
}

#[test]
fn test_link_builder_string_ownership() {
    let url = String::from("https://owned.com");
    let domain = String::from("owned.domain");

    let builder = LinkBuilder::new(url).domain(domain);
    let request = builder.build();

    assert_eq!(request.url, "https://owned.com");
    assert_eq!(request.domain.as_deref(), Some("owned.domain"));
}

#[test]
fn test_link_builder_str_slices() {
    let url: &str = "https://slice.com";
    let domain: &str = "slice.domain";

    let builder = LinkBuilder::new(url).domain(domain);
    let request = builder.build();

    assert_eq!(request.url, "https://slice.com");
    assert_eq!(request.domain.as_deref(), Some("slice.domain"));
}

#[test]
fn test_link_builder_empty_tags() {
    let builder = LinkBuilder::new("https://example.com").tags(Vec::<String>::new());
    let request = builder.build();

    assert_eq!(request.tags.as_ref().unwrap().len(), 0);
}

#[test]
fn test_link_builder_unicode() {
    let builder = LinkBuilder::new("https://example.com")
        .title("测试链接 🔗")
        .description("日本語のテスト")
        .tag("中文");

    let request = builder.build();

    assert_eq!(request.title.as_deref(), Some("测试链接 🔗"));
    assert_eq!(request.description.as_deref(), Some("日本語のテスト"));
    assert!(request.tags.as_ref().unwrap().contains(&"中文".to_string()));
}

#[test]
fn test_link_builder_special_characters() {
    let builder = LinkBuilder::new("https://example.com?param=value&other=123")
        .key("special-key-with-dashes")
        .title("Title with \"quotes\" and 'apostrophes'")
        .description("Description & more <html> tags");

    let request = builder.build();

    assert!(request.url.contains("?param=value"));
    assert_eq!(
        request.title.as_deref(),
        Some("Title with \"quotes\" and 'apostrophes'")
    );
}

#[test]
fn test_link_builder_long_values() {
    let long_url = format!("https://example.com/{}", "a".repeat(1000));
    let long_title = "T".repeat(500);

    let builder = LinkBuilder::new(&long_url).title(&long_title);
    let request = builder.build();

    assert_eq!(request.url.len(), long_url.len());
    assert_eq!(request.title.as_ref().unwrap().len(), 500);
}

#[test]
fn test_link_builder_clone() {
    let builder = LinkBuilder::new("https://example.com")
        .domain("dub.sh")
        .title("Cloneable");

    let cloned = builder.clone();
    let request1 = builder.build();
    let request2 = cloned.build();

    assert_eq!(request1.url, request2.url);
    assert_eq!(request1.domain, request2.domain);
    assert_eq!(request1.title, request2.title);
}

#[test]
fn test_link_builder_default() {
    let builder = LinkBuilder::default();
    let request = builder.build();

    // Default should have empty url
    assert_eq!(request.url, "");
}

#[test]
fn test_link_builder_modify_after_build() {
    let builder = LinkBuilder::new("https://example.com").title("Original");

    let request1 = builder.clone().build();
    let request2 = builder.title("Modified").build();

    assert_eq!(request1.title.as_deref(), Some("Original"));
    assert_eq!(request2.title.as_deref(), Some("Modified"));
}

#[test]
fn test_link_builder_tags_iterator() {
    let tags: Vec<&str> = vec!["a", "b", "c"];
    let builder = LinkBuilder::new("https://example.com").tags(tags);
    let request = builder.build();

    assert_eq!(request.tags.as_ref().unwrap().len(), 3);
}

#[test]
fn test_link_builder_with_all_mobile_platforms() {
    let builder = LinkBuilder::new("https://example.com")
        .ios("https://apps.apple.com/app/id123")
        .android("https://play.google.com/store/apps/details?id=com.example");

    let request = builder.build();

    assert!(request.ios.is_some());
    assert!(request.android.is_some());
    assert!(request.ios.as_ref().unwrap().contains("apple.com"));
    assert!(request
        .android
        .as_ref()
        .unwrap()
        .contains("play.google.com"));
}
