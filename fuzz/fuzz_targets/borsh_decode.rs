#![no_main]
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};

// Duplicate JournalOutput and ComplianceResult to avoid pulling in fuse-core (RISC Zero deps)
// This matches fuse-core/src/proof.rs exactly
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ComplianceResult {
    Pass = 0,
    Fail = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JournalOutput {
    result: ComplianceResult,
    claim_hash: Vec<u8>,
    redacted_json: String,
}

// Fuzz target for Borsh deserialization
// Tests Borsh deserialization used by RISC Zero for journal decoding.
// This helps catch issues with:
// - Malformed journal data
// - Buffer overflows
// - Memory safety issues
fuzz_target!(|data: &[u8]| {
    // Try deserializing - should handle errors gracefully
    let _ = bincode::deserialize::<JournalOutput>(data);
    
    // Also test with serde_json since we use that for some structures
    if let Ok(json_str) = std::str::from_utf8(data) {
        let _ = serde_json::from_str::<JournalOutput>(json_str);
    }
});
