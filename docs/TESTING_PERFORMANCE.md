# Testing Performance Optimizations

This guide explains how to test the performance mitigation features implemented in Part 1.

## Prerequisites

1. **RISC Zero Toolchain**: Ensure RISC Zero is installed
   ```bash
   # Install RISC Zero toolchain if not already installed
   cargo install cargo-risczero
   cargo risczero install
   ```

2. **Guest Program**: Build the guest program
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

## Testing Steps

### 1. Basic Compilation Tests

**Test default build (CPU only):**
```bash
cargo build --release
cargo test
```

**Test with GPU feature:**
```bash
cargo build --release --features gpu
cargo test --features gpu
```

### 2. CLI Functionality Tests

**Test default (CPU) prover:**
```bash
# Use dev mode for fast testing
RISC0_DEV_MODE=1 cargo run --release --bin fuse-prove -- \
  --spec examples/specs/soc2-control-x.json \
  --system examples/systems/sample-saas-logs.json \
  --prover local \
  --output test-local.vce

# Verify
cargo run --release --bin fuse-verify -- test-local.vce
```

**Test GPU prover (if hardware available):**
```bash
# Build with GPU feature
cargo build --release --features gpu

# Run with GPU prover
RISC0_DEV_MODE=1 cargo run --release --features gpu --bin fuse-prove -- \
  --spec examples/specs/soc2-control-x.json \
  --system examples/systems/sample-saas-logs.json \
  --prover gpu \
  --output test-gpu.vce

# Verify
cargo run --release --bin fuse-verify -- test-gpu.vce
```

**Test error handling (request GPU without feature):**
```bash
# Should fail with informative error
cargo run --release --bin fuse-prove -- \
  --spec examples/specs/soc2-control-x.json \
  --system examples/systems/sample-saas-logs.json \
  --prover gpu \
  --output test-error.vce
# Expected: Error message about 'gpu' feature not enabled
```

### 3. Guest Program SHA256 Optimization Test

**Verify SHA256 optimization works:**
```bash
# Test C2PA checker (uses optimized SHA256)
RISC0_DEV_MODE=1 cargo run --release --bin fuse-prove -- \
  --spec examples/specs/c2pa-selective-disclosure.json \
  --system examples/c2pa/C.jpg \
  --prover local \
  --output test-c2pa.vce

# Verify the proof contains correct claim hash
cargo run --release --bin fuse-verify -- test-c2pa.vce
```

The guest program should use `risc0_zkvm::guest::sha::sha256()` instead of the `sha2` crate, which provides hardware acceleration.

### 4. Performance Benchmarking

**Baseline (CPU):**
```bash
# Disable dev mode for real proof
time cargo run --release --bin fuse-prove -- \
  --spec examples/specs/ed25519-signature-verification.json \
  --system examples/systems/ed25519-test-data.json \
  --prover local \
  --output baseline.vce
```

**GPU (if available):**
```bash
# Build with GPU feature
cargo build --release --features gpu

# Benchmark GPU prover
time cargo run --release --features gpu --bin fuse-prove -- \
  --spec examples/specs/ed25519-signature-verification.json \
  --system examples/systems/ed25519-test-data.json \
  --prover gpu \
  --output gpu-benchmark.vce
```

**Compare results:**
- CPU baseline: ~8-15 minutes (expected)
- GPU: Should be 5-10x faster (~1-3 minutes if working)
- Note: Actual performance depends on hardware and workload

### 5. Integration Tests

**Run all integration tests:**
```bash
# Default build
cargo test --test integration

# With GPU feature
cargo test --features gpu --test integration

**Test specific prover types:**
```rust
// In tests/integration/zkvm_proofs.rs
// Tests should use ProverType::Local for basic tests
// Add GPU tests when feature is enabled
```

### 6. Error Handling Tests

**Test feature gate errors:**
```bash
# Request GPU without feature flag
cargo run --release --bin fuse-prove -- \
  --spec examples/specs/soc2-control-x.json \
  --system examples/systems/sample-saas-logs.json \
  --prover gpu \
  --output test.vce
# Should error: "GPU proving requested but 'gpu' feature is not enabled"

```

## Verification Checklist

- [ ] Default build compiles without errors
- [ ] GPU feature build compiles (if CUDA/Metal available)
- [ ] CLI `--prover local` works
- [ ] CLI `--prover gpu` works (with feature flag)
- [ ] Error messages are informative when features not enabled
- [ ] Guest program builds successfully (SHA256 optimization)
- [ ] C2PA checker works with optimized SHA256
- [ ] Integration tests pass
- [ ] Proofs generated with different provers are valid
- [ ] Performance improvement measurable (if GPU available)

## Troubleshooting

### Guest Program Build Issues

**Error: "Guest program ELF binary not found"**
- Ensure guest program is built: `cargo build -p fuse-guest --release --target riscv32im-risc0-zkvm-elf`
- Check ELF is in expected location: `target/riscv32im-risc0-zkvm-elf/release/fuse-guest`

### GPU Feature Issues

**Error: "GPU proving requested but 'gpu' feature is not enabled"**
- Build with `--features gpu`: `cargo build --release --features gpu`
- Ensure CUDA/Metal drivers are installed (for actual GPU acceleration)

**GPU not detected:**
- RISC Zero will fall back to CPU if GPU not available
- Check GPU drivers: `nvidia-smi` (NVIDIA) or system settings (Apple Silicon)

### SHA256 Optimization Issues

**Compilation error in guest program:**
- Verify `risc0_zkvm::guest::sha` is available in RISC Zero 1.0+
- Check `fuse-guest/Cargo.toml` doesn't have `sha2` dependency
- Ensure `risc0-zkvm` version is 1.0+

## Next Steps

After verifying all tests pass:

1. **Performance Benchmarking**: Run full benchmarks with real proofs (not dev mode)
2. **GPU Testing**: Test on actual GPU hardware if available
3. **Cloud Proving**: Future enhancement - can integrate Boundless or other cloud proving services if needed
4. **Documentation**: Update user-facing docs with performance numbers
5. **CI/CD**: Add feature-gated tests to CI pipeline

## Performance Targets

- **Current Baseline**: 10-15 minutes (CPU)
- **Target with GPU**: 2-5 minutes (5-10x improvement)
- **Future Target with Cloud**: 1-2 minutes (10-20x improvement) - if cloud proving is integrated

Note: Actual performance depends on hardware, workload, and RISC Zero version.
