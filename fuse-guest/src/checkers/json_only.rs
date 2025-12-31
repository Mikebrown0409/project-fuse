//! JSON parsing and selective disclosure checker (guest program)
//!
//! This checker is a micro-test that:
//! 1. Parses C2PA claim JSON from system data
//! 2. Performs selective disclosure filtering
//! 3. Skips all signature verification (for performance isolation)
//!
//! Purpose: Measure the cost of JSON parsing and filtering operations
//! independently of cryptographic verification.

use serde_json::{Value, Map};
use crate::checker::{ComplianceResult, JournalOutput};
use alloc::vec::Vec;
use alloc::string::String;

#[must_use] 
pub fn check(spec: &Value, system_data: &Value) -> JournalOutput {
    // Extract claim JSON (skip all cryptographic operations)
    let claim_json = match system_data.get("claim") {
        Some(v) => v,
        None => return JournalOutput { 
            result: ComplianceResult::Fail, 
            claim_hash: Vec::new(), 
            redacted_json: String::new() 
        },
    };

    // Filter fields based on disclosed_fields in spec
    let mut disclosed_json = Map::new();
    
    if let Some(fields) = spec.get("disclosed_fields").and_then(|v| v.as_array()) {
        if let Some(claim_map) = claim_json.as_object() {
            for field in fields {
                if let Some(field_name) = field.as_str() {
                    // Top-level only redaction
                    if let Some(val) = claim_map.get(field_name) {
                        disclosed_json.insert(field_name.into(), val.clone());
                    }
                    // Silent skip if field missing (as per Phase 2 decision)
                }
            }
        }
    }

    // Serialize the redacted JSON to a string for the journal
    let redacted_json_str = serde_json::to_string(&Value::Object(disclosed_json))
        .unwrap_or_else(|_| String::new());
    
    // Return with empty claim_hash (no signature verification performed)
    JournalOutput {
        result: ComplianceResult::Pass,
        claim_hash: Vec::new(), // Empty - no hash calculated
        redacted_json: redacted_json_str,
    }
}

