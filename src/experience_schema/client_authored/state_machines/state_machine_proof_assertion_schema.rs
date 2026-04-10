use serde::{Deserialize, Serialize};

/// Author-declared proof obligations evaluated against the lowered state-machine proof model.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateMachineProofAssertionSchema {
    #[serde(default)]
    pub label: Option<String>,
    pub kind: StateMachineProofAssertionKindSchema,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StateMachineProofAssertionKindSchema {
    ReachableState {
        state_name: String,
    },
    ForbiddenState {
        state_name: String,
    },
    EventuallyReachesState {
        state_name: String,
    },
    NoReachableDeadEnd,
    RequiredEventSequence {
        first: StateMachineProofAssertionEventSchema,
        then: StateMachineProofAssertionEventSchema,
    },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StateMachineProofAssertionEventScopeSchema {
    Global,
    Local,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateMachineProofAssertionEventSchema {
    pub scope: StateMachineProofAssertionEventScopeSchema,
    pub identifier: String,
}

#[cfg(test)]
mod tests {
    use super::{
        StateMachineProofAssertionEventSchema, StateMachineProofAssertionEventScopeSchema,
        StateMachineProofAssertionKindSchema, StateMachineProofAssertionSchema,
    };

    #[test]
    fn deserializes_required_event_sequence_assertion() {
        let assertion = serde_json::from_str::<StateMachineProofAssertionSchema>(
            r#"{
                "label": "paired_events",
                "kind": {
                    "RequiredEventSequence": {
                        "first": {
                            "scope": "global",
                            "identifier": "custom:sample_story:dispatch_intro_started"
                        },
                        "then": {
                            "scope": "global",
                            "identifier": "custom:sample_story:dispatch_intro_completed"
                        }
                    }
                }
            }"#,
        )
        .expect("required-event-sequence assertion should deserialize");

        assert_eq!(
            assertion,
            StateMachineProofAssertionSchema {
                label: Some("paired_events".to_string()),
                kind: StateMachineProofAssertionKindSchema::RequiredEventSequence {
                    first: StateMachineProofAssertionEventSchema {
                        scope: StateMachineProofAssertionEventScopeSchema::Global,
                        identifier: "custom:sample_story:dispatch_intro_started".to_string(),
                    },
                    then: StateMachineProofAssertionEventSchema {
                        scope: StateMachineProofAssertionEventScopeSchema::Global,
                        identifier: "custom:sample_story:dispatch_intro_completed".to_string(),
                    },
                },
            }
        );
    }
}
