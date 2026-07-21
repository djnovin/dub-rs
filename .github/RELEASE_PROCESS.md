# Release Process

This document describes how to create releases for dub-rs.

## 🤖 Automated Release Process

The project uses GitHub Actions to automatically generate changelogs and create releases.

### How It Works

1. **Commit with Conventional Commits**: All commits follow the conventional commit format
2. **PRs are Labeled**: Each PR gets appropriate labels (type, priority, status)
3. **Tag a Version**: Push a version tag to trigger the release
4. **Automatic Changelog**: GitHub Actions generates a categorized changelog
5. **Release Created**: A GitHub release is created with the changelog
6. **CHANGELOG.md Updated**: The changelog file is automatically updated

### Creating a Release

#### 1. Prepare the Release

```bash
# Ensure you're on main and up to date
git checkout main
git pull origin main

# Ensure all tests pass
cargo test --workspace

# Ensure no warnings
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

#### 2. Update Version Numbers

Update version in these files:
- `crates/dub/Cargo.toml`
- `crates/dub-api/Cargo.toml`
- `crates/dub-core/Cargo.toml`

```bash
# Example: Update to v0.2.0
sed -i '' 's/^version = "0.1.0"/version = "0.2.0"/' crates/*/Cargo.toml
```

#### 3. Commit Version Bump

```bash
git add crates/*/Cargo.toml
git commit -S -m "chore: bump version to v0.2.0"
git push origin main
```

#### 4. Create and Push Tag

```bash
# Create annotated tag
git tag -a v0.2.0 -m "Release v0.2.0"

# Push tag (triggers release workflow)
git push origin v0.2.0
```

#### 5. Monitor Release Creation

1. Go to: https://github.com/djnovin/dub-rs/actions
2. Watch the "Release" workflow run
3. Once complete, check: https://github.com/djnovin/dub-rs/releases

The release will be automatically created with:
- ✅ Categorized changelog from PRs and commits
- ✅ Links to all merged PRs
- ✅ Contributor attribution
- ✅ Asset downloads

### Versioning Guidelines

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (`1.0.0 → 2.0.0`): Breaking API changes
  - Use label: `breaking change`
  - Example: Renaming public structs, changing function signatures

- **MINOR** (`0.1.0 → 0.2.0`): New features (backwards compatible)
  - Use label: `type: feature` or `type: enhancement`
  - Example: Adding new API endpoints, new optional parameters

- **PATCH** (`0.1.0 → 0.1.1`): Bug fixes (backwards compatible)
  - Use label: `type: bug`
  - Example: Fixing parsing errors, correcting documentation

### Pre-releases

For alpha, beta, or release candidates:

```bash
# Alpha release
git tag -a v0.2.0-alpha.1 -m "Release v0.2.0-alpha.1"
git push origin v0.2.0-alpha.1

# Beta release
git tag -a v0.2.0-beta.1 -m "Release v0.2.0-beta.1"
git push origin v0.2.0-beta.1

# Release candidate
git tag -a v0.2.0-rc.1 -m "Release v0.2.0-rc.1"
git push origin v0.2.0-rc.1
```

Pre-releases are automatically marked as "pre-release" on GitHub.

## 📝 Changelog Categories

The automated changelog groups changes into these categories:

### 🚀 Features
- Label: `type: feature`, `enhancement`, `type: enhancement`
- New functionality added to the SDK

### 🐛 Bug Fixes
- Label: `type: bug`, `bug`
- Fixes to existing functionality

### 📚 Documentation
- Label: `type: docs`, `documentation`
- Documentation improvements

### 🔧 Refactoring
- Label: `type: refactor`, `refactor`
- Code improvements without changing behavior

### ⚡ Performance
- Label: `type: perf`, `performance`
- Performance optimizations

### 🧪 Testing
- Label: `type: test`, `tests`
- Test additions or improvements

### 🏗️ Build & CI
- Label: `type: build`, `type: ci`, `build`, `ci`
- Build system or CI/CD changes

### 🔨 Chores & Maintenance
- Label: `type: chore`, `chore`, `dependencies`
- Maintenance tasks and dependency updates

### 💥 Breaking Changes
- Label: `breaking change`
- **Highlighted separately** - requires major version bump

### 🔒 Security
- Label: `security`
- Security-related fixes and improvements

## 🏷️ Labeling PRs for Changelog

When creating a PR, add appropriate labels:

**Required - Choose ONE type:**
- `type: feature` - New feature
- `type: bug` - Bug fix
- `type: docs` - Documentation
- `type: refactor` - Refactoring
- `type: perf` - Performance
- `type: test` - Tests
- `type: build` - Build changes
- `type: ci` - CI changes
- `type: chore` - Maintenance

**Optional but recommended:**
- `breaking change` - Breaking API change
- `security` - Security-related
- `dependencies` - Dependency updates

**Priority (optional):**
- `priority: critical`
- `priority: high`
- `priority: medium`
- `priority: low`

## 📦 Publishing to crates.io

### Prerequisites

1. **Create crates.io account**: https://crates.io/
2. **Get API token**: https://crates.io/settings/tokens
3. **Add token to GitHub**:
   - Go to: https://github.com/djnovin/dub-rs/settings/secrets/actions
   - Add secret: `CARGO_REGISTRY_TOKEN`

### Manual Publishing

```bash
# Login to crates.io
cargo login YOUR_API_TOKEN

# Publish in dependency order
cargo publish -p dub-core
sleep 30  # Wait for crates.io to index

cargo publish -p dub-api
sleep 30

cargo publish -p dub
```

### Automated Publishing

Once `CARGO_REGISTRY_TOKEN` is set in GitHub secrets:

1. Edit `.github/workflows/release.yml`
2. Uncomment the crates.io publishing lines
3. Push a tag - it will automatically publish

## 📋 Release Checklist

Before creating a release:

- [ ] All tests pass (`cargo test --workspace`)
- [ ] No clippy warnings (`cargo clippy --workspace -- -D warnings`)
- [ ] Code is formatted (`cargo fmt --all --check`)
- [ ] Documentation is updated
- [ ] Version numbers bumped in all Cargo.toml files
- [ ] Breaking changes documented
- [ ] Examples updated if needed
- [ ] README updated if needed

## 🔍 Viewing Release History

- **GitHub Releases**: https://github.com/djnovin/dub-rs/releases
- **CHANGELOG.md**: Automatically updated on each release
- **Tags**: `git tag -l` or https://github.com/djnovin/dub-rs/tags
- **Crates.io**: https://crates.io/crates/dub (once published)

## 🛠️ Manual Changelog Editing

If you need to manually edit a release:

1. Go to: https://github.com/djnovin/dub-rs/releases
2. Find the release
3. Click "Edit release"
4. Update the description
5. Save

The automated changelog is a starting point - feel free to add:
- Migration guides for breaking changes
- Upgrade instructions
- Known issues
- Thanks to contributors

## ❓ Troubleshooting

### Release workflow failed

1. Check Actions tab for error details
2. Common issues:
   - Tag format incorrect (must be `vX.Y.Z`)
   - Workflow file syntax error
   - GitHub token permissions

### Changelog is empty

1. Ensure PRs had proper labels
2. Check `.github/release.yml` configuration
3. Verify commits follow conventional commit format

### Can't push tag

1. Ensure you have write access to repository
2. Check if tag already exists: `git tag -l`
3. Delete remote tag if needed: `git push --delete origin v0.1.0`

## 📚 Resources

- [Semantic Versioning](https://semver.org/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [GitHub Releases](https://docs.github.com/en/repositories/releasing-projects-on-github)
- [Crates.io Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)

---

**Next Release**: Track progress at https://github.com/djnovin/dub-rs/milestones
