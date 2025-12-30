//! Verifiable Compliance Envelope (VCE) structure

use serde::{Deserialize, Serialize};
use crate::spec::ComplianceSpec;
use crate::proof::ComplianceProof;
use crate::error::{VceError, Result};

/// Verifiable Compliance Envelope - the complete artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableComplianceEnvelope {
    /// The compliance specification that was verified
    pub spec: ComplianceSpec,

    /// The zero-knowledge proof
    pub proof: ComplianceProof,

    /// Optional signature (for future use)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl VerifiableComplianceEnvelope {
    /// Create a new VCE
    pub fn new(spec: ComplianceSpec, proof: ComplianceProof) -> Self {
        Self {
            spec,
            proof,
            signature: None,
        }
    }

    /// Verify the entire envelope
    pub fn verify(&mut self) -> Result<()> {
        // Validate the specification
        self.spec.validate()?;

        // Verify the proof matches the spec
        let spec_hash = self.spec.hash();
        if self.proof.spec_hash != spec_hash {
            return Err(VceError::ProofVerificationFailed(
                format!("Proof spec hash does not match specification hash. Expected: {}, Got: {}", 
                    spec_hash, self.proof.spec_hash)
            ));
        }

        // Verify the proof itself
        self.proof.verify()?;

        Ok(())
    }

    /// Check if the envelope represents a passing compliance check
    pub fn is_compliant(&mut self) -> Result<bool> {
        self.verify()?;
        Ok(self.proof.result == crate::proof::ComplianceResult::Pass)
    }

    /// Load a VCE from a file
    pub fn from_file(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let envelope: Self = serde_json::from_str(&content)?;
        Ok(envelope)
    }

    /// Save the VCE to a file
    pub fn to_file(&self, path: &std::path::Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::ComplianceSpec;
    use crate::proof::{ComplianceProof, ComplianceResult};
    use std::collections::BTreeMap;

    #[test]
    fn test_envelope_creation() {
        let spec = ComplianceSpec::new(
            "Test claim".to_string(),
            "abc123".to_string(),
            BTreeMap::new(),
            "US".to_string(),
            "1.0".to_string(),
            chrono::Utc::now() + chrono::Duration::days(365),
        );

        let proof = ComplianceProof::new(
            spec.hash(),
            ComplianceResult::Pass,
            vec![],
        );

        let mut envelope = VerifiableComplianceEnvelope::new(spec, proof);
        assert!(envelope.verify().is_ok());
    }
}

