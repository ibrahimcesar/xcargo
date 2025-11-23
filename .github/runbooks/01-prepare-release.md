# Runbook: Prepare Release

**Purpose:** Prepare the codebase for a new release
**Expected Duration:** 15-30 minutes
**Prerequisites:** Write access to repository, all tests passing

---

## Step 1: Verify System State

### Check Current Status

```bash
# Navigate to repository
cd /path/to/xcargo

# Ensure on main branch with latest code
git checkout main
git pull origin main

# Verify clean working directory
git status
# Expected: "nothing to commit, working tree clean"
```

### Run Quality Checks

```bash
# Run all tests
cargo test
# Expected: All tests pass

# Run linter
cargo clippy -- -D warnings
# Expected: No warnings

# Check formatting
cargo fmt --check
# Expected: No formatting issues

# Check test coverage (optional)
cargo tarpaulin --out Stdout
# Expected: ≥69% (target: 80% for v1.0)
```

**✋ STOP if any checks fail. Fix issues before proceeding.**

---

## Step 2: Update Version Numbers

### Determine Version Number

Follow semantic versioning:
- **Patch** (0.3.X): Bug fixes, no new features
- **Minor** (0.X.0): New features, backward compatible
- **Major** (X.0.0): Breaking changes

**Current version:** 0.3.0
**Next version:** ___________

### Update Cargo.toml

```bash
# Edit version
vim Cargo.toml

# Change line 3:
# FROM: version = "0.3.0"
# TO:   version = "0.4.0"  (or your target version)
```

### Update Documentation

```bash
# Update expected version in docs
sed -i '' 's/xcargo 0.3.0/xcargo 0.4.0/g' docs/installation.md
sed -i '' 's/v0.3.0/v0.4.0/g' README.md

# Verify changes
git diff docs/installation.md README.md
```

### Update CHANGELOG.md

```bash
# Edit changelog
vim CHANGELOG.md

# Add new section at top:
```

```markdown
## [0.4.0] - 2025-MM-DD

### Added
- Feature 1 description
- Feature 2 description

### Changed
- Change 1 description

### Fixed
- Bug fix 1 description

### Removed
- Deprecation 1 description
```

**Example changelog entry:**
```markdown
## [0.4.0] - 2025-11-24

### Added
- Professional distribution infrastructure with cargo-dist
- Homebrew tap for easy installation
- Comprehensive release documentation

### Changed
- Improved installation documentation with multiple methods
- Updated contributing guidelines

### Fixed
- Removed outdated TARGETS.md references
```

---

## Step 3: Verify cargo-dist Configuration

### Check dist plan

```bash
# Run cargo-dist planner
~/.cargo/bin/dist plan

# Verify output shows:
# - Version matches your target version
# - All 5 platforms listed (Linux GNU, Linux musl, macOS x64, macOS ARM64, Windows MSVC)
# - All 3 installers (shell, powershell, homebrew)
```

**Expected output:**
```
analyzing workspace:
  dist.profile = "dist"

analyzing package xcargo v0.4.0
  targets:
    - aarch64-apple-darwin
    - x86_64-apple-darwin
    - x86_64-unknown-linux-gnu
    - x86_64-unknown-linux-musl
    - x86_64-pc-windows-msvc

  installers:
    - shell
    - powershell
    - homebrew (ibrahimcesar/homebrew-tap)
```

**✋ STOP if configuration is incorrect.**

---

## Step 4: Commit Version Bump

### Create commit

```bash
# Stage changes
git add Cargo.toml docs/installation.md README.md CHANGELOG.md

# Verify staged changes
git diff --staged

# Create commit
git commit -m "chore: bump version to 0.4.0

Prepare for 0.4.0 release.

- Update version in Cargo.toml
- Update documentation with new version
- Add CHANGELOG.md entry
"

# Push to main
git push origin main
```

**✅ Checkpoint:** Version bump committed and pushed

---

## Step 5: Final Pre-Release Checks

### Verify CI passes

```bash
# Watch GitHub Actions
gh run list --limit 5

# Wait for latest run to complete
gh run watch

# Expected: All checks pass ✅
```

### Test build locally

```bash
# Build release binary
cargo build --release

# Verify binary works
./target/release/xcargo --version
# Expected: xcargo 0.4.0

./target/release/xcargo doctor
# Expected: System diagnostics run successfully
```

### Review release notes

```bash
# View what will be in release notes
git log $(git describe --tags --abbrev=0)..HEAD --oneline

# Generate summary
git log $(git describe --tags --abbrev=0)..HEAD --pretty=format:"- %s"
```

---

## Step 6: Create Pre-Release Checklist

Use this checklist before creating the tag:

- [ ] All tests passing
- [ ] Version updated in Cargo.toml
- [ ] Version updated in documentation
- [ ] CHANGELOG.md updated
- [ ] cargo-dist plan verified
- [ ] Version bump committed and pushed
- [ ] CI passing on main branch
- [ ] Release binary tested locally
- [ ] Homebrew tap repository exists and is accessible
- [ ] `HOMEBREW_TAP_TOKEN` secret is configured

---

## Troubleshooting

### Tests fail

```bash
# Run specific test
cargo test test_name -- --nocapture

# Check which tests failed
cargo test 2>&1 | grep FAILED

# Fix and re-run
```

### cargo-dist not installed

```bash
cargo install cargo-dist

# Verify
dist --version
```

### Cannot push to main

```bash
# Check branch protection rules
gh repo view --web

# Navigate to Settings → Branches → main
# Verify you have permission to push
```

### Version number typo

```bash
# If not yet pushed
git reset HEAD~1
# Make corrections and commit again

# If already pushed (use with caution)
git revert HEAD
# Make corrections in new commit
```

---

## Success Criteria

✅ All quality checks pass
✅ Version number updated consistently
✅ CHANGELOG.md updated
✅ cargo-dist plan verified
✅ Changes committed and pushed
✅ CI passing on main

**Next step:** [02-create-release.md](02-create-release.md)

---

**Last Updated:** 2025-11-23
**Owner:** xcargo maintainers
