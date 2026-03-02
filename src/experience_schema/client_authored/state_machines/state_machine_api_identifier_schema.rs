use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StateMachineApiIdentifierSchema {
    Math(MathStateMachineApiIdentifierSchema),
    PropertyMap(PropertyMapStateMachineApiIdentifierSchema),
    World(WorldStateMachineApiIdentifierSchema),
    Custom(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MathStateMachineApiIdentifierSchema {
    MatrixComposeTrs,
    MatrixDecomposeTrs,
    MatrixInverse,
    MatrixMultiply,
    MatrixTranspose,
    QuaternionFromTo,
    QuaternionLookRotation,
    QuaternionMultiply,
    QuaternionNormalize,
    QuaternionSlerp,
    TransformDirection,
    TransformPoint,
    TransformVector,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PropertyMapStateMachineApiIdentifierSchema {
    RemoveProperty,
    UpsertProperty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WorldStateMachineApiIdentifierSchema {
    SetNodePositionByTag,
    SetNodeVisibilityByTag,
    SpawnObjectTemplate,
}

impl Default for StateMachineApiIdentifierSchema {
    fn default() -> Self {
        Self::PropertyMap(PropertyMapStateMachineApiIdentifierSchema::RemoveProperty)
    }
}

impl StateMachineApiIdentifierSchema {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Math(MathStateMachineApiIdentifierSchema::MatrixComposeTrs) => "math:matrix_compose_trs",
            Self::Math(MathStateMachineApiIdentifierSchema::MatrixDecomposeTrs) => "math:matrix_decompose_trs",
            Self::Math(MathStateMachineApiIdentifierSchema::MatrixInverse) => "math:matrix_inverse",
            Self::Math(MathStateMachineApiIdentifierSchema::MatrixMultiply) => "math:matrix_multiply",
            Self::Math(MathStateMachineApiIdentifierSchema::MatrixTranspose) => "math:matrix_transpose",
            Self::Math(MathStateMachineApiIdentifierSchema::QuaternionFromTo) => "math:quaternion_from_to",
            Self::Math(MathStateMachineApiIdentifierSchema::QuaternionLookRotation) => "math:quaternion_look_rotation",
            Self::Math(MathStateMachineApiIdentifierSchema::QuaternionMultiply) => "math:quaternion_multiply",
            Self::Math(MathStateMachineApiIdentifierSchema::QuaternionNormalize) => "math:quaternion_normalize",
            Self::Math(MathStateMachineApiIdentifierSchema::QuaternionSlerp) => "math:quaternion_slerp",
            Self::Math(MathStateMachineApiIdentifierSchema::TransformDirection) => "math:transform_direction",
            Self::Math(MathStateMachineApiIdentifierSchema::TransformPoint) => "math:transform_point",
            Self::Math(MathStateMachineApiIdentifierSchema::TransformVector) => "math:transform_vector",
            Self::PropertyMap(PropertyMapStateMachineApiIdentifierSchema::RemoveProperty) => "property_map:remove_property",
            Self::PropertyMap(PropertyMapStateMachineApiIdentifierSchema::UpsertProperty) => "property_map:upsert_property",
            Self::World(WorldStateMachineApiIdentifierSchema::SetNodePositionByTag) => "world:set_node_position_by_tag",
            Self::World(WorldStateMachineApiIdentifierSchema::SetNodeVisibilityByTag) => "world:set_node_visibility_by_tag",
            Self::World(WorldStateMachineApiIdentifierSchema::SpawnObjectTemplate) => "world:spawn_object_template",
            Self::Custom(identifier) => identifier.as_str(),
        }
    }

    pub fn from_identifier(identifier: impl Into<String>) -> Self {
        let identifier = identifier.into();
        match identifier.as_str() {
            "math:matrix_compose_trs" => Self::Math(MathStateMachineApiIdentifierSchema::MatrixComposeTrs),
            "math:matrix_decompose_trs" => Self::Math(MathStateMachineApiIdentifierSchema::MatrixDecomposeTrs),
            "math:matrix_inverse" => Self::Math(MathStateMachineApiIdentifierSchema::MatrixInverse),
            "math:matrix_multiply" => Self::Math(MathStateMachineApiIdentifierSchema::MatrixMultiply),
            "math:matrix_transpose" => Self::Math(MathStateMachineApiIdentifierSchema::MatrixTranspose),
            "math:quaternion_from_to" => Self::Math(MathStateMachineApiIdentifierSchema::QuaternionFromTo),
            "math:quaternion_look_rotation" => Self::Math(MathStateMachineApiIdentifierSchema::QuaternionLookRotation),
            "math:quaternion_multiply" => Self::Math(MathStateMachineApiIdentifierSchema::QuaternionMultiply),
            "math:quaternion_normalize" => Self::Math(MathStateMachineApiIdentifierSchema::QuaternionNormalize),
            "math:quaternion_slerp" => Self::Math(MathStateMachineApiIdentifierSchema::QuaternionSlerp),
            "math:transform_direction" => Self::Math(MathStateMachineApiIdentifierSchema::TransformDirection),
            "math:transform_point" => Self::Math(MathStateMachineApiIdentifierSchema::TransformPoint),
            "math:transform_vector" => Self::Math(MathStateMachineApiIdentifierSchema::TransformVector),
            "property_map:remove_property" => Self::PropertyMap(PropertyMapStateMachineApiIdentifierSchema::RemoveProperty),
            "property_map:upsert_property" => Self::PropertyMap(PropertyMapStateMachineApiIdentifierSchema::UpsertProperty),
            "world:set_node_position_by_tag" => Self::World(WorldStateMachineApiIdentifierSchema::SetNodePositionByTag),
            "world:set_node_visibility_by_tag" => Self::World(WorldStateMachineApiIdentifierSchema::SetNodeVisibilityByTag),
            "world:spawn_object_template" => Self::World(WorldStateMachineApiIdentifierSchema::SpawnObjectTemplate),
            _ => Self::Custom(identifier),
        }
    }
}

impl From<String> for StateMachineApiIdentifierSchema {
    fn from(value: String) -> Self {
        Self::from_identifier(value)
    }
}

impl From<&str> for StateMachineApiIdentifierSchema {
    fn from(value: &str) -> Self {
        Self::from_identifier(value.to_string())
    }
}

impl Serialize for StateMachineApiIdentifierSchema {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for StateMachineApiIdentifierSchema {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let identifier = String::deserialize(deserializer)?;
        let trimmed_identifier = identifier.trim();
        if trimmed_identifier.is_empty() {
            return Err(de::Error::custom("state machine API identifier in schema must not be blank"));
        }
        Ok(Self::from_identifier(trimmed_identifier.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::StateMachineApiIdentifierSchema;

    #[test]
    fn canonical_identifier_round_trips_as_string() {
        let identifier = StateMachineApiIdentifierSchema::from("world:set_node_visibility_by_tag");
        let serialized = serde_json::to_string(&identifier).expect("serialize");
        assert_eq!(serialized, "\"world:set_node_visibility_by_tag\"");

        let deserialized: StateMachineApiIdentifierSchema = serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(deserialized, identifier);
    }

    #[test]
    fn unknown_identifier_is_preserved_as_custom() {
        let deserialized: StateMachineApiIdentifierSchema =
            serde_json::from_str("\"point_and_click:dispatch_progression_complete\"").expect("deserialize");
        assert_eq!(
            deserialized,
            StateMachineApiIdentifierSchema::Custom("point_and_click:dispatch_progression_complete".to_string())
        );
    }
}
