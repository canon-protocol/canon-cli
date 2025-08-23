# Canon CLI Specification

## Overview

The Canon CLI is a command-line tool that transforms unstructured human input into structured, versioned canonical specifications. Currently in MVP phase with basic functionality implemented.

## Current Implementation Status

### Working Commands
- `canon init` - Initialize a new Canon specification with basic structure
- `canon --help` - Display help information
- `canon --version` - Show version information

### Placeholder Commands (Not Yet Implemented)
- `canon validate` - Will validate canon.yml syntax
- `canon build` - Will generate canonical artifacts
- `canon clean` - Will remove generated files
- `canon config` - Will manage configuration

## Installation

### From Crates.io (Recommended)
```bash
cargo install canon-cli
```

### Pre-built Binaries
Available from [GitHub Releases](https://github.com/canon-protocol/canon-cli/releases) for:
- Linux x64
- macOS Intel & Apple Silicon
- Windows x64

## Project Structure

```
canon-cli/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli.rs               # Command-line argument parsing
│   ├── commands/            # Command implementations
│   ├── core/                # Core domain logic
│   ├── config/              # Configuration management
│   └── utils/               # Utilities and error handling
├── Cargo.toml               # Rust package configuration
└── README.md                # User documentation
```

## Canon Specification Format

When you run `canon init`, it creates a `canon.yml` file:

```yaml
canon: 1.0
type: canon-protocol.org/specification@1.0.0
name: my-project
version: 0.1.0

metadata:
  description: Project description
  author: Your Name
  license: MIT

sources:
  - path: sources/
    type: general
```

## Development

Built with Rust using:
- `clap` for CLI parsing
- `serde` for serialization
- `tokio` for async runtime

### Building from Source
```bash
git clone https://github.com/canon-protocol/canon-cli.git
cd canon-cli
cargo build --release
```

### Running Tests
```bash
cargo test
```

## License

Apache-2.0

## Links

- Repository: https://github.com/canon-protocol/canon-cli
- Homepage: https://canon-protocol.org
- Crate: https://crates.io/crates/canon-cli