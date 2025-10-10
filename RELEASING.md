# Release Process

This document describes the process for releasing a new version of elm-package-mcp-server.

## Prerequisites

- Ensure you have push access to the repository
- Ensure all tests are passing on the main branch
- Have the latest main branch checked out locally
- Ensure NPM_TOKEN secret is configured in GitHub (see "Setting up Secrets" below)

## Release Steps

### 1. Update Version Number

Update the version in both `Cargo.toml` and `package.json`:

```toml
# Cargo.toml
[package]
name = "elm-package-mcp-server"
version = "X.Y.Z"  # Update this line
```

```json
// package.json
{
  "name": "@caseywebb/elm-package-mcp-server",
  "version": "X.Y.Z"  // Update this line
}
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
git add Cargo.toml package.json CHANGELOG.md
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

3. **Publish to npm** with:
   - Both macOS binaries (x86_64 and aarch64) packaged together
   - Published as `@caseywebb/elm-package-mcp-server`
   - Available via `npx @caseywebb/elm-package-mcp-server`

You can monitor the progress at:
https://github.com/caseyWebb/elm-package-mcp-server/actions

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

3. Verify the npm package works:
   ```bash
   # Test with npx
   echo '{"jsonrpc":"2.0","id":1,"method":"ping","params":{}}' | npx @caseywebb/elm-package-mcp-server --mcp
   ```

   Check the npm package at:
   https://www.npmjs.com/package/@caseywebb/elm-package-mcp-server

## Troubleshooting

### Release workflow fails

1. Check the [Actions tab](https://github.com/caseyWebb/elm-package-mcp-server/actions) for error messages
2. Common issues:
   - Version mismatch between tag and package.json/Cargo.toml
   - Missing NPM_TOKEN secret (prevents npm publishing)
   - Build failures on specific platforms
   - Binary extraction or permission issues

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

To enable automatic publishing to npm, you need to set up the NPM_TOKEN secret:

1. Generate an npm access token:
   - Log in to https://www.npmjs.com/
   - Go to Account → Access Tokens
   - Click "Generate New Token" → "Automation"
   - Copy the generated token

2. Add it as a GitHub repository secret named `NPM_TOKEN`:
   - Go to your GitHub repository → Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `NPM_TOKEN`
   - Value: Your npm access token
   - Click "Add secret"

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
- [ ] Version numbers match in both Cargo.toml and package.json
- [ ] NPM_TOKEN secret is configured in GitHub repository

## Nightly Builds

Nightly builds are automatically published to npm when code is pushed to the main branch or on a daily schedule (2 AM UTC).

- Nightly versions follow the format: `X.Y.Z-nightly.YYYYMMDD.SHORT_SHA`
- Install with: `npx @caseywebb/elm-package-mcp-server@nightly`
- Published under the `nightly` tag on npm

You can manually trigger a nightly build from the [Actions tab](https://github.com/caseyWebb/elm-package-mcp-server/actions/workflows/nightly.yml).