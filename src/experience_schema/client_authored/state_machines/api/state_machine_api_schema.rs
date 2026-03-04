use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use super::{
    Animation2dStateMachineApiSchema, ExperienceStorageStateMachineApiSchema,
    MathStateMachineApiSchema, Physics2dStateMachineApiSchema, PropertyMapStateMachineApiSchema,
    WorldStateMachineApiSchema,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StateMachineApiSchema {
    Animation2d(Animation2dStateMachineApiSchema),
    ExperienceStorage(ExperienceStorageStateMachineApiSchema),
    Math(MathStateMachineApiSchema),
    Physics2d(Physics2dStateMachineApiSchema),
    PropertyMap(PropertyMapStateMachineApiSchema),
    World(WorldStateMachineApiSchema),
    Custom(String),
}

impl Default for StateMachineApiSchema {
    fn default() -> Self {
        Self::PropertyMap(PropertyMapStateMachineApiSchema::RemoveProperty)
    }
}

impl StateMachineApiSchema {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Animation2d(Animation2dStateMachineApiSchema::StepPlayers) => {
                "animation2d:step_players"
            }
            Self::ExperienceStorage(
                ExperienceStorageStateMachineApiSchema::LoadPropertyMapByKey,
            ) => "experience_storage:load_property_map_by_key",
            Self::ExperienceStorage(
                ExperienceStorageStateMachineApiSchema::SavePropertyMapByKey,
            ) => "experience_storage:save_property_map_by_key",
            Self::ExperienceStorage(
                ExperienceStorageStateMachineApiSchema::QueryPropertyMapByKey,
            ) => "experience_storage:query_property_map_by_key",
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
            Self::PropertyMap(PropertyMapStateMachineApiSchema::RemoveProperty) => {
                "property_map:remove_property"
            }
            Self::PropertyMap(PropertyMapStateMachineApiSchema::UpsertProperty) => {
                "property_map:upsert_property"
            }
            Self::World(WorldStateMachineApiSchema::SetNodePositionByTag) => {
                "world:set_node_position_by_tag"
            }
            Self::World(WorldStateMachineApiSchema::SetNodeVisibilityByTag) => {
                "world:set_node_visibility_by_tag"
            }
            Self::World(WorldStateMachineApiSchema::SpawnObjectTemplate) => {
                "world:spawn_object_template"
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
            // TODO(governance#91): Remove property_map compatibility aliases after
            // authored schemas stop using legacy persistence identifiers.
            "experience_storage:load_property_map_by_key"
            | "property_map:load_property_map_by_key" => Self::ExperienceStorage(
                ExperienceStorageStateMachineApiSchema::LoadPropertyMapByKey,
            ),
            "experience_storage:save_property_map_by_key"
            | "property_map:save_property_map_by_key" => Self::ExperienceStorage(
                ExperienceStorageStateMachineApiSchema::SavePropertyMapByKey,
            ),
            "experience_storage:query_property_map_by_key" => Self::ExperienceStorage(
                ExperienceStorageStateMachineApiSchema::QueryPropertyMapByKey,
            ),
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
            "property_map:remove_property" => {
                Self::PropertyMap(PropertyMapStateMachineApiSchema::RemoveProperty)
            }
            "property_map:upsert_property" => {
                Self::PropertyMap(PropertyMapStateMachineApiSchema::UpsertProperty)
            }
            "world:set_node_position_by_tag" => {
                Self::World(WorldStateMachineApiSchema::SetNodePositionByTag)
            }
            "world:set_node_visibility_by_tag" => {
                Self::World(WorldStateMachineApiSchema::SetNodeVisibilityByTag)
            }
            "world:spawn_object_template" => {
                Self::World(WorldStateMachineApiSchema::SpawnObjectTemplate)
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

#[cfg(test)]
mod tests {
    use super::StateMachineApiSchema;

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
            serde_json::from_str("\"point_and_click:dispatch_progression_complete\"")
                .expect("deserialize");
        assert_eq!(
            deserialized,
            StateMachineApiSchema::Custom(
                "point_and_click:dispatch_progression_complete".to_string()
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
    fn property_map_persistence_identifiers_normalize_to_experience_storage_namespace() {
        let api = StateMachineApiSchema::from("property_map:save_property_map_by_key");
        let serialized = serde_json::to_string(&api).expect("serialize");
        assert_eq!(
            serialized,
            "\"experience_storage:save_property_map_by_key\""
        );
    }
}
