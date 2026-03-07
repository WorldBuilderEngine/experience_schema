use crate::client_authored::state_machines::state_machine_schema::StateMachineSchema;
use crate::properties::property_map::PropertyMap;
use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct WorldObjectSchema {
    /// The custom properties (data) for this object.
    #[prost(message, required, tag = "1")]
    pub properties: PropertyMap,

    /// State machines (code) for this object.
    #[prost(message, repeated, tag = "2")]
    pub state_machines: Vec<StateMachineSchema>,
}
