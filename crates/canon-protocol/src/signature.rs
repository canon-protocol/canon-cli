use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Canon signature for verifying authenticity
#[derive(Debug, Serialize, Deserialize)]
pub struct CanonSignature {
    pub canon: String,
    pub signature_version: String,
    pub manifest_hash: String,
    pub signature: SignatureData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureData {
    pub algorithm: String,
    pub key_id: String,
    pub signature: String,
    pub signed_at: DateTime<Utc>,
}

/// Publisher keys for verification
#[derive(Debug, Serialize, Deserialize)]
pub struct PublisherKeys {
    pub version: String,
    pub keys: std::collections::HashMap<String, PublisherKey>,
    #[serde(default)]
    pub revoked_keys: std::collections::HashMap<String, RevokedKey>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublisherKey {
    pub algorithm: String,
    pub public_key: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked: bool,
    pub usage: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevokedKey {
    pub revoked_at: DateTime<Utc>,
    pub reason: String,
}