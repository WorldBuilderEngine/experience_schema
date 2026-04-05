use crate::wire_compat::json_message::{
    encode_as_json_message, json_message_encoded_len, merge_from_json_message,
};
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WorldProtocolProofAssertionSchema {
    #[serde(default)]
    pub label: Option<String>,
    pub kind: WorldProtocolProofAssertionKindSchema,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorldProtocolProofAssertionKindSchema {
    MachineAssertionHolds {
        machine_label: String,
        #[serde(default)]
        assertion_label: Option<String>,
        #[serde(default)]
        assertion_index: Option<usize>,
    },
    InvocationAllowed {
        invocation: WorldProtocolInvocationEventSchema,
    },
    InvocationForbidden {
        invocation: WorldProtocolInvocationEventSchema,
    },
    RequiredInvocationSequence {
        first: WorldProtocolInvocationEventSchema,
        then: WorldProtocolInvocationEventSchema,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldProtocolInvocationEventSchema {
    pub caller_machine_label: String,
    pub callee_machine_label: String,
    pub entrypoint: String,
}

impl Message for WorldProtocolProofAssertionSchema {
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

impl Default for WorldProtocolProofAssertionSchema {
    fn default() -> Self {
        Self {
            label: None,
            kind: WorldProtocolProofAssertionKindSchema::InvocationAllowed {
                invocation: WorldProtocolInvocationEventSchema::default(),
            },
        }
    }
}

impl Default for WorldProtocolInvocationEventSchema {
    fn default() -> Self {
        Self {
            caller_machine_label: String::new(),
            callee_machine_label: String::new(),
            entrypoint: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        WorldProtocolInvocationEventSchema, WorldProtocolProofAssertionKindSchema,
        WorldProtocolProofAssertionSchema,
    };

    #[test]
    fn deserializes_machine_assertion_protocol_contract() {
        let assertion = serde_json::from_str::<WorldProtocolProofAssertionSchema>(
            r#"{
                "label": "resolver_safe",
                "kind": {
                    "MachineAssertionHolds": {
                        "machine_label": "combat:resolver",
                        "assertion_label": "resolver_finishes"
                    }
                }
            }"#,
        )
        .expect("machine assertion protocol contract should deserialize");

        assert_eq!(
            assertion,
            WorldProtocolProofAssertionSchema {
                label: Some("resolver_safe".to_string()),
                kind: WorldProtocolProofAssertionKindSchema::MachineAssertionHolds {
                    machine_label: "combat:resolver".to_string(),
                    assertion_label: Some("resolver_finishes".to_string()),
                    assertion_index: None,
                },
            }
        );
    }

    #[test]
    fn deserializes_required_invocation_sequence_contract() {
        let assertion = serde_json::from_str::<WorldProtocolProofAssertionSchema>(
            r#"{
                "label": "resolve_then_apply_damage",
                "kind": {
                    "RequiredInvocationSequence": {
                        "first": {
                            "caller_machine_label": "combat:resolver",
                            "callee_machine_label": "combat:rules",
                            "entrypoint": "apply_damage"
                        },
                        "then": {
                            "caller_machine_label": "combat:rules",
                            "callee_machine_label": "combat:log",
                            "entrypoint": "record_damage"
                        }
                    }
                }
            }"#,
        )
        .expect("required invocation sequence contract should deserialize");

        assert_eq!(
            assertion,
            WorldProtocolProofAssertionSchema {
                label: Some("resolve_then_apply_damage".to_string()),
                kind: WorldProtocolProofAssertionKindSchema::RequiredInvocationSequence {
                    first: WorldProtocolInvocationEventSchema {
                        caller_machine_label: "combat:resolver".to_string(),
                        callee_machine_label: "combat:rules".to_string(),
                        entrypoint: "apply_damage".to_string(),
                    },
                    then: WorldProtocolInvocationEventSchema {
                        caller_machine_label: "combat:rules".to_string(),
                        callee_machine_label: "combat:log".to_string(),
                        entrypoint: "record_damage".to_string(),
                    },
                },
            }
        );
    }
}
