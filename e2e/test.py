#!/usr/bin/env python3
"""
End-to-end test for elm-package-mcp-server using elm/core package.
"""

import json
import subprocess
import sys
import os
from pathlib import Path

# Colors for output
RED = '\033[0;31m'
GREEN = '\033[0;32m'
YELLOW = '\033[1;33m'
NC = '\033[0m'  # No Color

# Deprecation warning prefix that all tool responses now include
DEPRECATION_PREFIX = "⚠️ DEPRECATED: This MCP server is deprecated. Use the `migrate-to-skills` prompt for migration instructions, or install the new plugin: /plugin marketplace add caseyWebb/elm-claude-plugin\n\n"

def strip_deprecation_warning(text):
    """Strip the deprecation warning prefix from tool responses."""
    if text.startswith(DEPRECATION_PREFIX):
        return text[len(DEPRECATION_PREFIX):]
    return text

# Test tracking
tests_passed = 0
tests_failed = 0


def print_test(test_name, passed):
    """Print test result with color."""
    global tests_passed, tests_failed

    if passed:
        print(f"{GREEN}✓{NC} {test_name}")
        tests_passed += 1
    else:
        print(f"{RED}✗{NC} {test_name}")
        tests_failed += 1


def send_request(request, timeout=5):
    """Send a JSON-RPC request to the MCP server and return the response."""
    try:
        # Run from the e2e directory where elm.json is located
        cmd = ["../target/debug/elm-package-mcp-server"]

        # Send request and get response
        proc = subprocess.Popen(
            cmd,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )

        # Send request and close stdin to signal EOF
        stdout, stderr = proc.communicate(input=json.dumps(request) + '\n', timeout=timeout)

        # Parse the first line of output (the JSON response)
        for line in stdout.split('\n'):
            line = line.strip()
            if line.startswith('{') and line.endswith('}'):
                return json.loads(line)

        # If no valid JSON found, return error
        return {"error": {"code": -32603, "message": "No valid JSON response found"}}

    except subprocess.TimeoutExpired:
        proc.kill()
        return {"error": {"code": -32603, "message": "Request timed out"}}
    except Exception as e:
        return {"error": {"code": -32603, "message": str(e)}}


def check_response(response, test_name):
    """Check if response has no error and print test result."""
    if "error" in response:
        print(f"{RED}Error in response:{NC}")
        print(json.dumps(response["error"], indent=2))
        print_test(test_name, False)
        return False
    else:
        print_test(test_name, True)
        return True


def main():
    """Run all end-to-end tests."""
    print("Building elm-package-mcp-server...")

    # Build the project
    build_result = subprocess.run(
        ["cargo", "build", "--quiet"],
        cwd="..",
        capture_output=True
    )

    if build_result.returncode != 0:
        print(f"{RED}Failed to build project{NC}")
        sys.exit(1)

    # Change to e2e directory where elm.json is located
    os.chdir(Path(__file__).parent)

    print(f"\n{YELLOW}Running end-to-end tests with elm/core package{NC}\n")

    # Test 1: List available tools
    print("Testing tools/list...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list"
    })

    if check_response(response, "tools/list executes without error"):
        tools = response.get("result", {}).get("tools", [])
        if len(tools) == 5:
            print_test("tools/list returns 5 tools", True)
        else:
            print_test(f"tools/list returns 5 tools (got {len(tools)})", False)

    # Test 2: List installed packages
    print("\nTesting list_installed_packages...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "list_installed_packages",
            "arguments": {}
        }
    })

    if check_response(response, "list_installed_packages executes without error"):
        content = response.get("result", {}).get("content", [{}])[0].get("text", "")
        packages_data = json.loads(strip_deprecation_warning(content))

        # Check if elm/core is in the list
        elm_core_found = any(
            p["author"] == "elm" and p["name"] == "core"
            for p in packages_data.get("packages", [])
        )
        print_test("list_installed_packages returns elm/core", elm_core_found)

    # Test 3: Get README for elm/core
    print("\nTesting get_elm_package_readme...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "get_elm_package_readme",
            "arguments": {
                "author": "elm",
                "name": "core",
                "version": "1.0.5"
            }
        }
    })

    if check_response(response, "get_elm_package_readme executes without error"):
        content = response.get("result", {}).get("content", [{}])[0].get("text", "")
        has_readme = "# Core Libraries" in content or "core" in content.lower()
        print_test("get_elm_package_readme returns valid README", has_readme)

    # Test 4: Get exports for elm/core List module
    print("\nTesting get_elm_package_exports...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "get_elm_package_exports",
            "arguments": {
                "author": "elm",
                "name": "core",
                "version": "1.0.5",
                "module": "List"
            }
        }
    })

    if check_response(response, "get_elm_package_exports executes without error"):
        content = response.get("result", {}).get("content", [{}])[0].get("text", "")
        exports_data = json.loads(strip_deprecation_warning(content))

        modules = exports_data.get("modules", [])
        if modules and modules[0].get("name") == "List":
            print_test("get_elm_package_exports returns List module exports", True)

            # Check that values don't have comments
            values = modules[0].get("values", [])
            has_no_comments = all("comment" not in v for v in values)
            print_test("get_elm_package_exports excludes comments", has_no_comments)
        else:
            print_test("get_elm_package_exports returns List module exports", False)

    # Test 5: Get exports for all modules
    print("\nTesting get_elm_package_exports without module filter...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "tools/call",
        "params": {
            "name": "get_elm_package_exports",
            "arguments": {
                "author": "elm",
                "name": "core",
                "version": "1.0.5"
            }
        }
    })

    if check_response(response, "get_elm_package_exports (all modules) executes without error"):
        content = response.get("result", {}).get("content", [{}])[0].get("text", "")
        exports_data = json.loads(strip_deprecation_warning(content))
        module_count = len(exports_data.get("modules", []))
        print_test("get_elm_package_exports returns multiple modules", module_count > 1)

    # Test 6: Get export docs for List.map
    print("\nTesting get_elm_package_export_docs...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 6,
        "method": "tools/call",
        "params": {
            "name": "get_elm_package_export_docs",
            "arguments": {
                "author": "elm",
                "name": "core",
                "version": "1.0.5",
                "module": "List",
                "export_name": "map"
            }
        }
    })

    if check_response(response, "get_elm_package_export_docs executes without error"):
        content = response.get("result", {}).get("content", [{}])[0].get("text", "")
        docs_data = json.loads(strip_deprecation_warning(content))

        has_map_docs = (
            docs_data.get("export_name") == "map" and
            "Apply a function" in docs_data.get("comment", "")
        )
        print_test("get_elm_package_export_docs returns map documentation", has_map_docs)

    # Test 7: Get export docs for non-existent export
    print("\nTesting get_elm_package_export_docs with invalid export...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 7,
        "method": "tools/call",
        "params": {
            "name": "get_elm_package_export_docs",
            "arguments": {
                "author": "elm",
                "name": "core",
                "version": "1.0.5",
                "module": "List",
                "export_name": "nonExistentFunction"
            }
        }
    })

    has_error = "error" in response
    print_test("get_elm_package_export_docs returns error for invalid export", has_error)

    # Test 8: Test resources/list
    print("\nTesting resources/list...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 8,
        "method": "resources/list"
    })

    if check_response(response, "resources/list executes without error"):
        resources = response.get("result", {}).get("resources", [])
        has_elm_json = any(r.get("uri") == "elm://elm.json" for r in resources)
        print_test("resources/list includes elm://elm.json", has_elm_json)

    # Test 9: Test resources/read for elm.json
    print("\nTesting resources/read...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 9,
        "method": "resources/read",
        "params": {
            "uri": "elm://elm.json"
        }
    })

    if check_response(response, "resources/read executes without error"):
        content = response.get("result", {}).get("content", {})
        if content:
            text = content.get("text", "")
            if text:
                try:
                    elm_json_data = json.loads(text)
                    has_elm_core = "elm/core" in elm_json_data.get("dependencies", {}).get("direct", {})
                    print_test("resources/read returns valid elm.json", has_elm_core)
                except json.JSONDecodeError:
                    print(f"{RED}Failed to parse elm.json content{NC}")
                    print_test("resources/read returns valid elm.json", False)
            else:
                print(f"{RED}Empty content in resources/read response{NC}")
                print_test("resources/read returns valid elm.json", False)
        else:
            print(f"{RED}No content in resources/read response{NC}")
            print_test("resources/read returns valid elm.json", False)

    # Test 10: Test search_packages
    print("\nTesting search_packages...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 10,
        "method": "tools/call",
        "params": {
            "name": "search_packages",
            "arguments": {
                "query": "json"
            }
        }
    }, timeout=15)

    if check_response(response, "search_packages executes without error"):
        content = response.get("result", {}).get("content", [{}])[0].get("text", "")
        search_data = json.loads(strip_deprecation_warning(content))
        results = search_data.get("results", [])
        has_results = len(results) > 0
        print_test("search_packages returns results", has_results)

        if has_results:
            # Check that at least one result is json-related
            has_json_package = any("json" in r["name"].lower() or "json" in r["summary"].lower() for r in results)
            print_test("search_packages returns json-related packages", has_json_package)

    # Test 11: Test search_packages with already_included=false
    print("\nTesting search_packages with exclusions...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 11,
        "method": "tools/call",
        "params": {
            "name": "search_packages",
            "arguments": {
                "query": "core",
                "already_included": False
            }
        }
    }, timeout=15)

    if check_response(response, "search_packages with exclusions executes without error"):
        content = response.get("result", {}).get("content", [{}])[0].get("text", "")
        search_data = json.loads(strip_deprecation_warning(content))
        results = search_data.get("results", [])

        # Check that elm/core is not in results (since it's in elm.json)
        has_elm_core = any(r["name"] == "elm/core" for r in results)
        print_test("search_packages excludes installed packages", not has_elm_core)

    # Test 12: Test list_installed_packages with include_indirect=true
    print("\nTesting list_installed_packages with indirect deps...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 12,
        "method": "tools/call",
        "params": {
            "name": "list_installed_packages",
            "arguments": {
                "include_indirect": True
            }
        }
    })

    if check_response(response, "list_installed_packages with indirect deps executes without error"):
        content = response.get("result", {}).get("content", [{}])[0].get("text", "")
        packages_data = json.loads(strip_deprecation_warning(content))

        indirect_count = packages_data.get("indirect_count", 0)
        total = packages_data.get("total", 0)
        direct_count = packages_data.get("direct_count", 0)

        # Check that we have both direct and indirect packages
        has_indirect = indirect_count > 0
        print_test("list_installed_packages includes indirect dependencies", has_indirect)

        # Check that total = direct + indirect
        correct_total = total == (direct_count + indirect_count)
        print_test("list_installed_packages has correct total count", correct_total)

    # Test 13: Test prompts/list
    print("\nTesting prompts/list...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 13,
        "method": "prompts/list"
    })

    if check_response(response, "prompts/list executes without error"):
        prompts = response.get("result", {}).get("prompts", [])
        if len(prompts) == 7:
            print_test("prompts/list returns 7 prompts", True)
        else:
            print_test(f"prompts/list returns 7 prompts (got {len(prompts)})", False)

    # Test 14: Test prompts/get for analyze-dependencies
    print("\nTesting prompts/get for analyze-dependencies...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 14,
        "method": "prompts/get",
        "params": {
            "name": "analyze-dependencies"
        }
    })

    if check_response(response, "prompts/get (analyze-dependencies) executes without error"):
        messages = response.get("result", {}).get("messages", [])
        has_messages = len(messages) > 0
        print_test("prompts/get (analyze-dependencies) returns messages", has_messages)

    # Test 15: Test prompts/get for explore-package
    print("\nTesting prompts/get for explore-package...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 15,
        "method": "prompts/get",
        "params": {
            "name": "explore-package",
            "arguments": {
                "package": "elm/json"
            }
        }
    })

    if check_response(response, "prompts/get (explore-package) executes without error"):
        messages = response.get("result", {}).get("messages", [])
        has_messages = len(messages) > 0
        print_test("prompts/get (explore-package) returns messages", has_messages)

        if has_messages:
            text = messages[0].get("content", {}).get("text", "")
            has_package_name = "elm/json" in text
            print_test("prompts/get (explore-package) includes package name", has_package_name)

    # Test 16: Test prompts/get for find-function
    print("\nTesting prompts/get for find-function...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 16,
        "method": "prompts/get",
        "params": {
            "name": "find-function",
            "arguments": {
                "capability": "parse json"
            }
        }
    })

    if check_response(response, "prompts/get (find-function) executes without error"):
        messages = response.get("result", {}).get("messages", [])
        has_messages = len(messages) > 0
        print_test("prompts/get (find-function) returns messages", has_messages)

    # Test 17: Test prompts/get for debug-import
    print("\nTesting prompts/get for debug-import...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 17,
        "method": "prompts/get",
        "params": {
            "name": "debug-import",
            "arguments": {
                "module_path": "List"
            }
        }
    })

    if check_response(response, "prompts/get (debug-import) executes without error"):
        messages = response.get("result", {}).get("messages", [])
        has_messages = len(messages) > 0
        print_test("prompts/get (debug-import) returns messages", has_messages)

    # Test 18: Test prompts/get for discover-packages
    print("\nTesting prompts/get for discover-packages...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 18,
        "method": "prompts/get",
        "params": {
            "name": "discover-packages",
            "arguments": {
                "need": "http requests"
            }
        }
    })

    if check_response(response, "prompts/get (discover-packages) executes without error"):
        messages = response.get("result", {}).get("messages", [])
        has_messages = len(messages) > 0
        print_test("prompts/get (discover-packages) returns messages", has_messages)

    # Test 19: Test prompts/get for package-comparison
    print("\nTesting prompts/get for package-comparison...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 19,
        "method": "prompts/get",
        "params": {
            "name": "package-comparison",
            "arguments": {
                "package1": "elm/json",
                "package2": "elm/html"
            }
        }
    })

    if check_response(response, "prompts/get (package-comparison) executes without error"):
        messages = response.get("result", {}).get("messages", [])
        has_messages = len(messages) > 0
        print_test("prompts/get (package-comparison) returns messages", has_messages)

        if has_messages:
            text = messages[0].get("content", {}).get("text", "")
            has_both_packages = "elm/json" in text and "elm/html" in text
            print_test("prompts/get (package-comparison) includes both packages", has_both_packages)

    # Summary
    print(f"\n{YELLOW}Test Summary:{NC}")
    print(f"Tests passed: {GREEN}{tests_passed}{NC}")
    print(f"Tests failed: {RED}{tests_failed}{NC}")

    if tests_failed == 0:
        print(f"\n{GREEN}All tests passed!{NC}")
        sys.exit(0)
    else:
        print(f"\n{RED}Some tests failed!{NC}")
        sys.exit(1)


if __name__ == "__main__":
    main()
