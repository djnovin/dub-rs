# CI/CD Setup Checklist

## Required for Releases

### 1. Add GitHub Secret: `CARGO_REGISTRY_TOKEN`
- Get from: https://crates.io/settings/tokens
- Scope: "Publish new crates"
- Add in: GitHub Settings → Secrets and variables → Actions

### 2. Enable GitHub Pages (for docs)
- Settings → Pages → Source: **GitHub Actions**

## Optional

### Add `CODECOV_TOKEN` for coverage reports
- Get from: https://codecov.io

### Branch Protection (Recommended)
- Settings → Branches → Add rule for `main`
- Require status checks: `CI Success`, `Cargo Audit`

## Usage

### Creating a Release
```bash
git tag v1.0.0
git push origin v1.0.0
```

The release workflow will automatically:
1. Run all tests
2. Build multi-platform binaries
3. Publish to crates.io
4. Create GitHub release

### Local Development
```bash
make check    # Run all CI checks
make test     # Run tests
make fmt      # Format code
make clippy   # Run linter
```
