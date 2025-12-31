//! Ed25519 signature verification checker (host-side)
//!
//! This is a simple host-side implementation for validation.
//! The actual zkVM proof uses the guest program implementation.

use fuse_core::{ComplianceSpec, ComplianceResult, Result};
use crate::ComplianceChecker;
use serde_json::Value;
use ed25519_compact::{PublicKey, Signature};

pub struct Ed25519Checker;

impl ComplianceChecker for Ed25519Checker {
    fn check(&self, _spec: &ComplianceSpec, system_data: &str) -> Result<ComplianceResult> {
        let data: Value = serde_json::from_str(system_data)
            .map_err(|e| fuse_core::VceError::InvalidSpec(
                format!("Failed to parse system data: {e}")
            ))?;

        // Extract public key, message, and signature
        let public_key_hex = data.get("public_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| fuse_core::VceError::InvalidSpec(
                "Missing 'public_key' field".to_string()
            ))?;

        let message_hex = data.get("message")
            .and_then(|v| v.as_str())
            .ok_or_else(|| fuse_core::VceError::InvalidSpec(
                "Missing 'message' field".to_string()
            ))?;

        let signature_hex = data.get("signature")
            .and_then(|v| v.as_str())
            .ok_or_else(|| fuse_core::VceError::InvalidSpec(
                "Missing 'signature' field".to_string()
            ))?;

        // Decode hex
        let public_key_bytes = hex::decode(public_key_hex)
            .map_err(|e| fuse_core::VceError::InvalidSpec(
                format!("Invalid public_key hex: {e}")
            ))?;

        let message_bytes = hex::decode(message_hex)
            .map_err(|e| fuse_core::VceError::InvalidSpec(
                format!("Invalid message hex: {e}")
            ))?;

        let signature_bytes = hex::decode(signature_hex)
            .map_err(|e| fuse_core::VceError::InvalidSpec(
                format!("Invalid signature hex: {e}")
            ))?;

        // Validate lengths
        if public_key_bytes.len() != 32 {
            return Err(fuse_core::VceError::InvalidSpec(
                "Public key must be 32 bytes".to_string()
            ));
        }
        if signature_bytes.len() != 64 {
            return Err(fuse_core::VceError::InvalidSpec(
                "Signature must be 64 bytes".to_string()
            ));
        }

        // Parse and verify
        let public_key = PublicKey::from_slice(&public_key_bytes)
            .map_err(|e| fuse_core::VceError::InvalidSpec(
                format!("Invalid public key: {e}")
            ))?;

        let signature = Signature::from_slice(&signature_bytes)
            .map_err(|e| fuse_core::VceError::InvalidSpec(
                format!("Invalid signature: {e}")
            ))?;

        // Verify signature
        match public_key.verify(&message_bytes, &signature) {
            Ok(()) => Ok(ComplianceResult::Pass),
            Err(_) => Ok(ComplianceResult::Fail),
        }
    }
}

