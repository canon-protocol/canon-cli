# Canon Protocol Library

Core types and validation for the Canon Protocol specification format.

## Overview

This library provides the fundamental data structures and parsing logic for the Canon Protocol, including:

- **Specification types** - Core data structures for Canon specifications
- **Dependency parsing** - URI parsing and dependency resolution
- **Repository configuration** - Management of Canon repositories
- **Manifest and signatures** - Cryptographic verification structures

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
canon-protocol = "0.1"
```

Then use in your code:

```rust
use canon_protocol::{Dependency, CanonSpecification, CanonRepository};

// Parse a dependency URI
let dep = Dependency::parse("canon-protocol.org/type@1.0.0")?;

// Create a new repository configuration
let repo = CanonRepository::new();
```

## Features

- **Type-safe** - Strongly typed representations of Canon Protocol structures
- **Validation** - Built-in validation for specifications and dependencies
- **Serialization** - Full serde support for YAML and JSON
- **Error handling** - Comprehensive error types with context

## License

Apache-2.0