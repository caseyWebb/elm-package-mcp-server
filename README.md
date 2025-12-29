# Elm Package MCP Server

> [!WARNING]
> **Deprecated:** This MCP server has been replaced by a [Claude Code Skills-based plugin](https://github.com/caseyWebb/elm-claude-plugin) that provides the same functionality without requiring a separate server process.
>
> **To migrate**, invoke the `migrate-to-skills` prompt in Claude Code, or follow these steps:
> 1. `/plugin marketplace add caseyWebb/elm-claude-plugin`
> 2. `/plugin install elm@caseyWebb`
> 3. `claude mcp remove elm-packages`
>
> The new plugin requires `curl` and `jq` to be installed.

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

The easiest way to use this MCP server is via npx (macOS only).

#### Quick Setup with Claude Code

```bash
claude mcp add elm-packages npx @caseywebb/elm-package-mcp-server
```

#### Manual Configuration (.mcp.json)

```json
{
  "mcpServers": {
    "elm-package": {
      "command": "npx",
      "args": ["@caseywebb/elm-package-mcp-server"]
    }
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

## Usage

The server must be run from a directory containing an elm.json file or any subdirectory of an Elm project. It will automatically find the elm.json file by searching up the directory tree.

### Available Tools

The server provides five tools for working with Elm packages. All tools are prefixed with `elm` to help with discoverability when working with Elm language projects.

#### list_installed_packages
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

#### search_packages
Search the Elm package registry for packages matching a query. Uses fuzzy matching on package names and descriptions. Perfect for discovering new packages.

Parameters:
- `query` (required, string): Search query - can be package name, keywords, or description of what you're looking for (e.g., 'json decode', 'http', 'date formatting')
- `already_included` (optional, boolean): Include packages already in elm.json (default: true). Set to false to only show packages not yet installed.

Example response:
```json
{
  "query": "json",
  "results": [
    {
      "name": "elm/json",
      "summary": "Encode and decode JSON values",
      "license": "BSD-3-Clause",
      "version": "1.1.3"
    }
  ],
  "count": 1,
  "excluded_installed": false
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

1. First, use `list_installed_packages` to discover available packages in your project, or use `search_packages` to find new packages:
   - `list_installed_packages` returns all packages with their authors, names, and versions
   - `search_packages` finds packages in the Elm registry matching your query
2. Then, use the returned information to fetch documentation:
   - Call `get_elm_package_readme` with author, name, and version for overview
   - Call `get_elm_package_exports` to see all available functions and types without comments
   - Call `get_elm_package_export_docs` to get detailed documentation for specific items

### Available Resources

#### elm://elm.json
The project's elm.json file, accessible as a resource.

### Available Prompts

The server provides six prompts to help with common Elm development workflows:

#### analyze-dependencies
Analyze your Elm project's dependencies, explaining what each package does and suggesting optimizations. Proactively used when you ask about your elm.json or project structure.

**No parameters required**

#### explore-package
Explore the capabilities of a specific Elm package by examining its exports, modules, and key functions.

**Parameters:**
- `package` (required): Package name in format 'author/name' (e.g., 'elm/core')

**Example:** `/explore-package package=elm/json`

#### find-function
Search for functions across your Elm dependencies that match a specific capability or use case.

**Parameters:**
- `capability` (required): What you want to accomplish (e.g., 'parse JSON', 'map over a list', 'handle HTTP errors')

**Example:** `/find-function capability="parse JSON"`

#### debug-import
Explain what functions and types are available from a specific Elm module import. Useful when you have import errors or questions about available functions from an import.

**Parameters:**
- `module_path` (required): Full module path (e.g., 'List', 'Html.Attributes', 'Json.Decode')

**Example:** `/debug-import module_path=Json.Decode`

#### discover-packages
Discover new Elm packages for a specific need or use case. Proactively used when you describe a problem that might need a new package.

**Parameters:**
- `need` (required): What you need to accomplish (e.g., 'parsing CSV', 'working with dates', 'making HTTP requests')

**Example:** `/discover-packages need="working with dates"`

#### package-comparison
Compare two Elm packages to help choose the best one for a specific use case.

**Parameters:**
- `package1` (required): First package in format 'author/name'
- `package2` (required): Second package in format 'author/name'

**Example:** `/package-comparison package1=elm/json package2=NoRedInk/elm-json-decode-pipeline`

## CLI Options

The server runs in MCP server mode by default. Use the following options to inspect the server's capabilities:

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

## License

Apache-2.0
