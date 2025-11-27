.PHONY: build install test clean fmt clippy check help

# Default target
help:
	@echo "Torrer Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  build      - Build the project"
	@echo "  install    - Install Torrer"
	@echo "  test       - Run tests"
	@echo "  clean      - Clean build artifacts"
	@echo "  fmt        - Format code"
	@echo "  clippy     - Run clippy"
	@echo "  check      - Check code without building"
	@echo "  verify     - Verify installation"
	@echo "  setup-tor  - Setup Tor daemon configuration"

# Build the project
build:
	cargo build --release

# Install Torrer
install:
	sudo ./install.sh

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/

# Format code
fmt:
	cargo fmt

# Run clippy
clippy:
	cargo clippy -- -D warnings

# Check code
check:
	cargo check

# Verify installation
verify:
	./scripts/verify-installation.sh

# Setup Tor daemon
setup-tor:
	sudo ./scripts/setup-tor.sh

# Development build
dev:
	cargo build

# Run with debug logging
run-debug:
	RUST_LOG=debug cargo run

