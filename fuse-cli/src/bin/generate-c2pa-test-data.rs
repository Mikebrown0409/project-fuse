//! Generate mock C2PA test data for testing C2PA signature verification
//!
//! This tool creates test data that simulates what would be extracted from a C2PA manifest.
//! The data includes a public key, signature, and message in the format expected by the C2PA checker.

use clap::Parser;
use fuse_cli::c2pa::{create_mock_c2pa_signature_data, c2pa_data_to_json};
use std::fs;

#[derive(Parser)]
#[command(name = "generate-c2pa-test-data")]
#[command(about = "Generate mock C2PA test data for signature verification")]
struct Args {
    /// Output file path
    #[arg(short, long, default_value = "examples/systems/c2pa-test-data.json")]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("Generating mock C2PA test data...");

    // Generate mock C2PA signature data
    let c2pa_data = create_mock_c2pa_signature_data()?;

    // Convert to JSON format for system data
    let system_data = c2pa_data_to_json(&c2pa_data);

    // Write to file
    let output_path = std::path::Path::new(&args.output);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json_output = serde_json::to_string_pretty(&system_data)?;
    fs::write(&args.output, json_output)?;

    println!("✅ Generated C2PA test data at: {}", args.output);
    println!("\nData summary:");
    println!("  Public key (hex): {} ({} bytes)", c2pa_data.public_key, c2pa_data.public_key.len() / 2);
    println!("  Message (hex): {} ({} bytes)", c2pa_data.message, c2pa_data.message.len() / 2);
    println!("  Signature (hex): {} ({} bytes)", c2pa_data.signature, c2pa_data.signature.len() / 2);

    Ok(())
}

