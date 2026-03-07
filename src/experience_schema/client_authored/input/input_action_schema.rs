use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct InputActionSchema {
    #[prost(string, tag = "1")]
    pub input_action: String,
    #[prost(string, tag = "2")]
    pub input_event: String,
}
