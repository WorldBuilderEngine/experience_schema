use crate::shared::state_machine_transition_schema::StateMachineTransitionSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Node action metadata that executes on state entry.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StateMachineNodeTypeSchema {
    ApiDispatch {
        api_identifier: String,
        args_property_map_id: Option<String>,
    },
}

impl Default for StateMachineNodeTypeSchema {
    fn default() -> Self {
        Self::ApiDispatch {
            api_identifier: String::new(),
            args_property_map_id: None,
        }
    }
}

/// Serializable state-node configuration keyed by state name.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct StateMachineNodeSchema {
    pub state_name: String,
    #[serde(default)]
    pub node_type: StateMachineNodeTypeSchema,
    #[serde(default)]
    pub transitions: Vec<StateMachineTransitionSchema>,
    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
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
            _extensions: HashMap::new(),
        }
    }

    pub fn add_transition(&mut self, transition: StateMachineTransitionSchema) {
        self.transitions.push(transition);
    }
}
