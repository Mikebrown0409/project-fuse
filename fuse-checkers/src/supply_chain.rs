//! Supply chain provenance checker

use fuse_core::{ComplianceSpec, ComplianceResult, Result};
use crate::ComplianceChecker;
use serde_json::Value;

pub struct SupplyChainChecker;

impl ComplianceChecker for SupplyChainChecker {
    fn check(&self, spec: &ComplianceSpec, system_data: &str) -> Result<ComplianceResult> {
        let data: Value = serde_json::from_str(system_data)
            .map_err(|e| fuse_core::VceError::InvalidSpec(
                format!("Failed to parse system data: {}", e)
            ))?;

        // Extract required provenance chain
        let required_chain = spec.constraints
            .get("provenance_chain")
            .ok_or_else(|| fuse_core::VceError::InvalidSpec(
                "Supply chain checker requires 'provenance_chain' constraint".to_string()
            ))?;

        // Check components
        let components = data.get("components")
            .and_then(|v| v.as_array())
            .ok_or_else(|| fuse_core::VceError::InvalidSpec(
                "System data must contain 'components' array".to_string()
            ))?;

        // Verify each component has valid provenance
        for component in components {
            let provenance = component.get("provenance")
                .and_then(|v| v.as_str())
                .ok_or_else(|| fuse_core::VceError::InvalidSpec(
                    "Each component must have a 'provenance' field".to_string()
                ))?;

            // Basic validation: check that provenance matches required chain
            // In production, this would verify cryptographic signatures
            if !provenance.contains(required_chain) {
                return Ok(ComplianceResult::Fail);
            }
        }

        Ok(ComplianceResult::Pass)
    }
}

