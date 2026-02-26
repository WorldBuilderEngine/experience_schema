use crate::client_authored::meta_data::client_published_metadata_schema::ClientPublishedMetadataSchema;
use crate::client_authored::{assets::asset_bundles_schema::AssetBundlesSchema, worlds::world_schema::WorldSchema};
use serde::{Deserialize, Serialize};

/// Describes unprivileged data, which can be authored from an untrustworthy client-side source.
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ClientAuthoredSchema {
    #[serde(default)]
    pub metadata: ClientPublishedMetadataSchema,

    #[serde(default)]
    #[serde(alias = "asset_stores")]
    pub asset_bundles: AssetBundlesSchema,

    /// All available worlds.
    pub worlds: Vec<WorldSchema>,
}

impl ClientAuthoredSchema {}
