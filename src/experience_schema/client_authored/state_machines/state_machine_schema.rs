use crate::client_authored::state_machines::api::StateMachineApiSchema;
use crate::client_authored::state_machines::state_machine_boot_handle_binding_schema::StateMachineBootHandleBindingSchema;
use crate::client_authored::state_machines::state_machine_boot_named_handle_binding_schema::StateMachineBootNamedHandleBindingSchema;
use crate::client_authored::state_machines::state_machine_local_field_schema::StateMachineLocalFieldSchema;
use crate::client_authored::state_machines::state_machine_local_schema::StateMachineLocalSchema;
use crate::client_authored::state_machines::state_machine_node_schema::{
    StateMachineNodeSchema, StateMachineNodeTypeSchema,
};
use crate::client_authored::state_machines::state_machine_owned_collection_capacity_schema::StateMachineOwnedCollectionCapacitySchema;
use crate::client_authored::state_machines::state_machine_transition_schema::StateMachineTransitionSchema;
use crate::properties::property_map::PropertyMap;
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
    pub boot_named_handle_bindings: Vec<StateMachineBootNamedHandleBindingSchema>,
    #[serde(default)]
    pub boot_handle_bindings: Vec<StateMachineBootHandleBindingSchema>,
    #[serde(default)]
    pub machine_owned_collection_capacities: Vec<StateMachineOwnedCollectionCapacitySchema>,
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
            initial_state_name: initial_state_name.into(),
            deterministic_seed,
            machine_locals: Vec::new(),
            boot_named_handle_bindings: Vec::new(),
            boot_handle_bindings: Vec::new(),
            machine_owned_collection_capacities: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn machine_locals(&self) -> &[StateMachineLocalSchema] {
        self.machine_locals.as_slice()
    }

    pub fn boot_named_handle_bindings(&self) -> &[StateMachineBootNamedHandleBindingSchema] {
        self.boot_named_handle_bindings.as_slice()
    }

    pub fn boot_handle_bindings(&self) -> &[StateMachineBootHandleBindingSchema] {
        self.boot_handle_bindings.as_slice()
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

    pub fn register_boot_named_handle_binding(
        &mut self,
        binding: StateMachineBootNamedHandleBindingSchema,
    ) {
        if let Some(existing_binding_index) =
            self.boot_named_handle_bindings
                .iter()
                .position(|existing_binding| {
                    existing_binding.local_id == binding.local_id
                        && existing_binding.property_id == binding.property_id
                })
        {
            self.boot_named_handle_bindings[existing_binding_index] = binding;
            return;
        }

        self.boot_named_handle_bindings.push(binding);
    }

    pub fn register_boot_handle_binding(&mut self, binding: StateMachineBootHandleBindingSchema) {
        if let Some(existing_binding_index) =
            self.boot_handle_bindings
                .iter()
                .position(|existing_binding| {
                    existing_binding.local_id == binding.local_id
                        && existing_binding.property_id == binding.property_id
                })
        {
            self.boot_handle_bindings[existing_binding_index] = binding;
            return;
        }

        self.boot_handle_bindings.push(binding);
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
                    emitted_local_event_names: Vec::new(),
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
                emitted_local_event_names: Vec::new(),
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
        StateMachineSchemaBinaryWire::from(self.clone()).encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut wire = StateMachineSchemaBinaryWire::from(self.clone());
        wire.merge_field(tag, wire_type, buf, ctx)?;
        *self = wire.into_schema()?;
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        StateMachineSchemaBinaryWire::from(self.clone()).encoded_len()
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone, PartialEq, Message)]
struct StateMachineSchemaBinaryWire {
    #[prost(string, tag = "16")]
    initial_state_name: String,
    #[prost(uint64, tag = "17")]
    deterministic_seed: u64,
    #[prost(message, repeated, tag = "18")]
    machine_locals: Vec<StateMachineLocalSchema>,
    #[prost(message, repeated, tag = "19")]
    boot_named_handle_bindings: Vec<StateMachineBootNamedHandleBindingSchema>,
    #[prost(message, repeated, tag = "20")]
    machine_owned_collection_capacities: Vec<StateMachineOwnedCollectionCapacitySchema>,
    #[prost(message, repeated, tag = "21")]
    nodes: Vec<StateMachineNodeSchema>,
    #[prost(message, repeated, tag = "22")]
    boot_handle_bindings: Vec<StateMachineBootHandleBindingSchema>,
}

impl From<StateMachineSchema> for StateMachineSchemaBinaryWire {
    fn from(value: StateMachineSchema) -> Self {
        let StateMachineSchema {
            initial_state_name,
            deterministic_seed,
            machine_locals,
            boot_named_handle_bindings,
            boot_handle_bindings,
            machine_owned_collection_capacities,
            nodes,
        } = value;
        Self {
            initial_state_name,
            deterministic_seed,
            machine_locals,
            boot_named_handle_bindings,
            boot_handle_bindings,
            machine_owned_collection_capacities,
            nodes,
        }
    }
}

impl StateMachineSchemaBinaryWire {
    fn into_schema(self) -> Result<StateMachineSchema, DecodeError> {
        Ok(StateMachineSchema {
            initial_state_name: self.initial_state_name,
            deterministic_seed: self.deterministic_seed,
            machine_locals: self.machine_locals,
            boot_named_handle_bindings: self.boot_named_handle_bindings,
            boot_handle_bindings: self.boot_handle_bindings,
            machine_owned_collection_capacities: self.machine_owned_collection_capacities,
            nodes: self.nodes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::StateMachineSchema;
    use crate::client_authored::state_machines::state_machine_boot_named_handle_binding_schema::{
        StateMachineBootNamedHandleBindingSchema, StateMachineBootNamedHandleKindSchema,
    };
    use crate::client_authored::state_machines::state_machine_owned_collection_capacity_schema::StateMachineOwnedCollectionCapacitySchema;
    use prost::Message;

    #[test]
    fn constructor_populates_core_fields() {
        let schema = StateMachineSchema::new("idle");
        assert_eq!(schema.initial_state_name, "idle".to_string());
        assert_eq!(schema.deterministic_seed, 0);
    }

    #[test]
    fn constructor_supports_explicit_seed() {
        let schema = StateMachineSchema::new_with_seed("idle", 7);
        assert_eq!(schema.initial_state_name, "idle");
        assert_eq!(schema.deterministic_seed, 7);
    }

    #[test]
    fn deserialization_ignores_legacy_proof_metadata_fields() {
        let schema = serde_json::from_str::<StateMachineSchema>(
            r#"{
                "proof_class":"finite",
                "initial_state_name":"idle",
                "finite_domain_abstractions": [],
                "proof_assertions": [],
                "nodes":[]
            }"#,
        )
        .expect("legacy proof metadata fields should be ignored");

        assert_eq!(schema.initial_state_name, "idle");
        assert!(schema.boot_named_handle_bindings.is_empty());
        assert!(schema.machine_owned_collection_capacities.is_empty());
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
    fn register_boot_named_handle_binding_replaces_existing_declaration() {
        let mut schema = StateMachineSchema::new("idle");

        schema.register_boot_named_handle_binding(StateMachineBootNamedHandleBindingSchema::new(
            "runtime_values",
            "target_handle_bytes",
            StateMachineBootNamedHandleKindSchema::Node,
            "ui:panel",
        ));
        schema.register_boot_named_handle_binding(StateMachineBootNamedHandleBindingSchema::new(
            "runtime_values",
            "target_handle_bytes",
            StateMachineBootNamedHandleKindSchema::Camera,
            "camera:primary",
        ));

        assert_eq!(
            schema.boot_named_handle_bindings,
            vec![StateMachineBootNamedHandleBindingSchema::new(
                "runtime_values",
                "target_handle_bytes",
                StateMachineBootNamedHandleKindSchema::Camera,
                "camera:primary",
            )]
        );
    }

    #[test]
    fn serialization_omits_proof_only_metadata_from_runtime_schema() {
        let mut schema = StateMachineSchema::new("idle");
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
    fn prost_round_trips_state_machine_schema_as_binary_message() {
        let mut schema = StateMachineSchema::new("idle");
        schema.deterministic_seed = 7;
        schema.register_machine_local(
            "runtime",
            crate::properties::property_map::PropertyMap::new(),
        );
        schema.register_machine_owned_collection_capacity("runtime", "scratch", 16);

        let encoded = schema.encode_to_vec();
        let decoded = StateMachineSchema::decode(encoded.as_slice())
            .expect("state machine schema should decode");

        assert_eq!(decoded, schema);
    }

    #[test]
    fn prost_state_machine_schema_omits_proof_only_metadata_from_runtime_wire() {
        let schema = StateMachineSchema::new_with_seed("idle", 7);

        let decoded = StateMachineSchema::decode(schema.encode_to_vec().as_slice())
            .expect("state machine schema should decode");

        assert_eq!(decoded, schema);
    }
}
