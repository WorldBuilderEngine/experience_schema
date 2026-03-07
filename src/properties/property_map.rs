use crate::assets::asset_ref::AssetRef;
use crate::properties::property::Property;
use crate::prost_json_message::{
    encode_as_json_message, json_message_encoded_len, merge_from_json_message,
};
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq)]
pub struct PropertyMap {
    pub properties: Vec<(String, Property)>,
}

impl PropertyMap {
    /// Creates a new empty PropertyMap.
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
        }
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
