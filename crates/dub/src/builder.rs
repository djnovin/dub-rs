use dub_api::links::CreateLinkRequest;

/// Builder for creating links with a fluent API
///
/// # Example
///
/// ```no_run
/// # use dub::Dub;
/// # use dub::LinkBuilder;
/// # async fn example() -> dub::Result<()> {
/// let dub = Dub::new("token")?;
///
/// let link = LinkBuilder::new("https://example.com")
///     .domain("dub.sh")
///     .key("my-link")
///     .title("Example Site")
///     .description("An example website")
///     .tags(vec!["example", "demo"])
///     .create(&dub)
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default)]
pub struct LinkBuilder {
    request: CreateLinkRequest,
}

impl LinkBuilder {
    /// Create a new link builder with a destination URL
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            request: CreateLinkRequest {
                url: url.into(),
                ..Default::default()
            },
        }
    }

    /// Set the domain for the short link
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.request.domain = Some(domain.into());
        self
    }

    /// Set the key (slug) for the short link
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.request.key = Some(key.into());
        self
    }

    /// Set the title for the link
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = Some(title.into());
        self
    }

    /// Set the description for the link
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// Set the social media image URL
    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.request.image = Some(image.into());
        self
    }

    /// Set the iOS App Store redirect URL
    pub fn ios(mut self, ios: impl Into<String>) -> Self {
        self.request.ios = Some(ios.into());
        self
    }

    /// Set the Android Play Store redirect URL
    pub fn android(mut self, android: impl Into<String>) -> Self {
        self.request.android = Some(android.into());
        self
    }

    /// Set tags for the link
    pub fn tags<I, S>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.request.tags = Some(tags.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Add a single tag to the link
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        let tags = self.request.tags.get_or_insert_with(Vec::new);
        tags.push(tag.into());
        self
    }

    /// Set comments for the link
    pub fn comments(mut self, comments: impl Into<String>) -> Self {
        self.request.comments = Some(comments.into());
        self
    }

    /// Build the request without sending it
    pub fn build(self) -> CreateLinkRequest {
        self.request
    }

    /// Create the link by sending the request
    pub async fn create(self, client: &crate::Dub) -> crate::Result<crate::Link> {
        client.links().create(self.request).await
    }
}
