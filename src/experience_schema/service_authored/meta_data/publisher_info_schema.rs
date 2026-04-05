use crate::wire_compat::json_message::{
    encode_as_json_message, json_message_encoded_len, merge_from_json_message,
};
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct PublisherInfoSchema {
    #[serde(default)]
    pub publisher_id: Option<String>,
    #[serde(default)]
    pub experience_id: Option<String>,
    #[serde(default)]
    pub publish_id: Option<String>,
    #[serde(default)]
    pub publish_version: Option<usize>,
    #[serde(default)]
    pub published_at_unix_seconds: Option<u64>,
}

impl PublisherInfoSchema {}

impl Message for PublisherInfoSchema {
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
