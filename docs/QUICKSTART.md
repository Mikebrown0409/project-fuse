# Quick Start Guide

This guide will help you get started with Project FUSE (VCE Protocol) in minutes.

## Prerequisites

- Rust 1.70 or later (install from [rustup.rs](https://rustup.rs/))
- Cargo (comes with Rust)

## Installation

1. Clone or navigate to the Project FUSE directory:
```bash
cd ProjectFuse
```

2. Build the project:
```bash
cargo build --release
```

## Basic Usage

### Step 1: Generate a Compliance Envelope

Use `fuse-prove` to create a Verifiable Compliance Envelope from a specification and system data:

```bash
cargo run --release --bin fuse-prove -- \
  --spec examples/specs/soc2-control-x.json \
  --system examples/systems/sample-saas-logs.json \
  --output soc2-compliance.vce
```

### Step 2: Verify a Compliance Envelope

Use `fuse-verify` to verify a `.vce` file:

```bash
cargo run --release --bin fuse-verify -- soc2-compliance.vce
```

For verbose output:

```bash
cargo run --release --bin fuse-verify -- --verbose soc2-compliance.vce
```

## Example Workflows

### SOC2 Control Verification

```bash
# Generate proof
cargo run --release --bin fuse-prove -- \
  --spec examples/specs/soc2-control-x.json \
  --system examples/systems/sample-saas-logs.json \
  --output soc2.vce

# Verify
cargo run --release --bin fuse-verify -- soc2.vce
```

### GDPR Data Residency

```bash
# Generate proof
cargo run --release --bin fuse-prove -- \
  --spec examples/specs/gdpr-data-residency.json \
  --system examples/systems/gdpr-storage-locations.json \
  --output gdpr.vce

# Verify
cargo run --release --bin fuse-verify -- gdpr.vce
```

### Supply Chain Provenance

```bash
# Generate proof
cargo run --release --bin fuse-prove -- \
  --spec examples/specs/supply-chain-provenance.json \
  --system examples/systems/supply-chain-components.json \
  --output supply-chain.vce

# Verify
cargo run --release --bin fuse-verify -- supply-chain.vce
```

### ML Model Usage Constraints

```bash
# Generate proof
cargo run --release --bin fuse-prove -- \
  --spec examples/specs/ml-model-usage.json \
  --system examples/systems/ml-model-usage-logs.json \
  --output ml-model.vce

# Verify
cargo run --release --bin fuse-verify -- ml-model.vce
```

## Understanding the Output

When you run `fuse-verify`, you'll see:

- ✅ **Envelope is valid!** - The cryptographic proof is valid
- ✅ **Compliance check: PASS** - The system passed the compliance check
- ❌ **Compliance check: FAIL** - The system failed the compliance check
- ❌ **Verification failed** - The envelope or proof is invalid

## Next Steps

- Read the [Architecture Documentation](ARCHITECTURE.md) to understand how VCE works
- Create your own compliance specifications
- Integrate RISC Zero for production zero-knowledge proofs
- Contribute checkers for additional compliance standards

## Troubleshooting

### "No checker found for claim"
Make sure your specification's `claim` field matches one of the registered checkers. Check `fuse-checkers/src/lib.rs` for available checkers.

### "Spec expired"
Update the `expiry` field in your specification to a future date.

### "Failed to parse system data"
Ensure your system data file is valid JSON and matches the expected format for your checker.

