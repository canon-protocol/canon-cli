use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Canon manifest for tracking file contents and integrity
#[derive(Debug, Serialize, Deserialize)]
pub struct CanonManifest {
    pub canon: String,
    pub manifest_version: String,
    pub created_at: DateTime<Utc>,
    pub specification: ManifestSpecification,
    pub files: Vec<ManifestFile>,
    pub canonical_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestSpecification {
    pub publisher: String,
    pub name: String,
    pub version: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestFile {
    pub path: String,
    pub hash: String,
    pub size: u64,
}

impl CanonManifest {
    /// Compute the canonical hash from file hashes
    pub fn compute_canonical_hash(files: &[ManifestFile]) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        
        // Sort files by path and concatenate hashes
        let mut sorted_files = files.to_vec();
        sorted_files.sort_by(|a, b| a.path.cmp(&b.path));
        
        for file in sorted_files {
            hasher.update(&file.hash);
        }
        
        use base64::Engine;
        let engine = base64::engine::general_purpose::STANDARD;
        format!("sha256:{}", engine.encode(hasher.finalize()))
    }
}