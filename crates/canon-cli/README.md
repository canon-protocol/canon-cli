# Canon CLI

Command-line tool for Canon protocol specifications.

## Installation

```bash
cargo install canon-cli
```

## Usage

```bash
# Initialize a new Canon repository
canon init

# Install dependencies
canon install

# Add a new dependency
canon add "api.io/openapi@2.0.0"

# Validate specifications
canon validate

# Build canonical artifacts
canon build --sign

# Clean cached specifications
canon clean
```

## Commands

- `init` - Initialize new Canon repository with core dependencies
- `install` - Install dependencies from canon.yml
- `add` - Add a new dependency to canon.yml
- `validate` - Validate specification syntax and structure
- `build` - Generate canonical artifacts, manifest, and signature
- `clean` - Remove Canon artifacts
- `config` - Manage configuration

## Development

This CLI is built on top of the `canon-protocol` library which provides the core types and validation logic.

## License

Apache-2.0