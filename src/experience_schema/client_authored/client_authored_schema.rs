use crate::client_authored::{
    assets::asset_bundles_schema::AssetBundlesSchema, worlds::world_schema::WorldSchema,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

/// Describes unprivileged data, which can be authored from an untrustworthy client-side source.
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ClientAuthoredSchema {
    /// All assets used in the experience.
    #[serde(default)]
    pub asset_bundles: AssetBundlesSchema,

    /// All available worlds keyed by short world identifier. Empty string is the default world.
    #[serde(default, deserialize_with = "deserialize_worlds_schema")]
    pub worlds: HashMap<String, WorldSchema>,

    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
}

impl ClientAuthoredSchema {}

#[derive(Deserialize)]
#[serde(untagged)]
enum WorldsSchemaWireFormat {
    Map(HashMap<String, WorldSchema>),
    Sequence(Vec<WorldSchema>),
}

fn deserialize_worlds_schema<'de, D>(deserializer: D) -> Result<HashMap<String, WorldSchema>, D::Error>
where
    D: Deserializer<'de>,
{
    let worlds_schema_wire_format = WorldsSchemaWireFormat::deserialize(deserializer)?;
    Ok(match worlds_schema_wire_format {
        WorldsSchemaWireFormat::Map(worlds_schema_map) => worlds_schema_map,
        WorldsSchemaWireFormat::Sequence(worlds_schema_sequence) => {
            worlds_schema_sequence
                .into_iter()
                .enumerate()
                .map(|(world_index, world_schema)| {
                    let world_identifier = if world_index == 0 {
                        String::new()
                    } else {
                        format!("legacy_world_{:08}", world_index)
                    };
                    (world_identifier, world_schema)
                })
                .collect::<HashMap<String, WorldSchema>>()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::ClientAuthoredSchema;

    #[test]
    fn deserializes_worlds_from_map_wire_shape() {
        let client_authored_schema_json = r#"{
            "asset_bundles": {"bundles":[]},
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

        let client_authored_schema = serde_json::from_str::<ClientAuthoredSchema>(client_authored_schema_json)
            .expect("expected map-based worlds schema to deserialize");

        assert_eq!(client_authored_schema.worlds.len(), 1);
        assert!(client_authored_schema.worlds.contains_key(""));
    }

    #[test]
    fn deserializes_worlds_from_legacy_sequence_wire_shape() {
        let client_authored_schema_json = r#"{
            "asset_bundles": {"bundles":[]},
            "worlds": [
                {
                    "objects": [],
                    "properties": {"properties":[]},
                    "state_machines": [],
                    "asset_bundle_ids": [],
                    "object_templates": {}
                },
                {
                    "objects": [],
                    "properties": {"properties":[]},
                    "state_machines": [],
                    "asset_bundle_ids": [],
                    "object_templates": {}
                }
            ]
        }"#;

        let client_authored_schema = serde_json::from_str::<ClientAuthoredSchema>(client_authored_schema_json)
            .expect("expected sequence-based worlds schema to deserialize");

        assert_eq!(client_authored_schema.worlds.len(), 2);
        assert!(client_authored_schema.worlds.contains_key(""));
        assert!(client_authored_schema.worlds.contains_key("legacy_world_00000001"));
    }
}
