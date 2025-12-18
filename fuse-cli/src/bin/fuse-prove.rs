//! CLI tool for generating Verifiable Compliance Envelopes

use clap::Parser;
use fuse_core::{ComplianceSpec, VerifiableComplianceEnvelope, Result};
use fuse_checkers::CheckerRegistry;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "fuse-prove")]
#[command(about = "Generate a Verifiable Compliance Envelope from a spec and system data")]
struct Args {
    /// Path to the compliance specification file (JSON or YAML)
    #[arg(short, long)]
    spec: PathBuf,

    /// Path to the system data file to verify
    #[arg(short, long)]
    system: PathBuf,

    /// Output path for the .vce file
    #[arg(short, long, default_value = "compliance.vce")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("🔍 Loading compliance specification...");
    let spec = if args.spec.extension().and_then(|s| s.to_str()) == Some("yaml")
        || args.spec.extension().and_then(|s| s.to_str()) == Some("yml")
    {
        ComplianceSpec::from_yaml_file(&args.spec)?
    } else {
        ComplianceSpec::from_json_file(&args.spec)?
    };

    println!("   Claim: {}", spec.claim);
    println!("   Jurisdiction: {}", spec.jurisdiction);
    println!("   Version: {}", spec.version);

    println!("\n📊 Loading system data...");
    let system_data = std::fs::read_to_string(&args.system)?;
    println!("   Loaded {} bytes of system data", system_data.len());

    println!("\n⚙️  Running compliance checker...");
    let registry = CheckerRegistry::new();
    let checker = registry.get_checker(&spec.claim)?;
    
    let result = checker.check(&spec, &system_data)?;
    println!("   Result: {}", result);

    println!("\n🔐 Generating zero-knowledge proof...");
    // TODO: In production, this would generate an actual RISC Zero proof
    // For MVP, we create a placeholder proof
    let spec_hash = spec.hash();
    let proof = fuse_core::ComplianceProof::new(
        spec_hash,
        result,
        system_data.as_bytes().to_vec(), // Journal contains public outputs
    );

    println!("\n📦 Creating Verifiable Compliance Envelope...");
    let envelope = VerifiableComplianceEnvelope::new(spec, proof);
    
    println!("\n💾 Saving envelope to {}...", args.output.display());
    envelope.to_file(&args.output)?;

    println!("\n✅ Success! Compliance envelope created at {}", args.output.display());
    println!("   Verify with: fuse-verify {}", args.output.display());

    Ok(())
}

