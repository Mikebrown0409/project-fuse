# Witness Spike Day 1-2: Benchmark Instructions

This document provides step-by-step instructions for running the Day 1-2 performance benchmarks.

**Strategy:** Hybrid approach - Use dev mode for rapid technical validation, then single real proof runs for performance measurement. This balances speed with getting actual performance data.

## Prerequisites

1. **RISC Zero toolchain installed**
   - Ensure `rzup` is installed
   - RISC Zero Rust toolchain should be available

2. **Guest program build**
   - The guest program must be built for `riscv32im-risc0-zkvm-elf` target
   - See [docs/PHASE1_STATUS.md](docs/PHASE1_STATUS.md) for build instructions

## Step 1: Generate Ed25519 Test Data

First, generate the test data for Ed25519 verification:

```bash
cargo run --release --bin generate-ed25519-test-data
```

This will create:
- `examples/systems/ed25519-test-data.json` (valid signature)
- `examples/systems/ed25519-test-data-invalid.json` (invalid signature for testing)

## Step 2: Build Guest Program

Build the guest program with Ed25519 support:

```bash
# Set RISC Zero toolchain (adjust path for your system)
export RUSTC="$HOME/.risc0/toolchains/v1.91.1-rust-aarch64-apple-darwin/bin/rustc"

# Build guest program
cargo build -p fuse-guest --release --target riscv32im-risc0-zkvm-elf

# Copy ELF to expected location (for build script)
cp target/riscv32im-risc0-zkvm-elf/release/fuse-guest \
   fuse-guest/target/riscv32im-risc0-zkvm-elf/release/fuse-guest

# Rebuild fuse-core to include ELF
cargo build -p fuse-core --release
```

## Step 3: Technical Validation (Dev Mode) - ~30 minutes

**Purpose:** Verify Ed25519 works in zkVM before spending time on real proofs.

### 3a. Test Baseline (Dev Mode)

```bash
RISC0_DEV_MODE=1 cargo run --release --bin fuse-prove -- \
  --spec examples/specs/soc2-control-x.json \
  --system examples/systems/sample-saas-logs-1000.json \
  --output test-baseline.vce

# Verify it works
cargo run --release --bin fuse-verify -- test-baseline.vce
```

**Expected:** Should complete in < 1 second and verify successfully.

### 3b. Test Ed25519 (Dev Mode)

```bash
RISC0_DEV_MODE=1 cargo run --release --bin fuse-prove -- \
  --spec examples/specs/ed25519-signature-verification.json \
  --system examples/systems/ed25519-test-data.json \
  --output test-ed25519.vce

# Verify it works
cargo run --release --bin fuse-verify -- test-ed25519.vce
```

**Expected:** Should complete in < 1 second and verify successfully.

**If either fails:** Fix the issue before proceeding to real proofs.

### 3c. Test Invalid Signature (Dev Mode)

```bash
RISC0_DEV_MODE=1 cargo run --release --bin fuse-prove -- \
  --spec examples/specs/ed25519-signature-verification.json \
  --system examples/systems/ed25519-test-data-invalid.json \
  --output test-ed25519-invalid.vce

# Check result (should fail verification or return Fail)
cargo run --release --bin fuse-verify -- test-ed25519-invalid.vce
```

**Expected:** Should detect invalid signature correctly.

## Step 4: Performance Measurement (Real Proofs) - ~30-60 minutes

**Purpose:** Get actual proof generation times to measure Ed25519 overhead.

**Note:** We run **single iterations** (not 3) to save time while still getting real performance data.

### 4a. Baseline Performance (Real Proof)

**Time estimate:** 10-20 minutes

```bash
# Run baseline benchmark (real proof, 1 iteration)
cargo run --release --bin fuse-benchmark -- \
  --checker baseline \
  --spec examples/specs/soc2-control-x.json \
  --system examples/systems/sample-saas-logs-1000.json \
  --iterations 1 \
  --json > baseline-results.json

# Also capture human-readable output
cargo run --release --bin fuse-benchmark -- \
  --checker baseline \
  --spec examples/specs/soc2-control-x.json \
  --system examples/systems/sample-saas-logs-1000.json \
  --iterations 1
```

**Record:** Duration in seconds and minutes.

### 4b. Ed25519 Performance (Real Proof)

**Time estimate:** 10-20 minutes

```bash
# Run Ed25519 benchmark (real proof, 1 iteration)
cargo run --release --bin fuse-benchmark -- \
  --checker ed25519 \
  --spec examples/specs/ed25519-signature-verification.json \
  --system examples/systems/ed25519-test-data.json \
  --iterations 1 \
  --json > ed25519-results.json

# Also capture human-readable output
cargo run --release --bin fuse-benchmark -- \
  --checker ed25519 \
  --spec examples/specs/ed25519-signature-verification.json \
  --system examples/systems/ed25519-test-data.json \
  --iterations 1
```

**Record:** Duration in seconds and minutes.

## Step 5: Calculate Overhead & Make Decision

1. **Extract times from results:**
   - Baseline duration: [X] seconds ([Y] minutes)
   - Ed25519 duration: [A] seconds ([B] minutes)

2. **Calculate overhead:**
   ```
   Overhead = Ed25519 Duration - Baseline Duration
   ```

3. **Make go/no-go decision:**
   - **If overhead > 5 minutes:** ❌ **STOP SPIKE** - Not viable
   - **If overhead 2-5 minutes:** ⚠️ **WARNING** - Continue but document as "needs optimization"
   - **If overhead < 2 minutes:** ✅ **PROCEED** - Continue with confidence

4. **Document in results file:**
   - Fill in [WITNESS_SPIKE_DAY1-2_RESULTS.md](WITNESS_SPIKE_DAY1-2_RESULTS.md)
   - Note that these are single-run measurements (not averages)
   - Document decision and rationale

## Time Estimates

**Total time:** ~1-2 hours
- Step 1 (Generate data): 1 minute
- Step 2 (Build guest): 5-10 minutes
- Step 3 (Dev mode validation): 5-10 minutes
- Step 4a (Baseline real proof): 10-20 minutes
- Step 4b (Ed25519 real proof): 10-20 minutes
- Step 5 (Documentation): 10 minutes

**Why single runs?**
- Day 1-2 goal is go/no-go decision, not statistical precision
- Single real proof run gives actual performance data
- Multiple runs would take 3-6 hours (not practical on laptop)
- Overhead calculation is still meaningful with single runs

## Troubleshooting

### Guest program not found
- Ensure guest program is built: `cargo build -p fuse-guest --release --target riscv32im-risc0-zkvm-elf`
- Check ELF is in expected location

### Compilation errors
- Check that `ed25519-compact` and `hex` dependencies are available
- Ensure `no_std` features are enabled
- Try building in dev mode first to catch compilation issues faster

### Proof generation fails
- Check that RISC Zero toolchain is properly installed
- Try dev mode first: `RISC0_DEV_MODE=1` to verify code works
- Check that guest program ELF is in correct location

### Dev mode works but real proof fails
- This indicates a zkVM-specific issue
- Check RISC Zero version compatibility
- Verify all dependencies are `no_std` compatible

### Performance measurement notes
- Single run includes first-time compilation overhead
- Subsequent runs would be faster, but we're doing single runs for speed
- The overhead calculation (Ed25519 - Baseline) accounts for this
- If you have time, you can run 2-3 iterations for better accuracy

