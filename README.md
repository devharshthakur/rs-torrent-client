# Rust Torrent Client

A **BitTorrent client** implementation in Rust, aiming to support downloading files using the BitTorrent protocol. This project is a work-in-progress and currently in its **very early phase**—core components are being built from the ground up!

**This project is a Rust rewrite of the original [Go Torrent Client](https://github.com/piyushgupta53/go-torrent-client).** by [Piyush Gupta](https://github.com/piyushgupta53)

## Project Status

The current implementation status and roadmap are tracked in [docs/CHECKPOINT.md](md/checkpoint.md). Please refer to it for up-to-date progress and details on what is implemented.

For the project's long-term goals and planned features, see [goals.md](md/goals.md).

## Development Setup

### Local Development

Common development tasks can be run using `just`:

- `just run` - Format code and run the application
- `just format` - Format Rust code and Markdown files
- `just build` - Format and build the project
- `just clean` - Clean build artifacts

See the `justfile` for all available commands.

### Docker Development

This project includes Docker setup for consistent development environments. Contributors can develop entirely inside containers without installing Rust or other dependencies locally.

#### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) (version 20.10 or later)
- [Docker Compose](https://docs.docker.com/compose/install/) (version 2.0 or later)

#### Quick Start

1. **Build and start the development container:**

   ```bash
   cd docker
   docker-compose up -d
   ```

   Or from the project root:

   ```bash
   docker-compose -f docker/docker-compose.yml up -d
   ```

2. **Enter the container:**

   ```bash
   docker-compose -f docker/docker-compose.yml exec dev bash
   ```

3. **Run commands inside the container:**

   ```bash
   # Format and run
   just run

   # Build the project
   just build

   # Format code
   just format

   # Or use cargo directly
   cargo build
   cargo run
   ```

#### Common Docker Commands

All commands should be run from the `docker/` directory or use the `-f docker/docker-compose.yml` flag:

- **Start container:** `docker-compose -f docker/docker-compose.yml up -d`
- **Stop container:** `docker-compose -f docker/docker-compose.yml down`
- **View logs:** `docker-compose -f docker/docker-compose.yml logs -f dev`
- **Execute command in container:** `docker-compose -f docker/docker-compose.yml exec dev <command>`
- **Rebuild container:** `docker-compose -f docker/docker-compose.yml build --no-cache`

> **Note:** Docker configuration files are located in the `docker/` directory. See [docker/README.md](docker/README.md) for detailed documentation.

#### Development Workflow

The container mounts your project directory as a volume, so any changes you make to files on your host machine are immediately reflected inside the container. You can:

- Edit files using your preferred editor on your host machine
- Run builds and tests inside the container
- Use all the same `just` commands as local development

#### Volume Mounts

The Docker setup uses named volumes for:

- `cargo-cache`: Cargo registry cache (speeds up dependency downloads)
- `cargo-git`: Cargo git dependencies cache
- `cargo-target`: Build artifacts cache (optional, can speed up builds)

These volumes persist between container restarts, improving build performance.

### VS Code Dev Container (Recommended)

For the best development experience, use VS Code's Dev Containers feature. This allows you to develop entirely inside the container with full IntelliSense, debugging, and extension support.

#### Prerequisites

- [VS Code](https://code.visualstudio.com/)
- [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
- Docker and Docker Compose (same as above)

#### Quick Start

1. **Open the project in VS Code:**

   ```bash
   code .
   ```

2. **Open Command Palette** (`Ctrl+Shift+P` or `Cmd+Shift+P`)

3. **Select:** `Dev Containers: Reopen in Container`

4. **Wait for the container to build** (first time takes a few minutes)

5. **Start coding!** VS Code will now run inside the container with:
   - Rust language server (rust-analyzer)
   - Code formatting on save
   - Clippy linting
   - Debugger support
   - All extensions pre-installed

See [.devcontainer/README.md](.devcontainer/README.md) for detailed documentation.

## Project Structure

```text
src/
  ├── main.rs         # Binary entry point
  ├── lib.rs          # Library root, re-exports modules
  ├── bencode/        # Bencode encoding/decoding logic
  │     ├── mod.rs
  │     ├── encoder.rs
  │     └── decoder.rs
  ├── torrent/        # Torrent file parsing, info hash, piece hashes
  │     ├── mod.rs
  │     ├── file.rs
  │     └── info_hash.rs
  ├── tracker/        # Tracker client logic
  │     └── mod.rs
  └── peer/           # Peer handshake and communication (WIP)
        ├── mod.rs
        └── handshake.rs
```

## Acknowledgments

- Inspired by the original [Go Torrent Client](https://github.com/piyushgupta53/go-torrent-client) ([piyushgupta53/go-torrent-client](https://github.com/piyushgupta53/go-torrent-client))
- [BitTorrent Protocol Specification](https://wiki.theory.org/BitTorrentSpecification)
- [Bencode Specification](https://wiki.theory.org/BitTorrentSpecification#Bencoding)

> ⚠️ **Note:** This project is not yet ready for general use. Contributions and feedback are welcome as the project evolves!
