use crate::client_authored::cursors::cursor_set_schema::CursorSetSchema;
use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct CursorSetsSchema {
    #[serde(default)]
    #[prost(btree_map = "string, message", tag = "1")]
    pub sets: BTreeMap<String, CursorSetSchema>,
}

impl CursorSetsSchema {
    pub fn is_empty(&self) -> bool {
        self.sets.is_empty()
    }
}
