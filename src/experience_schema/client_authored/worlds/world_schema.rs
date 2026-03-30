use crate::client_authored::state_machines::state_machine_schema::StateMachineSchema;
use crate::client_authored::worlds::world_object_schema::WorldObjectSchema;
use crate::client_authored::worlds::world_protocol_proof_schema::WorldProtocolProofAssertionSchema;
use crate::properties::property_map::PropertyMap;
use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Describes a world and its contents within this experience.
/// Note from owner: No longer differentiate 2d and 3d. This is not useful at the tail end of the schema.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct WorldSchema {
    /// Objects to be instantiated by this world.
    #[prost(message, repeated, tag = "1")]
    pub objects: Vec<WorldObjectSchema>,

    /// The custom properties (data) for this world.
    #[prost(message, required, tag = "2")]
    pub properties: PropertyMap,

    /// State machines (code) for this world.
    #[prost(message, repeated, tag = "3")]
    pub state_machines: Vec<StateMachineSchema>,

    /// Optional list of asset-bundle ids required by this world.
    #[prost(string, repeated, tag = "4")]
    pub asset_bundle_ids: Vec<String>,

    /// Reusable object templates that can be instantiated at runtime.
    #[prost(map = "string, message", tag = "5")]
    pub object_templates: HashMap<String, WorldObjectSchema>,

    /// World-scope protocol assertions over cooperating machine contracts.
    #[serde(default)]
    #[prost(message, repeated, tag = "6")]
    pub protocol_proof_assertions: Vec<WorldProtocolProofAssertionSchema>,
}

impl WorldSchema {
    pub fn register_protocol_proof_assertion(
        &mut self,
        assertion: WorldProtocolProofAssertionSchema,
    ) {
        self.protocol_proof_assertions.push(assertion);
    }
}

#[cfg(test)]
mod tests {
    use super::WorldSchema;
    use crate::client_authored::worlds::world_protocol_proof_schema::{
        WorldProtocolInvocationEventSchema, WorldProtocolProofAssertionKindSchema,
        WorldProtocolProofAssertionSchema,
    };
    use std::collections::HashMap;

    #[test]
    fn deserialization_defaults_protocol_proof_assertions_to_empty() {
        let world = serde_json::from_str::<WorldSchema>(
            r#"{
                "objects": [],
                "properties": {"properties":[]},
                "state_machines": [],
                "asset_bundle_ids": [],
                "object_templates": {}
            }"#,
        )
        .expect("world should deserialize");

        assert!(world.protocol_proof_assertions.is_empty());
    }

    #[test]
    fn register_protocol_proof_assertion_appends_contract() {
        let mut world = WorldSchema {
            objects: Vec::new(),
            properties: Default::default(),
            state_machines: Vec::new(),
            asset_bundle_ids: Vec::new(),
            object_templates: HashMap::new(),
            protocol_proof_assertions: Vec::new(),
        };

        world.register_protocol_proof_assertion(WorldProtocolProofAssertionSchema {
            label: Some("resolver_calls_rules".to_string()),
            kind: WorldProtocolProofAssertionKindSchema::InvocationAllowed {
                invocation: WorldProtocolInvocationEventSchema {
                    caller_machine_label: "combat:resolver".to_string(),
                    callee_machine_label: "combat:rules".to_string(),
                    entrypoint: "apply_damage".to_string(),
                },
            },
        });

        assert_eq!(world.protocol_proof_assertions.len(), 1);
    }
}
