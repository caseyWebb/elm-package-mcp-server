---
description: Update CHANGELOG.md with changes from a commit or working tree
---

You are helping update the CHANGELOG.md file with recent changes.

**Parameters:**
- Optional: commit hash (e.g., `/changelog abc1234` or `/changelog HEAD~1`)

**Steps to perform:**

1. **Determine what changes to analyze:**
   - If a commit hash is provided: Use that commit
   - Else if there are uncommitted changes in working tree: Use `git diff`
   - Else: Use the HEAD commit (`git log -1 --pretty=format:"%H"`)

2. **Get the changes:**
   - For a commit: `git show <commit>` or `git log -1 -p <commit>`
   - For working tree: `git diff` and `git diff --cached`
   - Show the user what changes you're analyzing

3. **Analyze the changes and categorize them:**
   - Review file diffs, commit messages, and code changes
   - Categorize into:
     - **Added**: New features, files, or functionality
     - **Changed**: Modifications to existing features (note if breaking with "**Breaking**:")
     - **Fixed**: Bug fixes
     - **Removed**: Deleted features or files
     - **Security**: Security-related changes
   - Write concise, user-facing descriptions (not technical implementation details)
   - Focus on WHAT changed for users, not HOW it was implemented

4. **Update CHANGELOG.md:**
   - Add entries to the "## Unreleased" section
   - Group by category (### Added, ### Changed, ### Fixed, etc.)
   - Use bullet points with clear, descriptive text
   - If the category doesn't exist yet under Unreleased, create it
   - Show the user the proposed changes

5. **Ask for confirmation** before writing to CHANGELOG.md

**Best practices:**
- Be concise but descriptive
- Focus on user-visible changes
- Mark breaking changes clearly
- Skip internal refactoring unless it impacts users
- Use present tense ("Add support for..." not "Added support for...")

Example entry:
```markdown
## Unreleased

### Added
- npm package support via `npx @caseywebb/elm-package-mcp-server`
- Automated nightly builds published to npm

### Changed
- **Breaking**: Removed cargo/crates.io publishing (npm only)
```
