# Docker Development Setup

This directory contains all Docker-related files for development:

- `Dockerfile` - Development container image definition
- `docker-compose.yml` - Container orchestration and volume management
- `.dockerignore` - Files to exclude from Docker build context
- `README.md` - This file

## Architecture

The Docker setup is designed for **development**, not production. It provides:

- **Consistent Environment**: All contributors work in the same environment
- **No Local Dependencies**: No need to install Rust, just, or pnpm locally
- **Live Development**: Changes on your host machine are immediately available in the container
- **Cached Builds**: Named volumes cache dependencies and build artifacts

## Container Features

The development container includes:

- **Rust toolchain** (latest stable)
- **just** command runner
- **pnpm** for markdown formatting
- **System dependencies** (OpenSSL, git, curl, etc.)

## Troubleshooting

### Container won't start

```bash
# Check logs (from docker/ directory)
docker-compose logs dev

# Or from project root
docker-compose -f docker/docker-compose.yml logs dev

# Rebuild from scratch
docker-compose build --no-cache
docker-compose up -d
```

### Build is slow

The first build will be slow as it downloads dependencies. Subsequent builds use cached layers and volumes for faster builds.

### Permission issues

If you encounter permission issues with mounted volumes, ensure your user has appropriate permissions. On Linux, you may need to adjust file ownership:

```bash
sudo chown -R $USER:$USER .
```

### Clean everything

To completely reset the Docker environment:

```bash
# Stop and remove containers, networks, and volumes
docker-compose down -v

# Remove the image
docker rmi rs-torrent-client-dev

# Rebuild (from docker/ directory)
docker-compose build
docker-compose up -d
```

> **Note:** All `docker-compose` commands should be run from the `docker/` directory, or use `-f docker/docker-compose.yml` from the project root.

## Quick Start

From the `docker/` directory:

```bash
# Start the container
docker-compose up -d

# Enter the container
docker-compose exec dev bash

# Run commands
just build
cargo test
```

From the project root:

```bash
# Start the container
docker-compose -f docker/docker-compose.yml up -d

# Enter the container
docker-compose -f docker/docker-compose.yml exec dev bash
```

## Advanced Usage

### Run a single command without entering the container

```bash
# From docker/ directory
docker-compose exec dev just build
docker-compose exec dev cargo test

# From project root
docker-compose -f docker/docker-compose.yml exec dev just build
```

### Access container shell directly

```bash
docker-compose exec dev /bin/bash
```

### Check container status

```bash
docker-compose ps
```

### View resource usage

```bash
docker stats rs-torrent-client-dev
```
