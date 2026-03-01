use crate::service_authored::meta_data::publisher_info_schema::PublisherInfoSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Describes privileged data, which must be authored from a trustworthy server-side source.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct ServiceAuthoredSchema {
    pub publisher_info: PublisherInfoSchema,
    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
}

impl ServiceAuthoredSchema {}
