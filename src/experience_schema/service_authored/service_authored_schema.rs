use crate::service_authored::assets::asset_bundle_manifest_schema::AssetBundleManifestSchema;
use crate::service_authored::meta_data::publisher_info_schema::PublisherInfoSchema;
use prost::Message;
use serde::{Deserialize, Serialize};

/// Describes privileged data, which must be authored from a trustworthy server-side source.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct ServiceAuthoredSchema {
    #[prost(message, required, tag = "1")]
    pub publisher_info: PublisherInfoSchema,

    /// Trusted list of bundle identifiers that backend services allow runtimes to consume.
    #[serde(default)]
    #[prost(message, optional, tag = "2")]
    pub asset_bundle_manifest: Option<AssetBundleManifestSchema>,
}

impl ServiceAuthoredSchema {}
