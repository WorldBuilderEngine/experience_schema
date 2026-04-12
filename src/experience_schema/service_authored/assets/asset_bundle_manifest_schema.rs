use crate::service_authored::assets::AssetBundleManifestEntrySchema;
use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trusted runtime-facing asset facts derived by backend publish/build flows.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct AssetBundleManifestSchema {
    /// Bundle identifiers keyed to trusted runtime metadata entries.
    #[serde(default)]
    #[prost(map = "string, message", tag = "1")]
    pub bundles: HashMap<String, AssetBundleManifestEntrySchema>,
}
