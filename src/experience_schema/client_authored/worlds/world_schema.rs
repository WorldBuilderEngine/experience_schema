use crate::client_authored::state_machines::state_machine_schema::StateMachineSchema;
use crate::client_authored::worlds::world_compatibility_schema::WorldCompatibilitySchema;
use crate::client_authored::worlds::world_object_schema::WorldObjectSchema;
use crate::properties::property_map::PropertyMap;
use crate::wire_compat::json_message::{
    encode_as_json_message, json_message_encoded_len, merge_from_json_message,
};
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Describes a world and its contents within this experience.
/// Note from owner: No longer differentiate 2d and 3d. This is not useful at the tail end of the schema.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct WorldSchema {
    /// Objects to be instantiated by this world.
    pub objects: Vec<WorldObjectSchema>,

    /// The custom properties (data) for this world.
    pub properties: PropertyMap,

    /// State machines (code) for this world.
    pub state_machines: Vec<StateMachineSchema>,

    /// Optional list of asset-bundle ids required by this world.
    pub asset_bundle_ids: Vec<String>,

    #[serde(default, flatten, skip_serializing)]
    compatibility: WorldCompatibilitySchema,
}

impl WorldSchema {
    pub fn compatibility(&self) -> &WorldCompatibilitySchema {
        &self.compatibility
    }

    pub fn compatibility_mut(&mut self) -> &mut WorldCompatibilitySchema {
        &mut self.compatibility
    }

    pub fn object_templates(&self) -> &HashMap<String, WorldObjectSchema> {
        self.compatibility.object_templates()
    }

    pub fn object_templates_mut(&mut self) -> &mut HashMap<String, WorldObjectSchema> {
        self.compatibility.object_templates_mut()
    }
}

impl Message for WorldSchema {
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

#[cfg(test)]
mod tests {
    use super::WorldSchema;

    #[test]
    fn deserialization_routes_legacy_world_payloads_into_compatibility_boundary() {
        let world = serde_json::from_str::<WorldSchema>(
            r#"{
                "objects": [],
                "properties": {"properties":[]},
                "state_machines": [],
                "asset_bundle_ids": [],
                "object_templates": {
                    "legacy_button": {
                        "properties": {"properties":[]},
                        "state_machines": []
                    }
                },
                "protocol_proof_assertions": [
                    {
                        "label": "legacy_contract",
                        "kind": {
                            "InvocationAllowed": {
                                "invocation": {
                                    "caller_machine_label": "caller",
                                    "callee_machine_label": "callee",
                                    "entrypoint": "boot"
                                }
                            }
                        }
                    }
                ]
            }"#,
        )
        .expect("world should deserialize");

        assert!(world.objects.is_empty());
        assert!(world.state_machines.is_empty());
        assert_eq!(world.object_templates().len(), 1);
        assert!(
            world
                .compatibility()
                .has_legacy_protocol_proof_assertions()
        );
    }

    #[test]
    fn serialization_omits_world_compatibility_payloads_from_active_shape() {
        let world = serde_json::from_str::<WorldSchema>(
            r#"{
                "objects": [],
                "properties": {"properties":[]},
                "state_machines": [],
                "asset_bundle_ids": [],
                "object_templates": {
                    "legacy_button": {
                        "properties": {"properties":[]},
                        "state_machines": []
                    }
                },
                "protocol_proof_assertions": [
                    { "label": "legacy_contract" }
                ]
            }"#,
        )
        .expect("legacy payload should deserialize");

        let serialized = serde_json::to_value(&world).expect("world should serialize");
        assert!(serialized.get("object_templates").is_none());
        assert!(serialized.get("protocol_proof_assertions").is_none());
    }
}
