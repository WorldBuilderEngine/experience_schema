use crate::client_authored::state_machines::api::StateMachineApiSchema;
use crate::client_authored::state_machines::state_machine_compatibility_schema::StateMachineCompatibilitySchema;
use crate::client_authored::state_machines::state_machine_local_field_schema::StateMachineLocalFieldSchema;
use crate::client_authored::state_machines::state_machine_local_schema::StateMachineLocalSchema;
use crate::client_authored::state_machines::state_machine_node_schema::{
    StateMachineNodeSchema, StateMachineNodeTypeSchema,
};
use crate::client_authored::state_machines::state_machine_owned_collection_capacity_schema::StateMachineOwnedCollectionCapacitySchema;
use crate::client_authored::state_machines::state_machine_proof_class_schema::StateMachineProofClassSchema;
use crate::client_authored::state_machines::state_machine_transition_schema::StateMachineTransitionSchema;
use crate::properties::property_map::PropertyMap;
use crate::wire_compat::json_message::{
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
    pub machine_locals: Vec<StateMachineLocalSchema>,
    #[serde(default)]
    pub machine_owned_collection_capacities: Vec<StateMachineOwnedCollectionCapacitySchema>,
    #[serde(default)]
    pub nodes: Vec<StateMachineNodeSchema>,
    #[serde(default, flatten, skip_serializing)]
    compatibility: StateMachineCompatibilitySchema,
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
            initial_state_name: initial_state_name.into(),
            deterministic_seed,
            machine_locals: Vec::new(),
            machine_owned_collection_capacities: Vec::new(),
            nodes: Vec::new(),
            compatibility: StateMachineCompatibilitySchema::with_proof_class(proof_class),
        }
    }

    pub fn compatibility(&self) -> &StateMachineCompatibilitySchema {
        &self.compatibility
    }

    pub fn compatibility_mut(&mut self) -> &mut StateMachineCompatibilitySchema {
        &mut self.compatibility
    }

    pub fn machine_locals(&self) -> &[StateMachineLocalSchema] {
        self.machine_locals.as_slice()
    }

    pub fn register_machine_local(&mut self, local_id: impl Into<String>, properties: PropertyMap) {
        let local_id_string = local_id.into().trim().to_string();
        if let Some(existing_local_index) = self
            .machine_locals
            .iter()
            .position(|existing_local| existing_local.local_id == local_id_string)
        {
            self.machine_locals[existing_local_index].fields =
                StateMachineLocalSchema::from_property_map("", properties).fields;
            return;
        }

        self.machine_locals
            .push(StateMachineLocalSchema::from_property_map(
                local_id_string,
                properties,
            ));
    }

    pub fn register_machine_local_fields(
        &mut self,
        local_id: impl Into<String>,
        fields: Vec<StateMachineLocalFieldSchema>,
    ) {
        let local_id_string = local_id.into().trim().to_string();
        if let Some(existing_local_index) = self
            .machine_locals
            .iter()
            .position(|existing_local| existing_local.local_id == local_id_string)
        {
            self.machine_locals[existing_local_index].fields = fields;
            return;
        }

        self.machine_locals
            .push(StateMachineLocalSchema::new(local_id_string, fields));
    }

    pub fn machine_local(&self, local_id: &str) -> Option<&StateMachineLocalSchema> {
        let normalized_local_id = local_id.trim();
        self.machine_locals
            .iter()
            .find(|local| local.local_id == normalized_local_id)
    }

    pub fn machine_local_property(
        &self,
        local_id: &str,
        field_id: &str,
    ) -> Option<&crate::properties::property::Property> {
        self.machine_local(local_id)
            .and_then(|machine_local| machine_local.field_value(field_id))
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
                    args_local_id: None,
                },
                vec![transition],
            ));
    }

    pub fn register_api_dispatch_node(
        &mut self,
        state_name: impl Into<String>,
        api: impl Into<StateMachineApiSchema>,
        args_local_id: Option<String>,
    ) {
        self.nodes.push(StateMachineNodeSchema::new(
            state_name,
            StateMachineNodeTypeSchema::ApiDispatch {
                api: api.into(),
                args_local_id,
            },
        ));
    }

    pub fn register_machine_owned_collection_capacity(
        &mut self,
        local_id: impl Into<String>,
        property_id: impl Into<String>,
        capacity: u32,
    ) {
        let declaration =
            StateMachineOwnedCollectionCapacitySchema::new(local_id, property_id, capacity);
        if let Some(existing_declaration_index) = self
            .machine_owned_collection_capacities
            .iter()
            .position(|existing_declaration| {
                existing_declaration.local_id == declaration.local_id
                    && existing_declaration.property_id == declaration.property_id
            })
        {
            self.machine_owned_collection_capacities[existing_declaration_index] = declaration;
            return;
        }

        self.machine_owned_collection_capacities.push(declaration);
    }

    pub fn machine_owned_collection_capacity(
        &self,
        local_id: &str,
        property_id: &str,
    ) -> Option<u32> {
        let normalized_local_id = local_id.trim();
        let normalized_property_id = property_id.trim();
        self.machine_owned_collection_capacities
            .iter()
            .find(|declaration| {
                declaration.local_id == normalized_local_id
                    && declaration.property_id == normalized_property_id
            })
            .map(|declaration| declaration.capacity)
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
    use crate::client_authored::state_machines::state_machine_finite_domain_abstraction_schema::{
        StateMachineFiniteDomainAbstractionSchema, StateMachineFiniteDomainSchema,
        StateMachineFiniteDomainSemanticsSchema, StateMachineFiniteDomainTargetSchema,
    };
    use crate::client_authored::state_machines::state_machine_owned_collection_capacity_schema::StateMachineOwnedCollectionCapacitySchema;
    use crate::client_authored::state_machines::state_machine_proof_assertion_schema::{
        StateMachineProofAssertionKindSchema, StateMachineProofAssertionSchema,
    };
    use crate::client_authored::state_machines::state_machine_proof_class_schema::StateMachineProofClassSchema;
    use crate::client_authored::state_machines::state_machine_proof_metadata_schema::StateMachineProofMetadataSchema;

    #[test]
    fn constructor_populates_core_fields() {
        let schema = StateMachineSchema::new("idle");
        assert_eq!(
            schema.compatibility().declared_proof_class(),
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

        assert_eq!(
            schema.compatibility().declared_proof_class(),
            StateMachineProofClassSchema::Finite
        );
        assert_eq!(schema.initial_state_name, "idle");
        assert_eq!(schema.deterministic_seed, 7);
        assert!(
            schema
                .compatibility()
                .finite_domain_abstractions()
                .is_empty()
        );
        assert!(schema.compatibility().proof_assertions().is_empty());
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
            schema.compatibility().declared_proof_class(),
            StateMachineProofClassSchema::EffectfulOpen
        );
        assert!(schema.machine_owned_collection_capacities.is_empty());
        assert!(
            schema
                .compatibility()
                .finite_domain_abstractions()
                .is_empty()
        );
        assert!(schema.compatibility().proof_assertions().is_empty());
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
    fn deserialization_preserves_inline_proof_metadata() {
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
            schema.compatibility().declared_proof_class(),
            StateMachineProofClassSchema::EffectfulOpen
        );
        assert!(
            schema
                .compatibility()
                .finite_domain_abstractions()
                .is_empty()
        );
        assert!(schema.compatibility().proof_assertions().is_empty());
    }

    #[test]
    fn register_finite_domain_abstraction_appends_declaration() {
        let mut schema = StateMachineSchema::new("idle");
        schema
            .compatibility_mut()
            .register_finite_domain_abstraction(StateMachineFiniteDomainAbstractionSchema {
                target: StateMachineFiniteDomainTargetSchema::PropertyField {
                    property_map_id: "runtime".to_string(),
                    property_id: "phase".to_string(),
                },
                domain: StateMachineFiniteDomainSchema::Enum {
                    values: vec!["idle".to_string(), "run".to_string()],
                },
                semantics: StateMachineFiniteDomainSemanticsSchema::Exact,
            });

        assert_eq!(schema.compatibility().finite_domain_abstractions().len(), 1);
    }

    #[test]
    fn register_proof_assertion_appends_declaration() {
        let mut schema = StateMachineSchema::new("idle");
        schema
            .compatibility_mut()
            .register_proof_assertion(StateMachineProofAssertionSchema {
                label: Some("idle_is_reachable".to_string()),
                kind: StateMachineProofAssertionKindSchema::ReachableState {
                    state_name: "idle".to_string(),
                },
            });

        assert_eq!(schema.compatibility().proof_assertions().len(), 1);
    }

    #[test]
    fn register_machine_local_replaces_existing_declaration() {
        let mut schema = StateMachineSchema::new("idle");
        let mut initial = crate::properties::property_map::PropertyMap::new();
        initial.insert_bool("is_visible", true);
        schema.register_machine_local("runtime", initial);

        let mut replacement = crate::properties::property_map::PropertyMap::new();
        replacement.insert_bool("is_visible", false);
        schema.register_machine_local("runtime", replacement);

        assert_eq!(schema.machine_locals().len(), 1);
        assert_eq!(schema.machine_locals()[0].local_id, "runtime");
        assert_eq!(
            schema.machine_local_property("runtime", "is_visible"),
            Some(&crate::properties::property::Property::Bool(false))
        );
    }

    #[test]
    fn set_proof_metadata_replaces_existing_metadata() {
        let mut schema = StateMachineSchema::new("idle");
        schema
            .compatibility_mut()
            .set_proof_metadata(StateMachineProofMetadataSchema {
                proof_class: StateMachineProofClassSchema::BoundedExtended,
                finite_domain_abstractions: vec![StateMachineFiniteDomainAbstractionSchema {
                    target: StateMachineFiniteDomainTargetSchema::PropertyField {
                        property_map_id: "runtime".to_string(),
                        property_id: "phase".to_string(),
                    },
                    domain: StateMachineFiniteDomainSchema::Enum {
                        values: vec!["idle".to_string(), "run".to_string()],
                    },
                    semantics: StateMachineFiniteDomainSemanticsSchema::Exact,
                }],
                proof_assertions: vec![StateMachineProofAssertionSchema {
                    label: Some("idle_is_reachable".to_string()),
                    kind: StateMachineProofAssertionKindSchema::ReachableState {
                        state_name: "idle".to_string(),
                    },
                }],
            });

        assert_eq!(
            schema.compatibility().declared_proof_class(),
            StateMachineProofClassSchema::BoundedExtended
        );
        assert_eq!(
            schema
                .compatibility()
                .proof_metadata()
                .finite_domain_abstractions
                .len(),
            1
        );
        assert_eq!(
            schema
                .compatibility()
                .proof_metadata()
                .proof_assertions
                .len(),
            1
        );
    }

    #[test]
    fn register_machine_owned_collection_capacity_replaces_existing_declaration() {
        let mut schema = StateMachineSchema::new("idle");

        schema.register_machine_owned_collection_capacity("runtime", "inventory", 3);
        schema.register_machine_owned_collection_capacity("runtime", "inventory", 5);

        assert_eq!(
            schema.machine_owned_collection_capacities,
            vec![StateMachineOwnedCollectionCapacitySchema::new(
                "runtime",
                "inventory",
                5,
            )]
        );
        assert_eq!(
            schema.machine_owned_collection_capacity("runtime", "inventory"),
            Some(5)
        );
    }

    #[test]
    fn serialization_omits_proof_only_metadata_from_runtime_schema() {
        let mut schema =
            StateMachineSchema::new_with_proof_class("idle", StateMachineProofClassSchema::Finite);
        schema
            .compatibility_mut()
            .register_finite_domain_abstraction(StateMachineFiniteDomainAbstractionSchema {
                target: StateMachineFiniteDomainTargetSchema::PropertyField {
                    property_map_id: "runtime".to_string(),
                    property_id: "phase".to_string(),
                },
                domain: StateMachineFiniteDomainSchema::Enum {
                    values: vec!["idle".to_string(), "done".to_string()],
                },
                semantics: StateMachineFiniteDomainSemanticsSchema::Exact,
            });
        schema
            .compatibility_mut()
            .register_proof_assertion(StateMachineProofAssertionSchema {
                label: Some("idle_is_reachable".to_string()),
                kind: StateMachineProofAssertionKindSchema::ReachableState {
                    state_name: "idle".to_string(),
                },
            });
        schema.register_machine_local(
            "runtime",
            crate::properties::property_map::PropertyMap::new(),
        );

        let serialized = serde_json::to_value(&schema).expect("schema should serialize");
        assert!(serialized.get("proof_class").is_none());
        assert!(serialized.get("finite_domain_abstractions").is_none());
        assert!(serialized.get("proof_assertions").is_none());
        assert!(serialized.get("property_maps").is_none());
        assert!(serialized.get("bounded_effect_contract").is_none());
        assert!(serialized.get("synchronous_invocation_contract").is_none());
    }

    #[test]
    fn explicit_compatibility_helpers_share_underlying_state() {
        let mut schema = StateMachineSchema::new("idle");
        schema
            .compatibility_mut()
            .set_proof_class(StateMachineProofClassSchema::Finite);
        schema.register_machine_local(
            "runtime",
            crate::properties::property_map::PropertyMap::new(),
        );

        assert_eq!(
            schema.compatibility().declared_proof_class(),
            StateMachineProofClassSchema::Finite
        );
        assert_eq!(schema.machine_locals().len(), 1);
    }
}
