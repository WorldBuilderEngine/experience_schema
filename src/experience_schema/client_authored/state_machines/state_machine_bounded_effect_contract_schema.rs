use serde::{Deserialize, Serialize};

/// Declares authored bounds that keep resource creation and persistence key spaces honest for
/// proof-enabled machines.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateMachineBoundedEffectContractSchema {
    #[serde(default)]
    pub resource_creation: Option<StateMachineResourceCreationContractSchema>,
    #[serde(default)]
    pub persistence_key_registry: Option<StateMachinePersistenceKeyRegistrySchema>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateMachineResourceCreationContractSchema {
    pub total_creations_upper_bound: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateMachinePersistenceKeyRegistrySchema {
    pub members: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::{
        StateMachineBoundedEffectContractSchema, StateMachinePersistenceKeyRegistrySchema,
        StateMachineResourceCreationContractSchema,
    };

    #[test]
    fn deserializes_bounded_effect_contracts() {
        let contract = serde_json::from_str::<StateMachineBoundedEffectContractSchema>(
            r#"{
                "resource_creation": {
                    "total_creations_upper_bound": 8
                },
                "persistence_key_registry": {
                    "members": ["profile/player-1", "profile/player-2"]
                }
            }"#,
        )
        .expect("bounded effect contract should deserialize");

        assert_eq!(
            contract,
            StateMachineBoundedEffectContractSchema {
                resource_creation: Some(StateMachineResourceCreationContractSchema {
                    total_creations_upper_bound: 8,
                }),
                persistence_key_registry: Some(StateMachinePersistenceKeyRegistrySchema {
                    members: vec![
                        "profile/player-1".to_string(),
                        "profile/player-2".to_string()
                    ],
                }),
            }
        );
    }
}
