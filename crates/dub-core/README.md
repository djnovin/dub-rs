# dub-core

Core functionality for the Dub.co Rust SDK.

This crate provides the foundational components used by the SDK:

- **HTTP Client**: `DubClient` - handles all HTTP requests to the Dub.co API
- **Configuration**: `DubConfig` - configurable API settings (base URL, timeout, auth)
- **Error Handling**: `DubError` - comprehensive error types
- **Handle**: `DubHandle` - thread-safe, cloneable client handle for resource management

## Usage

This crate is typically not used directly. Instead, use the `dub` crate which provides a higher-level API.

```rust
use dub_core::{DubConfig, DubClient};

let config = DubConfig::new("your-api-token")
    .with_timeout(60);

let client = DubClient::new(config)?;
```

## Features

- Async/await support via `reqwest`
- Configurable timeouts and base URLs
- Bearer token authentication
- Type-safe error handling with `thiserror`
