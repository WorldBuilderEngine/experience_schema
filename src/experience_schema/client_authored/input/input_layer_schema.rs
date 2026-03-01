use crate::client_authored::input::input_action_schema::InputActionSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Defines a set of actions that can be simultaneously activated and deactivated.
/// Generally, these input all fall in the same domain (UI, game, mini-game, etc.).
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct InputLayerSchema {
    pub name: String,
    pub input_actions: Vec<InputActionSchema>,
    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
}
