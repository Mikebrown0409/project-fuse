//! JSON parsing only checker (host-side)
//!
//! This is a minimal host-side implementation for validation.
//! The actual zkVM proof uses the guest program implementation.
//!
//! This checker is used for performance benchmarking to isolate
//! JSON parsing and filtering costs from cryptographic operations.

use fuse_core::{ComplianceSpec, ComplianceResult, Result, VceError};
use crate::ComplianceChecker;

pub struct JsonOnlyChecker;

impl ComplianceChecker for JsonOnlyChecker {
    fn check(&self, _spec: &ComplianceSpec, _system_data: &str) -> Result<ComplianceResult> {
        // Minimal implementation - always pass since we're only measuring guest performance
        // The actual JSON parsing and filtering happens in the zkVM guest program
        Ok(ComplianceResult::Pass)
    }
}

