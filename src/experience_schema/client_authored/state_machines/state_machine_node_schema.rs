use crate::shared::state_machine_transition_schema::StateMachineTransitionSchema;
use serde::{Deserialize, Serialize};

/// Script language metadata supported by serialized state-machine nodes.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ScriptLanguageSchema {
    Lua,
    JavaScript,
}

/// Node action metadata that executes on state entry.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum StateMachineNodeTypeSchema {
    #[default]
    Plain,
    Script {
        script_name: String,
        script_language: ScriptLanguageSchema,
    },
    Callback {
        callback_name: String,
    },
    SpawnObjectTemplate {
        template_name: String,
    },
    SpawnNestedStateMachine {
        state_machine: Box<crate::shared::state_machine_schema::StateMachineSchema>,
    },
    SetNodeVisibilityByTag {
        node_tag: String,
        is_visible: bool,
    },
    SetNodePositionByTag {
        node_tag: String,
        position_xyz: Vec<f64>,
    },
    Terminate,
}

/// Serializable state-node configuration keyed by state name.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct StateMachineNodeSchema {
    pub state_name: String,
    #[serde(default)]
    pub node_type: StateMachineNodeTypeSchema,
    #[serde(default)]
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
