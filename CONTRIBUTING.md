# Contributing to dub-rs

Thank you for your interest in contributing to dub-rs! This guide will help you get started.

## Code of Conduct

Be respectful and inclusive. We aim to maintain a welcoming community for everyone.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- A Dub.co account with an API token (for testing)

### Setup

1. Fork and clone the repository:
```bash
git clone https://github.com/YOUR_USERNAME/dub-rs.git
cd dub-rs
```

2. Build the project:
```bash
cargo build --workspace
```

3. Run tests:
```bash
cargo test --workspace
```

## Development Workflow

### Making Changes

1. Create a new branch:
```bash
git checkout -b feature/your-feature-name
```

2. Make your changes following our [coding standards](#coding-standards)

3. Add tests for your changes

4. **Follow commit conventions** (see [COMMIT_CONVENTIONS.md](.github/COMMIT_CONVENTIONS.md)):
```bash
git commit -S -m "feat: add your feature description"
```

5. Ensure all checks pass:
```bash
make check
```

Or run individually:
```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```

### Commit Messages

**All commits must follow conventional commit format and be signed.**

See our detailed [Commit Conventions Guide](.github/COMMIT_CONVENTIONS.md) for:
- ✅ Conventional commit format
- ✅ GPG/SSH signing setup
- ✅ Valid commit examples
- ✅ Troubleshooting

**Quick Reference:**
```bash
# Format: <type>(<scope>): <subject>
feat: add analytics endpoint
fix: resolve timeout error
docs: update API reference

# All commits must be signed (-S flag)
git commit -S -m "feat: your message"
```

**Valid types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`

### Pull Requests

1. Push your changes to your fork
2. Open a pull request against `main`
3. Fill out the PR template
4. Ensure CI passes
5. Wait for review

## Coding Standards

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting (enforced by CI)
- Use `cargo clippy` for linting (no warnings allowed)
- Write idiomatic Rust code

### Code Organization

```
dub-rs/
├── crates/
│   ├── dub/           # Main SDK crate
│   │   ├── src/
│   │   │   ├── client.rs      # Main Dub client
│   │   │   ├── builder.rs     # Builder patterns
│   │   │   └── resources/     # API resources
│   │   └── examples/          # Usage examples
│   ├── dub-api/       # API types and models
│   │   └── src/
│   │       ├── links/         # Links types
│   │       ├── analytics/     # Analytics types
│   │       └── ...            # Other resources
│   └── dub-core/      # Core functionality
│       └── src/
│           ├── client.rs      # HTTP client
│           ├── config.rs      # Configuration
│           └── error.rs       # Error types
```

### Adding New API Endpoints

When adding a new endpoint, follow this pattern:

1. **Define types** in `dub-api/src/{resource}/`:
   ```rust
   // types.rs
   pub struct Resource {
       pub id: String,
       // ... fields with proper serde attributes
   }
   
   // requests.rs
   pub struct CreateResourceRequest {
       // ... request fields
   }
   
   // responses.rs
   pub type ResourceResponse = Resource;
   ```

2. **Create resource** in `dub/src/resources/{resource}.rs`:
   ```rust
   pub struct Resources {
       handle: DubHandle,
   }
   
   impl Resources {
       pub async fn list(&self) -> Result<Vec<Resource>> {
           // Implementation
       }
   }
   ```

3. **Add to client** in `dub/src/client.rs`:
   ```rust
   pub fn resources(&self) -> Resources {
       Resources::new(self.handle.clone())
   }
   ```

4. **Export types** in `dub/src/lib.rs`:
   ```rust
   pub use dub_api::resource::{Resource, CreateResourceRequest};
   ```

5. **Add examples** in `dub/examples/`

6. **Write tests** in appropriate test files

### Documentation

- Add doc comments to all public APIs
- Include usage examples in doc comments
- Update README.md if adding major features
- Update API_REFERENCE.md for new endpoints

Example:
```rust
/// Create a new link
///
/// # Example
///
/// ```no_run
/// # use dub::Dub;
/// # async fn example() -> dub::Result<()> {
/// let dub = Dub::new("token")?;
/// let link = dub.links().create(request).await?;
/// # Ok(())
/// # }
/// ```
pub async fn create(&self, request: CreateLinkRequest) -> Result<Link> {
    // Implementation
}
```

### Testing

- Write unit tests for logic
- Write integration tests for API interactions
- Use mocks for HTTP requests
- Aim for high test coverage

Example test:
```rust
#[tokio::test]
async fn test_list_links() {
    let dub = Dub::new("test-token").unwrap();
    let result = dub.links().list(None).await;
    assert!(result.is_ok());
}
```

## Project-Specific Guidelines

### Error Handling

- Use `Result<T, DubError>` for all fallible operations
- Provide helpful error messages
- Include context in error variants

### Async/Await

- All API methods must be async
- Use `tokio` runtime
- Avoid blocking operations

### Serialization

- Use `serde` for JSON serialization
- API fields use camelCase (via `#[serde(rename)]`)
- Rust fields use snake_case
- Optional fields use `Option<T>` with `skip_serializing_if`

Example:
```rust
#[derive(Serialize)]
pub struct Request {
    #[serde(rename = "customerId")]
    pub customer_id: Option<String>,
}
```

## Running CI Locally

Before pushing, run the same checks that CI runs:

```bash
# Format check
cargo fmt --all --check

# Clippy (no warnings allowed)
cargo clippy --workspace --all-targets -- -D warnings

# Tests
cargo test --workspace

# Documentation
cargo doc --workspace --no-deps

# Or run all at once
make check
```

## Release Process

Releases are automated via GitHub Actions:

1. Update version in `Cargo.toml` files
2. Update CHANGELOG.md
3. Commit changes: `git commit -m "Release vX.Y.Z"`
4. Tag release: `git tag vX.Y.Z`
5. Push: `git push origin main --tags`

CI will automatically:
- Run all tests
- Build release artifacts
- Publish to crates.io
- Create GitHub release

## Questions?

- Open an issue for bugs or feature requests
- Start a discussion for questions
- Check existing issues and discussions first

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
