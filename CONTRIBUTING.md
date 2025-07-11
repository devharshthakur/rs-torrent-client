# Contributing to rs-torrent-client

Thank you for your interest in contributing! Please follow these guidelines to ensure a smooth contribution process.

## 🧠 Project Context File for AI Agents

- The project maintains a **context file** at [context.md](md/context.md).
- **Purpose:** This file provides a comprehensive, up-to-date overview of the project for AI agents for contributors convenience, includes architecture, module responsibilities, current status, and goals.
- **How to use:**
  - **Use** the context file by passing it to your ai agents so that they can understand the project context and architecture.
  - **Update** the context file in your PR if your changes introduce new modules, features, or alter the project structure/logic. This ensures AI agents have accurate context.
  - **Reference** the context file in your PR description if your changes are significant or architectural.

## 📝 Issue First Workflow

- **Create an issue** describing your feature, bug, or improvement, or **get assigned** to an existing issue.
- **Branch from `main`** using the issue title as the branch name (e.g., `improve-bencode-decoder`).

## 💻 Development Guidelines

- [x] Write **idiomatic, well-documented Rust code**.
- [x] **Commit often**: Each commit should be small, focused, and reviewable.
- [x] Use the recommended **commit message format**:
  - `feat: short description` for new features
  - `fix: short description` for bug fixes
  - `refactor: short description` for code improvements
  - `docs: short description` for documentation changes
  - `test: short description` for adding or updating tests
  - `chore: short description` for maintenance tasks (e.g., updating dependencies, build scripts)
  - `perf: short description` for performance improvements
  - Add a longer description if needed.
- [x] **Update [setup.md](md/setup.md)** to reflect any changes to project status or features.
- [x] **Test thoroughly** before opening a PR. Ensure the application builds successfully and passes all tests (if there are any).

> **Note:** A `Makefile` is available at the project root to simplify common tasks such as formatting, building, running, and cleaning the project. As the project evolves, additional commands and workflows may be added to the Makefile, allowing you to perform complex operations with a single command.

## 🚀 Pull Request (PR) Process

- Open a PR to the `main` branch.
- PR description should:
  - Clearly explain what was changed and why
  - Describe the logic/approach used
  - Reference the related issue
  - Summarize testing performed
  - List any follow-up work or known issues
  - **Always update the context file** if changes are substantcial such as feature additions, structural changes etc if it impacts the overall project context .
- PRs for code improvements and code reviews are highly appreciated. For improvements, submition of PR with the changes applied are highly appreciated.

## 🧑‍💻 Setup Instructions

- See [setup.md](md/setup.md) for setup instructions.
- Recommended: Use [GitHub Desktop](https://desktop.github.com/) for an easy workflow (see setup guide for details).
- Alternatively, use the [GitHub CLI](https://cli.github.com/) or standard git commands.

---

Maintainer: [@devharshthakur](https://github.com/devharshthakur)
