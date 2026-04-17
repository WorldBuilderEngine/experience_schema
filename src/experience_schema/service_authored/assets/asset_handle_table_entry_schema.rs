use crate::service_authored::assets::AssetHandleSchema;
use prost::Message;
use serde::{Deserialize, Serialize};

/// Trusted path-to-handle lowering result for one published asset.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct AssetHandleTableEntrySchema {
    #[prost(message, required, tag = "1")]
    pub asset_handle: AssetHandleSchema,

    #[serde(default)]
    #[prost(string, optional, tag = "2")]
    pub bundle_id: Option<String>,

    #[prost(string, tag = "3")]
    pub asset_path: String,
}
