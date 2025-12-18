//! GDPR compliance checker

use fuse_core::{ComplianceSpec, ComplianceResult, Result};
use crate::ComplianceChecker;
use serde_json::Value;

pub struct GdprDataResidencyChecker;

impl ComplianceChecker for GdprDataResidencyChecker {
    fn check(&self, spec: &ComplianceSpec, system_data: &str) -> Result<ComplianceResult> {
        let data: Value = serde_json::from_str(system_data)
            .map_err(|e| fuse_core::VceError::InvalidSpec(
                format!("Failed to parse system data: {}", e)
            ))?;

        // Extract required region from constraints
        let required_region = spec.constraints
            .get("data_region")
            .ok_or_else(|| fuse_core::VceError::InvalidSpec(
                "GDPR checker requires 'data_region' constraint".to_string()
            ))?;

        // Check data storage locations
        let storage_locations = data.get("storage_locations")
            .and_then(|v| v.as_array())
            .ok_or_else(|| fuse_core::VceError::InvalidSpec(
                "System data must contain 'storage_locations' array".to_string()
            ))?;

        // Verify all storage is in the required region
        for location in storage_locations {
            let region = location.get("region")
                .and_then(|v| v.as_str())
                .ok_or_else(|| fuse_core::VceError::InvalidSpec(
                    "Each storage location must have a 'region' field".to_string()
                ))?;

            if region != required_region {
                return Ok(ComplianceResult::Fail);
            }
        }

        Ok(ComplianceResult::Pass)
    }
}

