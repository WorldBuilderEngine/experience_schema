use crate::client_authored::worlds::world_object_schema::WorldObjectSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Transitional world-level compatibility data that remains readable for
/// migration and offline tooling.
///
/// This is deliberately separate from the stripped-core authored/runtime world
/// schema shape.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct WorldCompatibilitySchema {
    #[serde(default)]
    object_templates: HashMap<String, WorldObjectSchema>,
    #[serde(default, rename = "protocol_proof_assertions", skip_serializing)]
    legacy_protocol_proof_assertions: Vec<serde_json::Value>,
}

impl WorldCompatibilitySchema {
    pub fn is_empty(&self) -> bool {
        self.object_templates.is_empty() && self.legacy_protocol_proof_assertions.is_empty()
    }

    pub fn object_templates(&self) -> &HashMap<String, WorldObjectSchema> {
        &self.object_templates
    }

    pub fn object_templates_mut(&mut self) -> &mut HashMap<String, WorldObjectSchema> {
        &mut self.object_templates
    }

    pub fn register_object_template(
        &mut self,
        template_name: impl Into<String>,
        world_object_schema: WorldObjectSchema,
    ) {
        self.object_templates
            .insert(template_name.into().trim().to_string(), world_object_schema);
    }

    pub fn has_legacy_protocol_proof_assertions(&self) -> bool {
        !self.legacy_protocol_proof_assertions.is_empty()
    }
}
