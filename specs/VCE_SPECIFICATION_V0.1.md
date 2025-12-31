# Verifiable Compliance Envelope (VCE) Specification v0.1

**Status**: Published  
**Date**: January 2025  
**Version**: 0.1.0

---

## Table of Contents

1. [Introduction](#introduction)
2. [VCE File Format](#vce-file-format)
3. [ComplianceSpec Format](#compliancespec-format)
4. [Proof Format](#proof-format)
5. [Verification Process](#verification-process)
6. [Versioning](#versioning)
7. [Security Considerations](#security-considerations)
8. [Examples](#examples)

---

## Introduction

### What is VCE?

The **Verifiable Compliance Envelope (VCE)** is a standard cryptographic artifact that proves a specific procedural verification ran to completion, without revealing proprietary systems, sensitive data, or internal logic.

VCE is the standard format used by **Project FUSE** to package proofs-of-verification.

### Key Principle

VCE proves:
> "This system ran a procedural checker against this specification and passed."

**Without revealing:**
- Proprietary systems
- Sensitive data
- Internal logic
- Operational details

**Note**: VCE proves *that* a process occurred; it does not assert the *truth* of the content being verified. Higher-level trust decisions belong to downstream systems.

### Design Goals

1. **Portable**: `.vce` files work offline, anywhere (like PDFs)
2. **Verifiable**: Cryptographic verification without network dependency
3. **Privacy-preserving**: Zero-knowledge proofs protect sensitive data
4. **Standardized**: Open format, implementable by anyone
5. **Interoperable**: Works across platforms and languages

---

## VCE File Format

### Overview

A VCE file (`.vce`) is a JSON document containing:
- The compliance specification that was verified
- The cryptographic proof of verification
- Optional cryptographic signature

### Root Structure

```json
{
  "spec": { /* ComplianceSpec object */ },
  "proof": { /* ComplianceProof object */ },
  "signature": "..." /* Optional: cryptographic signature */
}
```

### Field Descriptions

#### `spec` (required)

The compliance specification that was verified. Must be a valid `ComplianceSpec` object (see [ComplianceSpec Format](#compliancespec-format)).

#### `proof` (required)

The zero-knowledge proof of compliance check execution. Must be a valid `ComplianceProof` object (see [Proof Format](#proof-format)).

#### `signature` (optional)

Optional cryptographic signature for additional authenticity. Currently reserved for future use. If present, must be a string.

### File Extension

VCE files use the `.vce` file extension and are valid JSON documents.

### Example VCE File

```json
{
  "spec": {
    "claim": "SOC2 control X verified",
    "system_hash": "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456",
    "constraints": {
      "control_X": "enforced in all transactions",
      "sampling": "last 1000 events"
    },
    "jurisdiction": "US, SEC",
    "version": "1.0",
    "expiry": "2026-12-31T23:59:59Z",
    "metadata": {
      "standard": "SOC2",
      "control_id": "CC6.1"
    }
  },
  "proof": {
    "proof_data": [ /* Array of bytes (RISC Zero receipt) */ ],
    "spec_hash": "abc123...",
    "result": "Pass",
    "timestamp": "2025-01-15T10:30:00Z",
    "journal": [ /* Array of bytes (public outputs) */ ]
  }
}
```

---

## ComplianceSpec Format

### Overview

A `ComplianceSpec` defines what needs to be verified. It is used both as input to proof generation and embedded in the VCE file.

### Structure

```json
{
  "claim": "string",
  "system_hash": "string",
  "constraints": { /* object */ },
  "jurisdiction": "string",
  "version": "string",
  "expiry": "string",
  "metadata": { /* object (optional) */ }
}
```

### Field Descriptions

#### `claim` (required, string)

Human-readable description of the compliance requirement being verified.

**Examples:**
- `"SOC2 control X verified"`
- `"GDPR data residency"`
- `"Supply chain provenance"`
- `"ML model usage constraint"`

**Validation:**
- Must be non-empty
- No specific format required, but should be descriptive

#### `system_hash` (required, string)

SHA256 hash (hex-encoded) of the system binary, configuration, or data being verified. This identifies the specific system version that was checked.

**Format:**
- Hexadecimal string (64 characters)
- SHA256 hash of the system being verified

**Example:**
```json
"system_hash": "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456"
```

**Validation:**
- Must be non-empty
- Should be a valid hex string (though format not strictly enforced in v0.1)

#### `constraints` (required, object)

Key-value pairs defining verification parameters and constraints for the compliance check. The structure is flexible to accommodate different compliance frameworks.

**Format:**
- Object with string keys and string values
- Keys and values are framework-specific

**Examples:**

SOC2:
```json
"constraints": {
  "control_X": "enforced in all transactions",
  "sampling": "last 1000 events"
}
```

GDPR:
```json
"constraints": {
  "data_region": "EU",
  "requirement": "All personal data must be stored within EU boundaries"
}
```

**Validation:**
- Must be an object
- Keys and values must be strings
- Empty object `{}` is allowed

#### `jurisdiction` (required, string)

Regulatory framework or jurisdiction applicable to this compliance check.

**Examples:**
- `"US, SEC"`
- `"EU, GDPR"`
- `"US, NIST"`
- `"US, Multi-jurisdictional"`

**Validation:**
- Must be non-empty
- Format is free-form but should identify the regulatory framework

#### `version` (required, string)

Version of the compliance specification. Used for tracking specification evolution.

**Format:**
- Semantic versioning recommended (e.g., `"1.0"`, `"1.2.3"`)
- Free-form string allowed

**Example:**
```json
"version": "1.0"
```

**Validation:**
- Must be non-empty

#### `expiry` (required, string)

RFC3339 datetime string indicating when this specification expires. Proofs generated after expiry are invalid.

**Format:**
- RFC3339 datetime (ISO 8601 with timezone)
- UTC timezone recommended

**Example:**
```json
"expiry": "2026-12-31T23:59:59Z"
```

**Validation:**
- Must be valid RFC3339 datetime
- Must be in the future (at time of proof generation)

#### `metadata` (optional, object)

Additional framework-specific metadata. Structure is flexible.

**Format:**
- Object with string keys and string values
- Optional field (may be omitted)

**Examples:**

```json
"metadata": {
  "standard": "SOC2",
  "control_id": "CC6.1",
  "description": "Access control verification for system events"
}
```

**Validation:**
- If present, must be an object
- Keys and values must be strings

### Complete ComplianceSpec Example

```json
{
  "claim": "SOC2 control X verified",
  "system_hash": "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456",
  "constraints": {
    "control_X": "enforced in all transactions",
    "sampling": "last 1000 events"
  },
  "jurisdiction": "US, SEC",
  "version": "1.0",
  "expiry": "2026-12-31T23:59:59Z",
  "metadata": {
    "standard": "SOC2",
    "control_id": "CC6.1",
    "description": "Access control verification for system events"
  }
}
```

---

## Proof Format

### Overview

The `ComplianceProof` contains the cryptographic proof that a compliance checker executed correctly against the specification.

### Structure

```json
{
  "proof_data": [ /* array of numbers (bytes) */ ],
  "spec_hash": "string",
  "result": "Pass" | "Fail",
  "timestamp": "string",
  "journal": [ /* array of numbers (bytes) */ ]
}
```

### Field Descriptions

#### `proof_data` (required, array of numbers)

The serialized RISC Zero receipt containing the cryptographic proof. Serialized using bincode format and represented as an array of byte values (0-255) in JSON.

**Format:**
- Array of integers (0-255), each representing a byte
- Binary data serialized using bincode
- For real proofs: Contains full RISC Zero STARK proof data (~33MB for typical proofs)
- For placeholder proofs: Empty array `[]`

**Example:**
```json
"proof_data": [3, 0, 0, 0, 0, 0, 0, 0, ...]
```

**Validation:**
- Must be an array
- Each element must be an integer between 0 and 255
- Empty array allowed (placeholder proof)

#### `spec_hash` (required, string)

SHA256 hash (hex-encoded) of the `ComplianceSpec` that was verified. Must match the hash of the `spec` field in the VCE file.

**Format:**
- Hexadecimal string (64 characters)
- Computed by serializing the `ComplianceSpec` to JSON (canonical form) and hashing with SHA256

**Computation:**
1. Serialize `ComplianceSpec` to JSON (using BTreeMap for deterministic key ordering)
2. Compute SHA256 hash of JSON bytes
3. Encode hash as hexadecimal string

**Example:**
```json
"spec_hash": "abc123def456..."
```

**Validation:**
- Must be non-empty
- Must match `spec.hash()` when computed

#### `result` (required, string)

Result of the compliance check. Must be either `"Pass"` or `"Fail"`.

**Values:**
- `"Pass"`: Compliance check passed
- `"Fail"`: Compliance check failed

**Validation:**
- Must be exactly `"Pass"` or `"Fail"` (case-sensitive)

#### `timestamp` (required, string)

RFC3339 datetime string indicating when the proof was generated.

**Format:**
- RFC3339 datetime (ISO 8601 with timezone)
- UTC timezone recommended

**Example:**
```json
"timestamp": "2025-01-15T10:30:00Z"
```

**Validation:**
- Must be valid RFC3339 datetime

#### `journal` (required, array of numbers)

RISC Zero journal containing public outputs from zkVM execution. Serialized as an array of byte values (0-255) in JSON.

**Format:**
- Array of integers (0-255), each representing a byte
- Contains public outputs committed by the guest program
- Includes the `ComplianceResult` encoded in the journal

**Example:**
```json
"journal": [0, 0, 0, 0, ...]
```

**Validation:**
- Must be an array
- Each element must be an integer between 0 and 255
- Non-empty for real proofs

### Complete ComplianceProof Example

```json
{
  "proof_data": [3, 0, 0, 0, 0, 0, 0, 0, ...],
  "spec_hash": "abc123def456...",
  "result": "Pass",
  "timestamp": "2025-01-15T10:30:00Z",
  "journal": [0, 0, 0, 0, ...]
}
```

---

## Verification Process

### Overview

Verifying a VCE file involves:
1. Validating the specification
2. Verifying the proof matches the specification
3. Cryptographically verifying the proof

### Step-by-Step Verification

#### Step 1: Validate Specification

1. Parse the `spec` field as a `ComplianceSpec`
2. Validate required fields are present and non-empty:
   - `claim` must be non-empty
   - `system_hash` must be non-empty
   - `constraints` must be an object
   - `jurisdiction` must be non-empty
   - `version` must be non-empty
   - `expiry` must be valid RFC3339 datetime
3. Check that `expiry` is in the future (specification not expired)

#### Step 2: Verify Spec Hash Match

1. Compute the hash of the `spec` field:
   - Serialize `spec` to JSON (canonical form, using BTreeMap for deterministic key ordering)
   - Compute SHA256 hash
   - Encode as hexadecimal string
2. Compare computed hash with `proof.spec_hash`
3. If hashes do not match, verification fails

#### Step 3: Verify Proof

1. Check if `proof.proof_data` is empty (placeholder proof):
   - If empty: Allow for backward compatibility (v0.1 allows placeholder proofs)
   - If non-empty: Proceed to cryptographic verification
2. For real proofs:
   - Deserialize `proof_data` as RISC Zero receipt (bincode format)
   - Compute image ID from guest program ELF binary
   - Verify receipt using RISC Zero verifier with image ID
   - If verification fails, proof is invalid
3. Decode `journal` to extract `ComplianceResult`
4. Verify `proof.result` matches decoded journal result

#### Step 4: Check Result

1. If all verifications pass and `proof.result == "Pass"`, compliance check passed
2. If `proof.result == "Fail"`, compliance check failed (but proof is still valid)

### Verification Pseudocode

```
function verify_vce(vce_file):
    // Step 1: Parse and validate spec
    spec = parse_json(vce_file.spec)
    if not validate_spec(spec):
        return ERROR_INVALID_SPEC
    
    // Step 2: Verify spec hash
    computed_hash = sha256(canonical_json(spec))
    if computed_hash != vce_file.proof.spec_hash:
        return ERROR_HASH_MISMATCH
    
    // Step 3: Verify proof
    if vce_file.proof.proof_data is empty:
        // Placeholder proof - allow for backward compatibility
        return SUCCESS
    else:
        receipt = deserialize_bincode(vce_file.proof.proof_data)
        image_id = compute_image_id(guest_elf_binary)
        if not verify_receipt(receipt, image_id):
            return ERROR_INVALID_PROOF
    
    // Step 4: Check result
    if vce_file.proof.result == "Pass":
        return COMPLIANCE_PASSED
    else:
        return COMPLIANCE_FAILED
```

### Performance Notes

- **Proof Generation**: Real proofs may take 10-20 minutes depending on data size. This is expected for cryptographic proofs and is a trade-off for security.
- **Proof Verification**: Verification typically completes in < 1 second for real proofs.
- **Proof Size**: Real proofs are typically 20-40MB in size (when serialized as JSON arrays). This is expected for full STARK proof data.

---

## Versioning

### Version Number

This specification is version **0.1.0** (v0.1).

### Changelog

#### v0.1.0 (January 2025)
- Initial published specification
- Defines VCE file format
- Defines ComplianceSpec format
- Defines ComplianceProof format
- Includes RISC Zero proof integration
- Supports placeholder proofs for backward compatibility

### Backward Compatibility

- **Placeholder Proofs**: v0.1 allows `proof_data` to be empty (placeholder proofs) for backward compatibility with pre-v0.1 implementations.
- **Future Versions**: v0.2+ may deprecate placeholder proofs. New fields may be added as optional to maintain backward compatibility.

### Versioning Policy

- **Major version (0.x)**: Breaking changes may occur
- **Minor version (x.1)**: New optional fields, clarifications
- **Patch version (x.x.1)**: Bug fixes, clarifications

---

## Security Considerations

### Spec Expiry

All specifications must have an `expiry` field. Proofs generated after the expiry date are invalid. This prevents use of stale compliance proofs.

**Recommendation**: Set expiry dates appropriately for your compliance framework (e.g., 1 year for annual audits).

### Hash Verification

The `spec_hash` in the proof must match the hash of the `spec` field. This ensures:
- The proof corresponds to the correct specification
- The specification has not been tampered with

**Implementation**: Use deterministic JSON serialization (BTreeMap for key ordering) to ensure consistent hashing.

### Proof Verification

Real proofs must be cryptographically verified using RISC Zero's verifier. This ensures:
- The proof was generated by the expected guest program
- The proof has not been tampered with
- The proof corresponds to the claimed execution

**Implementation**: 
- Compute image ID from guest program ELF binary
- Verify receipt using RISC Zero's `Receipt::verify(image_id)`

### Image ID Computation

The image ID is a cryptographic hash of the guest program. It ensures proofs were generated by the expected code.

**Security**: If the guest program changes, the image ID changes, and old proofs become invalid. This prevents proof reuse with modified checkers.

### System Hash

The `system_hash` identifies the specific system version being verified. This ensures proofs correspond to the correct system state.

**Recommendation**: Compute system hash from system binaries, configurations, or data being verified. Use SHA256 for consistency.

### Optional Signature

The `signature` field is reserved for future cryptographic signatures. Currently optional and not validated in v0.1.

---

## Examples

### Example 1: SOC2 Compliance

**ComplianceSpec:**
```json
{
  "claim": "SOC2 control X verified",
  "system_hash": "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456",
  "constraints": {
    "control_X": "enforced in all transactions",
    "sampling": "last 1000 events"
  },
  "jurisdiction": "US, SEC",
  "version": "1.0",
  "expiry": "2026-12-31T23:59:59Z",
  "metadata": {
    "standard": "SOC2",
    "control_id": "CC6.1",
    "description": "Access control verification for system events"
  }
}
```

### Example 2: GDPR Compliance

**ComplianceSpec:**
```json
{
  "claim": "GDPR data residency",
  "system_hash": "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210",
  "constraints": {
    "data_region": "EU",
    "requirement": "All personal data must be stored within EU boundaries"
  },
  "jurisdiction": "EU, GDPR",
  "version": "1.0",
  "expiry": "2026-12-31T23:59:59Z",
  "metadata": {
    "standard": "GDPR",
    "article": "Article 44-49",
    "description": "Data transfer and residency verification"
  }
}
```

### Example 3: Supply Chain Compliance

**ComplianceSpec:**
```json
{
  "claim": "Supply chain provenance",
  "system_hash": "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  "constraints": {
    "provenance_chain": "verified-supplier",
    "requirement": "All components must have verified provenance chain"
  },
  "jurisdiction": "US, NIST",
  "version": "1.0",
  "expiry": "2026-12-31T23:59:59Z",
  "metadata": {
    "standard": "NIST SP 800-161",
    "description": "Software supply chain security verification"
  }
}
```

### Example 4: ML Model Compliance

**ComplianceSpec:**
```json
{
  "claim": "ML model usage constraint",
  "system_hash": "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
  "constraints": {
    "max_usage": "10000",
    "allowed_domains": "research, healthcare, finance"
  },
  "jurisdiction": "US, Multi-jurisdictional",
  "version": "1.0",
  "expiry": "2026-12-31T23:59:59Z",
  "metadata": {
    "standard": "Custom",
    "description": "ML model usage restrictions and domain limitations"
  }
}
```

---

## References

- [RISC Zero Documentation](https://dev.risczero.com/)
- [JSON Schema Specification](https://json-schema.org/)
- [RFC 3339: Date and Time on the Internet](https://tools.ietf.org/html/rfc3339)
- [SHA-256 Specification](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf)

---

## License

This specification is published under an open-source license (to be determined). Implementations are encouraged to be compatible with this specification.

---

**End of Specification v0.1**

