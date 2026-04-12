use crate::service_authored::assets::AssetBundleManifestAssetMetadataSchema;
use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trusted runtime metadata for one asset bundle identifier.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
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
