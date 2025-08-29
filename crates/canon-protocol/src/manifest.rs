use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Canon Protocol Manifest
/// Conforms to canon-protocol.org/manifest@1.0.0
#[derive(Debug, Serialize, Deserialize)]
pub struct CanonManifest {
    pub canon: String,
    pub manifest_version: String,
    pub created_at: DateTime<Utc>,
    pub specification: ManifestSpecification,
    pub files: Vec<ManifestFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<String>>,
    pub total_size: u64,
    pub file_count: usize,
    pub canonical_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestSpecification {
    pub id: String,
    pub version: String,
    pub publisher: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestFile {
    pub path: String,
    pub size: u64,
    pub hash: String,
}

impl CanonManifest {
    /// Create a new manifest
    pub fn new(spec: ManifestSpecification, files: Vec<ManifestFile>) -> Self {
        let total_size = files.iter().map(|f| f.size).sum();
        let file_count = files.len();
        let canonical_hash = Self::compute_canonical_hash(&files);

        Self {
            canon: "1.0".to_string(),
            manifest_version: "1.0".to_string(),
            created_at: Utc::now(),
            specification: spec,
            files,
            directories: None,
            total_size,
            file_count,
            canonical_hash,
        }
    }

    /// Compute the canonical hash from file hashes
    pub fn compute_canonical_hash(files: &[ManifestFile]) -> String {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();

        // Sort files by path (lexicographically) and concatenate hashes
        let mut sorted_files = files.to_vec();
        sorted_files.sort_by(|a, b| a.path.cmp(&b.path));

        for file in sorted_files {
            hasher.update(&file.hash);
        }

        format!("sha256:{:x}", hasher.finalize())
    }
}
