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

### First-Time Crate Publishing
**IMPORTANT**: New crates must be published manually first due to crates.io Trusted Publishing limitations.

If you create a new crate in this workspace:
1. Publish it manually from your local machine first: `cargo publish -p new-crate-name`
2. Configure Trusted Publishing on crates.io for the new crate
3. Future releases will work automatically via GitHub Actions

This is a security limitation - Trusted Publishing tokens cannot create new crates, only update existing ones.

## Commit Messages and Versioning

### IMPORTANT: This Repository Uses Conventional Commits

This repository uses **Conventional Commits** for automatic versioning and changelog generation. Your commit messages directly affect version bumps during releases.

### Commit Format
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Version Impact

| Commit Type | Version Bump | When to Use | Example |
|-------------|--------------|-------------|---------|
| `feat:` | **Minor** (0.1.0 → 0.2.0) | New features or capabilities | `feat: add support for private registries` |
| `fix:` | **Patch** (0.1.0 → 0.1.1) | Bug fixes | `fix: resolve manifest parsing error` |
| `feat!:` or with `BREAKING CHANGE:` | **Major** (0.1.0 → 1.0.0) | Breaking changes | `feat!: redesign configuration format` |
| `chore:` | No bump | Maintenance tasks | `chore: update dependencies` |
| `docs:` | No bump | Documentation only | `docs: improve README examples` |
| `style:` | No bump | Code style changes | `style: format code with rustfmt` |
| `refactor:` | No bump | Code refactoring | `refactor: simplify validation logic` |
| `test:` | No bump | Test changes | `test: add integration tests for install command` |
| `ci:` | No bump | CI/CD changes | `ci: update GitHub Actions workflow` |

### Examples for Common Tasks

```bash
# When adding a new CLI command or feature
git commit -m "feat: add 'canon validate' command for checking manifest syntax"

# When fixing a bug
git commit -m "fix: correctly handle empty dependency arrays in manifest"

# When making breaking changes (RARE - avoid unless necessary)
git commit -m "feat!: change manifest format from v1 to v2

BREAKING CHANGE: Manifest files now require a 'version' field.
Users must migrate existing manifests using 'canon migrate'."

# When updating dependencies or maintenance
git commit -m "chore: update cargo dependencies to latest versions"

# When improving documentation
git commit -m "docs: add examples for using canon with private registries"

# When refactoring without changing functionality
git commit -m "refactor: extract manifest validation into separate module"
```

### Guidelines

1. **Default to `chore:`** for maintenance tasks that don't affect users
2. **Use `feat:`** only for user-visible features
3. **Use `fix:`** for actual bug fixes, not improvements
4. **Avoid `feat!:`** unless absolutely necessary - breaking changes are disruptive
5. **Be descriptive** - your message becomes part of the changelog
6. **Keep the first line under 72 characters**
7. **Use present tense** ("add" not "added")

### Git Identity

**IMPORTANT**: When making commits, use ONLY the system's git identity. Do NOT add:
- Co-authored-by trailers
- Any mention of Claude or AI assistance
- Secondary authors or sign-offs

Simply use the existing git user configuration for all commits. The commit should appear as if it was made directly by the repository owner.

### Special Considerations

- Multiple commits of different types between releases will cause the highest-priority bump (major > minor > patch)
- The release workflow can auto-detect the version bump using the `auto` option
- Commit messages are used to generate changelogs via git-cliff
- Poor commit messages = poor changelogs

## CI/CD

The repository uses GitHub Actions for CI:
- Every push runs: build, test, fmt check, and clippy
- Releases use workflow_dispatch with automatic version detection
- No binary builds - distribution is via `cargo install canon-cli`

### Release Process

Releases are triggered manually via GitHub Actions:
1. Go to Actions → Release workflow
2. Select crate(s) to release
3. Choose version bump (`auto` recommended - uses conventional commits)
4. Optionally enable dry run for testing

The workflow will:
- Analyze commits to determine version bump (if using `auto`)
- Update version numbers automatically
- Generate changelog from commit messages
- Publish to crates.io
- Create GitHub release

See `docs/RELEASE.md` for detailed release documentation.