use crate::shared::state_machine_node_schema::{StateMachineNodeSchema, StateMachineNodeTypeSchema};
use crate::shared::state_machine_transition_schema::StateMachineTransitionSchema;
use serde::{Deserialize, Serialize};

/// Serializable state-machine definition used in authored world schemas.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StateMachineSchema {
    pub initial_state_name: String,
    #[serde(default)]
    pub deterministic_seed: u64,
    #[serde(default)]
    pub nodes: Vec<StateMachineNodeSchema>,
}

impl Default for StateMachineSchema {
    fn default() -> Self {
        Self::new("")
    }
}

impl StateMachineSchema {
    pub fn new(initial_state_name: impl Into<String>) -> Self {
        Self::new_with_seed(initial_state_name, 0)
    }

    pub fn new_with_seed(initial_state_name: impl Into<String>, deterministic_seed: u64) -> Self {
        Self {
            initial_state_name: initial_state_name.into(),
            deterministic_seed,
            nodes: Vec::new(),
        }
    }

    pub fn add_transition(&mut self, transition: StateMachineTransitionSchema) {
        self.nodes
            .push(StateMachineNodeSchema::new_with_transitions(
                transition.from_state_name.clone(),
                StateMachineNodeTypeSchema::Plain,
                vec![transition],
            ));
    }

    pub fn register_callback_node(
        &mut self,
        state_name: impl Into<String>,
        callback_name: impl Into<String>,
    ) {
        self.nodes.push(StateMachineNodeSchema::new(
            state_name,
            StateMachineNodeTypeSchema::Callback {
                callback_name: callback_name.into(),
            },
        ));
    }

    pub fn register_spawn_nested_state_machine_node(
        &mut self,
        state_name: impl Into<String>,
        nested_state_machine: StateMachineSchema,
    ) {
        self.nodes.push(StateMachineNodeSchema::new(
            state_name,
            StateMachineNodeTypeSchema::SpawnNestedStateMachine {
                state_machine: Box::new(nested_state_machine),
            },
        ));
    }

    pub fn register_terminate_node(&mut self, state_name: impl Into<String>) {
        self.nodes.push(StateMachineNodeSchema::new(
            state_name,
            StateMachineNodeTypeSchema::Terminate,
        ));
    }
}
