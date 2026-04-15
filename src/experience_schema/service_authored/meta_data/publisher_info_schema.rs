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
        PublisherInfoSchemaBinaryWire::from(self.clone()).encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut wire = PublisherInfoSchemaBinaryWire::from(self.clone());
        wire.merge_field(tag, wire_type, buf, ctx)?;
        *self = wire.into_schema();
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        PublisherInfoSchemaBinaryWire::from(self.clone()).encoded_len()
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone, PartialEq, Message)]
struct PublisherInfoSchemaBinaryWire {
    #[prost(string, optional, tag = "16")]
    publisher_id: Option<String>,
    #[prost(string, optional, tag = "17")]
    experience_id: Option<String>,
    #[prost(string, optional, tag = "18")]
    publish_id: Option<String>,
    #[prost(uint64, optional, tag = "19")]
    publish_version: Option<u64>,
    #[prost(uint64, optional, tag = "20")]
    published_at_unix_seconds: Option<u64>,
}

impl From<PublisherInfoSchema> for PublisherInfoSchemaBinaryWire {
    fn from(value: PublisherInfoSchema) -> Self {
        Self {
            publisher_id: value.publisher_id,
            experience_id: value.experience_id,
            publish_id: value.publish_id,
            publish_version: value.publish_version.map(|value| value as u64),
            published_at_unix_seconds: value.published_at_unix_seconds,
        }
    }
}

impl PublisherInfoSchemaBinaryWire {
    fn into_schema(self) -> PublisherInfoSchema {
        PublisherInfoSchema {
            publisher_id: self.publisher_id,
            experience_id: self.experience_id,
            publish_id: self.publish_id,
            publish_version: self.publish_version.map(|value| value as usize),
            published_at_unix_seconds: self.published_at_unix_seconds,
        }
    }
}
