use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Physics2dStateMachineApiSchema {
    #[serde(alias = "set_node_linear_velocity_by_tag")]
    SetNodeLinearVelocity,
    #[serde(alias = "add_node_force_by_tag")]
    AddNodeForce,
    StepAndEmitCollisionEvents,
}
