use crate::client_authored::{
    assets::asset_bundles_schema::AssetBundlesSchema,
    material_grid_2d::material_grid_rule_table_schema::MaterialGrid2dRuleTablesSchema,
    worlds::world_schema::WorldSchema,
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

    /// Optional declarative neighborhood rules that compile into deterministic dense-grid kernels.
    #[serde(default)]
    #[prost(message, optional, tag = "5")]
    pub material_grid_rule_tables: Option<MaterialGrid2dRuleTablesSchema>,
}

impl ClientAuthoredSchema {}

#[cfg(test)]
mod tests {
    use super::ClientAuthoredSchema;
    use crate::client_authored::material_grid_2d::material_grid_rule_table_schema::{
        CURRENT_MATERIAL_GRID_2D_RULE_TABLES_FORMAT_VERSION, MaterialGrid2dRuleComparisonSchema,
        MaterialGrid2dRuleWriteOperationSchema,
    };
    use crate::properties::compiled_property_layout_schema::{
        CURRENT_COMPILED_PROPERTY_LAYOUTS_FORMAT_VERSION, CompiledPropertyStorageClassSchema,
        CompiledPropertyValueTypeSchema,
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
                    "asset_bundle_ids": [],
                    "object_templates": {}
                }
            }
        }"#;

        let client_authored_schema =
            serde_json::from_str::<ClientAuthoredSchema>(client_authored_schema_json)
                .expect("expected map-based worlds schema to deserialize");

        assert_eq!(client_authored_schema.worlds.len(), 1);
        assert!(client_authored_schema.worlds.contains_key(""));
        assert!(client_authored_schema.properties.is_none());
        assert!(client_authored_schema.material_grid_rule_tables.is_none());
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
        assert!(client_authored_schema.material_grid_rule_tables.is_none());
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
                        "compiled_layout_id": 0,
                        "layout_version": 1,
                        "storage_class": 1,
                        "fields": [
                            {
                                "identifier": "viewport_width_px",
                                "compiled_field_id": 0,
                                "slot_index": 0,
                                "value_type": 4,
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
            compiled_property_layouts.layouts[0].storage_class,
            CompiledPropertyStorageClassSchema::WarmFixedRecord as i32
        );
        assert_eq!(
            compiled_property_layouts.layouts[0].fields[0].value_type,
            CompiledPropertyValueTypeSchema::Float64 as i32
        );
        assert!(client_authored_schema.material_grid_rule_tables.is_none());
    }

    #[test]
    fn deserializes_material_grid_rule_tables() {
        let client_authored_schema_json = r#"{
            "asset_bundles": {"bundles":{}},
            "worlds": {},
            "material_grid_rule_tables": {
                "format_version": 1,
                "rule_tables": [
                    {
                        "rule_table_id": "falling_sand",
                        "compiled_rule_table_id": 0,
                        "rule_table_version": 1,
                        "layout_id": "material_grid_runtime",
                        "scan_order": 0,
                        "rules": [
                            {
                                "rule_id": "fill_target",
                                "conditions": [
                                    {
                                        "field_identifier": "material_id",
                                        "offset_x": 0,
                                        "offset_y": 0,
                                        "comparison": 0,
                                        "value": {"Int64": 0}
                                    },
                                    {
                                        "field_identifier": "material_id",
                                        "offset_x": 0,
                                        "offset_y": -1,
                                        "comparison": 0,
                                        "value": {"Int64": 1}
                                    }
                                ],
                                "writes": [
                                    {
                                        "target_field_identifier": "material_id",
                                        "operation": 0,
                                        "value": {"Int64": 1},
                                        "source_field_identifier": "",
                                        "source_offset_x": 0,
                                        "source_offset_y": 0,
                                        "int_delta": 0
                                    },
                                    {
                                        "target_field_identifier": "timer_ticks",
                                        "operation": 2,
                                        "value": null,
                                        "source_field_identifier": "timer_ticks",
                                        "source_offset_x": 0,
                                        "source_offset_y": -1,
                                        "int_delta": 1
                                    }
                                ]
                            }
                        ]
                    }
                ]
            }
        }"#;

        let client_authored_schema =
            serde_json::from_str::<ClientAuthoredSchema>(client_authored_schema_json)
                .expect("expected material grid rule tables to deserialize");

        let rule_tables = client_authored_schema
            .material_grid_rule_tables
            .expect("material grid rule tables should deserialize");
        assert_eq!(
            rule_tables.format_version,
            CURRENT_MATERIAL_GRID_2D_RULE_TABLES_FORMAT_VERSION
        );
        assert_eq!(rule_tables.rule_tables.len(), 1);
        assert_eq!(
            rule_tables.rule_tables[0].rules[0].conditions[1].comparison,
            MaterialGrid2dRuleComparisonSchema::Equals as i32
        );
        assert_eq!(
            rule_tables.rule_tables[0].rules[0].writes[1].operation,
            MaterialGrid2dRuleWriteOperationSchema::AddIntDelta as i32
        );
    }
}
