# dub-rs

[![CI](https://github.com/novinnoori/dub-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/novinnoori/dub-rs/actions/workflows/ci.yml)
[![Security](https://github.com/novinnoori/dub-rs/actions/workflows/security.yml/badge.svg)](https://github.com/novinnoori/dub-rs/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rust SDK for the Dub.co API - Complete link management, analytics, and conversion tracking.

**Production-ready** • **Type-safe** • **Fully tested** • **Complete API coverage**

## Features

- ✅ **100% API Coverage** - All 44 Dub.co endpoints implemented
- ✅ **12 API Resources** - Links, Analytics, Domains, Tags, Customers, Events, Tracking, Partners, Payouts, Commissions, Bounties, Workspaces
- ✅ **Type-safe** with full serde integration
- ✅ **Async/await** via tokio and reqwest
- ✅ **Bulk operations** for efficient link management
- ✅ **Conversion tracking** for leads and sales
- ✅ **Partner program** with full analytics and payout management
- ✅ **Financial APIs** for commissions and payouts
- ✅ **Bounty management** with approval workflows
- ✅ **Builder patterns** for ergonomic API usage
- ✅ **161+ tests** with comprehensive coverage

## Installation

```toml
[dependencies]
dub = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use dub::{Dub, CreateLinkRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dub = Dub::new("your-api-token")?;
    
    // Create a short link
    let link = dub.links().create(CreateLinkRequest {
        url: "https://example.com".to_string(),
        domain: Some("dub.sh".to_string()),
        key: Some("my-link".to_string()),
        ..Default::default()
    }).await?;
    
    println!("Created: {}/{}", link.domain, link.key);
    
    // Get analytics
    let analytics = dub.analytics().get_link(&link.id).await?;
    println!("Clicks: {}", analytics.clicks);
    
    // Track a conversion
    use dub::TrackLeadRequest;
    dub.track().lead(TrackLeadRequest {
        click_id: "click_123".to_string(),
        event_name: "Sign Up".to_string(),
        customer_email: Some("user@example.com".to_string()),
        ..Default::default()
    }).await?;
    
    Ok(())
}
```

## API Resources

| Resource | Operations |
|----------|------------|
| **Links** | list, get, create, update, delete, upsert, count, bulk operations |
| **Analytics** | get link analytics, workspace analytics |
| **Domains** | list, create, update, delete, register, check availability |
| **Tags** | list, create, update |
| **Customers** | list, get, update, delete |
| **Events** | list (clicks, leads, sales) |
| **Tracking** | track leads, track sales |
| **Partners** | full partner/affiliate management with analytics |
| **Payouts** | list payouts with filtering |
| **Commissions** | list, create, update commissions |
| **Bounties** | list submissions, approve, reject |
| **Workspaces** | list, get |

**Total: 12 resources, 44 endpoints, 100% coverage**

## Documentation

- **[API Reference](API_REFERENCE.md)** - Complete endpoint documentation
- **[Examples](crates/dub/examples/)** - Code examples for all features
- **[Contributing Guide](CONTRIBUTING.md)** - How to contribute

## Examples

```bash
# Basic usage
cargo run -p dub --example basic

# Builder pattern
cargo run -p dub --example builder

# All API endpoints
cargo run -p dub --example comprehensive_api
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) before submitting PRs.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/novinnoori/dub-rs.git
cd dub-rs

# Run tests
cargo test --workspace

# Format code
cargo fmt --all

# Run linter
cargo clippy --workspace --all-targets -- -D warnings

# Build documentation
cargo doc --workspace --no-deps --open
```

### Quick Commands

```bash
make check    # Run all CI checks locally
make test     # Run tests
make fmt      # Format code
make clippy   # Run linter
make doc      # Build docs
```

## Project Structure

- **`dub`** - Main SDK crate with ergonomic API
- **`dub-api`** - API types and models
- **`dub-core`** - Core client, config, and error handling

## License

MIT
