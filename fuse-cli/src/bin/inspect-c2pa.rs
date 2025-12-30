//! Tool to inspect a real C2PA manifest and identify extraction points
//!
//! This tool loads a signed C2PA file and prints available data
//! to help us implement the extraction logic for zkVM.

use c2pa::{Reader};
use std::path::Path;
use clap::Parser;
use img_parts::jpeg::Jpeg;
use std::fs::read;

#[derive(Parser)]
#[command(name = "inspect-c2pa")]
#[command(about = "Inspect a real C2PA manifest")]
struct Args {
    /// Path to C2PA-signed asset
    #[arg(short, long, default_value = "examples/c2pa/C.jpg")]
    input: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.input);

    println!("🔍 Loading C2PA manifest from: {:?}", path);
    
    // 1. Try high-level C2PA Reader
    if let Some(format) = c2pa::format_from_path(path) {
        println!("  Detected format: {}", format);
    }
    
    match Reader::from_file(path) {
        Ok(reader) => {
            println!("  C2PA Reader validation state: {:?}", reader.validation_state());
            if let Some(manifest) = reader.active_manifest() {
                println!("  Active manifest label: {:?}", manifest.label());
                if let Some(sig_info) = manifest.signature_info() {
                    println!("  Signature Algorithm: {:?}", sig_info.alg);
                }
            }
        },
        Err(e) => println!("  C2PA Reader failed: {}", e),
    }

    // 2. Try low-level img-parts to find JUMBF segments
    println!("\n🔍 Inspecting JPEG segments for C2PA data...");
    let bytes = read(path)?;
    let jpeg = Jpeg::from_bytes(bytes.into())?;
    
    let mut jumbf_segments = 0;
    for segment in jpeg.segments() {
        // APP11 marker is 0xEB
        if segment.marker() == 0xEB {
            jumbf_segments += 1;
            let data = segment.contents();
            println!("  Found APP11 (JUMBF) segment #{}", jumbf_segments);
            println!("    Size: {} bytes", data.len());
            if data.len() > 10 {
                println!("    Preview: {:02x?}", &data[..std::cmp::min(data.len(), 16)]);
            }
        }
    }
    
    if jumbf_segments == 0 {
        println!("  No JUMBF segments found.");
    }

    Ok(())
}

