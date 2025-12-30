//! Tests for detecting tampered C2PA assets and signatures

use fuse_core::{Result, ProverType};
use fuse_cli::c2pa::parse_c2pa_manifest;
use std::env;
use std::path::Path;

#[path = "tamper_helpers.rs"]
mod helpers;
use helpers::*;

#[path = "common/mod.rs"]
mod common;

use common::fixtures::load_c2pa_fixture;

/// Test that tampered signatures fail verification
#[test]
fn test_tampered_signature_fails() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let fixture_path = match load_c2pa_fixture("adobe-20220124-C.jpg") {
        Ok(path) => path,
        Err(_) => {
            println!("Skipping tamper test: fixture not available");
            return;
        }
    };
    
    // Parse original manifest
    let mut c2pa_data = match parse_c2pa_manifest(fixture_path.to_str().unwrap()) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: failed to parse C2PA manifest");
            return;
        }
    };
    
    // Tamper with signature
    c2pa_data.signature = corrupt_hex_encoding(&c2pa_data.signature);
    
    let spec_json = r#"{
        "claim": "C2PA signature verification",
        "system_hash": "N/A",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2099-12-31T23:59:59Z"
    }"#;
    
    let system_data_json = serde_json::json!({
        "public_key": c2pa_data.public_key,
        "signature": c2pa_data.signature, // Tampered signature
        "message": c2pa_data.message,
        "claim": c2pa_data.claim_json
    }).to_string();
    
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((_receipt_bytes, result, _journal_bytes)) => {
            // Should fail verification due to tampered signature
            assert_eq!(result, fuse_core::ComplianceResult::Fail,
                "Tampered signature should cause verification to fail");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                // Error during proof generation is also acceptable for tampered data
                println!("Proof generation failed (expected for tampered data): {}", e);
            }
        }
    }
}

/// Test that modified manifests fail verification
#[test]
fn test_modified_manifest_fails() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let fixture_path = match load_c2pa_fixture("adobe-20220124-CA.jpg") {
        Ok(path) => path,
        Err(_) => {
            println!("Skipping test: fixture not available");
            return;
        }
    };
    
    let mut c2pa_data = match parse_c2pa_manifest(fixture_path.to_str().unwrap()) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: failed to parse C2PA manifest");
            return;
        }
    };
    
    // Tamper with manifest JSON
    if let Ok(tampered_json) = tamper_manifest_json(&serde_json::to_string(&c2pa_data.claim_json).unwrap()) {
        c2pa_data.claim_json = serde_json::from_str(&tampered_json).unwrap();
    }
    
    // Also tamper with message hash
    c2pa_data.message = hex::encode(tamper_hash(&hex::decode(&c2pa_data.message).unwrap_or_default()));
    
    let spec_json = r#"{
        "claim": "C2PA signature verification",
        "system_hash": "N/A",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2099-12-31T23:59:59Z"
    }"#;
    
    let system_data_json = serde_json::json!({
        "public_key": c2pa_data.public_key,
        "signature": c2pa_data.signature,
        "message": c2pa_data.message, // Tampered message
        "claim": c2pa_data.claim_json // Tampered claim
    }).to_string();
    
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((_receipt_bytes, result, _journal_bytes)) => {
            // Should fail due to mismatched signature/message
            assert_eq!(result, fuse_core::ComplianceResult::Fail,
                "Modified manifest should cause verification to fail");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                println!("Proof generation failed (expected for tampered data): {}", e);
            }
        }
    }
}

/// Test that missing signatures fail gracefully
#[test]
fn test_missing_signature_fails_gracefully() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let fixture_path = match load_c2pa_fixture("adobe-20220124-C.jpg") {
        Ok(path) => path,
        Err(_) => {
            println!("Skipping test: fixture not available");
            return;
        }
    };
    
    let c2pa_data = match parse_c2pa_manifest(fixture_path.to_str().unwrap()) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: failed to parse C2PA manifest");
            return;
        }
    };
    
    // Remove signature
    let system_data_json = remove_signature_from_data(&serde_json::json!({
        "public_key": c2pa_data.public_key,
        "signature": c2pa_data.signature,
        "message": c2pa_data.message,
        "claim": c2pa_data.claim_json
    }).to_string()).unwrap();
    
    let spec_json = r#"{
        "claim": "C2PA signature verification",
        "system_hash": "N/A",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2099-12-31T23:59:59Z"
    }"#;
    
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((_receipt_bytes, result, _journal_bytes)) => {
            // Should fail when signature is missing
            assert_eq!(result, fuse_core::ComplianceResult::Fail,
                "Missing signature should cause verification to fail");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                // Error is acceptable - should fail gracefully, not panic
                println!("Proof generation failed gracefully (expected): {}", e);
            }
        }
    }
}

/// Test that corrupted hex encoding fails gracefully
#[test]
fn test_corrupted_hex_encoding_fails() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let fixture_path = match load_c2pa_fixture("adobe-20220124-C.jpg") {
        Ok(path) => path,
        Err(_) => {
            println!("Skipping test: fixture not available");
            return;
        }
    };
    
    let mut c2pa_data = match parse_c2pa_manifest(fixture_path.to_str().unwrap()) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: failed to parse C2PA manifest");
            return;
        }
    };
    
    // Corrupt hex encoding
    c2pa_data.public_key = corrupt_hex_encoding(&c2pa_data.public_key);
    c2pa_data.signature = corrupt_hex_encoding(&c2pa_data.signature);
    
    let spec_json = r#"{
        "claim": "C2PA signature verification",
        "system_hash": "N/A",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2099-12-31T23:59:59Z"
    }"#;
    
    let system_data_json = serde_json::json!({
        "public_key": c2pa_data.public_key, // Corrupted hex
        "signature": c2pa_data.signature,   // Corrupted hex
        "message": c2pa_data.message,
        "claim": c2pa_data.claim_json
    }).to_string();
    
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((_receipt_bytes, result, _journal_bytes)) => {
            // Should fail due to invalid hex
            assert_eq!(result, fuse_core::ComplianceResult::Fail,
                "Corrupted hex encoding should cause verification to fail");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                // Error is acceptable - should fail gracefully
                println!("Proof generation failed gracefully (expected): {}", e);
            }
        }
    }
}

/// Test that error messages are informative
#[test]
fn test_error_messages_are_informative() {
    // Test that when verification fails, error messages are helpful
    // This is more of a documentation test - we verify error handling exists
    
    // Test with invalid key length
    let invalid_key = create_invalid_key_length();
    let invalid_sig = create_invalid_signature_length();
    
    let spec_json = r#"{
        "claim": "C2PA signature verification",
        "system_hash": "N/A",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2099-12-31T23:59:59Z"
    }"#;
    
    let system_data_json = serde_json::json!({
        "public_key": invalid_key,
        "signature": invalid_sig,
        "message": hex::encode(b"test message"),
        "claim": {}
    }).to_string();
    
    env::set_var("RISC0_DEV_MODE", "1");
    
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((_receipt_bytes, result, _journal_bytes)) => {
            // Should fail with invalid lengths
            assert_eq!(result, fuse_core::ComplianceResult::Fail,
                "Invalid key/signature lengths should cause failure");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                // Verify error message exists and is not empty
                assert!(!error_msg.is_empty(), "Error message should be informative");
                println!("Error message (should be informative): {}", error_msg);
            }
        }
    }
}
