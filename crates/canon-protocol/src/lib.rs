//! Canon Protocol core types and validation
//!
//! This library provides the fundamental data structures and parsing logic
//! for the Canon Protocol specification format.

pub mod dependency;
pub mod error;
pub mod manifest;
pub mod signature;
pub mod specification;

// Re-export commonly used types at the crate root
pub use dependency::{Dependency, VersionOperator};
pub use error::{ProtocolError, ProtocolResult};
pub use manifest::{CanonManifest, ManifestFile, ManifestSpecification};
pub use signature::{CanonSignature, PublisherKeys, SignatureData};
pub use specification::{
    CanonSpecification, FieldType, OutputConfiguration, SchemaField, SourceDefinition,
    SpecificationMetadata,
};
