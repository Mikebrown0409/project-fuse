//! Test fixture utilities for loading C2PA and other test assets

use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

/// Metadata about a C2PA test fixture
#[derive(Debug, Clone)]
pub struct C2paFixtureMetadata {
    /// Name of the fixture file
    pub name: String,
    /// Signature algorithm used (if known)
    pub algorithm: Option<String>,
    /// Whether fixture contains a claim
    pub has_claim: bool,
    /// Whether fixture contains assertions
    pub has_assertions: bool,
    /// Whether fixture has a signature
    pub has_signature: bool,
    /// Source/origin of the fixture
    pub source: String,
}

/// Load a C2PA fixture by name
///
/// Returns the path to the fixture file, or an error if not found.
pub fn load_c2pa_fixture(name: &str) -> Result<PathBuf> {
    // Get the workspace root (where Cargo.toml is)
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    
    // Navigate to tests/fixtures/c2pa from workspace root
    let fixture_dir = workspace_root
        .join("tests")
        .join("fixtures")
        .join("c2pa");
    
    let fixture_path = fixture_dir.join(name);
    
    if !fixture_path.exists() {
        anyhow::bail!("C2PA fixture not found: {} (searched in {:?})", name, fixture_dir);
    }
    
    Ok(fixture_path)
}

/// List all available C2PA fixtures
pub fn list_available_c2pa_fixtures() -> Result<Vec<String>> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let fixture_dir = workspace_root
        .join("tests")
        .join("fixtures")
        .join("c2pa");
    
    if !fixture_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut fixtures = Vec::new();
    for entry in std::fs::read_dir(&fixture_dir)
        .context("Failed to read fixtures directory")? 
    {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with(".jpg") || name.ends_with(".jpeg") || name.ends_with(".png") {
                    fixtures.push(name.to_string());
                }
            }
        }
    }
    
    fixtures.sort();
    Ok(fixtures)
}

/// Get metadata about a C2PA fixture
///
/// This provides basic metadata based on the fixture filename.
/// For detailed information, parse the actual C2PA manifest.
pub fn get_c2pa_fixture_metadata(name: &str) -> C2paFixtureMetadata {
    // Parse metadata from filename patterns
    // Format: [supplier]-[YYYYMMDD]-[descriptive_string].[ext]
    let has_claim = name.contains("-C") || name.contains("-CA") || name.contains("-CII") || name.contains("-CIE");
    let has_assertions = name.contains("-A") || name.contains("-CA") || name.contains("-CACA");
    let has_signature = name.contains("-sig") || name.contains("-CA") || name.contains("-CIE");
    
    let algorithm = if name.contains("CIE-sig") {
        Some("RSA".to_string())
    } else if name.contains("truepic") {
        Some("Ed25519".to_string())
    } else {
        None
    };
    
    C2paFixtureMetadata {
        name: name.to_string(),
        algorithm,
        has_claim,
        has_assertions,
        has_signature,
        source: "C2PA Public Test Files (github.com/c2pa-org/public-testfiles)".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_fixtures() {
        let fixtures = list_available_c2pa_fixtures().unwrap();
        assert!(!fixtures.is_empty(), "Should have at least some fixtures");
    }

    #[test]
    fn test_load_fixture() {
        let fixtures = list_available_c2pa_fixtures().unwrap();
        if !fixtures.is_empty() {
            let path = load_c2pa_fixture(&fixtures[0]).unwrap();
            assert!(path.exists(), "Fixture should exist");
        }
    }

    #[test]
    fn test_fixture_metadata() {
        let metadata = get_c2pa_fixture_metadata("adobe-20220124-CA.jpg");
        assert!(metadata.has_claim);
        assert!(metadata.has_assertions);
        assert!(metadata.has_signature);
    }
}
