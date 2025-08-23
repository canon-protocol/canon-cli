# Canon CLI Development Commands

# Show available commands
default:
    just --list

# Install the binary locally
install:
    cargo install --path .

# Build for release
build:
    cargo build --release

# Run tests
test:
    cargo test

# Format code
fmt:
    cargo fmt

# Run clippy
lint:
    cargo clippy -- -D warnings

# Run all checks (format, lint, test)
check: fmt lint test

# Build for all targets (requires cross)
build-all:
    cargo build --release --target x86_64-unknown-linux-gnu
    cargo build --release --target x86_64-apple-darwin
    cargo build --release --target aarch64-apple-darwin
    cargo build --release --target x86_64-pc-windows-msvc

# Clean build artifacts
clean:
    cargo clean

# Create a new alpha release locally
alpha-local:
    #!/usr/bin/env bash
    set -euo pipefail
    
    echo "Building alpha release locally..."
    cargo build --release
    
    echo "Testing binary..."
    ./target/release/canon --version
    ./target/release/canon --help
    
    echo "Creating test project..."
    mkdir -p /tmp/canon-test
    cd /tmp/canon-test
    ../../../target/release/canon init test-alpha --author "Test User"
    
    echo "Alpha build successful!"
    echo "Binary location: $(pwd)/target/release/canon"

# Run the CLI with args
run *ARGS:
    cargo run -- {{ARGS}}

# Quick test of init command
test-init:
    #!/usr/bin/env bash
    set -euo pipefail
    
    # Create temp directory
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"
    
    echo "Testing canon init in: $TEMP_DIR"
    
    # Run init
    cargo run --manifest-path {{justfile_directory()}}/Cargo.toml -- init test-project --author "Test User" --license MIT
    
    # Verify files were created
    echo "Created files:"
    ls -la
    
    echo "canon.yml contents:"
    cat canon.yml
    
    echo "Test successful!"
    rm -rf "$TEMP_DIR"