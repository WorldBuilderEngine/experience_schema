use crate::client_authored::state_machines::api::StateMachineApiSchema;
use crate::client_authored::state_machines::state_machine_bounded_effect_contract_schema::StateMachineBoundedEffectContractSchema;
use crate::client_authored::state_machines::state_machine_finite_domain_abstraction_schema::StateMachineFiniteDomainAbstractionSchema;
use crate::client_authored::state_machines::state_machine_node_schema::{
    StateMachineNodeSchema, StateMachineNodeTypeSchema,
};
use crate::client_authored::state_machines::state_machine_proof_metadata_schema::StateMachineProofMetadataSchema;
use crate::client_authored::state_machines::state_machine_proof_assertion_schema::StateMachineProofAssertionSchema;
use crate::client_authored::state_machines::state_machine_proof_class_schema::StateMachineProofClassSchema;
use crate::client_authored::state_machines::state_machine_synchronous_invocation_contract_schema::StateMachineSynchronousInvocationContractSchema;
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
    pub initial_state_name: String,
    #[serde(default)]
    pub deterministic_seed: u64,
    #[serde(default)]
    pub property_maps: Vec<(String, PropertyMap)>,
    #[serde(default)]
    pub bounded_effect_contract: StateMachineBoundedEffectContractSchema,
    #[serde(default)]
    pub synchronous_invocation_contract: StateMachineSynchronousInvocationContractSchema,
    #[serde(default)]
    pub nodes: Vec<StateMachineNodeSchema>,
    #[serde(default, flatten, skip_serializing)]
    proof_metadata: StateMachineProofMetadataSchema,
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
        let mut proof_metadata = StateMachineProofMetadataSchema::default();
        proof_metadata.proof_class = proof_class;
        Self {
            initial_state_name: initial_state_name.into(),
            deterministic_seed,
            property_maps: Vec::new(),
            bounded_effect_contract: StateMachineBoundedEffectContractSchema::default(),
            synchronous_invocation_contract:
                StateMachineSynchronousInvocationContractSchema::default(),
            nodes: Vec::new(),
            proof_metadata,
        }
    }

    pub fn set_proof_class(&mut self, proof_class: StateMachineProofClassSchema) {
        self.proof_metadata.proof_class = proof_class;
    }

    pub fn declared_proof_class(&self) -> StateMachineProofClassSchema {
        self.proof_metadata.proof_class
    }

    pub fn finite_domain_abstractions(&self) -> &[StateMachineFiniteDomainAbstractionSchema] {
        self.proof_metadata.finite_domain_abstractions.as_slice()
    }

    pub fn proof_assertions(&self) -> &[StateMachineProofAssertionSchema] {
        self.proof_metadata.proof_assertions.as_slice()
    }

    pub fn proof_metadata(&self) -> &StateMachineProofMetadataSchema {
        &self.proof_metadata
    }

    pub fn set_proof_metadata(&mut self, proof_metadata: StateMachineProofMetadataSchema) {
        self.proof_metadata = proof_metadata;
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

    pub fn register_finite_domain_abstraction(
        &mut self,
        abstraction: StateMachineFiniteDomainAbstractionSchema,
    ) {
        self.proof_metadata.finite_domain_abstractions.push(abstraction);
    }

    pub fn register_proof_assertion(&mut self, assertion: StateMachineProofAssertionSchema) {
        self.proof_metadata.proof_assertions.push(assertion);
    }

    pub fn set_bounded_effect_contract(
        &mut self,
        bounded_effect_contract: StateMachineBoundedEffectContractSchema,
    ) {
        self.bounded_effect_contract = bounded_effect_contract;
    }

    pub fn set_synchronous_invocation_contract(
        &mut self,
        synchronous_invocation_contract: StateMachineSynchronousInvocationContractSchema,
    ) {
        self.synchronous_invocation_contract = synchronous_invocation_contract;
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
    use crate::client_authored::state_machines::state_machine_bounded_effect_contract_schema::{
        StateMachineBoundedEffectContractSchema, StateMachinePersistenceKeyRegistrySchema,
        StateMachineResourceCreationContractSchema,
    };
    use crate::client_authored::state_machines::state_machine_finite_domain_abstraction_schema::{
        StateMachineFiniteDomainAbstractionSchema, StateMachineFiniteDomainSchema,
        StateMachineFiniteDomainSemanticsSchema, StateMachineFiniteDomainTargetSchema,
    };
    use crate::client_authored::state_machines::state_machine_proof_assertion_schema::{
        StateMachineProofAssertionKindSchema, StateMachineProofAssertionSchema,
    };
    use crate::client_authored::state_machines::state_machine_proof_class_schema::StateMachineProofClassSchema;
    use crate::client_authored::state_machines::state_machine_synchronous_invocation_contract_schema::{
        StateMachineSchedulerCapabilitySchema, StateMachineSynchronousInvocationContractSchema,
    };

    #[test]
    fn constructor_populates_core_fields() {
        let schema = StateMachineSchema::new("idle");
        assert_eq!(
            schema.declared_proof_class(),
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

        assert_eq!(schema.declared_proof_class(), StateMachineProofClassSchema::Finite);
        assert_eq!(schema.initial_state_name, "idle");
        assert_eq!(schema.deterministic_seed, 7);
        assert_eq!(
            schema.bounded_effect_contract,
            StateMachineBoundedEffectContractSchema::default()
        );
        assert!(schema.finite_domain_abstractions().is_empty());
        assert!(schema.proof_assertions().is_empty());
        assert_eq!(
            schema.synchronous_invocation_contract,
            StateMachineSynchronousInvocationContractSchema::default()
        );
    }

    #[test]
    fn deserialization_defaults_missing_inline_proof_metadata() {
        let schema = serde_json::from_str::<StateMachineSchema>(
            r#"{
                "initial_state_name":"idle",
                "nodes":[]
            }"#,
        )
        .expect("missing inline proof metadata should deserialize");

        assert_eq!(
            schema.declared_proof_class(),
            StateMachineProofClassSchema::EffectfulOpen
        );
        assert!(schema.finite_domain_abstractions().is_empty());
        assert!(schema.proof_assertions().is_empty());
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

    #[test]
    fn deserialization_preserves_legacy_inline_proof_metadata() {
        let schema = serde_json::from_str::<StateMachineSchema>(
            r#"{
                "proof_class":"effectful_open",
                "initial_state_name":"idle",
                "finite_domain_abstractions": [],
                "proof_assertions": [],
                "nodes":[]
            }"#,
        )
        .expect("schema should deserialize");

        assert_eq!(
            schema.declared_proof_class(),
            StateMachineProofClassSchema::EffectfulOpen
        );
        assert_eq!(
            schema.bounded_effect_contract,
            StateMachineBoundedEffectContractSchema::default()
        );
        assert!(schema.finite_domain_abstractions().is_empty());
        assert!(schema.proof_assertions().is_empty());
        assert_eq!(
            schema.synchronous_invocation_contract,
            StateMachineSynchronousInvocationContractSchema::default()
        );
    }

    #[test]
    fn set_bounded_effect_contract_replaces_contract_metadata() {
        let mut schema = StateMachineSchema::new("idle");
        schema.set_bounded_effect_contract(StateMachineBoundedEffectContractSchema {
            resource_creation: Some(StateMachineResourceCreationContractSchema {
                total_creations_upper_bound: 4,
            }),
            persistence_key_registry: Some(StateMachinePersistenceKeyRegistrySchema {
                members: vec!["profile/player-1".to_string()],
            }),
        });

        assert_eq!(
            schema.bounded_effect_contract.resource_creation,
            Some(StateMachineResourceCreationContractSchema {
                total_creations_upper_bound: 4,
            })
        );
        assert_eq!(
            schema.bounded_effect_contract.persistence_key_registry,
            Some(StateMachinePersistenceKeyRegistrySchema {
                members: vec!["profile/player-1".to_string()],
            })
        );
    }

    #[test]
    fn register_finite_domain_abstraction_appends_declaration() {
        let mut schema = StateMachineSchema::new("idle");
        schema.register_finite_domain_abstraction(StateMachineFiniteDomainAbstractionSchema {
            target: StateMachineFiniteDomainTargetSchema::PropertyField {
                property_map_id: "runtime".to_string(),
                property_id: "phase".to_string(),
            },
            domain: StateMachineFiniteDomainSchema::Enum {
                values: vec!["idle".to_string(), "run".to_string()],
            },
            semantics: StateMachineFiniteDomainSemanticsSchema::Exact,
        });

        assert_eq!(schema.finite_domain_abstractions().len(), 1);
    }

    #[test]
    fn register_proof_assertion_appends_declaration() {
        let mut schema = StateMachineSchema::new("idle");
        schema.register_proof_assertion(StateMachineProofAssertionSchema {
            label: Some("idle_is_reachable".to_string()),
            kind: StateMachineProofAssertionKindSchema::ReachableState {
                state_name: "idle".to_string(),
            },
        });

        assert_eq!(schema.proof_assertions().len(), 1);
    }

    #[test]
    fn set_synchronous_invocation_contract_replaces_contract_metadata() {
        let mut schema = StateMachineSchema::new("idle");
        schema.set_synchronous_invocation_contract(
            StateMachineSynchronousInvocationContractSchema {
                machine_label: Some("combat:resolver".to_string()),
                scheduler_capability: StateMachineSchedulerCapabilitySchema::SyncCall,
                maximum_call_depth: Some(3),
                call_fuel_budget: Some(5),
                mutable_resources: vec!["world:turn_state".to_string()],
                receive_entrypoints: Vec::new(),
                outgoing_calls: Vec::new(),
            },
        );

        assert_eq!(
            schema.synchronous_invocation_contract,
            StateMachineSynchronousInvocationContractSchema {
                machine_label: Some("combat:resolver".to_string()),
                scheduler_capability: StateMachineSchedulerCapabilitySchema::SyncCall,
                maximum_call_depth: Some(3),
                call_fuel_budget: Some(5),
                mutable_resources: vec!["world:turn_state".to_string()],
                receive_entrypoints: Vec::new(),
                outgoing_calls: Vec::new(),
            }
        );
    }

    #[test]
    fn serialization_omits_proof_only_metadata_from_runtime_schema() {
        let mut schema = StateMachineSchema::new_with_proof_class(
            "idle",
            StateMachineProofClassSchema::Finite,
        );
        schema.register_finite_domain_abstraction(StateMachineFiniteDomainAbstractionSchema {
            target: StateMachineFiniteDomainTargetSchema::PropertyField {
                property_map_id: "runtime".to_string(),
                property_id: "phase".to_string(),
            },
            domain: StateMachineFiniteDomainSchema::Enum {
                values: vec!["idle".to_string(), "done".to_string()],
            },
            semantics: StateMachineFiniteDomainSemanticsSchema::Exact,
        });
        schema.register_proof_assertion(StateMachineProofAssertionSchema {
            label: Some("idle_is_reachable".to_string()),
            kind: StateMachineProofAssertionKindSchema::ReachableState {
                state_name: "idle".to_string(),
            },
        });

        let serialized = serde_json::to_value(&schema).expect("schema should serialize");
        assert!(serialized.get("proof_class").is_none());
        assert!(serialized.get("finite_domain_abstractions").is_none());
        assert!(serialized.get("proof_assertions").is_none());
    }
}
