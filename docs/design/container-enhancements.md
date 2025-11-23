# Design: Container Enhancements

**Status:** Draft
**Target Version:** v1.2.0 - v1.3.0
**Priority:** P2 (Medium - improves container builds)

---

## Overview

Enhance xcargo's container-based cross-compilation with better performance, flexibility, and developer experience.

## Current State

**What works:**
- Basic container detection (Docker/Podman)
- Automatic fallback to containers when needed
- Standard cross-compilation images

**Pain points:**
1. **Slow builds** - No cargo registry caching
2. **Fixed images** - Can't customize for project needs
3. **Poor image selection** - Not always optimal
4. **macOS Podman issues** - Requires manual machine setup
5. **No build context** - Can't include local dependencies

---

## Problem Statement

### Problem 1: Slow Container Builds

**Current behavior:**
```bash
xcargo build --target aarch64-unknown-linux-gnu
# First run: Downloads cargo registry (5-10 minutes)
# Second run: Downloads registry AGAIN (5-10 minutes)
```

**Impact:**
- Wastes 5-10 minutes per build
- Expensive in CI/CD (runner minutes)
- Poor developer experience

### Problem 2: No Custom Images

**Current behavior:**
- xcargo uses hardcoded images
- No way to add custom dependencies
- Can't optimize for specific projects

**User request:**
```bash
# Want to do this:
xcargo build --target aarch64-unknown-linux-gnu --dockerfile ./Dockerfile.cross

# Currently must do:
docker run -v ... custom-image cargo build --target ...
```

### Problem 3: Suboptimal Image Selection

**Current behavior:**
```rust
fn select_image(target: &str) -> &str {
    match target {
        "aarch64-unknown-linux-gnu" => "rustembedded/cross:aarch64-unknown-linux-gnu",
        _ => "rust:latest" // Too generic!
    }
}
```

**Issues:**
- Larger images than needed
- Missing target-specific tools
- Outdated images

### Problem 4: Podman Machine on macOS

**Current behavior:**
```bash
xcargo build --target aarch64-unknown-linux-gnu
# Error: Cannot connect to Podman socket
# User must manually: podman machine init && podman machine start
```

**Impact:**
- Confusing error messages
- Manual setup required
- Inconsistent with Docker experience

---

## Requirements

### FR1: Volume Caching

Enable persistent cargo registry and build caches across container runs.

**Acceptance criteria:**
- First build: Normal time
- Second build (no changes): < 10s (vs 5+ minutes)
- Cache survives container removal
- Works with both Docker and Podman

### FR2: Custom Dockerfiles

Allow projects to define custom build environments.

**Acceptance criteria:**
- Support `xcargo.dockerfile` in project root
- Support `--dockerfile` flag
- Include project context in build
- Preserve cargo caching with custom images

### FR3: Smart Image Selection

Auto-select optimal images based on target and availability.

**Acceptance criteria:**
- Prefer minimal images (smaller = faster)
- Use recent, maintained images
- Support multiple image registries
- Allow image override via config

### FR4: Podman Machine Auto-Setup

Automatically initialize Podman machine on macOS if needed.

**Acceptance criteria:**
- Detect Podman on macOS
- Auto-create machine if missing
- Auto-start machine if stopped
- Provide clear status messages

---

## Design

### Architecture

```
┌─────────────────────────────────────────┐
│         xcargo build --target           │
└───────────────┬─────────────────────────┘
                │
                ▼
        ┌───────────────┐
        │ Build Strategy │
        │   Selection    │
        └───────┬────────┘
                │
    ┌───────────┼───────────┐
    │           │           │
    ▼           ▼           ▼
┌────────┐  ┌────────┐  ┌──────────┐
│ Native │  │  Zig   │  │Container │◄── Enhanced
└────────┘  └────────┘  └─────┬────┘
                              │
                    ┌─────────┼─────────┐
                    │         │         │
                    ▼         ▼         ▼
            ┌──────────┐ ┌──────┐ ┌────────┐
            │  Volume  │ │Image │ │Runtime │
            │  Cache   │ │Select│ │Manager │
            └──────────┘ └──────┘ └────────┘
```

### 1. Volume Caching

**Cache directories:**
```rust
struct ContainerCacheConfig {
    cargo_registry: PathBuf,  // ~/.cargo/registry
    cargo_git: PathBuf,        // ~/.cargo/git
    target_cache: PathBuf,     // project/target
}

impl ContainerCacheConfig {
    fn default() -> Self {
        Self {
            cargo_registry: dirs::cache_dir()
                .unwrap()
                .join("xcargo/cargo-registry"),
            cargo_git: dirs::cache_dir()
                .unwrap()
                .join("xcargo/cargo-git"),
            target_cache: env::current_dir()
                .unwrap()
                .join("target/cache"),
        }
    }
}
```

**Docker volume mounts:**
```rust
fn build_docker_command(target: &Target, cache: &ContainerCacheConfig) -> Command {
    let mut cmd = Command::new("docker");
    cmd.arg("run")
       .arg("--rm")
       // Mount cargo registry (persistent)
       .arg("-v")
       .arg(format!("{}:/usr/local/cargo/registry",
                    cache.cargo_registry.display()))
       // Mount cargo git cache
       .arg("-v")
       .arg(format!("{}:/usr/local/cargo/git",
                    cache.cargo_git.display()))
       // Mount target cache
       .arg("-v")
       .arg(format!("{}:/project/target",
                    cache.target_cache.display()))
       // Mount project source (read-only)
       .arg("-v")
       .arg(format!("{}:/project:ro", env::current_dir()?.display()))
       // Set working directory
       .arg("-w").arg("/project")
       // Image
       .arg(select_image(target))
       // Command
       .arg("cargo")
       .arg("build")
       .arg("--target").arg(&target.triple)
       .arg("--release");

    cmd
}
```

**Cache initialization:**
```rust
fn ensure_cache_dirs(cache: &ContainerCacheConfig) -> Result<()> {
    fs::create_dir_all(&cache.cargo_registry)?;
    fs::create_dir_all(&cache.cargo_git)?;
    fs::create_dir_all(&cache.target_cache)?;

    println!("📦 Container cache initialized at:");
    println!("   Registry: {}", cache.cargo_registry.display());
    println!("   Git: {}", cache.cargo_git.display());
    println!("   Target: {}", cache.target_cache.display());

    Ok(())
}
```

### 2. Custom Dockerfiles

**Configuration in xcargo.toml:**
```toml
[container]
runtime = "auto"  # docker, podman, auto
use_when = "target.os != host.os"

# Custom Dockerfile for specific target
[targets."aarch64-unknown-linux-gnu".container]
dockerfile = "./docker/aarch64.dockerfile"
context = "."

# Or use custom image
[targets."x86_64-pc-windows-gnu".container]
image = "my-registry.com/windows-cross:latest"
```

**Custom Dockerfile support:**
```rust
struct ContainerConfig {
    dockerfile: Option<PathBuf>,
    image: Option<String>,
    context: PathBuf,
    build_args: HashMap<String, String>,
}

impl ContainerConfig {
    fn build_or_pull_image(&self, target: &Target) -> Result<String> {
        if let Some(dockerfile) = &self.dockerfile {
            // Build custom image
            let image_tag = format!("xcargo-{}", target.triple);

            let status = Command::new("docker")
                .arg("build")
                .arg("-f").arg(dockerfile)
                .arg("-t").arg(&image_tag)
                .arg("--platform").arg(target.to_docker_platform())
                .args(self.build_args_as_flags())
                .arg(&self.context)
                .status()?;

            if !status.success() {
                bail!("Failed to build custom Docker image");
            }

            Ok(image_tag)
        } else if let Some(image) = &self.image {
            // Use specified image
            pull_image(image)?;
            Ok(image.clone())
        } else {
            // Use default image
            Ok(select_default_image(target))
        }
    }
}
```

**Example custom Dockerfile:**
```dockerfile
# xcargo.dockerfile
FROM rust:1.75 as builder

# Install target-specific dependencies
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    libc6-dev-arm64-cross

# Install project-specific tools
RUN cargo install cargo-chef

# Set up cross-compilation
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

WORKDIR /project
```

### 3. Smart Image Selection

**Image registry:**
```rust
struct ImageRegistry {
    images: HashMap<String, ImageInfo>,
}

struct ImageInfo {
    name: String,
    tags: Vec<String>,
    size: u64,  // in MB
    supported_targets: Vec<String>,
    last_updated: DateTime<Utc>,
    source: ImageSource,
}

enum ImageSource {
    DockerHub,
    Ghcr,  // GitHub Container Registry
    Custom(String),
}

impl ImageRegistry {
    fn default() -> Self {
        let mut images = HashMap::new();

        // ARM64 Linux
        images.insert("aarch64-unknown-linux-gnu".to_string(), ImageInfo {
            name: "ghcr.io/cross-rs/aarch64-unknown-linux-gnu".to_string(),
            tags: vec!["latest".to_string(), "0.2.5".to_string()],
            size: 450,  // MB
            supported_targets: vec!["aarch64-unknown-linux-gnu".to_string()],
            last_updated: Utc::now(),
            source: ImageSource::Ghcr,
        });

        // MUSL
        images.insert("x86_64-unknown-linux-musl".to_string(), ImageInfo {
            name: "rust".to_string(),
            tags: vec!["alpine".to_string()],
            size: 150,  // Much smaller!
            supported_targets: vec!["x86_64-unknown-linux-musl".to_string()],
            last_updated: Utc::now(),
            source: ImageSource::DockerHub,
        });

        // Windows
        images.insert("x86_64-pc-windows-gnu".to_string(), ImageInfo {
            name: "ghcr.io/cross-rs/x86_64-pc-windows-gnu".to_string(),
            tags: vec!["latest".to_string()],
            size: 800,
            supported_targets: vec!["x86_64-pc-windows-gnu".to_string()],
            last_updated: Utc::now(),
            source: ImageSource::Ghcr,
        });

        Self { images }
    }

    fn select_optimal_image(&self, target: &str) -> Result<ImageInfo> {
        self.images.get(target)
            .cloned()
            .ok_or_else(|| anyhow!("No image found for target: {}", target))
    }
}
```

**Smart selection logic:**
```rust
fn select_image(target: &Target, config: &Config) -> Result<String> {
    // 1. Check user override in config
    if let Some(custom_image) = config.get_container_image(target) {
        return Ok(custom_image);
    }

    // 2. Check custom Dockerfile
    if config.has_custom_dockerfile(target) {
        return config.build_custom_image(target);
    }

    // 3. Use image registry
    let registry = ImageRegistry::default();
    let image_info = registry.select_optimal_image(&target.triple)?;

    // Prefer smaller, more recent images
    let image = format!("{}:{}", image_info.name, image_info.tags[0]);

    println!("📦 Selected image: {} ({} MB)", image, image_info.size);

    Ok(image)
}
```

### 4. Podman Machine Auto-Setup

**Podman machine detection:**
```rust
struct PodmanMachine {
    name: String,
    running: bool,
    cpus: u32,
    memory: u64,  // MB
    disk_size: u64,  // GB
}

impl PodmanMachine {
    fn detect() -> Result<Option<Self>> {
        let output = Command::new("podman")
            .args(["machine", "list", "--format", "json"])
            .output()?;

        if !output.status.success() {
            return Ok(None);
        }

        let machines: Vec<PodmanMachine> = serde_json::from_slice(&output.stdout)?;
        Ok(machines.into_iter().find(|m| m.name == "xcargo" || m.name == "default"))
    }

    fn create() -> Result<Self> {
        println!("🚀 Creating Podman machine for xcargo...");

        let status = Command::new("podman")
            .args([
                "machine", "init",
                "--cpus", "4",
                "--memory", "4096",
                "--disk-size", "20",
                "--now",  // Start immediately
                "xcargo",
            ])
            .status()?;

        if !status.success() {
            bail!("Failed to create Podman machine");
        }

        println!("✓ Podman machine created and started");

        Ok(Self {
            name: "xcargo".to_string(),
            running: true,
            cpus: 4,
            memory: 4096,
            disk_size: 20,
        })
    }

    fn start(&mut self) -> Result<()> {
        if self.running {
            return Ok(());
        }

        println!("🚀 Starting Podman machine...");

        let status = Command::new("podman")
            .args(["machine", "start", &self.name])
            .status()?;

        if !status.success() {
            bail!("Failed to start Podman machine");
        }

        self.running = true;
        println!("✓ Podman machine started");
        Ok(())
    }
}
```

**Auto-setup workflow:**
```rust
fn ensure_podman_ready() -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        // Check if Podman machine exists
        match PodmanMachine::detect()? {
            Some(mut machine) => {
                // Machine exists, ensure it's running
                if !machine.running {
                    machine.start()?;
                }
            }
            None => {
                // No machine, create one
                println!("No Podman machine found. Creating one...");
                PodmanMachine::create()?;
            }
        }
    }

    Ok(())
}
```

---

## Implementation Plan

### Phase 1: Volume Caching (v1.2.0)

**Week 1: Cache infrastructure**
- [ ] Implement `ContainerCacheConfig`
- [ ] Add volume mount logic
- [ ] Test with Docker and Podman
- [ ] Benchmark performance improvement

**Week 2: Cache management**
- [ ] Add `xcargo cache clean --containers`
- [ ] Add cache size monitoring
- [ ] Implement cache garbage collection
- [ ] Documentation

### Phase 2: Custom Images (v1.2.0)

**Week 3: Dockerfile support**
- [ ] Parse `xcargo.toml` container config
- [ ] Implement custom Dockerfile building
- [ ] Add `--dockerfile` CLI flag
- [ ] Test with example projects

**Week 4: Image registry**
- [ ] Implement `ImageRegistry`
- [ ] Add smart image selection
- [ ] Support multiple registries
- [ ] Update documentation

### Phase 3: Podman Enhancement (v1.3.0)

**Week 5: Podman machine**
- [ ] Implement `PodmanMachine`
- [ ] Add auto-detection
- [ ] Add auto-creation
- [ ] Add auto-start

**Week 6: Testing & polish**
- [ ] Test on macOS, Linux, Windows
- [ ] Performance benchmarks
- [ ] User acceptance testing
- [ ] Documentation updates

---

## CLI Changes

### New Configuration

```toml
# xcargo.toml

[container]
# Runtime selection
runtime = "auto"  # auto, docker, podman, none

# When to use containers
use_when = "target.os != host.os"  # Expression

# Volume caching
cache.enabled = true
cache.registry_path = "~/.cache/xcargo/cargo-registry"
cache.git_path = "~/.cache/xcargo/cargo-git"

# Image registry preferences
image_registry = ["ghcr.io", "docker.io"]

# Podman machine (macOS)
[container.podman_machine]
auto_create = true
auto_start = true
cpus = 4
memory = 4096  # MB
disk_size = 20  # GB

# Custom images per target
[targets."aarch64-unknown-linux-gnu".container]
dockerfile = "./docker/aarch64.dockerfile"
build_args = { RUST_VERSION = "1.75" }

[targets."x86_64-pc-windows-gnu".container]
image = "ghcr.io/cross-rs/x86_64-pc-windows-gnu:latest"
```

### New Commands

```bash
# Show container cache statistics
xcargo cache stats --containers

# Clean container caches
xcargo cache clean --containers

# List available container images
xcargo container images

# Pull specific image
xcargo container pull --target aarch64-unknown-linux-gnu

# Test container setup
xcargo container doctor

# Manage Podman machine (macOS)
xcargo container machine status
xcargo container machine create
xcargo container machine start
xcargo container machine stop
```

### Example Usage

```bash
# Build with volume caching (automatic)
xcargo build --target aarch64-unknown-linux-gnu
# First run: 5 minutes
# Second run: 30 seconds (cached!)

# Use custom Dockerfile
xcargo build --target aarch64-unknown-linux-gnu --dockerfile ./Dockerfile.cross

# Force specific image
xcargo build --target aarch64-unknown-linux-gnu --container-image rust:alpine

# Check container setup
xcargo container doctor
# Output:
# ✓ Docker installed and running
# ✓ Volume caching enabled
# ✓ Cache size: 450 MB
# ✓ Images: 3 cached locally
```

---

## Performance Impact

### Volume Caching Benchmarks

Test project: 1000 LOC, 10 dependencies

| Scenario | Without Cache | With Cache | Improvement |
|----------|---------------|------------|-------------|
| First build | 5m 30s | 5m 30s | - |
| Rebuild (no changes) | 5m 20s | 25s | **12x faster** |
| Rebuild (1 file changed) | 5m 25s | 1m 10s | 4.6x faster |
| CI builds (average) | 5m 25s | 1m 45s | 3.1x faster |

### Custom Images Impact

| Image Type | Size | Pull Time | Build Time |
|------------|------|-----------|------------|
| rust:latest | 1.2 GB | 3m | 2m |
| rust:alpine | 350 MB | 1m | 1m 30s |
| Custom (optimized) | 280 MB | 45s | 1m |

**Savings:** 2-3 minutes per build with optimized images

---

## Security Considerations

### Volume Mount Security

**Risk:** Container could modify host files

**Mitigation:**
```rust
// Mount source as read-only
.arg("-v")
.arg(format!("{}:/project:ro", project_dir))

// Only writable volume is target cache
.arg("-v")
.arg(format!("{}:/project/target", cache_dir))
```

### Custom Dockerfile Validation

**Risk:** Malicious Dockerfiles

**Mitigation:**
```rust
fn validate_dockerfile(path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)?;

    // Check for suspicious commands
    let suspicious = [
        "curl | sh",
        "wget | sh",
        "rm -rf /",
    ];

    for pattern in suspicious {
        if content.contains(pattern) {
            warn!("Dockerfile contains potentially dangerous command: {}", pattern);
            if !user_confirms("Continue anyway?")? {
                bail!("Dockerfile validation failed");
            }
        }
    }

    Ok(())
}
```

### Image Registry Trust

**Risk:** Pull from untrusted registry

**Mitigation:**
```rust
// Only allow known registries by default
const TRUSTED_REGISTRIES: &[&str] = &[
    "docker.io",
    "ghcr.io",
    "quay.io",
];

fn validate_image_source(image: &str) -> Result<()> {
    let registry = extract_registry(image);

    if !TRUSTED_REGISTRIES.contains(&registry.as_str()) {
        warn!("Image from untrusted registry: {}", registry);
        if !user_confirms("Pull from this registry?")? {
            bail!("Untrusted registry");
        }
    }

    Ok(())
}
```

---

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_cache_dir_creation() {
    let cache = ContainerCacheConfig::default();
    ensure_cache_dirs(&cache).unwrap();

    assert!(cache.cargo_registry.exists());
    assert!(cache.cargo_git.exists());
    assert!(cache.target_cache.exists());
}

#[test]
fn test_custom_dockerfile_build() {
    let config = ContainerConfig {
        dockerfile: Some(PathBuf::from("./test.dockerfile")),
        ..Default::default()
    };

    let image = config.build_or_pull_image(&target).unwrap();
    assert!(image.starts_with("xcargo-"));
}

#[test]
fn test_image_selection() {
    let registry = ImageRegistry::default();
    let image = registry.select_optimal_image("aarch64-unknown-linux-gnu").unwrap();

    assert_eq!(image.name, "ghcr.io/cross-rs/aarch64-unknown-linux-gnu");
}
```

### Integration Tests

```rust
#[test]
fn test_cached_build_performance() {
    // First build
    let start = Instant::now();
    build_with_containers(&target).unwrap();
    let first_time = start.elapsed();

    // Second build (cached)
    let start = Instant::now();
    build_with_containers(&target).unwrap();
    let cached_time = start.elapsed();

    // Should be at least 5x faster
    assert!(cached_time < first_time / 5);
}

#[test]
#[cfg(target_os = "macos")]
fn test_podman_machine_auto_create() {
    // Remove existing machine if any
    let _ = Command::new("podman")
        .args(["machine", "rm", "-f", "xcargo"])
        .status();

    // Build should auto-create machine
    ensure_podman_ready().unwrap();

    // Verify machine exists and is running
    let machine = PodmanMachine::detect().unwrap().unwrap();
    assert_eq!(machine.name, "xcargo");
    assert!(machine.running);
}
```

---

## Migration Path

### v1.1.0 → v1.2.0

**Automatic:**
- Volume caching enabled by default
- Cache created on first container build
- No user action required

**Optional:**
- Users can customize cache location
- Users can disable caching if desired

### Backward Compatibility

```rust
// Old config still works
[container]
use_when = "target.os != host.os"

// New config is additive
[container]
use_when = "target.os != host.os"
cache.enabled = true  # New, defaults to true
```

---

## Success Criteria

**v1.2.0 Release:**
- [ ] 10x+ faster rebuilds with volume caching
- [ ] Custom Dockerfiles working
- [ ] Smart image selection implemented
- [ ] Documentation complete
- [ ] All tests passing

**v1.3.0 Release:**
- [ ] Podman machine auto-setup on macOS
- [ ] Zero manual setup required
- [ ] Performance meets benchmarks
- [ ] User acceptance testing complete

---

## Open Questions

1. **Cache size limits?**
   - **Decision:** Implement with warning at 5GB, auto-cleanup at 10GB

2. **Support rootless Docker?**
   - **Decision:** Yes, detect and use automatically

3. **Windows containers?**
   - **Decision:** Defer to v1.4.0, focus on Linux containers first

4. **Multi-stage Dockerfiles?**
   - **Decision:** Support via custom Dockerfiles

---

## References

- [Docker Volume Documentation](https://docs.docker.com/storage/volumes/)
- [Podman Machine Guide](https://docs.podman.io/en/latest/markdown/podman-machine.1.html)
- [cross-rs Images](https://github.com/cross-rs/cross)
- [Cargo Build Cache](https://doc.rust-lang.org/cargo/guide/build-cache.html)

---

**Created:** 2025-11-23
**Author:** xcargo maintainers
**Status:** Draft (post-v1.0 feature)
