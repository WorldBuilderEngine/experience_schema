use crate::service_authored::assets::{AssetBundleManifestSchema, AssetHandleTableSchema};
use crate::service_authored::meta_data::publisher_info_schema::PublisherInfoSchema;
use prost::Message;
use serde::{Deserialize, Serialize};

/// Describes privileged data that must come from a trustworthy server-side source.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct ServiceAuthoredSchema {
    #[prost(message, required, tag = "1")]
    pub publisher_info: PublisherInfoSchema,

    /// Trusted runtime-facing asset facts derived outside the client-authored schema.
    ///
    /// This stays limited to runtime metadata such as bundle allow-lists and intrinsic asset facts.
    #[serde(default)]
    #[prost(message, optional, tag = "2")]
    pub asset_bundle_manifest: Option<AssetBundleManifestSchema>,

    /// Trusted publish-time path-to-handle lowering consumed directly by runtime loaders.
    #[serde(default)]
    #[prost(message, optional, tag = "3")]
    pub asset_handle_table: Option<AssetHandleTableSchema>,
}

impl ServiceAuthoredSchema {}
