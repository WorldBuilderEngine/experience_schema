use crate::client_authored::state_machines::api::StateMachineApiSchema;
use crate::client_authored::state_machines::state_machine_transition_schema::StateMachineTransitionSchema;
use prost::DecodeError;
use prost::Message;
use prost::Oneof;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::{Deserialize, Serialize};

/// Node action metadata that executes on state entry.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StateMachineNodeTypeSchema {
    ApiDispatch {
        #[serde(alias = "api_identifier")]
        api: StateMachineApiSchema,
        args_local_id: Option<String>,
        #[serde(default)]
        emitted_local_event_names: Vec<String>,
    },
}

impl Message for StateMachineNodeTypeSchema {
    fn encode_raw(&self, buf: &mut impl BufMut) {
        StateMachineNodeTypeBinaryWire::from(self.clone()).encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut wire = StateMachineNodeTypeBinaryWire::from(self.clone());
        wire.merge_field(tag, wire_type, buf, ctx)?;
        *self = wire.into_node_type();
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        StateMachineNodeTypeBinaryWire::from(self.clone()).encoded_len()
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone, PartialEq, Message)]
struct ApiDispatchNodeTypeBinaryWire {
    #[prost(message, optional, tag = "1")]
    api: Option<StateMachineApiSchema>,
    #[prost(string, optional, tag = "2")]
    args_local_id: Option<String>,
    #[prost(string, repeated, tag = "3")]
    emitted_local_event_names: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
struct StateMachineNodeTypeBinaryWire {
    #[prost(oneof = "state_machine_node_type_binary_wire::NodeType", tags = "16")]
    node_type: Option<state_machine_node_type_binary_wire::NodeType>,
}

mod state_machine_node_type_binary_wire {
    use super::*;

    #[derive(Clone, PartialEq, Oneof)]
    pub enum NodeType {
        #[prost(message, tag = "16")]
        ApiDispatch(ApiDispatchNodeTypeBinaryWire),
    }
}

impl From<StateMachineNodeTypeSchema> for StateMachineNodeTypeBinaryWire {
    fn from(value: StateMachineNodeTypeSchema) -> Self {
        let node_type = Some(match value {
            StateMachineNodeTypeSchema::ApiDispatch {
                api,
                args_local_id,
                emitted_local_event_names,
            } => state_machine_node_type_binary_wire::NodeType::ApiDispatch(
                ApiDispatchNodeTypeBinaryWire {
                    api: Some(api),
                    args_local_id,
                    emitted_local_event_names,
                },
            ),
        });
        Self { node_type }
    }
}

impl StateMachineNodeTypeBinaryWire {
    fn into_node_type(self) -> StateMachineNodeTypeSchema {
        match self.node_type {
            Some(state_machine_node_type_binary_wire::NodeType::ApiDispatch(value)) => {
                StateMachineNodeTypeSchema::ApiDispatch {
                    api: value.api.unwrap_or_default(),
                    args_local_id: value.args_local_id,
                    emitted_local_event_names: value.emitted_local_event_names,
                }
            }
            None => StateMachineNodeTypeSchema::default(),
        }
    }
}

impl Default for StateMachineNodeTypeSchema {
    fn default() -> Self {
        Self::ApiDispatch {
            api: StateMachineApiSchema::default(),
            args_local_id: None,
            emitted_local_event_names: Vec::new(),
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
    use prost::Message;

    #[test]
    fn deserializes_api_dispatch_from_api_identifier_field() {
        let node_type_json = r#"{
            "ApiDispatch": {
                "api_identifier": "world:set_node_visibility",
                "args_local_id": "args_visibility"
            }
        }"#;

        let parsed_node_type = serde_json::from_str::<StateMachineNodeTypeSchema>(node_type_json)
            .expect("expected api_identifier payload to deserialize");

        assert_eq!(
            parsed_node_type,
            StateMachineNodeTypeSchema::ApiDispatch {
                api: StateMachineApiSchema::from("world:set_node_visibility"),
                args_local_id: Some("args_visibility".to_string()),
                emitted_local_event_names: Vec::new(),
            }
        );
    }

    #[test]
    fn prost_round_trips_node_type_as_binary_message() {
        let node_type = StateMachineNodeTypeSchema::ApiDispatch {
            api: StateMachineApiSchema::from("runtime:no_op"),
            args_local_id: Some("runtime_args".to_string()),
            emitted_local_event_names: vec!["script:advance".to_string()],
        };

        let encoded = node_type.encode_to_vec();
        let decoded = StateMachineNodeTypeSchema::decode(encoded.as_slice())
            .expect("node type should decode");

        assert_eq!(decoded, node_type);
    }
}
