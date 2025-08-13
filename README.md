# Elm Package MCP Server

[![CI](https://github.com/casey/elm-package-mcp-server/workflows/CI/badge.svg)](https://github.com/casey/elm-package-mcp-server/actions/workflows/ci.yml)
[![Nightly Build](https://github.com/casey/elm-package-mcp-server/workflows/Nightly%20Build/badge.svg)](https://github.com/casey/elm-package-mcp-server/actions/workflows/nightly.yml)
[![GitHub release](https://img.shields.io/github/v/release/casey/elm-package-mcp-server)](https://github.com/casey/elm-package-mcp-server/releases/latest)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

An MCP (Model Context Protocol) server that provides tools for looking up Elm package documentation. This server allows AI assistants to read elm.json files and fetch package documentation from the official Elm package repository.

## Features

- **List Packages**: List all direct and indirect dependencies from elm.json
- **Fetch README**: Get the README content for any installed Elm package
- **Fetch Documentation**: Get the full API documentation for any installed Elm package, including:
  - Module documentation
  - Type definitions (unions and aliases)
  - Function signatures and documentation
  - Binary operators

## Installation

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

## Usage

The server must be run from a directory containing an elm.json file or any subdirectory of an Elm project. It will automatically find the elm.json file by searching up the directory tree.

### Available Tools

#### list_packages
Lists all packages from elm.json.

Parameters:
- `include_indirect` (optional, boolean): Include indirect dependencies (default: false)

Example response:
```json
{
  "packages": [
    {
      "name": "elm/core",
      "version": "1.0.5",
      "type": "direct"
    }
  ],
  "total": 1,
  "direct_count": 1,
  "indirect_count": 0
}
```

#### get_readme
Fetches the README for a specific package.

Parameters:
- `package` (required, string): Package name (e.g., "elm/core")

#### get_docs
Fetches the API documentation for a specific package.

Parameters:
- `package` (required, string): Package name (e.g., "elm/core")
- `module` (optional, string): Filter to a specific module

Example response includes:
- Module names and documentation
- Type definitions (unions and aliases)
- Function signatures with types and documentation
- Binary operators with precedence and associativity

### Available Resources

#### elm://elm.json
The project's elm.json file, accessible as a resource.

## CLI Options

- `--mcp`: Run as an MCP server
- `--tools`: Display available tools
- `--resources`: Display available resources
- `--prompts`: Display available prompts
- `--json`: Output information in JSON format

## Development

This server is built using:
- [rpc-router](https://github.com/jeremychone/rust-rpc-router/) for JSON-RPC routing
- [reqwest](https://github.com/seanmonstar/reqwest) for HTTP requests
- [clap](https://github.com/clap-rs/clap) for CLI argument parsing

## Future Plans

This MCP server is designed to be shipped with a Zed extension, which will be developed in the same repository.

## License

Apache-2.0