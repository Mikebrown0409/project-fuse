# Architecture Documentation

## Overview

Project FUSE (Verifiable Compliance Envelope Protocol) is a zero-knowledge proof system for verifiable compliance claims. This document describes the architecture and design decisions.

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

### Current State (MVP)

The MVP uses placeholder proofs for demonstration. The structure is ready for RISC Zero integration.

### Production Integration

To integrate RISC Zero:

1. **Create Guest Program**: Write checker logic as RISC Zero guest program
2. **Generate Proof**: Use `risc0_zkvm::prove()` to generate proofs
3. **Verify Proof**: Use `risc0_zkvm::verify()` to verify proofs
4. **Store Receipt**: Serialize RISC Zero receipt in proof data

Example structure (pseudo-code):

```rust
// In zkVM guest program
fn main() {
    let spec = env::read();
    let system_data = env::read();
    let result = checker.check(&spec, &system_data);
    env::commit(&result);
}

// In prover
let receipt = risc0_zkvm::prove(guest_program, &inputs)?;
let proof = ComplianceProof::new(spec_hash, result, receipt);
```

## File Formats

### Specification Format (JSON/YAML)

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

## Security Considerations

1. **Spec Expiry**: All specs must have expiry dates
2. **Hash Verification**: Proof spec hash must match spec hash
3. **Proof Verification**: All proofs must be cryptographically verified
4. **System Hash**: System being verified is identified by hash

## Future Enhancements

1. **RISC Zero Integration**: Full zkVM proof generation
2. **IPFS Storage**: Store specs and proofs on IPFS
3. **Signature Support**: Add cryptographic signatures to envelopes
4. **Circuit Library**: Reusable compliance circuits
5. **Governance**: Specification versioning and updates

