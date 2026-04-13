use crate::properties::property_map::PropertyMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StateMachineLocalSchema {
    pub local_id: String,
    #[serde(default)]
    pub properties: PropertyMap,
}

impl StateMachineLocalSchema {
    pub fn new(local_id: impl Into<String>, properties: PropertyMap) -> Self {
        Self {
            local_id: local_id.into().trim().to_string(),
            properties,
        }
    }
}
