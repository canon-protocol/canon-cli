use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Invalid URI format: {0}")]
    InvalidUri(String),

    #[error("Invalid specification: {0}")]
    InvalidSpecification(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type ProtocolResult<T> = std::result::Result<T, ProtocolError>;
