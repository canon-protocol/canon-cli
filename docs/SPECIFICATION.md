# Canon CLI Specification

## Overview

The Canon CLI is a command-line tool for creating and managing Canon Protocol specifications. It provides the reference implementation for working with Canon specifications locally.

## Architecture

This repository is a Cargo workspace containing two crates:
- `canon-protocol` - Core library with types and validation
- `canon-cli` - Command-line interface application

## Commands

### Implemented

#### `canon init`
Initialize a new Canon repository with core dependencies.
- Creates `canon.yml` with default configuration
- Sets up `.canon/` directory for cached specifications
- Adds `.canon/` to `.gitignore`

#### `canon install`
Install dependencies from `canon.yml`.
- Fetches specifications from configured registry
- Caches them in `.canon/specs/`
- Validates downloaded specifications

#### `canon add <uri>`
Add a new dependency to `canon.yml`.
- Parses dependency URI (e.g., "api.io/openapi@2.0.0")
- Updates `canon.yml` with new dependency
- Does not automatically install (run `canon install` after)

#### `canon clean [--all] [--purge]`
Remove cached specifications.
- Default: Removes `.canon/specs/`
- `--all`: Removes entire `.canon/` directory
- `--purge`: Complete uninstall (removes `.canon/` and `canon.yml`)

### Planned

#### `canon validate`
Validate specification syntax and structure.
- Check `canon.yml` format
- Validate against Canon Protocol schema
- Report errors and warnings

#### `canon build`
Generate canonical artifacts from sources.
- Execute transformations
- Generate manifest with file hashes
- Optionally sign with Ed25519

#### `canon config`
Manage CLI configuration.
- Set/get configuration values
- Manage registry settings
- Configure authentication (when needed)

## Installation

```bash
# From crates.io
cargo install canon-cli

# From source
git clone https://github.com/canon-protocol/canon-cli
cd canon-cli
cargo install --path crates/canon-cli
```

## Configuration

The CLI looks for configuration in this order:
1. Command-line flags
2. Environment variables
3. `.canon/config.yml` (future)
4. Global config file (future)

## Registry Interaction

Currently, the CLI interacts with registries using simple HTTP GET requests:
- No authentication required (public specifications only)
- Default registry: `https://spec.farm`
- Specifications stored at: `{registry}/specs/{publisher}/{name}/{version}/`

## Future Enhancements

- Transformation execution (AI-driven, template-based)
- Cryptographic signing and verification
- Registry authentication for private specifications
- Local specification development workflow
- Specification publishing to registries