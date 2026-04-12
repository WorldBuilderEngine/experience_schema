use prost::Message;
use serde::{Deserialize, Serialize};

/// Trusted intrinsic image metadata for runtime sizing.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct AssetBundleManifestImageMetadataSchema {
    /// Trusted intrinsic asset width in pixels for runtime sizing.
    #[serde(default)]
    #[prost(uint32, tag = "1")]
    pub width_px: u32,

    /// Trusted intrinsic asset height in pixels for runtime sizing.
    #[serde(default)]
    #[prost(uint32, tag = "2")]
    pub height_px: u32,
}
