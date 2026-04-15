use crate::assets::asset_ref::AssetRef;
use crate::properties::property::Property;
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::slice::Iter;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct PropertyMap {
    pub properties: Vec<(String, Property)>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct KeyedDataBufferWrite {
    pub key: String,
    pub bytes: Vec<u8>,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct KeyedDataBufferWrites {
    pub writes: Vec<KeyedDataBufferWrite>,
}

#[derive(Deserialize, Serialize)]
struct SerializedPropertyMap {
    #[serde(default)]
    properties: Vec<(String, Property)>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    data_buffer_writes: Vec<KeyedDataBufferWrite>,
}

impl Serialize for PropertyMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerializedPropertyMap {
            properties: self
                .properties
                .iter()
                .filter(|(_, property)| !matches!(property, Property::DataBuffer(_)))
                .cloned()
                .collect(),
            data_buffer_writes: self.keyed_data_buffer_writes().writes,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PropertyMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serialized_property_map = SerializedPropertyMap::deserialize(deserializer)?;
        let mut property_map = Self::new();

        for (key, property) in serialized_property_map.properties {
            property_map.insert(key, property);
        }

        property_map.apply_keyed_data_buffer_writes(&KeyedDataBufferWrites {
            writes: serialized_property_map.data_buffer_writes,
        });

        Ok(property_map)
    }
}

impl PropertyMap {
    /// Creates a new empty PropertyMap.
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
        }
    }

    pub fn keyed_data_buffer_writes(&self) -> KeyedDataBufferWrites {
        KeyedDataBufferWrites {
            writes: self
                .properties
                .iter()
                .filter_map(|(key, property)| match property {
                    Property::DataBuffer(bytes) => Some(KeyedDataBufferWrite {
                        key: key.clone(),
                        bytes: bytes.clone(),
                    }),
                    _ => None,
                })
                .collect(),
        }
    }

    pub fn apply_keyed_data_buffer_writes(&mut self, writes: &KeyedDataBufferWrites) {
        for write in &writes.writes {
            self.insert_data_buffer(write.key.clone(), write.bytes.clone());
        }
    }

    pub fn from_keyed_data_buffer_writes(writes: &KeyedDataBufferWrites) -> Self {
        let mut property_map = Self::new();
        property_map.apply_keyed_data_buffer_writes(writes);
        property_map
    }

    /// Checks whether a key exists.
    pub fn contains(&self, key: &str) -> bool {
        self.properties
            .iter()
            .any(|(next_key, _property)| next_key == key)
    }

    /// Returns the number of stored properties.
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Checks if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// Gets a reference to a property by key.
    pub fn get(&self, key: &str) -> Option<&Property> {
        self.properties
            .iter()
            .find(|(next_key, _property)| next_key == key)
            .map(|(_key, property)| property)
    }

    /// Removes a property by key.
    pub fn remove(&mut self, key: &str) -> Option<Property> {
        self.properties
            .iter()
            .position(|(next_key, _property)| next_key == key)
            .map(|index| self.properties.remove(index).1)
    }

    /// Inserts a property by key.
    pub fn insert(&mut self, key: impl Into<String>, property: Property) {
        let key_string = key.into();

        if let Some(index) = self
            .properties
            .iter()
            .position(|(next_key, _)| next_key == &key_string)
        {
            self.properties[index].1 = property;
        } else {
            self.properties.push((key_string, property));
        }
    }

    /// Inserts a boolean value.
    pub fn insert_bool(&mut self, key: impl Into<String>, value: bool) {
        self.insert(key, Property::Bool(value));
    }

    /// Inserts an integer value.
    pub fn insert_int(&mut self, key: impl Into<String>, value: i64) {
        self.insert(key, Property::Int64(value));
    }

    /// Inserts a float value.
    pub fn insert_float(&mut self, key: impl Into<String>, value: f64) {
        self.insert(key, Property::Float64(value));
    }

    /// Inserts a string value.
    pub fn insert_string(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.insert(key, Property::String(value.into()));
    }

    /// Inserts an asset reference.
    pub fn insert_asset_ref(&mut self, key: impl Into<String>, value: AssetRef) {
        self.insert(key, Property::AssetRef(value));
    }

    /// Inserts a boolean array value.
    pub fn insert_bool_array(&mut self, key: impl Into<String>, values: Vec<bool>) {
        self.insert(key, Property::BoolArray(values));
    }

    /// Inserts a uint8 array value.
    pub fn insert_data_buffer(&mut self, key: impl Into<String>, values: Vec<u8>) {
        self.insert(key, Property::DataBuffer(values));
    }

    /// Inserts an integer array value.
    pub fn insert_int_array(&mut self, key: impl Into<String>, values: Vec<i64>) {
        self.insert(key, Property::Int64Array(values));
    }

    /// Inserts a float array value.
    pub fn insert_float_array(&mut self, key: impl Into<String>, values: Vec<f64>) {
        self.insert(key, Property::Float64Array(values));
    }

    /// Inserts a string array value.
    pub fn insert_string_array(&mut self, key: impl Into<String>, values: Vec<String>) {
        self.insert(key, Property::StringArray(values));
    }

    /// Gets a bool value by key (if it exists and matches type).
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.get(key) {
            Some(Property::Bool(value)) => Some(*value),
            _ => None,
        }
    }

    /// Gets an i64 value by key.
    pub fn get_int(&self, key: &str) -> Option<i64> {
        match self.get(key) {
            Some(Property::Int64(value)) => Some(*value),
            _ => None,
        }
    }

    /// Gets a f64 value by key.
    pub fn get_float(&self, key: &str) -> Option<f64> {
        match self.get(key) {
            Some(Property::Float64(value)) => Some(*value),
            _ => None,
        }
    }

    /// Gets a string value by key.
    pub fn get_string(&self, key: &str) -> Option<&String> {
        match self.get(key) {
            Some(Property::String(value)) => Some(value),
            _ => None,
        }
    }

    /// Gets an asset ref value by key.
    pub fn get_asset_ref(&self, key: &str) -> Option<&AssetRef> {
        match self.get(key) {
            Some(Property::AssetRef(value)) => Some(value),
            _ => None,
        }
    }

    /// Gets a bool array value by key.
    pub fn get_bool_array(&self, key: &str) -> Option<&Vec<bool>> {
        match self.get(key) {
            Some(Property::BoolArray(values)) => Some(values),
            _ => None,
        }
    }

    /// Gets a uint8 array value by key.
    pub fn get_data_buffer(&self, key: &str) -> Option<&Vec<u8>> {
        match self.get(key) {
            Some(Property::DataBuffer(values)) => Some(values),
            _ => None,
        }
    }

    /// Gets an i64 array value by key.
    pub fn get_int_array(&self, key: &str) -> Option<&Vec<i64>> {
        match self.get(key) {
            Some(Property::Int64Array(values)) => Some(values),
            _ => None,
        }
    }

    /// Gets a f64 array value by key.
    pub fn get_float_array(&self, key: &str) -> Option<&Vec<f64>> {
        match self.get(key) {
            Some(Property::Float64Array(values)) => Some(values),
            _ => None,
        }
    }

    /// Gets a string array value by key.
    pub fn get_string_array(&self, key: &str) -> Option<&Vec<String>> {
        match self.get(key) {
            Some(Property::StringArray(values)) => Some(values),
            _ => None,
        }
    }
}

impl<'lifetime> IntoIterator for &'lifetime PropertyMap {
    type Item = (&'lifetime String, &'lifetime Property);
    type IntoIter = std::iter::Map<
        Iter<'lifetime, (String, Property)>,
        fn(&'lifetime (String, Property)) -> (&'lifetime String, &'lifetime Property),
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.properties
            .iter()
            .map(|(key, property)| (key, property))
    }
}

impl<'lifetime> IntoIterator for &'lifetime mut PropertyMap {
    type Item = (&'lifetime String, &'lifetime Property);
    type IntoIter = std::iter::Map<
        Iter<'lifetime, (String, Property)>,
        fn(&'lifetime (String, Property)) -> (&'lifetime String, &'lifetime Property),
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.properties
            .iter()
            .map(|(key, property)| (key, property))
    }
}

impl Message for PropertyMap {
    fn encode_raw(&self, buf: &mut impl BufMut) {
        PropertyMapBinaryWire::from(self.clone()).encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut wire = PropertyMapBinaryWire::from(self.clone());
        wire.merge_field(tag, wire_type, buf, ctx)?;
        *self = wire.into_property_map();
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        PropertyMapBinaryWire::from(self.clone()).encoded_len()
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone, PartialEq, Message)]
struct PropertyMapEntryBinaryWire {
    #[prost(string, tag = "1")]
    key: String,
    #[prost(message, optional, tag = "2")]
    property: Option<Property>,
}

#[derive(Clone, PartialEq, Message)]
struct KeyedDataBufferWriteBinaryWire {
    #[prost(string, tag = "1")]
    key: String,
    #[prost(bytes, tag = "2")]
    bytes: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
struct PropertyMapBinaryWire {
    #[prost(message, repeated, tag = "16")]
    properties: Vec<PropertyMapEntryBinaryWire>,
    #[prost(message, repeated, tag = "17")]
    data_buffer_writes: Vec<KeyedDataBufferWriteBinaryWire>,
}

impl From<PropertyMap> for PropertyMapBinaryWire {
    fn from(value: PropertyMap) -> Self {
        let mut properties = Vec::new();
        let mut data_buffer_writes = Vec::new();
        for (key, property) in value.properties {
            match property {
                Property::DataBuffer(bytes) => data_buffer_writes.push(KeyedDataBufferWriteBinaryWire { key, bytes }),
                other_property => properties.push(PropertyMapEntryBinaryWire {
                    key,
                    property: Some(other_property),
                }),
            }
        }

        Self {
            properties,
            data_buffer_writes,
        }
    }
}

impl PropertyMapBinaryWire {
    fn into_property_map(self) -> PropertyMap {
        let mut property_map = PropertyMap::new();
        for entry in self.properties {
            let Some(property) = entry.property else {
                continue;
            };
            property_map.insert(entry.key, property);
        }
        for write in self.data_buffer_writes {
            property_map.insert_data_buffer(write.key, write.bytes);
        }
        property_map
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyedDataBufferWrites, PropertyMap};
    use crate::properties::property::Property;
    use prost::Message;

    #[test]
    fn serde_emits_keyed_data_buffer_writes_for_publish_contract_consumers() {
        let mut property_map = PropertyMap::new();
        property_map.insert_string("title", "hello");
        property_map.insert_data_buffer("payload_bytes", vec![1, 2, 3]);

        let serialized =
            serde_json::to_value(&property_map).expect("property map should serialize");

        assert_eq!(
            serialized
                .get("data_buffer_writes")
                .and_then(|value| value.as_array())
                .map(|writes| writes.len()),
            Some(1)
        );
        assert_eq!(
            serialized
                .get("data_buffer_writes")
                .and_then(|value| value.get(0))
                .and_then(|value| value.get("key"))
                .and_then(|value| value.as_str()),
            Some("payload_bytes")
        );
        assert_eq!(
            serialized
                .get("properties")
                .and_then(|value| value.as_array())
                .map(|properties| properties.len()),
            Some(1)
        );
    }

    #[test]
    fn serde_accepts_keyed_data_buffer_writes_without_generic_property_entries() {
        let serialized = serde_json::json!({
            "properties": [
                ["title", {"String":"hello"}]
            ],
            "data_buffer_writes": [
                {
                    "key": "payload_bytes",
                    "bytes": [1, 2, 3]
                }
            ]
        });

        let property_map: PropertyMap =
            serde_json::from_value(serialized).expect("property map should deserialize");

        assert_eq!(
            property_map.get("title"),
            Some(&Property::String("hello".to_string()))
        );
        assert_eq!(
            property_map.get_data_buffer("payload_bytes"),
            Some(&vec![1, 2, 3])
        );
    }

    #[test]
    fn keyed_data_buffer_write_batches_round_trip_as_whole_buffer_updates() {
        let writes = KeyedDataBufferWrites {
            writes: vec![super::KeyedDataBufferWrite {
                key: "payload_bytes".to_string(),
                bytes: vec![4, 5, 6],
            }],
        };

        let property_map = PropertyMap::from_keyed_data_buffer_writes(&writes);

        assert_eq!(
            property_map.get_data_buffer("payload_bytes"),
            Some(&vec![4, 5, 6])
        );
    }

    #[test]
    fn prost_round_trips_property_map_as_binary_message() {
        let mut property_map = PropertyMap::new();
        property_map.insert_string("title", "hello");
        property_map.insert_data_buffer("payload_bytes", vec![1, 2, 3]);

        let encoded = property_map.encode_to_vec();
        let decoded = PropertyMap::decode(encoded.as_slice()).expect("property map should decode");

        assert_eq!(decoded, property_map);
    }

}
