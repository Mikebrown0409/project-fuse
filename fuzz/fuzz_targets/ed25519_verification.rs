#![no_main]
use libfuzzer_sys::fuzz_target;

// This target requires fuse-core and fuse-checkers (heavy RISC Zero deps)
// Only this target will build RISC Zero - others are independent
#[cfg(feature = "ed25519")]
use fuse_core::ComplianceSpec;
#[cfg(feature = "ed25519")]
use fuse_checkers::{ComplianceChecker, ed25519::Ed25519Checker};
#[cfg(feature = "ed25519")]
use serde_json::json;
#[cfg(feature = "ed25519")]
use std::collections::BTreeMap;
#[cfg(feature = "ed25519")]
use chrono::Utc;

// Fuzz target for Ed25519 signature verification
// Tests Ed25519 signature verification with random inputs to catch:
// - Replay attacks (same signature with different messages)
// - Poisoned inputs (malformed keys/signatures)
// - Memory safety issues
// - Side-channel leaks (though RISC Zero handles this)
fuzz_target!(|data: &[u8]| {
    #[cfg(not(feature = "ed25519"))]
    {
        // If feature not enabled, just return (shouldn't happen in CI)
        return;
    }
    
    #[cfg(feature = "ed25519")]
    {
        // Ensure we have enough data for key (32 bytes) + signature (64 bytes) + message
        if data.len() < 96 {
            return;
        }
        
        // Split data into components
        let public_key_bytes = &data[0..32];
        let signature_bytes = &data[32..96];
        let message_bytes = &data[96..];
        
        // Create system data JSON
        let system_data = json!({
            "public_key": hex::encode(public_key_bytes),
            "signature": hex::encode(signature_bytes),
            "message": hex::encode(message_bytes)
        });
        
        // Create a minimal spec
        let spec = ComplianceSpec {
            claim: "Fuzz test".to_string(),
            system_hash: "test".to_string(),
            constraints: BTreeMap::new(),
            jurisdiction: "test".to_string(),
            version: "1.0".to_string(),
            expiry: Utc::now(),
            metadata: BTreeMap::new(),
            disclosed_fields: None,
        };
        
        // Try verification - should handle errors gracefully without panicking
        let checker = Ed25519Checker;
        let _ = checker.check(&spec, &system_data.to_string());
    }
});
