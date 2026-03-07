use crate::assets::asset_bundle_schema::AssetBundleSchema;
use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Serializable asset-bundle payload for an experience.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct AssetBundlesSchema {
    /// Packed and/or generated bundles available to worlds in this experience.
    #[serde(default)]
    #[prost(btree_map = "string, message", tag = "1")]
    pub bundles: BTreeMap<String, AssetBundleSchema>,
}
