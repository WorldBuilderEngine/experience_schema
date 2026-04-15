use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use prost::{DecodeError, Message, Oneof};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StateMachineProofTargetSelectorSchema {
    MachineLocalField { local_id: String, field_id: String },
    StoreField { store_id: String, field_id: String },
}

impl Default for StateMachineProofTargetSelectorSchema {
    fn default() -> Self {
        Self::MachineLocalField {
            local_id: String::new(),
            field_id: String::new(),
        }
    }
}

impl Message for StateMachineProofTargetSelectorSchema {
    fn encode_raw(&self, buf: &mut impl BufMut) {
        StateMachineProofTargetSelectorBinaryWire::from(self.clone()).encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut wire = StateMachineProofTargetSelectorBinaryWire::from(self.clone());
        wire.merge_field(tag, wire_type, buf, ctx)?;
        *self = wire.into_selector();
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        StateMachineProofTargetSelectorBinaryWire::from(self.clone()).encoded_len()
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone, PartialEq, Message)]
struct StateMachineSelectorFieldBinaryWire {
    #[prost(string, tag = "1")]
    primary_id: String,
    #[prost(string, tag = "2")]
    field_id: String,
}

#[derive(Clone, PartialEq, Message)]
struct StateMachineProofTargetSelectorBinaryWire {
    #[prost(oneof = "state_machine_proof_target_selector_binary_wire::Selector", tags = "16, 17")]
    selector: Option<state_machine_proof_target_selector_binary_wire::Selector>,
}

mod state_machine_proof_target_selector_binary_wire {
    use super::*;

    #[derive(Clone, PartialEq, Oneof)]
    pub enum Selector {
        #[prost(message, tag = "16")]
        MachineLocalField(StateMachineSelectorFieldBinaryWire),
        #[prost(message, tag = "17")]
        StoreField(StateMachineSelectorFieldBinaryWire),
    }
}

impl From<StateMachineProofTargetSelectorSchema> for StateMachineProofTargetSelectorBinaryWire {
    fn from(value: StateMachineProofTargetSelectorSchema) -> Self {
        let selector = Some(match value {
            StateMachineProofTargetSelectorSchema::MachineLocalField { local_id, field_id } => {
                state_machine_proof_target_selector_binary_wire::Selector::MachineLocalField(
                    StateMachineSelectorFieldBinaryWire {
                        primary_id: local_id,
                        field_id,
                    },
                )
            }
            StateMachineProofTargetSelectorSchema::StoreField { store_id, field_id } => {
                state_machine_proof_target_selector_binary_wire::Selector::StoreField(
                    StateMachineSelectorFieldBinaryWire {
                        primary_id: store_id,
                        field_id,
                    },
                )
            }
        });
        Self { selector }
    }
}

impl StateMachineProofTargetSelectorBinaryWire {
    fn into_selector(self) -> StateMachineProofTargetSelectorSchema {
        match self.selector {
            Some(state_machine_proof_target_selector_binary_wire::Selector::MachineLocalField(field)) => {
                StateMachineProofTargetSelectorSchema::MachineLocalField {
                    local_id: field.primary_id,
                    field_id: field.field_id,
                }
            }
            Some(state_machine_proof_target_selector_binary_wire::Selector::StoreField(field)) => {
                StateMachineProofTargetSelectorSchema::StoreField {
                    store_id: field.primary_id,
                    field_id: field.field_id,
                }
            }
            None => StateMachineProofTargetSelectorSchema::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StateMachineProofTargetSelectorSchema;
    use prost::Message;

    #[test]
    fn deserializes_machine_local_field_selector() {
        let selector = serde_json::from_str::<StateMachineProofTargetSelectorSchema>(
            r#"{
                "MachineLocalField": {
                    "local_id": "runtime",
                    "field_id": "phase"
                }
            }"#,
        )
        .expect("machine-local field selector should deserialize");

        assert_eq!(
            selector,
            StateMachineProofTargetSelectorSchema::MachineLocalField {
                local_id: "runtime".to_string(),
                field_id: "phase".to_string(),
            }
        );
    }

    #[test]
    fn deserializes_store_field_selector() {
        let selector = serde_json::from_str::<StateMachineProofTargetSelectorSchema>(
            r#"{
                "StoreField": {
                    "store_id": "world.default.presentation_runtime.warm",
                    "field_id": "default_camera_viewport_size_px"
                }
            }"#,
        )
        .expect("store field selector should deserialize");

        assert_eq!(
            selector,
            StateMachineProofTargetSelectorSchema::StoreField {
                store_id: "world.default.presentation_runtime.warm".to_string(),
                field_id: "default_camera_viewport_size_px".to_string(),
            }
        );
    }

    #[test]
    fn prost_round_trips_selector_as_binary_message() {
        let selector = StateMachineProofTargetSelectorSchema::MachineLocalField {
            local_id: "runtime".to_string(),
            field_id: "phase".to_string(),
        };

        let encoded = selector.encode_to_vec();
        let decoded = StateMachineProofTargetSelectorSchema::decode(encoded.as_slice()).expect("selector should decode");

        assert_eq!(decoded, selector);
    }

}
