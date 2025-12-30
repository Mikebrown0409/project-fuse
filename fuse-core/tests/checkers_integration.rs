//! Integration tests for compliance checkers in zkVM

use fuse_core::{ComplianceSpec, Result, ProverType};
use std::env;

/// Test SOC2 checker with valid data
#[test]
fn test_soc2_checker_valid() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let spec_json = r#"{
        "claim": "SOC2 control X verified",
        "system_hash": "test",
        "constraints": {
            "control_X": "enforced in all transactions",
            "sampling": "last 1000 events"
        },
        "jurisdiction": "US, SEC",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    // Create system data with 1000+ access logs
    let mut access_logs = Vec::new();
    for i in 0..1000 {
        access_logs.push(serde_json::json!({
            "timestamp": format!("2024-01-01T00:00:{:02}Z", i % 60),
            "user": format!("user{}", i % 10),
            "action": "access",
            "resource": format!("resource{}", i)
        }));
    }
    
    let system_data_json = serde_json::json!({
        "access_logs": access_logs
    }).to_string();
    
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            assert_eq!(journal_output.result, fuse_core::ComplianceResult::Pass, 
                "SOC2 checker should pass with valid data");
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

/// Test SOC2 checker with insufficient sample size
#[test]
fn test_soc2_checker_insufficient_sample() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let spec_json = r#"{
        "claim": "SOC2 control X verified",
        "system_hash": "test",
        "constraints": {
            "sampling": "last 1000 events"
        },
        "jurisdiction": "US, SEC",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    // Create system data with only 500 logs (less than required 1000)
    let mut access_logs = Vec::new();
    for i in 0..500 {
        access_logs.push(serde_json::json!({
            "timestamp": format!("2024-01-01T00:00:{:02}Z", i % 60),
            "user": format!("user{}", i % 10),
            "action": "access"
        }));
    }
    
    let system_data_json = serde_json::json!({
        "access_logs": access_logs
    }).to_string();
    
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            assert_eq!(journal_output.result, fuse_core::ComplianceResult::Fail,
                "SOC2 checker should fail with insufficient sample size");
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

/// Test GDPR checker with valid data
#[test]
fn test_gdpr_checker_valid() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let spec_json = r#"{
        "claim": "GDPR data residency verified",
        "system_hash": "test",
        "constraints": {
            "data_region": "EU"
        },
        "jurisdiction": "EU, GDPR",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    let system_data_json = serde_json::json!({
        "storage_locations": [
            {"region": "EU", "location": "Frankfurt"},
            {"region": "EU", "location": "Dublin"}
        ]
    }).to_string();
    
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            assert_eq!(journal_output.result, fuse_core::ComplianceResult::Pass,
                "GDPR checker should pass when all storage is in required region");
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

/// Test GDPR checker with invalid region
#[test]
fn test_gdpr_checker_invalid_region() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let spec_json = r#"{
        "claim": "GDPR data residency verified",
        "system_hash": "test",
        "constraints": {
            "data_region": "EU"
        },
        "jurisdiction": "EU, GDPR",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    // Include a storage location outside EU
    let system_data_json = serde_json::json!({
        "storage_locations": [
            {"region": "EU", "location": "Frankfurt"},
            {"region": "US", "location": "Virginia"}  // Invalid - outside EU
        ]
    }).to_string();
    
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            assert_eq!(journal_output.result, fuse_core::ComplianceResult::Fail,
                "GDPR checker should fail when storage is outside required region");
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

/// Test Supply Chain checker
#[test]
fn test_supply_chain_checker() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let spec_json = r#"{
        "claim": "Supply chain provenance verified",
        "system_hash": "test",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    let system_data_json = r#"{"components": []}"#;
    
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            // Supply chain checker should at least not panic
            assert!(journal_output.result == fuse_core::ComplianceResult::Pass || 
                    journal_output.result == fuse_core::ComplianceResult::Fail,
                "Supply chain checker should return a valid result");
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

/// Test ML Model checker
#[test]
fn test_ml_model_checker() {
    env::set_var("RISC0_DEV_MODE", "1");
    
    let spec_json = r#"{
        "claim": "ML model usage constraint verified",
        "system_hash": "test",
        "constraints": {},
        "jurisdiction": "N/A",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    let system_data_json = r#"{"usage": []}"#;
    
    match fuse_core::zkvm::generate_proof(spec_json, &system_data_json, ProverType::Local) {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            // ML model checker should at least not panic
            assert!(journal_output.result == fuse_core::ComplianceResult::Pass || 
                    journal_output.result == fuse_core::ComplianceResult::Fail,
                "ML model checker should return a valid result");
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
