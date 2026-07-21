# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-07-21

### 🚀 Features

- Complete Rust SDK for Dub.co API
- 12 resources with 44 API endpoints
- Type-safe request/response models
- Async/await support with tokio
- Comprehensive error handling
- Automatic retries with exponential backoff
- Rate limiting support
- Connection pooling

### 📚 API Coverage

#### Core Resources
- **Links**: Create, update, delete, upsert links with full metadata support
- **Analytics**: Retrieve, count, timeseries, clicks, leads, sales metrics
- **Domains**: List, create, update, delete, transfer custom domains
- **Tags**: Create, update, delete workspace tags
- **Workspaces**: Get, create, update workspace information

#### Customer & Event Tracking
- **Customers**: Create, update, delete customer records
- **Events**: Track custom conversion events
- **Track**: Lead and sale conversion tracking

#### Partner Program
- **Partners**: Create, update, delete partner accounts
- **Payouts**: List partner payouts with filtering
- **Commissions**: Create, update, list commissions
- **Bounties**: List, approve, reject bounty submissions

### 🧪 Testing

- 161 comprehensive integration tests
- Test coverage across all API endpoints
- Mock-friendly architecture for testing

### 📖 Documentation

- Complete API reference documentation
- Usage examples for all resources
- Contribution guidelines
- Commit conventions guide
- Branch protection documentation

### 🔒 Security

- Signed commits required (GPG/SSH)
- Conventional commit enforcement
- Secret scanning enabled
- Vulnerability alerts active

### 🏗️ Infrastructure

- Terraform-managed GitHub repository
- Enterprise-grade branch protection
- Automated release workflows
- Automatic changelog generation
- CI/CD with GitHub Actions

### 🎯 Developer Experience

- Type-safe API with full IntelliSense support
- Builder patterns for complex requests
- Comprehensive error messages
- Examples directory with real-world usage
- Zero compiler warnings

[Unreleased]: https://github.com/djnovin/dub-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/djnovin/dub-rs/releases/tag/v0.1.0
