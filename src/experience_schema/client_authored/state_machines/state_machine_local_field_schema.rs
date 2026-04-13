use crate::properties::property::Property;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StateMachineLocalFieldSchema {
    pub field_id: String,
    #[serde(default)]
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
