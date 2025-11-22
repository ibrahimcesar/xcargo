---
sidebar_position: 1
---

# Introduction

Welcome to **xcargo** - the Rust cross-compilation tool that makes building for multiple targets effortless.

## What is xcargo?

**xcargo** is a command-line tool designed to simplify cross-compilation in Rust. It automates the entire cross-compilation process by:

- 🎯 **Detecting** target requirements automatically
- 🔧 **Installing** necessary toolchains and dependencies
- 🚀 **Building** for any target with a single command
- 🐳 **Using containers** only when necessary (with embedded runtime)

## Tagline

> **Cross-compilation, zero friction**

## Key Features

### Zero Configuration
Works out of the box for most targets. No complex setup required.

### Auto-Detection
Automatically figures out what toolchains and linkers you need for each target.

### Smart Container Usage
Uses native builds when possible for speed, falls back to containers when needed.

### Fast Builds
Parallel compilation, intelligent caching, and native-first strategy ensure maximum performance.

### Many Targets
Support for Linux, Windows, macOS, mobile (Android/iOS), embedded, and WebAssembly.

### CI/CD Ready
Perfect integration with GitHub Actions, GitLab CI, and other CI/CD platforms.

## Design Philosophy

xcargo follows a simple principle: **cross-compilation should just work**.

- ✅ **Native First**: Use native toolchains when available for maximum speed
- ✅ **Container Fallback**: Automatically switch to containers when needed
- ✅ **Transparent**: Always show what's happening under the hood
- ✅ **Flexible**: Override any behavior when you need custom control

## Status

:::caution Work in Progress
xcargo is currently in **pre-alpha** development. The architecture is being actively designed and implemented.

Current version: `0.1.0-alpha`
:::

## Next Steps

- [Installation](./installation.md) - Install xcargo on your system
- [Quick Start](./quick-start.md) - Get started in minutes
- [Architecture Overview](./architecture/overview.md) - Learn how xcargo works
