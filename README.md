# Canon CLI

A command-line tool for creating and managing Canon protocol specifications.

## Project Structure

This is a Cargo workspace containing two crates:

```
canon-cli/
├── Cargo.toml                    # Workspace configuration
├── crates/
│   ├── canon-protocol/           # Core protocol library
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs           # Library exports
│   │   │   ├── specification.rs # Canon specification types
│   │   │   ├── dependency.rs    # Dependency parsing
│   │   │   ├── repository.rs    # Repository configuration
│   │   │   ├── manifest.rs      # Manifest types
│   │   │   ├── signature.rs     # Signature types
│   │   │   └── error.rs         # Error types
│   │   └── README.md
│   │
│   └── canon-cli/                # CLI application
│       ├── Cargo.toml
│       ├── src/
│       │   ├── main.rs          # Entry point
│       │   ├── cli.rs           # Command-line parsing
│       │   ├── commands/        # Command implementations
│       │   ├── config/          # Configuration management
│       │   ├── core/            # Core re-exports
│       │   └── utils/           # Utilities
│       └── README.md
└── README.md                     # This file
```

## Crates

- **`canon-protocol`** - Core library with Canon Protocol types and validation (v0.1.0)
- **`canon-cli`** - Command-line interface tool (v0.2.2)

## Current Status

- ✅ `canon init` - Initialize Canon repository with core dependencies
- ✅ `canon install` - Install dependencies from registry
- ✅ `canon add` - Add new dependencies to canon.yml
- ✅ `canon clean` - Remove cached specifications with various options
- ✅ Dependency management system
- ✅ Basic CLI structure and error handling
- ⏳ Additional commands in development

## Usage

```bash
# Initialize a new Canon repository with core dependencies
canon init

# Install dependencies from canon.yml
canon install

# Add a new dependency
canon add "api.io/openapi@2.0.0"

# Clean cached specifications
canon clean                     # Remove .canon/specs/
canon clean --all              # Remove entire .canon/
canon clean --purge            # Remove .canon/ and canon.yml (complete uninstall)

# Other commands (stubs for now)
canon validate
canon build
canon config list
```

## Installation

### Install from Crates.io
```bash
# Install the latest version
cargo install canon-cli

# Update to the latest version
cargo install canon-cli --force

# Install a specific version
cargo install canon-cli@0.2.2
```

### Build from Source
```bash
git clone https://github.com/canon-protocol/canon-cli.git
cd canon-cli
cargo build --release
./target/release/canon --version
```

## Development

- **For developers**: See [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) for setup and workflow
- **For releases**: See [docs/RELEASE.md](docs/RELEASE.md) for the release process

## Contributing

This is an early MVP release. Contributions and feedback are welcome via [GitHub Issues](https://github.com/canon-protocol/canon-cli/issues).
