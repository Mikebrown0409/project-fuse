//! FUSE Integration Tests Package
//!
//! This package contains all integration tests for FUSE.
//! Tests are organized by functionality:
//! - C2PA integration tests
//! - Checker integration tests  
//! - Error path tests
//! - Tamper detection tests

// Re-export test modules so they can be discovered by Cargo
pub mod c2pa_integration;
pub mod checkers_integration;
pub mod error_paths;
pub mod c2pa_tamper;
pub mod zkvm_proofs;
pub mod checkers;
pub mod tamper_helpers;
