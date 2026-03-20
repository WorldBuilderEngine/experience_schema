use crate::client_authored::{
    assets::asset_bundles_schema::AssetBundlesSchema, worlds::world_schema::WorldSchema,
};
use crate::properties::property_map::PropertyMap;
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
}

impl ClientAuthoredSchema {}

#[cfg(test)]
mod tests {
    use super::ClientAuthoredSchema;

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
    }
}
