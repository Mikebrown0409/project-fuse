//! Helper functions for tampering with C2PA assets and test data

use std::path::Path;
use std::fs;
use anyhow::Result;

/// Tamper with signature bytes in a C2PA asset
/// 
/// This corrupts the signature by flipping bits, making verification fail.
pub fn tamper_signature(asset_path: &Path) -> Result<Vec<u8>> {
    let mut bytes = fs::read(asset_path)?;
    
    // Find and corrupt signature bytes
    // For JPEG files, we look for APP11 segments (JUMBF) and corrupt them
    // This is a simplified approach - in reality, we'd parse the JUMBF structure
    
    // Simple approach: flip bits in the last 64 bytes (where signature might be)
    if bytes.len() > 64 {
        let start = bytes.len() - 64;
        for byte in bytes.iter_mut().skip(start) {
            *byte ^= 0xFF; // Flip all bits
        }
    }
    
    Ok(bytes)
}

/// Tamper with hash in C2PA manifest data
/// 
/// This modifies the claim hash, making verification fail.
pub fn tamper_hash(data: &[u8]) -> Vec<u8> {
    let mut tampered = data.to_vec();
    
    // Find and corrupt hash-like sequences (32-byte sequences that look like SHA256)
    // This is a heuristic approach
    for i in 0..tampered.len().saturating_sub(32) {
        // Check if this looks like a hash (has some entropy)
        let slice = &tampered[i..i+32];
        let unique_bytes = slice.iter().collect::<std::collections::HashSet<_>>().len();
        
        if unique_bytes > 16 {
            // Likely a hash - corrupt it
            for byte in tampered.iter_mut().skip(i).take(32) {
                *byte ^= 0xAA; // Flip pattern
            }
            break;
        }
    }
    
    tampered
}

/// Tamper with manifest JSON
/// 
/// This modifies the JSON structure, making it invalid or changing values.
pub fn tamper_manifest_json(json_str: &str) -> Result<String> {
    let mut value: serde_json::Value = serde_json::from_str(json_str)?;
    
    // Modify a field in the JSON
    if let Some(obj) = value.as_object_mut() {
        // Change claim_generator or add a malicious field
        if let Some(claim_gen) = obj.get("claim_generator") {
            if claim_gen.is_string() {
                obj.insert("claim_generator".to_string(), serde_json::json!("TAMPERED_VALUE"));
            }
        } else {
            obj.insert("tampered".to_string(), serde_json::json!(true));
        }
    }
    
    Ok(serde_json::to_string(&value)?)
}

/// Remove signature from C2PA data
/// 
/// This creates data without a signature, which should fail verification.
pub fn remove_signature_from_data(system_data_json: &str) -> Result<String> {
    let mut value: serde_json::Value = serde_json::from_str(system_data_json)?;
    
    if let Some(obj) = value.as_object_mut() {
        // Remove signature field
        obj.remove("signature");
        // Or set it to empty
        obj.insert("signature".to_string(), serde_json::json!(""));
    }
    
    Ok(serde_json::to_string(&value)?)
}

/// Corrupt hex encoding
/// 
/// This creates invalid hex strings that should fail decoding.
pub fn corrupt_hex_encoding(hex_str: &str) -> String {
    // Insert invalid characters
    let mut corrupted = hex_str.to_string();
    if corrupted.len() > 10 {
        corrupted.insert(5, 'X'); // Insert non-hex character
    }
    corrupted
}

/// Create invalid Ed25519 key (wrong length)
pub fn create_invalid_key_length() -> String {
    // Return a key that's not 32 bytes when decoded
    hex::encode(vec![0u8; 31]) // 31 bytes instead of 32
}

/// Create invalid Ed25519 signature (wrong length)
pub fn create_invalid_signature_length() -> String {
    // Return a signature that's not 64 bytes when decoded
    hex::encode(vec![0u8; 63]) // 63 bytes instead of 64
}
