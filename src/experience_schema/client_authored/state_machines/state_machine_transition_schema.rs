use crate::client_authored::state_machines::state_machine_proof_target_selector_schema::StateMachineProofTargetSelectorSchema;
use prost::DecodeError;
use prost::Message;
use prost::Oneof;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::{Deserialize, Serialize};

/// Transition trigger types supported by serialized state-machine definitions.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum StateMachineTransitionTriggerSchema {
    #[default]
    Always,
    GlobalEvent(String),
    LocalEvent(String),
    Conditional {
        local_id: String,
        property_id: String,
    },
    ConditionalSelector {
        selector: StateMachineProofTargetSelectorSchema,
    },
    Default,
    DeterministicRandom {
        threshold_numerator: u32,
        threshold_denominator: u32,
    },
}

impl Message for StateMachineTransitionTriggerSchema {
    fn encode_raw(&self, buf: &mut impl BufMut) {
        StateMachineTransitionTriggerBinaryWire::from(self.clone()).encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut wire = StateMachineTransitionTriggerBinaryWire::from(self.clone());
        wire.merge_field(tag, wire_type, buf, ctx)?;
        *self = wire.into_trigger();
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        StateMachineTransitionTriggerBinaryWire::from(self.clone()).encoded_len()
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone, PartialEq, Message)]
struct ConditionalTriggerBinaryWire {
    #[prost(string, tag = "1")]
    local_id: String,
    #[prost(string, tag = "2")]
    property_id: String,
}

#[derive(Clone, PartialEq, Message)]
struct ConditionalSelectorTriggerBinaryWire {
    #[prost(message, optional, tag = "1")]
    selector: Option<StateMachineProofTargetSelectorSchema>,
}

#[derive(Clone, PartialEq, Message)]
struct DeterministicRandomTriggerBinaryWire {
    #[prost(uint32, tag = "1")]
    threshold_numerator: u32,
    #[prost(uint32, tag = "2")]
    threshold_denominator: u32,
}

#[derive(Clone, PartialEq, Message)]
struct StateMachineTransitionTriggerBinaryWire {
    #[prost(
        oneof = "state_machine_transition_trigger_binary_wire::Trigger",
        tags = "16, 17, 18, 19, 20, 21, 22"
    )]
    trigger: Option<state_machine_transition_trigger_binary_wire::Trigger>,
}

mod state_machine_transition_trigger_binary_wire {
    use super::*;

    #[derive(Clone, PartialEq, Oneof)]
    pub enum Trigger {
        #[prost(bool, tag = "16")]
        Always(bool),
        #[prost(string, tag = "17")]
        GlobalEvent(String),
        #[prost(string, tag = "18")]
        LocalEvent(String),
        #[prost(message, tag = "19")]
        Conditional(ConditionalTriggerBinaryWire),
        #[prost(message, tag = "20")]
        ConditionalSelector(ConditionalSelectorTriggerBinaryWire),
        #[prost(bool, tag = "21")]
        Default(bool),
        #[prost(message, tag = "22")]
        DeterministicRandom(DeterministicRandomTriggerBinaryWire),
    }
}

impl From<StateMachineTransitionTriggerSchema> for StateMachineTransitionTriggerBinaryWire {
    fn from(value: StateMachineTransitionTriggerSchema) -> Self {
        let trigger = Some(match value {
            StateMachineTransitionTriggerSchema::Always => {
                state_machine_transition_trigger_binary_wire::Trigger::Always(true)
            }
            StateMachineTransitionTriggerSchema::GlobalEvent(value) => {
                state_machine_transition_trigger_binary_wire::Trigger::GlobalEvent(value)
            }
            StateMachineTransitionTriggerSchema::LocalEvent(value) => {
                state_machine_transition_trigger_binary_wire::Trigger::LocalEvent(value)
            }
            StateMachineTransitionTriggerSchema::Conditional {
                local_id,
                property_id,
            } => state_machine_transition_trigger_binary_wire::Trigger::Conditional(
                ConditionalTriggerBinaryWire {
                    local_id,
                    property_id,
                },
            ),
            StateMachineTransitionTriggerSchema::ConditionalSelector { selector } => {
                state_machine_transition_trigger_binary_wire::Trigger::ConditionalSelector(
                    ConditionalSelectorTriggerBinaryWire {
                        selector: Some(selector),
                    },
                )
            }
            StateMachineTransitionTriggerSchema::Default => {
                state_machine_transition_trigger_binary_wire::Trigger::Default(true)
            }
            StateMachineTransitionTriggerSchema::DeterministicRandom {
                threshold_numerator,
                threshold_denominator,
            } => state_machine_transition_trigger_binary_wire::Trigger::DeterministicRandom(
                DeterministicRandomTriggerBinaryWire {
                    threshold_numerator,
                    threshold_denominator,
                },
            ),
        });
        Self { trigger }
    }
}

impl StateMachineTransitionTriggerBinaryWire {
    fn into_trigger(self) -> StateMachineTransitionTriggerSchema {
        match self.trigger {
            Some(state_machine_transition_trigger_binary_wire::Trigger::Always(_)) => {
                StateMachineTransitionTriggerSchema::Always
            }
            Some(state_machine_transition_trigger_binary_wire::Trigger::GlobalEvent(value)) => {
                StateMachineTransitionTriggerSchema::GlobalEvent(value)
            }
            Some(state_machine_transition_trigger_binary_wire::Trigger::LocalEvent(value)) => {
                StateMachineTransitionTriggerSchema::LocalEvent(value)
            }
            Some(state_machine_transition_trigger_binary_wire::Trigger::Conditional(value)) => {
                StateMachineTransitionTriggerSchema::Conditional {
                    local_id: value.local_id,
                    property_id: value.property_id,
                }
            }
            Some(state_machine_transition_trigger_binary_wire::Trigger::ConditionalSelector(
                value,
            )) => StateMachineTransitionTriggerSchema::ConditionalSelector {
                selector: value.selector.unwrap_or(
                    StateMachineProofTargetSelectorSchema::MachineLocalField {
                        local_id: String::new(),
                        field_id: String::new(),
                    },
                ),
            },
            Some(state_machine_transition_trigger_binary_wire::Trigger::Default(_)) => {
                StateMachineTransitionTriggerSchema::Default
            }
            Some(state_machine_transition_trigger_binary_wire::Trigger::DeterministicRandom(
                value,
            )) => StateMachineTransitionTriggerSchema::DeterministicRandom {
                threshold_numerator: value.threshold_numerator,
                threshold_denominator: value.threshold_denominator,
            },
            None => StateMachineTransitionTriggerSchema::Always,
        }
    }
}

/// Directed transition between source and destination states.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Message)]
pub struct StateMachineTransitionSchema {
    #[prost(string, tag = "1")]
    pub from_state_name: String,
    #[prost(string, tag = "2")]
    pub to_state_name: String,
    #[prost(message, required, tag = "3")]
    pub trigger: StateMachineTransitionTriggerSchema,
}

impl StateMachineTransitionSchema {
    pub fn new(
        from_state_name: impl Into<String>,
        to_state_name: impl Into<String>,
        trigger: StateMachineTransitionTriggerSchema,
    ) -> Self {
        Self {
            from_state_name: from_state_name.into(),
            to_state_name: to_state_name.into(),
            trigger,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StateMachineTransitionTriggerSchema;
    use crate::client_authored::state_machines::state_machine_proof_target_selector_schema::StateMachineProofTargetSelectorSchema;
    use prost::Message;

    #[test]
    fn transition_trigger_deserializes_conditional_selector() {
        let trigger = serde_json::from_str::<StateMachineTransitionTriggerSchema>(
            r#"{
                "ConditionalSelector": {
                    "selector": {
                        "MachineLocalField": {
                            "local_id": "runtime",
                            "field_id": "flag"
                        }
                    }
                }
            }"#,
        )
        .expect("conditional selector trigger should deserialize");

        assert_eq!(
            trigger,
            StateMachineTransitionTriggerSchema::ConditionalSelector {
                selector: StateMachineProofTargetSelectorSchema::MachineLocalField {
                    local_id: "runtime".to_string(),
                    field_id: "flag".to_string(),
                },
            }
        );
    }

    #[test]
    fn prost_round_trips_trigger_as_binary_message() {
        let trigger = StateMachineTransitionTriggerSchema::DeterministicRandom {
            threshold_numerator: 1,
            threshold_denominator: 3,
        };

        let encoded = trigger.encode_to_vec();
        let decoded = StateMachineTransitionTriggerSchema::decode(encoded.as_slice())
            .expect("trigger should decode");

        assert_eq!(decoded, trigger);
    }
}
