# Canon CLI Release Guide

## Build System

GitHub Actions workflows:
- **CI** - Tests and linting on every commit
- **Alpha** - Automatic builds from `main`
- **Release** - Tagged releases with binaries


## Quick Reference

### Install from Crates.io
```bash
# Install latest stable version
cargo install canon-cli

# Update to latest version
cargo install canon-cli --force
```

### Get Latest Alpha Binary
```bash
# Download latest alpha build (auto-updated on every main push)
wget https://github.com/canon-protocol/canon-cli/releases/download/alpha/canon-linux-x64
chmod +x canon-linux-x64
./canon-linux-x64 --version
```

### Create Alpha Release
```bash
# Option 1: Automatic (just push to main)
git push origin main

# Option 2: Manual trigger
# Go to GitHub → Actions → "Alpha Release" → "Run workflow"
```

### Create Stable Release
```bash
# Create and push tag (triggers GitHub release + crates.io publish)
git tag v0.2.0
git push origin v0.2.0

# For pre-release (GitHub only, not published to crates.io)
git tag v0.2.0-beta.1
git push origin v0.2.0-beta.1
```

### Local Development
```bash
# Build and test locally (preferred)
just alpha-local

# Or with make
make alpha-local

# Or manually
cargo build --release && ./target/release/canon --version
```

## GitHub Actions Workflows

### 1. CI Workflow (`.github/workflows/ci.yml`)
**Trigger:** Every push to `main`, every PR
**Purpose:** Quality assurance and basic validation

**What it does:**
- Runs `cargo test --verbose`
- Checks code formatting with `cargo fmt -- --check`
- Runs linting with `cargo clippy -- -D warnings`
- Validates build on Ubuntu latest
- Uses Rust cache for faster builds

**When it runs:**
- ✅ Every commit to main branch
- ✅ Every pull request
- ✅ Manual trigger available

### 2. Alpha Release Workflow (`.github/workflows/alpha.yml`)
**Trigger:** Push to `main`, manual dispatch
**Purpose:** Bleeding-edge builds for testing

**What it builds:**
- Linux x64 (`canon-linux-x64`)
- macOS Intel (`canon-macos-intel`)
- macOS Apple Silicon (`canon-macos-apple`)
- Windows x64 (`canon-windows.exe`)

**Special behavior:**
- Overwrites previous alpha release
- Tagged as `alpha` (not versioned)
- Marked as pre-release
- 30-day artifact retention
- Includes commit hash in release notes

### 3. Release Workflow (`.github/workflows/release.yml`)
**Trigger:** Git tags matching `v*`, manual dispatch
**Purpose:** Official releases with full cross-platform support

**What it does:**
1. **Publishes to Crates.io** (for stable versions only)
2. **Builds binaries** for all platforms:
   - Linux x64 + ARM64
   - macOS Intel + Apple Silicon  
   - Windows x64
3. **Creates GitHub Release** with:
   - SHA256 checksums for all binaries
   - Stripped binaries for smaller size
   - Installation instructions

**Features:**
- Automatic crates.io publish for non-prerelease versions
- Automatic pre-release detection (`-alpha`, `-beta`, `-rc`)
- Generated release notes
- Cross-compilation for ARM64 Linux

## Versioning

Semantic Versioning:

### Version Format
```
v<MAJOR>.<MINOR>.<PATCH>[-<PRERELEASE>][+<BUILD>]
```

### Examples
```bash
v0.1.0           # Stable release
v0.1.0-alpha.1   # Alpha pre-release
v0.1.0-beta.2    # Beta pre-release  
v0.1.0-rc.1      # Release candidate
v1.0.0           # Major stable release
```

### Version Bump Guidelines
- **MAJOR**: Breaking changes to CLI interface or behavior
- **MINOR**: New features, new commands, backwards-compatible
- **PATCH**: Bug fixes, documentation, internal improvements

### Pre-release Types
- **alpha**: Early development, may be unstable
- **beta**: Feature-complete, needs testing
- **rc**: Release candidate, stable for production testing

## Release Types

### Alpha Releases (Automatic)
**When:** Every push to `main`
**Who:** Developers, early testers
**Stability:** Unstable, latest features

```bash
# Automatically created, no manual action needed
# Available at: /releases/tag/alpha
# Filename: canon-{platform}
```

**Use cases:**
- Testing latest features
- Bug reproduction
- Development validation

### Stable Releases (Manual)
**When:** Ready for production
**Who:** End users, production deployments  
**Stability:** Stable, well-tested

```bash
# Create stable release
git tag v0.2.0
git push origin v0.2.0
```

**Includes:**
- Full cross-platform binaries
- Release notes
- Installation instructions
- SHA256 checksums

### Pre-releases (Manual)
**When:** Testing before stable
**Who:** Beta testers, CI/CD systems
**Stability:** Mostly stable, needs validation

```bash
# Create pre-release
git tag v0.2.0-beta.1
git push origin v0.2.0-beta.1
```

## Local Development

### Prerequisites
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install just (optional but recommended)
cargo install just

# Clone repository
git clone https://github.com/canon-protocol/canon-cli.git
cd canon-cli
```

### Development Commands

#### Using Just (Recommended)
```bash
just --list                    # Show all available commands
just build                     # Build for release
just test                      # Run tests  
just check                     # Run fmt + lint + test
just alpha-local              # Build and test alpha locally
just run -- init my-project  # Run with arguments
just test-init                # Quick test of init command
```

#### Using Make (Traditional)
```bash
make help                     # Show available targets
make build                    # Build for release
make test                     # Run tests
make check                    # Run all checks
make alpha-local             # Build and test locally
```

#### Manual Commands
```bash
cargo build --release        # Build optimized binary
cargo test                   # Run test suite
cargo fmt                    # Format code
cargo clippy -- -D warnings  # Lint code
```

### Testing Your Build
```bash
# Build and test the init command
just test-init

# Or manually
cargo run -- init test-project --author "Your Name"
ls test-project/              # Should show canon.yml, sources/, .canonignore
```

## Cross-Platform Building

### Supported Platforms
| Platform | Target | Binary Name | Notes |
|----------|--------|-------------|-------|
| Linux x64 | `x86_64-unknown-linux-gnu` | `canon-linux-x64` | Primary development |
| Linux ARM64 | `aarch64-unknown-linux-gnu` | `canon-linux-arm64` | Cross-compiled |
| macOS Intel | `x86_64-apple-darwin` | `canon-macos-intel` | Native build |
| macOS Apple | `aarch64-apple-darwin` | `canon-macos-apple` | Native build |
| Windows x64 | `x86_64-pc-windows-msvc` | `canon-windows.exe` | Native build |

### Build Matrix
Our GitHub Actions use a strategic build matrix:

- **Alpha builds**: Skip ARM64 Linux for faster iteration
- **Release builds**: Include all platforms for complete coverage
- **CI builds**: Ubuntu only for speed

### Local Cross-Platform Building
```bash
# Install targets (one-time setup)
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-msvc

# Build for specific target
cargo build --release --target x86_64-unknown-linux-gnu

# Build for all targets (requires cross)
cargo install cross
just build-all  # If using just
```

## Troubleshooting

### Common Issues

#### Build Failures
```bash
# Clear cache and rebuild
cargo clean
cargo build --release

# Check for formatting issues
cargo fmt -- --check

# Check for linting issues  
cargo clippy -- -D warnings
```

#### Missing Dependencies
```bash
# Update Rust toolchain
rustup update

# Update dependencies
cargo update
```

#### Cross-compilation Issues
```bash
# For ARM64 Linux (on Ubuntu)
sudo apt-get update
sudo apt-get install -y gcc-aarch64-linux-gnu

# Set linker
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
```

#### GitHub Actions Failures
1. Check the Actions tab in GitHub
2. Look for red X marks on commits
3. Click into failed jobs for detailed logs
4. Common fixes:
   - Formatting: `cargo fmt`  
   - Linting: Fix clippy warnings
   - Tests: Ensure all tests pass locally

### Release Issues

#### Alpha Release Not Updating
- Check if the push actually triggered the workflow
- Go to Actions tab → Alpha Release → Check recent runs
- Manual trigger: Actions → Alpha Release → "Run workflow"

#### Tag Release Failed  
- Ensure tag follows `v*` pattern: `v0.1.0`, not `0.1.0`
- Check if tag already exists: `git tag -l`
- Verify push included the tag: `git push origin v0.1.0`

#### Binary Not Working
- Check if binary has execute permissions: `chmod +x canon-linux-x64`
- Verify architecture matches your system
- Try the verbose flag: `./canon-linux-x64 --verbose`

## Manual Procedures

### Creating a New Release

#### 1. Prepare Release
```bash
# Ensure main branch is ready
git checkout main
git pull origin main

# Run all checks
just check  # or make check

# Test locally  
just alpha-local
```

#### 2. Update Version (if needed)
```bash
# Edit Cargo.toml version
# Update CHANGELOG.md or release notes
git add .
git commit -m "Bump version to v0.2.0"
git push origin main
```

#### 3. Create and Push Tag
```bash
# Create tag
git tag v0.2.0

# Push tag (triggers release + crates.io publish)
git push origin v0.2.0
```

#### 4. Verify Release
- Check crates.io page: https://crates.io/crates/canon-cli
- Go to GitHub Releases page
- Check all binaries are present
- Test installation: `cargo install canon-cli`
- Update any documentation links

### Publishing to Crates.io

#### Automatic Publishing (Recommended)
When you push a tag matching `v*` without pre-release suffix, the release workflow automatically publishes to crates.io.

#### Manual Publishing
```bash
# First-time setup (store token locally)
cargo login <your-crates-io-token>

# Publish to crates.io
cargo publish

# Dry run (test without publishing)
cargo publish --dry-run
```

#### Required for Crates.io
- Valid `Cargo.toml` metadata (name, version, description, license, etc.)
- README.md file (becomes crate documentation)
- Unique version number (can't republish same version)

### Emergency Procedures

#### Rollback a Release
```bash
# Delete the tag locally and remotely
git tag -d v0.2.0
git push origin :refs/tags/v0.2.0

# Delete the GitHub release manually in web interface
# Create new corrected release
```

#### Fix Alpha Build
```bash
# If alpha is broken, push a fix to main
git push origin main

# Or manually trigger new alpha
# GitHub → Actions → Alpha Release → Run workflow
```

### Monitoring

#### Check Build Status
- GitHub Actions tab shows all workflow runs
- Green checkmark = success
- Red X = failure
- Yellow circle = in progress

#### Release Metrics
- Download counts visible on releases page
- Binary sizes listed for each release
- Release dates and Git SHAs tracked


## Quick Help

- **Alpha build**: Push to main or check `/releases/tag/alpha`
- **Release**: `git tag v0.x.0 && git push origin v0.x.0`
- **Local testing**: `just alpha-local` or `make alpha-local`
- **CI failing**: Run `just check` locally first