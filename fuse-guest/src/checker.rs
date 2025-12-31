//! Generic checker execution in zkVM
//!
//! This module provides the generic checker framework that reads
//! spec and system data, parses them, and executes checker logic.

use risc0_zkvm::guest::env;
use alloc::string::String;
use alloc::vec::Vec;
use serde_json::Value;
use serde::{Serialize, Deserialize};

/// Result of a compliance check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceResult {
    Pass = 0,
    Fail = 1,
}

/// The complete output committed to the journal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalOutput {
    pub result: ComplianceResult,
    // Use empty vec instead of Option to avoid serialization issues
    pub claim_hash: Vec<u8>,
    // Serialize JSON as string to avoid RISC Zero journal format issues
    pub redacted_json: String,
}

/// Execute a compliance check
/// 
/// Reads spec and system data from host, parses JSON, and executes
/// the appropriate checker based on the claim type.
#[must_use] 
pub fn execute_checker() -> JournalOutput {
    // Read inputs from host
    let spec_json: String = env::read();
    let system_data_json: String = env::read();
    
    // Parse JSON inputs
    let spec: Value = match serde_json::from_str(&spec_json) {
        Ok(v) => v,
        Err(_) => return JournalOutput { result: ComplianceResult::Fail, claim_hash: Vec::new(), redacted_json: String::new() },
    };
    
    let system_data: Value = match serde_json::from_str(&system_data_json) {
        Ok(v) => v,
        Err(_) => return JournalOutput { result: ComplianceResult::Fail, claim_hash: Vec::new(), redacted_json: String::new() },
    };
    
    // Extract claim type from spec
    let claim = spec.get("claim")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    
    // Route to appropriate checker based on claim
    if claim.contains("JSON parsing only") {
        crate::checkers::json_only::check(&spec, &system_data)
    } else if claim.contains("C2PA") || claim.contains("C2PA signature") {
        crate::checkers::c2pa::check(&spec, &system_data)
    } else if claim.contains("SOC2") {
        let result = crate::checkers::soc2::check(&spec, &system_data);
        JournalOutput { result, claim_hash: Vec::new(), redacted_json: String::new() }
    } else if claim.contains("GDPR") {
        let result = crate::checkers::gdpr::check(&spec, &system_data);
        JournalOutput { result, claim_hash: Vec::new(), redacted_json: String::new() }
    } else if claim.contains("Supply chain") || claim.contains("provenance") {
        let result = crate::checkers::supply_chain::check(&spec, &system_data);
        JournalOutput { result, claim_hash: Vec::new(), redacted_json: String::new() }
    } else if claim.contains("ML model") || claim.contains("usage constraint") {
        let result = crate::checkers::ml_model::check(&spec, &system_data);
        JournalOutput { result, claim_hash: Vec::new(), redacted_json: String::new() }
    } else if claim.contains("Ed25519") || claim.contains("signature verification") {
        let result = crate::checkers::ed25519::check(&spec, &system_data);
        JournalOutput { result, claim_hash: Vec::new(), redacted_json: String::new() }
    } else {
        // Default: basic validation
        if !spec_json.is_empty() && !system_data_json.is_empty() {
            JournalOutput { result: ComplianceResult::Pass, claim_hash: Vec::new(), redacted_json: String::new() }
        } else {
            JournalOutput { result: ComplianceResult::Fail, claim_hash: Vec::new(), redacted_json: String::new() }
        }
    }
}
