# Canon CLI Development Guide

## Quick Start

```bash
# Clone and setup
git clone https://github.com/canon-protocol/canon-cli.git
cd canon-cli

# Install tools (recommended)
cargo install just

# Run all checks
just check

# Build and test
just alpha-local
```

## Development Commands

### With Just (Recommended)
```bash
just --list                    # Show all commands
just build                     # Build release binary  
just test                      # Run test suite
just fmt                       # Format code
just lint                      # Run clippy
just check                     # fmt + lint + test
just alpha-local              # Build and test locally
just run -- --help           # Run with arguments
just test-init                # Test the init command
just clean                    # Clean build artifacts
```

### With Make (Alternative)
```bash
make help                     # Show available targets
make build test fmt lint check alpha-local clean
```

## Code Standards

- **Formatting**: `cargo fmt` (enforced by CI)
- **Linting**: `cargo clippy -- -D warnings` (zero warnings allowed)
- **Testing**: All tests must pass
- **Documentation**: Public APIs need docs

## Architecture

```
src/
├── main.rs              # Entry point & CLI setup
├── cli.rs               # Command-line parsing (clap)
├── commands/            # Command implementations
│   ├── init.rs         # canon init (implemented)
│   ├── validate.rs     # canon validate (stub)
│   ├── build.rs        # canon build (stub)
│   ├── clean.rs        # canon clean (stub)
│   └── config.rs       # canon config (stub)
├── core/               # Domain logic
│   └── specification.rs # Canon spec data structures
├── config/             # Configuration management
├── utils/              # Utilities & error handling
    └── error.rs        # Error types with thiserror
```

## Testing

### Quick Test
```bash
# Test the main feature
just test-init

# Or manually
cargo run -- init test-project --author "Test User"
cd test-project && ls -la
# Should see: canon.yml, sources/, .canonignore
```

### Full Test Suite
```bash
just test                     # Run all tests
cargo test -- --nocapture   # Show output
cargo test init              # Test specific module
```

## Release Process

### For Alpha Testing
```bash
git push origin main          # Triggers automatic alpha build
# Check: https://github.com/canon-protocol/canon-cli/releases/tag/alpha
```

### For Stable Release
```bash
# Update version in Cargo.toml first
git tag v0.2.0               # Create version tag
git push origin v0.2.0       # Triggers release build + crates.io publish
```

### Publishing to Crates.io
Stable releases (e.g., `v0.1.0`) are automatically published to crates.io.
Pre-releases (e.g., `v0.1.0-beta.1`) are GitHub-only.

```bash
# Users can then install via:
cargo install canon-cli
```

See [RELEASE.md](./RELEASE.md) for complete release documentation.

## Common Issues

### Build Fails
```bash
cargo clean && cargo build --release
```

### Tests Fail
```bash
cargo test -- --nocapture    # See detailed output
```

### CI Fails
1. Run `just check` locally first
2. Fix any formatting/linting issues
3. Ensure all tests pass

### Need Help?
- Check [RELEASE.md](./RELEASE.md) for release issues
- Look at GitHub Actions logs
- Test locally with `just alpha-local`

## Current Status

- **canon init**: Implemented
- **Other commands**: Stubs only
- **Build system**: Complete with CI/CD
- **Cross-platform**: All major platforms supported

