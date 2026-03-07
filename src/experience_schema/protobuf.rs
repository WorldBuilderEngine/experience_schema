use anyhow::Context;
use prost::Message;
use std::collections::HashMap;

use crate::experience_schema::client_authored::{
    assets::asset_bundles_schema::AssetBundlesSchema,
    client_authored_schema::ClientAuthoredSchema,
    state_machines::{
        state_machine_node_schema::StateMachineNodeSchema,
        state_machine_schema::StateMachineSchema,
        state_machine_transition_schema::{
            StateMachineTransitionSchema, StateMachineTransitionTriggerSchema,
        },
    },
    worlds::{world_object_schema::WorldObjectSchema, world_schema::WorldSchema},
};
use crate::experience_schema::experience_schema::{
    CURRENT_EXPERIENCE_SCHEMA_VERSION, ExperienceSchema, ExperienceSchemaVersion,
};
use crate::experience_schema::service_authored::service_authored_schema::ServiceAuthoredSchema;

#[derive(Clone, PartialEq, Message)]
pub struct ExperienceSchemaProto {
    #[prost(enumeration = "ExperienceSchemaVersion", tag = "1")]
    pub schema_version: i32,
    #[prost(message, optional, tag = "2")]
    pub service_authored_schema: Option<ServiceAuthoredSchemaProto>,
    #[prost(message, optional, tag = "3")]
    pub client_authored_schema: Option<ClientAuthoredSchemaProto>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ServiceAuthoredSchemaProto {
    #[prost(bytes = "vec", tag = "1")]
    pub json_payload: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ClientAuthoredSchemaProto {
    #[prost(message, optional, tag = "1")]
    pub asset_bundles: Option<AssetBundlesSchemaProto>,
    #[prost(map = "string, message", tag = "2")]
    pub worlds: HashMap<String, WorldSchemaProto>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AssetBundlesSchemaProto {
    #[prost(bytes = "vec", tag = "1")]
    pub json_payload: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
pub struct WorldSchemaProto {
    #[prost(message, repeated, tag = "1")]
    pub objects: Vec<WorldObjectSchemaProto>,
    #[prost(bytes = "vec", tag = "2")]
    pub properties_json: Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub state_machines: Vec<StateMachineSchemaProto>,
    #[prost(string, repeated, tag = "4")]
    pub asset_bundle_ids: Vec<String>,
    #[prost(map = "string, message", tag = "5")]
    pub object_templates: HashMap<String, WorldObjectSchemaProto>,
}

#[derive(Clone, PartialEq, Message)]
pub struct WorldObjectSchemaProto {
    #[prost(bytes = "vec", tag = "1")]
    pub properties_json: Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub state_machines: Vec<StateMachineSchemaProto>,
}

#[derive(Clone, PartialEq, Message)]
pub struct StateMachineSchemaProto {
    #[prost(string, tag = "1")]
    pub initial_state_name: String,
    #[prost(uint64, tag = "2")]
    pub deterministic_seed: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub property_maps_json: Vec<u8>,
    #[prost(message, repeated, tag = "4")]
    pub nodes: Vec<StateMachineNodeSchemaProto>,
}

#[derive(Clone, PartialEq, Message)]
pub struct StateMachineNodeSchemaProto {
    #[prost(string, tag = "1")]
    pub state_name: String,
    #[prost(bytes = "vec", tag = "2")]
    pub node_type_json: Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub transitions: Vec<StateMachineTransitionSchemaProto>,
}

#[derive(Clone, PartialEq, Message)]
pub struct StateMachineTransitionSchemaProto {
    #[prost(string, tag = "1")]
    pub from_state_name: String,
    #[prost(string, tag = "2")]
    pub to_state_name: String,
    #[prost(bytes = "vec", tag = "3")]
    pub trigger_json: Vec<u8>,
}

impl ExperienceSchema {
    pub fn encode_protobuf(&self) -> anyhow::Result<Vec<u8>> {
        let proto_schema = ExperienceSchemaProto::try_from(self)?;
        Ok(proto_schema.encode_to_vec())
    }

    pub fn decode_protobuf(schema_bytes: &[u8]) -> anyhow::Result<Self> {
        let proto_schema = ExperienceSchemaProto::decode(schema_bytes)?;
        proto_schema.try_into()
    }
}

impl TryFrom<&ExperienceSchema> for ExperienceSchemaProto {
    type Error = anyhow::Error;

    fn try_from(value: &ExperienceSchema) -> Result<Self, Self::Error> {
        let normalized_version = if value.schema_version == 0 {
            CURRENT_EXPERIENCE_SCHEMA_VERSION
        } else {
            value.schema_version
        };

        Ok(Self {
            schema_version: normalized_version as i32,
            service_authored_schema: Some(ServiceAuthoredSchemaProto::try_from(
                &value.service_authored_schema,
            )?),
            client_authored_schema: Some(ClientAuthoredSchemaProto::try_from(
                &value.client_authored_schema,
            )?),
        })
    }
}

impl TryFrom<ExperienceSchemaProto> for ExperienceSchema {
    type Error = anyhow::Error;

    fn try_from(value: ExperienceSchemaProto) -> Result<Self, Self::Error> {
        if value.schema_version != 0 {
            ExperienceSchemaVersion::try_from(value.schema_version)
                .context("unsupported schema version in protobuf payload")?;
        }

        Ok(Self {
            schema_version: u32::try_from(value.schema_version)
                .unwrap_or(CURRENT_EXPERIENCE_SCHEMA_VERSION),
            service_authored_schema: value
                .service_authored_schema
                .map(TryFrom::try_from)
                .transpose()?
                .unwrap_or_default(),
            client_authored_schema: value
                .client_authored_schema
                .map(TryFrom::try_from)
                .transpose()?
                .unwrap_or_default(),
        })
    }
}

impl TryFrom<&ServiceAuthoredSchema> for ServiceAuthoredSchemaProto {
    type Error = anyhow::Error;

    fn try_from(value: &ServiceAuthoredSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            json_payload: serde_json::to_vec(value)?,
        })
    }
}

impl TryFrom<ServiceAuthoredSchemaProto> for ServiceAuthoredSchema {
    type Error = anyhow::Error;

    fn try_from(value: ServiceAuthoredSchemaProto) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice::<ServiceAuthoredSchema>(
            &value.json_payload,
        )?)
    }
}

impl TryFrom<&ClientAuthoredSchema> for ClientAuthoredSchemaProto {
    type Error = anyhow::Error;

    fn try_from(value: &ClientAuthoredSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            asset_bundles: Some(AssetBundlesSchemaProto::try_from(&value.asset_bundles)?),
            worlds: value
                .worlds
                .iter()
                .map(|(world_id, world_schema)| {
                    Ok((world_id.clone(), WorldSchemaProto::try_from(world_schema)?))
                })
                .collect::<Result<HashMap<_, _>, anyhow::Error>>()?,
        })
    }
}

impl TryFrom<ClientAuthoredSchemaProto> for ClientAuthoredSchema {
    type Error = anyhow::Error;

    fn try_from(value: ClientAuthoredSchemaProto) -> Result<Self, Self::Error> {
        Ok(Self {
            asset_bundles: value
                .asset_bundles
                .map(TryFrom::try_from)
                .transpose()?
                .unwrap_or_default(),
            worlds: value
                .worlds
                .into_iter()
                .map(|(world_id, world_schema)| Ok((world_id, world_schema.try_into()?)))
                .collect::<Result<HashMap<_, _>, anyhow::Error>>()?,
        })
    }
}

impl TryFrom<&AssetBundlesSchema> for AssetBundlesSchemaProto {
    type Error = anyhow::Error;

    fn try_from(value: &AssetBundlesSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            json_payload: serde_json::to_vec(value)?,
        })
    }
}

impl TryFrom<AssetBundlesSchemaProto> for AssetBundlesSchema {
    type Error = anyhow::Error;

    fn try_from(value: AssetBundlesSchemaProto) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice::<AssetBundlesSchema>(
            &value.json_payload,
        )?)
    }
}

impl TryFrom<&WorldSchema> for WorldSchemaProto {
    type Error = anyhow::Error;

    fn try_from(value: &WorldSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            objects: value
                .objects
                .iter()
                .map(WorldObjectSchemaProto::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            properties_json: serde_json::to_vec(&value.properties)?,
            state_machines: value
                .state_machines
                .iter()
                .map(StateMachineSchemaProto::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            asset_bundle_ids: value.asset_bundle_ids.clone(),
            object_templates: value
                .object_templates
                .iter()
                .map(|(template_id, template_schema)| {
                    Ok((
                        template_id.clone(),
                        WorldObjectSchemaProto::try_from(template_schema)?,
                    ))
                })
                .collect::<Result<HashMap<_, _>, anyhow::Error>>()?,
        })
    }
}

impl TryFrom<WorldSchemaProto> for WorldSchema {
    type Error = anyhow::Error;

    fn try_from(value: WorldSchemaProto) -> Result<Self, Self::Error> {
        Ok(Self {
            objects: value
                .objects
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            properties: serde_json::from_slice(&value.properties_json)?,
            state_machines: value
                .state_machines
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            asset_bundle_ids: value.asset_bundle_ids,
            object_templates: value
                .object_templates
                .into_iter()
                .map(|(template_id, template_schema)| {
                    Ok((template_id, template_schema.try_into()?))
                })
                .collect::<Result<HashMap<_, _>, anyhow::Error>>()?,
        })
    }
}

impl TryFrom<&WorldObjectSchema> for WorldObjectSchemaProto {
    type Error = anyhow::Error;

    fn try_from(value: &WorldObjectSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            properties_json: serde_json::to_vec(&value.properties)?,
            state_machines: value
                .state_machines
                .iter()
                .map(StateMachineSchemaProto::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<WorldObjectSchemaProto> for WorldObjectSchema {
    type Error = anyhow::Error;

    fn try_from(value: WorldObjectSchemaProto) -> Result<Self, Self::Error> {
        Ok(Self {
            properties: serde_json::from_slice(&value.properties_json)?,
            state_machines: value
                .state_machines
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<&StateMachineSchema> for StateMachineSchemaProto {
    type Error = anyhow::Error;

    fn try_from(value: &StateMachineSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            initial_state_name: value.initial_state_name.clone(),
            deterministic_seed: value.deterministic_seed,
            property_maps_json: serde_json::to_vec(&value.property_maps)?,
            nodes: value
                .nodes
                .iter()
                .map(StateMachineNodeSchemaProto::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<StateMachineSchemaProto> for StateMachineSchema {
    type Error = anyhow::Error;

    fn try_from(value: StateMachineSchemaProto) -> Result<Self, Self::Error> {
        Ok(Self {
            initial_state_name: value.initial_state_name,
            deterministic_seed: value.deterministic_seed,
            property_maps: serde_json::from_slice(&value.property_maps_json)?,
            nodes: value
                .nodes
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<&StateMachineNodeSchema> for StateMachineNodeSchemaProto {
    type Error = anyhow::Error;

    fn try_from(value: &StateMachineNodeSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            state_name: value.state_name.clone(),
            node_type_json: serde_json::to_vec(&value.node_type)?,
            transitions: value
                .transitions
                .iter()
                .map(StateMachineTransitionSchemaProto::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<StateMachineNodeSchemaProto> for StateMachineNodeSchema {
    type Error = anyhow::Error;

    fn try_from(value: StateMachineNodeSchemaProto) -> Result<Self, Self::Error> {
        Ok(Self {
            state_name: value.state_name,
            node_type: serde_json::from_slice(&value.node_type_json)?,
            transitions: value
                .transitions
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<&StateMachineTransitionSchema> for StateMachineTransitionSchemaProto {
    type Error = anyhow::Error;

    fn try_from(value: &StateMachineTransitionSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            from_state_name: value.from_state_name.clone(),
            to_state_name: value.to_state_name.clone(),
            trigger_json: serde_json::to_vec(&value.trigger)?,
        })
    }
}

impl TryFrom<StateMachineTransitionSchemaProto> for StateMachineTransitionSchema {
    type Error = anyhow::Error;

    fn try_from(value: StateMachineTransitionSchemaProto) -> Result<Self, Self::Error> {
        Ok(Self {
            from_state_name: value.from_state_name,
            to_state_name: value.to_state_name,
            trigger: serde_json::from_slice::<StateMachineTransitionTriggerSchema>(
                &value.trigger_json,
            )?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ExperienceSchema;
    use crate::experience_schema::client_authored::state_machines::api::StateMachineApiSchema;
    use crate::experience_schema::client_authored::state_machines::state_machine_node_schema::{
        StateMachineNodeSchema, StateMachineNodeTypeSchema,
    };
    use crate::experience_schema::client_authored::state_machines::state_machine_schema::StateMachineSchema;

    #[test]
    fn protobuf_round_trip_preserves_custom_api_identifiers() {
        let mut schema = ExperienceSchema::default();
        schema
            .client_authored_schema
            .worlds
            .insert("".to_string(), Default::default());
        schema
            .client_authored_schema
            .worlds
            .get_mut("")
            .expect("world should exist")
            .state_machines
            .push(StateMachineSchema {
                initial_state_name: "idle".to_string(),
                deterministic_seed: 7,
                property_maps: Vec::new(),
                nodes: vec![StateMachineNodeSchema::new(
                    "idle",
                    StateMachineNodeTypeSchema::ApiDispatch {
                        api: StateMachineApiSchema::from(
                            "point_and_click:dispatch_progression_complete",
                        ),
                        args_property_map_id: Some("args".to_string()),
                    },
                )],
            });

        let bytes = schema.encode_protobuf().expect("encode");
        let decoded = ExperienceSchema::decode_protobuf(&bytes).expect("decode");

        assert_eq!(
            decoded.client_authored_schema.worlds[""].state_machines[0].nodes[0].node_type,
            StateMachineNodeTypeSchema::ApiDispatch {
                api: StateMachineApiSchema::Custom(
                    "point_and_click:dispatch_progression_complete".to_string()
                ),
                args_property_map_id: Some("args".to_string()),
            }
        );
    }
}
