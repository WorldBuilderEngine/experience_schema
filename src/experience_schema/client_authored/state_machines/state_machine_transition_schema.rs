use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Directed transition between source and destination states.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct StateMachineTransitionSchema {
    pub from_state_name: String,
    pub to_state_name: String,
    pub trigger: StateMachineTransitionTriggerSchema,
    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
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
            _extensions: HashMap::new(),
        }
    }
}
