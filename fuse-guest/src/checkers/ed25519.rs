//! Ed25519 signature verification checker for guest program
//!
//! This is a minimal Ed25519 signature verification implementation
//! for benchmarking performance impact in zkVM.

use serde_json::Value;
use crate::checker::ComplianceResult;
use ed25519_dalek::{VerifyingKey, Signature};

/// Check Ed25519 signature verification
///
/// Expected system_data format:
/// {
///   "public_key": "<hex-encoded 32-byte public key>",
///   "message": "<hex-encoded message bytes>",
///   "signature": "<hex-encoded 64-byte signature>"
/// }
pub fn check(_spec: &Value, system_data: &Value) -> ComplianceResult {
    // Extract public key, message, and signature from system_data
    let public_key_hex = match system_data.get("public_key")
        .and_then(|v| v.as_str())
    {
        Some(key) => key,
        None => return ComplianceResult::Fail,
    };

    let message_hex = match system_data.get("message")
        .and_then(|v| v.as_str())
    {
        Some(msg) => msg,
        None => return ComplianceResult::Fail,
    };

    let signature_hex = match system_data.get("signature")
        .and_then(|v| v.as_str())
    {
        Some(sig) => sig,
        None => return ComplianceResult::Fail,
    };

    // Decode hex strings to bytes
    let public_key_bytes = match hex::decode(public_key_hex) {
        Ok(bytes) => bytes,
        Err(_) => return ComplianceResult::Fail,
    };

    let message_bytes = match hex::decode(message_hex) {
        Ok(bytes) => bytes,
        Err(_) => return ComplianceResult::Fail,
    };

    let signature_bytes = match hex::decode(signature_hex) {
        Ok(bytes) => bytes,
        Err(_) => return ComplianceResult::Fail,
    };

    // Validate lengths
    if public_key_bytes.len() != 32 {
        return ComplianceResult::Fail;
    }
    if signature_bytes.len() != 64 {
        return ComplianceResult::Fail;
    }

    // Parse public key and signature (ed25519-dalek API)
    // Convert slices to fixed-size arrays
    let public_key_array: [u8; 32] = match public_key_bytes.try_into() {
        Ok(arr) => arr,
        Err(_) => return ComplianceResult::Fail,
    };
    
    let signature_array: [u8; 64] = match signature_bytes.try_into() {
        Ok(arr) => arr,
        Err(_) => return ComplianceResult::Fail,
    };

    let public_key = match VerifyingKey::from_bytes(&public_key_array) {
        Ok(key) => key,
        Err(_) => return ComplianceResult::Fail,
    };

    // ed25519-dalek Signature::from_bytes doesn't return Result
    let signature = Signature::from_bytes(&signature_array);

    // Verify signature (ed25519-dalek uses verify_strict for better security)
    match public_key.verify_strict(&message_bytes, &signature) {
        Ok(_) => ComplianceResult::Pass,
        Err(_) => ComplianceResult::Fail,
    }
}

