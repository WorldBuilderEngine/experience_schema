use crate::{
    client_authored::client_authored_schema::ClientAuthoredSchema,
    service_authored::service_authored_schema::ServiceAuthoredSchema,
};
use serde::{Deserialize, Serialize};

/// Describes a fully serialized experience consumed by runtimes/clients.
///
/// Layering contract for schemas:
/// - Game/studio compilers first produce typed runtime schemas (for example `World2dSchema`).
/// - Translators then wrap typed runtime schemas into this universal experience envelope.
/// - This envelope should stay minimal and avoid direct game-specific fields.
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct ExperienceSchema {
    pub service_authored_schema: ServiceAuthoredSchema,
    pub client_authored_schema: ClientAuthoredSchema,
}

impl ExperienceSchema {}
