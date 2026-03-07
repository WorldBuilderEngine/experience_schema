use crate::assets::asset_store_schema::AssetBundleSchema;
use prost::Message;
use serde::{Deserialize, Serialize};

/// Serializable asset-bundle payload for an experience.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct AssetBundlesSchema {
    /// Packed and/or generated bundles available to worlds in this experience.
    #[serde(default)]
    #[prost(message, repeated, tag = "1")]
    pub bundles: Vec<AssetBundleSchema>,
}
