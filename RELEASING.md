# Release Process

This document describes the process for releasing a new version of elm-package-mcp-server.

## Prerequisites

- Ensure you have push access to the repository
- Ensure all tests are passing on the main branch
- Have the latest main branch checked out locally

## Release Steps

### 1. Update Version Number

Update the version in `Cargo.toml`:

```toml
[package]
name = "elm-package-mcp-server"
version = "X.Y.Z"  # Update this line
```

### 2. Update CHANGELOG

Move all items from the "Unreleased" section to a new version section in `CHANGELOG.md`:

```markdown
## Unreleased

## [X.Y.Z] - YYYY-MM-DD

### Added
- Feature descriptions...

### Changed
- Change descriptions...

### Fixed
- Bug fix descriptions...
```

### 3. Commit Changes

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: prepare release vX.Y.Z"
git push origin main
```

### 4. Create and Push Tag

The tag MUST start with `v` and match the version in Cargo.toml:

```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

### 5. Monitor Release

The GitHub Actions workflow will automatically:

1. **Build binaries** for:
   - Linux x86_64
   - macOS x86_64 (Intel)
   - macOS aarch64 (Apple Silicon)

2. **Create a GitHub release** with:
   - Release notes extracted from CHANGELOG.md
   - All compiled binaries as downloadable assets
   - Installation instructions

3. **Publish to crates.io** (if CARGO_REGISTRY_TOKEN is configured)

You can monitor the progress at:
https://github.com/YOUR_USERNAME/elm-package-mcp-server/actions

### 6. Verify Release

Once the workflow completes:

1. Check the [Releases page](https://github.com/YOUR_USERNAME/elm-package-mcp-server/releases) to ensure:
   - The release is published
   - All binaries are attached
   - Release notes are formatted correctly

2. Test downloading and running a binary:
   ```bash
   # Download and extract (example for Linux)
   curl -L https://github.com/YOUR_USERNAME/elm-package-mcp-server/releases/download/vX.Y.Z/elm-package-mcp-server-linux-x86_64.tar.gz | tar xz
   ./elm-package-mcp-server --version
   ```

3. If published to crates.io, verify at:
   https://crates.io/crates/elm-package-mcp-server

## Troubleshooting

### Release workflow fails

1. Check the [Actions tab](https://github.com/YOUR_USERNAME/elm-package-mcp-server/actions) for error messages
2. Common issues:
   - Version mismatch between tag and Cargo.toml
   - Missing CARGO_REGISTRY_TOKEN secret (only affects crates.io publishing)
   - Build failures on specific platforms

### Need to update a release

If you need to fix something in a release:

1. Delete the tag locally and remotely:
   ```bash
   git tag -d vX.Y.Z
   git push origin :refs/tags/vX.Y.Z
   ```

2. Delete the GitHub release (if created)

3. Fix the issues, commit, and start over from step 4

## Setting up Secrets

To enable publishing to crates.io:

1. Get your API token from https://crates.io/settings/tokens
2. Add it as a repository secret named `CARGO_REGISTRY_TOKEN`:
   - Go to Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `CARGO_REGISTRY_TOKEN`
   - Value: Your crates.io API token

## Version Numbering

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version (X.0.0): Incompatible API changes
- **MINOR** version (0.Y.0): Backwards-compatible functionality additions
- **PATCH** version (0.0.Z): Backwards-compatible bug fixes

## Pre-release Checklist

Before starting the release process:

- [ ] All CI checks are passing
- [ ] Documentation is up to date
- [ ] CHANGELOG.md includes all changes
- [ ] No uncommitted changes in working directory
- [ ] You're on the main branch with latest changes