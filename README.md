# Project FUSE (Verifiable Proof-of-Verification)

**Tagline:** "The checksum for verification — portable, machine-verifiable assurance."

![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)
![Version](https://img.shields.io/badge/version-1.2.0-blue.svg)
![Specification](https://img.shields.io/badge/spec-VCE%20v0.1-green.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)

## Overview

Project FUSE defines a standard cryptographic artifact — the **Verifiable Compliance Envelope (VCE)** — that proves a specific procedural verification ran to completion, without revealing proprietary systems, sensitive data, or internal logic.

**Key Principle:** FUSE proves *that* a process occurred; it does not assert the *truth* of the content being verified. We prove "This system ran a procedural checker against this specification and passed."

## Status

Stable open-source infrastructure for verifiable procedural proofs.

**Version 1.2.0**: ✅ **Current** - Hardened security, finalized scope boundaries, and explicit stability guarantees.

**VCE Specification v0.1**: ✅ **Published** - The formal VCE specification is now available. See [specs/VCE_SPECIFICATION_V0.1.md](specs/VCE_SPECIFICATION_V0.1.md) for the complete specification.

## Stability Contract

FUSE follows [Semantic Versioning](https://semver.org/). As of v1.0.0, the core proof format and verification semantics are considered stable.

- **Stable**: VCE file format, `ComplianceSpec` structure, and proof verification logic.
- **Experimental**: GPU/Hardware acceleration (see `SECURITY.md` for known limitations).

*Breaking changes to the proof surface or core protocol will only be introduced in v2.0.0.*

## Quick Start

### Installation

```bash
cargo build --release
```

### Usage

**Generate a compliance proof:**
```bash
cargo run --release --bin fuse-prove -- \
  --spec examples/specs/soc2-control-x.json \
  --system examples/systems/sample-saas-logs.json \
  --output compliance.vce
```

**Verify a compliance envelope:**
```bash
cargo run --release --bin fuse-verify -- compliance.vce
```

For more examples, see [docs/QUICKSTART.md](docs/QUICKSTART.md).

## Project Structure

```
ProjectFuse/
├── fuse-core/          # Core protocol implementation
├── fuse-cli/           # CLI tools (fuse-prove, fuse-verify)
├── fuse-checkers/      # Example procedural checkers
├── examples/           # Example specs and test data
│   ├── specs/         # Example specification files
│   └── systems/       # Sample system data for testing
├── specs/              # VCE specification and schemas
│   ├── VCE_SPECIFICATION_V0.1.md  # Formal specification
│   └── schemas/       # JSON Schema validation files
└── docs/              # Documentation
```

## Core Components

1. **CLI Tool**
   - `fuse-prove <spec> <system>` → outputs `.vce`
   - `fuse-verify <.vce>` → returns pass/fail

2. **Example Spec Files**
   - SOC2 procedural check
   - GDPR data residency verification
   - Supply-chain provenance validation
   - ML model usage constraint

3. **Reference Implementation**
   - Open-source, documented
   - Production-ready with real cryptographic proofs

## Technical Stack

- **zkVM**: RISC Zero (Rust, CPU/GPU)
- **Spec format**: JSON/YAML
- **Envelope format**: `.vce` (JSON container with proof + metadata)
- **Proof type**: ZK-SNARK / STARK

## Roadmap

| Phase | Timeline | Focus | Key Deliverables |
|-------|----------|-------|------------------|
| Phase 1 | ✅ Complete | zkVM integration & proof generation | Real RISC Zero proofs, CLI tools, core checkers |
| Phase 2 | ✅ Complete | Testing & Reliability | Official C2PA fixtures, tamper detection, integration tests |
| Phase 3 | ✅ Complete | Security Basics | Fuzzing, internal review, security audit readiness |
| Phase 4 | Q4 2026 | Ecosystem expansion | Auditor tools, governance framework, enterprise features |

## Specification

**VCE Specification v0.1** is now published as an open standard:

- **[VCE Specification v0.1](specs/VCE_SPECIFICATION_V0.1.md)** - Complete specification document
- **[JSON Schemas](specs/schemas/)** - Validation schemas for `.vce` files and input specs
- **[Specs Directory](specs/)** - Specification documentation and schemas

The VCE format is designed to be:
- **Portable**: Works offline, no network dependency
- **Verifiable**: Cryptographic verification without platform lock-in
- **Standardized**: Open format, implementable by anyone
- **Interoperable**: Works across platforms and languages

## Documentation

- [Quick Start Guide](docs/QUICKSTART.md) - Get started in minutes
- [Architecture Documentation](docs/ARCHITECTURE.md) - Technical deep dive
- [Testing Guide](docs/TESTING.md) - How to run tests and measure coverage
- [Implementation Status](docs/IMPLEMENTATION_STATUS.md) - Current state and roadmap
- [VCE Specification v0.1](specs/VCE_SPECIFICATION_V0.1.md) - Formal specification

## Testing

Run tests with:
```bash
# All tests (uses dev mode for speed)
RISC0_DEV_MODE=1 cargo test --workspace

# Or use Makefile
make test
```

Measure coverage:
```bash
make coverage
```

Run security checks:
```bash
make audit          # Dependency vulnerability scan
make lint-security  # Security-focused clippy checks
```

See [docs/TESTING.md](docs/TESTING.md) for detailed testing information.

## Security

**Status**: Pre-audit dev version (internal security review completed)

- ✅ Dependency scanning (`cargo audit`)
- ✅ Fuzzing infrastructure (5 targets)
- ✅ Internal security review completed
- ⏳ External audit pending

See [SECURITY.md](SECURITY.md) for security policy and [docs/SECURITY_REVIEW.md](docs/SECURITY_REVIEW.md) for detailed security analysis.

**For Pilots**: Use "Pre-audit dev version" disclaimer until external audit completed.

## License

This project is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

> **Note:** FUSE is a proof-of-verification infrastructure. Higher-level trust decisions belong to downstream systems.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to Project FUSE.

## Note on Zero-Knowledge Proofs

**Status**: ✅ **Complete** - RISC Zero zkVM integration is fully operational. The system generates real cryptographic proofs using RISC Zero 1.2.6. 

**Performance**: Real proof generation takes 10-20+ minutes depending on data size. For development and testing, use `RISC0_DEV_MODE=1` for instant proofs (not cryptographically secure).

**Usage**: Once the guest program is built, `fuse-prove` automatically generates real zkVM proofs. The system maintains backward compatibility with placeholder proofs when the guest program is not available.
