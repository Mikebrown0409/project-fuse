# Project FUSE (Verifiable Compliance Envelope Protocol)

**Tagline:** "The checksum for compliance — portable, machine-verifiable assurance."

## Overview

Project FUSE defines a standard cryptographic artifact — the Verifiable Compliance Envelope (VCE) — that proves a specific compliance claim was mechanically verified, without revealing proprietary systems, sensitive data, or internal logic.

**Key Principle:** We prove "This system ran a checker against this specification and passed."

## Status

Early PoC — software-only, open-source, standards-focused.

**MVP Status**: ✅ Complete - Ready for demonstration and early adoption. See [docs/IMPLEMENTATION_STATUS.md](docs/IMPLEMENTATION_STATUS.md) for details.

**VCE Specification v0.1**: ✅ **Published** - The formal VCE specification is now available. See [specs/VCE_SPECIFICATION_V0.1.md](specs/VCE_SPECIFICATION_V0.1.md) for the complete specification.

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
├── fuse-core/          # Core VCE protocol implementation
├── fuse-cli/           # CLI tools (fuse-prove, fuse-verify)
├── fuse-checkers/      # Example compliance checkers
├── examples/           # Example specs and test data
│   ├── specs/         # VCE specification files
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
   - SOC2 audit check
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
| Phase 2 | Q1 2026 | Checker registry & enhanced validation | Plugin system, JSON schema validation, additional frameworks |
| Phase 3 | Q2-Q3 2026 | Performance optimization | GPU acceleration, proof batching, < 5 minute generation |
| Phase 4 | Q4 2026 | Ecosystem expansion | Auditor tools, governance framework, enterprise features |

## License

[To be determined - open source]

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
- [Implementation Status](docs/IMPLEMENTATION_STATUS.md) - Current state and roadmap
- [VCE Specification v0.1](specs/VCE_SPECIFICATION_V0.1.md) - Formal specification

## Contributing

This is an early-stage PoC. Contributions welcome!

## Note on Zero-Knowledge Proofs

**Phase 1 Status**: ✅ **Complete** - RISC Zero zkVM integration is fully operational. The system generates real cryptographic proofs using RISC Zero 1.2.6. 

**Performance**: Real proof generation takes 10-20+ minutes depending on data size. For development and testing, use `RISC0_DEV_MODE=1` for instant proofs (not cryptographically secure).

**Usage**: Once the guest program is built, `fuse-prove` automatically generates real zkVM proofs. The system maintains backward compatibility with placeholder proofs when the guest program is not available.

