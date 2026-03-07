use crate::prost_json_message::{
    encode_as_json_message, json_message_encoded_len, merge_from_json_message,
};
use prost::DecodeError;
use prost::Message;
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
        property_map_id: String,
        property_id: String,
    },
    Default,
    DeterministicRandom {
        threshold_numerator: u32,
        threshold_denominator: u32,
    },
}

impl Message for StateMachineTransitionTriggerSchema {
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
