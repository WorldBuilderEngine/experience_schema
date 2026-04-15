use crate::client_authored::state_machines::state_machine_local_field_schema::StateMachineLocalFieldSchema;
use crate::properties::{property::Property, property_map::PropertyMap};
use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Message)]
pub struct StateMachineLocalSchema {
    #[prost(string, tag = "1")]
    pub local_id: String,
    #[serde(default)]
    #[prost(message, repeated, tag = "2")]
    pub fields: Vec<StateMachineLocalFieldSchema>,
}

impl StateMachineLocalSchema {
    pub fn new(local_id: impl Into<String>, fields: Vec<StateMachineLocalFieldSchema>) -> Self {
        Self {
            local_id: local_id.into().trim().to_string(),
            fields,
        }
    }

    pub fn from_property_map(local_id: impl Into<String>, properties: PropertyMap) -> Self {
        Self::new(
            local_id,
            properties
                .properties
                .into_iter()
                .map(|(field_id, value)| StateMachineLocalFieldSchema::new(field_id, value))
                .collect(),
        )
    }

    pub fn fields(&self) -> &[StateMachineLocalFieldSchema] {
        self.fields.as_slice()
    }

    pub fn field_value(&self, field_id: &str) -> Option<&Property> {
        let normalized_field_id = field_id.trim();
        self.fields
            .iter()
            .find(|field| field.field_id == normalized_field_id)
            .map(|field| &field.value)
    }
}
