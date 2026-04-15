use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Message)]
pub struct StateMachineOwnedCollectionCapacitySchema {
    #[prost(string, tag = "1")]
    pub local_id: String,
    #[prost(string, tag = "2")]
    pub property_id: String,
    #[prost(uint32, tag = "3")]
    pub capacity: u32,
}

impl StateMachineOwnedCollectionCapacitySchema {
    pub fn new(local_id: impl Into<String>, property_id: impl Into<String>, capacity: u32) -> Self {
        Self {
            local_id: local_id.into().trim().to_string(),
            property_id: property_id.into().trim().to_string(),
            capacity,
        }
    }
}
