use crate::assets::asset_ref::AssetRef;
use prost::{Enumeration, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Enumeration)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum StateMachineBootHandleKindSchema {
    Node = 0,
    Camera = 1,
    Asset = 2,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Message)]
pub struct StateMachineBootHandleBindingSchema {
    #[prost(string, tag = "1")]
    pub local_id: String,
    #[prost(string, tag = "2")]
    pub property_id: String,
    #[prost(enumeration = "StateMachineBootHandleKindSchema", tag = "3")]
    pub handle_kind: i32,
    #[prost(uint32, optional, tag = "4")]
    pub target_object_index: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag = "5")]
    pub target_asset_ref: Option<AssetRef>,
}

impl StateMachineBootHandleBindingSchema {
    pub fn new_object(
        local_id: impl Into<String>,
        property_id: impl Into<String>,
        handle_kind: StateMachineBootHandleKindSchema,
        target_object_index: u32,
    ) -> Self {
        Self {
            local_id: local_id.into().trim().to_string(),
            property_id: property_id.into().trim().to_string(),
            handle_kind: handle_kind as i32,
            target_object_index: Some(target_object_index),
            target_asset_ref: None,
        }
    }

    pub fn new_asset(
        local_id: impl Into<String>,
        property_id: impl Into<String>,
        target_asset_ref: AssetRef,
    ) -> Self {
        Self {
            local_id: local_id.into().trim().to_string(),
            property_id: property_id.into().trim().to_string(),
            handle_kind: StateMachineBootHandleKindSchema::Asset as i32,
            target_object_index: None,
            target_asset_ref: Some(target_asset_ref),
        }
    }

    pub fn resolved_handle_kind(&self) -> StateMachineBootHandleKindSchema {
        StateMachineBootHandleKindSchema::try_from(self.handle_kind)
            .unwrap_or(StateMachineBootHandleKindSchema::Node)
    }
}
