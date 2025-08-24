# Canon CLI Release Guide

## Overview

Canon CLI uses a professional, automated release process with GitHub Actions. The repository contains two crates in a Cargo workspace that can be released independently:

- `canon-protocol` - Core protocol library
- `canon-cli` - Command-line interface application

## Key Features

- **Automated Version Bumping**: No manual Cargo.toml edits required
- **Automatic Version Detection**: Can analyze commits to determine version bump
- **Independent Releases**: Release crates separately or together
- **Dry Run Mode**: Test the release process without publishing
- **Trusted Publishing**: Uses crates.io OIDC authentication (no tokens needed)
- **Automatic Dependency Updates**: CLI's dependency on protocol is updated automatically
- **Professional Release Notes**: Auto-generated changelogs with installation instructions
- **Conventional Commits Support**: Automatic changelog generation from commit messages

## Release Process

### Quick Start

To release a new version:

1. Go to [Actions → Release workflow](https://github.com/canon-protocol/canon-cli/actions/workflows/release.yml)
2. Click "Run workflow"
3. Select options:
   - **Crate**: Choose which crate(s) to release
   - **Version bump**: Select patch, minor, or major
   - **Dry run**: Test without publishing (optional)
4. Click "Run workflow" to start

### Release Options

#### Crate Selection
- `canon-protocol` - Release only the protocol library
- `canon-cli` - Release only the CLI application
- `both` - Release both crates with the same version bump

#### Version Bump Types
- `auto` - Automatically detect based on conventional commits (recommended)
- `patch` - Bug fixes and minor updates (0.1.0 → 0.1.1)
- `minor` - New features, backward compatible (0.1.0 → 0.2.0)
- `major` - Breaking changes (0.1.0 → 1.0.0)

#### Dry Run Mode
Enable this to test the release process without:
- Publishing to crates.io
- Creating git commits
- Creating GitHub releases

Perfect for validating changes before an actual release.

## What Happens During Release

### 1. Prepare Release
- Automatically bumps version numbers using `cargo-edit`
- Updates canon-cli's dependency on canon-protocol if both are released
- Runs full test suite with new versions
- Commits version changes to main branch

### 2. Publish to crates.io
- Publishes canon-protocol first (if selected)
- Waits for crates.io indexing
- Publishes canon-cli (if selected)
- Uses Trusted Publishing - no tokens required!

### 3. Create GitHub Release
- Creates a tagged release with auto-generated notes
- Includes installation instructions
- Shows version changes for each crate
- Links to crates.io pages

### 4. Summary Report
- Provides detailed status of all operations
- Shows which crates were published
- Reports any failures clearly

## Release Scenarios

### Releasing Both Crates Together
Perfect for coordinated releases with related changes:
```
Crate: both
Version bump: minor
Result: Both crates get same version bump
```

### Releasing Protocol Only
When only the core library has changes:
```
Crate: canon-protocol
Version bump: patch
Result: Only protocol is released
```

### Releasing CLI Only
For CLI-specific improvements:
```
Crate: canon-cli
Version bump: patch
Result: Only CLI is released
```

## Trusted Publishing Setup

The workflow uses crates.io Trusted Publishing (OIDC authentication). To configure:

1. Go to crates.io → Your crate → Settings → Trusted Publishing
2. Add configuration:
   - Repository owner: `canon-protocol`
   - Repository name: `canon-cli`
   - Workflow filename: `release.yml`
   - Environment: (leave empty)
3. No CARGO_REGISTRY_TOKEN needed!

## Installation Methods

Users can install the released packages:

### CLI Installation
```bash
# Latest version
cargo install canon-cli

# Specific version
cargo install canon-cli@0.2.3
```

### Library Usage
```toml
[dependencies]
canon-protocol = "0.1"
```

## Local Development

### Building
```bash
cargo build --workspace
```

### Testing
```bash
cargo test --workspace
```

### Formatting
```bash
cargo fmt --all -- --check
```

### Linting
```bash
cargo clippy --workspace -- -D warnings
```

### Running CLI
```bash
cargo run -p canon-cli -- --help
```

## Manual Publishing (Emergency Only)

If the automated workflow fails, you can publish manually:

```bash
# 1. Update versions in Cargo.toml files manually
# 2. Commit and push changes
# 3. Publish in order:
cargo publish -p canon-protocol
# Wait for indexing...
cargo publish -p canon-cli
# 4. Create GitHub release manually
```

## Troubleshooting

### Workflow Fails at Version Bump
- Check that cargo-edit is installed
- Verify Cargo.toml files are valid

### Publishing Fails
- Ensure crates.io Trusted Publishing is configured
- Check that versions don't already exist
- Verify canon-protocol is indexed before canon-cli publishes

### Tests Fail After Version Bump
- The workflow runs tests after bumping versions
- Fix any issues and restart the workflow

## Conventional Commits

The release workflow supports automatic version detection based on [Conventional Commits](https://www.conventionalcommits.org/). This allows the workflow to automatically determine the appropriate version bump based on your commit messages.

### Commit Message Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Version Bump Rules

| Commit Type | Version Bump | Example |
|-------------|--------------|---------|
| `feat:` | Minor | `feat: add support for YAML configs` |
| `fix:` | Patch | `fix: resolve parsing error in manifest` |
| `feat!:` or `BREAKING CHANGE:` | Major | `feat!: redesign configuration format` |
| `chore:`, `docs:`, `style:`, `refactor:`, `test:` | None | `chore: update dependencies` |

### Examples

```bash
# Patch release (bug fix)
git commit -m "fix: correct validation logic for dependencies"

# Minor release (new feature)
git commit -m "feat: add support for private registries"

# Major release (breaking change)
git commit -m "feat!: change manifest format to v2

BREAKING CHANGE: The manifest format has been completely redesigned.
Old manifests will need to be migrated using the canon migrate command."

# No release (maintenance)
git commit -m "chore: update CI configuration"
git commit -m "docs: improve README examples"
```

### Auto-Version Detection

When you select `auto` for version bump, the workflow will:

1. Analyze all commits since the last release tag
2. Identify the highest-priority change type:
   - Breaking changes → Major bump
   - New features → Minor bump
   - Bug fixes → Patch bump
   - Other changes → Patch bump (default)
3. Apply the appropriate version bump

This ensures consistent versioning based on the actual changes in your codebase.

## Best Practices

1. **Use conventional commits** for automatic version detection
2. **Always use dry run first** for major releases
3. **Release protocol before CLI** when both have changes
4. **Write descriptive commit messages** - they appear in changelogs
5. **Tag releases consistently** using the generated tags
6. **Review auto-detected versions** before publishing

## Version History

The workflow automatically creates tags:
- `v0.2.3` - CLI releases
- `protocol-v0.1.2` - Protocol-only releases
- Both use the CLI version tag when released together

## Support

For issues with the release process:
1. Check the workflow run logs in GitHub Actions
2. Verify crates.io Trusted Publishing configuration
3. Open an issue in the repository if problems persist