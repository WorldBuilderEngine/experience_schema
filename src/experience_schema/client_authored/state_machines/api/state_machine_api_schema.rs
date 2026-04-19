use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use super::{
    Animation2dStateMachineApiSchema, DataBufferStateMachineApiSchema, MathStateMachineApiSchema,
    Physics2dStateMachineApiSchema, RuntimeStateMachineApiSchema, StringStateMachineApiSchema,
    WorldStateMachineApiSchema,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StateMachineApiSchema {
    Animation2d(Animation2dStateMachineApiSchema),
    DataBuffer(DataBufferStateMachineApiSchema),
    Math(MathStateMachineApiSchema),
    Physics2d(Physics2dStateMachineApiSchema),
    Runtime(RuntimeStateMachineApiSchema),
    String(StringStateMachineApiSchema),
    World(WorldStateMachineApiSchema),
    Custom(String),
}

impl Default for StateMachineApiSchema {
    fn default() -> Self {
        Self::Runtime(RuntimeStateMachineApiSchema::NoOp)
    }
}

impl StateMachineApiSchema {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Animation2d(Animation2dStateMachineApiSchema::StepPlayers) => {
                "animation2d:step_players"
            }
            Self::DataBuffer(DataBufferStateMachineApiSchema::Copy) => "data_buffer:copy",
            Self::DataBuffer(DataBufferStateMachineApiSchema::Concat) => "data_buffer:concat",
            Self::DataBuffer(DataBufferStateMachineApiSchema::Alloc) => "data_buffer:alloc",
            Self::DataBuffer(DataBufferStateMachineApiSchema::EncodeGenerationalHandle) => {
                "data_buffer:encode_generational_handle"
            }
            Self::DataBuffer(DataBufferStateMachineApiSchema::EncodeAssetHandle) => {
                "data_buffer:encode_asset_handle"
            }
            Self::DataBuffer(DataBufferStateMachineApiSchema::CopySlice) => {
                "data_buffer:copy_slice"
            }
            Self::DataBuffer(DataBufferStateMachineApiSchema::CopySliceInto) => {
                "data_buffer:copy_slice_into"
            }
            Self::DataBuffer(DataBufferStateMachineApiSchema::AddScalarF64LeSlice) => {
                "data_buffer:add_scalar_f64_le_slice"
            }
            Self::DataBuffer(DataBufferStateMachineApiSchema::AddScalarI32LeSlice) => {
                "data_buffer:add_scalar_i32_le_slice"
            }
            Self::DataBuffer(DataBufferStateMachineApiSchema::MulScalarF64LeSlice) => {
                "data_buffer:mul_scalar_f64_le_slice"
            }
            Self::DataBuffer(DataBufferStateMachineApiSchema::FillSliceU8) => {
                "data_buffer:fill_slice_u8"
            }
            Self::DataBuffer(DataBufferStateMachineApiSchema::Length) => "data_buffer:length",
            Self::DataBuffer(DataBufferStateMachineApiSchema::ReadU8) => "data_buffer:read_u8",
            Self::DataBuffer(DataBufferStateMachineApiSchema::ValidateSlice) => {
                "data_buffer:validate_slice"
            }
            Self::DataBuffer(DataBufferStateMachineApiSchema::WriteU8) => "data_buffer:write_u8",
            Self::DataBuffer(DataBufferStateMachineApiSchema::WriteU8Into) => {
                "data_buffer:write_u8_into"
            }
            Self::Math(MathStateMachineApiSchema::Add) => "math:add",
            Self::Math(MathStateMachineApiSchema::Sub) => "math:sub",
            Self::Math(MathStateMachineApiSchema::Mul) => "math:mul",
            Self::Math(MathStateMachineApiSchema::Div) => "math:div",
            Self::Math(MathStateMachineApiSchema::Mod) => "math:mod",
            Self::Math(MathStateMachineApiSchema::Abs) => "math:abs",
            Self::Math(MathStateMachineApiSchema::Min) => "math:min",
            Self::Math(MathStateMachineApiSchema::Max) => "math:max",
            Self::Math(MathStateMachineApiSchema::Clamp) => "math:clamp",
            Self::Math(MathStateMachineApiSchema::Floor) => "math:floor",
            Self::Math(MathStateMachineApiSchema::Ceil) => "math:ceil",
            Self::Math(MathStateMachineApiSchema::Round) => "math:round",
            Self::Math(MathStateMachineApiSchema::Trunc) => "math:trunc",
            Self::Math(MathStateMachineApiSchema::Sign) => "math:sign",
            Self::Math(MathStateMachineApiSchema::Sqrt) => "math:sqrt",
            Self::Math(MathStateMachineApiSchema::Pow) => "math:pow",
            Self::Math(MathStateMachineApiSchema::Sin) => "math:sin",
            Self::Math(MathStateMachineApiSchema::Cos) => "math:cos",
            Self::Math(MathStateMachineApiSchema::Tan) => "math:tan",
            Self::Math(MathStateMachineApiSchema::Asin) => "math:asin",
            Self::Math(MathStateMachineApiSchema::Acos) => "math:acos",
            Self::Math(MathStateMachineApiSchema::Atan) => "math:atan",
            Self::Math(MathStateMachineApiSchema::Atan2) => "math:atan2",
            Self::Math(MathStateMachineApiSchema::Lerp) => "math:lerp",
            Self::Math(MathStateMachineApiSchema::VectorAdd) => "math:vector_add",
            Self::Math(MathStateMachineApiSchema::VectorSub) => "math:vector_sub",
            Self::Math(MathStateMachineApiSchema::VectorMul) => "math:vector_mul",
            Self::Math(MathStateMachineApiSchema::VectorDiv) => "math:vector_div",
            Self::Math(MathStateMachineApiSchema::VectorScale) => "math:vector_scale",
            Self::Math(MathStateMachineApiSchema::VectorDot) => "math:vector_dot",
            Self::Math(MathStateMachineApiSchema::VectorCross) => "math:vector_cross",
            Self::Math(MathStateMachineApiSchema::VectorLength) => "math:vector_length",
            Self::Math(MathStateMachineApiSchema::VectorLengthSquared) => {
                "math:vector_length_squared"
            }
            Self::Math(MathStateMachineApiSchema::VectorNormalize) => "math:vector_normalize",
            Self::Math(MathStateMachineApiSchema::VectorDistance) => "math:vector_distance",
            Self::Math(MathStateMachineApiSchema::VectorDistanceSquared) => {
                "math:vector_distance_squared"
            }
            Self::Math(MathStateMachineApiSchema::VectorLerp) => "math:vector_lerp",
            Self::Math(MathStateMachineApiSchema::VectorClampMagnitude) => {
                "math:vector_clamp_magnitude"
            }
            Self::Math(MathStateMachineApiSchema::VectorReflect) => "math:vector_reflect",
            Self::Math(MathStateMachineApiSchema::VectorProject) => "math:vector_project",
            Self::Math(MathStateMachineApiSchema::VectorReject) => "math:vector_reject",
            Self::Math(MathStateMachineApiSchema::MatrixComposeTrs) => "math:matrix_compose_trs",
            Self::Math(MathStateMachineApiSchema::MatrixDecomposeTrs) => {
                "math:matrix_decompose_trs"
            }
            Self::Math(MathStateMachineApiSchema::MatrixInverse) => "math:matrix_inverse",
            Self::Math(MathStateMachineApiSchema::MatrixMultiply) => "math:matrix_multiply",
            Self::Math(MathStateMachineApiSchema::MatrixTranspose) => "math:matrix_transpose",
            Self::Math(MathStateMachineApiSchema::QuaternionFromTo) => "math:quaternion_from_to",
            Self::Math(MathStateMachineApiSchema::QuaternionLookRotation) => {
                "math:quaternion_look_rotation"
            }
            Self::Math(MathStateMachineApiSchema::QuaternionMultiply) => "math:quaternion_multiply",
            Self::Math(MathStateMachineApiSchema::QuaternionNormalize) => {
                "math:quaternion_normalize"
            }
            Self::Math(MathStateMachineApiSchema::QuaternionSlerp) => "math:quaternion_slerp",
            Self::Math(MathStateMachineApiSchema::TransformDirection) => "math:transform_direction",
            Self::Math(MathStateMachineApiSchema::TransformPoint) => "math:transform_point",
            Self::Math(MathStateMachineApiSchema::TransformVector) => "math:transform_vector",
            Self::Physics2d(Physics2dStateMachineApiSchema::SetNodeLinearVelocityByTag) => {
                "physics2d:set_node_linear_velocity_by_tag"
            }
            Self::Physics2d(Physics2dStateMachineApiSchema::AddNodeForceByTag) => {
                "physics2d:add_node_force_by_tag"
            }
            Self::Physics2d(Physics2dStateMachineApiSchema::StepAndEmitCollisionEvents) => {
                "physics2d:step_and_emit_collision_events"
            }
            Self::Runtime(RuntimeStateMachineApiSchema::NoOp) => "runtime:no_op",
            Self::Runtime(RuntimeStateMachineApiSchema::QueryStepDeltaSeconds) => {
                "runtime:query_step_delta_seconds"
            }
            Self::Runtime(RuntimeStateMachineApiSchema::QuerySimulatedStepDurationSeconds) => {
                "runtime:query_simulated_step_duration_seconds"
            }
            Self::Runtime(RuntimeStateMachineApiSchema::QuerySimulatedElapsedSeconds) => {
                "runtime:query_simulated_elapsed_seconds"
            }
            Self::Runtime(RuntimeStateMachineApiSchema::QueryDirectCallEntrypoint) => {
                "runtime:query_direct_call_entrypoint"
            }
            Self::Runtime(RuntimeStateMachineApiSchema::QueryDirectCallRequestTypeId) => {
                "runtime:query_direct_call_request_type_id"
            }
            Self::Runtime(RuntimeStateMachineApiSchema::QueryDirectCallResultTypeId) => {
                "runtime:query_direct_call_result_type_id"
            }
            Self::Runtime(RuntimeStateMachineApiSchema::CopyDirectCallRequestBytes) => {
                "runtime:copy_direct_call_request_bytes"
            }
            Self::Runtime(RuntimeStateMachineApiSchema::ReturnDirectCallResult) => {
                "runtime:return_direct_call_result"
            }
            Self::String(StringStateMachineApiSchema::Copy) => "string:copy",
            Self::String(StringStateMachineApiSchema::Concat) => "string:concat",
            Self::String(StringStateMachineApiSchema::ConcatBytes) => "string:concat_bytes",
            Self::String(StringStateMachineApiSchema::AppendBytes) => "string:append_bytes",
            Self::String(StringStateMachineApiSchema::DecodeUtf8Bytes) => {
                "string:decode_utf8_bytes"
            }
            Self::String(StringStateMachineApiSchema::StringFromBytes) => {
                "string:string_from_bytes"
            }
            Self::String(StringStateMachineApiSchema::Length) => "string:length",
            Self::String(StringStateMachineApiSchema::StringLenBytes) => "string:string_len_bytes",
            Self::String(StringStateMachineApiSchema::FormatInt) => "string:format_int",
            Self::String(StringStateMachineApiSchema::FormatIntBytes) => "string:format_int_bytes",
            Self::String(StringStateMachineApiSchema::FormatFloat) => "string:format_float",
            Self::String(StringStateMachineApiSchema::FormatFloatBytes) => {
                "string:format_float_bytes"
            }
            Self::String(StringStateMachineApiSchema::ArrayLength) => "string:array_length",
            Self::World(WorldStateMachineApiSchema::SetNodePosition) => "world:set_node_position",
            Self::World(WorldStateMachineApiSchema::SetNodeScale) => "world:set_node_scale",
            Self::World(WorldStateMachineApiSchema::SetNodeVisibility) => {
                "world:set_node_visibility"
            }
            Self::World(WorldStateMachineApiSchema::SetNodeText) => "world:set_node_text",
            Self::World(WorldStateMachineApiSchema::SetNodeTextColor) => {
                "world:set_node_text_color"
            }
            Self::World(WorldStateMachineApiSchema::ReorderNode) => "world:reorder_node",
            Self::World(WorldStateMachineApiSchema::FollowActiveCamera) => {
                "world:follow_active_camera"
            }
            Self::World(WorldStateMachineApiSchema::CallStateMachine) => "world:call_state_machine",
            Self::World(WorldStateMachineApiSchema::RemoveStateMachine) => {
                "world:remove_state_machine"
            }
            Self::Custom(identifier) => identifier.as_str(),
        }
    }

    pub fn from_identifier(identifier: impl Into<String>) -> Self {
        let identifier = identifier.into();
        match identifier.as_str() {
            "animation2d:step_players" => {
                Self::Animation2d(Animation2dStateMachineApiSchema::StepPlayers)
            }
            "data_buffer:copy" => Self::DataBuffer(DataBufferStateMachineApiSchema::Copy),
            "data_buffer:concat" => Self::DataBuffer(DataBufferStateMachineApiSchema::Concat),
            "data_buffer:alloc" => Self::DataBuffer(DataBufferStateMachineApiSchema::Alloc),
            "data_buffer:encode_generational_handle" => {
                Self::DataBuffer(DataBufferStateMachineApiSchema::EncodeGenerationalHandle)
            }
            "data_buffer:encode_asset_handle" => {
                Self::DataBuffer(DataBufferStateMachineApiSchema::EncodeAssetHandle)
            }
            "data_buffer:copy_slice" => {
                Self::DataBuffer(DataBufferStateMachineApiSchema::CopySlice)
            }
            "data_buffer:copy_slice_into" => {
                Self::DataBuffer(DataBufferStateMachineApiSchema::CopySliceInto)
            }
            "data_buffer:add_scalar_f64_le_slice" => {
                Self::DataBuffer(DataBufferStateMachineApiSchema::AddScalarF64LeSlice)
            }
            "data_buffer:add_scalar_i32_le_slice" => {
                Self::DataBuffer(DataBufferStateMachineApiSchema::AddScalarI32LeSlice)
            }
            "data_buffer:mul_scalar_f64_le_slice" => {
                Self::DataBuffer(DataBufferStateMachineApiSchema::MulScalarF64LeSlice)
            }
            "data_buffer:fill_slice_u8" => {
                Self::DataBuffer(DataBufferStateMachineApiSchema::FillSliceU8)
            }
            "data_buffer:length" => Self::DataBuffer(DataBufferStateMachineApiSchema::Length),
            "data_buffer:read_u8" => Self::DataBuffer(DataBufferStateMachineApiSchema::ReadU8),
            "data_buffer:validate_slice" => {
                Self::DataBuffer(DataBufferStateMachineApiSchema::ValidateSlice)
            }
            "data_buffer:write_u8" => Self::DataBuffer(DataBufferStateMachineApiSchema::WriteU8),
            "data_buffer:write_u8_into" => {
                Self::DataBuffer(DataBufferStateMachineApiSchema::WriteU8Into)
            }
            "math:add" => Self::Math(MathStateMachineApiSchema::Add),
            "math:sub" => Self::Math(MathStateMachineApiSchema::Sub),
            "math:mul" => Self::Math(MathStateMachineApiSchema::Mul),
            "math:div" => Self::Math(MathStateMachineApiSchema::Div),
            "math:mod" => Self::Math(MathStateMachineApiSchema::Mod),
            "math:abs" => Self::Math(MathStateMachineApiSchema::Abs),
            "math:min" => Self::Math(MathStateMachineApiSchema::Min),
            "math:max" => Self::Math(MathStateMachineApiSchema::Max),
            "math:clamp" => Self::Math(MathStateMachineApiSchema::Clamp),
            "math:floor" => Self::Math(MathStateMachineApiSchema::Floor),
            "math:ceil" => Self::Math(MathStateMachineApiSchema::Ceil),
            "math:round" => Self::Math(MathStateMachineApiSchema::Round),
            "math:trunc" => Self::Math(MathStateMachineApiSchema::Trunc),
            "math:sign" => Self::Math(MathStateMachineApiSchema::Sign),
            "math:sqrt" => Self::Math(MathStateMachineApiSchema::Sqrt),
            "math:pow" => Self::Math(MathStateMachineApiSchema::Pow),
            "math:sin" => Self::Math(MathStateMachineApiSchema::Sin),
            "math:cos" => Self::Math(MathStateMachineApiSchema::Cos),
            "math:tan" => Self::Math(MathStateMachineApiSchema::Tan),
            "math:asin" => Self::Math(MathStateMachineApiSchema::Asin),
            "math:acos" => Self::Math(MathStateMachineApiSchema::Acos),
            "math:atan" => Self::Math(MathStateMachineApiSchema::Atan),
            "math:atan2" => Self::Math(MathStateMachineApiSchema::Atan2),
            "math:lerp" => Self::Math(MathStateMachineApiSchema::Lerp),
            "math:vector_add" => Self::Math(MathStateMachineApiSchema::VectorAdd),
            "math:vector_sub" => Self::Math(MathStateMachineApiSchema::VectorSub),
            "math:vector_mul" => Self::Math(MathStateMachineApiSchema::VectorMul),
            "math:vector_div" => Self::Math(MathStateMachineApiSchema::VectorDiv),
            "math:vector_scale" => Self::Math(MathStateMachineApiSchema::VectorScale),
            "math:vector_dot" => Self::Math(MathStateMachineApiSchema::VectorDot),
            "math:vector_cross" => Self::Math(MathStateMachineApiSchema::VectorCross),
            "math:vector_length" => Self::Math(MathStateMachineApiSchema::VectorLength),
            "math:vector_length_squared" => {
                Self::Math(MathStateMachineApiSchema::VectorLengthSquared)
            }
            "math:vector_normalize" => Self::Math(MathStateMachineApiSchema::VectorNormalize),
            "math:vector_distance" => Self::Math(MathStateMachineApiSchema::VectorDistance),
            "math:vector_distance_squared" => {
                Self::Math(MathStateMachineApiSchema::VectorDistanceSquared)
            }
            "math:vector_lerp" => Self::Math(MathStateMachineApiSchema::VectorLerp),
            "math:vector_clamp_magnitude" => {
                Self::Math(MathStateMachineApiSchema::VectorClampMagnitude)
            }
            "math:vector_reflect" => Self::Math(MathStateMachineApiSchema::VectorReflect),
            "math:vector_project" => Self::Math(MathStateMachineApiSchema::VectorProject),
            "math:vector_reject" => Self::Math(MathStateMachineApiSchema::VectorReject),
            "math:matrix_compose_trs" => Self::Math(MathStateMachineApiSchema::MatrixComposeTrs),
            "math:matrix_decompose_trs" => {
                Self::Math(MathStateMachineApiSchema::MatrixDecomposeTrs)
            }
            "math:matrix_inverse" => Self::Math(MathStateMachineApiSchema::MatrixInverse),
            "math:matrix_multiply" => Self::Math(MathStateMachineApiSchema::MatrixMultiply),
            "math:matrix_transpose" => Self::Math(MathStateMachineApiSchema::MatrixTranspose),
            "math:quaternion_from_to" => Self::Math(MathStateMachineApiSchema::QuaternionFromTo),
            "math:quaternion_look_rotation" => {
                Self::Math(MathStateMachineApiSchema::QuaternionLookRotation)
            }
            "math:quaternion_multiply" => Self::Math(MathStateMachineApiSchema::QuaternionMultiply),
            "math:quaternion_normalize" => {
                Self::Math(MathStateMachineApiSchema::QuaternionNormalize)
            }
            "math:quaternion_slerp" => Self::Math(MathStateMachineApiSchema::QuaternionSlerp),
            "math:transform_direction" => Self::Math(MathStateMachineApiSchema::TransformDirection),
            "math:transform_point" => Self::Math(MathStateMachineApiSchema::TransformPoint),
            "math:transform_vector" => Self::Math(MathStateMachineApiSchema::TransformVector),
            "physics2d:set_node_linear_velocity_by_tag" => {
                Self::Physics2d(Physics2dStateMachineApiSchema::SetNodeLinearVelocityByTag)
            }
            "physics2d:add_node_force_by_tag" => {
                Self::Physics2d(Physics2dStateMachineApiSchema::AddNodeForceByTag)
            }
            "physics2d:step_and_emit_collision_events" => {
                Self::Physics2d(Physics2dStateMachineApiSchema::StepAndEmitCollisionEvents)
            }
            "runtime:no_op" => Self::Runtime(RuntimeStateMachineApiSchema::NoOp),
            "runtime:query_step_delta_seconds" => {
                Self::Runtime(RuntimeStateMachineApiSchema::QueryStepDeltaSeconds)
            }
            "runtime:query_simulated_step_duration_seconds" => {
                Self::Runtime(RuntimeStateMachineApiSchema::QuerySimulatedStepDurationSeconds)
            }
            "runtime:query_simulated_elapsed_seconds" => {
                Self::Runtime(RuntimeStateMachineApiSchema::QuerySimulatedElapsedSeconds)
            }
            "runtime:query_direct_call_entrypoint" => {
                Self::Runtime(RuntimeStateMachineApiSchema::QueryDirectCallEntrypoint)
            }
            "runtime:query_direct_call_request_type_id" => {
                Self::Runtime(RuntimeStateMachineApiSchema::QueryDirectCallRequestTypeId)
            }
            "runtime:query_direct_call_result_type_id" => {
                Self::Runtime(RuntimeStateMachineApiSchema::QueryDirectCallResultTypeId)
            }
            "runtime:copy_direct_call_request_bytes" => {
                Self::Runtime(RuntimeStateMachineApiSchema::CopyDirectCallRequestBytes)
            }
            "runtime:return_direct_call_result" => {
                Self::Runtime(RuntimeStateMachineApiSchema::ReturnDirectCallResult)
            }
            "string:copy" => Self::String(StringStateMachineApiSchema::Copy),
            "string:concat" => Self::String(StringStateMachineApiSchema::Concat),
            "string:concat_bytes" => Self::String(StringStateMachineApiSchema::ConcatBytes),
            "string:append_bytes" => Self::String(StringStateMachineApiSchema::AppendBytes),
            "string:decode_utf8_bytes" => {
                Self::String(StringStateMachineApiSchema::DecodeUtf8Bytes)
            }
            "string:string_from_bytes" => {
                Self::String(StringStateMachineApiSchema::StringFromBytes)
            }
            "string:length" => Self::String(StringStateMachineApiSchema::Length),
            "string:string_len_bytes" => Self::String(StringStateMachineApiSchema::StringLenBytes),
            "string:format_int" => Self::String(StringStateMachineApiSchema::FormatInt),
            "string:format_int_bytes" => Self::String(StringStateMachineApiSchema::FormatIntBytes),
            "string:format_float" => Self::String(StringStateMachineApiSchema::FormatFloat),
            "string:format_float_bytes" => {
                Self::String(StringStateMachineApiSchema::FormatFloatBytes)
            }
            "string:array_length" => Self::String(StringStateMachineApiSchema::ArrayLength),
            "world:set_node_position" | "world:set_node_position_by_tag" => {
                Self::World(WorldStateMachineApiSchema::SetNodePosition)
            }
            "world:set_node_scale" | "world:set_node_scale_by_tag" => {
                Self::World(WorldStateMachineApiSchema::SetNodeScale)
            }
            "world:set_node_visibility" | "world:set_node_visibility_by_tag" => {
                Self::World(WorldStateMachineApiSchema::SetNodeVisibility)
            }
            "world:set_node_text" | "world:set_node_text_by_tag" => {
                Self::World(WorldStateMachineApiSchema::SetNodeText)
            }
            "world:set_node_text_color" | "world:set_node_text_color_by_tag" => {
                Self::World(WorldStateMachineApiSchema::SetNodeTextColor)
            }
            "world:reorder_node" | "world:reorder_node_by_tag" => {
                Self::World(WorldStateMachineApiSchema::ReorderNode)
            }
            "world:follow_active_camera" | "world:follow_active_camera_by_tag" => {
                Self::World(WorldStateMachineApiSchema::FollowActiveCamera)
            }
            "world:call_state_machine" => Self::World(WorldStateMachineApiSchema::CallStateMachine),
            "world:remove_state_machine" => {
                Self::World(WorldStateMachineApiSchema::RemoveStateMachine)
            }
            _ => Self::Custom(identifier),
        }
    }
}

impl From<String> for StateMachineApiSchema {
    fn from(value: String) -> Self {
        Self::from_identifier(value)
    }
}

impl From<&str> for StateMachineApiSchema {
    fn from(value: &str) -> Self {
        Self::from_identifier(value.to_string())
    }
}

impl Serialize for StateMachineApiSchema {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for StateMachineApiSchema {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let identifier = String::deserialize(deserializer)?;
        let trimmed_identifier = identifier.trim();
        if trimmed_identifier.is_empty() {
            return Err(de::Error::custom(
                "state machine API in schema must not be blank",
            ));
        }
        Ok(Self::from_identifier(trimmed_identifier.to_string()))
    }
}

impl Message for StateMachineApiSchema {
    fn encode_raw(&self, buf: &mut impl BufMut) {
        StateMachineApiSchemaBinaryWire {
            identifier: self.as_str().to_string(),
        }
        .encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut wire = StateMachineApiSchemaBinaryWire {
            identifier: self.as_str().to_string(),
        };
        wire.merge_field(tag, wire_type, buf, ctx)?;
        *self = StateMachineApiSchema::from_identifier(wire.identifier);
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        StateMachineApiSchemaBinaryWire {
            identifier: self.as_str().to_string(),
        }
        .encoded_len()
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone, PartialEq, Message)]
struct StateMachineApiSchemaBinaryWire {
    #[prost(string, tag = "16")]
    identifier: String,
}

#[cfg(test)]
mod tests {
    use super::{
        DataBufferStateMachineApiSchema, RuntimeStateMachineApiSchema, StateMachineApiSchema,
        StringStateMachineApiSchema,
    };
    use prost::Message;

    #[test]
    fn canonical_identifier_round_trips_as_string() {
        let api = StateMachineApiSchema::from("world:set_node_visibility");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"world:set_node_visibility\"");

        let deserialized: StateMachineApiSchema =
            serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(deserialized, api);
    }

    #[test]
    fn unknown_identifier_is_preserved_as_custom() {
        let deserialized: StateMachineApiSchema =
            serde_json::from_str("\"custom:sample_story:dispatch_progression_complete\"")
                .expect("deserialize");
        assert_eq!(
            deserialized,
            StateMachineApiSchema::Custom(
                "custom:sample_story:dispatch_progression_complete".to_string()
            )
        );
    }

    #[test]
    fn animation2d_identifier_round_trips_as_string() {
        let api = StateMachineApiSchema::from("animation2d:step_players");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"animation2d:step_players\"");

        let deserialized: StateMachineApiSchema =
            serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(deserialized, api);
    }

    #[test]
    fn deleted_experience_storage_identifier_stays_quarantined_as_custom() {
        let api = StateMachineApiSchema::from("experience_storage:save_property_map_by_key");
        assert_eq!(
            api,
            StateMachineApiSchema::Custom(
                "experience_storage:save_property_map_by_key".to_string()
            )
        );
    }

    #[test]
    fn world_reorder_identifier_round_trips_as_canonical() {
        let api = StateMachineApiSchema::from("world:reorder_node");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"world:reorder_node\"");
    }

    #[test]
    fn legacy_world_tag_identifier_deserializes_to_selector_canonical_identifier() {
        let api = StateMachineApiSchema::from("world:set_node_visibility_by_tag");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"world:set_node_visibility\"");
    }

    #[test]
    fn world_remove_state_machine_identifier_round_trips_as_canonical() {
        let api = StateMachineApiSchema::from("world:remove_state_machine");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"world:remove_state_machine\"");
    }

    #[test]
    fn runtime_query_step_delta_identifier_round_trips_as_canonical() {
        let api = StateMachineApiSchema::from("runtime:query_step_delta_seconds");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"runtime:query_step_delta_seconds\"");
    }

    #[test]
    fn runtime_query_simulated_step_duration_identifier_round_trips_as_canonical() {
        let api = StateMachineApiSchema::from("runtime:query_simulated_step_duration_seconds");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(
            serialized,
            "\"runtime:query_simulated_step_duration_seconds\""
        );
    }

    #[test]
    fn runtime_query_simulated_elapsed_identifier_round_trips_as_canonical() {
        let api = StateMachineApiSchema::from("runtime:query_simulated_elapsed_seconds");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"runtime:query_simulated_elapsed_seconds\"");
    }

    #[test]
    fn runtime_no_op_identifier_round_trips_as_canonical() {
        let api = StateMachineApiSchema::from("runtime:no_op");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"runtime:no_op\"");

        let deserialized: StateMachineApiSchema =
            serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(
            deserialized,
            StateMachineApiSchema::Runtime(RuntimeStateMachineApiSchema::NoOp)
        );
    }

    #[test]
    fn string_identifier_round_trips_as_canonical() {
        let api = StateMachineApiSchema::from("string:concat");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"string:concat\"");

        let deserialized: StateMachineApiSchema =
            serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(
            deserialized,
            StateMachineApiSchema::String(StringStateMachineApiSchema::Concat)
        );
    }

    #[test]
    fn string_byte_alias_identifiers_round_trip_as_canonical() {
        let append_bytes = StateMachineApiSchema::from("string:append_bytes");
        let string_from_bytes = StateMachineApiSchema::from("string:string_from_bytes");
        let string_len_bytes = StateMachineApiSchema::from("string:string_len_bytes");

        assert_eq!(
            serde_json::to_string(&append_bytes).expect("serialize"),
            "\"string:append_bytes\""
        );
        assert_eq!(
            serde_json::to_string(&string_from_bytes).expect("serialize"),
            "\"string:string_from_bytes\""
        );
        assert_eq!(
            serde_json::to_string(&string_len_bytes).expect("serialize"),
            "\"string:string_len_bytes\""
        );

        assert_eq!(
            serde_json::from_str::<StateMachineApiSchema>("\"string:append_bytes\"")
                .expect("deserialize"),
            StateMachineApiSchema::String(StringStateMachineApiSchema::AppendBytes)
        );
        assert_eq!(
            serde_json::from_str::<StateMachineApiSchema>("\"string:string_from_bytes\"")
                .expect("deserialize"),
            StateMachineApiSchema::String(StringStateMachineApiSchema::StringFromBytes)
        );
        assert_eq!(
            serde_json::from_str::<StateMachineApiSchema>("\"string:string_len_bytes\"")
                .expect("deserialize"),
            StateMachineApiSchema::String(StringStateMachineApiSchema::StringLenBytes)
        );
    }

    #[test]
    fn data_buffer_identifier_round_trips_as_string() {
        let api = StateMachineApiSchema::DataBuffer(DataBufferStateMachineApiSchema::Concat);
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"data_buffer:concat\"");

        let deserialized: StateMachineApiSchema =
            serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(deserialized, api);
    }

    #[test]
    fn data_buffer_validate_slice_identifier_round_trips_as_string() {
        let api = StateMachineApiSchema::DataBuffer(DataBufferStateMachineApiSchema::ValidateSlice);
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"data_buffer:validate_slice\"");

        let deserialized: StateMachineApiSchema =
            serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(deserialized, api);
    }

    #[test]
    fn data_buffer_indexed_identifiers_round_trip_as_strings() {
        let copy_slice =
            StateMachineApiSchema::DataBuffer(DataBufferStateMachineApiSchema::CopySlice);
        let copy_slice_into =
            StateMachineApiSchema::DataBuffer(DataBufferStateMachineApiSchema::CopySliceInto);
        let add_scalar_f64_le_slice =
            StateMachineApiSchema::DataBuffer(DataBufferStateMachineApiSchema::AddScalarF64LeSlice);
        let fill_slice =
            StateMachineApiSchema::DataBuffer(DataBufferStateMachineApiSchema::FillSliceU8);
        let read_u8 = StateMachineApiSchema::DataBuffer(DataBufferStateMachineApiSchema::ReadU8);
        let write_u8 = StateMachineApiSchema::DataBuffer(DataBufferStateMachineApiSchema::WriteU8);

        assert_eq!(
            serde_json::to_string(&copy_slice).expect("serialize"),
            "\"data_buffer:copy_slice\""
        );
        assert_eq!(
            serde_json::to_string(&copy_slice_into).expect("serialize"),
            "\"data_buffer:copy_slice_into\""
        );
        assert_eq!(
            serde_json::to_string(&add_scalar_f64_le_slice).expect("serialize"),
            "\"data_buffer:add_scalar_f64_le_slice\""
        );
        assert_eq!(
            serde_json::to_string(&fill_slice).expect("serialize"),
            "\"data_buffer:fill_slice_u8\""
        );
        assert_eq!(
            serde_json::to_string(&read_u8).expect("serialize"),
            "\"data_buffer:read_u8\""
        );
        assert_eq!(
            serde_json::to_string(&write_u8).expect("serialize"),
            "\"data_buffer:write_u8\""
        );

        assert_eq!(
            serde_json::from_str::<StateMachineApiSchema>("\"data_buffer:copy_slice\"")
                .expect("deserialize"),
            copy_slice
        );
        assert_eq!(
            serde_json::from_str::<StateMachineApiSchema>("\"data_buffer:copy_slice_into\"")
                .expect("deserialize"),
            copy_slice_into
        );
        assert_eq!(
            serde_json::from_str::<StateMachineApiSchema>(
                "\"data_buffer:add_scalar_f64_le_slice\""
            )
            .expect("deserialize"),
            add_scalar_f64_le_slice
        );
        assert_eq!(
            serde_json::from_str::<StateMachineApiSchema>("\"data_buffer:fill_slice_u8\"")
                .expect("deserialize"),
            fill_slice
        );
        assert_eq!(
            serde_json::from_str::<StateMachineApiSchema>("\"data_buffer:read_u8\"")
                .expect("deserialize"),
            read_u8
        );
        assert_eq!(
            serde_json::from_str::<StateMachineApiSchema>("\"data_buffer:write_u8\"")
                .expect("deserialize"),
            write_u8
        );
    }

    #[test]
    fn data_buffer_handle_builder_identifiers_round_trip_as_strings() {
        let encode_generational = StateMachineApiSchema::DataBuffer(
            DataBufferStateMachineApiSchema::EncodeGenerationalHandle,
        );
        let encode_asset_handle =
            StateMachineApiSchema::DataBuffer(DataBufferStateMachineApiSchema::EncodeAssetHandle);

        assert_eq!(
            serde_json::to_string(&encode_generational).expect("serialize"),
            "\"data_buffer:encode_generational_handle\""
        );
        assert_eq!(
            serde_json::to_string(&encode_asset_handle).expect("serialize"),
            "\"data_buffer:encode_asset_handle\""
        );

        assert_eq!(
            serde_json::from_str::<StateMachineApiSchema>(
                "\"data_buffer:encode_generational_handle\""
            )
            .expect("deserialize"),
            encode_generational
        );
        assert_eq!(
            serde_json::from_str::<StateMachineApiSchema>("\"data_buffer:encode_asset_handle\"")
                .expect("deserialize"),
            encode_asset_handle
        );
    }

    #[test]
    fn prost_round_trips_api_schema_as_binary_message() {
        let api = StateMachineApiSchema::from("data_buffer:write_u8_into");

        let encoded = api.encode_to_vec();
        let decoded =
            StateMachineApiSchema::decode(encoded.as_slice()).expect("api schema should decode");

        assert_eq!(decoded, api);
    }
}
