use crate::shared::state_machine_schema::StateMachineSchema;
use properties::property_map::PropertyMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct WorldObjectSchema {
    /// The custom properties (data) for this object.
    pub properties: PropertyMap,

    /// State machines (code) for this object.
    pub state_machines: Vec<StateMachineSchema>,
}
