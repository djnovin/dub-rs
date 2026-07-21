# dub

The official Rust SDK for [Dub.co](https://dub.co) - URL shortening and link management.

## Installation

```toml
[dependencies]
dub = { path = "path/to/dub-rs/crates/dub" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use dub::{Dub, CreateLinkRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dub = Dub::new("your-api-token")?;
    
    // Create a short link
    let request = CreateLinkRequest {
        url: "https://example.com".to_string(),
        domain: Some("dub.sh".to_string()),
        key: Some("my-link".to_string()),
        ..Default::default()
    };
    
    let link = dub.links().create(request).await?;
    println!("Created: {}/{}", link.domain, link.key);
    
    Ok(())
}
```

## Resources

### Links

```rust
// List links
let links = dub.links().list(None).await?;

// Get a link
let link = dub.links().get("link-id").await?;

// Create a link
let request = CreateLinkRequest { /* ... */ };
let link = dub.links().create(request).await?;

// Update a link
let update = UpdateLinkRequest { /* ... */ };
let link = dub.links().update("link-id", update).await?;

// Delete a link
dub.links().delete("link-id").await?;
```

### Analytics

```rust
// Get analytics for a link
let analytics = dub.analytics().get_link("link-id").await?;
println!("Clicks: {}", analytics.clicks);

// Get workspace analytics
let analytics = dub.analytics().get_workspace().await?;
```

### Workspaces

```rust
// List workspaces
let workspaces = dub.workspaces().list().await?;

// Get a workspace
let workspace = dub.workspaces().get("workspace-id").await?;
```

## Configuration

```rust
use dub::Dub;
use dub_core::DubConfig;

let config = DubConfig::new("your-api-token")
    .with_base_url("https://api.dub.co")
    .with_timeout(60);

let dub = Dub::with_config(config)?;
```

## Error Handling

The SDK uses a `Result<T, DubError>` pattern:

```rust
use dub::DubError;

match dub.links().get("link-id").await {
    Ok(link) => println!("Found: {}", link.url),
    Err(DubError::Api { status, message }) => {
        eprintln!("API error {}: {}", status, message);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Examples

See the `examples/` directory for complete examples:

```bash
DUB_API_TOKEN=your-token cargo run --example basic
```
