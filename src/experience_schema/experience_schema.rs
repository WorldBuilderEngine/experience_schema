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
/// This is the universal target format for all published experiences.
/// Experiences may have their own internal schemas and formats that transpile to this format.
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ExperienceSchema {
    /// The version of this schema. Older versions will need migration steps.
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,

    /// Schema populated by backend trusted services.
    pub service_authored_schema: ServiceAuthoredSchema,

    /// Schema populated by clients. May still need verification on the backend side if submitted for publishing.
    pub client_authored_schema: ClientAuthoredSchema,

    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
}

impl ExperienceSchema {}
