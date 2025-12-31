//! CLI tool for generating Verifiable Compliance Envelopes

use clap::{Parser, ValueEnum};
use fuse_core::{ComplianceSpec, VerifiableComplianceEnvelope, Result, ProverType};
use fuse_checkers::CheckerRegistry;
use std::path::PathBuf;

/// CLI argument representation of `ProverType`
#[derive(Debug, Clone, Copy, ValueEnum)]
enum ProverTypeArg {
    /// Local CPU prover (default)
    Local,
    /// GPU-accelerated prover
    Gpu,
}

impl From<ProverTypeArg> for ProverType {
    fn from(arg: ProverTypeArg) -> Self {
        match arg {
            ProverTypeArg::Local => ProverType::Local,
            ProverTypeArg::Gpu => ProverType::Gpu,
        }
    }
}

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

    /// Prover type to use for proof generation
    /// 
    /// - local: CPU-based proving (default, always available)
    /// - gpu: GPU-accelerated proving (requires GPU hardware and --features gpu)
    #[arg(long, default_value = "local", value_enum)]
    prover: ProverTypeArg,
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
    let extension = args.system.extension().and_then(|s| s.to_str()).unwrap_or_default().to_lowercase();
    let is_media = ["jpg", "jpeg", "png", "c2pa"].contains(&extension.as_str());

    let system_data = if is_media && spec.claim.to_lowercase().contains("c2pa") {
        println!("   Detected media file with C2PA claim. Extracting manifest...");
        let c2pa_data = fuse_cli::c2pa::parse_c2pa_manifest(args.system.to_str().unwrap_or_default())
            .map_err(|e| fuse_core::VceError::InputSerialization(e.to_string()))?;
        let json = fuse_cli::c2pa::c2pa_data_to_json(&c2pa_data);
        serde_json::to_string(&json).map_err(|e| fuse_core::VceError::InputSerialization(e.to_string()))?
    } else {
        std::fs::read_to_string(&args.system)?
    };
    println!("   Loaded {} bytes of processed system data", system_data.len());

    println!("\n⚙️  Running compliance checker...");
    let registry = CheckerRegistry::new();
    let checker = registry.get_checker(&spec.claim)?;
    
    let result = checker.check(&spec, &system_data)?;
    println!("   Result: {result}");

    println!("\n🔐 Generating zero-knowledge proof...");
    let spec_hash = spec.hash();
    
    // Try to generate a real RISC Zero proof
    // If it fails (e.g., guest program not built), fall back to placeholder
    let prover_type: ProverType = args.prover.into();
    let proof = match fuse_core::zkvm::generate_proof(
        &serde_json::to_string(&spec).unwrap_or_default(),
        &system_data,
        prover_type,
    ) {
        Ok((receipt_bytes, zk_result, journal)) => {
            println!("   ✓ Real zkVM proof generated");
            fuse_core::ComplianceProof::from_risc_zero_receipt(
                spec_hash,
                receipt_bytes,
                zk_result,
                journal,
            )
        }
        Err(e) => {
            println!("   ⚠ Falling back to placeholder proof: {e}");
            println!("   (This is expected if guest program is not yet built)");
            fuse_core::ComplianceProof::new(
                spec_hash,
                result,
                system_data.as_bytes().to_vec(), // Journal contains public outputs
            )
        }
    };

    println!("\n📦 Creating Verifiable Compliance Envelope...");
    let envelope = VerifiableComplianceEnvelope::new(spec, proof);
    
    println!("\n💾 Saving envelope to {}...", args.output.display());
    envelope.to_file(&args.output)?;

    println!("\n✅ Success! Compliance envelope created at {}", args.output.display());
    println!("   Verify with: fuse-verify {}", args.output.display());

    Ok(())
}

