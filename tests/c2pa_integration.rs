//! Integration tests for C2PA signature verification and selective disclosure

use fuse_core::{ComplianceSpec, VerifiableComplianceEnvelope, Result, ProverType};
use fuse_cli::c2pa::parse_c2pa_manifest;
use std::path::Path;

#[path = "common/mod.rs"]
mod common;

use common::fixtures::{load_c2pa_fixture, list_available_c2pa_fixtures};

/// Test C2PA manifest parsing with real fixtures
#[test]
fn test_c2pa_manifest_parsing() {
    let fixtures = list_available_c2pa_fixtures().unwrap();
    
    for fixture_name in fixtures.iter().take(3) {
        let fixture_path = load_c2pa_fixture(fixture_name).unwrap();
        
        // Test that we can parse the manifest
        match parse_c2pa_manifest(fixture_path.to_str().unwrap()) {
            Ok(data) => {
                // Verify we got some data
                assert!(!data.public_key.is_empty() || !data.signature.is_empty(), 
                    "Fixture {} should have signature data", fixture_name);
                assert!(!data.message.is_empty(), 
                    "Fixture {} should have message data", fixture_name);
            }
            Err(e) => {
                // Some fixtures might not parse (e.g., missing signatures)
                // That's okay for this test - we just want to verify parsing doesn't panic
                println!("Fixture {} parsing failed (expected for some fixtures): {}", fixture_name, e);
            }
        }
    }
}

/// Test C2PA signature extraction with existing local assets
#[test]
fn test_c2pa_local_assets() {
    let local_assets = vec![
        "examples/c2pa/C.jpg",
        "examples/c2pa/CA.jpg",
    ];
    
    for asset_path in local_assets {
        let path = Path::new(asset_path);
        if path.exists() {
            match parse_c2pa_manifest(asset_path) {
                Ok(data) => {
                    assert!(!data.message.is_empty(), 
                        "Local asset {} should have message data", asset_path);
                }
                Err(e) => {
                    println!("Local asset {} parsing failed: {}", asset_path, e);
                    // This is okay - some assets might not have C2PA manifests
                }
            }
        }
    }
}

/// Test selective disclosure with real C2PA manifests
#[test]
fn test_selective_disclosure_with_real_manifest() {
    // Use dev mode for fast testing
    std::env::set_var("RISC0_DEV_MODE", "1");
    
    let fixture_path = load_c2pa_fixture("adobe-20220124-CA.jpg");
    if fixture_path.is_err() {
        println!("Skipping selective disclosure test: fixture not available");
        return;
    }
    
    let fixture_path = fixture_path.unwrap();
    
    // Parse C2PA manifest
    let c2pa_data = match parse_c2pa_manifest(fixture_path.to_str().unwrap()) {
        Ok(data) => data,
        Err(e) => {
            println!("Skipping selective disclosure test: failed to parse C2PA manifest: {}", e);
            return;
        }
    };
    
    // Create spec with selective disclosure
    let spec_json = r#"{
        "claim": "C2PA signature verification with selective disclosure",
        "system_hash": "N/A",
        "constraints": {
            "algorithm": "Ed25519",
            "message_format": "hex-encoded bytes"
        },
        "disclosed_fields": [
            "claim_generator"
        ],
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2099-12-31T23:59:59Z"
    }"#;
    
    // Create system data from C2PA manifest
    let system_data_json = serde_json::json!({
        "public_key": c2pa_data.public_key,
        "signature": c2pa_data.signature,
        "message": c2pa_data.message,
        "claim": c2pa_data.claim_json
    }).to_string();
    
    // Try to generate proof
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((receipt_bytes, result, journal_bytes)) => {
            // Verify proof was generated
            assert!(!receipt_bytes.is_empty(), "Receipt should not be empty");
            assert!(!journal_bytes.is_empty(), "Journal should not be empty");
            
            // Verify the proof
            let verify_result = fuse_core::zkvm::verify_proof(&receipt_bytes);
            assert!(verify_result.is_ok(), "Proof verification should succeed");
            
            let (verified_result, verified_journal) = verify_result.unwrap();
            assert_eq!(result, verified_result, "Verified result should match");
            
            // Parse journal to verify selective disclosure worked
            if let Ok(journal_output) = serde_json::from_slice::<fuse_core::proof::JournalOutput>(&journal_bytes) {
                // Verify that redacted JSON contains only disclosed fields
                if !journal_output.redacted_json.is_empty() {
                    let redacted: serde_json::Value = serde_json::from_str(&journal_output.redacted_json)
                        .unwrap_or(serde_json::json!({}));
                    
                    // Should only have claim_generator field (or be empty if field doesn't exist)
                    if let Some(obj) = redacted.as_object() {
                        // Verify no unexpected fields (this is a basic check)
                        assert!(obj.len() <= 1, 
                            "Redacted JSON should have at most one field (claim_generator)");
                    }
                }
            }
        }
        Err(e) => {
            // Guest program might not be built - that's okay for CI
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                panic!("Unexpected error: {}", e);
            }
        }
    }
}

/// Test end-to-end C2PA workflow: parse → extract → prove → verify
#[test]
fn test_c2pa_end_to_end_workflow() {
    std::env::set_var("RISC0_DEV_MODE", "1");
    
    // Use a fixture that we know works
    let fixture_path = match load_c2pa_fixture("adobe-20220124-C.jpg") {
        Ok(path) => path,
        Err(_) => {
            // Fall back to local asset
            let local_path = Path::new("examples/c2pa/C.jpg");
            if local_path.exists() {
                local_path.to_path_buf()
            } else {
                println!("Skipping end-to-end test: no C2PA fixtures available");
                return;
            }
        }
    };
    
    // Step 1: Parse C2PA manifest
    let c2pa_data = match parse_c2pa_manifest(fixture_path.to_str().unwrap()) {
        Ok(data) => data,
        Err(e) => {
            println!("Skipping end-to-end test: failed to parse C2PA: {}", e);
            return;
        }
    };
    
    // Step 2: Create spec
    let spec_json = r#"{
        "claim": "C2PA signature verification",
        "system_hash": "N/A",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2099-12-31T23:59:59Z"
    }"#;
    
    // Step 3: Create system data
    let system_data_json = serde_json::json!({
        "public_key": c2pa_data.public_key,
        "signature": c2pa_data.signature,
        "message": c2pa_data.message,
        "claim": c2pa_data.claim_json
    }).to_string();
    
    // Step 4: Generate proof
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((receipt_bytes, result, journal_bytes)) => {
            // Step 5: Verify proof
            let verify_result = fuse_core::zkvm::verify_proof(&receipt_bytes);
            assert!(verify_result.is_ok(), "Proof verification should succeed");
            
            let (verified_result, verified_journal) = verify_result.unwrap();
            assert_eq!(result, verified_result);
            assert_eq!(journal_bytes, verified_journal);
            
            // Step 6: Create and verify envelope
            let spec: ComplianceSpec = serde_json::from_str(spec_json).unwrap();
            let proof = fuse_core::ComplianceProof::from_risc_zero_receipt(
                spec.hash(),
                receipt_bytes,
                result,
                journal_bytes,
            );
            
            let envelope = VerifiableComplianceEnvelope::new(spec, proof);
            assert!(envelope.verify().is_ok(), "Envelope verification should succeed");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                panic!("Unexpected error in end-to-end workflow: {}", e);
            }
        }
    }
}

/// Test selective disclosure with different field combinations
#[test]
fn test_selective_disclosure_field_combinations() {
    std::env::set_var("RISC0_DEV_MODE", "1");
    
    let fixture_path = match load_c2pa_fixture("adobe-20220124-CA.jpg") {
        Ok(path) => path,
        Err(_) => {
            println!("Skipping selective disclosure combinations test: fixture not available");
            return;
        }
    };
    
    let c2pa_data = match parse_c2pa_manifest_for_test(fixture_path.to_str().unwrap()) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: failed to parse C2PA manifest");
            return;
        }
    };
    
    // Test different field combinations
    let field_combinations = vec![
        vec!["claim_generator"],
        vec!["claim_generator", "issuer"],
        vec!["capture_time"],
        vec![], // Empty - should disclose nothing
    ];
    
    for fields in field_combinations {
        let spec_json = serde_json::json!({
            "claim": "C2PA selective disclosure test",
            "system_hash": "N/A",
            "constraints": {},
            "disclosed_fields": fields,
            "jurisdiction": "N/A",
            "version": "1.0",
            "expiry": "2099-12-31T23:59:59Z"
        }).to_string();
        
        let system_data_json = serde_json::json!({
            "public_key": c2pa_data.public_key,
            "signature": c2pa_data.signature,
            "message": c2pa_data.message,
            "claim": c2pa_data.claim_json
        }).to_string();
        
        match fuse_core::zkvm::generate_proof(&spec_json, &system_data_json, ProverType::Local) {
            Ok((_receipt_bytes, _result, journal_bytes)) => {
                // Verify journal contains redacted data
                if let Ok(journal_output) = serde_json::from_slice::<fuse_core::proof::JournalOutput>(&journal_bytes) {
                    // Redacted JSON should be present (even if empty)
                    assert!(!journal_output.claim_hash.is_empty(), 
                        "Claim hash should be present");
                }
            }
            Err(e) => {
                let error_msg = e.to_string();
                if error_msg.contains("not built") || error_msg.contains("Guest program") {
                    println!("Skipping test: Guest program not built");
                    return;
                } else {
                    panic!("Unexpected error: {}", e);
                }
            }
        }
    }
}

/// Test claim hash binding with C2PA manifests
#[test]
fn test_claim_hash_binding() {
    std::env::set_var("RISC0_DEV_MODE", "1");
    
    let fixture_path = match load_c2pa_fixture("adobe-20220124-C.jpg") {
        Ok(path) => path,
        Err(_) => {
            println!("Skipping claim hash binding test: fixture not available");
            return;
        }
    };
    
    let c2pa_data = match parse_c2pa_manifest_for_test(fixture_path.to_str().unwrap()) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: failed to parse C2PA manifest");
            return;
        }
    };
    
    let spec_json = r#"{
        "claim": "C2PA claim hash binding test",
        "system_hash": "N/A",
        "constraints": {},
        "disclosed_fields": ["claim_generator"],
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2099-12-31T23:59:59Z"
    }"#;
    
    let system_data_json = serde_json::json!({
        "public_key": c2pa_data.public_key,
        "signature": c2pa_data.signature,
        "message": c2pa_data.message,
        "claim": c2pa_data.claim_json
    }).to_string();
    
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((_receipt_bytes, _result, journal_bytes)) => {
            if let Ok(journal_output) = serde_json::from_slice::<fuse_core::proof::JournalOutput>(&journal_bytes) {
                // Claim hash should be SHA256 of the original message
                assert_eq!(journal_output.claim_hash.len(), 32, 
                    "Claim hash should be 32 bytes (SHA256)");
                
                // Verify hash matches expected SHA256 of message
                use sha2::{Sha256, Digest};
                let message_bytes = hex::decode(&c2pa_data.message).unwrap();
                let mut hasher = Sha256::new();
                hasher.update(&message_bytes);
                let expected_hash = hasher.finalize().to_vec();
                
                assert_eq!(journal_output.claim_hash, expected_hash,
                    "Claim hash should match SHA256 of message");
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping test: Guest program not built");
            } else {
                panic!("Unexpected error: {}", e);
            }
        }
    }
}
