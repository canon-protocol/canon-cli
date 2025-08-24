# Claude Code Guidelines for canon-cli

This document provides specific guidance for Claude Code when working on the canon-cli repository.

## Critical: Line Endings in Rust Files

### The Issue
When creating or modifying Rust source files (`.rs`), rustfmt REQUIRES that every file ends with exactly one newline character. This is a consistent issue that causes CI failures.

### Root Cause
The file writing mechanism sometimes strips or doesn't properly include the final newline character at the end of files. This happens because:
- Text editors and file APIs handle "end of file" differently
- Some systems treat the final newline as optional
- The Write tool might be trimming trailing whitespace

### The Solution
**ALWAYS ensure Rust files end with a newline character.**

When writing any `.rs` file:
1. Include a final newline in the content
2. Do NOT add extra blank lines at the end
3. The last character in the file should be `\n` (newline)

### Example - CORRECT:
```rust
fn main() {
    println!("Hello, world!");
}
```
(Note: There IS a newline after the closing brace, even though it's not visible)

### Example - INCORRECT:
```rust
fn main() {
    println!("Hello, world!");
}```
(Note: No newline after the closing brace)

### Verification
After writing any Rust file, if you see CI failures with messages like:
- "Diff in ... No newline at end of file"
- Rustfmt showing a `+` at the end of the file

This means the final newline is missing. Add it by ensuring the file content ends with a newline character.

## Repository Structure

This is a Cargo workspace with two crates:
- `crates/canon-protocol/` - Core library with protocol types
- `crates/canon-cli/` - CLI application

## Testing Commands

Always run these before committing:
```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
```

## Important Patterns

### Adding Dependencies
- Library dependencies go in `crates/canon-protocol/Cargo.toml`
- CLI-specific dependencies go in `crates/canon-cli/Cargo.toml`
- Workspace-wide dependency versions can be defined in the root `Cargo.toml`

### Error Handling
- Protocol library uses `ProtocolError` and `ProtocolResult`
- CLI uses `CanonError` and `CanonResult` which wrap protocol errors

### Publishing Order
When releasing:
1. First publish `canon-protocol` (the library)
2. Wait for crates.io indexing
3. Then publish `canon-cli` (which depends on canon-protocol)

## CI/CD

The repository uses GitHub Actions for CI:
- Every push runs: build, test, fmt check, and clippy
- Release tags (v*) trigger publishing to crates.io
- No binary builds - distribution is via `cargo install canon-cli`