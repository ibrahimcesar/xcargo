# Container Security Best Practices

**Purpose:** Ensure safe and secure container-based cross-compilation with xcargo.

**Audience:** Users performing container builds, security-conscious developers, CI/CD engineers.

---

## Table of Contents

1. [Overview](#overview)
2. [Image Verification](#image-verification)
3. [Image Selection](#image-selection)
4. [Runtime Security](#runtime-security)
5. [Network Security](#network-security)
6. [Build Isolation](#build-isolation)
7. [CI/CD Security](#cicd-security)
8. [Troubleshooting](#troubleshooting)

---

## Overview

### Why Container Security Matters

Container-based builds provide isolation and reproducibility, but they also introduce security considerations:

- **Image trust** - Are you running code from a trusted source?
- **Network exposure** - Can malicious images access external resources?
- **Host access** - Is your system protected from container breakout?
- **Data leakage** - Are your source files and credentials protected?

**xcargo's approach:** Security by design with sensible defaults and user control.

---

## Image Verification

### Default Images

xcargo uses **official Rust images** from Docker Hub by default:

```toml
[container]
# Default: Official Rust image (verified by Docker Hub)
image = "rust:latest"
```

**Trust level:** ✅ High - Official images are:
- Maintained by the Rust team
- Scanned for vulnerabilities
- Signed by Docker Hub
- Regularly updated

### Image Hash Pinning (Recommended)

**🔒 Best Practice:** Pin to specific image digests for maximum security and reproducibility.

```toml
[container]
# Pin to specific SHA256 digest
image = "rust:1.70@sha256:7a2e4c9f8a3b1d5e6f7c8d9e0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b"
```

**Benefits:**
- ✅ Immutable - exact same image every time
- ✅ Tamper-proof - hash mismatch = pull fails
- ✅ Auditable - specific version in version control
- ✅ Reproducible - builds are deterministic

### How to Get Image Digests

#### Method 1: Docker Inspect

```bash
# Pull the image you want
docker pull rust:1.70

# Get the digest
docker inspect rust:1.70 --format='{{.RepoDigests}}'

# Output: [rust@sha256:abc123...]
```

#### Method 2: Docker Hub Web UI

1. Visit https://hub.docker.com/_/rust
2. Click on a tag (e.g., "1.70")
3. Copy the "Digest" value
4. Use in config: `rust:1.70@sha256:<digest>`

#### Method 3: Manifest Inspection

```bash
# Get manifest digest
docker manifest inspect rust:1.70 | jq -r '.config.digest'
```

### Complete Example

```toml
# xcargo.toml
[container]
# Use specific Rust version with digest pinning
image = "rust:1.70.0-slim@sha256:7a2e4c9f8a3b1d5e6f7c8d9e0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b"

# Or use bullseye variant for Debian-based builds
# image = "rust:1.70.0-bullseye@sha256:..."

# Or use alpine for minimal images
# image = "rust:1.70.0-alpine@sha256:..."
```

**Update process:**

```bash
# 1. Test new image
docker pull rust:1.71.0
docker inspect rust:1.71.0 --format='{{.RepoDigests}}'

# 2. Update xcargo.toml
# image = "rust:1.71.0@sha256:newdigest..."

# 3. Test build
xcargo build --target x86_64-unknown-linux-musl --container

# 4. Commit to version control
git add xcargo.toml
git commit -m "chore: update Rust container image to 1.71.0"
```

---

## Image Selection

### Choosing the Right Base Image

#### Official Rust Images (Recommended) ✅

```toml
# Debian-based (default, full features)
image = "rust:1.70"

# Slim variant (smaller, fewer tools)
image = "rust:1.70-slim"

# Alpine-based (smallest, musl libc)
image = "rust:1.70-alpine"

# Ubuntu-based
image = "rust:1.70-bookworm"
```

**Recommendation:** Start with `rust:slim` for a good balance of size and functionality.

#### Third-Party Images (Use with Caution) ⚠️

Only use third-party images from trusted sources:

```toml
# messense's cross images (popular for cross-compilation)
image = "ghcr.io/cross-rs/aarch64-unknown-linux-gnu:latest"
```

**Before using third-party images:**

1. ✅ Verify the publisher's reputation
2. ✅ Check GitHub repository for Dockerfile
3. ✅ Review image layers: `docker history <image>`
4. ✅ Scan for vulnerabilities (see below)
5. ✅ Pin to specific digest

#### Custom Images (Advanced) 🔧

For specialized needs, build your own:

```dockerfile
# Dockerfile
FROM rust:1.70-slim

# Install cross-compilation tools
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

# Add Zig for better musl support
RUN wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz \
    && tar -xf zig-linux-x86_64-0.11.0.tar.xz \
    && mv zig-linux-x86_64-0.11.0/zig /usr/local/bin/

USER rust
```

```bash
# Build and tag
docker build -t my-xcargo-rust:1.70 .

# Get digest
docker inspect my-xcargo-rust:1.70 --format='{{.Id}}'

# Use in xcargo.toml
# image = "my-xcargo-rust:1.70@sha256:..."
```

---

## Runtime Security

### Rootless Containers (Recommended) 🔒

Run Docker/Podman in rootless mode for better security:

#### Rootless Docker

```bash
# Install rootless Docker
curl -fsSL https://get.docker.com/rootless | sh

# Set environment variables
export PATH=$HOME/bin:$PATH
export DOCKER_HOST=unix://$XDG_RUNTIME_DIR/docker.sock

# Verify
docker info | grep rootless
```

#### Rootless Podman (Built-in)

```bash
# Podman is rootless by default
podman info | grep runAsUser
# Should show your user ID, not 0 (root)
```

**Benefits:**
- ✅ Container escape affects only your user
- ✅ No privileged operations possible
- ✅ Better multi-user systems security

### Resource Limits

Prevent resource exhaustion attacks:

```toml
[container]
# Memory limit (prevents memory bombs)
memory_limit = "4G"

# CPU limit (prevents CPU exhaustion)
cpu_limit = "2.0"

# Build timeout (prevents infinite loops)
timeout_seconds = 3600
```

**Note:** xcargo doesn't yet support these options, but you can configure them in Docker:

```bash
# Manual resource limits
docker run --memory=4g --cpus=2.0 --timeout=3600 rust:latest cargo build
```

### Read-Only Mounts

Mount source code as read-only when possible:

```bash
# xcargo automatically mounts project as read-only for check operations
xcargo check --target x86_64-unknown-linux-gnu --container
```

**Future feature:** Explicit read-only mode in xcargo.toml.

---

## Network Security

### Network Isolation

By default, containers have network access. For sensitive builds:

#### Offline Builds

```bash
# Pre-pull dependencies
cargo vendor

# Build with vendored deps (no network)
xcargo build --target x86_64-unknown-linux-musl --container -- --offline
```

#### Network Namespace Isolation

```bash
# Docker: Disable network
docker run --network=none rust:latest cargo build --offline

# Podman: Same syntax
podman run --network=none rust:latest cargo build --offline
```

**Future xcargo feature:**

```toml
[container]
# Disable network access during builds
network = "none"
```

### Private Registries

For airgapped or private environments:

```bash
# Use private registry
docker pull myregistry.company.com/rust:1.70

# Tag for xcargo
docker tag myregistry.company.com/rust:1.70 rust:latest
```

```toml
[container]
# Reference private image
image = "myregistry.company.com/rust:1.70@sha256:..."
```

---

## Build Isolation

### Secrets Management

**🚨 NEVER** include secrets in container images or build contexts.

#### What to Avoid ❌

```dockerfile
# BAD: Hardcoded secrets
ENV API_KEY="secret123"

# BAD: Copying .env files
COPY .env /app/
```

#### Safe Practices ✅

```bash
# 1. Use build-time secrets (Docker BuildKit)
docker build --secret id=cargo_token,src=$HOME/.cargo/credentials .

# 2. Runtime secrets (environment variables)
docker run -e CARGO_REGISTRY_TOKEN="$(cat ~/.cargo/credentials)" rust:latest

# 3. Secret files (mount at runtime, not build time)
docker run -v ~/.cargo/credentials:/root/.cargo/credentials:ro rust:latest
```

**xcargo approach:** Credentials are never passed to containers by default.

### File Permissions

Containers may create files with different ownership:

```bash
# After container build, fix ownership
sudo chown -R $USER:$USER target/
```

**Better approach:** Use rootless containers (see above).

---

## CI/CD Security

### GitHub Actions

```yaml
name: Secure Container Builds

on: [push]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Install xcargo
        run: |
          curl --proto '=https' --tlsv1.2 -LsSf \
            https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.sh | sh

      # Security: Pin image digest
      - name: Verify container image
        run: |
          EXPECTED_DIGEST="sha256:abc123..."
          ACTUAL_DIGEST=$(docker inspect rust:1.70 --format='{{index .RepoDigests 0}}' | cut -d@ -f2)

          if [ "$ACTUAL_DIGEST" != "$EXPECTED_DIGEST" ]; then
            echo "Image digest mismatch!"
            exit 1
          fi

      - name: Build with verified image
        run: xcargo build --target x86_64-unknown-linux-musl --container

      # Security: Scan built artifacts
      - name: Scan binary
        run: |
          # Example: check for suspicious strings
          strings target/x86_64-unknown-linux-musl/release/myapp | grep -v "expected_string" || true
```

### GitLab CI

```yaml
container_build:
  image: docker:latest
  services:
    - docker:dind

  script:
    # Verify image before use
    - docker pull rust:1.70@sha256:abc123...
    - docker inspect rust:1.70 --format='{{.RepoDigests}}'

    # Build with xcargo
    - xcargo build --target x86_64-unknown-linux-gnu --container

  only:
    - main
```

### Image Scanning

Scan images for vulnerabilities before use:

```bash
# Using Trivy (recommended)
trivy image rust:1.70

# Using Grype
grype rust:1.70

# Using Docker Scout
docker scout cves rust:1.70
```

**Example output:**

```
rust:1.70 (debian 11.6)
=======================
Total: 0 (CRITICAL: 0, HIGH: 0, MEDIUM: 0, LOW: 0)
```

**Action on vulnerabilities:**

- **CRITICAL/HIGH:** Do not use, wait for patched image
- **MEDIUM:** Assess risk, document exception
- **LOW:** Monitor, acceptable for most use cases

---

## Troubleshooting

### Image Pull Failures

**Problem:** `Error: Failed to pull image: rust:1.70@sha256:...`

**Causes:**
1. Digest mismatch (image updated)
2. Network issues
3. Registry authentication

**Solutions:**

```bash
# 1. Check current digest
docker pull rust:1.70
docker inspect rust:1.70 --format='{{.RepoDigests}}'

# 2. Update xcargo.toml with new digest

# 3. Clear Docker cache
docker system prune -a

# 4. Check network/proxy
docker pull --debug rust:1.70
```

### Permission Denied

**Problem:** `permission denied while trying to connect to Docker daemon`

**Solutions:**

```bash
# 1. Add user to docker group
sudo usermod -aG docker $USER
newgrp docker

# 2. Use rootless Docker (recommended)
curl -fsSL https://get.docker.com/rootless | sh

# 3. Use Podman (no daemon, rootless by default)
brew install podman
xcargo build --container  # automatically detects podman
```

### Build Artifacts Owned by Root

**Problem:** Files in `target/` owned by root after container build

**Solutions:**

```bash
# 1. Use rootless containers (prevents issue)
# See "Rootless Containers" section above

# 2. Fix ownership after build
sudo chown -R $USER:$USER target/

# 3. Run container as your user (advanced)
docker run --user $(id -u):$(id -g) rust:latest cargo build
```

---

## Security Checklist

Use this checklist for production container builds:

### Image Security ✅

- [ ] Using official Rust images or verified third-party images
- [ ] Image digest is pinned in xcargo.toml
- [ ] Image has been scanned for vulnerabilities
- [ ] Image is from a trusted registry
- [ ] Dockerfile reviewed (if custom image)

### Runtime Security ✅

- [ ] Using rootless Docker/Podman
- [ ] Resource limits configured (if applicable)
- [ ] Network isolation configured (for sensitive builds)
- [ ] No secrets in container images or environment
- [ ] Source code mounted as read-only (where possible)

### Build Security ✅

- [ ] Dependencies vendored (for offline builds)
- [ ] No credentials passed to containers
- [ ] Build outputs scanned (optional)
- [ ] Provenance tracking enabled (cargo-dist)

### CI/CD Security ✅

- [ ] Image verification in CI pipeline
- [ ] Secrets managed via CI secrets (not hardcoded)
- [ ] Build artifacts checksum verified
- [ ] Security scanning integrated

---

## Quick Reference

### Secure Configuration Example

```toml
# xcargo.toml - Production-grade container security

[container]
# Use specific Rust version with digest pinning
image = "rust:1.70.0-slim@sha256:7a2e4c9f8a3b1d5e6f7c8d9e0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b"

# Use containers only when cross-compiling
use_when = "target.os != host.os"

# Prefer Podman for rootless security
runtime = "podman"
```

### Command Examples

```bash
# Build with verified image
xcargo build --target x86_64-unknown-linux-musl --container

# Offline build (no network)
cargo vendor
xcargo build --container -- --offline

# Check without building (read-only)
xcargo check --target aarch64-unknown-linux-gnu --container

# Scan image before use
trivy image rust:1.70
```

---

## Additional Resources

- [Docker Security Best Practices](https://docs.docker.com/engine/security/)
- [Podman Security](https://podman.io/getting-started/security)
- [OWASP Container Security](https://owasp.org/www-project-docker-security/)
- [Rust Docker Official Images](https://hub.docker.com/_/rust)
- [Container Signing with Cosign](https://github.com/sigstore/cosign)

---

## Getting Help

- **Security issues:** security@xcargo.dev or [GitHub Security Advisories](https://github.com/ibrahimcesar/xcargo/security/advisories)
- **General questions:** [GitHub Discussions](https://github.com/ibrahimcesar/xcargo/discussions)
- **Bugs:** [GitHub Issues](https://github.com/ibrahimcesar/xcargo/issues)

---

**Last updated:** 2025-11-23
**Applies to:** xcargo v1.0.0+
