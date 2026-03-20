use crate::{
    client_authored::client_authored_schema::ClientAuthoredSchema,
    service_authored::service_authored_schema::ServiceAuthoredSchema,
};
use prost::{Enumeration, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enumeration)]
#[repr(i32)]
pub enum ExperienceSchemaVersion {
    V1 = 1,
}

pub const CURRENT_EXPERIENCE_SCHEMA_VERSION: u32 = ExperienceSchemaVersion::V1 as i32 as u32;

fn default_schema_version() -> u32 {
    CURRENT_EXPERIENCE_SCHEMA_VERSION
}

/// Describes a fully serialized experience consumed by runtimes/clients.
///
/// This is the universal target format for all published experiences.
/// Experiences may have their own internal schemas and formats that transpile to this format.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct ExperienceSchema {
    /// The version of this schema. Older versions will need migration steps.
    #[serde(default = "default_schema_version")]
    #[prost(uint32, tag = "1")]
    pub schema_version: u32,

    /// Schema populated by backend trusted services.
    #[prost(message, required, tag = "2")]
    pub service_authored_schema: ServiceAuthoredSchema,

    /// Schema populated by clients. May still need verification on the backend side if submitted for publishing.
    #[prost(message, required, tag = "3")]
    pub client_authored_schema: ClientAuthoredSchema,
}

impl ExperienceSchema {
    pub fn encode_prost(&self) -> anyhow::Result<Vec<u8>> {
        Ok(self.encode_to_vec())
    }

    pub fn decode_prost(schema_bytes: &[u8]) -> anyhow::Result<Self> {
        Ok(Self::decode(schema_bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use super::ExperienceSchema;
    use crate::experience_schema::client_authored::state_machines::api::StateMachineApiSchema;
    use crate::experience_schema::client_authored::state_machines::state_machine_node_schema::{
        StateMachineNodeSchema, StateMachineNodeTypeSchema,
    };
    use crate::experience_schema::client_authored::state_machines::state_machine_proof_class_schema::StateMachineProofClassSchema;
    use crate::experience_schema::client_authored::state_machines::state_machine_schema::StateMachineSchema;

    #[test]
    fn prost_round_trip_preserves_custom_api_identifiers() {
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
                proof_class: StateMachineProofClassSchema::BoundedExtended,
                initial_state_name: "idle".to_string(),
                deterministic_seed: 7,
                property_maps: Vec::new(),
                finite_domain_abstractions: Vec::new(),
                nodes: vec![StateMachineNodeSchema::new(
                    "idle",
                    StateMachineNodeTypeSchema::ApiDispatch {
                        api: StateMachineApiSchema::from(
                            "puppet_master:dispatch_progression_complete",
                        ),
                        args_property_map_id: Some("args".to_string()),
                    },
                )],
            });

        let bytes = schema.encode_prost().expect("encode");
        let decoded = ExperienceSchema::decode_prost(&bytes).expect("decode");

        assert_eq!(
            decoded.client_authored_schema.worlds[""].state_machines[0].proof_class,
            StateMachineProofClassSchema::BoundedExtended
        );
        assert_eq!(
            decoded.client_authored_schema.worlds[""].state_machines[0].nodes[0].node_type,
            StateMachineNodeTypeSchema::ApiDispatch {
                api: StateMachineApiSchema::Custom(
                    "puppet_master:dispatch_progression_complete".to_string()
                ),
                args_property_map_id: Some("args".to_string()),
            }
        );
    }
}
