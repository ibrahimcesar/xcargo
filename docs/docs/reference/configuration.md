# Configuration Reference

xcargo can be configured using an `xcargo.toml` file in your project root. This file allows you to customize target selection, build behavior, container usage, and more.

## Configuration Discovery

xcargo automatically searches for `xcargo.toml` starting from the current directory and moving up the directory tree until it finds one or reaches the filesystem root. This allows you to:

- Have a project-level configuration in your project root
- Have a workspace-level configuration that applies to all projects
- Override settings via CLI flags

## File Format

The configuration file uses TOML format with four main sections:

```toml
[targets]    # Target platform configuration
[build]      # Build behavior configuration
[container]  # Container runtime configuration
[profiles]   # Named build profiles
```

## Targets Section

Configure which targets to build and how to build them.

### `targets.default`

Array of target triples to build when no target is specified via CLI.

```toml
[targets]
default = [
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
]
```

**Type**: Array of strings
**Default**: `[]` (empty)
**Example**: `["x86_64-unknown-linux-gnu"]`

### Per-Target Configuration

Customize settings for specific targets using `[targets."target-triple"]` sections.

```toml
[targets."x86_64-pc-windows-gnu"]
linker = "x86_64-w64-mingw32-gcc"
force_container = false
rustflags = ["--cfg", "feature=\"custom\""]

[targets."x86_64-pc-windows-gnu".env]
CC = "x86_64-w64-mingw32-gcc"
CXX = "x86_64-w64-mingw32-g++"
```

#### `linker`

Custom linker to use for this target.

**Type**: String (optional)
**Default**: Auto-detected
**Example**: `"x86_64-w64-mingw32-gcc"`

#### `force_container`

Force container build for this specific target, even if native build is possible.

**Type**: Boolean (optional)
**Default**: `false`
**Example**: `true`

#### `env`

Additional environment variables to set when building for this target.

**Type**: Table of string key-value pairs
**Default**: `{}`
**Example**: `{ CC = "clang", CFLAGS = "-O3" }`

#### `rustflags`

Additional RUSTFLAGS to pass to the compiler for this target.

**Type**: Array of strings (optional)
**Default**: `[]`
**Example**: `["--cfg", "feature=\"custom\""]`

## Build Section

Configure build behavior and performance.

```toml
[build]
parallel = true
jobs = 4
cache = true
force_container = false
cargo_flags = ["--verbose"]
```

### `build.parallel`

Enable parallel builds when building multiple targets.

**Type**: Boolean
**Default**: `true`
**Example**: `true`

When enabled, xcargo will build multiple targets concurrently using available CPU cores.

### `build.jobs`

Number of parallel jobs to run. When omitted, xcargo auto-detects based on available CPU cores.

**Type**: Integer (optional)
**Default**: Auto-detected
**Example**: `4`
**Constraints**: Must be greater than 0

### `build.cache`

Enable build caching to speed up subsequent builds.

**Type**: Boolean
**Default**: `true`
**Example**: `true`

### `build.force_container`

Force all builds to use containers, even when native compilation is possible.

**Type**: Boolean
**Default**: `false`
**Example**: `false`

Useful for ensuring reproducible builds across different development environments.

### `build.cargo_flags`

Additional flags to pass to cargo for all builds.

**Type**: Array of strings
**Default**: `[]`
**Example**: `["--verbose", "--locked"]`

## Container Section

Configure container runtime behavior.

```toml
[container]
runtime = "auto"
use_when = "target.os != host.os"
registry = "ghcr.io/xcargo"
pull_policy = "if-not-present"
```

### `container.runtime`

Which container runtime to use.

**Type**: String
**Default**: `"auto"`
**Valid values**: `"auto"`, `"youki"`, `"docker"`, `"podman"`

- `"auto"`: Automatically detect available runtime (prefers youki → docker → podman)
- `"youki"`: Use embedded youki runtime
- `"docker"`: Use Docker
- `"podman"`: Use Podman

### `container.use_when`

Condition that determines when to use containers.

**Type**: String
**Default**: `"target.os != host.os"`
**Valid values**: `"always"`, `"never"`, `"target.os != host.os"`

- `"always"`: Always use containers for builds
- `"never"`: Never use containers, only native builds
- `"target.os != host.os"`: Use containers when cross-compiling to a different OS (recommended)

### `container.registry`

Custom container image registry to pull build images from.

**Type**: String (optional)
**Default**: Docker Hub
**Example**: `"ghcr.io/xcargo"`

### `container.pull_policy`

Image pull policy for container images.

**Type**: String
**Default**: `"if-not-present"`
**Valid values**: `"always"`, `"never"`, `"if-not-present"`

- `"always"`: Always pull the latest image
- `"never"`: Never pull, use cached images only
- `"if-not-present"`: Pull only if image is not cached locally

## Profiles Section

Define named profiles for different build scenarios.

```toml
[profiles.ci]
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
]

[profiles.release-all]
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
    "x86_64-apple-darwin",
    "aarch64-unknown-linux-gnu",
    "aarch64-apple-darwin",
]
```

### Profile Structure

Each profile can override targets and build configuration.

#### `profiles.<name>.targets`

Array of targets to build when using this profile.

**Type**: Array of strings
**Required**: Yes
**Example**: `["x86_64-unknown-linux-gnu", "aarch64-apple-darwin"]`

#### Build Configuration Override

You can include any `[build]` section fields directly in the profile to override the default build configuration.

```toml
[profiles.fast]
targets = ["x86_64-unknown-linux-gnu"]
parallel = true
jobs = 8
cache = true
```

## Example Configurations

### Minimal Configuration

```toml
[targets]
default = ["x86_64-unknown-linux-gnu"]
```

### Development Setup

```toml
[targets]
default = ["x86_64-unknown-linux-gnu"]

[build]
parallel = true
cache = true
cargo_flags = ["--verbose"]

[container]
runtime = "auto"
use_when = "target.os != host.os"
```

### CI/CD Configuration

```toml
[targets]
default = []

[build]
parallel = true
force_container = true
cache = true

[container]
runtime = "docker"
use_when = "always"
pull_policy = "always"

[profiles.ci]
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
    "x86_64-apple-darwin",
]
```

### Multi-Platform Release

```toml
[targets]
default = ["x86_64-unknown-linux-gnu"]

[build]
parallel = true
jobs = 4
cache = true

[container]
runtime = "auto"
use_when = "target.os != host.os"
pull_policy = "if-not-present"

[profiles.release]
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
    "x86_64-apple-darwin",
    "aarch64-unknown-linux-gnu",
    "aarch64-apple-darwin",
    "aarch64-pc-windows-msvc",
]

[targets."x86_64-pc-windows-gnu"]
linker = "x86_64-w64-mingw32-gcc"

[targets."x86_64-pc-windows-gnu".env]
CC = "x86_64-w64-mingw32-gcc"
```

## Configuration Merging

When configuration is specified in multiple places, xcargo merges them with the following precedence (highest to lowest):

1. CLI flags (e.g., `--target`, `--jobs`)
2. `xcargo.toml` in current directory
3. `xcargo.toml` in parent directories
4. Default configuration

## Validation

xcargo validates configuration on load and will report errors for:

- Invalid runtime values (must be: auto, youki, docker, podman)
- Invalid pull policy values (must be: always, never, if-not-present)
- Invalid jobs count (must be > 0)
- Unknown fields (strict parsing)

## Environment Variables

Some configuration can be overridden via environment variables:

- `XCARGO_RUNTIME`: Override `container.runtime`
- `XCARGO_JOBS`: Override `build.jobs`
- `XCARGO_CACHE`: Override `build.cache` (1=true, 0=false)

## See Also

- [CLI Commands](./cli-commands.md) - Command-line interface reference
- [Targets](./targets.md) - Supported target platforms
- [Environment Variables](./environment-variables.md) - Environment variable reference
