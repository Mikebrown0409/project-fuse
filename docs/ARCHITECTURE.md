# Architecture Documentation

## Overview

Project FUSE (Verifiable Compliance Envelope Protocol) is a zero-knowledge proof system for verifiable compliance claims. This document describes the architecture and design decisions.

**Note**: For the complete, formal specification of the VCE format, see [VCE Specification v0.1](../specs/VCE_SPECIFICATION_V0.1.md). This architecture document describes the implementation; the specification document defines the standard format.

## Core Components

### 1. Compliance Specification (`fuse-core/src/spec.rs`)

The `ComplianceSpec` defines what needs to be verified:

- **Claim**: Human-readable description of the compliance requirement
- **System Hash**: SHA256 hash of the system being verified
- **Constraints**: Key-value pairs defining verification parameters
- **Jurisdiction**: Regulatory framework (e.g., "US, SEC", "EU, GDPR")
- **Version**: Specification version
- **Expiry**: When the specification expires
- **Metadata**: Additional optional information

### 2. Compliance Proof (`fuse-core/src/proof.rs`)

The `ComplianceProof` contains:

- **Proof Data**: Zero-knowledge proof (RISC Zero receipt in production)
- **Spec Hash**: Hash of the specification that was verified
- **Result**: Pass or Fail
- **Timestamp**: When the proof was generated
- **Journal**: Public outputs from zkVM execution

### 3. Verifiable Compliance Envelope (`fuse-core/src/envelope.rs`)

The `VerifiableComplianceEnvelope` (VCE) packages everything together:

- **Spec**: The compliance specification
- **Proof**: The zero-knowledge proof
- **Signature**: Optional cryptographic signature

## Workflow

### Proof Generation

1. **Load Specification**: Parse JSON/YAML spec file
2. **Load System Data**: Read system data to verify
3. **Run Checker**: Execute compliance checker in zkVM (RISC Zero)
4. **Generate Proof**: Create zero-knowledge proof of checker execution
5. **Package Envelope**: Combine spec + proof into `.vce` file

### Proof Verification

1. **Load Envelope**: Parse `.vce` file
2. **Validate Spec**: Check spec is valid and not expired
3. **Verify Proof**: Use RISC Zero verifier to check proof validity
4. **Check Result**: Return Pass/Fail status

## Checker System

Checkers implement the `ComplianceChecker` trait:

```rust
pub trait ComplianceChecker {
    fn check(&self, spec: &ComplianceSpec, system_data: &str) -> Result<ComplianceResult>;
}
```

Built-in checkers:
- **SOC2**: Access control verification
- **GDPR**: Data residency verification
- **Supply Chain**: Provenance verification
- **ML Model**: Usage constraint verification

## Zero-Knowledge Integration

### Phase 1 Status: ✅ Complete

Phase 1 zkVM integration is fully operational. The system generates real RISC Zero cryptographic proofs using the 1.2.6 API. All components are implemented, tested, and working.

### Architecture

**Guest Program** (`fuse-guest/`):
- Runs inside RISC Zero zkVM (riscv32im-risc0-zkvm-elf target)
- Reads spec and system data from host via `env::read()`
- Executes appropriate checker based on claim type (SOC2, GDPR, Supply Chain, ML Model)
- Commits result to journal (public output) via `env::commit()`
- Built with `#![no_std]` and `#![no_main]` for zkVM environment

**Host Program** (`fuse-core/src/zkvm.rs`):
- Generates proofs using RISC Zero `ExecutorImpl` and `ProverServer`
- Verifies proofs using RISC Zero `Receipt::verify()` with computed image ID
- Handles serialization/deserialization of receipts using `bincode`
- Computes image ID from ELF binary for verification

**Workflow**:
1. Host prepares inputs (spec JSON, system data JSON) using `ExecutorEnv::builder().write_slice()`
2. Host executes guest program in zkVM via `ExecutorImpl::from_elf()` and `exec.run()`
3. Guest program runs checker and commits `ComplianceResult` to journal
4. Host generates proof via `prover.prove_session()` (can take 10-20+ minutes for real proofs)
5. Receipt is serialized using `bincode` and stored in `.vce` file
6. Verification extracts receipt, computes image ID, and verifies cryptographically

### Implementation Status

- ✅ Guest program structure complete and built
- ✅ All checkers implemented in guest program (SOC2, GDPR, Supply Chain, ML Model)
- ✅ Proof generation/verification infrastructure complete
- ✅ RISC Zero 1.2.6 API integration complete
- ✅ Image ID computation working
- ✅ End-to-end testing complete
- ✅ Error handling with actionable messages
- ✅ Backward compatibility maintained (placeholder proofs still work)

### Performance Characteristics

- **Real Proof Generation**: 10-20+ minutes (first proof), 5-15 minutes (subsequent)
- **Dev Mode Proof Generation**: < 1 second (for testing only, not cryptographically secure)
- **Proof Verification**: < 1 second
- **Use `RISC0_DEV_MODE=1` for development/testing**

### Performance Optimization

FUSE supports multiple prover backends to optimize proof generation performance:

#### Prover Types

1. **Local CPU Prover** (default)
   - Always available, no additional setup required
   - Proof generation: 10-20+ minutes
   - Suitable for: Development, testing, small-scale production

2. **GPU Prover** (optional, requires hardware)
   - **5-10x faster** than CPU proving
   - Requires NVIDIA GPU with CUDA support or Apple Silicon with Metal
   - Proof generation: 2-5 minutes (estimated)
   - Enable with: `cargo build --features gpu` or `--prover gpu`

#### Using GPU Proving

**Build with GPU support:**
```bash
cargo build --release --features gpu
```

**Use GPU prover via CLI:**
```bash
cargo run --release --bin fuse-prove -- \
  --spec examples/specs/soc2-control-x.json \
  --system examples/systems/sample-saas-logs.json \
  --prover gpu \
  --output compliance.vce
```


#### Guest Program Optimizations

The guest program uses RISC Zero's built-in SHA256 accelerator (`risc0_zkvm::guest::sha`) instead of the standard `sha2` crate. This provides:
- Hardware acceleration when available
- Reduced proof generation time
- Lower memory usage in zkVM

#### Performance Targets

- **Technical Feasibility**: < 5 minutes (achievable with GPU)
- **Production Viability**: < 2 minutes (achievable with GPU on high-end hardware)
- **Current Baseline**: 10-15 minutes (CPU-only)

For production deployments, GPU proving is recommended when available.

## File Formats

**Formal Specification**: The complete VCE file format and ComplianceSpec format are defined in [VCE Specification v0.1](../specs/VCE_SPECIFICATION_V0.1.md). JSON schemas for validation are available in [specs/schemas/](../specs/schemas/).

### Specification Format (JSON/YAML)

The `ComplianceSpec` format is fully documented in the [VCE Specification v0.1](../specs/VCE_SPECIFICATION_V0.1.md#compliancespec-format). Example:

```json
{
  "claim": "SOC2 control X verified",
  "system_hash": "sha256...",
  "constraints": {
    "control_X": "enforced in all transactions",
    "sampling": "last 1000 events"
  },
  "jurisdiction": "US, SEC",
  "version": "1.0",
  "expiry": "2026-12-31T23:59:59Z"
}
```

### VCE Format (.vce)

The complete VCE file format is defined in [VCE Specification v0.1](../specs/VCE_SPECIFICATION_V0.1.md#vce-file-format). Example structure:

```json
{
  "spec": { /* ComplianceSpec */ },
  "proof": {
    "proof_data": [ /* binary proof */ ],
    "spec_hash": "...",
    "result": "Pass",
    "timestamp": "...",
    "journal": [ /* public outputs */ ]
  }
}
```

For complete field descriptions, validation rules, and examples, see the [formal specification](../specs/VCE_SPECIFICATION_V0.1.md).

## Security Considerations

1. **Spec Expiry**: All specs must have expiry dates
2. **Hash Verification**: Proof spec hash must match spec hash
3. **Proof Verification**: All proofs must be cryptographically verified
4. **System Hash**: System being verified is identified by hash

## Future Enhancements

1. **IPFS Storage**: Store specs and proofs on IPFS
2. **Signature Support**: Add cryptographic signatures to envelopes
3. **Circuit Library**: Reusable compliance circuits
4. **Governance**: Specification versioning and updates
5. **Cloud Proving**: Integration with Boundless or other cloud proving services (future enhancement, if needed)
6. **Proof Batching**: Batch multiple proofs together for efficiency

