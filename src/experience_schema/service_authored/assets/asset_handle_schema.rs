use prost::Message;
use serde::{Deserialize, Serialize};

/// Trusted runtime identity for one asset inside the published asset table.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Message)]
pub struct AssetHandleSchema {
    #[prost(uint32, tag = "1")]
    pub bundle_index: u32,

    #[prost(uint32, tag = "2")]
    pub asset_index: u32,
}
