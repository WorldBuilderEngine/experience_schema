use crate::assets::asset_ref::AssetRef;
use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct CursorSetSchema {
    #[serde(default)]
    #[prost(message, required, tag = "1")]
    pub idle: AssetRef,

    #[serde(default)]
    #[prost(message, required, tag = "2")]
    pub point: AssetRef,

    #[serde(default)]
    #[prost(message, required, tag = "3")]
    pub pressed: AssetRef,

    #[serde(default)]
    #[prost(message, required, tag = "4")]
    pub drag: AssetRef,

    #[serde(default)]
    #[prost(uint32, tag = "5")]
    pub hotspot_x_px: u32,

    #[serde(default)]
    #[prost(uint32, tag = "6")]
    pub hotspot_y_px: u32,
}
