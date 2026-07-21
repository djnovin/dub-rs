# Branch Protection Strategy

This document explains the branch protection rules configured for `main` branch in dub-rs.

## 🛡️ Protection Rules

### Strict Status Checks (`strict = true`)

**What it does:**
- PRs **must** be up-to-date with `main` before merging
- Forces contributors to rebase/update when main has moved forward
- Prevents "works on my branch" but breaks main issues

**Example workflow:**
```
1. Contributor opens PR based on commit A
2. Another PR merges (main is now at commit B)
3. Original PR cannot merge until it's updated with commit B
4. GitHub shows "Update branch" button
5. After update, CI runs again, then merge is allowed
```

**Why this matters:**
- Ensures every PR is tested against the **current** main branch
- Prevents race conditions where two PRs pass tests independently but conflict when merged
- Guarantees main branch always builds and passes tests

### Required Status Checks

All PRs must pass these CI checks before merging:
- ✅ `test (stable)` - Tests on Rust stable
- ✅ `test (beta)` - Tests on Rust beta  
- ✅ `clippy` - Rust linter
- ✅ `fmt` - Code formatting

**Enforcement:**
- Cannot merge with failing checks
- Cannot bypass checks (even as admin, unless `enforce_admins = false` is used)
- Re-runs required if branch is updated

### Linear History Enforcement

**What it does:**
- `required_linear_history = true` prevents merge commits
- Only allows fast-forward or squash merges
- Combined with `allow_squash_merge = true` only, ensures clean history

**Result:**
```
# Clean linear history
* feat: add feature C
* fix: resolve bug B
* feat: add feature A

# Instead of messy merge commits
* Merge branch 'feature-c'
|\
| * WIP commits
| * More WIP
|/
* Merge branch 'feature-b'
```

### Conversation Resolution

**What it does:**
- `require_conversation_resolution = true`
- All PR review comments must be resolved
- Prevents accidental merges with open discussions

**Workflow:**
1. Reviewer leaves comment: "Can we improve this?"
2. Contributor responds and updates code
3. Reviewer clicks "Resolve conversation"
4. Once all resolved, PR can merge

### Branch Deletion & Force Push Protection

```terraform
allows_force_pushes = false  # No force pushes to main
allows_deletions    = false  # Cannot delete main branch
```

**Protects against:**
- Accidental `git push --force` to main
- Accidentally deleting the main branch
- History rewriting disasters

### Stale Review Dismissal

```terraform
dismiss_stale_reviews = true
```

**What it does:**
- If PR gets approved, then new commits are pushed
- Old approval is dismissed automatically
- Ensures reviewers see the final code

**Example:**
```
1. PR approved ✅
2. Contributor pushes "just one more fix"
3. Approval dismissed ⚠️
4. Re-review needed (or not, if required_approving_review_count = 0)
```

## 🎯 Maintainer Experience

### For You (Repository Owner)

Since `enforce_admins = false`:
- You **can** bypass all rules if needed (emergency fixes)
- You can merge without approval (`required_approving_review_count = 0`)
- You still get the benefit of CI checks catching issues

### For Contributors

Since `strict = true`:
- They must keep PR up-to-date with main
- All CI must pass
- All conversations must be resolved
- Results in high-quality, tested contributions

## 📊 Workflow Diagram

```
PR Opened
    ↓
CI Checks Running (test, clippy, fmt)
    ↓
All Checks Pass? → NO → Fix code, push again
    ↓ YES
Branch up-to-date? → NO → "Update branch" required
    ↓ YES
Conversations resolved? → NO → Resolve comments
    ↓ YES
Merge Button Enabled ✅
    ↓
Squash & Merge
    ↓
Branch Auto-Deleted
```

## 🔧 What "Strict" Really Means

**Without `strict = true`:**
```
main:     A---B---C
            \
PR branch:   D---E  ← Can merge even though B & C happened
```
**Problem:** E was never tested with B & C changes. Main could break!

**With `strict = true`:**
```
main:     A---B---C
                  \
PR branch:         D'---E'  ← Must update to include B & C first
```
**Benefit:** E' is tested with B & C. Main guaranteed to work!

## 🚀 Benefits

### For Code Quality
- ✅ Every merge is tested against current main
- ✅ No "surprise" breakages after merge
- ✅ Clean, linear history for debugging
- ✅ Easy to bisect and find when bugs were introduced

### For Maintainer Sanity
- ✅ Less broken main branches
- ✅ No "it worked on my machine" issues
- ✅ Clear commit history
- ✅ Easy rollbacks (just revert one commit)

### For Contributors
- ✅ Clear requirements (CI must pass)
- ✅ Obvious when updates are needed
- ✅ Confidence their code won't break main

## 🎬 Example Scenarios

### Scenario 1: Multiple PRs
```
Time 0: Main is at commit A
        PR #1 branches off A
        PR #2 branches off A

Time 1: PR #1 merges → Main is now at B

Time 2: PR #2 tries to merge
        ❌ BLOCKED: "Branch is out-of-date"
        → Must update PR #2 with commit B
        → CI runs again with B included
        → ✅ NOW can merge
```

### Scenario 2: Emergency Fix
```
You notice a critical bug in production

As maintainer:
1. Create branch from main
2. Fix bug
3. Push (CI runs)
4. CI fails on stable but you need to deploy
5. Use admin bypass (enforce_admins = false)
6. Merge anyway
```

## 📝 Configuration Summary

```terraform
Branch Protection {
  strict status checks:     ✅ Branches must be up-to-date
  required checks:          ✅ test (stable, beta), clippy, fmt
  linear history:           ✅ No merge commits
  force push:               ❌ Disabled
  delete branch:            ❌ Disabled
  conversation resolution:  ✅ Required
  stale review dismissal:   ✅ Enabled
  admin bypass:             ✅ Allowed (for maintainer)
}

Merge Strategy {
  squash merge:             ✅ Only allowed method
  merge commit:             ❌ Disabled
  rebase merge:             ❌ Disabled
  auto-delete branches:     ✅ Enabled
}
```

## 🎓 Best Practices

### For Contributors
1. Always branch off latest `main`
2. Keep PRs small and focused
3. Respond to review comments
4. Click "Update branch" when GitHub prompts
5. Ensure all CI checks pass

### For You (Maintainer)
1. Review PRs promptly (stale PRs need updates)
2. Use squash merge to keep history clean
3. Write good squash commit messages
4. Use admin bypass sparingly (only emergencies)
5. Let CI catch issues before merging

## 🔍 Monitoring

Check these regularly:
- Open PRs with "out of date" status
- Failed CI checks (help contributors fix)
- Unresolved conversations
- Main branch build status

---

**Result:** A professional, maintainable open-source project with clear contribution standards! 🎉
