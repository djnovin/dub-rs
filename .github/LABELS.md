# GitHub Labels Reference

This document describes the labeling system used in the dub-rs repository.

## Label Categories

### 🏷️ Type Labels

Labels that categorize the **nature** of the issue or PR:

| Label | Color | Description |
|-------|-------|-------------|
| `type: bug` | ![#d73a4a](https://via.placeholder.com/15/d73a4a/000000?text=+) `#d73a4a` | Something isn't working |
| `type: feature` | ![#a2eeef](https://via.placeholder.com/15/a2eeef/000000?text=+) `#a2eeef` | New feature or request |
| `type: enhancement` | ![#84b6eb](https://via.placeholder.com/15/84b6eb/000000?text=+) `#84b6eb` | Improvement to existing functionality |
| `type: docs` | ![#0075ca](https://via.placeholder.com/15/0075ca/000000?text=+) `#0075ca` | Documentation improvements or additions |
| `type: refactor` | ![#fbca04](https://via.placeholder.com/15/fbca04/000000?text=+) `#fbca04` | Code refactoring without functional changes |
| `type: test` | ![#c5def5](https://via.placeholder.com/15/c5def5/000000?text=+) `#c5def5` | Testing improvements or additions |
| `type: chore` | ![#fef2c0](https://via.placeholder.com/15/fef2c0/000000?text=+) `#fef2c0` | Maintenance tasks, dependencies, tooling |

### ⚡ Priority Labels

Labels that indicate the **urgency** of the issue:

| Label | Color | Description |
|-------|-------|-------------|
| `priority: critical` | ![#b60205](https://via.placeholder.com/15/b60205/000000?text=+) `#b60205` | Critical priority - needs immediate attention |
| `priority: high` | ![#d93f0b](https://via.placeholder.com/15/d93f0b/000000?text=+) `#d93f0b` | High priority |
| `priority: medium` | ![#fbca04](https://via.placeholder.com/15/fbca04/000000?text=+) `#fbca04` | Medium priority (default) |
| `priority: low` | ![#0e8a16](https://via.placeholder.com/15/0e8a16/000000?text=+) `#0e8a16` | Low priority |

### 📊 Status Labels

Labels that track the **current state** of the issue:

| Label | Color | Description |
|-------|-------|-------------|
| `status: triage` | ![#ededed](https://via.placeholder.com/15/ededed/000000?text=+) `#ededed` | Needs investigation and prioritization |
| `status: blocked` | ![#d93f0b](https://via.placeholder.com/15/d93f0b/000000?text=+) `#d93f0b` | Blocked by other issues or external factors |
| `status: in progress` | ![#0052cc](https://via.placeholder.com/15/0052cc/000000?text=+) `#0052cc` | Work in progress |
| `status: needs review` | ![#fbca04](https://via.placeholder.com/15/fbca04/000000?text=+) `#fbca04` | Awaiting code review |
| `status: needs feedback` | ![#cc317c](https://via.placeholder.com/15/cc317c/000000?text=+) `#cc317c` | Needs feedback from maintainers or community |
| `status: wontfix` | ![#ffffff](https://via.placeholder.com/15/ffffff/000000?text=+) `#ffffff` | This will not be worked on |

### 🎯 Special Labels

Miscellaneous labels for specific purposes:

| Label | Color | Description |
|-------|-------|-------------|
| `good first issue` | ![#7057ff](https://via.placeholder.com/15/7057ff/000000?text=+) `#7057ff` | Good for newcomers |
| `help wanted` | ![#008672](https://via.placeholder.com/15/008672/000000?text=+) `#008672` | Extra attention is needed |
| `breaking change` | ![#e99695](https://via.placeholder.com/15/e99695/000000?text=+) `#e99695` | Introduces breaking changes to the API |
| `security` | ![#ee0701](https://via.placeholder.com/15/ee0701/000000?text=+) `#ee0701` | Security-related issue |
| `dependencies` | ![#0366d6](https://via.placeholder.com/15/0366d6/000000?text=+) `#0366d6` | Dependency updates |
| `duplicate` | ![#cfd3d7](https://via.placeholder.com/15/cfd3d7/000000?text=+) `#cfd3d7` | This issue or pull request already exists |
| `invalid` | ![#e4e669](https://via.placeholder.com/15/e4e669/000000?text=+) `#e4e669` | This doesn't seem right |
| `question` | ![#d876e3](https://via.placeholder.com/15/d876e3/000000?text=+) `#d876e3` | Further information is requested |

## Usage Guidelines

### Best Practices

1. **Every issue should have at least:**
   - One `type:` label
   - One `priority:` label (defaults to medium if not specified)
   - One `status:` label (starts with `status: triage`)

2. **Examples of well-labeled issues:**
   ```
   type: bug + priority: high + status: in progress
   type: feature + priority: medium + status: needs review + breaking change
   type: docs + priority: low + good first issue
   ```

3. **Label workflow for issues:**
   - **Created** → `status: triage`
   - **Prioritized** → Add `priority:` label, keep or remove triage
   - **Working** → `status: in progress`
   - **PR Ready** → `status: needs review`
   - **Blocked** → `status: blocked`
   - **Closed** → Remove status labels or mark `status: wontfix`

4. **Special case workflows:**
   - Security issues: `type: bug` + `security` + `priority: critical`
   - Breaking changes: Must have `breaking change` label for changelog generation
   - Dependencies: Automated PRs get `dependencies` + `type: chore`

### For Contributors

- If you're new, look for issues with `good first issue`
- If you can help, look for `help wanted` issues
- Always add appropriate labels when creating issues
- Update `status:` labels as work progresses

### For Maintainers

- Triage new issues within 48 hours
- Security issues get immediate `priority: critical` treatment
- Breaking changes require discussion before approval
- Use `status: needs feedback` when community input is needed
