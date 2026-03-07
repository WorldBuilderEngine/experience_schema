use crate::client_authored::state_machines::api::StateMachineApiSchema;
use crate::client_authored::state_machines::state_machine_node_schema::{
    StateMachineNodeSchema, StateMachineNodeTypeSchema,
};
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
            property_maps: Vec::new(),
            nodes: Vec::new(),
        }
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

    #[test]
    fn constructor_populates_core_fields() {
        let schema = StateMachineSchema::new("idle");
        assert_eq!(schema.initial_state_name, "idle".to_string());
        assert_eq!(schema.deterministic_seed, 0);
    }
}
