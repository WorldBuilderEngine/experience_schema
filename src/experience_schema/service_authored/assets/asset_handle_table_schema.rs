use crate::service_authored::assets::AssetHandleTableEntrySchema;
use prost::Message;
use serde::{Deserialize, Serialize};

/// Trusted publish-time asset-id lowering table consumed by runtime loaders.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct AssetHandleTableSchema {
    #[serde(default)]
    #[prost(message, repeated, tag = "1")]
    pub entries: Vec<AssetHandleTableEntrySchema>,
}
