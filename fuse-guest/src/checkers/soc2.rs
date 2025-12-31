//! SOC2 compliance checker for guest program

use serde_json::Value;
use crate::checker::ComplianceResult;

/// Check SOC2 compliance
#[must_use] 
pub fn check(spec: &Value, system_data: &Value) -> ComplianceResult {
    // Parse system data (expected to be JSON with access logs)
    let logs = match system_data.get("access_logs")
        .and_then(|v| v.as_array())
    {
        Some(logs) => logs,
        None => return ComplianceResult::Fail,
    };

    // Extract sampling constraint
    let sample_size = spec.get("constraints")
        .and_then(|c| c.get("sampling"))
        .and_then(|s| s.as_str())
        .and_then(|s| s.split_whitespace().last())
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1000);

    // Verify we have at least the required sample size
    if logs.len() < sample_size {
        return ComplianceResult::Fail;
    }

    // Check that all logs in the sample conform to control X
    // For MVP, we check that each log has required fields
    for log in logs.iter().take(sample_size) {
        if !log.is_object() {
            return ComplianceResult::Fail;
        }

        // Basic validation: ensure required fields exist
        // In production, this would be more sophisticated
        let obj = log.as_object().unwrap();
        if !obj.contains_key("timestamp") || !obj.contains_key("user") {
            return ComplianceResult::Fail;
        }
    }

    ComplianceResult::Pass
}

