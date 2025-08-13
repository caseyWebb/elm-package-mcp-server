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
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "list_packages", "arguments": {} } }' | ./target/debug/elm-package-mcp-server --mcp

# List all packages (including indirect)
list-packages-all:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "list_packages", "arguments": {"include_indirect": true} } }' | ./target/debug/elm-package-mcp-server --mcp

# Get README for elm/core
get-readme-core:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "get_readme", "arguments": {"package": "elm/core"} } }' | ./target/debug/elm-package-mcp-server --mcp

# Get docs for elm/core
get-docs-core:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "get_docs", "arguments": {"package": "elm/core"} } }' | ./target/debug/elm-package-mcp-server --mcp

# Get docs for specific module in elm/core
get-docs-core-list:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "get_docs", "arguments": {"package": "elm/core", "module": "List"} } }' | ./target/debug/elm-package-mcp-server --mcp

# Build debug version
build:
  cargo build

# Build release version
build-release:
  cargo build --release

# Run tests
test:
  cargo test

# Clean build artifacts
clean:
  cargo clean
