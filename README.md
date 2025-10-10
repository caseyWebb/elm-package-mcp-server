# Elm Package MCP Server

> [!CAUTION]
> This is vibe-coded. I barely know Rust. I've read the code, but use at your own risk.

[![CI](https://github.com/caseyWebb/elm-package-mcp-server/workflows/CI/badge.svg)](https://github.com/caseyWebb/elm-package-mcp-server/actions/workflows/ci.yml)
[![Nightly Build](https://github.com/caseyWebb/elm-package-mcp-server/workflows/Nightly%20Build/badge.svg)](https://github.com/caseyWebb/elm-package-mcp-server/actions/workflows/nightly.yml)
[![GitHub release](https://img.shields.io/github/v/release/caseyWebb/elm-package-mcp-server)](https://github.com/caseyWebb/elm-package-mcp-server/releases/latest)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

An MCP (Model Context Protocol) server that provides tools for looking up Elm package documentation. This server allows AI assistants to read elm.json files and fetch package documentation from the official Elm package repository.

## Features

- **List Elm Packages**: List all direct and indirect Elm language dependencies from elm.json
- **Fetch Elm Package README**: Get the README content for any Elm package by specifying author, name, and version
- **Get Elm Package Exports**: Get all exports from Elm package modules with their type signatures but WITHOUT comments (more efficient for exploring available functions)
- **Get Elm Package Export Docs**: Get the documentation comment for a specific export (function, type, or alias) in an Elm package module

## Installation

### Using npx (Recommended)

The easiest way to use this MCP server is via npx (macOS only):

```bash
npx @caseywebb/elm-package-mcp-server --mcp
```

To configure with Claude Desktop:

```json
{
  "mcpServers": {
    "elm-package": {
      "command": "npx",
      "args": ["@caseywebb/elm-package-mcp-server", "--mcp"]
    }
  }
}
```

To configure with Zed:

```json
{
  "elm-package": {
    "command": "npx",
    "args": ["@caseywebb/elm-package-mcp-server", "--mcp"],
    "env": {}
  }
}
```

### From GitHub Releases

The easiest way to install is to download a pre-built binary from the [latest release](https://github.com/caseyWebb/elm-package-mcp-server/releases/latest).

1. Download the appropriate binary for your platform:
   - Linux x86_64: `elm-package-mcp-server-linux-x86_64.tar.gz`
   - macOS x86_64 (Intel): `elm-package-mcp-server-macos-x86_64.tar.gz`
   - macOS aarch64 (Apple Silicon): `elm-package-mcp-server-macos-aarch64.tar.gz`

2. Extract the binary:
   ```bash
   tar xzf elm-package-mcp-server-*.tar.gz
   ```

3. Move the binary to a location in your PATH:
   ```bash
   # System-wide installation
   sudo mv elm-package-mcp-server /usr/local/bin/
   
   # User installation
   mv elm-package-mcp-server ~/.local/bin/
   ```

4. Make sure the binary is executable:
   ```bash
   chmod +x /path/to/elm-package-mcp-server
   ```

### Building from Source

1. Make sure you have Rust installed (https://rustup.rs/)
2. Clone this repository
3. Build the binary:

```bash
cargo build --release
```

The binary will be available at `target/release/elm-package-mcp-server`

## Configuration

### Claude Desktop

To use this MCP server with Claude Desktop, add the following to your Claude Desktop configuration:

1. Open Claude Desktop settings
2. Go to Developer → Edit Config
3. Add the following to the `mcpServers` section:

```json
{
  "mcpServers": {
    "elm-package": {
      "command": "/path/to/elm-package-mcp-server",
      "args": ["--mcp"]
    }
  }
}
```

Replace `/path/to/elm-package-mcp-server` with the actual path to your built binary.

### Zed

- Open the Assistant Panel (<kbd>Cmd</kbd>+<kbd>?</kbd>)
- Click "Add Custom Server..."
- Enter the following in the window that appears:

```json
{
  "elm-package": {
    "command": "/path/to/elm-package-mcp-server",
    "args": ["--mcp"],
    "env": {}
  }
}
```

Replace `/path/to/elm-package-mcp-server` with the actual path to your built binary.

## Usage

The server must be run from a directory containing an elm.json file or any subdirectory of an Elm project. It will automatically find the elm.json file by searching up the directory tree.

### Available Tools

The server provides four tools for working with Elm packages. All tools are prefixed with `elm` to help with discoverability when working with Elm language projects.

#### list_elm_packages
Lists all Elm packages from elm.json file. This tool discovers available Elm language dependencies in your project.

Parameters:
- `include_indirect` (optional, boolean): Include indirect dependencies (default: false)

Example response:
```json
{
  "packages": [
    {
      "author": "elm",
      "name": "core",
      "version": "1.0.5",
      "type": "direct"
    }
  ],
  "total": 1,
  "direct_count": 1,
  "indirect_count": 0
}
```

#### get_elm_package_readme
Fetches the README documentation for a specific Elm language package from package.elm-lang.org.

Parameters:
- `author` (required, string): Package author (e.g., "elm")
- `name` (required, string): Package name (e.g., "core")
- `version` (required, string): Package version (e.g., "1.0.5")

#### get_elm_package_exports
Get all exports from Elm package modules with their type signatures but WITHOUT comments. This is more efficient than get_elm_package_docs when you just need to explore available functions.

Parameters:
- `author` (required, string): Package author (e.g., "elm")
- `name` (required, string): Package name (e.g., "core")
- `version` (required, string): Package version (e.g., "1.0.5")
- `module` (optional, string): Filter to a specific module

Example response includes all exports organized by type (unions, aliases, values, binops) with their type signatures but without documentation comments.

#### get_elm_package_export_docs
Get the documentation comment for a specific export (function, type, or alias) in an Elm package module.

Parameters:
- `author` (required, string): Package author (e.g., "elm")
- `name` (required, string): Package name (e.g., "core")
- `version` (required, string): Package version (e.g., "1.0.5")
- `module` (required, string): Module name (e.g., "List")
- `export_name` (required, string): Name of the export to get comment for (e.g., "map", "Maybe")

Example response:
```json
{
  "author": "elm",
  "name": "core",
  "version": "1.0.5",
  "module": "List",
  "export_name": "map",
  "export_type": "value",
  "type_annotation": "map : (a -> b) -> List a -> List b",
  "comment": "Apply a function to every element of a list..."
}
```

### Workflow Example

1. First, use `list_elm_packages` to discover available packages:
   - This returns all packages with their authors, names, and versions
2. Then, use the returned information to fetch documentation:
   - Call `get_elm_package_readme` with author, name, and version for overview
   - Call `get_elm_package_exports` to see all available functions and types without comments
   - Call `get_elm_package_export_docs` to get detailed documentation for specific items

### Available Resources

#### elm://elm.json
The project's elm.json file, accessible as a resource.

## CLI Options

- `--mcp`: Run as an MCP server for Elm package documentation
- `--tools`: Display available Elm package tools
- `--resources`: Display available resources
- `--prompts`: Display available prompts
- `--json`: Output information in JSON format

## Development

This server is built using:
- [rpc-router](https://github.com/jeremychone/rust-rpc-router/) for JSON-RPC routing
- [reqwest](https://github.com/seanmonstar/reqwest) for HTTP requests
- [clap](https://github.com/clap-rs/clap) for CLI argument parsing

### Building for npm

To build and publish the npm package (macOS only):

```bash
# Prepare binaries for current architecture
just npm-prepare-binaries

# Create npm package tarball
just npm-pack

# Test locally
just npm-test-local

# Publish to npm (requires npm login)
just npm-publish
```

## Future Plans

This MCP server is designed to be shipped with a Zed extension, which will be developed in the same repository.

## License

Apache-2.0
