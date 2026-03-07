use crate::client_authored::state_machines::state_machine_schema::StateMachineSchema;
use crate::client_authored::worlds::world_object_schema::WorldObjectSchema;
use crate::properties::property_map::PropertyMap;
use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Describes a world and its contents within this experience.
/// Note from owner: No longer differentiate 2d and 3d. This is not useful at the tail end of the schema.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct WorldSchema {
    /// Objects to be instantiated by this world.
    #[prost(message, repeated, tag = "1")]
    pub objects: Vec<WorldObjectSchema>,

    /// The custom properties (data) for this world.
    #[prost(message, required, tag = "2")]
    pub properties: PropertyMap,

    /// State machines (code) for this world.
    #[prost(message, repeated, tag = "3")]
    pub state_machines: Vec<StateMachineSchema>,

    /// Optional list of asset-bundle ids required by this world.
    #[prost(string, repeated, tag = "4")]
    pub asset_bundle_ids: Vec<String>,

    /// Reusable object templates that can be instantiated at runtime.
    #[prost(map = "string, message", tag = "5")]
    pub object_templates: HashMap<String, WorldObjectSchema>,
}

impl WorldSchema {}
