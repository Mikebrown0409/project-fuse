# Testing Guide

This document explains the testing structure and how to run tests for FUSE.

## Test Structure

```
tests/
├── common/              # Shared test utilities
│   ├── mod.rs          # Common utilities module
│   └── fixtures.rs     # Fixture loading utilities
├── fixtures/            # Test fixtures
│   ├── c2pa/           # C2PA test assets
│   └── README.md       # Fixture documentation
├── integration/         # Integration tests
│   ├── c2pa.rs         # C2PA signature verification tests
│   ├── checkers.rs     # Checker integration tests
│   ├── error_paths.rs  # Error handling tests
│   └── zkvm_proofs.rs  # zkVM proof generation tests
└── tamper/             # Tamper detection tests
    ├── mod.rs          # Tamper tests module
    ├── helpers.rs      # Tamper utilities
    └── c2pa_tamper.rs  # C2PA tamper tests
```

## Running Tests

### All Tests

```bash
# Run all tests (uses dev mode for speed)
RISC0_DEV_MODE=1 cargo test --workspace

# Or use Makefile
make test
```

### Integration Tests

```bash
# Run integration tests only
RISC0_DEV_MODE=1 cargo test --test integration

# Or use Makefile
make test-integration
```

### Tamper Tests

```bash
# Run tamper detection tests
RISC0_DEV_MODE=1 cargo test --test tamper

# Or use Makefile
make test-tamper
```

### Specific Test Suites

```bash
# C2PA tests
RISC0_DEV_MODE=1 cargo test --test integration c2pa

# Error path tests
RISC0_DEV_MODE=1 cargo test --test integration error_paths

# Checker tests
RISC0_DEV_MODE=1 cargo test --test integration checkers
```

## Test Fixtures

### C2PA Fixtures

Official C2PA test fixtures are stored in `tests/fixtures/c2pa/`. These are sourced from the [C2PA Public Test Files repository](https://github.com/c2pa-org/public-testfiles).

Available fixtures:
- `adobe-20220124-C.jpg` - Basic C2PA manifest (Claim only)
- `adobe-20220124-CA.jpg` - C2PA manifest with Claim and Assertion
- `adobe-20220124-A.jpg` - C2PA manifest with Assertion only
- `adobe-20220124-CIE-sig-CA.jpg` - C2PA manifest with CIE signature
- `adobe-20220124-CII.jpg` - C2PA manifest with CII
- `truepic-20230212-landscape.jpg` - Truepic-generated C2PA asset

### Loading Fixtures in Tests

```rust
use tests::common::fixtures::load_c2pa_fixture;

let fixture_path = load_c2pa_fixture("adobe-20220124-C.jpg")?;
```

## Test Types

### Integration Tests

Integration tests verify end-to-end functionality:
- **C2PA Tests** (`tests/integration/c2pa.rs`): Test C2PA manifest parsing, signature verification, selective disclosure
- **Checker Tests** (`tests/integration/checkers.rs`): Test SOC2, GDPR, Supply Chain, ML Model checkers
- **zkVM Proof Tests** (`tests/integration/zkvm_proofs.rs`): Test proof generation and verification

### Tamper Tests

Tamper tests verify that tampered or malicious inputs are detected:
- **Signature Tampering**: Corrupted signatures should fail verification
- **Manifest Modification**: Modified manifests should fail verification
- **Missing Signatures**: Missing signatures should fail gracefully
- **Invalid Encoding**: Corrupted hex encoding should fail gracefully

### Error Path Tests

Error path tests ensure all failure modes are handled gracefully:
- **Malformed JSON**: Invalid JSON should not panic
- **Missing Fields**: Missing required fields should return errors
- **Invalid Inputs**: Invalid hex, wrong lengths, etc. should fail gracefully
- **No Panics**: Code should never panic on invalid input

## Dev Mode vs Real Proofs

Tests use `RISC0_DEV_MODE=1` for fast execution. Dev mode proofs are:
- ✅ Fast (< 1 second)
- ✅ Useful for testing logic
- ❌ Not cryptographically secure
- ❌ Not suitable for production

For real proof testing, remove `RISC0_DEV_MODE=1` (proofs take 10-15 minutes).

## Coverage Measurement

### Running Coverage

```bash
# Using script
bash scripts/coverage.sh

# Using Makefile
make coverage
```

### Coverage Target

Target: **80%+ code coverage**

Coverage measurement excludes:
- Test files (`tests/**`)
- Example files (`examples/**`)
- Guest program (`fuse-guest`) - tested separately

### Viewing Coverage Report

After running coverage, view the HTML report:
```bash
open target/coverage/tarpaulin-report.html
```

## Troubleshooting

### Guest Program Not Built

If tests fail with "Guest program not built", build it first:

```bash
export RUSTC="$HOME/.risc0/toolchains/v1.91.1-rust-aarch64-apple-darwin/bin/rustc"
cargo build -p fuse-guest --release --target riscv32im-risc0-zkvm-elf
```

### Fixtures Not Found

If fixture loading fails:
1. Ensure fixtures are downloaded: `tests/fixtures/c2pa/` should contain `.jpg` files
2. Check fixture names match exactly (case-sensitive)
3. Verify `tests/common/fixtures.rs` paths are correct

### Tests Timing Out

If tests timeout:
1. Ensure `RISC0_DEV_MODE=1` is set for fast execution
2. Check system resources (proof generation can be CPU-intensive)
3. Run tests individually to isolate slow tests

### Coverage Not Generating

If coverage fails:
1. Install cargo-tarpaulin: `cargo install cargo-tarpaulin`
2. Ensure tests pass first: `cargo test`
3. Check tarpaulin timeout settings in `scripts/coverage.sh`

## Best Practices

1. **Always use dev mode for tests**: Set `RISC0_DEV_MODE=1` for fast iteration
2. **Test error paths**: Ensure all failure modes are tested
3. **Use fixtures**: Load test data from fixtures, don't hardcode
4. **No panics**: Code should never panic on invalid input
5. **Informative errors**: Error messages should help debug issues
6. **Coverage first**: Aim for 80%+ coverage before adding features

## CI/CD Integration

Tests should run in CI with:
- Dev mode enabled for speed
- All test suites executed
- Coverage measurement
- Guest program built if possible

Example CI configuration:
```yaml
- name: Run tests
  run: RISC0_DEV_MODE=1 cargo test --workspace
  
- name: Measure coverage
  run: bash scripts/coverage.sh
```
