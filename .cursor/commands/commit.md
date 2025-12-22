# Generate Commit Message

Generate a commit title and description for ALL changes (tracked, untracked, staged, and unstaged) following the project's commit conventions from CONTRIBUTING.md.

## Process

1. **Check Git Status**: Run `git status` to identify all changes:
   - Staged changes (Changes to be committed)
   - Unstaged changes (Changes not staged for commit)
   - Untracked files
   - Deleted files

2. **Analyze All Changes**: For each file category, examine the changes:
   - **Staged**: Use `git diff --cached <file>` or `git diff --cached` for all staged files
   - **Unstaged**: Use `git diff <file>` or `git diff` for all unstaged files
   - **Untracked**: Read the file content to understand what it contains
   - **Deleted**: Note what was removed and why

3. **Categorize Changes**: Group changes by type:
   - Documentation: README.md, CONTRIBUTING.md, docs/, \*.md files
   - Configuration: Cargo.toml, Cargo.lock, justfile, Makefile, .gitignore, etc.
   - Source code: src/\*_/_.rs files
   - Tests: tests/\*_/_.rs files
   - Build/CI: .github/, Dockerfile, etc.
   - Other: Any other file types

4. **Determine Commit Type**: Based on CONTRIBUTING.md conventions, use one of:
   - `feat:` - New features
   - `fix:` - Bug fixes
   - `refactor:` - Code improvements without changing functionality
   - `docs:` - Documentation changes
   - `test:` - Adding or updating tests
   - `chore:` - Maintenance tasks (dependencies, build scripts, config)
   - `perf:` - Performance improvements

   **Rules:**
   - If multiple types apply, choose the most significant one
   - If it's a mix of docs and code, prefer the code type unless docs are the primary change
   - Configuration/build changes are usually `chore:`

5. **Generate Commit Title**: Format: `<type>: <short description>`
   - Keep it under 72 characters
   - Use imperative mood ("add feature" not "added feature")
   - Be specific but concise
   - Don't end with a period

6. **Generate Commit Description**: Structure the description as follows:
   - **Opening paragraph** (1-3 sentences): What was changed and why, high-level context if needed
   - **Detailed list** (if multiple changes): Use bullet points with `-`, group related changes, be specific about what changed in each file/area
   - **Additional context** (if needed): Breaking changes, migration notes, dependencies added/removed, testing notes

   **Formatting rules:**
   - Wrap lines at 72 characters
   - Use blank lines to separate paragraphs
   - Use backticks for file names, commands, and code references
   - Use `-` for unordered lists
   - Be clear and specific

7. **Include All File Changes**: Make sure the description mentions:
   - All modified files (staged and unstaged)
   - All new files (untracked)
   - All deleted files
   - Significant changes in each file

8. **Verify Completeness**: Before finalizing, ensure:
   - ✅ All changed files are accounted for
   - ✅ Commit type matches the changes
   - ✅ Title is clear and follows conventions
   - ✅ Description explains what and why
   - ✅ Formatting follows the 72-character line limit
   - ✅ No typos or unclear language

## Output Format

```
<type>: <title>

<Opening paragraph explaining what and why>

<Detailed list of changes if applicable>:
- Change in file A: what was changed
- Change in file B: what was changed
- New file C: what it contains/does

<Additional context if needed>
```

## Special Cases

- **Multiple Unrelated Changes**: If changes are completely unrelated, suggest splitting into multiple commits
- **Large Refactoring**: Explain motivation, list major changes, note breaking changes, mention test updates
- **Dependency Updates**: List dependencies added/removed/updated, note version changes, mention security updates or breaking changes
