//! CLI tool for verifying Verifiable Compliance Envelopes

use clap::Parser;
use fuse_core::{VerifiableComplianceEnvelope, Result};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "fuse-verify")]
#[command(about = "Verify a Verifiable Compliance Envelope (.vce file)")]
struct Args {
    /// Path to the .vce file to verify
    envelope: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("📦 Loading Verifiable Compliance Envelope...");
    let envelope = VerifiableComplianceEnvelope::from_file(&args.envelope)?;

    if args.verbose {
        println!("\n📋 Specification:");
        println!("   Claim: {}", envelope.spec.claim);
        println!("   System Hash: {}", envelope.spec.system_hash);
        println!("   Jurisdiction: {}", envelope.spec.jurisdiction);
        println!("   Version: {}", envelope.spec.version);
        println!("   Expiry: {}", envelope.spec.expiry);
        println!("\n🔐 Proof:");
        println!("   Spec Hash: {}", envelope.proof.spec_hash);
        println!("   Result: {}", envelope.proof.result);
        println!("   Timestamp: {}", envelope.proof.timestamp);
    }

    println!("\n🔍 Verifying envelope...");
    match envelope.verify() {
        Ok(()) => {
            println!("✅ Envelope is valid!");
            
            match envelope.is_compliant() {
                Ok(true) => {
                    println!("✅ Compliance check: PASS");
                    std::process::exit(0);
                }
                Ok(false) => {
                    println!("❌ Compliance check: FAIL");
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("❌ Error checking compliance: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Verification failed: {}", e);
            std::process::exit(1);
        }
    }
}

