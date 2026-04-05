use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateMachineOwnedCollectionCapacitySchema {
    pub property_map_id: String,
    pub property_id: String,
    pub capacity: u32,
}

impl StateMachineOwnedCollectionCapacitySchema {
    pub fn new(
        property_map_id: impl Into<String>,
        property_id: impl Into<String>,
        capacity: u32,
    ) -> Self {
        Self {
            property_map_id: property_map_id.into().trim().to_string(),
            property_id: property_id.into().trim().to_string(),
            capacity,
        }
    }
}
