# Canon CLI

A command-line tool for creating and managing Canon protocol specifications.

## Structure

```
canon-cli/
├── Cargo.toml              # Project configuration and dependencies
├── src/
│   ├── main.rs            # Entry point
│   ├── cli.rs             # Command-line argument parsing with clap
│   ├── commands/          # Command implementations
│   │   ├── mod.rs         # Command routing
│   │   ├── init.rs        # canon init command (implemented)
│   │   ├── validate.rs    # canon validate command (stub)
│   │   ├── build.rs       # canon build command (stub)
│   │   ├── clean.rs       # canon clean command (stub)
│   │   └── config.rs      # canon config commands (stubs)
│   ├── core/              # Core domain logic
│   │   ├── mod.rs         # Module exports
│   │   └── specification.rs # Canon specification data structures
│   ├── config/            # Configuration management (placeholder)
│   │   └── mod.rs
│   └── utils/             # Utilities
│       ├── mod.rs         # Module exports
│       └── error.rs       # Error handling with thiserror
```

## Current Status

- ✅ `canon init` - Create new Canon specifications
- ✅ Basic CLI structure and error handling
- ⏳ Additional commands in development

## Usage

```bash
# Initialize a new Canon specification
canon init my-spec --author "Your Name" --license MIT

# Initialize with template (when implemented)
canon init --template api-spec

# Other commands (stubs for now)
canon validate
canon build
canon clean
canon config list
```

## Installation

### Recommended: Install from Crates.io
```bash
# Install the latest version
cargo install canon-cli

# Update to the latest version
cargo install canon-cli --force

# Install a specific version
cargo install canon-cli@0.1.0
```

### Alternative: Download Pre-built Binary
```bash
# Linux x64
wget https://github.com/canon-protocol/canon-cli/releases/latest/download/canon-linux-x64
chmod +x canon-linux-x64
./canon-linux-x64 --version

# macOS Apple Silicon
wget https://github.com/canon-protocol/canon-cli/releases/latest/download/canon-macos-apple
chmod +x canon-macos-apple
./canon-macos-apple --version

# Windows - download canon-windows.exe from releases page
# Note: You may see a SmartScreen warning - see docs/WINDOWS-SECURITY.md for info
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
- **For releases**: See [docs/RELEASE.md](docs/RELEASE.md) for build and release process

## Contributing

This is an early MVP release. Contributions and feedback are welcome via [GitHub Issues](https://github.com/canon-protocol/canon-cli/issues).
