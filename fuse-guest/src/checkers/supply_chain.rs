//! Supply chain provenance checker for guest program

use serde_json::Value;
use crate::checker::ComplianceResult;

/// Check supply chain provenance compliance
#[must_use] 
pub fn check(spec: &Value, system_data: &Value) -> ComplianceResult {
    // Extract required provenance chain
    let required_chain = match spec.get("constraints")
        .and_then(|c| c.get("provenance_chain"))
        .and_then(|v| v.as_str())
    {
        Some(chain) => chain,
        None => return ComplianceResult::Fail,
    };

    // Check components
    let components = match system_data.get("components")
        .and_then(|v| v.as_array())
    {
        Some(components) => components,
        None => return ComplianceResult::Fail,
    };

    // Verify each component has valid provenance
    for component in components {
        let provenance = match component.get("provenance")
            .and_then(|v| v.as_str())
        {
            Some(provenance) => provenance,
            None => return ComplianceResult::Fail,
        };

        // Basic validation: check that provenance matches required chain
        // In production, this would verify cryptographic signatures
        if !provenance.contains(required_chain) {
            return ComplianceResult::Fail;
        }
    }

    ComplianceResult::Pass
}

