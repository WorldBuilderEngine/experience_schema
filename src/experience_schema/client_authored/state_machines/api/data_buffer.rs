use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataBufferStateMachineApiSchema {
    Copy,
    Concat,
    Alloc,
    EncodeStateMachineHandle,
    EncodeNodeHandle,
    EncodeCameraHandle,
    EncodeUiHandle,
    EncodeAssetRef,
    CopySlice,
    CopySliceInto,
    AddScalarF64LeSlice,
    AddScalarI32LeSlice,
    MulScalarF64LeSlice,
    FillSliceU8,
    Length,
    ReadU8,
    ValidateSlice,
    WriteU8,
    WriteU8Into,
}
