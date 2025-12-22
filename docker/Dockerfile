# Development Dockerfile for rs-torrent-client
# This image provides a consistent development environment for contributors

FROM rust:slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    git \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install just (command runner)
RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin

# Install pnpm (for markdown formatting)
RUN curl -fsSL https://get.pnpm.io/install.sh | sh -
ENV PNPM_HOME="/root/.local/share/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

# Set working directory
WORKDIR /workspace

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY package.json pnpm-lock.yaml ./

# Install pnpm dependencies
RUN pnpm install --frozen-lockfile

# Copy the rest of the project
COPY . .

# Pre-build dependencies to speed up subsequent builds
RUN cargo fetch

# Default command (can be overridden)
CMD ["/bin/bash"]