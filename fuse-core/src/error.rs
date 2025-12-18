//! Error types for VCE protocol

use thiserror::Error;

pub type Result<T> = std::result::Result<T, VceError>;

#[derive(Error, Debug)]
pub enum VceError {
    #[error("Invalid specification: {0}")]
    InvalidSpec(String),

    #[error("Invalid envelope format: {0}")]
    InvalidEnvelope(String),

    #[error("Proof verification failed: {0}")]
    ProofVerificationFailed(String),

    #[error("Spec expired: expiry date {0}")]
    SpecExpired(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("YAML parsing error: {0}")]
    YamlParsing(#[from] serde_yaml::Error),

    #[error("RISC Zero error: {0}")]
    RiscZero(String),
}

