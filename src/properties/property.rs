use crate::assets::asset_ref::AssetRef;
use prost::DecodeError;
use prost::Message;
use prost::Oneof;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum Property {
    Bool(bool),
    BoolArray(Vec<bool>),
    DataBuffer(Vec<u8>),
    Int64(i64),
    Int64Array(Vec<i64>),
    Float64(f64),
    Float64Array(Vec<f64>),
    String(String),
    StringArray(Vec<String>),

    /// A reference to an asset by content path.
    AssetRef(AssetRef),
}

impl Default for Property {
    fn default() -> Self {
        Self::Bool(false)
    }
}

impl Message for Property {
    fn encode_raw(&self, buf: &mut impl BufMut) {
        PropertyBinaryWire::from(self.clone()).encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut wire = PropertyBinaryWire::from(self.clone());
        wire.merge_field(tag, wire_type, buf, ctx)?;
        *self = wire.into_property();
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        PropertyBinaryWire::from(self.clone()).encoded_len()
    }

    fn clear(&mut self) {
        *self = Self::Bool(false);
    }
}

#[derive(Clone, PartialEq, Message)]
struct BoolArrayBinaryWire {
    #[prost(bool, repeated, tag = "1")]
    values: Vec<bool>,
}

#[derive(Clone, PartialEq, Message)]
struct Int64ArrayBinaryWire {
    #[prost(int64, repeated, tag = "1")]
    values: Vec<i64>,
}

#[derive(Clone, PartialEq, Message)]
struct Float64ArrayBinaryWire {
    #[prost(double, repeated, tag = "1")]
    values: Vec<f64>,
}

#[derive(Clone, PartialEq, Message)]
struct StringArrayBinaryWire {
    #[prost(string, repeated, tag = "1")]
    values: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
struct PropertyBinaryWire {
    #[prost(
        oneof = "property_binary_wire::Value",
        tags = "16, 17, 18, 19, 20, 21, 22, 23, 24, 25"
    )]
    value: Option<property_binary_wire::Value>,
}

mod property_binary_wire {
    use super::*;

    #[derive(Clone, PartialEq, Oneof)]
    pub enum Value {
        #[prost(bool, tag = "16")]
        Bool(bool),
        #[prost(message, tag = "17")]
        BoolArray(BoolArrayBinaryWire),
        #[prost(bytes, tag = "18")]
        DataBuffer(Vec<u8>),
        #[prost(int64, tag = "19")]
        Int64(i64),
        #[prost(message, tag = "20")]
        Int64Array(Int64ArrayBinaryWire),
        #[prost(double, tag = "21")]
        Float64(f64),
        #[prost(message, tag = "22")]
        Float64Array(Float64ArrayBinaryWire),
        #[prost(string, tag = "23")]
        String(String),
        #[prost(message, tag = "24")]
        StringArray(StringArrayBinaryWire),
        #[prost(message, tag = "25")]
        AssetRef(AssetRef),
    }
}

impl From<Property> for PropertyBinaryWire {
    fn from(value: Property) -> Self {
        let value = Some(match value {
            Property::Bool(value) => property_binary_wire::Value::Bool(value),
            Property::BoolArray(values) => {
                property_binary_wire::Value::BoolArray(BoolArrayBinaryWire { values })
            }
            Property::DataBuffer(values) => property_binary_wire::Value::DataBuffer(values),
            Property::Int64(value) => property_binary_wire::Value::Int64(value),
            Property::Int64Array(values) => {
                property_binary_wire::Value::Int64Array(Int64ArrayBinaryWire { values })
            }
            Property::Float64(value) => property_binary_wire::Value::Float64(value),
            Property::Float64Array(values) => {
                property_binary_wire::Value::Float64Array(Float64ArrayBinaryWire { values })
            }
            Property::String(value) => property_binary_wire::Value::String(value),
            Property::StringArray(values) => {
                property_binary_wire::Value::StringArray(StringArrayBinaryWire { values })
            }
            Property::AssetRef(value) => property_binary_wire::Value::AssetRef(value),
        });
        Self { value }
    }
}

impl PropertyBinaryWire {
    fn into_property(self) -> Property {
        match self.value {
            Some(property_binary_wire::Value::Bool(value)) => Property::Bool(value),
            Some(property_binary_wire::Value::BoolArray(values)) => {
                Property::BoolArray(values.values)
            }
            Some(property_binary_wire::Value::DataBuffer(values)) => Property::DataBuffer(values),
            Some(property_binary_wire::Value::Int64(value)) => Property::Int64(value),
            Some(property_binary_wire::Value::Int64Array(values)) => {
                Property::Int64Array(values.values)
            }
            Some(property_binary_wire::Value::Float64(value)) => Property::Float64(value),
            Some(property_binary_wire::Value::Float64Array(values)) => {
                Property::Float64Array(values.values)
            }
            Some(property_binary_wire::Value::String(value)) => Property::String(value),
            Some(property_binary_wire::Value::StringArray(values)) => {
                Property::StringArray(values.values)
            }
            Some(property_binary_wire::Value::AssetRef(value)) => Property::AssetRef(value),
            None => Property::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Property;
    use crate::assets::asset_ref::AssetRef;
    use prost::Message;
    use std::path::PathBuf;

    #[test]
    fn prost_round_trips_property_as_binary_message() {
        let property = Property::StringArray(vec!["alpha".to_string(), "beta".to_string()]);

        let encoded = property.encode_to_vec();
        let decoded = Property::decode(encoded.as_slice()).expect("property should decode");

        assert_eq!(decoded, property);
    }

    #[test]
    fn prost_round_trips_asset_ref_property_without_json_wrapper() {
        let property = Property::AssetRef(AssetRef::new_with_bundle_id(
            "base",
            PathBuf::from("textures/example.png"),
        ));

        let encoded = property.encode_to_vec();
        let decoded =
            Property::decode(encoded.as_slice()).expect("asset ref property should decode");

        assert_eq!(decoded, property);
    }
}
