# Makefile for FUSE project

.PHONY: test coverage build clean

# Run all tests (excluding guest program which is RISC-V only)
test:
	RISC0_DEV_MODE=1 cargo test -p fuse-core -p fuse-cli -p fuse-checkers

# Run integration tests only (in fuse-core)
test-integration:
	RISC0_DEV_MODE=1 cargo test -p fuse-core --test '*'

# Run specific test suites
test-c2pa:
	RISC0_DEV_MODE=1 cargo test -p fuse-core --test c2pa_integration

test-tamper:
	RISC0_DEV_MODE=1 cargo test -p fuse-core --test c2pa_tamper

# Measure test coverage
coverage:
	@bash scripts/coverage.sh

# Build guest program first (required for fuse-core)
build-guest:
	@echo "Building guest program..."
	@export RUSTC="$$HOME/.risc0/toolchains/v1.91.1-rust-aarch64-apple-darwin/bin/rustc" 2>/dev/null || \
	 export RUSTC="$$HOME/.risc0/toolchains/v1.91.1-rust-$$(uname -m)-unknown-$$(uname -s | tr '[:upper:]' '[:lower:]')/bin/rustc" 2>/dev/null || \
	 export RUSTC="$$HOME/.risc0/toolchains/v1.91.1-rust-x86_64-unknown-linux-gnu/bin/rustc" 2>/dev/null || \
	 { echo "Error: RISC Zero toolchain not found. Run 'rzup install' first."; exit 1; }; \
	 cargo build -p fuse-guest --release --target riscv32im-risc0-zkvm-elf

# Build release (includes guest program)
build: build-guest
	cargo build --release

# Build with GPU support
build-gpu: build-guest
	cargo build --release --features gpu

# Clean build artifacts (preserves guest program warning)
clean:
	cargo clean
	@echo "Note: Guest program ELF removed. Run 'make build-guest' before building again."

# Run linter
lint:
	cargo clippy --workspace -- -D warnings

# Run security-focused linter
lint-security:
	cargo clippy --workspace -- -D warnings -W clippy::suspicious -W clippy::cargo -W clippy::pedantic

# Run security audit
audit:
	@bash scripts/security-audit.sh

# Format code
fmt:
	cargo fmt --all

# Run all checks (lint, format, test)
check: fmt lint test
