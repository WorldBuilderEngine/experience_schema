use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trusted runtime-facing asset facts derived by backend publish/build flows.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct AssetBundleManifestSchema {
    /// Bundle identifiers keyed to trusted runtime metadata entries.
    #[serde(default)]
    #[prost(map = "string, message", tag = "1")]
    pub bundles: HashMap<String, AssetBundleManifestEntrySchema>,
}

/// Trusted runtime metadata for one asset bundle identifier.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct AssetBundleManifestEntrySchema {
    /// Optional runtime-visible bundle content hash derived during publish/build flows.
    #[serde(default)]
    #[prost(string, tag = "1")]
    pub content_hash: String,

    /// Trusted per-asset runtime facts derived from bundle bytes.
    ///
    /// Keep this limited to facts stripped runtimes actually consume after payload stripping.
    #[serde(default)]
    #[prost(map = "string, message", tag = "2")]
    pub assets: HashMap<String, AssetBundleManifestAssetMetadataSchema>,
}

/// Trusted intrinsic asset metadata for a single runtime-visible asset.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct AssetBundleManifestAssetMetadataSchema {
    /// Trusted intrinsic asset width in pixels for runtime sizing.
    #[serde(default)]
    #[prost(uint32, tag = "1")]
    pub width_px: u32,

    /// Trusted intrinsic asset height in pixels for runtime sizing.
    #[serde(default)]
    #[prost(uint32, tag = "2")]
    pub height_px: u32,
}
