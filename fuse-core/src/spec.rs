//! Compliance specification definitions

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use chrono::{DateTime, Utc};

/// Compliance specification that defines what needs to be verified
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSpec {
    /// Human-readable claim description
    pub claim: String,

    /// SHA256 hash of the system binary/config being verified
    pub system_hash: String,

    /// Constraints and parameters for the compliance check
    pub constraints: BTreeMap<String, String>,

    /// Jurisdiction or regulatory framework (e.g., "US, SEC", "EU, GDPR")
    pub jurisdiction: String,

    /// Specification version
    pub version: String,

    /// Expiry date for this specification
    pub expiry: DateTime<Utc>,

    /// Optional metadata
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,

    /// Optional list of top-level JSON fields to disclose in the proof journal.
    /// Used for selective disclosure (e.g., in C2PA manifest verification).
    #[serde(default)]
    pub disclosed_fields: Option<Vec<String>>,
}

impl ComplianceSpec {
    /// Create a new compliance specification
    #[must_use] 
    pub fn new(
        claim: String,
        system_hash: String,
        constraints: BTreeMap<String, String>,
        jurisdiction: String,
        version: String,
        expiry: DateTime<Utc>,
    ) -> Self {
        Self {
            claim,
            system_hash,
            constraints,
            jurisdiction,
            version,
            expiry,
            metadata: BTreeMap::new(),
            disclosed_fields: None,
        }
    }

    /// Compute the hash of this specification
    /// Uses `BTreeMap` for deterministic key ordering
    #[must_use] 
    pub fn hash(&self) -> String {
        let json = serde_json::to_string(self).expect("Failed to serialize spec");
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Check if the specification has expired
    #[must_use] 
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiry
    }

    /// Validate the specification
    pub fn validate(&self) -> crate::Result<()> {
        if self.claim.is_empty() {
            return Err(crate::VceError::InvalidSpec("Claim cannot be empty".to_string()));
        }

        if self.system_hash.is_empty() {
            return Err(crate::VceError::InvalidSpec(
                "System hash cannot be empty".to_string(),
            ));
        }

        if self.is_expired() {
            return Err(crate::VceError::SpecExpired(
                self.expiry.to_rfc3339(),
            ));
        }

        Ok(())
    }

    /// Load a specification from a JSON file
    pub fn from_json_file(path: &std::path::Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let spec: Self = serde_json::from_str(&content)?;
        spec.validate()?;
        Ok(spec)
    }

    /// Load a specification from a YAML file
    pub fn from_yaml_file(path: &std::path::Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let spec: Self = serde_yaml::from_str(&content)?;
        spec.validate()?;
        Ok(spec)
    }

    /// Save the specification to a JSON file
    pub fn to_json_file(&self, path: &std::path::Path) -> crate::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_validation() {
        let spec = ComplianceSpec::new(
            "SOC2 control X verified".to_string(),
            "abc123".to_string(),
            BTreeMap::new(),
            "US, SEC".to_string(),
            "1.0".to_string(),
            Utc::now() + chrono::Duration::days(365),
        );

        assert!(spec.validate().is_ok());
    }

    #[test]
    fn test_spec_expiry() {
        let spec = ComplianceSpec::new(
            "Test claim".to_string(),
            "abc123".to_string(),
            BTreeMap::new(),
            "US".to_string(),
            "1.0".to_string(),
            Utc::now() - chrono::Duration::days(1),
        );

        assert!(spec.is_expired());
        assert!(spec.validate().is_err());
    }
}

