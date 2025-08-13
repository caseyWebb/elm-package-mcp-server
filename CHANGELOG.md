# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added
- GitHub Actions workflows for CI/CD
  - Continuous Integration (CI) for testing on multiple platforms
  - Nightly builds with automated releases
  - Tagged release workflow for stable versions
  - Manual release workflow for creating releases from GitHub UI
- Cross-platform binary builds for:
  - macOS (Intel x86_64, Apple Silicon aarch64, and Universal binary)
  - Linux (x86_64 and ARM64)
- Automated dependency updates via Dependabot
- Initial release of elm-package-mcp-server
- MCP server implementation for Elm package documentation
- Three main tools:
  - `list_packages`: Lists all packages from elm.json with option to include indirect dependencies
  - `get_readme`: Fetches the README for any installed Elm package
  - `get_docs`: Fetches full API documentation for any installed Elm package, with optional module filtering
- Resource support for `elm://elm.json` to access the project's elm.json file
- Automatic elm.json discovery by searching up the directory tree
- CLI interface for testing and debugging
- Support for JSON output format
- Comprehensive error handling and reporting
- Documentation and usage examples

### Technical Details
- Built with Rust for performance and small binary size
- Uses rpc-router for JSON-RPC handling
- Async HTTP requests with reqwest
- Cross-platform compatibility
