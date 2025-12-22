# justfile for rs-torrent-client
#
# This justfile is provided for contributor convenience.
# It contains common commands for formatting, building, running, and cleaning the project.
#
# Usage examples:
#   just run     # Format code and run the application
#   just format  # Format the codebase
#   just build   # Format and build the project
#   just clean   # Clean build artifacts
#
# As the project evolves, more complex workflows and commands may be added as new recipes.
# Contributors are encouraged to use and extend this justfile to streamline development tasks.
#
# Markdown formatting requires Prettier (https://prettier.io/):
#   pnpm add -D prettier
#
# Use 'just format-md' to format all Markdown files.

run:
    cargo fmt
    cargo run

format:
    cargo fmt
    pnpm format

format-md:
    pnpm format

build:
    cargo fmt
    cargo build

clean:
    cargo clean

