use crate::service_authored::meta_data::publisher_info_schema::PublisherInfoSchema;
use serde::{Deserialize, Serialize};

/// Describes privileged data, which must be authored from a trustworthy server-side source.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct ServiceAuthoredSchema {
    pub publisher_info: PublisherInfoSchema,
}

impl ServiceAuthoredSchema {}
