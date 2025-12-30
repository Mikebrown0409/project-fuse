#!/bin/bash
# Coverage measurement script using cargo-tarpaulin

set -e

echo "Installing cargo-tarpaulin..."
cargo install cargo-tarpaulin --locked || echo "cargo-tarpaulin may already be installed"

echo ""
echo "Running coverage measurement..."
echo "Target: 80%+ coverage"
echo ""

# Run coverage with dev mode for faster execution
export RISC0_DEV_MODE=1

# Run tarpaulin with output to terminal and XML file
cargo tarpaulin \
    --workspace \
    --exclude fuse-guest \
    --exclude-files 'tests/**' \
    --exclude-files 'examples/**' \
    --out Xml \
    --out Stdout \
    --timeout 300 \
    --output-dir ./target/coverage

echo ""
echo "Coverage report generated in target/coverage/"
echo "View HTML report: open target/coverage/tarpaulin-report.html"
