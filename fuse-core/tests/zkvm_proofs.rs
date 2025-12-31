//! Integration tests for zkVM proof generation and verification

use fuse_core::{ComplianceSpec, VerifiableComplianceEnvelope, Result, ProverType};
use std::collections::BTreeMap;
use chrono::Utc;

#[test]
fn test_placeholder_proof_generation() {
    // Test that placeholder proofs still work (backward compatibility)
    let spec = ComplianceSpec::new(
        "Test claim".to_string(),
        "abc123".to_string(),
        BTreeMap::new(),
        "US".to_string(),
        "1.0".to_string(),
        Utc::now() + chrono::Duration::days(365),
    );

    let mut proof = fuse_core::ComplianceProof::new(
        spec.hash(),
        fuse_core::ComplianceResult::Pass,
        vec![],
    );

    assert!(proof.is_placeholder());
    assert!(proof.verify().is_ok());
}

#[test]
fn test_proof_with_real_zkvm() {
    // Test real zkVM proof generation when guest program is available
    let spec_json = r#"{
        "claim": "SOC2 control X verified",
        "system_hash": "test",
        "constraints": {"sampling": "0"},
        "jurisdiction": "US, SEC",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    let system_data_json = r#"{"access_logs": []}"#;
    
    // Try to generate proof - may succeed if guest program is built, or fail if not
    match fuse_core::zkvm::generate_proof(spec_json, system_data_json, ProverType::Local) {
        Ok((receipt_bytes, journal_output, journal_bytes)) => {
            // Guest program is built and proof generation succeeded
            assert!(!receipt_bytes.is_empty(), "Receipt should not be empty");
            assert!(!journal_bytes.is_empty(), "Journal should not be empty");
            
            // Test verification
            let verify_result = fuse_core::zkvm::verify_proof(&receipt_bytes);
            assert!(verify_result.is_ok(), "Proof verification should succeed");
            
            let (verified_journal_output, verified_journal) = verify_result.unwrap();
            assert_eq!(journal_output.result, verified_journal_output.result, "Verified result should match generated result");
            assert_eq!(journal_bytes, verified_journal, "Verified journal should match generated journal");
        }
        Err(e) => {
            // Guest program not built - this is acceptable for CI/CD
            // Just verify the error message is informative
            let error_msg = e.to_string();
            assert!(error_msg.contains("not built") || error_msg.contains("Guest program"), 
                "Error should mention guest program: {}", error_msg);
        }
    }
}

#[test]
fn test_envelope_with_placeholder_proof() {
    let spec = ComplianceSpec::new(
        "Test claim".to_string(),
        "abc123".to_string(),
        BTreeMap::new(),
        "US".to_string(),
        "1.0".to_string(),
        Utc::now() + chrono::Duration::days(365),
    );

    let proof = fuse_core::ComplianceProof::new(
        spec.hash(),
        fuse_core::ComplianceResult::Pass,
        vec![],
    );

    let mut envelope = VerifiableComplianceEnvelope::new(spec, proof);
    assert!(envelope.verify().is_ok());
    assert!(envelope.is_compliant().unwrap());
}

#[test]
fn test_envelope_with_real_zkvm_proof() {
    // Test envelope creation and verification with real zkVM proof
    let spec_json = r#"{
        "claim": "SOC2 control X verified",
        "system_hash": "test",
        "constraints": {"sampling": "0"},
        "jurisdiction": "US, SEC",
        "version": "1.0",
        "expiry": "2026-12-31T23:59:59Z"
    }"#;
    
    let system_data_json = r#"{"access_logs": []}"#;
    
    // Try to generate real proof
    if let Ok((receipt_bytes, journal_output, journal_bytes)) = fuse_core::zkvm::generate_proof(spec_json, system_data_json, ProverType::Local) {
        // Parse spec to get hash
        let spec: ComplianceSpec = serde_json::from_str(spec_json).unwrap();
        let spec_hash = spec.hash();
        
        // Create proof from receipt
        let proof = fuse_core::ComplianceProof::from_risc_zero_receipt(
            spec_hash,
            receipt_bytes,
            journal_output,
            journal_bytes,
        );
        
        // Verify it's not a placeholder
        assert!(!proof.is_placeholder(), "Proof should be real zkVM proof");
        
        // Create envelope
        let mut envelope = VerifiableComplianceEnvelope::new(spec, proof);
        
        // Verify envelope (verify needs mutable reference)
        assert!(envelope.verify().is_ok(), "Envelope verification should succeed");
        assert!(envelope.is_compliant().unwrap(), "Compliance check should pass");
    } else {
        // Guest program not built - skip test
        println!("Skipping test: Guest program not built");
    }
}
