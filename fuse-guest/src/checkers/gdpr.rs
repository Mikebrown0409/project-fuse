//! GDPR compliance checker for guest program

use serde_json::Value;
use crate::checker::ComplianceResult;

/// Check GDPR data residency compliance
#[must_use] 
pub fn check(spec: &Value, system_data: &Value) -> ComplianceResult {
    // Extract required region from constraints
    let required_region = match spec.get("constraints")
        .and_then(|c| c.get("data_region"))
        .and_then(|v| v.as_str())
    {
        Some(region) => region,
        None => return ComplianceResult::Fail,
    };

    // Check data storage locations
    let storage_locations = match system_data.get("storage_locations")
        .and_then(|v| v.as_array())
    {
        Some(locations) => locations,
        None => return ComplianceResult::Fail,
    };

    // Verify all storage is in the required region
    for location in storage_locations {
        let region = match location.get("region")
            .and_then(|v| v.as_str())
        {
            Some(region) => region,
            None => return ComplianceResult::Fail,
        };

        if region != required_region {
            return ComplianceResult::Fail;
        }
    }

    ComplianceResult::Pass
}

