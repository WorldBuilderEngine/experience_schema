use crate::client_authored::{input::input_layer_schema::InputLayerSchema, worlds::world_object_schema::WorldObjectSchema};
use crate::shared::state_machine_schema::StateMachineSchema;
use properties::property_map::PropertyMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Describes a world and its contents within this experience.
/// Note from owner: No longer differentiate 2d and 3d. This is not useful at the tail end of the schema.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WorldSchema {
    /// Objects to be instantiated by this world.
    pub objects: Vec<WorldObjectSchema>,

    /// The custom properties (data) for this world.
    pub properties: PropertyMap,

    /// State machines (code) for this world.
    pub state_machines: Vec<StateMachineSchema>,

    /// The input layers, which can be activated or deactivated.
    pub input_layers: Vec<InputLayerSchema>,

    /// Optional list of asset-bundle ids required by this world.
    pub asset_bundle_ids: Vec<String>,

    /// Reusable object templates that can be instantiated at runtime.
    pub object_templates: HashMap<String, WorldObjectSchema>,
}

impl WorldSchema {}
