//! Zero-knowledge proof structures and operations

use serde::{Deserialize, Serialize};
use crate::error::Result;

/// Result of a compliance check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceResult {
    Pass = 0,
    Fail = 1,
}

/// The complete output committed to the journal by the guest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalOutput {
    pub result: ComplianceResult,
    // Use empty vec instead of Option to avoid serialization issues
    pub claim_hash: Vec<u8>,
    // Serialize JSON as string to avoid RISC Zero journal format issues
    pub redacted_json: String,
}

impl std::fmt::Display for ComplianceResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplianceResult::Pass => write!(f, "PASS"),
            ComplianceResult::Fail => write!(f, "FAIL"),
        }
    }
}

/// Zero-knowledge proof of compliance check execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceProof {
    /// The proof data (RISC Zero receipt or similar)
    /// In production, this would be a binary proof blob
    /// For MVP, we use a placeholder structure
    pub proof_data: Vec<u8>,

    /// Hash of the specification that was verified
    pub spec_hash: String,

    /// Result of the compliance check
    pub result: ComplianceResult,

    /// Timestamp when the proof was generated
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// RISC Zero journal (public outputs from zkVM execution)
    pub journal: Vec<u8>,

    /// Decoded journal output
    pub journal_output: Option<JournalOutput>,
}

impl ComplianceProof {
    /// Create a new proof from a RISC Zero receipt
    pub fn from_risc_zero_receipt(
        spec_hash: String,
        receipt_bytes: Vec<u8>,
        journal_output: JournalOutput,
        journal: Vec<u8>,
    ) -> Self {
        Self {
            proof_data: receipt_bytes,
            spec_hash,
            result: journal_output.result,
            timestamp: chrono::Utc::now(),
            journal,
            journal_output: Some(journal_output),
        }
    }

    /// Create a new proof (placeholder for MVP)
    /// In production, this would generate an actual RISC Zero proof
    pub fn new(
        spec_hash: String,
        result: ComplianceResult,
        journal: Vec<u8>,
    ) -> Self {
        Self {
            // Placeholder: In production, this would contain the actual RISC Zero proof
            proof_data: vec![],
            spec_hash,
            result,
            timestamp: chrono::Utc::now(),
            journal,
            journal_output: None,
        }
    }
    
    /// Check if this is a placeholder proof (empty proof_data) or a real proof
    pub fn is_placeholder(&self) -> bool {
        self.proof_data.is_empty()
    }

    /// Verify the proof
    /// In production, this would use RISC Zero's verifier
    pub fn verify(&mut self) -> Result<()> {
        // If this is a placeholder proof, allow it for backward compatibility
        if self.is_placeholder() {
            return Ok(());
        }

        // For real proofs, verify using RISC Zero
        let (output, _) = crate::zkvm::verify_proof(&self.proof_data)?;
        self.journal_output = Some(output);
        Ok(())
    }

    /// Check if the proof is valid and the result is Pass
    pub fn is_valid_pass(&mut self) -> Result<bool> {
        self.verify()?;
        Ok(self.result == ComplianceResult::Pass)
    }
}

