//! Canon Protocol core types and validation
//!
//! This library provides the fundamental data structures and parsing logic
//! for the Canon Protocol specification format.

pub mod dependency;
pub mod error;
pub mod manifest;
pub mod repository;
pub mod signature;
pub mod specification;

// Re-export commonly used types at the crate root
pub use dependency::Dependency;
pub use error::{ProtocolError, ProtocolResult};
pub use manifest::{CanonManifest, ManifestFile, ManifestSpecification};
pub use repository::{CanonRepository, RegistryConfig};
pub use signature::{CanonSignature, PublisherKeys, SignatureData};
pub use specification::{
    CanonSpecification, OutputConfiguration, SourceDefinition, SpecificationMetadata,
};