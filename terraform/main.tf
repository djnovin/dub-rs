# Terraform configuration for dub-rs GitHub repository
# Requires: terraform and GitHub provider

terraform {
  required_version = ">= 1.0"
  
  required_providers {
    github = {
      source  = "integrations/github"
      version = "~> 6.0"
    }
  }
}

# Configure the GitHub Provider
# Set GITHUB_TOKEN environment variable or use github_token attribute
provider "github" {
  owner = var.github_owner
}

variable "github_owner" {
  description = "GitHub username or organization"
  type        = string
  default     = "novinnoori"
}

variable "repo_name" {
  description = "Repository name"
  type        = string
  default     = "dub-rs"
}

# Create the repository
resource "github_repository" "dub_rs" {
  name        = var.repo_name
  description = "Complete Rust SDK for the Dub.co API - Type-safe link management, analytics, and conversion tracking"
  homepage_url = "https://dub.co"
  
  visibility = "public"
  
  has_issues      = true
  has_discussions = true
  has_projects    = true
  has_wiki        = false
  has_downloads   = true
  
  allow_merge_commit     = true
  allow_squash_merge     = true
  allow_rebase_merge     = true
  allow_auto_merge       = false
  delete_branch_on_merge = true
  
  topics = [
    "rust",
    "sdk",
    "api-client",
    "dub",
    "link-shortener",
    "analytics",
    "url-shortener",
    "async",
    "tokio",
    "reqwest"
  ]
  
  vulnerability_alerts = true
  
  security_and_analysis {
    secret_scanning {
      status = "enabled"
    }
    secret_scanning_push_protection {
      status = "enabled"
    }
  }
}

# Branch protection for main
resource "github_branch_protection" "main" {
  repository_id = github_repository.dub_rs.node_id
  pattern       = "main"
  
  enforce_admins = false
  
  required_status_checks {
    strict   = true
    contexts = [
      "test (stable)",
      "test (beta)",
      "clippy",
      "fmt"
    ]
  }
  
  required_pull_request_reviews {
    dismiss_stale_reviews           = true
    require_code_owner_reviews      = false
    required_approving_review_count = 0
  }
}

# Issue labels
resource "github_issue_label" "bug" {
  repository  = github_repository.dub_rs.name
  name        = "bug"
  color       = "d73a4a"
  description = "Something isn't working"
}

resource "github_issue_label" "enhancement" {
  repository  = github_repository.dub_rs.name
  name        = "enhancement"
  color       = "a2eeef"
  description = "New feature or request"
}

resource "github_issue_label" "documentation" {
  repository  = github_repository.dub_rs.name
  name        = "documentation"
  color       = "0075ca"
  description = "Improvements or additions to documentation"
}

resource "github_issue_label" "good_first_issue" {
  repository  = github_repository.dub_rs.name
  name        = "good first issue"
  color       = "7057ff"
  description = "Good for newcomers"
}

resource "github_issue_label" "help_wanted" {
  repository  = github_repository.dub_rs.name
  name        = "help wanted"
  color       = "008672"
  description = "Extra attention is needed"
}

# Outputs
output "repository_url" {
  value       = github_repository.dub_rs.html_url
  description = "The URL of the repository"
}

output "repository_ssh_clone_url" {
  value       = github_repository.dub_rs.ssh_clone_url
  description = "The SSH clone URL"
}

output "repository_http_clone_url" {
  value       = github_repository.dub_rs.http_clone_url
  description = "The HTTP clone URL"
}
