.PHONY: help install build test fmt lint check clean alpha-local run

# Default target
help:
	@echo "Available targets:"
	@echo "  install     - Install the binary locally"
	@echo "  build       - Build for release"
	@echo "  test        - Run tests"
	@echo "  fmt         - Format code"
	@echo "  lint        - Run clippy"
	@echo "  check       - Run all checks (format, lint, test)"
	@echo "  clean       - Clean build artifacts"
	@echo "  alpha-local - Build and test alpha locally"

install:
	cargo install --path .

build:
	cargo build --release

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check: fmt lint test

clean:
	cargo clean

alpha-local: build
	@echo "Testing binary..."
	@./target/release/canon --version
	@./target/release/canon --help
	@echo "Alpha build successful!"
	@echo "Binary location: $(PWD)/target/release/canon"

run:
	cargo run -- $(ARGS)