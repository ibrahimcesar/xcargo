---
sidebar_position: 1
---

# Architecture Overview

Understanding how xcargo works under the hood.

## High-Level Architecture

xcargo is designed around a simple workflow:

```
┌─────────────────────────────────────┐
│  User Command: xcargo build         │
└──────────────┬──────────────────────┘
               │
               ▼
     ┌─────────────────────┐
     │  Target Detection   │
     │  - Parse triple     │
     │  - Classify tier    │
     └──────────┬──────────┘
                │
                ▼
     ┌─────────────────────┐
     │  Requirement Check  │
     │  - Detect linker    │
     │  - Check tools      │
     └──────────┬──────────┘
                │
                ▼
     ┌─────────────────────┐
     │  Build Strategy     │
     │  - Native/Container │
     └──────────┬──────────┘
                │
                ▼
     ┌─────────────────────┐
     │  Execute Build      │
     └─────────────────────┘
```

## Core Modules

### Target Module

The foundation of xcargo. Handles:

- **Target Triple Parsing**: Breaking down target identifiers
- **Tier Classification**: Categorizing targets (Native, Container, Specialized)
- **Requirement Detection**: Identifying needed tools and linkers
- **Alias Resolution**: Converting friendly names like "linux" to full triples

[Learn more →](./target-detection.md)

### Toolchain Module

Manages Rust toolchains and cross-compilation tools:

- **Rustup Integration**: Automatic target installation
- **Linker Detection**: Finding appropriate linkers
- **Tool Verification**: Checking for required system tools

[Learn more →](./toolchain-management.md)

### Build Module

Orchestrates the compilation process:

- **Strategy Selection**: Native vs container builds
- **Parallel Execution**: Building multiple targets simultaneously  
- **Caching**: Intelligent build artifact caching

[Learn more →](./build-strategy.md)

### Container Module

Handles containerized builds:

- **Runtime Selection**: youki (embedded) → docker → podman
- **Image Management**: Caching and updates
- **Volume Mounting**: Efficient source code access

[Learn more →](./container-runtime.md)

### Config Module

Configuration file parsing and management:

- **TOML Parsing**: Reading `xcargo.toml`
- **Defaults**: Sensible default configurations
- **Validation**: Ensuring config correctness

## Data Flow

1. **Command Parse**: CLI arguments are parsed by clap
2. **Config Load**: `xcargo.toml` is loaded if present
3. **Target Resolution**: Target names are resolved to triples
4. **Capability Detection**: System capabilities are checked
5. **Strategy Selection**: Native or container build is chosen
6. **Build Execution**: Cargo is invoked with appropriate settings
7. **Result Collection**: Binaries are collected and reported

## Key Design Principles

### Native-First Strategy

xcargo always prefers native builds for performance. Container builds are only used when:
- Required cross-compilation tools are missing
- The target is classified as requiring containers (Tier 2/3)
- User explicitly requests container build

### Transparent Operation

xcargo shows exactly what it's doing:
- Which linker is being used
- Whether a container is needed
- What tools are missing

### Minimal Dependencies

The core of xcargo has minimal dependencies to:
- Keep binary size small
- Reduce compilation time
- Minimize security surface

## Error Handling

xcargo uses Rust's type system for robust error handling:

```rust
pub enum Error {
    Io(std::io::Error),
    TargetNotFound(String),
    Toolchain(String),
    Build(String),
    Config(String),
}
```

Errors are propagated using `Result<T, Error>` and include context for debugging.

## Performance Considerations

- **Parallel Builds**: Multiple targets can be built concurrently
- **Incremental Compilation**: Leverages Cargo's incremental build
- **Tool Caching**: Linker and tool detection results are cached
- **Container Reuse**: Container images are cached and reused

## Future Architecture

Planned additions:

- **Plugin System**: Allow custom build strategies
- **Remote Builders**: Offload builds to remote machines
- **Build Server**: Long-running daemon for faster builds
- **Distributed Caching**: Share artifacts across team

## Next Steps

Dive deeper into specific components:

- [Target Detection](./target-detection.md)
- [Toolchain Management](./toolchain-management.md)
- [Build Strategy](./build-strategy.md)
- [Container Runtime](./container-runtime.md)
