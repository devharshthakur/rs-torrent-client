# Project Setup Guide

Welcome! Follow these steps to get started with development on `rs-torrent-client`.

## 1. Fork & Clone the Repository

### Using GitHub Desktop (Recommended)

1. [Fork the repository](https://github.com/devharshthakur/rs-torrent-client) to your own GitHub account.
2. Open GitHub Desktop and sign in.
3. Click **File > Clone Repository** and select your fork.
4. Choose a local path and click **Clone**.

### Using GitHub CLI or Git

```sh
git clone https://github.com/<your-username>/rs-torrent-client.git
cd rs-torrent-client
git remote add upstream https://github.com/devharshthakur/rs-torrent-client.git
```

## 2. Create a Branch for Your Work

- Always branch from `main`.
- Name your branch after the issue title (e.g., `improve-bencode-decoder`).

## 3. Build & Run

- Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
- Build the project:
  ```sh
  cargo build
  ```
- Run tests:
  ```sh
  cargo test
  ```

## 4. Commit & Push

- Commit small, reviewable changes with clear messages.
- Push to your fork and open a Pull Request to `main`.

## 5. Keeping Your Fork Up-to-Date

You can keep your fork in sync with the main repository using either GitHub Desktop (recommended for convenience) or the command line.

### Using GitHub Desktop (Recommended)

1. Open your local repository in GitHub Desktop.
2. Click on the **Current Branch** dropdown and select `main`.
3. Click **Repository > Fetch origin** to fetch the latest changes from your fork.
4. To sync with the upstream repository:
   - Go to **Repository > Repository Settings**.
   - Under **Remotes**, ensure `upstream` is set to `https://github.com/devharshthakur/rs-torrent-client.git`. If not, click **Add** and enter the URL.
   - Click **Branch > Merge into Current Branch** and select `upstream/main` to merge the latest changes from the main repository into your local `main` branch.

### Using Git (Command Line)

```sh
# Fetch the latest changes from your fork
git fetch origin

# Switch to main branch
git checkout main

# Merge the latest changes from your fork
git merge origin/main

# To sync with the upstream repository
git fetch upstream

# Merge the latest changes from upstream
git merge upstream/main

# Push the updated main branch to your fork
git push origin main
```

## 6. Using the Makefile

A `Makefile` is provided at the project root for your convenience. It currently includes basic commands for formatting, building, running, and cleaning the project:

- `make run` – Format code and run the application
- `make format` – Format the codebase
- `make build` – Format and build the project
- `make clean` – Clean build artifacts

As the project grows, more complex sequences of commands and workflows may be added as new Makefile targets, making it easier to perform common or lengthy tasks with a single command.

You can always inspect or modify the `Makefile` to suit your workflow.
