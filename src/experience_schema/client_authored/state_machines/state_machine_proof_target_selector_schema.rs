use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StateMachineProofTargetSelectorSchema {
    MachineLocalField { local_id: String, field_id: String },
    StoreField { store_id: String, field_id: String },
}

#[cfg(test)]
mod tests {
    use super::StateMachineProofTargetSelectorSchema;

    #[test]
    fn deserializes_machine_local_field_selector() {
        let selector = serde_json::from_str::<StateMachineProofTargetSelectorSchema>(
            r#"{
                "MachineLocalField": {
                    "local_id": "runtime",
                    "field_id": "phase"
                }
            }"#,
        )
        .expect("machine-local field selector should deserialize");

        assert_eq!(
            selector,
            StateMachineProofTargetSelectorSchema::MachineLocalField {
                local_id: "runtime".to_string(),
                field_id: "phase".to_string(),
            }
        );
    }

    #[test]
    fn deserializes_store_field_selector() {
        let selector = serde_json::from_str::<StateMachineProofTargetSelectorSchema>(
            r#"{
                "StoreField": {
                    "store_id": "world.default.presentation_runtime.warm",
                    "field_id": "default_camera_viewport_size_px"
                }
            }"#,
        )
        .expect("store field selector should deserialize");

        assert_eq!(
            selector,
            StateMachineProofTargetSelectorSchema::StoreField {
                store_id: "world.default.presentation_runtime.warm".to_string(),
                field_id: "default_camera_viewport_size_px".to_string(),
            }
        );
    }
}
