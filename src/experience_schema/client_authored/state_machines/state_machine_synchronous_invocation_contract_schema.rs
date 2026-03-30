use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StateMachineSchedulerCapabilitySchema {
    #[default]
    QueuedOnly,
    SyncReceive,
    SyncCall,
    SyncCallAndReceive,
}

impl StateMachineSchedulerCapabilitySchema {
    pub const fn allows_sync_call(self) -> bool {
        matches!(self, Self::SyncCall | Self::SyncCallAndReceive)
    }

    pub const fn allows_sync_receive(self) -> bool {
        matches!(self, Self::SyncReceive | Self::SyncCallAndReceive)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateMachineSynchronousInvocationContractSchema {
    #[serde(default)]
    pub machine_label: Option<String>,
    #[serde(default)]
    pub scheduler_capability: StateMachineSchedulerCapabilitySchema,
    #[serde(default)]
    pub maximum_call_depth: Option<u32>,
    #[serde(default)]
    pub call_fuel_budget: Option<u32>,
    #[serde(default)]
    pub mutable_resources: Vec<String>,
    #[serde(default)]
    pub receive_entrypoints: Vec<StateMachineSynchronousReceiveEntrypointSchema>,
    #[serde(default)]
    pub outgoing_calls: Vec<StateMachineSynchronousOutgoingCallSchema>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateMachineSynchronousReceiveEntrypointSchema {
    pub name: String,
    #[serde(default)]
    pub request_property_map_id: Option<String>,
    #[serde(default)]
    pub response_property_map_id: Option<String>,
    #[serde(default)]
    pub mutable_resources: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateMachineSynchronousOutgoingCallSchema {
    pub target_machine_label: String,
    pub target_entrypoint: String,
    #[serde(default)]
    pub request_property_map_id: Option<String>,
    #[serde(default)]
    pub response_property_map_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{
        StateMachineSchedulerCapabilitySchema, StateMachineSynchronousInvocationContractSchema,
        StateMachineSynchronousOutgoingCallSchema, StateMachineSynchronousReceiveEntrypointSchema,
    };

    #[test]
    fn defaults_to_queued_only_with_no_contract_data() {
        let contract = StateMachineSynchronousInvocationContractSchema::default();

        assert_eq!(
            contract.scheduler_capability,
            StateMachineSchedulerCapabilitySchema::QueuedOnly
        );
        assert!(contract.machine_label.is_none());
        assert!(contract.maximum_call_depth.is_none());
        assert!(contract.call_fuel_budget.is_none());
        assert!(contract.mutable_resources.is_empty());
        assert!(contract.receive_entrypoints.is_empty());
        assert!(contract.outgoing_calls.is_empty());
    }

    #[test]
    fn deserializes_sync_call_and_receive_contracts() {
        let contract = serde_json::from_str::<StateMachineSynchronousInvocationContractSchema>(
            r#"{
                "machine_label":"combat:resolver",
                "scheduler_capability":"sync_call_and_receive",
                "maximum_call_depth":3,
                "call_fuel_budget":5,
                "mutable_resources":["world:turn_state"],
                "receive_entrypoints":[
                    {
                        "name":"resolve_hit",
                        "request_property_map_id":"request",
                        "response_property_map_id":"response",
                        "mutable_resources":["world:turn_state"]
                    }
                ],
                "outgoing_calls":[
                    {
                        "target_machine_label":"combat:rules",
                        "target_entrypoint":"apply_damage",
                        "request_property_map_id":"request",
                        "response_property_map_id":"response"
                    }
                ]
            }"#,
        )
        .expect("sync contract should deserialize");

        assert_eq!(
            contract,
            StateMachineSynchronousInvocationContractSchema {
                machine_label: Some("combat:resolver".to_string()),
                scheduler_capability: StateMachineSchedulerCapabilitySchema::SyncCallAndReceive,
                maximum_call_depth: Some(3),
                call_fuel_budget: Some(5),
                mutable_resources: vec!["world:turn_state".to_string()],
                receive_entrypoints: vec![StateMachineSynchronousReceiveEntrypointSchema {
                    name: "resolve_hit".to_string(),
                    request_property_map_id: Some("request".to_string()),
                    response_property_map_id: Some("response".to_string()),
                    mutable_resources: vec!["world:turn_state".to_string()],
                }],
                outgoing_calls: vec![StateMachineSynchronousOutgoingCallSchema {
                    target_machine_label: "combat:rules".to_string(),
                    target_entrypoint: "apply_damage".to_string(),
                    request_property_map_id: Some("request".to_string()),
                    response_property_map_id: Some("response".to_string()),
                }],
            }
        );
    }
}
