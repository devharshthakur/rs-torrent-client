# Development Dockerfile for rs-torrent-client
# This image provides a consistent development environment for contributors

FROM rust:1-bookworm

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    git \
    pkg-config \
    libssl-dev \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install just (command runner) via cargo
RUN cargo install just

# Ensure stable toolchain and install clippy (required for rust-analyzer check on save)
RUN rustup default stable && \
    rustup component add clippy

# Install Node.js v24 (LTS) and pnpm via apt (NodeSource)
RUN curl -fsSL https://deb.nodesource.com/setup_24.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g pnpm@latest

# Set working directory
WORKDIR /workspace