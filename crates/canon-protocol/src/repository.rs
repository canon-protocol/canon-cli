use serde::{Deserialize, Serialize};

/// Root-level canon.yml structure for repository management
#[derive(Debug, Serialize, Deserialize)]
pub struct CanonRepository {
    pub canon: String,
    pub registry: RegistryConfig,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub default: String,
}

impl CanonRepository {
    /// Create a new repository configuration with core dependencies
    pub fn new() -> Self {
        Self {
            canon: "1.0".to_string(),
            registry: RegistryConfig {
                default: "https://spec.farm".to_string(),
            },
            dependencies: vec![
                "canon-protocol.org/type@1.0.0".to_string(),
                "canon-protocol.org/transformation@1.0.0".to_string(),
                "canon-protocol.org/auto-version@1.0.0".to_string(),
            ],
        }
    }

    /// Add a new dependency to the repository
    pub fn add_dependency(&mut self, uri: String) {
        if !self.dependencies.contains(&uri) {
            self.dependencies.push(uri);
        }
    }
}

impl Default for CanonRepository {
    fn default() -> Self {
        Self::new()
    }
}
