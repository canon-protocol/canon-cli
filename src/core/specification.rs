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

impl CanonSpecification {
    pub fn new(
        name: String,
        spec_type: String,
        author: Option<String>,
        license: Option<String>,
    ) -> Self {
        Self {
            canon: "1.0".to_string(),
            r#type: spec_type,
            name,
            version: "0.1.0".to_string(),
            metadata: Some(SpecificationMetadata {
                description: None,
                author,
                license,
                tags: None,
            }),
            sources: Some(vec![SourceDefinition {
                path: "sources/".to_string(),
                r#type: "general".to_string(),
                include: None,
                exclude: None,
            }]),
            transformations: None,
            output: None,
        }
    }
}
