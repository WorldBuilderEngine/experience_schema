use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorldStateMachineApiSchema {
    SetNodePositionByTag,
    SetNodeScaleByTag,
    SetNodeVisibilityByTag,
    SetNodeTextByTag,
    SetNodeTextColorByTag,
    ReorderNodeByTag,
    FollowActiveCameraByTag,
    CallStateMachine,
    RemoveStateMachine,
}
