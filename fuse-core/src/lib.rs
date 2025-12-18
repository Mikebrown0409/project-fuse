//! Core library for Verifiable Compliance Envelope (VCE) Protocol
//!
//! This module provides the fundamental data structures and operations
//! for creating and verifying compliance envelopes.

pub mod spec;
pub mod envelope;
pub mod proof;
pub mod error;

pub use spec::ComplianceSpec;
pub use envelope::VerifiableComplianceEnvelope;
pub use proof::{ComplianceProof, ComplianceResult};
pub use error::{VceError, Result};

