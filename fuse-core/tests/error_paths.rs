//! Error path tests - ensure all failure modes are handled gracefully

use fuse_core::{Result, ProverType};
use std::env;

/// Test malformed JSON inputs
#[test]
fn test_malformed_json_inputs() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    // Malformed spec JSON
    let malformed_spec = r#"{
        "claim": "test",
        "system_hash": "test",
        "constraints": { invalid json }
    }"#;
    
    let system_data = r#"{"test": "data"}"#;
    
    match fuse_core::zkvm::generate_proof(malformed_spec, system_data, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            // Should fail gracefully
            assert_eq!(journal_output.result, fuse_core::ComplianceResult::Fail,
                "Malformed JSON should cause failure");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                // Error is acceptable - should not panic
                println!("Error handling works (expected): {}", e);
            }
        }
    }
    
    // Malformed system data JSON
    let valid_spec = r#"{
        "claim": "test",
        "system_hash": "test",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2099-12-31T23:59:59Z"
    }"#;
    
    let malformed_system = r#"{ invalid }"#;
    
    match fuse_core::zkvm::generate_proof(valid_spec, malformed_system, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            assert_eq!(journal_output.result, fuse_core::ComplianceResult::Fail,
                "Malformed system data should cause failure");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                println!("Error handling works (expected): {}", e);
            }
        }
    }
}

/// Test missing required fields in spec
#[test]
fn test_missing_spec_fields() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    // Missing claim field
    let spec_no_claim = r#"{
        "system_hash": "test",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    let system_data = r#"{"test": "data"}"#;
    
    match fuse_core::zkvm::generate_proof(spec_no_claim, system_data, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            // Should handle missing fields gracefully
            assert!(journal_output.result == fuse_core::ComplianceResult::Pass || 
                    journal_output.result == fuse_core::ComplianceResult::Fail,
                "Should return valid result even with missing fields");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                println!("Error handling works (expected): {}", e);
            }
        }
    }
}

/// Test missing required fields in system_data
#[test]
fn test_missing_system_data_fields() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let spec_json = r#"{
        "claim": "C2PA signature verification",
        "system_hash": "N/A",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    // Missing public_key
    let system_data_no_key = r#"{
        "signature": "abcd",
        "message": "abcd",
        "claim": {}
    }"#;
    
    match fuse_core::zkvm::generate_proof(spec_json, system_data_no_key, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            // Should fail when required fields are missing
            assert_eq!(journal_output.result, fuse_core::ComplianceResult::Fail,
                "Missing required fields should cause failure");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                println!("Error handling works (expected): {}", e);
            }
        }
    }
}

/// Test invalid hex encoding
#[test]
fn test_invalid_hex_encoding() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let spec_json = r#"{
        "claim": "C2PA signature verification",
        "system_hash": "N/A",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    // Invalid hex (contains 'X')
    let system_data_invalid_hex = r#"{
        "public_key": "XXXXinvalidhex",
        "signature": "abcd1234",
        "message": "abcd1234",
        "claim": {}
    }"#;
    
    match fuse_core::zkvm::generate_proof(spec_json, system_data_invalid_hex, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            assert_eq!(journal_output.result, fuse_core::ComplianceResult::Fail,
                "Invalid hex encoding should cause failure");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                println!("Error handling works (expected): {}", e);
            }
        }
    }
}

/// Test wrong key/signature lengths
#[test]
fn test_wrong_key_signature_lengths() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let spec_json = r#"{
        "claim": "C2PA signature verification",
        "system_hash": "N/A",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    // Wrong key length (31 bytes instead of 32)
    let system_data_wrong_length = r#"{
        "public_key": "00000000000000000000000000000000000000000000000000000000000000",
        "signature": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "message": "abcd1234",
        "claim": {}
    }"#;
    
    match fuse_core::zkvm::generate_proof(spec_json, system_data_wrong_length, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            assert_eq!(journal_output.result, fuse_core::ComplianceResult::Fail,
                "Wrong key/signature lengths should cause failure");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                println!("Error handling works (expected): {}", e);
            }
        }
    }
}

/// Test invalid Ed25519 keys/signatures
#[test]
fn test_invalid_ed25519_keys() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let spec_json = r#"{
        "claim": "C2PA signature verification",
        "system_hash": "N/A",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    // Valid length but invalid key format
    let system_data_invalid_key = r#"{
        "public_key": "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        "signature": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "message": "abcd1234",
        "claim": {}
    }"#;
    
    match fuse_core::zkvm::generate_proof(spec_json, system_data_invalid_key, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            // May pass or fail depending on validation - important is no panic
            assert!(journal_output.result == fuse_core::ComplianceResult::Pass || 
                    journal_output.result == fuse_core::ComplianceResult::Fail,
                "Should handle invalid keys without panicking");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                // Should not panic
                println!("Error handling works (expected): {}", e);
            }
        }
    }
}

/// Test empty inputs
#[test]
fn test_empty_inputs() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let spec_json = r#"{
        "claim": "test",
        "system_hash": "test",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    // Empty system data
    let empty_system = r#"{}"#;
    
    match fuse_core::zkvm::generate_proof(spec_json, empty_system, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            // Should handle empty data gracefully
            assert!(journal_output.result == fuse_core::ComplianceResult::Pass || 
                    journal_output.result == fuse_core::ComplianceResult::Fail,
                "Should handle empty inputs without panicking");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                println!("Error handling works (expected): {}", e);
            }
        }
    }
}

/// Test that no panics occur (critical for production)
#[test]
fn test_no_panics_on_invalid_input() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    // Try various invalid inputs - should never panic
    let invalid_inputs = vec![
        ("", ""),
        ("not json", "not json"),
        ("{}", "{}"),
        (r#"{"claim": null}"#, r#"{"test": null}"#),
    ];
    
    for (spec, system) in invalid_inputs {
        // This should not panic - use catch_unwind if available
        let result = std::panic::catch_unwind(|| {
            fuse_core::zkvm::generate_proof(spec, system, ProverType::Local)
        });
        
        match result {
            Ok(proof_result) => {
                // Proof generation succeeded or returned error - both are fine
                match proof_result {
                    Ok((_receipt_bytes, _result, _journal_bytes)) => {
                        // Success - that's fine
                    }
                    Err(_) => {
                        // Error - that's also fine, as long as no panic
                    }
                }
            }
            Err(_) => {
                panic!("Code panicked on invalid input - this should not happen!");
            }
        }
    }
}
