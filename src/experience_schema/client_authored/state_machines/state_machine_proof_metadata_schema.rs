use crate::client_authored::state_machines::state_machine_finite_domain_abstraction_schema::StateMachineFiniteDomainAbstractionSchema;
use crate::client_authored::state_machines::state_machine_proof_assertion_schema::StateMachineProofAssertionSchema;
use crate::client_authored::state_machines::state_machine_proof_class_schema::StateMachineProofClassSchema;
use serde::{Deserialize, Serialize};

/// Proof-only metadata used by offline analysis tooling rather than runtime execution.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StateMachineProofMetadataSchema {
    #[serde(default)]
    pub proof_class: StateMachineProofClassSchema,
    #[serde(default)]
    pub finite_domain_abstractions: Vec<StateMachineFiniteDomainAbstractionSchema>,
    #[serde(default)]
    pub proof_assertions: Vec<StateMachineProofAssertionSchema>,
}
