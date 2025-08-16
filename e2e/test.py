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


def send_request(request):
    """Send a JSON-RPC request to the MCP server and return the response."""
    try:
        # Run from the e2e directory where elm.json is located
        cmd = ["../target/debug/elm-package-mcp-server", "--mcp"]

        # Send request and get response
        proc = subprocess.Popen(
            cmd,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )

        # Send request and close stdin to signal EOF
        stdout, stderr = proc.communicate(input=json.dumps(request) + '\n', timeout=5)

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
        if len(tools) == 4:
            print_test("tools/list returns 4 tools", True)
        else:
            print_test(f"tools/list returns 4 tools (got {len(tools)})", False)

    # Test 2: List Elm packages
    print("\nTesting list_elm_packages...")
    response = send_request({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "list_elm_packages",
            "arguments": {}
        }
    })

    if check_response(response, "list_elm_packages executes without error"):
        content = response.get("result", {}).get("content", [{}])[0].get("text", "")
        packages_data = json.loads(content)

        # Check if elm/core is in the list
        elm_core_found = any(
            p["author"] == "elm" and p["name"] == "core"
            for p in packages_data.get("packages", [])
        )
        print_test("list_elm_packages returns elm/core", elm_core_found)

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
        exports_data = json.loads(content)

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
        exports_data = json.loads(content)
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
        docs_data = json.loads(content)

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
