# Makefile for dub-rs development
# Mirrors CI checks for local development

.PHONY: help check fmt clippy test test-all build build-release doc clean audit security coverage install-tools

# Default target
help:
	@echo "dub-rs development tasks:"
	@echo ""
	@echo "  make check        - Run all checks (fmt, clippy, test)"
	@echo "  make fmt          - Format code"
	@echo "  make clippy       - Run clippy lints"
	@echo "  make test         - Run tests"
	@echo "  make test-all     - Run tests with all features"
	@echo "  make build        - Build all crates"
	@echo "  make build-release - Build release binaries"
	@echo "  make doc          - Build documentation"
	@echo "  make audit        - Run security audit"
	@echo "  make security     - Run all security checks"
	@echo "  make coverage     - Generate test coverage"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make install-tools - Install development tools"
	@echo ""

# Run all checks (like CI)
check: fmt clippy test doc
	@echo "✅ All checks passed!"

# Format code
fmt:
	@echo "🎨 Formatting code..."
	cargo fmt --all

# Check formatting
fmt-check:
	@echo "🔍 Checking formatting..."
	cargo fmt --all -- --check

# Run clippy
clippy:
	@echo "📎 Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
	@echo "🧪 Running tests..."
	cargo test --all-features

# Run tests with verbose output
test-verbose:
	@echo "🧪 Running tests (verbose)..."
	cargo test --all-features --verbose

# Run tests on all crates
test-all:
	@echo "🧪 Running all tests..."
	cargo test --all-features --workspace

# Build all crates
build:
	@echo "🔨 Building all crates..."
	cargo build --all-features

# Build release binaries
build-release:
	@echo "🔨 Building release binaries..."
	cargo build --release --all-features

# Build documentation
doc:
	@echo "📚 Building documentation..."
	cargo doc --all-features --no-deps --document-private-items

# Open documentation in browser
doc-open:
	@echo "📚 Opening documentation..."
	cargo doc --all-features --no-deps --open

# Run cargo audit
audit:
	@echo "🔒 Running security audit..."
	cargo audit

# Run cargo deny
deny:
	@echo "🚫 Running cargo deny..."
	cargo deny check

# Run all security checks
security: audit deny
	@echo "✅ Security checks passed!"

# Generate test coverage
coverage:
	@echo "📊 Generating test coverage..."
	cargo llvm-cov --all-features --workspace --html
	@echo "📊 Coverage report generated in target/llvm-cov/html/index.html"

# Generate coverage lcov format
coverage-lcov:
	@echo "📊 Generating test coverage (lcov)..."
	cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
	@echo "📊 Coverage report: lcov.info"

# Check for outdated dependencies
outdated:
	@echo "📦 Checking for outdated dependencies..."
	cargo outdated --root-deps-only

# Check for unused dependencies
udeps:
	@echo "📦 Checking for unused dependencies..."
	cargo +nightly udeps --all-targets --all-features

# Check minimal versions
minimal-versions:
	@echo "🔍 Checking minimal versions..."
	cargo +nightly minimal-versions check --all-features

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# Install development tools
install-tools:
	@echo "🔧 Installing development tools..."
	cargo install cargo-audit
	cargo install cargo-deny
	cargo install cargo-llvm-cov
	cargo install cargo-outdated
	cargo install cargo-udeps
	cargo install cargo-minimal-versions
	cargo install cargo-deadlinks
	@echo "✅ Development tools installed!"

# Run a specific crate's tests
test-core:
	@echo "🧪 Testing dub-core..."
	cargo test -p dub-core --all-features

test-api:
	@echo "🧪 Testing dub-api..."
	cargo test -p dub-api --all-features

test-dub:
	@echo "🧪 Testing dub..."
	cargo test -p dub --all-features

# Build a specific crate
build-core:
	@echo "🔨 Building dub-core..."
	cargo build -p dub-core --all-features

build-api:
	@echo "🔨 Building dub-api..."
	cargo build -p dub-api --all-features

build-dub:
	@echo "🔨 Building dub..."
	cargo build -p dub --all-features

# Pre-commit hook (run before committing)
pre-commit: fmt-check clippy test
	@echo "✅ Pre-commit checks passed!"

# Pre-push hook (run before pushing)
pre-push: check audit
	@echo "✅ Pre-push checks passed!"

# CI simulation (run everything CI runs)
ci: fmt-check clippy test-all doc audit deny
	@echo "✅ CI simulation complete!"
