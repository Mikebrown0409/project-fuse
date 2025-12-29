# Witness Demo: Selective Disclosure of C2PA Manifests

This demo demonstrates the core value proposition of Witness: **cryptographically proving that a full C2PA manifest was verified, while only revealing a small, chosen subset of fields**.

## What This Demo Shows

1. **Full Manifest Extraction**: Extracts the complete C2PA manifest from a signed image
2. **Selective Disclosure**: Generates a zero-knowledge proof that verifies the full manifest, but only commits selected fields to the public proof journal
3. **Cryptographic Binding**: Shows how the redacted output is cryptographically bound to the original manifest via SHA256 hash
4. **Public Verification**: Demonstrates that anyone can verify the proof without access to the original image or hidden data

## Prerequisites

- **Rust & Cargo**: Installed and up-to-date
- **Guest Program Built**: The RISC Zero guest program must be compiled:
  ```bash
  cargo build --release --package fuse-guest --target riscv32im-risc0-zkvm-elf
  ```
- **C2PA Asset**: A C2PA-signed image (default: `examples/c2pa/C.jpg`)

## How to Run

From the project root directory:

```bash
./examples/demo-witness.sh
```

Or with explicit paths:

```bash
cd examples
./demo-witness.sh
```

## What to Expect

The demo will:

1. **Extract the full manifest** (~30 seconds)
   - Shows all fields in the C2PA manifest
   - Displays the complete claim JSON

2. **Generate VCE with selective disclosure** (~10-15 minutes)
   - Creates a zero-knowledge proof
   - Only commits selected fields to the proof journal
   - This is the computationally expensive step

3. **Verify the VCE** (~5-10 seconds)
   - Verifies the cryptographic proof
   - Extracts the redacted manifest from the proof journal

4. **Display comparison**
   - Shows **Full Manifest** (all fields) vs **Redacted Manifest** (only disclosed fields)
   - Displays the cryptographic hash binding

5. **Value proposition summary**
   - Explains what was proven
   - Explains what was hidden
   - Explains cryptographic guarantees

## Selective Disclosure Configuration

The demo uses `witness-spec.json`, which specifies:

- **Disclosed Fields**: `claim_generator`, `capture_time`, `issuer`
- **Hidden Fields**: All other fields in the manifest remain private

You can modify `witness-spec.json` to change which fields are disclosed. The selective disclosure operates at **top-level JSON keys only** (no nested field selection in v0.1).

## Understanding the Output

### Full Manifest
Shows the complete C2PA claim JSON with all fields visible. This represents what a traditional C2PA viewer would show.

### Redacted Manifest
Shows only the fields specified in `disclosed_fields`. All other fields are hidden from the verifier, but the proof guarantees they were verified.

### Claim Hash
The SHA256 hash of the original claim bytes. This cryptographically binds the redacted output to the specific signed manifest, preventing tampering or substitution.

## Key Concepts

### Zero-Knowledge Proof
The proof demonstrates that:
- The full manifest was verified
- The signature was validated
- Only selected fields were disclosed

**Without revealing:**
- Hidden field values
- The complete manifest structure
- Any data not explicitly disclosed

### Cryptographic Binding
The claim hash ensures that:
- The redacted output corresponds to a specific signed manifest
- The proof cannot be reused for a different manifest
- Verifiers can detect if the manifest was tampered with

### Public Verifiability
Anyone can:
- Verify the proof without trusted parties
- Confirm that verification occurred
- Trust that hidden fields were verified (even though they can't see them)

## Troubleshooting

### "Guest program not built"
Build the guest program:
```bash
cargo build --release --package fuse-guest --target riscv32im-risc0-zkvm-elf
```

### "C2PA image not found"
Ensure `examples/c2pa/C.jpg` exists, or modify the script to point to your C2PA-signed image.

### Proof generation takes a long time
This is expected. Current performance is ~10-15 minutes for proof generation. Optimization is planned for post-spike work.

### "Failed to extract C2PA manifest"
Ensure the image is a valid C2PA-signed asset. You can test with:
```bash
cargo run --release --bin inspect-c2pa -- --input examples/c2pa/C.jpg
```

## Next Steps

After running this demo:
- Review the [Witness Spike documentation](../../WITNESS_SPIKE.md)
- Check [Phase 2 Results](../../WITNESS_SPIKE_PHASE2_RESULTS.md) for performance details
- Explore modifying `witness-spec.json` to disclose different fields

## Related Files

- `witness-spec.json` - Selective disclosure specification
- `output.vce` - Generated Verifiable Compliance Envelope (created after running demo)
- `../../fuse-cli/src/bin/fuse-prove.rs` - VCE generation tool
- `../../fuse-cli/src/bin/fuse-verify.rs` - VCE verification tool

