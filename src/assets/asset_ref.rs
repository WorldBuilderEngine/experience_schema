use crate::wire_compat::json_message::{
    encode_as_json_message, json_message_encoded_len, merge_from_json_message,
};
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn normalize_bundle_identifier(bundle_identifier: impl Into<String>) -> String {
    bundle_identifier.into().trim().to_string()
}

#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq)]
pub struct AssetRef {
    bundle_id: Option<String>,
    asset_path: PathBuf,
}

impl AssetRef {
    /// Creates a new asset reference from a relative path.
    pub fn new(asset_path: PathBuf) -> Self {
        Self {
            bundle_id: None,
            asset_path,
        }
    }

    /// Creates a new asset reference from a bundle identifier and relative path.
    pub fn new_with_bundle_id(bundle_id: impl Into<String>, asset_path: PathBuf) -> Self {
        Self {
            bundle_id: Some(normalize_bundle_identifier(bundle_id)),
            asset_path,
        }
    }

    /// Creates a new asset reference from a store identifier and relative path.
    pub fn new_with_store_id(store_id: impl Into<String>, asset_path: PathBuf) -> Self {
        Self::new_with_bundle_id(store_id, asset_path)
    }

    /// Gets the optional bundle identifier this reference targets.
    pub fn get_bundle_id(&self) -> Option<&str> {
        self.bundle_id.as_deref()
    }

    /// Gets the optional store identifier this reference targets.
    pub fn get_store_id(&self) -> Option<&str> {
        self.get_bundle_id()
    }

    /// Gets the relative path to the referenced asset.
    pub fn get_asset_path(&self) -> &PathBuf {
        &self.asset_path
    }
}

impl Message for AssetRef {
    fn encode_raw(&self, buf: &mut impl BufMut) {
        encode_as_json_message(self, buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        merge_from_json_message(self, tag, wire_type, buf, ctx)
    }

    fn encoded_len(&self) -> usize {
        json_message_encoded_len(self)
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}
