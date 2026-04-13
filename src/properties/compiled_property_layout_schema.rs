use crate::properties::{property::Property, property_map::PropertyMap};
use prost::{Enumeration, Message};
use serde::{Deserialize, Serialize};

pub const CURRENT_COMPILED_PROPERTY_LAYOUTS_FORMAT_VERSION: u32 = 1;
pub const CURRENT_COMPILED_PROPERTY_LAYOUT_VERSION: u32 = 1;

fn default_compiled_property_layouts_format_version() -> u32 {
    CURRENT_COMPILED_PROPERTY_LAYOUTS_FORMAT_VERSION
}

fn default_compiled_property_layout_version() -> u32 {
    CURRENT_COMPILED_PROPERTY_LAYOUT_VERSION
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Enumeration)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum CompiledPropertyStorageClassSchema {
    ColdDynamic = 0,
    WarmFixedRecord = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Enumeration)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum CompiledPropertyValueTypeSchema {
    Bool = 0,
    BoolArray = 1,
    Int64 = 2,
    Int64Array = 3,
    Float64 = 4,
    Float64Array = 5,
    String = 6,
    StringArray = 7,
    AssetRef = 8,
    UInt8 = 9,
    UInt16 = 10,
    UInt32 = 11,
    Int8 = 12,
    Int16 = 13,
    Int32 = 14,
    Float32 = 15,
}

impl CompiledPropertyValueTypeSchema {
    pub fn from_property(property: &Property) -> Self {
        match property {
            Property::Bool(_) => Self::Bool,
            Property::BoolArray(_) => Self::BoolArray,
            Property::Int64(_) => Self::Int64,
            Property::Int64Array(_) => Self::Int64Array,
            Property::Float64(_) => Self::Float64,
            Property::Float64Array(_) => Self::Float64Array,
            Property::String(_) => Self::String,
            Property::StringArray(_) => Self::StringArray,
            Property::AssetRef(_) => Self::AssetRef,
        }
    }
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct CompiledPropertyFieldSchema {
    #[prost(string, tag = "1")]
    pub identifier: String,
    #[prost(uint64, tag = "2")]
    pub compiled_field_id: u64,
    #[prost(uint32, tag = "3")]
    pub slot_index: u32,
    #[prost(enumeration = "CompiledPropertyValueTypeSchema", tag = "4")]
    pub value_type: i32,
    #[serde(default)]
    #[prost(message, optional, tag = "5")]
    pub default_value: Option<Property>,
}

impl CompiledPropertyFieldSchema {
    pub fn new(
        identifier: impl Into<String>,
        slot_index: u32,
        value_type: CompiledPropertyValueTypeSchema,
    ) -> Self {
        let identifier = identifier.into();
        Self {
            compiled_field_id: canonical_compiled_identifier(identifier.as_str()),
            identifier,
            slot_index,
            value_type: value_type as i32,
            default_value: None,
        }
    }

    pub fn with_default_value(mut self, default_value: Property) -> Self {
        self.default_value = Some(default_value);
        self
    }
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct CompiledPropertyOwnedCollectionFieldSchema {
    #[prost(string, tag = "1")]
    pub identifier: String,
    #[prost(uint64, tag = "2")]
    pub compiled_field_id: u64,
    #[prost(enumeration = "CompiledPropertyValueTypeSchema", tag = "3")]
    pub value_type: i32,
    #[serde(default)]
    #[prost(message, optional, tag = "4")]
    pub default_value: Option<Property>,
    #[prost(uint32, tag = "5")]
    pub declared_capacity: u32,
}

impl CompiledPropertyOwnedCollectionFieldSchema {
    pub fn new(
        identifier: impl Into<String>,
        value_type: CompiledPropertyValueTypeSchema,
        declared_capacity: u32,
    ) -> Self {
        let identifier = identifier.into();
        Self {
            compiled_field_id: canonical_compiled_identifier(identifier.as_str()),
            identifier,
            value_type: value_type as i32,
            default_value: None,
            declared_capacity,
        }
    }

    pub fn with_default_value(mut self, default_value: Property) -> Self {
        self.default_value = Some(default_value);
        self
    }
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct CompiledPropertyLayoutSchema {
    #[prost(string, tag = "1")]
    pub layout_id: String,
    #[prost(uint64, tag = "2")]
    pub compiled_layout_id: u64,
    #[serde(default = "default_compiled_property_layout_version")]
    #[prost(uint32, tag = "3")]
    pub layout_version: u32,
    #[serde(default)]
    #[prost(enumeration = "CompiledPropertyStorageClassSchema", tag = "4")]
    pub storage_class: i32,
    #[serde(default)]
    #[prost(message, repeated, tag = "5")]
    pub fields: Vec<CompiledPropertyFieldSchema>,
    #[serde(default)]
    #[prost(message, repeated, tag = "6")]
    pub owned_collection_fields: Vec<CompiledPropertyOwnedCollectionFieldSchema>,
}

impl CompiledPropertyLayoutSchema {
    pub fn new(
        layout_id: impl Into<String>,
        storage_class: CompiledPropertyStorageClassSchema,
    ) -> Self {
        let layout_id = layout_id.into();
        Self {
            compiled_layout_id: canonical_compiled_identifier(layout_id.as_str()),
            layout_id,
            layout_version: CURRENT_COMPILED_PROPERTY_LAYOUT_VERSION,
            storage_class: storage_class as i32,
            fields: Vec::new(),
            owned_collection_fields: Vec::new(),
        }
    }

    pub fn register_field(
        &mut self,
        identifier: impl Into<String>,
        value_type: CompiledPropertyValueTypeSchema,
        default_value: Option<Property>,
    ) -> u32 {
        let slot_index = self.fields.len() as u32;
        let mut field = CompiledPropertyFieldSchema::new(identifier, slot_index, value_type);
        field.default_value = default_value;
        self.fields.push(field);
        slot_index
    }

    pub fn register_owned_collection_field(
        &mut self,
        identifier: impl Into<String>,
        value_type: CompiledPropertyValueTypeSchema,
        declared_capacity: u32,
        default_value: Option<Property>,
    ) {
        let mut field = CompiledPropertyOwnedCollectionFieldSchema::new(
            identifier,
            value_type,
            declared_capacity,
        );
        field.default_value = default_value;
        self.owned_collection_fields.push(field);
    }

    pub fn compile_property_map_defaults(
        layout_id: impl Into<String>,
        storage_class: CompiledPropertyStorageClassSchema,
        property_map: &PropertyMap,
    ) -> Self {
        let mut layout = Self::new(layout_id, storage_class);
        for (identifier, property) in property_map {
            layout.register_field(
                identifier.clone(),
                CompiledPropertyValueTypeSchema::from_property(property),
                Some(property.clone()),
            );
        }
        layout
    }
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct CompiledPropertyLayoutsSchema {
    #[serde(default = "default_compiled_property_layouts_format_version")]
    #[prost(uint32, tag = "1")]
    pub format_version: u32,
    #[serde(default)]
    #[prost(message, repeated, tag = "2")]
    pub layouts: Vec<CompiledPropertyLayoutSchema>,
}

impl CompiledPropertyLayoutsSchema {
    pub fn current() -> Self {
        Self {
            format_version: CURRENT_COMPILED_PROPERTY_LAYOUTS_FORMAT_VERSION,
            layouts: Vec::new(),
        }
    }
}

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

fn canonical_compiled_identifier(identifier: &str) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for byte in identifier.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::{
        CURRENT_COMPILED_PROPERTY_LAYOUTS_FORMAT_VERSION, CompiledPropertyLayoutSchema,
        CompiledPropertyLayoutsSchema, CompiledPropertyStorageClassSchema,
        CompiledPropertyValueTypeSchema, canonical_compiled_identifier,
    };
    use crate::properties::{property::Property, property_map::PropertyMap};

    #[test]
    fn register_field_assigns_compiled_ids_and_contiguous_slots() {
        let mut layout = CompiledPropertyLayoutSchema::new(
            "camera_runtime",
            CompiledPropertyStorageClassSchema::WarmFixedRecord,
        );
        let first_slot = layout.register_field(
            "position_x",
            CompiledPropertyValueTypeSchema::Float64,
            Some(Property::Float64(0.0)),
        );
        let second_slot = layout.register_field(
            "position_y",
            CompiledPropertyValueTypeSchema::Float64,
            Some(Property::Float64(0.0)),
        );

        assert_eq!(first_slot, 0);
        assert_eq!(second_slot, 1);
        assert_eq!(
            layout.compiled_layout_id,
            canonical_compiled_identifier("camera_runtime")
        );
        assert_eq!(
            layout.fields[0].compiled_field_id,
            canonical_compiled_identifier("position_x")
        );
        assert_eq!(layout.fields[1].slot_index, 1);
    }

    #[test]
    fn compile_property_map_defaults_captures_default_values() {
        let mut property_map = PropertyMap::new();
        property_map.insert_float("step_delta_seconds", 0.25);
        property_map.insert_string("phase", "boot");

        let layout = CompiledPropertyLayoutSchema::compile_property_map_defaults(
            "runtime_metrics",
            CompiledPropertyStorageClassSchema::WarmFixedRecord,
            &property_map,
        );

        assert_eq!(layout.fields.len(), 2);
        assert_eq!(
            layout.fields[0].default_value,
            Some(Property::Float64(0.25))
        );
        assert_eq!(
            layout.fields[1].value_type,
            CompiledPropertyValueTypeSchema::String as i32
        );
    }

    #[test]
    fn register_owned_collection_field_captures_default_and_capacity() {
        let mut layout = CompiledPropertyLayoutSchema::new(
            "inventory_runtime",
            CompiledPropertyStorageClassSchema::WarmFixedRecord,
        );
        layout.register_owned_collection_field(
            "inventory",
            CompiledPropertyValueTypeSchema::StringArray,
            4,
            Some(Property::StringArray(vec![
                "key".to_string(),
                "map".to_string(),
            ])),
        );

        assert_eq!(layout.owned_collection_fields.len(), 1);
        assert_eq!(layout.owned_collection_fields[0].identifier, "inventory");
        assert_eq!(layout.owned_collection_fields[0].declared_capacity, 4);
        assert_eq!(
            layout.owned_collection_fields[0].default_value,
            Some(Property::StringArray(vec![
                "key".to_string(),
                "map".to_string()
            ]))
        );
    }

    #[test]
    fn compiled_property_layouts_schema_defaults_to_current_format_version() {
        let layouts = CompiledPropertyLayoutsSchema::current();
        assert_eq!(
            layouts.format_version,
            CURRENT_COMPILED_PROPERTY_LAYOUTS_FORMAT_VERSION
        );
        assert!(layouts.layouts.is_empty());
    }

    #[test]
    fn packed_narrow_value_types_have_stable_schema_discriminants() {
        assert_eq!(CompiledPropertyValueTypeSchema::UInt8 as i32, 9);
        assert_eq!(CompiledPropertyValueTypeSchema::UInt16 as i32, 10);
        assert_eq!(CompiledPropertyValueTypeSchema::UInt32 as i32, 11);
        assert_eq!(CompiledPropertyValueTypeSchema::Int8 as i32, 12);
        assert_eq!(CompiledPropertyValueTypeSchema::Int16 as i32, 13);
        assert_eq!(CompiledPropertyValueTypeSchema::Int32 as i32, 14);
        assert_eq!(CompiledPropertyValueTypeSchema::Float32 as i32, 15);
    }
}
