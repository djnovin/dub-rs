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

# Issue labels - organized by category

# Type labels
resource "github_issue_label" "type_bug" {
  repository  = github_repository.dub_rs.name
  name        = "type: bug"
  color       = "d73a4a"
  description = "Something isn't working"
}

resource "github_issue_label" "type_feature" {
  repository  = github_repository.dub_rs.name
  name        = "type: feature"
  color       = "a2eeef"
  description = "New feature or request"
}

resource "github_issue_label" "type_enhancement" {
  repository  = github_repository.dub_rs.name
  name        = "type: enhancement"
  color       = "84b6eb"
  description = "Improvement to existing functionality"
}

resource "github_issue_label" "type_docs" {
  repository  = github_repository.dub_rs.name
  name        = "type: docs"
  color       = "0075ca"
  description = "Documentation improvements or additions"
}

resource "github_issue_label" "type_refactor" {
  repository  = github_repository.dub_rs.name
  name        = "type: refactor"
  color       = "fbca04"
  description = "Code refactoring without functional changes"
}

resource "github_issue_label" "type_test" {
  repository  = github_repository.dub_rs.name
  name        = "type: test"
  color       = "c5def5"
  description = "Testing improvements or additions"
}

resource "github_issue_label" "type_chore" {
  repository  = github_repository.dub_rs.name
  name        = "type: chore"
  color       = "fef2c0"
  description = "Maintenance tasks, dependencies, tooling"
}

# Priority labels
resource "github_issue_label" "priority_critical" {
  repository  = github_repository.dub_rs.name
  name        = "priority: critical"
  color       = "b60205"
  description = "Critical priority - needs immediate attention"
}

resource "github_issue_label" "priority_high" {
  repository  = github_repository.dub_rs.name
  name        = "priority: high"
  color       = "d93f0b"
  description = "High priority"
}

resource "github_issue_label" "priority_medium" {
  repository  = github_repository.dub_rs.name
  name        = "priority: medium"
  color       = "fbca04"
  description = "Medium priority"
}

resource "github_issue_label" "priority_low" {
  repository  = github_repository.dub_rs.name
  name        = "priority: low"
  color       = "0e8a16"
  description = "Low priority"
}

# Status labels
resource "github_issue_label" "status_triage" {
  repository  = github_repository.dub_rs.name
  name        = "status: triage"
  color       = "ededed"
  description = "Needs investigation and prioritization"
}

resource "github_issue_label" "status_blocked" {
  repository  = github_repository.dub_rs.name
  name        = "status: blocked"
  color       = "d93f0b"
  description = "Blocked by other issues or external factors"
}

resource "github_issue_label" "status_in_progress" {
  repository  = github_repository.dub_rs.name
  name        = "status: in progress"
  color       = "0052cc"
  description = "Work in progress"
}

resource "github_issue_label" "status_needs_review" {
  repository  = github_repository.dub_rs.name
  name        = "status: needs review"
  color       = "fbca04"
  description = "Awaiting code review"
}

resource "github_issue_label" "status_needs_feedback" {
  repository  = github_repository.dub_rs.name
  name        = "status: needs feedback"
  color       = "cc317c"
  description = "Needs feedback from maintainers or community"
}

resource "github_issue_label" "status_wontfix" {
  repository  = github_repository.dub_rs.name
  name        = "status: wontfix"
  color       = "ffffff"
  description = "This will not be worked on"
}

# Special labels
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

resource "github_issue_label" "breaking_change" {
  repository  = github_repository.dub_rs.name
  name        = "breaking change"
  color       = "e99695"
  description = "Introduces breaking changes to the API"
}

resource "github_issue_label" "security" {
  repository  = github_repository.dub_rs.name
  name        = "security"
  color       = "ee0701"
  description = "Security-related issue"
}

resource "github_issue_label" "dependencies" {
  repository  = github_repository.dub_rs.name
  name        = "dependencies"
  color       = "0366d6"
  description = "Dependency updates"
}

resource "github_issue_label" "duplicate" {
  repository  = github_repository.dub_rs.name
  name        = "duplicate"
  color       = "cfd3d7"
  description = "This issue or pull request already exists"
}

resource "github_issue_label" "invalid" {
  repository  = github_repository.dub_rs.name
  name        = "invalid"
  color       = "e4e669"
  description = "This doesn't seem right"
}

resource "github_issue_label" "question" {
  repository  = github_repository.dub_rs.name
  name        = "question"
  color       = "d876e3"
  description = "Further information is requested"
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
