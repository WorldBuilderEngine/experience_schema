use crate::{
    client_authored::client_authored_schema::ClientAuthoredSchema,
    service_authored::service_authored_schema::ServiceAuthoredSchema,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const CURRENT_EXPERIENCE_SCHEMA_VERSION: u32 = 1;

fn default_schema_version() -> u32 {
    CURRENT_EXPERIENCE_SCHEMA_VERSION
}

/// Describes a fully serialized experience consumed by runtimes/clients.
///
/// Layering contract for schemas:
/// - Game/studio compilers first produce typed runtime schemas (for example `World2dSchema`).
/// - Translators then wrap typed runtime schemas into this universal experience envelope.
/// - This envelope should stay minimal and avoid direct game-specific fields.
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ExperienceSchema {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    pub service_authored_schema: ServiceAuthoredSchema,
    pub client_authored_schema: ClientAuthoredSchema,
    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
}

impl ExperienceSchema {}
