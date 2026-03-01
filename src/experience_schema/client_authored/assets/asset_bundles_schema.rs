use assets::asset_store_schema::AssetBundleSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Serializable asset-bundle payload for an experience.
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct AssetBundlesSchema {
    /// Packed and/or generated bundles available to worlds in this experience.
    #[serde(default)]
    pub bundles: Vec<AssetBundleSchema>,
    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
}

pub type AssetStoresSchema = AssetBundlesSchema;
