use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorldStateMachineApiSchema {
    #[serde(alias = "set_node_position_by_tag")]
    SetNodePosition,
    #[serde(alias = "set_node_scale_by_tag")]
    SetNodeScale,
    #[serde(alias = "set_node_visibility_by_tag")]
    SetNodeVisibility,
    #[serde(alias = "set_node_text_by_tag")]
    SetNodeText,
    #[serde(alias = "set_node_text_color_by_tag")]
    SetNodeTextColor,
    #[serde(alias = "reorder_node_by_tag")]
    ReorderNode,
    #[serde(alias = "follow_active_camera_by_tag")]
    FollowActiveCamera,
    CallStateMachine,
    RemoveStateMachine,
}
