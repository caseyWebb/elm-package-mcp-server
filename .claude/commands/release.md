---
description: Guide through the release process for elm-package-mcp-server
---

You are helping with the release process for elm-package-mcp-server. Follow the steps in RELEASING.md to create a new release.

**Steps to perform:**

1. **Run pre-release checks:**
   - Check git status (no uncommitted changes)
   - Run tests: `cargo test`
   - Run clippy: `cargo clippy -- -D warnings`
   - Run e2e tests: `cd e2e && python3 test.py`
   - Verify on main branch with latest changes

2. **Determine next version from CHANGELOG.md:**
   - Read CHANGELOG.md to find the last released version
   - Read the "Unreleased" section to understand what changes are included
   - Apply semantic versioning rules:
     - **MAJOR** (X.0.0): Breaking changes (look for "Breaking:" or similar)
     - **MINOR** (0.Y.0): New features in "Added" section
     - **PATCH** (0.0.Z): Only bug fixes in "Fixed" section
   - Calculate and show the next version number
   - If Unreleased section is empty, warn the user and ask if they want to proceed

3. **Update version numbers:**
   - Update `Cargo.toml` version field
   - Update `package.json` version field
   - Show the user the changes for confirmation

4. **Update CHANGELOG.md:**
   - Move items from "Unreleased" section to a new version section with the calculated version
   - Add today's date in format YYYY-MM-DD
   - Show the user the changes for confirmation

5. **Commit and push:**
   - Stage: `Cargo.toml`, `package.json`, `CHANGELOG.md`
   - Commit with message: "chore: prepare release vX.Y.Z"
   - Push to main

6. **Create and push tag:**
   - Create tag: `git tag vX.Y.Z`
   - Push tag: `git push origin vX.Y.Z`

7. **Explain what happens next:**
   - GitHub Actions will build binaries for all platforms
   - Create a GitHub release with changelog
   - Publish to npm automatically with NPM_TOKEN
   - Provide link to monitor: https://github.com/caseyWebb/elm-package-mcp-server/actions

**Important reminders:**
- Tag MUST start with 'v' and match versions in Cargo.toml and package.json
- NPM_TOKEN secret must be configured in GitHub
- The npm package will include both macOS binaries (x86_64 and aarch64)
- Users can install with: `npx @caseywebb/elm-package-mcp-server`

Be thorough and ask for confirmation before making any changes.
