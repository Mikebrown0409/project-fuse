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
    let mut envelope = VerifiableComplianceEnvelope::from_file(&args.envelope)?;

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
    
    // Check if this is a placeholder or real proof
    if envelope.proof.is_placeholder() {
        println!("   ℹ️  Placeholder proof detected (backward compatibility mode)");
    } else {
        println!("   ✓ Real zkVM proof detected");
    }
    
    match envelope.verify() {
        Ok(()) => {
            println!("✅ Envelope is valid!");

            // If it's a C2PA claim, let's look at the journal for selective disclosure
            if envelope.spec.claim.to_lowercase().contains("c2pa") {
                println!("\n📂 Selective Disclosure (from Proof Journal):");
                
                if let Some(output) = &envelope.proof.journal_output {
                    println!("   Compliance Status: {:?}", output.result);
                    
                    if !output.claim_hash.is_empty() {
                        println!("   Original Claim Hash (SHA256): {}", hex::encode(&output.claim_hash));
                    }
                    
                    if !output.redacted_json.is_empty() {
                        println!("   Redacted Manifest Data:");
                        // Parse the JSON string back to Value for pretty printing
                        match serde_json::from_str::<serde_json::Value>(&output.redacted_json) {
                            Ok(json) => println!("{}", serde_json::to_string_pretty(&json).unwrap_or_default()),
                            Err(_) => println!("   {}", output.redacted_json),
                        }
                    }
                } else {
                    println!("   (No detailed journal output found)");
                }
            }
            
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

