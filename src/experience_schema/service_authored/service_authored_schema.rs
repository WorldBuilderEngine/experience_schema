use crate::service_authored::meta_data::publisher_info_schema::PublisherInfoSchema;
use prost::Message;
use serde::{Deserialize, Serialize};

/// Describes privileged data, which must be authored from a trustworthy server-side source.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct ServiceAuthoredSchema {
    #[prost(message, required, tag = "1")]
    pub publisher_info: PublisherInfoSchema,
}

impl ServiceAuthoredSchema {}
