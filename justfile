# Test basic ping
ping:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "ping" }' | ./target/debug/elm-package-mcp-server --mcp

# List available prompts (currently empty)
prompts-list:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "prompts/list" }' | ./target/debug/elm-package-mcp-server --mcp

# List available tools
tools-list:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/list" }' | ./target/debug/elm-package-mcp-server --mcp

# List available resources
resources-list:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "resources/list" }' | ./target/debug/elm-package-mcp-server --mcp

# Read elm.json resource
read-elm-json:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "resources/read", "params": { "uri": "elm://elm.json" } }' | ./target/debug/elm-package-mcp-server --mcp

# List all packages (direct only)
list-packages:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "list_elm_packages", "arguments": {} } }' | ./target/debug/elm-package-mcp-server --mcp

# List all packages (including indirect)
list-packages-all:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "list_elm_packages", "arguments": {"include_indirect": true} } }' | ./target/debug/elm-package-mcp-server --mcp

# Get README for elm/core
get-readme-core:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "get_elm_package_readme", "arguments": {"author": "elm", "name": "core", "version": "1.0.5"} } }' | ./target/debug/elm-package-mcp-server --mcp

# Get exports for elm/core
get-exports-core:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "get_elm_package_exports", "arguments": {"author": "elm", "name": "core", "version": "1.0.5"} } }' | ./target/debug/elm-package-mcp-server --mcp

# Get export docs for List.map in elm/core
get-export-docs-list-map:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "get_elm_package_export_docs", "arguments": {"author": "elm", "name": "core", "version": "1.0.5", "module": "List", "export_name": "map"} } }' | ./target/debug/elm-package-mcp-server --mcp

# Build debug version
build:
  cargo build

# Build release version
build-release:
  cargo build --release

# Run tests
test:
  cargo test

# Run end-to-end tests
test-e2e:
  cd e2e && python3 test.py

# Clean build artifacts
clean:
  cargo clean

# Check if version in Cargo.toml matches the given tag
check-version TAG:
  #!/usr/bin/env bash
  TAG_VERSION=${TAG#v}
  CARGO_VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
  if [ "$TAG_VERSION" != "$CARGO_VERSION" ]; then
    echo "Error: Tag version ($TAG_VERSION) does not match Cargo.toml version ($CARGO_VERSION)"
    exit 1
  else
    echo "Version check passed: $CARGO_VERSION"
  fi

# Build release binaries for all platforms (local architecture only)
build-release-local:
  cargo build --release
  @echo "Release binary built at: target/release/elm-package-mcp-server"

# Create a release tarball for the current platform
package-release VERSION:
  #!/usr/bin/env bash
  set -e
  ARCH=$(uname -m)
  OS=$(uname -s | tr '[:upper:]' '[:lower:]')
  if [ "$OS" = "darwin" ]; then OS="macos"; fi
  ASSET_NAME="elm-package-mcp-server-${OS}-${ARCH}"

  echo "Building release binary..."
  cargo build --release

  echo "Creating tarball: ${ASSET_NAME}.tar.gz"
  cd target/release
  tar czf ../../${ASSET_NAME}.tar.gz elm-package-mcp-server
  cd ../..

  echo "Release package created: ${ASSET_NAME}.tar.gz"

# Dry run of cargo publish
publish-dry-run:
  cargo publish --dry-run

# Verify the project is ready for release
pre-release-check:
  @echo "Running pre-release checks..."
  @echo "1. Checking for uncommitted changes..."
  @git diff-index --quiet HEAD -- || (echo "Error: Uncommitted changes found" && exit 1)
  @echo "2. Running tests..."
  @cargo test --quiet
  @echo "3. Running clippy..."
  @cargo clippy -- -D warnings
  @echo "4. Checking formatting..."
  @cargo fmt -- --check
  @echo "5. Building release binary..."
  @cargo build --release --quiet
  @echo "All checks passed! ✅"

# Show current version
version:
  @grep '^version' Cargo.toml | head -1 | cut -d'"' -f2
