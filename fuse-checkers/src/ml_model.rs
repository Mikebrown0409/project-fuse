//! ML model usage constraint checker

use fuse_core::{ComplianceSpec, ComplianceResult, Result};
use crate::ComplianceChecker;
use serde_json::Value;

pub struct MlModelChecker;

impl ComplianceChecker for MlModelChecker {
    fn check(&self, spec: &ComplianceSpec, system_data: &str) -> Result<ComplianceResult> {
        let data: Value = serde_json::from_str(system_data)
            .map_err(|e| fuse_core::VceError::InvalidSpec(
                format!("Failed to parse system data: {e}")
            ))?;

        // Extract usage constraints
        let max_usage = spec.constraints
            .get("max_usage")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(u64::MAX);

        let allowed_domains = spec.constraints
            .get("allowed_domains")
            .map(|s| s.split(',').map(str::trim).collect::<Vec<_>>())
            .unwrap_or_default();

        // Check model usage
        let usage_logs = data.get("usage_logs")
            .and_then(|v| v.as_array())
            .ok_or_else(|| fuse_core::VceError::InvalidSpec(
                "System data must contain 'usage_logs' array".to_string()
            ))?;

        // Check total usage count
        if usage_logs.len() as u64 > max_usage {
            return Ok(ComplianceResult::Fail);
        }

        // Check domain restrictions if specified
        if !allowed_domains.is_empty() {
            for log in usage_logs {
                let domain = log.get("domain")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| fuse_core::VceError::InvalidSpec(
                        "Each usage log must have a 'domain' field".to_string()
                    ))?;

                if !allowed_domains.contains(&domain) {
                    return Ok(ComplianceResult::Fail);
                }
            }
        }

        Ok(ComplianceResult::Pass)
    }
}

