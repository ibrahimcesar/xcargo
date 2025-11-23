# Runbook: Create Release

**Purpose:** Tag and trigger automated release process
**Expected Duration:** 5-10 minutes (automation) + 15-20 minutes (monitoring)
**Prerequisites:** [01-prepare-release.md](01-prepare-release.md) completed

---

## Step 1: Create Git Tag

### Verify you're ready

```bash
# Confirm version to release
grep '^version = ' Cargo.toml
# Expected: version = "0.4.0" (or your target version)

# Confirm on main with latest code
git branch --show-current
# Expected: main

git pull origin main
# Expected: Already up to date

git status
# Expected: nothing to commit, working tree clean
```

### Create annotated tag

```bash
# Set version variable for convenience
export VERSION="0.4.0"

# Create annotated tag
git tag -a "v${VERSION}" -m "Release v${VERSION}

This release includes:
- [Brief description of major changes]
- [Additional notable changes]

See CHANGELOG.md for full details.
"

# Verify tag
git tag -l "v${VERSION}" -n9
```

**Example tag message:**
```
Release v0.4.0

This release includes:
- Professional distribution infrastructure with cargo-dist
- Homebrew tap for macOS/Linux users
- Shell and PowerShell installers
- Comprehensive release documentation

See CHANGELOG.md for full details.
```

### Push tag to trigger release

```bash
# Push tag
git push origin "v${VERSION}"

# Expected output:
# Enumerating objects: 1, done.
# To https://github.com/ibrahimcesar/xcargo
#  * [new tag]         v0.4.0 -> v0.4.0
```

**⚠️ IMPORTANT:** Pushing the tag triggers the release workflow immediately!

---

## Step 2: Monitor Release Workflow

### Open GitHub Actions

```bash
# Open in browser
gh run list --workflow=release.yml --limit 1

# Or view in terminal
gh run watch
```

**Or manually:** https://github.com/ibrahimcesar/xcargo/actions/workflows/release.yml

### Expected workflow stages

The release workflow runs in this order:

1. **plan** (1-2 min)
   - Generates release plan
   - Validates configuration
   - Status: ✅ Should complete quickly

2. **build-* jobs** (5-10 min each, run in parallel)
   - `build-aarch64-apple-darwin` - macOS ARM64
   - `build-x86_64-apple-darwin` - macOS Intel
   - `build-x86_64-unknown-linux-gnu` - Linux glibc
   - `build-x86_64-unknown-linux-musl` - Linux musl
   - `build-x86_64-pc-windows-msvc` - Windows
   - Status: ✅ All should pass

3. **host** (2-3 min)
   - Creates GitHub release
   - Uploads all binaries
   - Uploads installers
   - Status: ✅ Should complete

4. **publish-homebrew-formula** (1-2 min)
   - Checks out homebrew-tap repository
   - Updates Formula/xcargo.rb
   - Commits and pushes
   - Status: ✅ Should complete

5. **announce** (< 1 min)
   - Final success confirmation
   - Status: ✅ Should complete

**Total expected time:** 15-20 minutes

---

## Step 3: Monitor Each Stage

### Watch the plan stage

```bash
# View plan job logs
gh run view --job=plan --log
```

**Look for:**
- ✅ "Generated release plan"
- ✅ Version matches (v0.4.0)
- ✅ All 5 targets listed
- ✅ All 3 installers listed

**🚨 Red flags:**
- ❌ Version mismatch
- ❌ Missing targets
- ❌ Configuration errors

### Watch build stages

```bash
# View all build jobs
gh run view

# Check specific build
gh run view --job=build-x86_64-unknown-linux-gnu --log
```

**Look for:**
- ✅ "Building for target: x86_64-unknown-linux-gnu"
- ✅ "Build succeeded"
- ✅ Binary created and uploaded

**🚨 Red flags:**
- ❌ Compilation errors
- ❌ Missing dependencies
- ❌ Upload failures

### Watch host stage

```bash
# View host job
gh run view --job=host --log
```

**Look for:**
- ✅ "Creating release v0.4.0"
- ✅ "Uploading assets"
- ✅ Multiple "✓ uploaded" messages
- ✅ "Release published successfully"

**🚨 Red flags:**
- ❌ Upload failures
- ❌ Permission errors
- ❌ Network timeouts

### Watch Homebrew publish stage

```bash
# View homebrew job
gh run view --job=publish-homebrew-formula --log
```

**Look for:**
- ✅ "Checked out ibrahimcesar/homebrew-tap"
- ✅ "Updated Formula/xcargo.rb"
- ✅ "Committed formula"
- ✅ "Pushed to tap"

**🚨 Red flags:**
- ❌ Authentication failures (check HOMEBREW_TAP_TOKEN)
- ❌ Push failures
- ❌ Formula validation errors

---

## Step 4: Verify Release Artifacts

### Check GitHub Release page

```bash
# Open release page
gh release view "v${VERSION}" --web

# Or view in terminal
gh release view "v${VERSION}"
```

**Verify presence of:**

**Binaries (5):**
- [ ] `xcargo-aarch64-apple-darwin.tar.xz`
- [ ] `xcargo-x86_64-apple-darwin.tar.xz`
- [ ] `xcargo-x86_64-unknown-linux-gnu.tar.xz`
- [ ] `xcargo-x86_64-unknown-linux-musl.tar.xz`
- [ ] `xcargo-x86_64-pc-windows-msvc.zip`

**Checksums (5):**
- [ ] `xcargo-aarch64-apple-darwin.tar.xz.sha256`
- [ ] `xcargo-x86_64-apple-darwin.tar.xz.sha256`
- [ ] `xcargo-x86_64-unknown-linux-gnu.tar.xz.sha256`
- [ ] `xcargo-x86_64-unknown-linux-musl.tar.xz.sha256`
- [ ] `xcargo-x86_64-pc-windows-msvc.zip.sha256`

**Installers (3):**
- [ ] `xcargo-installer.sh`
- [ ] `xcargo-installer.ps1`
- [ ] `xcargo.rb`

**Other:**
- [ ] `sha256.sum` (combined checksums)
- [ ] `source.tar.gz`
- [ ] `source.tar.gz.sha256`

**Total artifacts:** 16 files

### Verify release notes

Check that release notes include:
- [ ] Version number matches
- [ ] CHANGELOG.md content included
- [ ] Installation instructions present
- [ ] Download links for all platforms

---

## Step 5: Verify Homebrew Tap

### Check tap repository

```bash
# View latest commit in tap
gh repo view ibrahimcesar/homebrew-tap

# Or clone and check
git clone https://github.com/ibrahimcesar/homebrew-tap
cd homebrew-tap
cat Formula/xcargo.rb
```

**Verify formula contains:**
- [ ] Correct version number (0.4.0)
- [ ] SHA256 checksums for all platforms
- [ ] Download URLs point to v0.4.0
- [ ] Latest commit message: "xcargo 0.4.0"

---

## Troubleshooting

### Build job fails

**Symptoms:** One or more build-* jobs fail

**Investigation:**
```bash
# View failed job logs
gh run view --job=build-PLATFORM --log

# Look for error messages
```

**Common causes:**
1. **Compilation error:** Fix in code, re-tag
2. **Missing dependency:** Update build environment
3. **Timeout:** Retry the workflow

**Resolution:**
```bash
# Delete the failed release
gh release delete "v${VERSION}" --yes

# Delete the tag
git tag -d "v${VERSION}"
git push origin :refs/tags/"v${VERSION}"

# Fix the issue
# Re-run from Step 1
```

### Homebrew publish fails

**Symptoms:** publish-homebrew-formula job fails

**Investigation:**
```bash
gh run view --job=publish-homebrew-formula --log
```

**Common causes:**
1. **Missing HOMEBREW_TAP_TOKEN:** Add secret
2. **Token expired:** Regenerate token
3. **Permission denied:** Check token permissions

**Resolution:**
```bash
# Manual fix: Update formula manually
git clone https://github.com/ibrahimcesar/homebrew-tap
cd homebrew-tap

# Download formula from release
curl -L "https://github.com/ibrahimcesar/xcargo/releases/download/v${VERSION}/xcargo.rb" \
  -o Formula/xcargo.rb

# Commit and push
git add Formula/xcargo.rb
git commit -m "xcargo ${VERSION}"
git push
```

### Release artifacts missing

**Symptoms:** Some files not uploaded

**Investigation:**
```bash
gh release view "v${VERSION}"
# Count artifacts
```

**Resolution:**
```bash
# Re-run the failed upload job
gh run rerun <run-id>

# Or manually upload missing artifacts
gh release upload "v${VERSION}" path/to/missing/file
```

### Wrong version in release

**Symptoms:** Release shows wrong version number

**Resolution:**
```bash
# Delete release
gh release delete "v${VERSION}" --yes

# Delete tag
git tag -d "v${VERSION}"
git push origin :refs/tags/"v${VERSION}"

# Fix version in Cargo.toml
# Return to 01-prepare-release.md Step 2
```

---

## Emergency Rollback

If critical issues are discovered after release:

### Option 1: Mark as pre-release

```bash
# Edit release to mark as pre-release
gh release edit "v${VERSION}" --prerelease

# Add warning to release notes
gh release edit "v${VERSION}" --notes "⚠️ This release has known issues. Use v0.3.0 instead.

See issue #XXX for details."
```

### Option 2: Delete and re-release

```bash
# Delete release (keeps tag)
gh release delete "v${VERSION}" --yes

# Fix issues
# Create new patch version (v0.4.1)
# Follow complete release process
```

See [04-hotfix-release.md](04-hotfix-release.md) for detailed rollback procedures.

---

## Success Criteria

✅ Release workflow completed successfully
✅ All 16 artifacts present in GitHub release
✅ Homebrew tap updated with new version
✅ Release notes accurate and complete
✅ No errors in workflow logs

**Next step:** [03-verify-release.md](03-verify-release.md)

---

**Last Updated:** 2025-11-23
**Owner:** xcargo maintainers
