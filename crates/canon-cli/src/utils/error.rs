use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)] // These variants will be used in future phases
pub enum CanonError {
    #[error("Specification validation failed: {message}")]
    ValidationError { message: String },

    #[error("Transformation failed: {transformation} - {reason}")]
    TransformationError {
        transformation: String,
        reason: String,
    },

    #[error("Registry error: {url} - {status}")]
    RegistryError { url: String, status: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_yaml::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Command failed: {message}")]
    Command { message: String },
    
    #[error("Protocol error: {0}")]
    Protocol(#[from] canon_protocol::ProtocolError),
}

pub type CanonResult<T> = std::result::Result<T, CanonError>;
