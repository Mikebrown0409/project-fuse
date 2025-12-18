//! SOC2 compliance checker

use fuse_core::{ComplianceSpec, ComplianceResult, Result};
use crate::ComplianceChecker;
use serde_json::Value;

pub struct Soc2ControlXChecker;

impl ComplianceChecker for Soc2ControlXChecker {
    fn check(&self, spec: &ComplianceSpec, system_data: &str) -> Result<ComplianceResult> {
        // Parse system data (expected to be JSON with access logs)
        let data: Value = serde_json::from_str(system_data)
            .map_err(|e| fuse_core::VceError::InvalidSpec(
                format!("Failed to parse system data: {}", e)
            ))?;

        // Extract sampling constraint
        let sample_size = spec.constraints
            .get("sampling")
            .and_then(|s| s.split_whitespace().last())
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1000);

        // Check if we have access logs
        let logs = data.get("access_logs")
            .and_then(|v| v.as_array())
            .ok_or_else(|| fuse_core::VceError::InvalidSpec(
                "System data must contain 'access_logs' array".to_string()
            ))?;

        // Verify we have at least the required sample size
        if logs.len() < sample_size {
            return Ok(ComplianceResult::Fail);
        }

        // Check that all logs in the sample conform to control X
        // For MVP, we check that each log has required fields
        for log in logs.iter().take(sample_size) {
            if !log.is_object() {
                return Ok(ComplianceResult::Fail);
            }

            // Basic validation: ensure required fields exist
            // In production, this would be more sophisticated
            let obj = log.as_object().unwrap();
            if !obj.contains_key("timestamp") || !obj.contains_key("user") {
                return Ok(ComplianceResult::Fail);
            }
        }

        Ok(ComplianceResult::Pass)
    }
}

