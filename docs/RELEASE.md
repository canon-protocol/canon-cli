# Canon CLI Release Guide

## Overview

Canon CLI uses a simple, clean release process focused on publishing to crates.io. The repository contains two crates in a workspace:
- `canon-protocol` - Core library (published first)
- `canon-cli` - CLI application (depends on canon-protocol)

## GitHub Actions Workflows

- **CI** - Runs tests, formatting, and clippy on every push/PR
- **Release** - Publishes to crates.io when a version tag is pushed

## Release Process

### 1. Update Version Numbers

Update version in both crates as needed:
```bash
# Update crates/canon-protocol/Cargo.toml if protocol changes
# Update crates/canon-cli/Cargo.toml for CLI changes
```

### 2. Create and Push Tag

```bash
# For stable releases (publishes to crates.io)
git tag v0.3.0
git push origin v0.3.0

# For pre-releases (GitHub release only, no crates.io)
git tag v0.3.0-beta.1
git push origin v0.3.0-beta.1
```

### 3. Workflow Automatically:
1. Runs full test suite
2. Publishes `canon-protocol` to crates.io (if stable)
3. Waits for indexing
4. Publishes `canon-cli` to crates.io (if stable)
5. Creates GitHub release with notes

## Installation

Users install from crates.io:
```bash
# Install CLI
cargo install canon-cli

# Or add library to project
[dependencies]
canon-protocol = "0.1"
```

## Local Development

```bash
# Build everything
cargo build --workspace

# Run tests
cargo test --workspace

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace -- -D warnings

# Run the CLI
cargo run -p canon-cli -- --help
```

## Publishing Manually (if needed)

```bash
# Publish in order (protocol first, then CLI)
cargo publish -p canon-protocol
# Wait for crates.io to index...
cargo publish -p canon-cli
```

## Notes

- Uses crates.io Trusted Publishing (no token needed in CI)
- Only stable versions (v1.2.3) publish to crates.io
- Pre-release versions (v1.2.3-beta) only create GitHub releases
- Both crates can share the same version tag