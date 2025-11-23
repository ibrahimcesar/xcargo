# cargo-dist Integration TODO

**Goal:** Professional binary distribution using cargo-dist, with xcargo building itself.

**Status:** Not started
**Target:** v0.5.0 or v1.1.0
**Effort:** 3-4 days
**Priority:** P1 (High value, not blocking v1.0)

---

## Phase 1: Basic Setup ⏳

**Effort:** 2 hours

- [ ] Install cargo-dist: `cargo install cargo-dist`
- [ ] Add workspace metadata to Cargo.toml:
  ```toml
  [workspace.metadata.dist]
  cargo-dist-version = "0.22.1"
  ci = ["github"]
  installers = ["shell", "powershell", "homebrew"]
  targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc"
  ]
  ```
- [ ] Run `cargo dist init` to generate `.github/workflows/release.yml`
- [ ] Review generated workflow and commit
- [ ] Create test release branch: `git checkout -b test/cargo-dist`
- [ ] Tag test release: `git tag v0.4.0-rc1`
- [ ] Push and verify GitHub Actions build all targets
- [ ] Download artifacts and verify binaries work on each platform
- [ ] Document release process in CONTRIBUTING.md

---

## Phase 2: Self-Hosting (Dogfooding) ⏳

**Effort:** 4 hours

**Goal:** Make xcargo build itself using its own cross-compilation capabilities.

- [ ] Modify `.github/workflows/release.yml`:
  ```yaml
  - name: Build xcargo with xcargo
    run: |
      # First build xcargo for the host
      cargo build --release

      # Then use it to cross-compile for all targets
      ./target/release/xcargo build --all --parallel --release
  ```

- [ ] Configure target-specific build steps:
  - [ ] Linux: Use Zig for musl static builds
  - [ ] macOS: Build both x86_64 and ARM64 (universal binary optional)
  - [ ] Windows: MSVC builds (GNU as fallback)

- [ ] Add artifact collection step:
  ```yaml
  - name: Collect xcargo binaries
    run: |
      mkdir -p dist
      cp target/x86_64-unknown-linux-gnu/release/xcargo dist/xcargo-x86_64-linux
      cp target/x86_64-unknown-linux-musl/release/xcargo dist/xcargo-x86_64-linux-musl
      # ... etc for all targets
  ```

- [ ] Feed artifacts to cargo-dist for packaging
- [ ] Test dogfooding workflow on test branch
- [ ] Verify all 5 target binaries work correctly
- [ ] Measure build time improvement vs default cross-compilation
- [ ] Document self-hosting setup in docs/

---

## Phase 3: Enhanced Distribution ⏳

**Effort:** 1 day

### Homebrew Tap

- [ ] Create GitHub repo: `ibrahimcesar/homebrew-tap`
- [ ] Configure cargo-dist to auto-update formula:
  ```toml
  [workspace.metadata.dist]
  tap = "ibrahimcesar/homebrew-tap"
  publish-jobs = ["homebrew"]
  ```
- [ ] Test installation: `brew install ibrahimcesar/tap/xcargo`
- [ ] Add tap to README installation instructions

### Shell Installers

- [ ] Verify generated `xcargo-installer.sh` includes:
  - [ ] Platform detection (Linux/macOS/Windows)
  - [ ] Architecture detection (x86_64/aarch64)
  - [ ] System checks (cargo, rustup availability)
  - [ ] PATH configuration instructions

- [ ] Test installers on clean VMs:
  - [ ] Ubuntu 22.04 LTS
  - [ ] macOS 13 (Intel)
  - [ ] macOS 14 (Apple Silicon)
  - [ ] Windows 11

- [ ] Create custom install domain:
  - [ ] Set up `https://xcargo.sh` redirect
  - [ ] Host installer at `https://install.xcargo.sh`
  - [ ] Update README: `curl https://xcargo.sh | sh`

### Windows MSI Installer

- [ ] Enable MSI in cargo-dist config:
  ```toml
  installers = ["shell", "powershell", "msi"]
  ```
- [ ] Test MSI installer on Windows
- [ ] Verify PATH is configured automatically
- [ ] Add Scoop manifest (optional)

---

## Phase 4: Production Polish ⏳

**Effort:** 2 days

### Package Manager Integration

- [ ] **APT Repository** (for Debian/Ubuntu):
  - [ ] Set up Debian repository on GitHub Pages
  - [ ] Generate .deb packages via cargo-dist
  - [ ] Test: `apt install xcargo`
  - [ ] Add to docs/installation.md

- [ ] **Scoop** (for Windows):
  - [ ] Create Scoop manifest
  - [ ] Submit to Scoop main bucket
  - [ ] Test: `scoop install xcargo`

- [ ] **AUR** (Arch Linux - community maintained):
  - [ ] Create PKGBUILD template
  - [ ] Document AUR submission process
  - [ ] Coordinate with Arch community maintainer

### Offline Installation

- [ ] Create offline installation bundle:
  - [ ] Package all target binaries in single archive
  - [ ] Include installation script
  - [ ] Add verification checksums

- [ ] Document offline installation:
  ```bash
  # Download bundle
  curl -LO https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-offline.tar.gz

  # Extract and install
  tar xzf xcargo-offline.tar.gz
  cd xcargo-offline
  ./install.sh
  ```

### Verification & Security

- [ ] Add SHA256 checksums to releases
- [ ] Sign releases with minisign/GPG
- [ ] Add verification instructions to README
- [ ] Configure cargo-dist to generate SBOM (Software Bill of Materials)

### Documentation

- [ ] Create comprehensive installation guide:
  - [ ] docs/installation.md with all methods
  - [ ] Platform-specific troubleshooting
  - [ ] Verification instructions
  - [ ] Uninstallation instructions

- [ ] Update README.md with quick install:
  ```markdown
  ## Installation

  ### Quick Install (Recommended)
  ```bash
  curl https://xcargo.sh | sh
  ```

  ### Package Managers
  - **macOS:** `brew install xcargo`
  - **Windows:** `scoop install xcargo`
  - **Arch Linux:** `yay -S xcargo`

  ### From Source
  ```bash
  cargo install xcargo
  ```
  ```

- [ ] Add release documentation:
  - [ ] How to create releases (for maintainers)
  - [ ] Release checklist
  - [ ] Rollback procedures

---

## Success Metrics

### Phase 1 (Basic)
- [ ] All 5 target binaries build successfully
- [ ] Binaries work on fresh VMs
- [ ] Release process takes < 15 minutes

### Phase 2 (Self-Hosting)
- [ ] xcargo successfully builds itself for all targets
- [ ] Build time ≤ cross/zig baseline
- [ ] Zero platform-specific hacks needed

### Phase 3 (Distribution)
- [ ] Homebrew installation works: `brew install xcargo`
- [ ] Shell installer works: `curl https://xcargo.sh | sh`
- [ ] Windows MSI installs without admin rights

### Phase 4 (Production)
- [ ] 5+ installation methods documented
- [ ] Release binaries are signed and verified
- [ ] Installation success rate > 95%

---

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| cargo-dist breaking changes | Medium | Pin to specific version, monitor releases |
| Self-hosting circular dependency | High | Keep fallback to standard cargo build |
| Platform-specific build failures | Medium | Comprehensive CI testing matrix |
| Installer security concerns | High | Sign releases, provide checksums, HTTPS only |
| Homebrew formula rejection | Low | Follow Homebrew guidelines strictly |

---

## Timeline

**Optimistic:** 3 days (full focus)
**Realistic:** 4 days (with testing)
**Conservative:** 1 week (with community feedback)

**Recommended Start:** After v1.0.0 release (80% test coverage achieved)

---

## Notes

- **Dogfooding showcase:** xcargo building xcargo is a powerful demonstration
- **Professional credibility:** Homebrew/Scoop support signals maturity
- **Reduced friction:** `brew install` vs `cargo install` is huge UX win
- **Maintenance benefit:** Automated releases free up developer time
- **Community growth:** Easier installation → more users → more contributors

---

*Created: 2025-11-23*
*Status: Planning*
*Owner: TBD*
