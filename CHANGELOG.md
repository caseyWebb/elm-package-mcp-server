# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed
- **Breaking**: Server now runs in MCP mode by default without requiring `--mcp` flag
  - Remove `--mcp` from your configuration (Claude Desktop, Zed, etc.)
  - Use `--tools`, `--resources`, or `--prompts` flags to inspect server capabilities
  - Simply run the binary directly to start the MCP server

## [0.3.0] - 2025-10-10

### Added
- Package search tool (`search_packages`) that searches the Elm package registry using fuzzy matching
- `discover-packages` prompt for finding packages based on specific needs
- Documentation for all 6 available prompts in README

### Changed
- **Breaking**: Renamed `list_elm_packages` tool to `list_installed_packages` for clarity
- Updated prompt workflows to leverage package search functionality
- Enhanced prompt descriptions to use search capabilities proactively

### Dependencies
- Added `nucleo-matcher` dependency for fuzzy search functionality
- Re-added `reqwest` dependency for fetching package search index

## [0.2.3] - 2025-10-10

### Added
- Linux x86_64 support for npm package distribution

## [0.2.2] - 2025-10-10

### Fixed
- Fixed YAML syntax error in CI workflow

## [0.2.1] - 2025-10-10

### Changed
- Package documentation is now read from local `~/.elm/` cache instead of fetching from package.elm-lang.org (b9eb870)
- Removed `reqwest` dependency as HTTP requests are no longer needed (b9eb870)
- Tools now require packages to be installed locally via `elm install` (b9eb870)

## [0.2.0] - 2025-08-15

### Changed
- **Breaking**: Renamed tools for better discoverability:
  - `list_packages` → `list_elm_packages`
  - `get_readme` → `get_elm_package_readme`
- **Breaking**: Removed `get_docs`
- **Breaking**: `get_elm_package_readme` and `get_elm_package_docs` tools now take individual parameters (author, name, version) instead of a combined package name
  - This provides a cleaner API where the LLM first calls `list_elm_packages` to discover available packages
  - Then calls `get_elm_package_readme` or `get_elm_package_docs` with the specific author, name, and version information
- Added: `get_elm_package_exports` and `get_elm_package_export_docs` tools
- Improved tool descriptions with more detailed information about Elm language packages
- Added server instructions to help LLMs understand the workflow
- Updated package description and keywords for better discoverability
- Updated GitHub Actions workflows to use v4 of artifact and cache actions (v3 will be deprecated January 30, 2025)
- Added end-to-end test

### Fixed
- Fixed all clippy warnings and errors for cleaner, more idiomatic Rust code
- Fixed signal handler that was incorrectly using an infinite loop
- Removed redundant file operation flags
- Updated format strings to use inline variables

## [0.1.0] - 2025-08-13

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
