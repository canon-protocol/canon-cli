use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;

/// Canon Protocol Specification
///
/// This represents ANY Canon document - whether it's a type definition,
/// a project specification, or any other kind of Canon specification.
/// The `type` field determines what kind of specification this is.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonSpecification {
    /// Protocol version (e.g., "1.0")
    pub canon: String,

    /// Type reference - determines what kind of specification this is
    /// Examples:
    /// - "canon-protocol.org/type@1.0.0" for type definitions
    /// - "canon-protocol.org/project@1.0.0" for projects
    /// - "content.org/blog-post@1.0.0" for a blog post
    pub r#type: String,

    /// Required metadata for all specifications
    pub metadata: SpecificationMetadata,

    /// Optional includes for type composition (used when this is a type definition)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<String>>,

    /// Optional schema definition (only present when this is a type definition)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<HashMap<String, SchemaField>>,

    /// Type-specific content (any fields defined by the type's schema)
    #[serde(flatten)]
    pub content: HashMap<String, Value>,
}

/// Required metadata for all specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecificationMetadata {
    /// Unique identifier (lowercase, alphanumeric with hyphens)
    pub id: String,

    /// Semantic version (MAJOR.MINOR.PATCH)
    pub version: String,

    /// Publisher domain or subdomain
    pub publisher: String,

    /// Optional human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Schema field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    /// Field type
    pub r#type: FieldType,

    /// Whether the field is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// For ref types, the URI with optional version operators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// Pattern for string validation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    /// Enumeration of allowed values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<Value>>,

    /// For objects, property definitions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, SchemaField>>,

    /// For arrays, item schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<SchemaField>>,

    /// Field description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Supported field types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Object,
    Array,
    Ref,
    Any,
}

// Keep for backward compatibility during migration
#[derive(Debug, Serialize, Deserialize)]
pub struct OutputConfiguration {
    pub artifacts: Option<Vec<String>>,
    pub directory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceDefinition {
    pub path: String,
    pub r#type: String,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}
