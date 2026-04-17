use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StringStateMachineApiSchema {
    Copy,
    Concat,
    ConcatBytes,
    AppendBytes,
    Length,
    StringLenBytes,
    FormatInt,
    FormatFloat,
    ArrayLength,
    DecodeUtf8Bytes,
    StringFromBytes,
    FormatIntBytes,
    FormatFloatBytes,
}
