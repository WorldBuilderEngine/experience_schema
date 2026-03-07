use crate::client_authored::{
    assets::asset_bundles_schema::AssetBundlesSchema, worlds::world_schema::WorldSchema,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Describes unprivileged data, which can be authored from an untrustworthy client-side source.
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ClientAuthoredSchema {
    /// All assets used in the experience.
    #[serde(default)]
    pub asset_bundles: AssetBundlesSchema,

    /// All available worlds keyed by short world identifier. Empty string is the default world.
    #[serde(default)]
    pub worlds: HashMap<String, WorldSchema>,

}

impl ClientAuthoredSchema {}

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

}
