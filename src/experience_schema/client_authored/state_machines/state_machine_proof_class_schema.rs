use serde::{Deserialize, Serialize};

/// Declares the expected proof tier for a state machine in authored and compiled schemas.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StateMachineProofClassSchema {
    Finite,
    BoundedExtended,
    #[default]
    EffectfulOpen,
}
