# Makefile for rs-torrent-client
#
# This Makefile is provided for contributor convenience.
# It contains common commands for formatting, building, running, and cleaning the project.
#
# Usage examples:
#   make run     # Format code and run the application
#   make format  # Format the codebase
#   make build   # Format and build the project
#   make clean   # Clean build artifacts
#
# As the project evolves, more complex workflows and commands may be added as new targets.
# Contributors are encouraged to use and extend this Makefile to streamline development tasks.

# Markdown formatting requires Prettier (https://prettier.io/):
#   pnpm add -D prettier
#
# Use 'make format-md' to format all Markdown files.
#

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
