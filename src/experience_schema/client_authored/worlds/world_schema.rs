use crate::client_authored::state_machines::state_machine_schema::StateMachineSchema;
use crate::client_authored::worlds::world_compatibility_schema::WorldCompatibilitySchema;
use crate::client_authored::worlds::world_object_schema::WorldObjectSchema;
use crate::properties::property_map::PropertyMap;
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType, message, skip_field, string};
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
        WorldSchemaBinaryWire::from(self.clone()).encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        match tag {
            16 => message::merge_repeated(wire_type, &mut self.objects, buf, ctx),
            17 => message::merge(wire_type, &mut self.properties, buf, ctx),
            18 => message::merge_repeated(wire_type, &mut self.state_machines, buf, ctx),
            19 => string::merge_repeated(wire_type, &mut self.asset_bundle_ids, buf, ctx),
            20 => {
                let mut object_template_entry = WorldObjectTemplateEntryBinaryWire::default();
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
        WorldSchemaBinaryWire::from(self.clone()).encoded_len()
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone, PartialEq, Message)]
struct WorldObjectTemplateEntryBinaryWire {
    #[prost(string, tag = "1")]
    key: String,
    #[prost(message, optional, tag = "2")]
    value: Option<WorldObjectSchema>,
}

#[derive(Clone, PartialEq, Message)]
struct WorldSchemaBinaryWire {
    #[prost(message, repeated, tag = "16")]
    objects: Vec<WorldObjectSchema>,
    #[prost(message, optional, tag = "17")]
    properties: Option<PropertyMap>,
    #[prost(message, repeated, tag = "18")]
    state_machines: Vec<crate::client_authored::state_machines::state_machine_schema::StateMachineSchema>,
    #[prost(string, repeated, tag = "19")]
    asset_bundle_ids: Vec<String>,
    #[prost(message, repeated, tag = "20")]
    object_templates: Vec<WorldObjectTemplateEntryBinaryWire>,
}

impl From<WorldSchema> for WorldSchemaBinaryWire {
    fn from(value: WorldSchema) -> Self {
        let WorldSchema {
            objects,
            properties,
            state_machines,
            asset_bundle_ids,
            compatibility,
        } = value;
        Self {
            objects,
            properties: Some(properties),
            state_machines,
            asset_bundle_ids,
            object_templates: compatibility
                .object_templates()
                .iter()
                .map(|(key, value)| WorldObjectTemplateEntryBinaryWire {
                    key: key.clone(),
                    value: Some(value.clone()),
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WorldSchema;
    use crate::client_authored::worlds::world_object_schema::WorldObjectSchema;
    use crate::properties::property_map::PropertyMap;
    use prost::Message;

    #[test]
    fn prost_round_trips_world_schema_as_binary_message() {
        let mut world = WorldSchema {
            objects: vec![WorldObjectSchema {
                kinded: None,
                properties: PropertyMap::default(),
                state_machines: Vec::new(),
            }],
            properties: PropertyMap::default(),
            state_machines: Vec::new(),
            asset_bundle_ids: vec!["ui".to_string()],
            compatibility: super::WorldCompatibilitySchema::default(),
        };
        world.object_templates_mut().insert(
            "template_button".to_string(),
            WorldObjectSchema {
                kinded: None,
                properties: PropertyMap::default(),
                state_machines: Vec::new(),
            },
        );

        let encoded = world.encode_to_vec();
        let decoded = WorldSchema::decode(encoded.as_slice()).expect("world schema should decode");

        assert_eq!(decoded, world);
    }

}
