use crate::prost_json_message::{
    encode_as_json_message, json_message_encoded_len, merge_from_json_message,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

fn normalize_bundle_identifier(bundle_identifier: impl Into<String>) -> String {
    bundle_identifier.into().trim().to_string()
}

/// The high-level source type for an asset bundle.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum AssetBundleKind {
    /// Authored content loaded from loose files.
    Unpacked,
    /// Runtime-ready content loaded from packed bytes.
    #[default]
    Packed,
    /// Runtime-generated content (for tests or procedural content).
    Generated,
}

/// A single serialized asset entry.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct StoredAssetSchema {
    /// Bundle-relative asset path.
    pub asset_path: PathBuf,
    /// Serialized bytes for this asset.
    #[serde(deserialize_with = "deserialize_asset_data_bytes")]
    pub asset_data: Vec<u8>,
}

impl StoredAssetSchema {
    pub fn new(asset_path: PathBuf, asset_data: Vec<u8>) -> Self {
        Self {
            asset_path,
            asset_data,
        }
    }
}

impl Message for StoredAssetSchema {
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

/// A serializable asset bundle.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct AssetBundleSchema {
    /// Unique identifier for this bundle within an experience.
    pub bundle_id: String,
    /// Bundle source classification.
    pub bundle_kind: AssetBundleKind,
    /// Serialized assets for this bundle.
    pub assets: Vec<StoredAssetSchema>,
}

impl AssetBundleSchema {
    pub fn new(bundle_id: impl Into<String>, bundle_kind: AssetBundleKind) -> Self {
        Self {
            bundle_id: normalize_bundle_identifier(bundle_id),
            bundle_kind,
            assets: Vec::new(),
        }
    }

    pub fn upsert_asset(&mut self, asset_path: PathBuf, asset_data: Vec<u8>) {
        if let Some(asset_schema_index) = self
            .assets
            .iter()
            .position(|asset_schema| asset_schema.asset_path == asset_path)
        {
            self.assets[asset_schema_index].asset_data = asset_data;
        } else {
            self.assets
                .push(StoredAssetSchema::new(asset_path, asset_data));
        }
    }

    pub fn find_asset(&self, asset_path: &Path) -> Option<&StoredAssetSchema> {
        self.assets
            .iter()
            .find(|asset_schema| asset_schema.asset_path == asset_path)
    }
}

impl Message for AssetBundleSchema {
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

fn deserialize_asset_data_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct AssetDataVisitor;

    impl<'de> Visitor<'de> for AssetDataVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a byte array or a base64-encoded byte string")
        }

        fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut bytes = Vec::new();
            while let Some(byte_value) = sequence.next_element::<u8>()? {
                bytes.push(byte_value);
            }
            Ok(bytes)
        }

        fn visit_str<E>(self, encoded_value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            BASE64_STANDARD
                .decode(encoded_value)
                .map_err(|decode_error| {
                    E::custom(format!("invalid base64 asset_data: {decode_error}"))
                })
        }

        fn visit_string<E>(self, encoded_value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_str(encoded_value.as_str())
        }

        fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(bytes.to_vec())
        }

        fn visit_byte_buf<E>(self, bytes: Vec<u8>) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(bytes)
        }
    }

    deserializer.deserialize_any(AssetDataVisitor)
}

#[cfg(test)]
mod tests {
    use super::StoredAssetSchema;
    use std::path::PathBuf;

    #[test]
    fn stored_asset_schema_deserializes_base64_asset_data() {
        let serialized = r#"{"asset_path":"sprites/example.png","asset_data":"AP9/"}"#;
        let parsed: StoredAssetSchema =
            serde_json::from_str(serialized).expect("base64 encoding should parse");

        assert_eq!(parsed.asset_path, PathBuf::from("sprites/example.png"));
        assert_eq!(parsed.asset_data, vec![0, 255, 127]);
    }
}
