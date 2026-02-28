use crate::shared::state_machine_node_schema::{
    StateMachineNodeSchema, StateMachineNodeTypeSchema,
};
use crate::shared::state_machine_transition_schema::StateMachineTransitionSchema;
use properties::property_map::PropertyMap;
use serde::{Deserialize, Serialize};

pub const CURRENT_STATE_MACHINE_SCHEMA_VERSION: &str = "2.0.0";
pub const LEGACY_STATE_MACHINE_SCHEMA_VERSION: &str = "1.0.0";
pub const CURRENT_RUNTIME_SNAPSHOT_SCHEMA_VERSION: &str = "2.0.0";
pub const LEGACY_RUNTIME_SNAPSHOT_SCHEMA_VERSION: &str = "1.0.0";

fn default_state_machine_schema_version() -> String {
    LEGACY_STATE_MACHINE_SCHEMA_VERSION.to_string()
}

fn default_runtime_snapshot_schema_version() -> String {
    LEGACY_RUNTIME_SNAPSHOT_SCHEMA_VERSION.to_string()
}

/// Serializable state-machine definition used in authored world schemas.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StateMachineSchema {
    #[serde(default = "default_state_machine_schema_version")]
    pub schema_version: String,
    #[serde(default = "default_runtime_snapshot_schema_version")]
    pub runtime_snapshot_version: String,
    pub initial_state_name: String,
    #[serde(default)]
    pub deterministic_seed: u64,
    #[serde(default)]
    pub property_maps: Vec<(String, PropertyMap)>,
    #[serde(default)]
    pub nodes: Vec<StateMachineNodeSchema>,
}

impl Default for StateMachineSchema {
    fn default() -> Self {
        Self::new("")
    }
}

impl StateMachineSchema {
    pub fn new(initial_state_name: impl Into<String>) -> Self {
        Self::new_with_seed(initial_state_name, 0)
    }

    pub fn new_with_seed(initial_state_name: impl Into<String>, deterministic_seed: u64) -> Self {
        Self {
            schema_version: CURRENT_STATE_MACHINE_SCHEMA_VERSION.to_string(),
            runtime_snapshot_version: CURRENT_RUNTIME_SNAPSHOT_SCHEMA_VERSION.to_string(),
            initial_state_name: initial_state_name.into(),
            deterministic_seed,
            property_maps: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn add_transition(
        &mut self,
        api_identifier: impl Into<String>,
        transition: StateMachineTransitionSchema,
    ) {
        self.nodes
            .push(StateMachineNodeSchema::new_with_transitions(
                transition.from_state_name.clone(),
                StateMachineNodeTypeSchema::ApiDispatch {
                    api_identifier: api_identifier.into(),
                    args_property_map_id: None,
                },
                vec![transition],
            ));
    }

    pub fn register_api_dispatch_node(
        &mut self,
        state_name: impl Into<String>,
        api_identifier: impl Into<String>,
        args_property_map_id: Option<String>,
    ) {
        self.nodes.push(StateMachineNodeSchema::new(
            state_name,
            StateMachineNodeTypeSchema::ApiDispatch {
                api_identifier: api_identifier.into(),
                args_property_map_id,
            },
        ));
    }

    pub fn register_property_map(
        &mut self,
        property_map_id: impl Into<String>,
        property_map: PropertyMap,
    ) {
        let property_map_id_string = property_map_id.into().trim().to_string();
        if let Some(existing_property_map_index) =
            self.property_maps
                .iter()
                .position(|(existing_property_map_id, _)| {
                    existing_property_map_id == &property_map_id_string
                })
        {
            self.property_maps[existing_property_map_index].1 = property_map;
            return;
        }

        self.property_maps
            .push((property_map_id_string, property_map));
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CURRENT_RUNTIME_SNAPSHOT_SCHEMA_VERSION, CURRENT_STATE_MACHINE_SCHEMA_VERSION, LEGACY_RUNTIME_SNAPSHOT_SCHEMA_VERSION,
        LEGACY_STATE_MACHINE_SCHEMA_VERSION, StateMachineSchema,
    };

    #[test]
    fn constructor_defaults_to_current_versions() {
        let schema = StateMachineSchema::new("idle");
        assert_eq!(schema.schema_version, CURRENT_STATE_MACHINE_SCHEMA_VERSION);
        assert_eq!(
            schema.runtime_snapshot_version,
            CURRENT_RUNTIME_SNAPSHOT_SCHEMA_VERSION
        );
    }

    #[test]
    fn legacy_payload_without_version_fields_defaults_to_n_minus_one() {
        let schema = serde_json::from_value::<StateMachineSchema>(serde_json::json!({
            "initial_state_name": "idle",
            "deterministic_seed": 7,
            "property_maps": [],
            "nodes": []
        }))
        .expect("legacy state-machine schema fixture should parse");

        assert_eq!(schema.schema_version, LEGACY_STATE_MACHINE_SCHEMA_VERSION);
        assert_eq!(
            schema.runtime_snapshot_version,
            LEGACY_RUNTIME_SNAPSHOT_SCHEMA_VERSION
        );
    }
}
