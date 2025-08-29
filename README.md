# Canon CLI

Command-line tool for Canon Protocol - a minimal, extensible standard for defining and publishing typed specifications.

## Canon Protocol

Canon Protocol provides a single meta-type that enables the creation of any specification type, combined with DNS-based publisher verification and semantic versioning. This allows communities to build rich ecosystems of interoperable specifications without central coordination.

## Installation

```bash
# Install from crates.io
cargo install canon-cli

# Build from source
git clone https://github.com/canon-protocol/canon-cli.git
cd canon-cli
cargo build --release
./target/release/canon --version
```

## Quick Start

```bash
# Initialize a new Canon project
canon init

# Install dependencies from canon.yml
canon install

# Add a new dependency
canon add "profiles.org/author@1.0.0"

# Clean cached specifications
canon clean
```

## Commands

### `canon init`
Initialize a new Canon Protocol project. This command:
- Downloads the `canon-protocol.org/project@1.0.0` type
- Downloads the `canon-protocol.org/type@1.0.0` meta-type
- Creates a `canon.yml` file with Canon Protocol format
- Sets up `.canon/` directory for dependencies

### `canon install`
Install all dependencies listed in your `canon.yml` file. Specifications are fetched from `https://canon.canon-protocol.org/`.

### `canon add <uri>`
Add a new dependency to your project. Accepts URIs in the format:
- `publisher/id@version` - Exact version
- `publisher/id@^1.0.0` - Compatible versions (in schemas only)
- `publisher/id@~1.0.0` - Patch versions (in schemas only)

### `canon clean`
Remove cached specifications:
- `canon clean` - Remove `.canon/` (all cached dependencies)
- `canon clean --all` - Remove entire `.canon/` (same as default)
- `canon clean --purge` - Remove `.canon/` and `canon.yml` (complete uninstall)

## Project Structure

```
my-project/
├── canon.yml          # Your Canon specification
└── .canon/            # Cached dependencies
    └── publisher/
        └── id/
            └── version/
                ├── canon.yml
                ├── canon-manifest.yml
                └── canon-signature.yml
```

## Example canon.yml

```yaml
canon: 1.0
type: canon-protocol.org/project@1.0.0
metadata:
  id: my-project
  version: 0.1.0
  publisher: example.com
  title: My Canon Project
  description: A project using Canon Protocol

dependencies:
  - canon-protocol.org/type@1.0.0
  - profiles.org/author@1.0.0
```

## Creating Types

Types are created using the meta-type. Example:

```yaml
canon: 1.0
type: canon-protocol.org/type@1.0.0
metadata:
  id: blog-post
  version: 1.0.0
  publisher: content.org
  title: Blog Post Type

schema:
  title:
    type: string
    required: true
    description: Post title
  
  author:
    type: ref
    uri: profiles.org/author@^1.0.0
    required: true
  
  content:
    type: string
    required: true
```

## Development

```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Format
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings
```

## Release Process

See [`docs/RELEASE.md`](docs/RELEASE.md) for the automated release workflow using GitHub Actions.

## Contributing

Contributions are welcome! Please open issues and pull requests on [GitHub](https://github.com/canon-protocol/canon-cli).

## License

Apache-2.0