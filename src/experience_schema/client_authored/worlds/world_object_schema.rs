use crate::shared::state_machine_schema::StateMachineSchema;
use properties::property_map::PropertyMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct WorldObjectSchema {
    /// The custom properties (data) for this object.
    pub properties: PropertyMap,

    /// State machines (code) for this object.
    pub state_machines: Vec<StateMachineSchema>,
    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
}
