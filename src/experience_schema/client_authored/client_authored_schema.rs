use crate::client_authored::{
    assets::asset_bundles_schema::AssetBundlesSchema, worlds::world_schema::WorldSchema,
};
use crate::properties::{
    compiled_property_layout_schema::CompiledPropertyLayoutsSchema, property_map::PropertyMap,
};
use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Describes unprivileged data, which can be authored from an untrustworthy client-side source.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct ClientAuthoredSchema {
    /// All assets used in the experience.
    #[serde(default)]
    #[prost(message, required, tag = "1")]
    pub asset_bundles: AssetBundlesSchema,

    /// All available worlds keyed by short world identifier. Empty string is the default world.
    #[serde(default)]
    #[prost(map = "string, message", tag = "2")]
    pub worlds: HashMap<String, WorldSchema>,

    /// Experience-wide authored defaults shared across worlds.
    #[serde(default)]
    #[prost(message, optional, tag = "3")]
    pub properties: Option<PropertyMap>,

    /// Optional versioned compiled layout hints that runtimes may load instead of repeatedly
    /// rediscovering hot field identity through dynamic property bags.
    #[serde(default)]
    #[prost(message, optional, tag = "4")]
    pub compiled_property_layouts: Option<CompiledPropertyLayoutsSchema>,
}

impl ClientAuthoredSchema {}

#[cfg(test)]
mod tests {
    use super::ClientAuthoredSchema;
    use crate::properties::compiled_property_layout_schema::{
        CURRENT_COMPILED_PROPERTY_LAYOUTS_FORMAT_VERSION, CompiledPropertyValueTypeSchema,
    };

    #[test]
    fn deserializes_worlds_from_map_wire_shape() {
        let client_authored_schema_json = r#"{
            "asset_bundles": {"bundles":{}},
            "worlds": {
                "": {
                    "objects": [],
                    "properties": {"properties":[]},
                    "state_machines": [],
                    "asset_bundle_ids": []
                }
            }
        }"#;

        let client_authored_schema =
            serde_json::from_str::<ClientAuthoredSchema>(client_authored_schema_json)
                .expect("expected map-based worlds schema to deserialize");

        assert_eq!(client_authored_schema.worlds.len(), 1);
        assert!(client_authored_schema.worlds.contains_key(""));
        assert!(client_authored_schema.properties.is_none());
    }

    #[test]
    fn deserializes_experience_scope_properties() {
        let client_authored_schema_json = r#"{
            "asset_bundles": {"bundles":{}},
            "worlds": {},
            "properties": {
                "properties": [
                    ["presentation_policy_2d", {"String":"fixed_height_reveal"}]
                ]
            }
        }"#;

        let client_authored_schema =
            serde_json::from_str::<ClientAuthoredSchema>(client_authored_schema_json)
                .expect("expected experience-scope properties to deserialize");

        assert_eq!(
            client_authored_schema
                .properties
                .as_ref()
                .expect("experience properties should deserialize")
                .get_string("presentation_policy_2d")
                .map(|value| value.as_str()),
            Some("fixed_height_reveal")
        );
        assert!(client_authored_schema.compiled_property_layouts.is_none());
    }

    #[test]
    fn deserializes_compiled_property_layouts() {
        let client_authored_schema_json = r#"{
            "asset_bundles": {"bundles":{}},
            "worlds": {},
            "compiled_property_layouts": {
                "format_version": 1,
                "layouts": [
                    {
                        "layout_id": "camera_runtime",
                        "layout_version": 1,
                        "fields": [
                            {
                                "identifier": "viewport_width_px",
                                "slot_index": 0,
                                "value_type": 5,
                                "default_value": {"Float64": 960.0}
                            }
                        ]
                    }
                ]
            }
        }"#;

        let client_authored_schema =
            serde_json::from_str::<ClientAuthoredSchema>(client_authored_schema_json)
                .expect("expected compiled property layouts to deserialize");

        let compiled_property_layouts = client_authored_schema
            .compiled_property_layouts
            .expect("compiled layouts should deserialize");
        assert_eq!(
            compiled_property_layouts.format_version,
            CURRENT_COMPILED_PROPERTY_LAYOUTS_FORMAT_VERSION
        );
        assert_eq!(compiled_property_layouts.layouts.len(), 1);
        assert_eq!(
            compiled_property_layouts.layouts[0].fields[0].value_type,
            CompiledPropertyValueTypeSchema::Float64 as i32
        );
    }
}
