use crate::client_authored::state_machines::state_machine_finite_domain_abstraction_schema::StateMachineFiniteDomainAbstractionSchema;
use crate::client_authored::state_machines::state_machine_proof_assertion_schema::StateMachineProofAssertionSchema;
use crate::client_authored::state_machines::state_machine_proof_class_schema::StateMachineProofClassSchema;
use crate::client_authored::state_machines::state_machine_proof_metadata_schema::StateMachineProofMetadataSchema;
use serde::{Deserialize, Serialize};

/// Transitional machine-local compatibility data that remains readable for migration and offline tooling.
///
/// This is deliberately separate from the stripped-core authored/runtime schema shape.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StateMachineCompatibilitySchema {
    #[serde(default, flatten)]
    proof_metadata: StateMachineProofMetadataSchema,
}

impl StateMachineCompatibilitySchema {
    pub fn with_proof_class(proof_class: StateMachineProofClassSchema) -> Self {
        let mut compatibility = Self::default();
        compatibility.set_proof_class(proof_class);
        compatibility
    }

    pub fn is_empty(&self) -> bool {
        self.proof_metadata == StateMachineProofMetadataSchema::default()
    }

    pub fn set_proof_class(&mut self, proof_class: StateMachineProofClassSchema) {
        self.proof_metadata.proof_class = proof_class;
    }

    pub fn declared_proof_class(&self) -> StateMachineProofClassSchema {
        self.proof_metadata.proof_class
    }

    pub fn finite_domain_abstractions(&self) -> &[StateMachineFiniteDomainAbstractionSchema] {
        self.proof_metadata.finite_domain_abstractions.as_slice()
    }

    pub fn proof_assertions(&self) -> &[StateMachineProofAssertionSchema] {
        self.proof_metadata.proof_assertions.as_slice()
    }

    pub fn proof_metadata(&self) -> &StateMachineProofMetadataSchema {
        &self.proof_metadata
    }

    pub fn set_proof_metadata(&mut self, proof_metadata: StateMachineProofMetadataSchema) {
        self.proof_metadata = proof_metadata;
    }

    pub fn register_finite_domain_abstraction(
        &mut self,
        abstraction: StateMachineFiniteDomainAbstractionSchema,
    ) {
        self.proof_metadata
            .finite_domain_abstractions
            .push(abstraction);
    }

    pub fn register_proof_assertion(&mut self, assertion: StateMachineProofAssertionSchema) {
        self.proof_metadata.proof_assertions.push(assertion);
    }
}
