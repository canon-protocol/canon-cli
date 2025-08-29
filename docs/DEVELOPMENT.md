# Development Guide

## Quick Start

```bash
# Clone the repository
git clone https://github.com/canon-protocol/canon-cli.git
cd canon-cli

# Build the project
cargo build --workspace

# Run tests
cargo test --workspace

# Run the CLI
cargo run -p canon-cli -- --help
```

## Development Workflow

### Building

```bash
# Debug build
cargo build --workspace

# Release build
cargo build --workspace --release
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace -- -D warnings
```

### Using Just

If you have [just](https://github.com/casey/just) installed:

```bash
just --list                    # Show all commands
just build                     # Build release binary  
just test                      # Run test suite
just fmt                       # Format code
just lint                      # Run clippy
just check                     # fmt + lint + test
```

### Using Make

Alternatively, use the Makefile:

```bash
make help                      # Show available targets
make build test fmt lint check
```

## Project Structure

```
canon-cli/
├── crates/
│   ├── canon-protocol/       # Core protocol library
│   │   └── src/
│   │       ├── dependency.rs # URI parsing and version operators
│   │       ├── manifest.rs   # Manifest structure
│   │       ├── signature.rs  # Signature types
│   │       └── specification.rs # Canon spec types
│   │
│   └── canon-cli/            # CLI application
│       └── src/
│           ├── main.rs       # Entry point
│           ├── cli.rs        # Command definitions
│           └── commands/     # Command implementations
│               ├── init.rs   # Initialize project
│               ├── install.rs # Install dependencies
│               ├── add.rs    # Add dependency
│               └── clean.rs  # Clean cache
│
├── Cargo.toml               # Workspace configuration
├── justfile                 # Just commands
└── Makefile                 # Make targets
```

## Testing Commands

```bash
# Test init command
cargo run -p canon-cli -- init

# Test with verbose output
cargo run -p canon-cli -- --verbose init

# Test add command
cargo run -p canon-cli -- add "profiles.org/author@1.0.0"

# Test install
cargo run -p canon-cli -- install
```

## Code Standards

- **Rust Edition**: 2021
- **Formatting**: Use `cargo fmt` (enforced by CI)
- **Linting**: Pass `cargo clippy` with no warnings
- **Tests**: All tests must pass
- **Documentation**: Document public APIs

## Commit Guidelines

Use conventional commits for automatic versioning:

```bash
# Bug fix (patch release)
git commit -m "fix: resolve URI parsing error"

# New feature (minor release)  
git commit -m "feat: add support for version operators"

# Breaking change (major release)
git commit -m "feat!: change manifest format"

# No release
git commit -m "chore: update dependencies"
git commit -m "docs: improve README"
```

## Release Process

See [RELEASE.md](RELEASE.md) for the automated release workflow.