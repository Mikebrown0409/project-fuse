#![no_main]
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use chrono::{DateTime, Utc};

// Duplicate ComplianceSpec struct to avoid pulling in fuse-core (RISC Zero deps)
// This matches fuse-core/src/spec.rs exactly
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ComplianceSpec {
    claim: String,
    system_hash: String,
    constraints: BTreeMap<String, String>,
    jurisdiction: String,
    version: String,
    expiry: DateTime<Utc>,
    #[serde(default)]
    metadata: BTreeMap<String, String>,
    #[serde(default)]
    disclosed_fields: Option<Vec<String>>,
}

// Fuzz target for compliance spec validation
// Tests that spec parsing and validation handles malformed inputs gracefully:
// - Invalid JSON
// - Missing required fields
// - Invalid field types
// - Malformed dates/timestamps
// - Extremely large inputs
fuzz_target!(|data: &[u8]| {
    // Try to parse as JSON spec
    if let Ok(json_value) = serde_json::from_slice::<serde_json::Value>(data) {
        // Try to deserialize as ComplianceSpec
        let _ = serde_json::from_value::<ComplianceSpec>(json_value);
    }
    
    // Also test as string input
    if let Ok(json_str) = std::str::from_utf8(data) {
        let _ = serde_json::from_str::<ComplianceSpec>(json_str);
    }
    
    // If parsing fails, that's expected - we just want no panics
});
