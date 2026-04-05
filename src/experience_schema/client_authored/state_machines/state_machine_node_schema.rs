use crate::client_authored::state_machines::api::StateMachineApiSchema;
use crate::client_authored::state_machines::state_machine_transition_schema::StateMachineTransitionSchema;
use crate::wire_compat::json_message::{
    encode_as_json_message, json_message_encoded_len, merge_from_json_message,
};
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::{Deserialize, Serialize};

/// Node action metadata that executes on state entry.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StateMachineNodeTypeSchema {
    ApiDispatch {
        #[serde(alias = "api_identifier")]
        api: StateMachineApiSchema,
        args_property_map_id: Option<String>,
    },
}

impl Message for StateMachineNodeTypeSchema {
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

impl Default for StateMachineNodeTypeSchema {
    fn default() -> Self {
        Self::ApiDispatch {
            api: StateMachineApiSchema::default(),
            args_property_map_id: None,
        }
    }
}

/// Serializable state-node configuration keyed by state name.
#[derive(Clone, Serialize, Deserialize, PartialEq, Message)]
pub struct StateMachineNodeSchema {
    #[prost(string, tag = "1")]
    pub state_name: String,
    #[serde(default)]
    #[prost(message, required, tag = "2")]
    pub node_type: StateMachineNodeTypeSchema,
    #[serde(default)]
    #[prost(message, repeated, tag = "3")]
    pub transitions: Vec<StateMachineTransitionSchema>,
}

impl StateMachineNodeSchema {
    pub fn new(state_name: impl Into<String>, node_type: StateMachineNodeTypeSchema) -> Self {
        Self::new_with_transitions(state_name, node_type, Vec::new())
    }

    pub fn new_with_transitions(
        state_name: impl Into<String>,
        node_type: StateMachineNodeTypeSchema,
        transitions: Vec<StateMachineTransitionSchema>,
    ) -> Self {
        Self {
            state_name: state_name.into(),
            node_type,
            transitions,
        }
    }

    pub fn add_transition(&mut self, transition: StateMachineTransitionSchema) {
        self.transitions.push(transition);
    }
}

#[cfg(test)]
mod tests {
    use super::StateMachineNodeTypeSchema;
    use crate::client_authored::state_machines::api::StateMachineApiSchema;

    #[test]
    fn deserializes_api_dispatch_from_api_identifier_field() {
        let node_type_json = r#"{
            "ApiDispatch": {
                "api_identifier": "world:set_node_visibility_by_tag",
                "args_property_map_id": "args_visibility"
            }
        }"#;

        let parsed_node_type = serde_json::from_str::<StateMachineNodeTypeSchema>(node_type_json)
            .expect("expected api_identifier payload to deserialize");

        assert_eq!(
            parsed_node_type,
            StateMachineNodeTypeSchema::ApiDispatch {
                api: StateMachineApiSchema::from("world:set_node_visibility_by_tag"),
                args_property_map_id: Some("args_visibility".to_string()),
            }
        );
    }
}
