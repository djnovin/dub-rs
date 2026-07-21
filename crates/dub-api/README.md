# dub-api

Generated API types and models for the Dub.co API.

This crate contains the types, request/response models, and data structures that match the Dub.co API specification. These types are designed to be generated from the OpenAPI spec.

## Structure

- `links/` - Link management types (Link, CreateLinkRequest, UpdateLinkRequest)
- `analytics/` - Analytics data types
- `workspaces/` - Workspace/project types
- `common.rs` - Common types used across resources

## Usage

This crate is typically not used directly. Instead, use the `dub` crate which re-exports these types.

```rust
use dub_api::links::{Link, CreateLinkRequest};

let request = CreateLinkRequest {
    url: "https://example.com".to_string(),
    ..Default::default()
};
```

## Code Generation

In the future, this crate's contents can be automatically generated from the Dub.co OpenAPI specification using tools like `openapi-generator` or custom code generation scripts.
