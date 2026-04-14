use crate::wire_compat::json_message::{
    encode_as_json_message, json_message_encoded_len, merge_from_json_message,
};
use prost::DecodeError;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use super::{
    Animation2dStateMachineApiSchema, ByteBufferStateMachineApiSchema, MathStateMachineApiSchema, Physics2dStateMachineApiSchema,
    RuntimeStateMachineApiSchema, StringStateMachineApiSchema, WorldStateMachineApiSchema,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StateMachineApiSchema {
    Animation2d(Animation2dStateMachineApiSchema),
    ByteBuffer(ByteBufferStateMachineApiSchema),
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
            Self::ByteBuffer(ByteBufferStateMachineApiSchema::Copy) => "byte_buffer:copy",
            Self::ByteBuffer(ByteBufferStateMachineApiSchema::Concat) => "byte_buffer:concat",
            Self::ByteBuffer(ByteBufferStateMachineApiSchema::Length) => "byte_buffer:length",
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
            Self::String(StringStateMachineApiSchema::Copy) => "string:copy",
            Self::String(StringStateMachineApiSchema::Concat) => "string:concat",
            Self::String(StringStateMachineApiSchema::ConcatBytes) => "string:concat_bytes",
            Self::String(StringStateMachineApiSchema::DecodeUtf8Bytes) => "string:decode_utf8_bytes",
            Self::String(StringStateMachineApiSchema::Length) => "string:length",
            Self::String(StringStateMachineApiSchema::FormatInt) => "string:format_int",
            Self::String(StringStateMachineApiSchema::FormatIntBytes) => {
                "string:format_int_bytes"
            }
            Self::String(StringStateMachineApiSchema::FormatFloat) => "string:format_float",
            Self::String(StringStateMachineApiSchema::FormatFloatBytes) => {
                "string:format_float_bytes"
            }
            Self::String(StringStateMachineApiSchema::ArrayLength) => "string:array_length",
            Self::World(WorldStateMachineApiSchema::SetNodePositionByTag) => {
                "world:set_node_position_by_tag"
            }
            Self::World(WorldStateMachineApiSchema::SetNodeVisibilityByTag) => {
                "world:set_node_visibility_by_tag"
            }
            Self::World(WorldStateMachineApiSchema::SetNodeTextByTag) => {
                "world:set_node_text_by_tag"
            }
            Self::World(WorldStateMachineApiSchema::ReorderNodeByTag) => {
                "world:reorder_node_by_tag"
            }
            Self::World(WorldStateMachineApiSchema::FollowActiveCameraByTag) => {
                "world:follow_active_camera_by_tag"
            }
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
            "byte_buffer:copy" => Self::ByteBuffer(ByteBufferStateMachineApiSchema::Copy),
            "byte_buffer:concat" => Self::ByteBuffer(ByteBufferStateMachineApiSchema::Concat),
            "byte_buffer:length" => Self::ByteBuffer(ByteBufferStateMachineApiSchema::Length),
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
            "string:copy" => Self::String(StringStateMachineApiSchema::Copy),
            "string:concat" => Self::String(StringStateMachineApiSchema::Concat),
            "string:concat_bytes" => Self::String(StringStateMachineApiSchema::ConcatBytes),
            "string:decode_utf8_bytes" => {
                Self::String(StringStateMachineApiSchema::DecodeUtf8Bytes)
            }
            "string:length" => Self::String(StringStateMachineApiSchema::Length),
            "string:format_int" => Self::String(StringStateMachineApiSchema::FormatInt),
            "string:format_int_bytes" => {
                Self::String(StringStateMachineApiSchema::FormatIntBytes)
            }
            "string:format_float" => Self::String(StringStateMachineApiSchema::FormatFloat),
            "string:format_float_bytes" => {
                Self::String(StringStateMachineApiSchema::FormatFloatBytes)
            }
            "string:array_length" => Self::String(StringStateMachineApiSchema::ArrayLength),
            "world:set_node_position" => {
                Self::World(WorldStateMachineApiSchema::SetNodePositionByTag)
            }
            "world:set_node_position_by_tag" => {
                Self::World(WorldStateMachineApiSchema::SetNodePositionByTag)
            }
            "world:set_node_visibility" => {
                Self::World(WorldStateMachineApiSchema::SetNodeVisibilityByTag)
            }
            "world:set_node_visibility_by_tag" => {
                Self::World(WorldStateMachineApiSchema::SetNodeVisibilityByTag)
            }
            "world:set_node_text_by_tag" => {
                Self::World(WorldStateMachineApiSchema::SetNodeTextByTag)
            }
            "world:reorder_node_by_tag" => {
                Self::World(WorldStateMachineApiSchema::ReorderNodeByTag)
            }
            "world:follow_active_camera_by_tag" => {
                Self::World(WorldStateMachineApiSchema::FollowActiveCameraByTag)
            }
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
        encode_as_json_message(self, buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        merge_from_json_message(self, tag, wire_type, buf, ctx)
    }

    fn encoded_len(&self) -> usize {
        json_message_encoded_len(self)
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::{ByteBufferStateMachineApiSchema, RuntimeStateMachineApiSchema, StateMachineApiSchema, StringStateMachineApiSchema, WorldStateMachineApiSchema};

    #[test]
    fn canonical_identifier_round_trips_as_string() {
        let api = StateMachineApiSchema::from("world:set_node_visibility_by_tag");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"world:set_node_visibility_by_tag\"");

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
    fn legacy_world_identifier_aliases_lower_to_canonical_variants() {
        let api = StateMachineApiSchema::from("world:set_node_visibility");
        assert_eq!(
            api,
            StateMachineApiSchema::World(WorldStateMachineApiSchema::SetNodeVisibilityByTag)
        );
        assert_eq!(api.as_str(), "world:set_node_visibility_by_tag");
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
        let api = StateMachineApiSchema::from("world:reorder_node_by_tag");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"world:reorder_node_by_tag\"");
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
    fn byte_buffer_identifier_round_trips_as_string() {
        let api = StateMachineApiSchema::ByteBuffer(ByteBufferStateMachineApiSchema::Concat);
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(serialized, "\"byte_buffer:concat\"");

        let deserialized: StateMachineApiSchema =
            serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(deserialized, api);
    }
}
