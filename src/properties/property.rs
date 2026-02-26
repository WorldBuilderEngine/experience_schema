use crate::assets::asset_ref::AssetRef;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum Property {
    Bool(bool),
    BoolArray(Vec<bool>),
    Int64(i64),
    Int64Array(Vec<i64>),
    Float64(f64),
    Float64Array(Vec<f64>),
    String(String),
    StringArray(Vec<String>),

    /// A reference to an asset by content path.
    AssetRef(AssetRef),
}
