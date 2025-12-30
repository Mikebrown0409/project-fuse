//! ML model usage constraint checker for guest program

use serde_json::Value;
use alloc::vec::Vec;
use crate::checker::ComplianceResult;

/// Check ML model usage constraint compliance
pub fn check(spec: &Value, system_data: &Value) -> ComplianceResult {
    // Extract usage constraints
    let max_usage = spec.get("constraints")
        .and_then(|c| c.get("max_usage"))
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(u64::MAX);

    let allowed_domains: Vec<&str> = spec.get("constraints")
        .and_then(|c| c.get("allowed_domains"))
        .and_then(|v| v.as_str())
        .map(|s| s.split(',').map(|d| d.trim()).collect())
        .unwrap_or_default();

    // Check model usage
    let usage_logs = match system_data.get("usage_logs")
        .and_then(|v| v.as_array())
    {
        Some(logs) => logs,
        None => return ComplianceResult::Fail,
    };

    // Check total usage count
    if usage_logs.len() as u64 > max_usage {
        return ComplianceResult::Fail;
    }

    // Check domain restrictions if specified
    if !allowed_domains.is_empty() {
        for log in usage_logs {
            let domain = match log.get("domain")
                .and_then(|v| v.as_str())
            {
                Some(domain) => domain,
                None => return ComplianceResult::Fail,
            };

            if !allowed_domains.contains(&domain) {
                return ComplianceResult::Fail;
            }
        }
    }

    ComplianceResult::Pass
}

