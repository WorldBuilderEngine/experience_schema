use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trusted asset bundle inventory authored by backend services.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct AssetBundleManifestSchema {
    /// Bundle identifiers keyed to trusted metadata entries.
    #[serde(default)]
    #[prost(map = "string, message", tag = "1")]
    pub bundles: HashMap<String, AssetBundleManifestEntrySchema>,
}

/// Metadata entry for one trusted asset bundle identifier.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct AssetBundleManifestEntrySchema {
    /// Optional future-proof field for trusted backend bundle content hash.
    #[serde(default)]
    #[prost(string, tag = "1")]
    pub content_hash: String,
}
