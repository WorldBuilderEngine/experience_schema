use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct PublisherInfoSchema {
    #[serde(default)]
    pub publisher_id: Option<String>,
    #[serde(default)]
    pub experience_id: Option<String>,
    #[serde(default)]
    pub publish_id: Option<String>,
    #[serde(default)]
    pub publish_version: Option<usize>,
    #[serde(default)]
    pub published_at_unix_seconds: Option<u64>,
    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
}

impl PublisherInfoSchema {}
