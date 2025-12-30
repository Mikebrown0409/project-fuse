//! Tool to generate a real, signed C2PA manifest for testing
//!
//! This tool uses the `c2pa` crate to create a manifest, sign it with an Ed25519 key,
//! and save it to a file. This provides a realistic test case for the C2PA parser.

use c2pa::{Builder, Signer, SigningAlg};
use std::path::Path;
use std::fs;
use std::io::Cursor;
use clap::Parser;
use rcgen::{CertificateParams, KeyPair, DistinguishedName, IsCa, BasicConstraints, KeyUsagePurpose};
use serde_json::json;
use ed25519_dalek::{SigningKey, Signer as DalekSigner, pkcs8::DecodePrivateKey};

#[derive(Parser)]
#[command(name = "generate-real-c2pa")]
#[command(about = "Generate a real signed C2PA manifest")]
struct Args {
    /// Output directory
    #[arg(short, long, default_value = "examples/c2pa")]
    output_dir: String,

    /// Dummy source file to sign
    #[arg(short, long, default_value = "examples/c2pa/dummy.jpg")]
    source: String,
}

struct Ed25519C2paSigner {
    signing_key: SigningKey,
    cert_chain_der: Vec<Vec<u8>>,
}

impl Signer for Ed25519C2paSigner {
    fn sign(&self, data: &[u8]) -> c2pa::Result<Vec<u8>> {
        let signature = self.signing_key.sign(data);
        Ok(signature.to_bytes().to_vec())
    }

    fn alg(&self) -> SigningAlg {
        SigningAlg::Ed25519
    }

    fn certs(&self) -> c2pa::Result<Vec<Vec<u8>>> {
        Ok(self.cert_chain_der.clone())
    }

    fn reserve_size(&self) -> usize {
        64 // Ed25519 signature size
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let output_path = Path::new(&args.output_dir);
    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    }

    // 1. Create a dummy source file if it doesn't exist
    let source_path = Path::new(&args.source);
    if !source_path.exists() {
        fs::write(source_path, b"Dummy image data for C2PA testing")?;
        println!("Created dummy source file: {}", args.source);
    }

    // 2. Generate a CA and a signed Ed25519 certificate
    println!("Generating CA and signed Ed25519 certificate...");
    
    // Create CA
    let mut ca_params = CertificateParams::default();
    ca_params.distinguished_name = DistinguishedName::new();
    ca_params.distinguished_name.push(rcgen::DnType::CommonName, "FUSE Test CA");
    ca_params.distinguished_name.push(rcgen::DnType::OrganizationName, "FUSE");
    ca_params.distinguished_name.push(rcgen::DnType::CountryName, "US");
    ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    ca_params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::DigitalSignature];
    
    let ca_keypair = KeyPair::generate_for(&rcgen::PKCS_ED25519)?;
    let ca_cert = ca_params.self_signed(&ca_keypair)?;
    
    // Create End-Entity cert
    let ee_keypair = KeyPair::generate_for(&rcgen::PKCS_ED25519)?;
    let mut ee_params = CertificateParams::default();
    ee_params.distinguished_name = DistinguishedName::new();
    ee_params.distinguished_name.push(rcgen::DnType::CommonName, "FUSE Test EE");
    ee_params.distinguished_name.push(rcgen::DnType::OrganizationName, "FUSE");
    ee_params.distinguished_name.push(rcgen::DnType::CountryName, "US");
    ee_params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    
    let ee_cert = ee_params.signed_by(&ee_keypair, &ca_cert, &ca_keypair)?;
    
    let cert_chain_der = vec![ee_cert.der().to_vec(), ca_cert.der().to_vec()];
    
    // Extract the public key raw bytes for reference
    let pub_key_bytes = ee_keypair.public_key_raw();
    println!("Generated Ed25519 public key: {}", hex::encode(pub_key_bytes));

    // Create a signing key for the signer
    let pkcs8_der = ee_keypair.serialize_der();
    let signing_key = SigningKey::from_pkcs8_der(&pkcs8_der)
        .map_err(|e| anyhow::anyhow!("Failed to parse PKCS#8: {}", e))?;

    // 3. Create a C2PA manifest definition
    let manifest_json = json!({
        "title": "FUSE Test Image",
        "format": "application/c2pa",
        "claim_generator": "fuse-test",
        "assertions": [
            {
                "label": "c2pa.test",
                "data": {
                    "test_key": "test_value",
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "location": "New York, NY"
                }
            }
        ]
    }).to_string();

    let mut builder = Builder::from_json(&manifest_json)?;

    // 4. Set up the signer
    let signer = Ed25519C2paSigner {
        signing_key,
        cert_chain_der,
    };

    // 5. Sign and embed
    let mut source_reader = Cursor::new(fs::read(source_path)?);
    let mut dest_writer = Cursor::new(Vec::new());
    
    builder.sign(&signer, "application/c2pa", &mut source_reader, &mut dest_writer)?;

    let dest_path = output_path.join("signed_test.c2pa");
    fs::write(&dest_path, dest_writer.into_inner())?;

    println!("✅ Generated real signed C2PA manifest at: {:?}", dest_path);
    
    // Save the public key separately for reference
    let pub_key_path = output_path.join("test_pub_key.hex");
    fs::write(&pub_key_path, hex::encode(pub_key_bytes))?;
    println!("Saved public key to: {:?}", pub_key_path);

    Ok(())
}
