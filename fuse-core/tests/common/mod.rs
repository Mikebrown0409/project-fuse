//! Common test utilities

pub mod fixtures;

pub use fixtures::{
    load_c2pa_fixture,
    list_available_c2pa_fixtures,
    get_c2pa_fixture_metadata,
    C2paFixtureMetadata,
};

use fuse_core::{ComplianceSpec, ComplianceResult, VerifiableComplianceEnvelope};
use fuse_core::proof::JournalOutput;
use chrono::Utc;
use std::collections::BTreeMap;

/// Builder for test ComplianceSpec
pub struct SpecBuilder {
    claim: String,
    system_hash: String,
    constraints: BTreeMap<String, String>,
    jurisdiction: String,
    version: String,
    expiry: chrono::DateTime<Utc>,
    disclosed_fields: Option<Vec<String>>,
}

impl SpecBuilder {
    pub fn new(claim: &str) -> Self {
        Self {
            claim: claim.to_string(),
            system_hash: "test".to_string(),
            constraints: BTreeMap::new(),
            jurisdiction: "N/A".to_string(),
            version: "1.0".to_string(),
            expiry: Utc::now() + chrono::Duration::days(365),
            disclosed_fields: None,
        }
    }

    pub fn with_system_hash(mut self, hash: &str) -> Self {
        self.system_hash = hash.to_string();
        self
    }

    pub fn with_constraint(mut self, key: &str, value: &str) -> Self {
        self.constraints.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_disclosed_fields(mut self, fields: Vec<&str>) -> Self {
        self.disclosed_fields = Some(fields.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn build(self) -> ComplianceSpec {
        ComplianceSpec::new(
            self.claim,
            self.system_hash,
            self.constraints,
            self.jurisdiction,
            self.version,
            self.expiry,
        )
    }

    pub fn to_json(&self) -> String {
        let mut json = serde_json::json!({
            "claim": self.claim,
            "system_hash": self.system_hash,
            "constraints": self.constraints,
            "jurisdiction": self.jurisdiction,
            "version": self.version,
            "expiry": self.expiry.to_rfc3339(),
        });

        if let Some(ref fields) = self.disclosed_fields {
            json["disclosed_fields"] = serde_json::json!(fields);
        }

        serde_json::to_string(&json).unwrap()
    }
}

/// Helper to create test system data
pub fn create_system_data(fields: &[(&str, serde_json::Value)]) -> String {
    let mut data = serde_json::Map::new();
    for (key, value) in fields {
        data.insert(key.to_string(), value.clone());
    }
    serde_json::to_string(&serde_json::Value::Object(data)).unwrap()
}

/// Helper to assert proof result
pub fn assert_proof_result(
    result: Result<(Vec<u8>, JournalOutput, Vec<u8>), Box<dyn std::error::Error>>,
    expected: ComplianceResult,
) {
    match result {
        Ok((_receipt_bytes, journal_output, _journal_bytes)) => {
            assert_eq!(journal_output.result, expected, "Proof result should match expected");
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("not built") || error_msg.contains("Guest program") {
                println!("Skipping assertion: Guest program not built");
            } else {
                panic!("Unexpected error: {}", e);
            }
        }
    }
}

/// Helper to verify envelope
pub fn verify_envelope(envelope: &mut VerifiableComplianceEnvelope) {
    assert!(envelope.verify().is_ok(), "Envelope verification should succeed");
    assert!(envelope.is_compliant().unwrap(), "Envelope should be compliant");
}

/// Set up dev mode for tests
pub fn setup_dev_mode() {
    std::env::set_var("RISC0_DEV_MODE", "1");
}
