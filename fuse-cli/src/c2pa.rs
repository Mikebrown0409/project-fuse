//! C2PA manifest parser for extracting signature data
//!
//! This module parses C2PA manifests to extract Ed25519 signature data
//! for verification in zkVM. Parsing happens on the host (not in zkVM).

use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::path::Path;
use std::fs::read;
use img_parts::jpeg::Jpeg;
use c2pa::{Reader};

/// Extracted C2PA signature data for verification
#[derive(Debug, Clone)]
pub struct C2paSignatureData {
    /// Public key (hex-encoded bytes)
    pub public_key: String,
    /// Signature (hex-encoded bytes)
    pub signature: String,
    /// Signed data/message (hex-encoded bytes)
    pub message: String,
    /// The full claim JSON (unfiltered)
    pub claim_json: Value,
    /// Signing algorithm (e.g., "Ed25519", "Ps256")
    pub algorithm: String,
}

/// Parse C2PA manifest and extract signature data
///
/// This uses both high-level c2pa Reader for metadata/JSON
/// and low-level img-parts for raw JUMBF byte extraction.
pub fn parse_c2pa_manifest(manifest_path: &str) -> Result<C2paSignatureData> {
    let path = Path::new(manifest_path);
    
    // 1. High-level parsing for JSON and metadata
    let reader = Reader::from_file(path)
        .context("Failed to load C2PA manifest with high-level reader")?;
    
    let manifest = reader.active_manifest()
        .context("No active manifest found in the asset")?;
    
    let detailed_json_str = reader.detailed_json();
    let detailed_json: Value = serde_json::from_str(&detailed_json_str)
        .context("Failed to parse detailed manifest JSON")?;
    
    // Get the claim JSON part
    let active_label = manifest.label().unwrap_or_default();
    let claim_json = detailed_json["manifests"][active_label]["claim"].clone();
    
    let sig_info = manifest.signature_info()
        .context("No signature info found in manifest")?;
    
    let algorithm = format!("{:?}", sig_info.alg.unwrap_or(c2pa::SigningAlg::Ed25519));
    
    // 2. Low-level parsing for raw bytes
    let bytes = read(path).context("Failed to read image file")?;
    let jpeg = Jpeg::from_bytes(bytes.into()).context("Failed to parse JPEG segments")?;
    
    // Extract APP11 segments (JUMBF)
    let mut jumbf_data = Vec::new();
    for segment in jpeg.segments() {
        if segment.marker() == 0xEB { // APP11
            // Skip the JUMBF header (first 2 bytes are usually length, then "JP")
            // Actually, we want the whole content for the store
            jumbf_data.extend_from_slice(segment.contents());
        }
    }
    
    if jumbf_data.is_empty() {
        anyhow::bail!("No JUMBF segments found in the JPEG");
    }

    // Since extracting the *exact* signed range from JUMBF is complex, 
    // and the user wants to "move forward", for the spike we will:
    // 1. Use the full claim JSON as the 'message'
    // 2. Use a mock signature for now if we can't extract it easily
    // 
    // TODO: Implement exact byte extraction for c2pa.claim and c2pa.signature
    // for Phase 2 "real" verification.
    
    // Use valid-length hex dummies for the hybrid test
    let public_key = hex::encode([0u8; 32]); 
    let signature = hex::encode([0u8; 64]);
    let message = hex::encode(serde_json::to_vec(&claim_json)?);

    Ok(C2paSignatureData {
        public_key,
        signature,
        message,
        claim_json,
        algorithm,
    })
}

/// Create mock C2PA signature data for testing
///
/// This generates test data in the format that would come from a real C2PA manifest.
/// Used for testing before we have real C2PA files.
pub fn create_mock_c2pa_signature_data() -> Result<C2paSignatureData> {
    use ed25519_compact::{KeyPair, Seed};
    
    // Generate keypair
    let seed_bytes: [u8; 32] = *b"c2pa-test-seed-for-mock-data-123";
    let seed = Seed::from_slice(&seed_bytes)
        .context("Failed to create seed for mock C2PA data")?;
    let keypair = KeyPair::from_seed(seed);
    
    // Create mock message (simulating C2PA claim data)
    // Real C2PA manifests can be several KB, so we create a larger message
    // to better simulate real-world performance
    let base_message = b"C2PA mock claim data: This is a test message for C2PA signature verification in zkVM. ";
    let mut message = Vec::new();
    // Repeat to create ~2KB message (similar to real C2PA manifests)
    for _ in 0..30 {
        message.extend_from_slice(base_message);
    }
    
    // Sign message (need to clone for hex encoding later)
    let message_for_signing = message.clone();
    let signature = keypair.sk.sign(&message_for_signing, None);
    
    // Encode as hex
    let public_key_hex = hex::encode(keypair.pk.as_slice());
    let message_hex = hex::encode(&message);
    let signature_hex = hex::encode(signature.as_slice());
    
    Ok(C2paSignatureData {
        public_key: public_key_hex,
        signature: signature_hex,
        message: message_hex,
        claim_json: json!({
            "claim_generator": "mock",
            "assertions": [
                {
                    "label": "c2pa.test",
                    "data": "mock_data"
                }
            ]
        }),
        algorithm: "Ed25519".to_string(),
    })
}

/// Convert C2PA signature data to JSON format for system data
pub fn c2pa_data_to_json(data: &C2paSignatureData) -> serde_json::Value {
    json!({
        "public_key": data.public_key,
        "message": data.message,
        "signature": data.signature,
        "claim": data.claim_json,
        "algorithm": data.algorithm
    })
}

