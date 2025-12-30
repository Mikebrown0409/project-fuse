# Makefile for FUSE project

.PHONY: test coverage build clean

# Run all tests
test:
	RISC0_DEV_MODE=1 cargo test --workspace

# Run integration tests only
test-integration:
	RISC0_DEV_MODE=1 cargo test --test integration

# Run tamper tests
test-tamper:
	RISC0_DEV_MODE=1 cargo test --test tamper

# Measure test coverage
coverage:
	@bash scripts/coverage.sh

# Build release
build:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean

# Run linter
lint:
	cargo clippy --workspace -- -D warnings

# Format code
fmt:
	cargo fmt --all

# Run all checks (lint, format, test)
check: fmt lint test
