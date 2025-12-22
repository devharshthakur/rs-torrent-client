# VS Code Dev Container Setup

This directory contains the configuration for VS Code's Dev Containers feature, which allows you to develop entirely inside a Docker container directly from VS Code.

## Prerequisites

- [VS Code](https://code.visualstudio.com/)
- [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
- [Docker](https://docs.docker.com/get-docker/) (version 20.10 or later)
- [Docker Compose](https://docs.docker.com/compose/install/) (version 2.0 or later)

## Quick Start

1. **Open the project in VS Code:**

   ```bash
   code .
   ```

2. **Open Command Palette** (`Ctrl+Shift+P` or `Cmd+Shift+P`)

3. **Select:** `Dev Containers: Reopen in Container`

4. **Wait for the container to build** (first time will take a few minutes)

5. **Start coding!** VS Code will now be running inside the container.

## What's Included

### Extensions Installed Automatically

- **rust-analyzer** - Rust language server with code completion, diagnostics, and more
- **CodeLLDB** - Debugger for Rust
- **crates** - Helps manage Cargo dependencies
- **Even Better TOML** - TOML file support
- **Error Lens** - Inline error highlighting
- **JSON** - JSON file support

### Features

- **Automatic Formatting** - Code formats on save
- **Clippy Integration** - Rust linter runs on save
- **All Cargo Features** - Full feature set enabled
- **Pre-configured Settings** - Optimized for Rust development

## How It Works

The `devcontainer.json` file:

- References the existing `docker/docker-compose.yml` file
- Uses the `dev` service from docker-compose
- Mounts your project directory to `/workspace`
- Installs VS Code extensions automatically
- Configures editor settings for Rust development

## Commands Available

Once inside the container, you can use all the same commands:

```bash
# Format and run
just run

# Build the project
just build

# Format code
just format

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run
```

## Troubleshooting

### Container won't start

1. Make sure Docker is running
2. Check VS Code output: `View > Output > Dev Containers`
3. Try rebuilding: `Dev Containers: Rebuild Container`

### Extensions not installing

- Check VS Code output for errors
- Manually install extensions from the Extensions view
- Reload VS Code window

### Port forwarding issues

If your application needs to listen on ports, uncomment and modify the `forwardPorts` section in `devcontainer.json`.

## Benefits

✅ **Consistent Environment** - Everyone uses the same setup  
✅ **No Local Installation** - No need to install Rust, Cargo, or tools locally  
✅ **Isolated** - Doesn't affect your system  
✅ **Easy Onboarding** - New contributors can start coding immediately  
✅ **VS Code Integration** - Full IntelliSense, debugging, and extensions work seamlessly

## Related Files

- `docker/Dockerfile` - Container image definition
- `docker/docker-compose.yml` - Container orchestration
- `docker/README.md` - Docker setup documentation
