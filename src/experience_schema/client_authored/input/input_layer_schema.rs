use crate::client_authored::input::input_action_schema::InputActionSchema;
use prost::Message;
use serde::{Deserialize, Serialize};

/// Defines a set of actions that can be simultaneously activated and deactivated.
/// Generally, these input all fall in the same domain (UI, game, mini-game, etc.).
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct InputLayerSchema {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(message, repeated, tag = "2")]
    pub input_actions: Vec<InputActionSchema>,
}
