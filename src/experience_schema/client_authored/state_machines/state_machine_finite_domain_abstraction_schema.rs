use crate::client_authored::state_machines::api::StateMachineApiSchema;
use crate::client_authored::state_machines::state_machine_proof_target_selector_schema::StateMachineProofTargetSelectorSchema;
use serde::{Deserialize, Serialize};

/// Declares a finite-domain contract that proof tooling may rely on for property fields or API outputs.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StateMachineFiniteDomainAbstractionSchema {
    pub target: StateMachineFiniteDomainTargetSchema,
    pub domain: StateMachineFiniteDomainSchema,
    #[serde(default)]
    pub semantics: StateMachineFiniteDomainSemanticsSchema,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StateMachineFiniteDomainSemanticsSchema {
    #[default]
    Exact,
    Conservative,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StateMachineFiniteDomainTargetSchema {
    PropertyField {
        local_id: String,
        property_id: String,
    },
    RuntimeTarget {
        selector: StateMachineProofTargetSelectorSchema,
    },
    ApiOutput {
        #[serde(alias = "api_identifier")]
        api: StateMachineApiSchema,
        local_id: String,
        property_id: String,
    },
    ApiOutputSelector {
        #[serde(alias = "api_identifier")]
        api: StateMachineApiSchema,
        selector: StateMachineProofTargetSelectorSchema,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StateMachineFiniteDomainSchema {
    Enum {
        values: Vec<String>,
    },
    FloatBuckets {
        buckets: Vec<StateMachineFloatBucketSchema>,
    },
    BoundedCounter {
        minimum: i64,
        maximum: i64,
    },
    BoundedIntVector {
        components: Vec<StateMachineFiniteIntRangeSchema>,
    },
    FiniteRegistry {
        members: Vec<String>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StateMachineFloatBucketSchema {
    pub label: String,
    #[serde(default)]
    pub min_inclusive: Option<f64>,
    #[serde(default)]
    pub max_exclusive: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateMachineFiniteIntRangeSchema {
    pub label: String,
    pub minimum: i64,
    pub maximum: i64,
}

#[cfg(test)]
mod tests {
    use super::{
        StateMachineFiniteDomainAbstractionSchema, StateMachineFiniteDomainSchema,
        StateMachineFiniteDomainSemanticsSchema, StateMachineFiniteDomainTargetSchema,
        StateMachineFiniteIntRangeSchema, StateMachineFloatBucketSchema,
    };
    use crate::client_authored::state_machines::api::{
        RuntimeStateMachineApiSchema, StateMachineApiSchema,
    };
    use crate::client_authored::state_machines::state_machine_proof_target_selector_schema::StateMachineProofTargetSelectorSchema;

    #[test]
    fn deserializes_property_field_abstraction() {
        let abstraction = serde_json::from_str::<StateMachineFiniteDomainAbstractionSchema>(
            r#"{
                "target": {
                    "PropertyField": {
                        "local_id": "runtime",
                        "property_id": "phase"
                    }
                },
                "domain": {
                    "Enum": {
                        "values": ["boot", "run", "done"]
                    }
                },
                "semantics": "exact"
            }"#,
        )
        .expect("property-field abstraction should deserialize");

        assert_eq!(
            abstraction,
            StateMachineFiniteDomainAbstractionSchema {
                target: StateMachineFiniteDomainTargetSchema::PropertyField {
                    local_id: "runtime".to_string(),
                    property_id: "phase".to_string(),
                },
                domain: StateMachineFiniteDomainSchema::Enum {
                    values: vec!["boot".to_string(), "run".to_string(), "done".to_string()],
                },
                semantics: StateMachineFiniteDomainSemanticsSchema::Exact,
            }
        );
    }

    #[test]
    fn deserializes_api_output_abstraction_from_api_identifier_alias() {
        let abstraction = serde_json::from_str::<StateMachineFiniteDomainAbstractionSchema>(
            r#"{
                "target": {
                    "ApiOutput": {
                        "api_identifier": "runtime:query_step_delta_seconds",
                        "local_id": "runtime",
                        "property_id": "step_delta_seconds"
                    }
                },
                "domain": {
                    "FloatBuckets": {
                        "buckets": [
                            { "label": "small", "min_inclusive": 0.0, "max_exclusive": 0.25 },
                            { "label": "large", "min_inclusive": 0.25 }
                        ]
                    }
                },
                "semantics": "conservative"
            }"#,
        )
        .expect("api-output abstraction should deserialize");

        assert_eq!(
            abstraction,
            StateMachineFiniteDomainAbstractionSchema {
                target: StateMachineFiniteDomainTargetSchema::ApiOutput {
                    api: StateMachineApiSchema::Runtime(
                        RuntimeStateMachineApiSchema::QueryStepDeltaSeconds
                    ),
                    local_id: "runtime".to_string(),
                    property_id: "step_delta_seconds".to_string(),
                },
                domain: StateMachineFiniteDomainSchema::FloatBuckets {
                    buckets: vec![
                        StateMachineFloatBucketSchema {
                            label: "small".to_string(),
                            min_inclusive: Some(0.0),
                            max_exclusive: Some(0.25),
                        },
                        StateMachineFloatBucketSchema {
                            label: "large".to_string(),
                            min_inclusive: Some(0.25),
                            max_exclusive: None,
                        },
                    ],
                },
                semantics: StateMachineFiniteDomainSemanticsSchema::Conservative,
            }
        );
    }

    #[test]
    fn deserializes_runtime_target_selector_abstraction() {
        let abstraction = serde_json::from_str::<StateMachineFiniteDomainAbstractionSchema>(
            r#"{
                "target": {
                    "RuntimeTarget": {
                        "selector": {
                            "MachineLocalField": {
                                "local_id": "runtime",
                                "field_id": "phase"
                            }
                        }
                    }
                },
                "domain": {
                    "Enum": {
                        "values": ["boot", "run", "done"]
                    }
                },
                "semantics": "exact"
            }"#,
        )
        .expect("runtime-target abstraction should deserialize");

        assert_eq!(
            abstraction,
            StateMachineFiniteDomainAbstractionSchema {
                target: StateMachineFiniteDomainTargetSchema::RuntimeTarget {
                    selector: StateMachineProofTargetSelectorSchema::MachineLocalField {
                        local_id: "runtime".to_string(),
                        field_id: "phase".to_string(),
                    },
                },
                domain: StateMachineFiniteDomainSchema::Enum {
                    values: vec!["boot".to_string(), "run".to_string(), "done".to_string()],
                },
                semantics: StateMachineFiniteDomainSemanticsSchema::Exact,
            }
        );
    }

    #[test]
    fn deserializes_bounded_int_vector_abstraction() {
        let abstraction = serde_json::from_str::<StateMachineFiniteDomainAbstractionSchema>(
            r#"{
                "target": {
                    "PropertyField": {
                        "local_id": "runtime",
                        "property_id": "ball_position"
                    }
                },
                "domain": {
                    "BoundedIntVector": {
                        "components": [
                            { "label": "x", "minimum": 0, "maximum": 11 },
                            { "label": "y", "minimum": 0, "maximum": 7 }
                        ]
                    }
                },
                "semantics": "exact"
            }"#,
        )
        .expect("bounded-int-vector abstraction should deserialize");

        assert_eq!(
            abstraction,
            StateMachineFiniteDomainAbstractionSchema {
                target: StateMachineFiniteDomainTargetSchema::PropertyField {
                    local_id: "runtime".to_string(),
                    property_id: "ball_position".to_string(),
                },
                domain: StateMachineFiniteDomainSchema::BoundedIntVector {
                    components: vec![
                        StateMachineFiniteIntRangeSchema {
                            label: "x".to_string(),
                            minimum: 0,
                            maximum: 11,
                        },
                        StateMachineFiniteIntRangeSchema {
                            label: "y".to_string(),
                            minimum: 0,
                            maximum: 7,
                        },
                    ],
                },
                semantics: StateMachineFiniteDomainSemanticsSchema::Exact,
            }
        );
    }

    #[test]
    fn deserializes_api_output_selector_abstraction() {
        let abstraction = serde_json::from_str::<StateMachineFiniteDomainAbstractionSchema>(
            r#"{
                "target": {
                    "ApiOutputSelector": {
                        "api_identifier": "runtime:query_step_delta_seconds",
                        "selector": {
                            "MachineLocalField": {
                                "local_id": "runtime",
                                "field_id": "step_delta_seconds"
                            }
                        }
                    }
                },
                "domain": {
                    "FloatBuckets": {
                        "buckets": [
                            { "label": "small", "min_inclusive": 0.0, "max_exclusive": 0.25 },
                            { "label": "large", "min_inclusive": 0.25 }
                        ]
                    }
                },
                "semantics": "conservative"
            }"#,
        )
        .expect("api-output selector abstraction should deserialize");

        assert_eq!(
            abstraction,
            StateMachineFiniteDomainAbstractionSchema {
                target: StateMachineFiniteDomainTargetSchema::ApiOutputSelector {
                    api: StateMachineApiSchema::Runtime(
                        RuntimeStateMachineApiSchema::QueryStepDeltaSeconds
                    ),
                    selector: StateMachineProofTargetSelectorSchema::MachineLocalField {
                        local_id: "runtime".to_string(),
                        field_id: "step_delta_seconds".to_string(),
                    },
                },
                domain: StateMachineFiniteDomainSchema::FloatBuckets {
                    buckets: vec![
                        StateMachineFloatBucketSchema {
                            label: "small".to_string(),
                            min_inclusive: Some(0.0),
                            max_exclusive: Some(0.25),
                        },
                        StateMachineFloatBucketSchema {
                            label: "large".to_string(),
                            min_inclusive: Some(0.25),
                            max_exclusive: None,
                        },
                    ],
                },
                semantics: StateMachineFiniteDomainSemanticsSchema::Conservative,
            }
        );
    }
}
