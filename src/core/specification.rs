use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CanonSpecification {
    pub canon: String,
    pub r#type: String,
    pub name: String,
    pub version: String,
    pub metadata: Option<SpecificationMetadata>,
    pub sources: Option<Vec<SourceDefinition>>,
    pub transformations: Option<Vec<String>>,
    pub output: Option<OutputConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificationMetadata {
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceDefinition {
    pub path: String,
    pub r#type: String,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputConfiguration {
    pub artifacts: Option<Vec<String>>,
    pub directory: Option<String>,
}

