use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateMachineOwnedCollectionCapacitySchema {
    pub local_id: String,
    pub property_id: String,
    pub capacity: u32,
}

impl StateMachineOwnedCollectionCapacitySchema {
    pub fn new(
        local_id: impl Into<String>,
        property_id: impl Into<String>,
        capacity: u32,
    ) -> Self {
        Self {
            local_id: local_id.into().trim().to_string(),
            property_id: property_id.into().trim().to_string(),
            capacity,
        }
    }
}
