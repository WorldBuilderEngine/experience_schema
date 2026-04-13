use crate::client_authored::state_machines::state_machine_schema::StateMachineSchema;
use crate::client_authored::worlds::world_compatibility_schema::WorldCompatibilitySchema;
use crate::client_authored::worlds::world_object_schema::WorldObjectSchema;
use crate::properties::property_map::PropertyMap;
use crate::wire_compat::json_message::{encode_as_json_message, json_message_encoded_len};
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType, bytes, message, skip_field, string};
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

#[allow(deprecated)]
fn schema_json_decode_error(error: serde_json::Error) -> DecodeError {
    DecodeError::new(format!("schema JSON message decode failed: {error}"))
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
        match tag {
            1 => {
                let mut payload = Vec::new();
                bytes::merge(wire_type, &mut payload, buf, ctx)?;
                let first_non_whitespace_byte = payload
                    .iter()
                    .copied()
                    .find(|byte| !byte.is_ascii_whitespace());
                if matches!(first_non_whitespace_byte, Some(b'{') | Some(b'[')) {
                    *self = serde_json::from_slice::<WorldSchema>(&payload)
                        .map_err(schema_json_decode_error)?;
                    return Ok(());
                }

                let world_object_schema = WorldObjectSchema::decode(payload.as_slice())?;
                self.objects.push(world_object_schema);
                Ok(())
            }
            2 => message::merge(wire_type, &mut self.properties, buf, ctx),
            3 => message::merge_repeated(wire_type, &mut self.state_machines, buf, ctx),
            4 => string::merge_repeated(wire_type, &mut self.asset_bundle_ids, buf, ctx),
            5 => {
                let mut object_template_entry = LegacyWorldObjectTemplateEntry::default();
                message::merge(wire_type, &mut object_template_entry, buf, ctx)?;
                if let Some(world_object_schema) = object_template_entry.value {
                    self.compatibility
                        .register_object_template(object_template_entry.key, world_object_schema);
                }
                Ok(())
            }
            _ => skip_field(wire_type, tag, buf, ctx),
        }
    }

    fn encoded_len(&self) -> usize {
        json_message_encoded_len(self)
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone, PartialEq, Message)]
struct LegacyWorldObjectTemplateEntry {
    #[prost(string, tag = "1")]
    key: String,
    #[prost(message, optional, tag = "2")]
    value: Option<WorldObjectSchema>,
}

#[cfg(test)]
mod tests {
    use super::LegacyWorldObjectTemplateEntry;
    use super::WorldSchema;
    use crate::client_authored::worlds::world_object_schema::WorldObjectSchema;
    use crate::properties::property_map::PropertyMap;
    use prost::Message;

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
        assert!(world.compatibility().has_legacy_protocol_proof_assertions());
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

    #[test]
    fn decode_supports_legacy_field_based_world_wire_shape() {
        #[derive(Clone, PartialEq, Message)]
        struct LegacyWorldSchema {
            #[prost(message, repeated, tag = "1")]
            objects: Vec<WorldObjectSchema>,
            #[prost(message, required, tag = "2")]
            properties: PropertyMap,
            #[prost(message, repeated, tag = "3")]
            state_machines: Vec<
                crate::client_authored::state_machines::state_machine_schema::StateMachineSchema,
            >,
            #[prost(string, repeated, tag = "4")]
            asset_bundle_ids: Vec<String>,
            #[prost(message, repeated, tag = "5")]
            object_templates: Vec<LegacyWorldObjectTemplateEntry>,
        }

        let legacy_world = LegacyWorldSchema {
            objects: vec![WorldObjectSchema {
                kinded: None,
                properties: PropertyMap::default(),
                state_machines: Vec::new(),
            }],
            properties: PropertyMap::default(),
            state_machines: Vec::new(),
            asset_bundle_ids: vec!["ui".to_string()],
            object_templates: vec![LegacyWorldObjectTemplateEntry {
                key: "template_button".to_string(),
                value: Some(WorldObjectSchema {
                    kinded: None,
                    properties: PropertyMap::default(),
                    state_machines: Vec::new(),
                }),
            }],
        };

        let decoded_world = WorldSchema::decode(legacy_world.encode_to_vec().as_slice())
            .expect("legacy world wire shape should decode");

        assert_eq!(decoded_world.objects.len(), 1);
        assert_eq!(decoded_world.asset_bundle_ids, vec!["ui".to_string()]);
        assert!(
            decoded_world
                .object_templates()
                .contains_key("template_button")
        );
    }
}
