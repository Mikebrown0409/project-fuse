//! Helper tool to extract full C2PA claim JSON for demo purposes

use clap::Parser;
use fuse_cli::c2pa::parse_c2pa_manifest;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "extract-c2pa-claim")]
#[command(about = "Extract full C2PA claim JSON from a signed image")]
struct Args {
    /// Path to C2PA-signed asset
    #[arg(short, long)]
    input: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    let c2pa_data = parse_c2pa_manifest(args.input.to_str().unwrap_or_default())?;
    
    // Print the full claim JSON as pretty-printed JSON
    println!("{}", serde_json::to_string_pretty(&c2pa_data.claim_json)?);
    
    Ok(())
}

