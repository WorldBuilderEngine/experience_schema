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
        AssetRefBinaryWire::from(self.clone()).encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut wire = AssetRefBinaryWire::from(self.clone());
        wire.merge_field(tag, wire_type, buf, ctx)?;
        *self = wire.into_asset_ref();
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        AssetRefBinaryWire::from(self.clone()).encoded_len()
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone, PartialEq, Message)]
struct AssetRefBinaryWire {
    #[prost(string, optional, tag = "16")]
    bundle_id: Option<String>,
    #[prost(string, tag = "17")]
    asset_path: String,
}

impl From<AssetRef> for AssetRefBinaryWire {
    fn from(value: AssetRef) -> Self {
        Self {
            bundle_id: value.bundle_id,
            asset_path: value.asset_path.to_string_lossy().to_string(),
        }
    }
}

impl AssetRefBinaryWire {
    fn into_asset_ref(self) -> AssetRef {
        AssetRef {
            bundle_id: self.bundle_id.map(normalize_bundle_identifier),
            asset_path: PathBuf::from(self.asset_path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AssetRef;
    use prost::Message;
    use std::path::PathBuf;

    #[test]
    fn prost_round_trips_asset_ref_as_binary_message() {
        let asset_ref = AssetRef::new_with_bundle_id("core", PathBuf::from("sprites/player.png"));

        let encoded = asset_ref.encode_to_vec();
        let decoded = AssetRef::decode(encoded.as_slice()).expect("asset ref should decode");

        assert_eq!(decoded, asset_ref);
    }
}
