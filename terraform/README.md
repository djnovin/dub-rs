# Terraform IaC Setup for dub-rs Repository

This directory contains Infrastructure as Code (IaC) configuration for managing the dub-rs GitHub repository.

## Quick Start with GitHub CLI (Recommended)

The easiest way to publish with full automation:

```bash
cd ~/dub-rs

# Make sure gh CLI is installed and authenticated
gh auth status

# Run the automated IaC script
./publish-iac.sh
```

This will:
- Create the repository
- Push code and tags
- Configure settings (topics, features, etc.)
- Create the initial release
- All done via GitHub CLI!

## Using Terraform

For more control and infrastructure-as-code tracking:

### Prerequisites

1. Install Terraform:
   ```bash
   brew install terraform  # macOS
   ```

2. Set GitHub token:
   ```bash
   export GITHUB_TOKEN=your_github_personal_access_token
   ```

   Create token at: https://github.com/settings/tokens
   Required scopes: `repo`, `delete_repo`, `admin:repo_hook`

### Usage

```bash
cd terraform

# Initialize Terraform
terraform init

# Preview changes
terraform plan

# Apply configuration
terraform apply

# View outputs
terraform output
```

### What It Configures

- ✅ Repository creation (public)
- ✅ Description and homepage
- ✅ Topics/tags
- ✅ Enable: Issues, Discussions, Projects
- ✅ Disable: Wiki
- ✅ Security features (secret scanning, vulnerability alerts)
- ✅ Branch protection rules for `main`
- ✅ Issue labels (bug, enhancement, documentation, etc.)

### Customization

Edit `terraform/main.tf`:

```hcl
variable "github_owner" {
  default = "your-username"  # Change this
}

variable "repo_name" {
  default = "dub-rs"
}
```

### Cleanup

To destroy the repository (careful!):

```bash
terraform destroy
```

## Comparison: GitHub CLI vs Terraform

| Feature | GitHub CLI (`./publish-iac.sh`) | Terraform |
|---------|--------------------------------|-----------|
| **Setup** | Simple, one command | Requires token setup |
| **Speed** | Fast, immediate | Slightly slower |
| **Control** | Good for most cases | Fine-grained control |
| **State** | No state management | Tracks state |
| **Best for** | Quick publish & configure | Team management, CI/CD |

## Manual Alternative

If you prefer manual control:

```bash
# Create repo manually on GitHub, then:
git remote add origin https://github.com/novinnoori/dub-rs.git
git push -u origin main
git tag -a v0.1.0 -m "Initial release"
git push origin v0.1.0
```

Then configure settings in GitHub UI.

## Recommended Approach

For publishing dub-rs:

1. **First time**: Use `./publish-iac.sh` - fastest and easiest
2. **Ongoing management**: Use Terraform if you want IaC tracking
3. **Simple projects**: Manual is fine too

Choose what works best for your workflow!
