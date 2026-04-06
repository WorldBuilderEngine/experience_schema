use crate::client_authored::state_machines::state_machine_bounded_effect_contract_schema::StateMachineBoundedEffectContractSchema;
use crate::client_authored::state_machines::state_machine_finite_domain_abstraction_schema::StateMachineFiniteDomainAbstractionSchema;
use crate::client_authored::state_machines::state_machine_proof_assertion_schema::StateMachineProofAssertionSchema;
use crate::client_authored::state_machines::state_machine_proof_class_schema::StateMachineProofClassSchema;
use crate::client_authored::state_machines::state_machine_proof_metadata_schema::StateMachineProofMetadataSchema;
use crate::client_authored::state_machines::state_machine_synchronous_invocation_contract_schema::StateMachineSynchronousInvocationContractSchema;
use crate::properties::property_map::PropertyMap;
use serde::{Deserialize, Serialize};

/// Transitional machine-local baggage that remains readable for migration and offline tooling.
///
/// This is deliberately separate from the stripped-core authored/runtime schema shape.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StateMachineLegacyCompatibilitySchema {
    #[serde(default)]
    pub property_maps: Vec<(String, PropertyMap)>,
    #[serde(default)]
    pub bounded_effect_contract: StateMachineBoundedEffectContractSchema,
    #[serde(default)]
    pub synchronous_invocation_contract: StateMachineSynchronousInvocationContractSchema,
    #[serde(default, flatten)]
    proof_metadata: StateMachineProofMetadataSchema,
}

impl StateMachineLegacyCompatibilitySchema {
    pub fn with_proof_class(proof_class: StateMachineProofClassSchema) -> Self {
        let mut compatibility = Self::default();
        compatibility.set_proof_class(proof_class);
        compatibility
    }

    pub fn is_empty(&self) -> bool {
        self.property_maps.is_empty()
            && self.bounded_effect_contract == StateMachineBoundedEffectContractSchema::default()
            && self.synchronous_invocation_contract
                == StateMachineSynchronousInvocationContractSchema::default()
            && self.proof_metadata == StateMachineProofMetadataSchema::default()
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

    pub fn register_property_map(
        &mut self,
        property_map_id: impl Into<String>,
        property_map: PropertyMap,
    ) {
        let property_map_id_string = property_map_id.into().trim().to_string();
        if let Some(existing_property_map_index) =
            self.property_maps
                .iter()
                .position(|(existing_property_map_id, _)| {
                    existing_property_map_id == &property_map_id_string
                })
        {
            self.property_maps[existing_property_map_index].1 = property_map;
            return;
        }

        self.property_maps
            .push((property_map_id_string, property_map));
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

    pub fn set_bounded_effect_contract(
        &mut self,
        bounded_effect_contract: StateMachineBoundedEffectContractSchema,
    ) {
        self.bounded_effect_contract = bounded_effect_contract;
    }

    pub fn set_synchronous_invocation_contract(
        &mut self,
        synchronous_invocation_contract: StateMachineSynchronousInvocationContractSchema,
    ) {
        self.synchronous_invocation_contract = synchronous_invocation_contract;
    }
}
