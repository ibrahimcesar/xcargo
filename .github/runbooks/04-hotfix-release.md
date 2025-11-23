# Runbook: Hotfix Release

**Purpose:** Quickly release a patch to fix critical bugs
**Expected Duration:** 30-60 minutes
**Prerequisites:** Critical bug identified, fix ready

---

## When to Use This Runbook

Use hotfix procedure when:
- **Critical bug** in latest release
- **Security vulnerability** discovered
- **Installer broken** for major platform
- **Data loss** or **crash** affecting users

Do NOT use for:
- Minor bugs (wait for next regular release)
- Feature requests
- Documentation fixes
- Cosmetic issues

---

## Step 1: Assess Severity

### Determine urgency

**P0 - Critical (immediate hotfix):**
- Security vulnerability
- Data loss or corruption
- Complete failure on major platform
- Installer doesn't work

**P1 - High (hotfix within 24-48h):**
- Feature broken for significant use case
- Crash in common scenario
- Major performance regression

**P2 - Medium (include in next release):**
- Minor functionality broken
- Edge case failures
- Non-critical warnings

**Decision:** If P0 or P1, proceed with hotfix. If P2, defer to regular release cycle.

---

## Step 2: Prepare Hotfix Branch

### Create hotfix branch

```bash
# Get latest release tag
export CURRENT_VERSION=$(git describe --tags --abbrev=0)
# Example: v0.4.0

# Calculate patch version
export HOTFIX_VERSION="v0.4.1"

# Create hotfix branch from release tag
git checkout -b "hotfix/${HOTFIX_VERSION}" "${CURRENT_VERSION}"

# Verify you're on the hotfix branch
git branch --show-current
# Expected: hotfix/v0.4.1
```

### Cherry-pick fixes

```bash
# If fix already exists in main
git cherry-pick <commit-hash>

# Or apply fix directly
vim src/path/to/bug.rs

# Commit the fix
git add .
git commit -m "fix: critical bug causing [problem]

[Detailed description of bug and fix]

Fixes #123
"
```

### Update version for hotfix

```bash
# Edit Cargo.toml
vim Cargo.toml
# Change: version = "0.4.0"
# To:     version = "0.4.1"

# Update CHANGELOG.md
vim CHANGELOG.md
# Add hotfix entry at top:
```

```markdown
## [0.4.1] - 2025-11-24 (Hotfix)

### Fixed
- Critical bug causing installer to fail on Windows (#123)
- Security vulnerability in dependency parsing (#124)

**Note:** This is a hotfix release for v0.4.0. Upgrade immediately.
```

### Commit version bump

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.4.1 (hotfix)"
```

---

## Step 3: Test Hotfix

### Run full test suite

```bash
# Run tests
cargo test
# Expected: All tests pass

# Run clippy
cargo clippy -- -D warnings
# Expected: No warnings

# Build release binary
cargo build --release

# Test the specific bug fix
./target/release/xcargo [command that was broken]
# Expected: Now works correctly
```

### Test affected installation method

If installer was broken:
```bash
# Test installer locally
# See 03-verify-release.md for full testing

# For Windows installer issue example:
# Test PowerShell installer in Windows VM
```

**⚠️ Critical:** Verify the specific bug is fixed before proceeding

---

## Step 4: Merge to Main

### Push hotfix branch

```bash
git push origin "hotfix/${HOTFIX_VERSION}"
```

### Create Pull Request

```bash
# Create PR
gh pr create \
  --title "Hotfix v0.4.1: Fix critical installer bug" \
  --body "**Hotfix Release**

This PR contains urgent fixes for v0.4.0:

- Fix Windows installer failure (#123)
- Fix security vulnerability (#124)

**Testing:**
- [x] Tested on affected platforms
- [x] All tests passing
- [x] Specific bug verified fixed

**Urgency:** High - Users cannot install on Windows

cc @maintainers" \
  --label "hotfix" \
  --label "priority:high"

# Or create manually at:
# https://github.com/ibrahimcesar/xcargo/compare/main...hotfix/v0.4.1
```

### Fast-track review

```bash
# Request immediate review
gh pr review --approve

# Merge (if you have permissions)
gh pr merge --squash

# Or wait for maintainer approval
```

---

## Step 5: Release Hotfix

### Tag and push

```bash
# Switch to main (after merge)
git checkout main
git pull origin main

# Create hotfix tag
git tag -a "${HOTFIX_VERSION}" -m "Hotfix v0.4.1

Critical fixes:
- Fix Windows installer failure (#123)
- Fix security vulnerability (#124)

This is an urgent patch for v0.4.0. Users should upgrade immediately.
"

# Push tag to trigger release
git push origin "${HOTFIX_VERSION}"
```

### Monitor release workflow

```bash
# Watch release
gh run watch

# Or view in browser
gh run list --workflow=release.yml --limit 1
```

**Follow monitoring steps from [02-create-release.md](02-create-release.md) Step 2**

---

## Step 6: Expedited Verification

### Quick smoke tests

Focus on affected areas:

```bash
# Test the specific bug fix
# Example for installer issue:
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/ibrahimcesar/xcargo/releases/download/v0.4.1/xcargo-installer.sh | sh

xcargo --version
# Expected: xcargo 0.4.1
```

**Full verification:** Follow [03-verify-release.md](03-verify-release.md)

---

## Step 7: Emergency Communication

### Update GitHub release notes

```bash
# Edit release to add urgent notice
gh release edit "v0.4.1" --notes "⚠️ **HOTFIX RELEASE** - Upgrade Recommended

This release fixes critical issues in v0.4.0:

- **Windows installer failure** - Fixed installer script (#123)
- **Security vulnerability** - Updated dependency with security patch (#124)

**If you installed v0.4.0, please upgrade immediately:**

\`\`\`bash
# Homebrew
brew upgrade xcargo

# Shell installer
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.sh | sh
\`\`\`

## Changelog

$(cat CHANGELOG.md | sed -n '/## \[0.4.1\]/,/## \[0.4.0\]/p' | sed '$d')
"
```

### Mark previous release

```bash
# Add warning to v0.4.0 release
gh release edit "v0.4.0" --notes "⚠️ **SUPERSEDED BY v0.4.1**

This release has known critical issues. Please use v0.4.1 instead.

Issues fixed in v0.4.1:
- Windows installer failure (#123)
- Security vulnerability (#124)

---

[Original release notes below]

$(gh release view v0.4.0 --json body -q .body)
"
```

### Notify users

**GitHub Discussions:**
```markdown
Title: 🚨 URGENT: Please upgrade to v0.4.1

A critical issue was discovered in v0.4.0 that affects [description].

**Who is affected:**
- Users who installed v0.4.0 on Windows
- [Other affected scenarios]

**Action required:**
Upgrade to v0.4.1 immediately:

```bash
brew upgrade xcargo
# Or re-run installer
```

**Details:**
[Link to issue and release notes]
```

**Email (if mailing list exists):**
- Send notification to users
- Include upgrade instructions
- Link to GitHub release

**Social Media (if critical security issue):**
- Post on Twitter/X
- Post on Rust forums
- Consider CVE if security-related

---

## Step 8: Post-Hotfix Review

### Analyze root cause

**Within 24 hours:**
- [ ] Document what caused the bug
- [ ] Identify why it wasn't caught in testing
- [ ] Create issue for process improvement

**Questions to answer:**
1. How did this bug get through testing?
2. What tests should we add to prevent recurrence?
3. Should we update CI/CD?
4. Do we need better pre-release testing?

### Update processes

```bash
# Create issues for improvements
gh issue create --title "Add test for Windows installer verification" \
  --body "Hotfix v0.4.1 revealed we don't test Windows installer in CI.

**Action items:**
- [ ] Add Windows VM to test matrix
- [ ] Test PowerShell installer in CI
- [ ] Add installer integration tests

Related: Hotfix v0.4.1"

# Label for tracking
gh issue edit --add-label "process-improvement"
```

---

## Troubleshooting

### Hotfix build fails

```bash
# If cherry-pick has conflicts
git cherry-pick --abort
# Apply fix manually

# If tests fail
# Fix the issue
git add .
git commit --amend

# Push force to hotfix branch
git push -f origin "hotfix/${HOTFIX_VERSION}"
```

### Can't merge to main

```bash
# If branch protection prevents merge
# Request admin override

# Or merge manually
git checkout main
git merge "hotfix/${HOTFIX_VERSION}"
git push origin main

# Then tag
git tag -a "${HOTFIX_VERSION}" -m "..."
git push origin "${HOTFIX_VERSION}"
```

### Homebrew tap not updating

```bash
# Manual update
git clone https://github.com/ibrahimcesar/homebrew-tap
cd homebrew-tap

curl -L "https://github.com/ibrahimcesar/xcargo/releases/download/${HOTFIX_VERSION}/xcargo.rb" \
  -o Formula/xcargo.rb

git add Formula/xcargo.rb
git commit -m "xcargo ${HOTFIX_VERSION} (hotfix)"
git push
```

---

## Checklist

### Before Release

- [ ] Bug confirmed as critical (P0 or P1)
- [ ] Fix implemented and tested
- [ ] Hotfix branch created from release tag
- [ ] Version bumped (patch level)
- [ ] CHANGELOG.md updated with hotfix notice
- [ ] All tests passing
- [ ] Specific bug verified fixed

### During Release

- [ ] PR created and approved
- [ ] Merged to main
- [ ] Tag created and pushed
- [ ] Release workflow completes successfully
- [ ] All artifacts present

### After Release

- [ ] Quick verification completed
- [ ] Previous release marked with warning
- [ ] Users notified (GitHub, email, social)
- [ ] Post-mortem issue created
- [ ] Process improvements identified

---

## Success Criteria

✅ Critical bug fixed
✅ Hotfix released within target timeframe
✅ Users notified
✅ Previous release marked appropriately
✅ Root cause identified
✅ Process improvements planned

---

## Example Timeline

**Hour 0:** Critical bug reported
**Hour 0-1:** Assess severity, create hotfix branch
**Hour 1-2:** Implement and test fix
**Hour 2:** Create PR, fast-track review
**Hour 2-3:** Release hotfix
**Hour 3-4:** Verify and notify users
**Day 1:** Post-mortem and process improvements

**Total: 3-4 hours for critical P0 hotfix**

---

**Last Updated:** 2025-11-23
**Owner:** xcargo maintainers
