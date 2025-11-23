# Runbook: Verify Release

**Purpose:** Test all installation methods and verify release quality
**Expected Duration:** 20-30 minutes
**Prerequisites:** [02-create-release.md](02-create-release.md) completed, release published

---

## Step 1: Test Shell Installer (Linux/macOS)

### Test on macOS

```bash
# Set version
export VERSION="0.4.0"

# Test installer
curl --proto '=https' --tlsv1.2 -LsSf \
  "https://github.com/ibrahimcesar/xcargo/releases/download/v${VERSION}/xcargo-installer.sh" | sh

# Verify installation
xcargo --version
# Expected: xcargo 0.4.0

# Test basic functionality
xcargo doctor
# Expected: System diagnostics run successfully

# Clean up (if testing in temp environment)
rm ~/.cargo/bin/xcargo
```

**✅ Pass criteria:**
- Installer downloads and runs
- Binary installed to ~/.cargo/bin
- Version matches release
- Basic commands work

### Test on Linux (Docker)

```bash
# Test in clean Ubuntu environment
docker run -it --rm ubuntu:22.04 bash

# Inside container:
apt update && apt install -y curl build-essential
curl --proto '=https' --tlsv1.2 -LsSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

curl --proto '=https' --tlsv1.2 -LsSf \
  "https://github.com/ibrahimcesar/xcargo/releases/download/v${VERSION}/xcargo-installer.sh" | sh

xcargo --version
# Expected: xcargo 0.4.0

exit
```

---

## Step 2: Test PowerShell Installer (Windows)

### Test on Windows

```powershell
# Set version
$VERSION = "0.4.0"

# Test installer
irm "https://github.com/ibrahimcesar/xcargo/releases/download/v${VERSION}/xcargo-installer.ps1" | iex

# Verify installation
xcargo --version
# Expected: xcargo 0.4.0

# Test basic functionality
xcargo doctor
```

**✅ Pass criteria:**
- Installer downloads and runs
- Binary installed correctly
- Version matches release
- Basic commands work

---

## Step 3: Test Homebrew Installation

### Update Homebrew and test

```bash
# Update Homebrew
brew update

# Check formula info
brew info ibrahimcesar/tap/xcargo
# Expected:
# ibrahimcesar/tap/xcargo: 0.4.0
# Cross-compilation, zero friction
# https://ibrahimcesar.github.io/xcargo

# Install
brew install ibrahimcesar/tap/xcargo

# Verify
xcargo --version
# Expected: xcargo 0.4.0

# Test functionality
xcargo doctor

# Test upgrade path (if previous version installed)
brew upgrade xcargo
```

**✅ Pass criteria:**
- Homebrew tap updated with new version
- Installation succeeds
- Version matches release
- Upgrade works (if applicable)

**🚨 If Homebrew formula not updated:**
```bash
# Check tap repository
curl https://raw.githubusercontent.com/ibrahimcesar/homebrew-tap/main/Formula/xcargo.rb | grep version

# If version wrong, see troubleshooting in 02-create-release.md
```

---

## Step 4: Test Prebuilt Binary Downloads

### Test macOS Apple Silicon

```bash
export VERSION="0.4.0"

# Download
curl -LO "https://github.com/ibrahimcesar/xcargo/releases/download/v${VERSION}/xcargo-aarch64-apple-darwin.tar.xz"
curl -LO "https://github.com/ibrahimcesar/xcargo/releases/download/v${VERSION}/xcargo-aarch64-apple-darwin.tar.xz.sha256"

# Verify checksum
shasum -a 256 -c xcargo-aarch64-apple-darwin.tar.xz.sha256
# Expected: xcargo-aarch64-apple-darwin.tar.xz: OK

# Extract
tar -xf xcargo-aarch64-apple-darwin.tar.xz

# Test
./xcargo --version
# Expected: xcargo 0.4.0

# Clean up
rm xcargo xcargo-*.tar.xz*
```

### Test Linux (glibc)

```bash
# Download
curl -LO "https://github.com/ibrahimcesar/xcargo/releases/download/v${VERSION}/xcargo-x86_64-unknown-linux-gnu.tar.xz"
curl -LO "https://github.com/ibrahimcesar/xcargo/releases/download/v${VERSION}/xcargo-x86_64-unknown-linux-gnu.tar.xz.sha256"

# Verify checksum
sha256sum -c xcargo-x86_64-unknown-linux-gnu.tar.xz.sha256

# Extract and test
tar -xf xcargo-x86_64-unknown-linux-gnu.tar.xz
./xcargo --version

# Clean up
rm xcargo xcargo-*.tar.xz*
```

### Test Windows

```powershell
# Download
Invoke-WebRequest -Uri "https://github.com/ibrahimcesar/xcargo/releases/download/v${VERSION}/xcargo-x86_64-pc-windows-msvc.zip" -OutFile "xcargo.zip"

# Extract
Expand-Archive -Path xcargo.zip -DestinationPath .

# Test
.\xcargo.exe --version

# Clean up
Remove-Item xcargo.exe, xcargo.zip
```

**✅ Pass criteria for each platform:**
- Download succeeds
- SHA256 verification passes
- Binary extracts successfully
- Binary is executable
- Version matches release

---

## Step 5: Functional Testing

### Test cross-compilation features

```bash
# Create test project
cargo new --bin test-xcargo-release
cd test-xcargo-release

# Test basic build
xcargo build --target x86_64-unknown-linux-gnu
# Expected: Build succeeds

# Test with Zig (if installed)
xcargo build --target x86_64-unknown-linux-musl
# Expected: Build succeeds with Zig

# Test parallel builds
xcargo build --target x86_64-unknown-linux-gnu,aarch64-unknown-linux-gnu
# Expected: Both targets build

# Test list command
xcargo list
# Expected: Shows installed targets

# Clean up
cd ..
rm -rf test-xcargo-release
```

**✅ Pass criteria:**
- All basic commands work
- Cross-compilation succeeds
- Zig integration works (if available)
- Parallel builds work

---

## Step 6: Documentation Verification

### Check release page

Visit: https://github.com/ibrahimcesar/xcargo/releases/tag/v0.4.0

**Verify:**
- [ ] Release title matches version
- [ ] Release notes are complete
- [ ] CHANGELOG.md content included
- [ ] Installation instructions present
- [ ] All 16 artifacts listed
- [ ] Download counts start incrementing

### Check documentation site

Visit: https://ibrahimcesar.github.io/xcargo

**Verify:**
- [ ] Installation page updated with new version
- [ ] Links to v0.4.0 release work
- [ ] Documentation reflects new features

### Check README

Visit: https://github.com/ibrahimcesar/xcargo

**Verify:**
- [ ] Installation instructions work
- [ ] Version badges updated (if applicable)
- [ ] Links to latest release work

---

## Step 7: Community Notifications

### Update project website

```bash
# Update version on website
cd docs/
vim src/pages/index.js

# Update version number
# Commit and deploy
git add .
git commit -m "docs: update version to 0.4.0"
git push
```

### Post announcements (optional)

**GitHub Discussions:**
```markdown
Title: xcargo v0.4.0 Released! 🎉

We're excited to announce the release of xcargo v0.4.0!

**What's New:**
- [List major features from CHANGELOG]

**Installation:**
```bash
# Shell (Linux/macOS)
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.sh | sh

# Homebrew
brew install ibrahimcesar/tap/xcargo
```

Full changelog: https://github.com/ibrahimcesar/xcargo/releases/tag/v0.4.0
```

**Social Media (optional):**
- Twitter/X announcement
- Rust Users Forum
- Reddit r/rust

---

## Step 8: Monitor for Issues

### Watch for user reports

```bash
# Monitor issues
gh issue list --state open --label bug

# Monitor discussions
gh repo view --web
# Navigate to Discussions tab
```

### Check download statistics

```bash
# View release download counts
gh release view "v${VERSION}"

# Check after 24 hours, 1 week
```

### Monitor Homebrew analytics

```bash
# If Homebrew analytics enabled
brew analytics tap-installs ibrahimcesar/tap --30d
```

---

## Verification Checklist

### Installation Methods

- [ ] Shell installer works on macOS
- [ ] Shell installer works on Linux
- [ ] PowerShell installer works on Windows
- [ ] Homebrew installation works
- [ ] Direct binary download works (all platforms)

### Binary Quality

- [ ] All SHA256 checksums verify
- [ ] Binaries are executable
- [ ] Version numbers correct
- [ ] Basic commands work

### Functionality

- [ ] Cross-compilation works
- [ ] Zig integration works
- [ ] Parallel builds work
- [ ] Doctor command works

### Documentation

- [ ] Release notes complete
- [ ] Installation docs updated
- [ ] Website updated
- [ ] All links work

### Distribution

- [ ] GitHub release complete (16 artifacts)
- [ ] Homebrew tap updated
- [ ] No errors reported

---

## Common Issues

### Checksum verification fails

**Cause:** File corrupted during download or upload

**Resolution:**
```bash
# Re-download the file
# If persists, check GitHub release assets

# Manual verification
curl -L "URL" | shasum -a 256
# Compare with .sha256 file
```

### Homebrew version wrong

**Cause:** Tap not updated or cache issue

**Resolution:**
```bash
brew update --force
brew uninstall xcargo
brew install ibrahimcesar/tap/xcargo

# Or manually update
brew edit ibrahimcesar/tap/xcargo
```

### Binary won't run

**macOS quarantine:**
```bash
xattr -d com.apple.quarantine xcargo
chmod +x xcargo
```

**Linux permissions:**
```bash
chmod +x xcargo
```

**Windows SmartScreen:**
- Right-click → Properties → Unblock

---

## Success Criteria

✅ All installation methods tested and working
✅ All platforms verified
✅ Checksums verify correctly
✅ Functional tests pass
✅ Documentation updated
✅ No critical issues reported in first 24 hours

**Status:** Release verification complete ✅

---

## If Issues Found

**Minor issues:** Create issues, plan for next patch release
**Major issues:** Execute [04-hotfix-release.md](04-hotfix-release.md)

---

**Last Updated:** 2025-11-23
**Owner:** xcargo maintainers
