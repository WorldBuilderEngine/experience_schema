use crate::client_authored::state_machines::api::StateMachineApiSchema;
use crate::client_authored::state_machines::state_machine_node_schema::{
    StateMachineNodeSchema, StateMachineNodeTypeSchema,
};
use crate::client_authored::state_machines::state_machine_proof_class_schema::StateMachineProofClassSchema;
use crate::client_authored::state_machines::state_machine_transition_schema::StateMachineTransitionSchema;
use crate::properties::property_map::PropertyMap;
use crate::prost_json_message::{
    encode_as_json_message, json_message_encoded_len, merge_from_json_message,
};
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::{Deserialize, Serialize};

/// Serializable state-machine definition used in authored world schemas.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StateMachineSchema {
    pub proof_class: StateMachineProofClassSchema,
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
        Self::new_with_proof_class(
            initial_state_name,
            StateMachineProofClassSchema::EffectfulOpen,
        )
    }

    pub fn new_with_seed(initial_state_name: impl Into<String>, deterministic_seed: u64) -> Self {
        Self::new_with_seed_and_proof_class(
            initial_state_name,
            deterministic_seed,
            StateMachineProofClassSchema::EffectfulOpen,
        )
    }

    pub fn new_with_proof_class(
        initial_state_name: impl Into<String>,
        proof_class: StateMachineProofClassSchema,
    ) -> Self {
        Self::new_with_seed_and_proof_class(initial_state_name, 0, proof_class)
    }

    pub fn new_with_seed_and_proof_class(
        initial_state_name: impl Into<String>,
        deterministic_seed: u64,
        proof_class: StateMachineProofClassSchema,
    ) -> Self {
        Self {
            proof_class,
            initial_state_name: initial_state_name.into(),
            deterministic_seed,
            property_maps: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn set_proof_class(&mut self, proof_class: StateMachineProofClassSchema) {
        self.proof_class = proof_class;
    }

    pub fn add_transition(
        &mut self,
        api: impl Into<StateMachineApiSchema>,
        transition: StateMachineTransitionSchema,
    ) {
        self.nodes
            .push(StateMachineNodeSchema::new_with_transitions(
                transition.from_state_name.clone(),
                StateMachineNodeTypeSchema::ApiDispatch {
                    api: api.into(),
                    args_property_map_id: None,
                },
                vec![transition],
            ));
    }

    pub fn register_api_dispatch_node(
        &mut self,
        state_name: impl Into<String>,
        api: impl Into<StateMachineApiSchema>,
        args_property_map_id: Option<String>,
    ) {
        self.nodes.push(StateMachineNodeSchema::new(
            state_name,
            StateMachineNodeTypeSchema::ApiDispatch {
                api: api.into(),
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

impl Message for StateMachineSchema {
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
    use super::StateMachineSchema;
    use crate::client_authored::state_machines::state_machine_proof_class_schema::StateMachineProofClassSchema;

    #[test]
    fn constructor_populates_core_fields() {
        let schema = StateMachineSchema::new("idle");
        assert_eq!(
            schema.proof_class,
            StateMachineProofClassSchema::EffectfulOpen
        );
        assert_eq!(schema.initial_state_name, "idle".to_string());
        assert_eq!(schema.deterministic_seed, 0);
    }

    #[test]
    fn constructor_supports_explicit_proof_class() {
        let schema = StateMachineSchema::new_with_seed_and_proof_class(
            "idle",
            7,
            StateMachineProofClassSchema::Finite,
        );

        assert_eq!(schema.proof_class, StateMachineProofClassSchema::Finite);
        assert_eq!(schema.initial_state_name, "idle");
        assert_eq!(schema.deterministic_seed, 7);
    }

    #[test]
    fn deserialization_requires_proof_class_metadata() {
        let parse_error = serde_json::from_str::<StateMachineSchema>(
            r#"{
                "initial_state_name":"idle",
                "nodes":[]
            }"#,
        )
        .expect_err("missing proof_class should fail to deserialize");

        assert!(parse_error.to_string().contains("proof_class"));
    }

    #[test]
    fn deserialization_rejects_unknown_proof_class_values() {
        let parse_error = serde_json::from_str::<StateMachineSchema>(
            r#"{
                "proof_class":"not_real",
                "initial_state_name":"idle",
                "nodes":[]
            }"#,
        )
        .expect_err("unknown proof_class should fail to deserialize");

        assert!(parse_error.to_string().contains("unknown variant"));
    }
}
