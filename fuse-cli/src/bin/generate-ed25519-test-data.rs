//! Generate Ed25519 test data for benchmarking
//!
//! This tool generates Ed25519 keypairs, signs a test message,
//! and outputs JSON test data for the Ed25519 checker.

use ed25519_compact::{KeyPair, Seed};
use serde_json::json;
use std::fs;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("Generating Ed25519 test data...");

    // Generate keypair from a fixed seed for reproducibility
    // In production, you'd use random seed
    // Seed must be exactly 32 bytes
    let seed_bytes: [u8; 32] = *b"test-seed-for-ed25519-benchmark!";
    let seed = Seed::from_slice(&seed_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to create seed: {}", e))?;
    let keypair = KeyPair::from_seed(seed);

    // Test message
    let message = b"Hello, World! This is a test message for Ed25519 signature verification.";

    // Sign message (sign method is on the secret key)
    let signature = keypair.sk.sign(message, None);

    // Encode as hex
    let public_key_hex = hex::encode(keypair.pk.as_slice());
    let message_hex = hex::encode(message);
    let signature_hex = hex::encode(signature.as_slice());

    // Create test data JSON
    let test_data = json!({
        "public_key": public_key_hex,
        "message": message_hex,
        "signature": signature_hex
    });

    // Write to file
    let output_path = PathBuf::from("examples/systems/ed25519-test-data.json");
    fs::create_dir_all(output_path.parent().unwrap())?;
    fs::write(&output_path, serde_json::to_string_pretty(&test_data)?)?;

    println!("✓ Generated Ed25519 test data");
    println!("  Public key: {}...", &public_key_hex[..16]);
    println!("  Message length: {} bytes", message.len());
    println!("  Signature: {}...", &signature_hex[..16]);
    println!("  Output: {}", output_path.display());

    // Also generate invalid signature for negative testing
    let mut invalid_signature = signature.as_slice().to_vec();
    invalid_signature[0] ^= 0xFF; // Flip bits to make invalid
    let invalid_signature_hex = hex::encode(&invalid_signature);

    let invalid_test_data = json!({
        "public_key": public_key_hex,
        "message": message_hex,
        "signature": invalid_signature_hex
    });

    let invalid_output_path = PathBuf::from("examples/systems/ed25519-test-data-invalid.json");
    fs::write(&invalid_output_path, serde_json::to_string_pretty(&invalid_test_data)?)?;

    println!("✓ Generated invalid signature test data");
    println!("  Output: {}", invalid_output_path.display());

    Ok(())
}

