# xcargo Release Runbooks

Operational runbooks for xcargo release management.

## Purpose

These runbooks provide step-by-step procedures for release operations, ensuring:
- Consistent release quality
- Minimal errors
- Fast recovery from issues
- Knowledge transfer to new maintainers

## Available Runbooks

### Core Release Process

1. **[01-prepare-release.md](01-prepare-release.md)** - Prepare for new release
   - Duration: 15-30 minutes
   - Prerequisites: Write access, tests passing
   - Output: Version bumped, changelog updated, ready to tag

2. **[02-create-release.md](02-create-release.md)** - Create and publish release
   - Duration: 20-35 minutes (mostly automated)
   - Prerequisites: Runbook 01 complete
   - Output: Release published with all artifacts

3. **[03-verify-release.md](03-verify-release.md)** - Verify release quality
   - Duration: 20-30 minutes
   - Prerequisites: Runbook 02 complete
   - Output: All installation methods verified

### Emergency Procedures

4. **[04-hotfix-release.md](04-hotfix-release.md)** - Emergency patch release
   - Duration: 3-4 hours
   - Prerequisites: Critical bug identified
   - Output: Hotfix released and users notified

## Quick Start

### For Regular Releases

```bash
# Follow in order:
1. Prepare release     → 01-prepare-release.md
2. Create release      → 02-create-release.md
3. Verify release      → 03-verify-release.md
```

**Total time:** ~1-1.5 hours

### For Hotfix Releases

```bash
# Emergency path:
1. Follow → 04-hotfix-release.md
2. Then  → 03-verify-release.md (expedited)
```

**Total time:** ~3-4 hours

## Before You Start

### Required Access

- [ ] Write access to `ibrahimcesar/xcargo` repository
- [ ] Write access to `ibrahimcesar/homebrew-tap` repository
- [ ] GitHub token configured as `HOMEBREW_TAP_TOKEN` secret
- [ ] `gh` CLI installed and authenticated
- [ ] `cargo-dist` CLI installed locally

### Required Tools

```bash
# Verify prerequisites
gh --version          # GitHub CLI
git --version         # Git
cargo --version       # Rust/Cargo
dist --version        # cargo-dist

# Verify authentication
gh auth status        # Should show authenticated

# Verify repository access
gh repo view ibrahimcesar/xcargo
gh repo view ibrahimcesar/homebrew-tap
```

## Release Cadence

**Recommended schedule:**

| Type | Frequency | Version | Examples |
|------|-----------|---------|----------|
| **Major** | Quarterly | X.0.0 | Breaking changes, major features |
| **Minor** | Monthly | 0.X.0 | New features, backward compatible |
| **Patch** | As needed | 0.0.X | Bug fixes only |
| **Hotfix** | Emergency | 0.0.X | Critical bugs, security |

**Current version:** 0.3.0
**Next planned:** 0.4.0 (minor) or 1.0.0 (major)

## Version Numbering

Follow [Semantic Versioning 2.0.0](https://semver.org/):

```
MAJOR.MINOR.PATCH

MAJOR: Incompatible API changes
MINOR: New functionality (backward compatible)
PATCH: Bug fixes (backward compatible)
```

**Examples:**
- `0.3.0` → `0.3.1` - Bug fix
- `0.3.0` → `0.4.0` - New feature
- `0.3.0` → `1.0.0` - First stable release
- `1.2.3` → `2.0.0` - Breaking change

**Pre-release labels:**
- `0.4.0-rc.1` - Release candidate
- `0.4.0-beta.1` - Beta release
- `0.4.0-alpha.1` - Alpha release

## Release Artifacts

Each release produces 16 artifacts:

**Binaries (5):**
- macOS Apple Silicon (`aarch64-apple-darwin.tar.xz`)
- macOS Intel (`x86_64-apple-darwin.tar.xz`)
- Linux glibc (`x86_64-unknown-linux-gnu.tar.xz`)
- Linux musl (`x86_64-unknown-linux-musl.tar.xz`)
- Windows MSVC (`x86_64-pc-windows-msvc.zip`)

**Checksums (5):**
- SHA256 files for each binary (`.sha256`)

**Installers (3):**
- Shell installer (`xcargo-installer.sh`)
- PowerShell installer (`xcargo-installer.ps1`)
- Homebrew formula (`xcargo.rb`)

**Other (3):**
- Combined checksums (`sha256.sum`)
- Source tarball (`source.tar.gz`)
- Source checksum (`source.tar.gz.sha256`)

## Success Metrics

Track these metrics for each release:

**Quality metrics:**
- [ ] All tests passing
- [ ] Zero clippy warnings
- [ ] Test coverage ≥ target (69% current, 80% target for v1.0)
- [ ] No known P0/P1 bugs

**Distribution metrics:**
- [ ] All 16 artifacts present
- [ ] All checksums verify
- [ ] All installers tested
- [ ] Homebrew tap updated

**User metrics (post-release):**
- Downloads per platform (GitHub Insights)
- Homebrew analytics (if enabled)
- Issue reports within 48h
- Social media feedback

## Common Scenarios

### Scenario 1: Regular Monthly Release

```bash
# Month-end release with new features

1. Run full test suite
2. Follow 01-prepare-release.md
3. Follow 02-create-release.md
4. Follow 03-verify-release.md
5. Announce release
```

**Time:** 1-1.5 hours

### Scenario 2: Patch Release

```bash
# Quick bug fix release

1. Fix bug(s) in main
2. Follow 01-prepare-release.md (abbreviated)
3. Follow 02-create-release.md
4. Quick verification of affected area
```

**Time:** 30-45 minutes

### Scenario 3: Hotfix Release

```bash
# Critical bug discovered

1. Assess severity (is it really critical?)
2. Follow 04-hotfix-release.md
3. Notify users immediately
```

**Time:** 3-4 hours

### Scenario 4: Pre-Release (RC, Beta)

```bash
# Testing before major release

1. Follow 01-prepare-release.md
2. Use version like: 1.0.0-rc.1
3. Create as pre-release on GitHub
4. Limited testing with volunteers
5. Iterate until stable
```

**Time:** Multiple iterations

## Rollback Procedures

If a release needs to be rolled back:

**Option 1: Mark as pre-release (recommended)**
```bash
gh release edit "v0.4.0" --prerelease
gh release edit "v0.4.0" --notes "⚠️ This release has issues. Use v0.3.0 instead."
```

**Option 2: Delete and hotfix**
```bash
gh release delete "v0.4.0" --yes
# Follow 04-hotfix-release.md to create v0.4.1
```

**Option 3: Revert tag (last resort)**
```bash
git tag -d v0.4.0
git push origin :refs/tags/v0.4.0
# Breaks links, avoid if possible
```

## Troubleshooting

### Build fails during release

**Symptoms:** GitHub Actions build job fails

**Quick fix:**
```bash
# View logs
gh run view --job=build-PLATFORM --log

# Common causes:
# - Compilation error → Fix code, delete tag, re-release
# - Network timeout → Retry workflow
# - Missing dependency → Update CI environment
```

### Homebrew tap doesn't update

**Symptoms:** Formula not updated after release

**Quick fix:**
```bash
# Check secret exists
gh secret list | grep HOMEBREW_TAP_TOKEN

# Manual update
git clone https://github.com/ibrahimcesar/homebrew-tap
cd homebrew-tap
curl -L "RELEASE_URL/xcargo.rb" -o Formula/xcargo.rb
git commit -am "xcargo 0.4.0"
git push
```

### Installer doesn't work

**Symptoms:** Users report installation failures

**Quick fix:**
1. Test installer yourself
2. Check GitHub release artifacts
3. If broken, create hotfix (04-hotfix-release.md)

## Communication Templates

### GitHub Release Announcement

```markdown
# xcargo v0.4.0 Released! 🎉

We're excited to announce the release of xcargo v0.4.0!

## What's New

[Highlight 2-3 major features from CHANGELOG]

## Installation

```bash
# Shell (Linux/macOS)
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.sh | sh

# Homebrew
brew install ibrahimcesar/tap/xcargo

# PowerShell (Windows)
irm https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.ps1 | iex
```

## Full Changelog

[Paste CHANGELOG.md entry]
```

### Social Media Post

```
🚀 xcargo v0.4.0 is here!

New: [1-2 key features]

Install:
• brew install ibrahimcesar/tap/xcargo
• Or: https://github.com/ibrahimcesar/xcargo/releases

#rustlang #crosscompilation
```

## Getting Help

**For runbook issues:**
- Open issue: https://github.com/ibrahimcesar/xcargo/issues
- Label: `runbook`, `documentation`

**For release issues:**
- Check troubleshooting sections in each runbook
- Contact: maintainers via GitHub

## Contributing

Improvements to these runbooks are welcome!

**To update a runbook:**
1. Make changes
2. Test with a real release (or dry run)
3. Update "Last Updated" date
4. Submit PR with label `runbook`

---

**Runbooks maintained by:** xcargo maintainers
**Last reviewed:** 2025-11-23
**Next review:** Before v1.0.0 release
