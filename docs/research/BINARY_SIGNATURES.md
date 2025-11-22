# Binary Signature Research

**Goal:** Allow users to verify xcargo binaries and sign their own built binaries for distribution.

## Use Cases

### 1. xcargo Release Binaries
Users download xcargo from GitHub releases and want to verify:
- Binary hasn't been tampered with
- Binary is from official source
- Supply chain integrity

### 2. User's Built Binaries
Users build their applications with xcargo and want to:
- Sign binaries for distribution
- Meet platform requirements (macOS, Windows)
- Establish trust with end users

## Signature Approaches

### Option A: GPG/PGP Signing (Traditional, Universal)

**Pros:**
- ✅ Universal - works on all platforms
- ✅ No cost - free to use
- ✅ Well-understood by developers
- ✅ Works with package managers (apt, yum, brew)

**Cons:**
- ❌ Complex key management
- ❌ Users need GPG installed
- ❌ Doesn't satisfy platform requirements (macOS Gatekeeper)

**Implementation:**
```bash
# Sign binary
gpg --armor --detach-sign xcargo

# Verify
gpg --verify xcargo.asc xcargo
```

**For xcargo releases:**
```yaml
# .github/workflows/release.yml
- name: Sign binaries with GPG
  run: |
    echo "${{ secrets.GPG_PRIVATE_KEY }}" | gpg --import
    gpg --armor --detach-sign target/release/xcargo
```

### Option B: Minisign (Modern, Simple)

**Pros:**
- ✅ Extremely simple - single command
- ✅ Small keys (256-bit Ed25519)
- ✅ Fast and secure
- ✅ Used by rustup, many Rust tools
- ✅ No dependencies (standalone binary)

**Cons:**
- ❌ Doesn't satisfy platform requirements
- ❌ Less widely known than GPG
- ❌ Users need minisign tool

**Implementation:**
```bash
# Generate key (once)
minisign -G

# Sign binary
minisign -Sm xcargo

# Verify
minisign -Vm xcargo -P <public_key>
```

**For xcargo:**
```toml
# Public key in README
minisign public key: RWQ...xyz
```

### Option C: Cosign (Container-focused, Keyless)

**Pros:**
- ✅ Keyless signing option (OIDC)
- ✅ Transparency log (Rekor)
- ✅ Modern, supply chain focused
- ✅ Works with containers

**Cons:**
- ❌ Complex setup
- ❌ Primarily for containers
- ❌ Doesn't satisfy platform requirements

### Option D: macOS Code Signing

**Pros:**
- ✅ Required for macOS distribution
- ✅ Satisfies Gatekeeper
- ✅ Allows notarization
- ✅ Best user experience on macOS

**Cons:**
- ❌ **Requires Apple Developer account ($99/year)**
- ❌ macOS only
- ❌ Complex process

**Implementation:**
```bash
# Sign with Apple Developer certificate
codesign --sign "Developer ID Application" xcargo

# Notarize
xcrun notarytool submit xcargo.zip

# Staple
xcrun stapler staple xcargo
```

### Option E: Windows Authenticode

**Pros:**
- ✅ Required for Windows distribution
- ✅ Satisfies SmartScreen
- ✅ Best user experience on Windows

**Cons:**
- ❌ **Requires code signing certificate ($100-500/year)**
- ❌ Windows only
- ❌ Complex setup

**Implementation:**
```bash
# Sign with certificate
signtool sign /f cert.pfx /p password xcargo.exe
```

### Option F: Sigstore (Free, Modern)

**Pros:**
- ✅ **Free and keyless**
- ✅ Transparency log
- ✅ Modern approach
- ✅ Growing adoption

**Cons:**
- ❌ New/experimental
- ❌ Doesn't satisfy platform requirements
- ❌ Requires online verification

## Recommended Approach for xcargo

### Tier 1: Minisign (Immediate Implementation)

**Why Minisign:**
1. Simple for users and maintainers
2. Used by rustup (familiar to Rust developers)
3. No cost, no certificates
4. Easy GitHub Actions integration

**Implementation Plan:**

1. **Generate signing key:**
```bash
minisign -G -p xcargo.pub -s xcargo.key
```

2. **Add to GitHub Actions:**
```yaml
- name: Sign binaries with minisign
  run: |
    # Install minisign
    cargo install minisign

    # Sign each binary
    echo "${{ secrets.MINISIGN_KEY }}" > xcargo.key
    for file in target/*/release/xcargo*; do
      minisign -Sm "$file" -s xcargo.key
    done
```

3. **Publish public key in README:**
```markdown
## Verifying xcargo Binaries

Download: https://github.com/ibrahimcesar/xcargo/releases

Verify with [minisign](https://jedisct1.github.io/minisign/):
\`\`\`bash
minisign -Vm xcargo -P RWQ...xyz
\`\`\`

Public key: `RWQ...xyz`
```

### Tier 2: Platform Signing (Future, When Revenue Justifies Cost)

**macOS Code Signing:**
- When: After getting Apple Developer account
- Cost: $99/year
- Benefit: Better macOS user experience

**Windows Code Signing:**
- When: After getting certificate
- Cost: $100-500/year
- Benefit: Better Windows user experience

### Tier 3: User Binary Signing

Add `xcargo sign` subcommand to help users sign their own binaries:

```bash
# xcargo automatically signs built binaries
xcargo build --target x86_64-pc-windows-gnu --sign

# Or sign existing binaries
xcargo sign target/x86_64-pc-windows-gnu/release/myapp.exe
```

**Configuration:**
```toml
# xcargo.toml
[signing]
# Auto-sign on build
enabled = true

# Signing method: minisign, gpg, codesign (macOS), signtool (Windows)
method = "minisign"

# Key path
key = "~/.xcargo/signing.key"

# macOS: Developer ID
macos_identity = "Developer ID Application: Your Name (TEAMID)"

# Windows: Certificate path
windows_cert = "path/to/cert.pfx"
```

## Implementation Phases

### Phase 1: xcargo Release Signing (Week 1)
- [ ] Generate minisign key pair
- [ ] Add minisign to GitHub Actions
- [ ] Sign all release binaries
- [ ] Document verification in README
- [ ] Publish public key

### Phase 2: Documentation & Verification (Week 1)
- [ ] Add "Verifying Downloads" section to README
- [ ] Create verification instructions
- [ ] Add checksums (SHA256) to releases
- [ ] Create verification script

### Phase 3: User Binary Signing (Month 2)
- [ ] Create `src/signing/` module
- [ ] Implement `xcargo sign` subcommand
- [ ] Support minisign, GPG
- [ ] Add signing configuration to xcargo.toml
- [ ] Document user signing workflow

### Phase 4: Platform Signing (When Revenue Justifies)
- [ ] Acquire Apple Developer account
- [ ] Implement macOS code signing
- [ ] Acquire Windows certificate
- [ ] Implement Windows Authenticode
- [ ] Automate notarization/verification

## Quick Win: GitHub Actions Release Workflow

Update `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-and-sign:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - name: Build release binary
        run: |
          cargo build --release --target ${{ matrix.target }}

      - name: Install minisign
        run: cargo install minisign

      - name: Sign binary
        run: |
          echo "${{ secrets.MINISIGN_KEY }}" > signing.key
          minisign -Sm target/${{ matrix.target }}/release/xcargo* -s signing.key
          rm signing.key

      - name: Generate checksums
        run: |
          cd target/${{ matrix.target }}/release
          sha256sum xcargo* > SHA256SUMS

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: xcargo-${{ matrix.target }}
          path: |
            target/${{ matrix.target }}/release/xcargo*
            target/${{ matrix.target }}/release/*.minisig
            target/${{ matrix.target }}/release/SHA256SUMS
```

## Comparison Matrix

| Method | Cost | Complexity | Platform Support | User Adoption | Recommendation |
|--------|------|------------|------------------|---------------|----------------|
| **Minisign** | Free | Low | All | High (Rust) | ✅ **Start here** |
| GPG | Free | Medium | All | Medium | ⚠️ Fallback |
| Cosign | Free | High | Containers | Low | ❌ Skip |
| macOS Sign | $99/yr | High | macOS only | High | 🔵 Later |
| Windows Sign | $100-500/yr | High | Windows only | High | 🔵 Later |

## Security Considerations

1. **Key Storage:**
   - Store signing keys in GitHub Secrets
   - Never commit keys to repository
   - Rotate keys annually

2. **Verification Instructions:**
   - Make verification easy and documented
   - Provide automated verification scripts
   - Include checksums for multiple checks

3. **Transparency:**
   - Publish public keys prominently
   - Document signing process
   - Maintain key rotation history

## Next Steps

1. **Immediate (This Week):**
   - Generate minisign key pair
   - Update GitHub Actions workflow
   - Add verification docs to README

2. **Short Term (This Month):**
   - Test signing workflow on release
   - Create verification script
   - Add checksums to releases

3. **Future (When Budget Allows):**
   - Acquire signing certificates
   - Implement platform-specific signing
   - Add `xcargo sign` for user binaries

## Example README Section

```markdown
## 🔐 Verifying Downloads

All xcargo release binaries are signed with [minisign](https://jedisct1.github.io/minisign/).

### Install minisign
\`\`\`bash
# macOS
brew install minisign

# Linux
cargo install minisign

# Windows
scoop install minisign
\`\`\`

### Verify Binary
\`\`\`bash
# Download binary and signature
wget https://github.com/ibrahimcesar/xcargo/releases/download/v0.2.0/xcargo-x86_64-unknown-linux-gnu
wget https://github.com/ibrahimcesar/xcargo/releases/download/v0.2.0/xcargo-x86_64-unknown-linux-gnu.minisig

# Verify
minisign -Vm xcargo-x86_64-unknown-linux-gnu -P RWQ...xyz
\`\`\`

### Public Key
\`\`\`
RWQ...xyz
\`\`\`

### SHA256 Checksums
Checksums are available in each release's `SHA256SUMS` file.
```

## Resources

- [Minisign](https://jedisct1.github.io/minisign/)
- [Rustup's signing approach](https://github.com/rust-lang/rustup/blob/master/ci/sign.sh)
- [Sigstore](https://www.sigstore.dev/)
- [Apple Code Signing](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution)
- [Windows Authenticode](https://learn.microsoft.com/en-us/windows/win32/seccrypto/cryptography-tools)
