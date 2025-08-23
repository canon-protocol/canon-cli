use crate::utils::{CanonError, CanonResult};
use std::path::PathBuf;

/// Represents a parsed dependency URI
#[derive(Debug, Clone)]
pub struct Dependency {
    pub publisher: String,
    pub name: String,
    pub version: Option<String>,
}

impl Dependency {
    /// Parse a dependency URI like "canon-protocol.org/type@1.0.0"
    pub fn parse(uri: &str) -> CanonResult<Self> {
        // Split by @ to separate the path from version
        let parts: Vec<&str> = uri.splitn(2, '@').collect();

        // Parse the path part
        let path_parts: Vec<&str> = parts[0].split('/').collect();
        if path_parts.len() != 2 {
            return Err(CanonError::Config {
                message: format!("Invalid dependency URI format: {}", uri),
            });
        }

        let publisher = path_parts[0].to_string();
        let name = path_parts[1].to_string();

        // Parse version if present
        let version = if parts.len() > 1 {
            Some(parts[1].to_string())
        } else {
            None
        };

        Ok(Self {
            publisher,
            name,
            version,
        })
    }

    /// Get the local storage path for this dependency
    pub fn local_path(&self, registry_domain: &str) -> PathBuf {
        let mut path = PathBuf::from(".canon");
        path.push(registry_domain);
        path.push(&self.publisher);
        path.push(&self.name);
        if let Some(ref version) = self.version {
            path.push(version);
        }
        path
    }

    /// Construct the registry URL for this dependency
    pub fn registry_url(&self, registry_base: &str) -> String {
        let version = self.version.as_deref().unwrap_or("latest");
        format!(
            "{}/{}/{}/{}/",
            registry_base.trim_end_matches('/'),
            self.publisher,
            self.name,
            version
        )
    }

    /// Check if this dependency is already installed
    pub fn is_installed(&self, registry_domain: &str) -> bool {
        self.local_path(registry_domain).exists()
    }
}
