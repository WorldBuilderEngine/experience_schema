use crate::service_authored::assets::AssetBundleManifestImageMetadataSchema;
use prost::Message;
use serde::{Deserialize, Serialize};

/// Trusted runtime metadata for a single runtime-visible asset.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct AssetBundleManifestAssetMetadataSchema {
    /// Optional runtime-visible content hash derived from raw asset bytes during publish/build flows.
    #[serde(default)]
    #[prost(string, tag = "2")]
    pub content_hash: String,

    /// Kinded service-authored asset metadata.
    #[serde(flatten)]
    #[prost(
        oneof = "asset_bundle_manifest_asset_metadata_schema::Metadata",
        tags = "1"
    )]
    pub metadata: Option<asset_bundle_manifest_asset_metadata_schema::Metadata>,
}

pub mod asset_bundle_manifest_asset_metadata_schema {
    use super::AssetBundleManifestImageMetadataSchema;
    use prost::Oneof;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Clone, PartialEq, Oneof)]
    #[serde(tag = "kind", content = "value")]
    pub enum Metadata {
        #[serde(rename = "image")]
        #[prost(message, tag = "1")]
        Image(AssetBundleManifestImageMetadataSchema),
    }
}
