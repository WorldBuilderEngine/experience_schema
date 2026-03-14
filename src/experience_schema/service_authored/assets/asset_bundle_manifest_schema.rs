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

    /// Trusted per-asset metadata derived from backend-visible bundle bytes.
    #[serde(default)]
    #[prost(map = "string, message", tag = "2")]
    pub assets: HashMap<String, AssetBundleManifestAssetMetadataSchema>,
}

/// Trusted spatial metadata for a single asset within a bundle.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct AssetBundleManifestAssetMetadataSchema {
    /// Trusted intrinsic asset width in pixels.
    #[serde(default)]
    #[prost(uint32, tag = "1")]
    pub width_px: u32,

    /// Trusted intrinsic asset height in pixels.
    #[serde(default)]
    #[prost(uint32, tag = "2")]
    pub height_px: u32,
}
