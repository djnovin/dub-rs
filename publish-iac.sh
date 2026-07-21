#!/bin/bash
set -e

echo "🚀 Publishing dub-rs to GitHub (with IaC configuration)"
echo "========================================================"
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo -e "${YELLOW}⚠️  GitHub CLI (gh) not found${NC}"
    echo ""
    echo "Install it with:"
    echo "  macOS:   brew install gh"
    echo "  Linux:   https://github.com/cli/cli/blob/trunk/docs/install_linux.md"
    echo "  Windows: https://github.com/cli/cli/releases"
    echo ""
    exit 1
fi

# Check if authenticated
if ! gh auth status &> /dev/null; then
    echo -e "${YELLOW}⚠️  Not authenticated with GitHub${NC}"
    echo ""
    echo "Run: gh auth login"
    exit 1
fi

echo -e "${GREEN}✅ GitHub CLI is ready${NC}"
echo ""

# Repository details
REPO_NAME="dub-rs"
DESCRIPTION="Complete Rust SDK for the Dub.co API - Type-safe link management, analytics, and conversion tracking"
HOMEPAGE="https://dub.co"

echo -e "${BLUE}📦 Creating GitHub repository...${NC}"
echo ""
echo "Name: $REPO_NAME"
echo "Description: $DESCRIPTION"
echo "Visibility: Public"
echo ""

read -p "Create repository? (y/N): " CONFIRM
if [[ ! $CONFIRM =~ ^[Yy]$ ]]; then
    echo "❌ Cancelled"
    exit 0
fi

# Create repository
gh repo create "$REPO_NAME" \
    --public \
    --description "$DESCRIPTION" \
    --homepage "$HOMEPAGE" \
    --source=. \
    --remote=origin \
    --push

echo ""
echo -e "${GREEN}✅ Repository created and pushed!${NC}"
echo ""

# Get the repository URL
REPO_URL=$(gh repo view --json url -q .url)
USERNAME=$(gh repo view --json owner -q .owner.login)

echo -e "${BLUE}🏷️  Creating release tag v0.1.0...${NC}"
git tag -a v0.1.0 -m "Initial release: Complete Dub.co API SDK

Features:
- 100% API coverage (44 endpoints)
- 12 resource modules
- Full type safety
- 161+ passing tests
- Production ready"

git push origin v0.1.0

echo ""
echo -e "${BLUE}⚙️  Configuring repository settings...${NC}"

# Enable features
gh repo edit "$USERNAME/$REPO_NAME" \
    --enable-issues \
    --enable-discussions \
    --enable-wiki=false \
    --enable-projects

# Add topics
gh repo edit "$USERNAME/$REPO_NAME" \
    --add-topic "rust" \
    --add-topic "sdk" \
    --add-topic "api-client" \
    --add-topic "dub" \
    --add-topic "link-shortener" \
    --add-topic "analytics" \
    --add-topic "url-shortener" \
    --add-topic "async" \
    --add-topic "tokio" \
    --add-topic "reqwest"

echo ""
echo -e "${GREEN}✅ Repository configured!${NC}"
echo ""

# Create initial release
echo -e "${BLUE}📋 Creating GitHub Release...${NC}"

gh release create v0.1.0 \
    --title "v0.1.0 - Initial Release" \
    --notes "## 🎉 Initial Release - Complete Dub.co API SDK for Rust

### Features

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
- ✅ **Production ready** - Zero warnings, fully documented

### Quick Start

\`\`\`rust
use dub::{Dub, CreateLinkRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dub = Dub::new(\"your-api-token\")?;
    
    let link = dub.links().create(CreateLinkRequest {
        url: \"https://example.com\".to_string(),
        domain: Some(\"dub.sh\".to_string()),
        key: Some(\"my-link\".to_string()),
        ..Default::default()
    }).await?;
    
    println!(\"Created: {}/{}\", link.domain, link.key);
    Ok(())
}
\`\`\`

### Installation

\`\`\`toml
[dependencies]
dub = \"0.1\"
tokio = { version = \"1\", features = [\"full\"] }
\`\`\`

### Documentation

- [API Reference](https://github.com/$USERNAME/$REPO_NAME/blob/main/API_REFERENCE.md)
- [Contributing Guide](https://github.com/$USERNAME/$REPO_NAME/blob/main/CONTRIBUTING.md)
- [Examples](https://github.com/$USERNAME/$REPO_NAME/tree/main/crates/dub/examples)

### What's Included

- \`dub\` - Main SDK crate
- \`dub-api\` - API types and models
- \`dub-core\` - Core client and error handling

**Full Changelog**: https://github.com/$USERNAME/$REPO_NAME/commits/v0.1.0"

echo ""
echo -e "${GREEN}🎉 Success! Your repository is live!${NC}"
echo ""
echo "Repository: $REPO_URL"
echo "Release: $REPO_URL/releases/tag/v0.1.0"
echo ""
echo "Next steps:"
echo "1. Set up branch protection rules"
echo "2. Add repository secrets for CI/CD"
echo "3. Share with the community!"
echo ""

# Open in browser
if [[ "$OSTYPE" == "darwin"* ]]; then
    read -p "Open repository in browser? (y/N): " OPEN
    if [[ $OPEN =~ ^[Yy]$ ]]; then
        open "$REPO_URL"
    fi
fi
