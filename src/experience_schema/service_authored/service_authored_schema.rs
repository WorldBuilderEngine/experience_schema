use crate::service_authored::assets::asset_bundle_manifest_schema::AssetBundleManifestSchema;
use crate::service_authored::meta_data::publisher_info_schema::PublisherInfoSchema;
use prost::Message;
use serde::{Deserialize, Serialize};

/// Describes privileged data that must come from a trustworthy server-side source.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct ServiceAuthoredSchema {
    #[prost(message, required, tag = "1")]
    pub publisher_info: PublisherInfoSchema,

    /// Trusted runtime-facing asset facts derived outside the client-authored schema.
    ///
    /// This stays limited to runtime metadata such as bundle allow-lists and intrinsic asset facts.
    #[serde(default)]
    #[prost(message, optional, tag = "2")]
    pub asset_bundle_manifest: Option<AssetBundleManifestSchema>,
}

impl ServiceAuthoredSchema {}
