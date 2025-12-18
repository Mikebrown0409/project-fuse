//! Compliance checkers for various standards

use fuse_core::{ComplianceSpec, ComplianceResult, Result};
use std::collections::HashMap;

pub mod soc2;
pub mod gdpr;
pub mod supply_chain;
pub mod ml_model;

/// Trait for compliance checkers
pub trait ComplianceChecker {
    /// Check compliance against a specification
    fn check(&self, spec: &ComplianceSpec, system_data: &str) -> Result<ComplianceResult>;
}

/// Registry of available checkers
pub struct CheckerRegistry {
    checkers: HashMap<String, Box<dyn ComplianceChecker>>,
}

impl CheckerRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            checkers: HashMap::new(),
        };

        // Register built-in checkers
        registry.register("SOC2 control X verified".to_string(), Box::new(soc2::Soc2ControlXChecker));
        registry.register("SOC2 control".to_string(), Box::new(soc2::Soc2ControlXChecker));
        registry.register("GDPR data residency".to_string(), Box::new(gdpr::GdprDataResidencyChecker));
        registry.register("GDPR".to_string(), Box::new(gdpr::GdprDataResidencyChecker));
        registry.register("Supply chain provenance".to_string(), Box::new(supply_chain::SupplyChainChecker));
        registry.register("ML model usage constraint".to_string(), Box::new(ml_model::MlModelChecker));

        registry
    }

    pub fn register(&mut self, claim_pattern: String, checker: Box<dyn ComplianceChecker>) {
        self.checkers.insert(claim_pattern, checker);
    }

    pub fn get_checker(&self, claim: &str) -> Result<&dyn ComplianceChecker> {
        // Try exact match first
        if let Some(checker) = self.checkers.get(claim) {
            return Ok(checker.as_ref());
        }

        // Try partial match
        for (pattern, checker) in &self.checkers {
            if claim.contains(pattern) || pattern.contains(claim) {
                return Ok(checker.as_ref());
            }
        }

        Err(fuse_core::VceError::InvalidSpec(
            format!("No checker found for claim: {}", claim),
        ))
    }
}

impl Default for CheckerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

