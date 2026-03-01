use crate::client_authored::{
    assets::asset_bundles_schema::AssetBundlesSchema, worlds::world_schema::WorldSchema,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Describes unprivileged data, which can be authored from an untrustworthy client-side source.
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ClientAuthoredSchema {
    #[serde(default)]
    pub asset_bundles: AssetBundlesSchema,

    /// All available worlds.
    pub worlds: Vec<WorldSchema>,

    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
}

impl ClientAuthoredSchema {}
