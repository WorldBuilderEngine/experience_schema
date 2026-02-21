use crate::assets::asset_store_schema::AssetBundleSchema;
use serde::{Deserialize, Serialize};

/// Serializable asset-bundle payload for an experience.
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct AssetBundlesSchema {
    /// Packed and/or generated bundles available to worlds in this experience.
    #[serde(default, alias = "stores")]
    pub bundles: Vec<AssetBundleSchema>,
}

pub type AssetStoresSchema = AssetBundlesSchema;
