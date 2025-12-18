# Project FUSE (Verifiable Compliance Envelope Protocol)

**Tagline:** "The checksum for compliance — portable, machine-verifiable assurance."

## Overview

Project FUSE defines a standard cryptographic artifact — the Verifiable Compliance Envelope (VCE) — that proves a specific compliance claim was mechanically verified, without revealing proprietary systems, sensitive data, or internal logic.

**Key Principle:** We prove "This system ran a checker against this specification and passed."

## Status

Early PoC — software-only, open-source, grant-focused.

**MVP Status**: ✅ Complete - Ready for demonstration and grant proposals. See [docs/IMPLEMENTATION_STATUS.md](docs/IMPLEMENTATION_STATUS.md) for details.

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
   - Ready for grant reviewers

## Technical Stack

- **zkVM**: RISC Zero (Rust, CPU/GPU)
- **Spec format**: JSON/YAML
- **Envelope format**: `.vce` (JSON container with proof + metadata)
- **Proof type**: ZK-SNARK / STARK

## Roadmap

| Phase | Timeline | Focus | Grant Target |
|-------|----------|-------|--------------|
| PoC | Dec 2025 – Jan 2026 | Single compliance artifact | Bootstrap, $0 |
| Small Wins | Jan – Mar 2026 | Open proposal + 3–5 artifacts | Gitcoin / EF / PSE, $50–150k |
| Audits / Governance | Apr – Dec 2026 | Circuit library + council | RISC Zero fund, additional EF, $100–400k |
| Scale | 2027+ | Marketplace, enterprise pilots | SBIR Phase I/II, dual-use optional, $500k+ |

## License

[To be determined - open source]

## Documentation

- [Quick Start Guide](docs/QUICKSTART.md) - Get started in minutes
- [Architecture Documentation](docs/ARCHITECTURE.md) - Technical deep dive
- [Implementation Status](docs/IMPLEMENTATION_STATUS.md) - Current state and roadmap

## Contributing

This is an early-stage PoC. Contributions welcome!

## Note on Zero-Knowledge Proofs

The MVP includes the complete structure for zero-knowledge proofs using RISC Zero. Currently, placeholder proofs are used for demonstration. Full RISC Zero integration is the next priority for production use. The architecture is designed to seamlessly integrate actual zkVM proofs when ready.

