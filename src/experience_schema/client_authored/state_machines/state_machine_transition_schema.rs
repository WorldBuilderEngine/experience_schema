use serde::{Deserialize, Serialize};

/// Transition trigger types supported by serialized state-machine definitions.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum StateMachineTransitionTriggerSchema {
    #[default]
    Always,
    GlobalEvent(String),
    LocalEvent(String),
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
