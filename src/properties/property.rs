use crate::assets::asset_ref::AssetRef;
use crate::prost_json_message::{
    encode_as_json_message, json_message_encoded_len, merge_from_json_message,
};
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum Property {
    Bool(bool),
    BoolArray(Vec<bool>),
    Int64(i64),
    Int64Array(Vec<i64>),
    Float64(f64),
    Float64Array(Vec<f64>),
    String(String),
    StringArray(Vec<String>),

    /// A reference to an asset by content path.
    AssetRef(AssetRef),
}

impl Message for Property {
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
        *self = Self::Bool(false);
    }
}
