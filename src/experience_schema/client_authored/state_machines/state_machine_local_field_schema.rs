use crate::properties::property::Property;
use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Message)]
pub struct StateMachineLocalFieldSchema {
    #[prost(string, tag = "1")]
    pub field_id: String,
    #[serde(default)]
    #[prost(message, required, tag = "2")]
    pub value: Property,
}

impl StateMachineLocalFieldSchema {
    pub fn new(field_id: impl Into<String>, value: Property) -> Self {
        Self {
            field_id: field_id.into().trim().to_string(),
            value,
        }
    }
}
