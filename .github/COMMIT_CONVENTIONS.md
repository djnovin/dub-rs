# Commit Message Guidelines

This project enforces **conventional commits** and **signed commits** for all contributions.

## 📝 Conventional Commits

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Required: Type + Subject

```
feat: add partner analytics endpoint
fix: resolve memory leak in client
docs: update API reference for bounties
```

### Full Example

```
feat(analytics): add click tracking for partner links

Implements real-time click tracking with the following features:
- Geolocation data capture
- Device type detection
- Referrer tracking

Closes #123
```

## 🏷️ Commit Types

| Type | Description | Example |
|------|-------------|---------|
| **feat** | New feature | `feat: add commission create endpoint` |
| **fix** | Bug fix | `fix: resolve timeout in analytics query` |
| **docs** | Documentation only | `docs: add examples for link creation` |
| **style** | Code style (formatting, etc) | `style: apply rustfmt to all modules` |
| **refactor** | Code refactoring | `refactor: simplify error handling` |
| **perf** | Performance improvement | `perf: optimize batch link creation` |
| **test** | Add or update tests | `test: add integration tests for payouts` |
| **build** | Build system changes | `build: update Cargo dependencies` |
| **ci** | CI/CD changes | `ci: add security audit to workflow` |
| **chore** | Maintenance tasks | `chore: update .gitignore` |
| **revert** | Revert previous commit | `revert: revert "feat: add feature X"` |

## 🔒 Signed Commits

All commits to `main` branch **must be signed** with GPG or SSH keys.

### Why Signed Commits?

- ✅ **Verification**: Proves commits are from you
- ✅ **Security**: Prevents impersonation
- ✅ **Trust**: Shows "Verified" badge on GitHub
- ✅ **Compliance**: Industry best practice

### Setup GPG Signing

#### 1. Generate GPG Key (if you don't have one)

```bash
gpg --full-generate-key
# Choose: RSA and RSA, 4096 bits, no expiration
# Enter your name and email (must match GitHub)
```

#### 2. List Your Keys

```bash
gpg --list-secret-keys --keyid-format=long
# Output shows: sec   rsa4096/YOUR_KEY_ID
```

#### 3. Export Public Key

```bash
gpg --armor --export YOUR_KEY_ID
```

#### 4. Add to GitHub

1. Copy the GPG key (including `-----BEGIN PGP PUBLIC KEY BLOCK-----`)
2. Go to: https://github.com/settings/keys
3. Click "New GPG key"
4. Paste and save

#### 5. Configure Git

```bash
# Set your signing key
git config --global user.signingkey YOUR_KEY_ID

# Enable signing by default
git config --global commit.gpgsign true

# Tell Git to use GPG
git config --global gpg.program gpg
```

#### 6. Test It

```bash
git commit -S -m "test: verify GPG signing works"
git log --show-signature -1
```

### Setup SSH Signing (Alternative)

#### 1. Generate SSH Key (if you don't have one)

```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
```

#### 2. Add to GitHub

```bash
# Copy your public key
cat ~/.ssh/id_ed25519.pub

# Add at: https://github.com/settings/keys
# Select "Signing Key" as the type
```

#### 3. Configure Git

```bash
# Set SSH as signing format
git config --global gpg.format ssh

# Set your signing key
git config --global user.signingkey ~/.ssh/id_ed25519.pub

# Enable signing by default
git config --global commit.gpgsign true
```

#### 4. Test It

```bash
git commit -S -m "test: verify SSH signing works"
```

## 🚫 Enforcement

### Branch Protection

The `main` branch has these requirements:

```yaml
require_signed_commits: true      # All commits must be signed
required_linear_history: true     # No merge commits
strict status checks: true        # Branch must be up-to-date
```

### CI Checks

GitHub Actions will check:
- ✅ **PR Title**: Must follow conventional commit format
- ✅ **Commit Messages**: All commits must be conventional
- ✅ **Signatures**: All commits must be signed (enforced by branch protection)

### What Happens If I Don't Sign?

```
❌ Push rejected:
   "Commits must have verified signatures"
```

You'll need to:
1. Setup GPG/SSH signing (see above)
2. Amend your commits to sign them
3. Force push to your branch

```bash
# Sign the last commit
git commit --amend --no-edit -S

# Force push (allowed on your branch, not main)
git push --force-with-lease
```

## ✅ Valid Commit Examples

### Simple Feature

```
feat: add bounty submission endpoint
```

### Bug Fix with Scope

```
fix(client): handle connection timeout gracefully
```

### Breaking Change

```
feat!: redesign authentication flow

BREAKING CHANGE: The auth token format has changed.
Users must regenerate their API tokens.
```

### With Issue Reference

```
fix: resolve race condition in link creation

Fixes intermittent 500 errors when creating multiple
links simultaneously.

Closes #456
```

### Performance Improvement

```
perf(analytics): optimize database query for clicks

Reduces query time from 2s to 200ms by adding index
on timestamp column.
```

## ❌ Invalid Commit Examples

```
❌ Added new feature               # No type
❌ feat:Add feature                # Missing space after colon
❌ Feat: add feature               # Type should be lowercase
❌ feat: Add feature               # Subject should be lowercase
❌ feat: add feature.              # No period at end
❌ feature: add feature            # Invalid type
❌ feat(API): add feature          # Scope should be lowercase
```

## 🎯 Best Practices

### 1. Keep Commits Atomic

Each commit should be a single logical change:

```bash
✅ Good:
git commit -m "feat: add click tracking"
git commit -m "test: add tests for click tracking"
git commit -m "docs: document click tracking API"

❌ Bad:
git commit -m "feat: add click tracking, tests, and docs"
```

### 2. Use Present Tense

```
✅ "add feature"    (present tense)
❌ "added feature"  (past tense)
```

### 3. Be Descriptive

```
✅ "fix: prevent race condition in concurrent requests"
❌ "fix: bug"
```

### 4. Reference Issues

```
feat: add rate limiting

Implements token bucket algorithm for API rate limiting.
Limits per-workspace and per-token.

Closes #123
Refs #124
```

### 5. Breaking Changes

Mark breaking changes clearly:

```
feat!: change API response format

BREAKING CHANGE: All endpoints now return `data` wrapper.
Before: { "id": "123" }
After: { "data": { "id": "123" } }
```

## 🔧 Git Hooks (Optional)

Install commitlint locally for faster feedback:

```bash
# Install commitlint
npm install -g @commitlint/cli @commitlint/config-conventional

# Install husky for git hooks
npm install -g husky

# Setup hook
husky install
npx husky add .husky/commit-msg 'npx --no -- commitlint --edit "$1"'
```

Now invalid commits will be rejected locally before you push.

## 🔍 Checking Your Commits

### Before pushing:

```bash
# Check last commit message
git log -1 --pretty=%B

# Check if last commit is signed
git log --show-signature -1

# Verify all commits in your branch
git log origin/main..HEAD --oneline
```

### Amend last commit to sign:

```bash
git commit --amend --no-edit -S
```

### Sign all commits in your branch:

```bash
# Rebase and sign all commits
git rebase --exec 'git commit --amend --no-edit -n -S' -i origin/main
```

## 🆘 Troubleshooting

### "gpg failed to sign the data"

```bash
# Make sure GPG is installed
gpg --version

# Test GPG
echo "test" | gpg --clearsign

# Restart GPG agent
gpgconf --kill gpg-agent
gpgconf --launch gpg-agent

# Check Git config
git config --global user.signingkey
git config --global commit.gpgsign
```

### "Invalid commit message format"

Check your message against the format:
- Type must be lowercase
- Subject must be lowercase
- No period at the end
- Space after colon

### "Unsigned commits detected"

All commits must be signed before merging:

```bash
# Sign all unsigned commits
git rebase --exec 'git commit --amend --no-edit -n -S' origin/main
git push --force-with-lease
```

## 📚 Resources

- [Conventional Commits Specification](https://www.conventionalcommits.org/)
- [GitHub: Signing Commits](https://docs.github.com/en/authentication/managing-commit-signature-verification)
- [Commitlint](https://commitlint.js.org/)
- [Git Hooks with Husky](https://typicode.github.io/husky/)

## 🎓 Quick Reference Card

```bash
# Commit format
<type>(<scope>): <subject>

# Valid types
feat fix docs style refactor perf test build ci chore revert

# Sign commits
git commit -S -m "feat: your message"

# Check signature
git log --show-signature -1

# Amend and sign
git commit --amend --no-edit -S
```

---

**Remember:** Signed, conventional commits help maintain a professional, secure, and maintainable codebase! 🎉
