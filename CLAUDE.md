# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

An MCP (Model Context Protocol) server written in Rust that provides tools for looking up Elm package documentation. The server allows AI assistants to read elm.json files and fetch package documentation from package.elm-lang.org.

## Build & Development Commands

```bash
# Build (development)
cargo build

# Build (release)
cargo build --release

# Run tests
cargo test

# Run end-to-end tests (must run from e2e/ directory)
cd e2e && python3 test.py

# Run the MCP server
./target/debug/elm-package-mcp-server
```

## Key Architecture

### Module Structure

- **src/main.rs**: Entry point. Builds RPC router, handles stdin JSON-RPC requests, processes notifications and method calls. Routes `tools/call` requests to specific tool handlers by extracting the tool name from params.
- **src/mcp/**: MCP protocol implementation
  - **tools.rs**: Registers 4 tools (list_elm_packages, get_elm_package_readme, get_elm_package_exports, get_elm_package_export_docs) and defines their schemas
  - **resources.rs**: Provides elm://elm.json resource
  - **utilities.rs**: MCP lifecycle handlers (initialize, ping, logging, etc.)
  - **types.rs**: JSON-RPC and MCP type definitions
  - **prompts.rs**: Prompt handlers (currently empty)
- **src/elm/**: Elm-specific functionality
  - **reader.rs**: Parses elm.json to extract direct/indirect dependencies
  - **fetcher.rs**: Fetches package data from package.elm-lang.org (README, docs.json) and caches in ~/.elm/
  - **mod.rs**: Defines PackageInfo struct (author, name, version)

### Data Flow

1. MCP client sends JSON-RPC request to stdin
2. main.rs reads line, logs to /tmp/mcp.jsonl
3. For `tools/call` requests, extracts tool name and arguments, transforms into internal RPC request
4. Router dispatches to appropriate handler in tools.rs
5. Handlers use elm::reader to parse local elm.json or elm::fetcher to retrieve remote package data
6. Response serialized to JSON-RPC and written to stdout

### Package Data Fetching

The server fetches package documentation from package.elm-lang.org with the following URL patterns:
- README: `https://package.elm-lang.org/packages/{author}/{name}/{version}/README.md`
- Docs: `https://package.elm-lang.org/packages/{author}/{name}/{version}/docs.json`

Package data is cached in `~/.elm/0.19.1/packages/{author}/{name}/{version}/` to avoid repeated network requests. The fetcher checks this cache directory first before making HTTP requests.

### Elm.json Discovery

The server must run from a directory containing elm.json or a subdirectory of an Elm project. It searches up the directory tree to find elm.json.

## Testing

- Unit tests: Run `cargo test`
- E2E tests: Located in e2e/, uses Python script to send JSON-RPC requests and validate responses
- CI: GitHub Actions workflows in .github/workflows/ (ci.yml runs tests, release.yml builds release binaries)

## Release Process

- Version in Cargo.toml must match git tag
- Use `just check-version TAG` to verify version alignment
- Release builds use aggressive optimization (strip, lto, opt-level="z") for minimal binary size
- Binaries built for Linux x86_64, macOS x86_64, and macOS aarch64
