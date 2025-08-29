use crate::error::{ProtocolError, ProtocolResult};
use std::path::PathBuf;

/// Represents a parsed dependency URI
#[derive(Debug, Clone)]
pub struct Dependency {
    pub publisher: String,
    pub id: String,
    pub version: Option<String>,
    pub version_operator: Option<VersionOperator>,
}

/// Version operators for flexible versioning in schemas
#[derive(Debug, Clone, PartialEq)]
pub enum VersionOperator {
    /// Caret (^) - Compatible changes
    Caret,
    /// Tilde (~) - Patch-level changes  
    Tilde,
}

impl Dependency {
    /// Parse a dependency URI like "canon-protocol.org/type@1.0.0"
    /// or with version operators like "canon-protocol.org/type@^1.0.0"
    pub fn parse(uri: &str) -> ProtocolResult<Self> {
        // Split by @ to separate the path from version
        let parts: Vec<&str> = uri.splitn(2, '@').collect();

        // Parse the path part
        let path_parts: Vec<&str> = parts[0].split('/').collect();
        if path_parts.len() != 2 {
            return Err(ProtocolError::InvalidUri(format!(
                "Invalid dependency URI format: {}. Expected format: publisher/id[@version]",
                uri
            )));
        }

        let publisher = path_parts[0].to_string();
        let id = path_parts[1].to_string();

        // Parse version if present
        let (version, version_operator) = if parts.len() > 1 {
            let version_str = parts[1];

            // Check for version operators
            if let Some(stripped) = version_str.strip_prefix('^') {
                (Some(stripped.to_string()), Some(VersionOperator::Caret))
            } else if let Some(stripped) = version_str.strip_prefix('~') {
                (Some(stripped.to_string()), Some(VersionOperator::Tilde))
            } else {
                (Some(version_str.to_string()), None)
            }
        } else {
            (None, None)
        };

        Ok(Self {
            publisher,
            id,
            version,
            version_operator,
        })
    }

    /// Get the local storage path for this dependency
    pub fn local_path(&self) -> PathBuf {
        let mut path = PathBuf::from(".canon");
        path.push(&self.publisher);
        path.push(&self.id);
        if let Some(ref version) = self.version {
            path.push(version);
        }
        path
    }

    /// Construct the URL for fetching from canon.canon-protocol.org
    pub fn canon_url(&self) -> String {
        let version = self.version.as_deref().unwrap_or("latest");
        format!(
            "https://canon.canon-protocol.org/{}/{}/{}/canon.yml",
            self.publisher, self.id, version
        )
    }

    /// Check if this dependency is already installed
    pub fn is_installed(&self) -> bool {
        let canon_file = self.local_path().join("canon.yml");
        canon_file.exists()
    }

    /// Format the dependency as a URI string
    pub fn to_uri(&self) -> String {
        match (&self.version, &self.version_operator) {
            (Some(v), Some(VersionOperator::Caret)) => {
                format!("{}/{}@^{}", self.publisher, self.id, v)
            }
            (Some(v), Some(VersionOperator::Tilde)) => {
                format!("{}/{}@~{}", self.publisher, self.id, v)
            }
            (Some(v), None) => format!("{}/{}@{}", self.publisher, self.id, v),
            (None, _) => format!("{}/{}", self.publisher, self.id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dependency() {
        let dep = Dependency::parse("canon-protocol.org/type@1.0.0").unwrap();
        assert_eq!(dep.publisher, "canon-protocol.org");
        assert_eq!(dep.id, "type");
        assert_eq!(dep.version, Some("1.0.0".to_string()));
        assert_eq!(dep.version_operator, None);

        let dep_no_version = Dependency::parse("example.com/api").unwrap();
        assert_eq!(dep_no_version.publisher, "example.com");
        assert_eq!(dep_no_version.id, "api");
        assert_eq!(dep_no_version.version, None);
        assert_eq!(dep_no_version.version_operator, None);
    }

    #[test]
    fn test_parse_dependency_with_operators() {
        let dep_caret = Dependency::parse("profiles.org/author@^1.0.0").unwrap();
        assert_eq!(dep_caret.publisher, "profiles.org");
        assert_eq!(dep_caret.id, "author");
        assert_eq!(dep_caret.version, Some("1.0.0".to_string()));
        assert_eq!(dep_caret.version_operator, Some(VersionOperator::Caret));

        let dep_tilde = Dependency::parse("standards.org/metadata@~2.1.0").unwrap();
        assert_eq!(dep_tilde.publisher, "standards.org");
        assert_eq!(dep_tilde.id, "metadata");
        assert_eq!(dep_tilde.version, Some("2.1.0".to_string()));
        assert_eq!(dep_tilde.version_operator, Some(VersionOperator::Tilde));
    }

    #[test]
    fn test_local_path() {
        let dep = Dependency {
            publisher: "canon-protocol.org".to_string(),
            id: "type".to_string(),
            version: Some("1.0.0".to_string()),
            version_operator: None,
        };

        let path = dep.local_path();
        assert_eq!(path, PathBuf::from(".canon/canon-protocol.org/type/1.0.0"));
    }

    #[test]
    fn test_canon_url() {
        let dep = Dependency {
            publisher: "canon-protocol.org".to_string(),
            id: "type".to_string(),
            version: Some("1.0.0".to_string()),
            version_operator: None,
        };

        let url = dep.canon_url();
        assert_eq!(
            url,
            "https://canon.canon-protocol.org/canon-protocol.org/type/1.0.0/canon.yml"
        );
    }

    #[test]
    fn test_to_uri() {
        let dep = Dependency {
            publisher: "canon-protocol.org".to_string(),
            id: "type".to_string(),
            version: Some("1.0.0".to_string()),
            version_operator: None,
        };
        assert_eq!(dep.to_uri(), "canon-protocol.org/type@1.0.0");

        let dep_caret = Dependency {
            publisher: "profiles.org".to_string(),
            id: "author".to_string(),
            version: Some("1.0.0".to_string()),
            version_operator: Some(VersionOperator::Caret),
        };
        assert_eq!(dep_caret.to_uri(), "profiles.org/author@^1.0.0");
    }
}
